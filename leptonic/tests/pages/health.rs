//! Diagnostic collection. Fixture expectations and validation policy live in `fixtures`.
//! Missing instrumentation is an error, never an empty healthy report.

use std::{fmt, sync::Arc};

use browser_test::thirtyfour::session::handle::SessionHandle;
use rootcause::{Report, prelude::ResultExt};
use serde::Deserialize;

use super::element::DESCRIBE;

/// The attributes holding id references (one id, or a space-separated list): ARIA relations and
/// HTML's `for` (labels, outputs), `list` (inputs), `form` (form controls), `headers` (cells).
const ID_REFERENCE_ATTRIBUTES: [&str; 12] = [
    "aria-labelledby",
    "aria-describedby",
    "aria-controls",
    "aria-activedescendant",
    "aria-owns",
    "aria-errormessage",
    "aria-details",
    "aria-flowto",
    "for",
    "list",
    "form",
    "headers",
];

/// What the page reported since it loaded, collected by the test-app (`testing/test-app/src/app.rs`).
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
pub struct Diagnostics {
    /// Rust panics.
    pub panics: Vec<String>,
    /// Uncaught errors and unhandled promise rejections, e.g. wasm-bindgen errors thrown from
    /// event handlers.
    pub uncaught_errors: Vec<String>,
    /// Every other `console.error` message.
    pub console_errors: Vec<String>,
    /// `console.warn` messages, e.g. Leptos' warnings about reading a signal outside a reactive
    /// context or after it was disposed.
    pub console_warnings: Vec<String>,
}

/// Defines `diagnostics()`, which reads the page's [`Diagnostics`].
const DIAGNOSTICS: &str = "const diagnostics = () => {
        if (![window.__panics, window.__uncaughtErrors, window.__consoleErrors, window.__consoleWarnings].every(Array.isArray)) throw new Error('test diagnostics are not installed');
        return ({
        panics: [...(window.__panics || [])],
        uncaught_errors: [...(window.__uncaughtErrors || [])],
        console_errors: [...(window.__consoleErrors || [])],
        console_warnings: [...(window.__consoleWarnings || [])],
    }); };";

/// What the page reported since it loaded.
pub async fn diagnostics(session: &Arc<SessionHandle>) -> Result<Diagnostics, Report> {
    let script = format!("{DIAGNOSTICS} return diagnostics();");
    Ok(session
        .execute(&script, vec![])
        .await
        .context("failed to read what the page reported")?
        .convert()?)
}

/// The page's [`Diagnostics`] and the problems of its DOM, read in one script.
#[derive(Debug, Deserialize)]
pub struct Health {
    /// The page's path (`location.pathname`), for fixture policy.
    pub path: String,
    pub diagnostics: Diagnostics,
    /// Ids several elements have, with the elements.
    pub duplicate_ids: Vec<String>,
    /// Id references to no element: the element, the attribute and the id.
    pub dangling_references: Vec<String>,
    /// Literal `attr:` attributes: `attr:` only means something on components; on elements it
    /// becomes an attribute of that name.
    pub literal_attributes: Vec<String>,
}

/// Reads [`Health`]: `arguments[0]` are the [`ID_REFERENCE_ATTRIBUTES`]. References resolve in
/// the element's own root (document or shadow root).
const HEALTH: &str = "const [referenceAttributes] = arguments;
    const elementsById = new Map();
    for (const element of document.querySelectorAll('[id]')) {
        if (!element.id) continue;
        elementsById.set(element.id, [...(elementsById.get(element.id) ?? []), element]);
    }
    const duplicate_ids = [...elementsById]
        .filter(([, elements]) => elements.length > 1)
        .map(([id, elements]) => `#${id}: ${elements.map(describe).join(', ')}`);
    const dangling_references = [];
    const selector = referenceAttributes.map(name => `[${name}]`).join(', ');
    for (const element of document.querySelectorAll(selector)) {
        const root = element.getRootNode();
        for (const name of referenceAttributes) {
            for (const id of (element.getAttribute(name) ?? '').split(/\\s+/).filter(Boolean)) {
                if (!root.getElementById?.(id)) {
                    dangling_references.push(`${describe(element)}: ${name}=${JSON.stringify(id)}`);
                }
            }
        }
    }
    const literal_attributes = [...document.querySelectorAll('*')].flatMap(element =>
        [...element.attributes]
            .filter(attribute => attribute.name.startsWith('attr:'))
            .map(attribute => `${describe(element)}: ${attribute.name}`));
    return {
        path: location.pathname, diagnostics: diagnostics(), duplicate_ids, dangling_references,
        literal_attributes,
    };";

pub async fn read(session: &Arc<SessionHandle>) -> Result<Health, Report> {
    let script = format!("{DESCRIBE} {DIAGNOSTICS} {HEALTH}");
    Ok(session
        .execute(&script, vec![ID_REFERENCE_ATTRIBUTES.into()])
        .await
        .context("failed to collect page health")?
        .convert()?)
}

/// The page panicked (Rust): an error no retry can fix. Waits give up on it
/// through [`expect_no_panic`], and fixture health policy reports it first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PagePanicked {
    /// The panic messages (with their locations).
    pub messages: Vec<String>,
}

impl fmt::Display for PagePanicked {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "the page panicked:\n  - {}",
            self.messages.join("\n  - ")
        )
    }
}

impl std::error::Error for PagePanicked {}

/// The errors that tell whether the page panicked: waits give up on them
/// (`giving_up_on(|error: &Report| error.page_panicked())`).
pub trait PagePanic {
    fn page_panicked(&self) -> bool;
}

impl PagePanic for Report {
    fn page_panicked(&self) -> bool {
        self.iter_reports()
            .any(|report| report.downcast_current_context::<PagePanicked>().is_some())
    }
}

/// Fail with a [`PagePanicked`] if the page panicked.
pub async fn expect_no_panic(session: &Arc<SessionHandle>) -> Result<(), Report> {
    let panics: Vec<String> = session
        .execute("if (!Array.isArray(window.__panics)) throw new Error('test diagnostics are not installed'); return [...window.__panics];", vec![])
        .await
        .context("failed to read the page's panics")?
        .convert()?;
    if panics.is_empty() {
        return Ok(());
    }
    Err(Report::new_sendsync(PagePanicked { messages: panics }).into_dynamic())
}

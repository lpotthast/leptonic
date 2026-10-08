//! The helpers every browser test uses: [`PageActions`] on a page, [`ElementActions`] on an element it
//! found, [`Locator`]s to say which elements, [`SyntheticEvent`]s. Page objects for single
//! fixtures live in the submodules.
//!
//! Every state has the same three methods: read it (`attr`, `prop`, `inner_text`, `count`), wait
//! until it is a value (`wait_for_*`), check that it stays a value (`*_stays`). Lookups take a
//! [`Locator`] and exist on pages and elements alike (`element`, `elements`, `count`,
//! `inner_texts`, `wait_for_count`, `count_stays`).

pub mod dnd;
mod element;
mod event;
pub mod focus_manager;
mod locator;
mod lookup;

use browser_test::{
    StepExt,
    thirtyfour::{TypingData, WebDriver, WebElement},
};
pub use element::{Dispatched, ElementActions};
pub use event::SyntheticEvent;
pub use locator::{Locator, css, role, xpath};
use rootcause::{Report, bail, prelude::ResultExt};
use serde::{Deserialize, de::DeserializeOwned};

use crate::polling::{TIMEOUT, expect, wait_for};

/// A page object without page-specific helpers. Tests that only need [`PageActions`] use this
/// instead of defining their own page type.
pub struct Page<'d> {
    pub driver: &'d WebDriver,
    pub base_url: &'d str,
}

impl PageActions for Page<'_> {
    fn driver(&self) -> &WebDriver {
        self.driver
    }

    fn base_url(&self) -> &str {
        self.base_url
    }
}

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
    /// `console.warn` messages.
    pub console_warnings: Vec<String>,
}

/// Shared page-object actions. Page objects only need to provide the driver and base URL.
///
/// Lookups wait for their element with thirtyfour's element query (see `crate::polling`).
/// Navigations, lookups and waits run as `browser_test` steps, which failure reports list.
#[allow(dead_code)] // Not every test binary uses every helper.
pub trait PageActions {
    fn driver(&self) -> &WebDriver;
    fn base_url(&self) -> &str;

    // Navigation and the page's own reports.

    /// Navigate to `path` and wait until the test-app finished hydrating, so that event handlers
    /// are attached before the test starts interacting with the page.
    async fn goto_path(&self, path: &str) -> Result<(), Report> {
        // The page we leave must not have reported errors.
        self.expect_no_page_errors().await?;
        let url = format!("{}{path}", self.base_url());
        self.driver()
            .goto(&url)
            .step("navigate")
            .detail(path)
            .await
            .context_with(|| format!("failed to go to {url}"))?;
        if let Err(error) = self
            .element("body[data-hydrated]")
            .step("wait_for_hydration")
            .detail(path)
            .await
        {
            // Usually a panic while hydrating: report what the page caught.
            let diagnostics = self.diagnostics().await.unwrap_or_default();
            return Err(error
                .context(format!(
                    "{url} did not finish hydrating; the page reported {diagnostics:#?}"
                ))
                .into_dynamic());
        }
        Ok(())
    }

    /// What the page reported since it loaded.
    async fn diagnostics(&self) -> Result<Diagnostics, Report> {
        self.eval(
            "return {
                 panics: window.__panics || [],
                 uncaught_errors: window.__uncaughtErrors || [],
                 console_errors: window.__consoleErrors || [],
                 console_warnings: window.__consoleWarnings || [],
             };",
            vec![],
        )
        .await
    }

    /// Forget what the page reported so far, e.g. after checking an expected warning.
    async fn clear_diagnostics(&self) -> Result<(), Report> {
        self.eval::<()>(
            "for (const list of [window.__panics, window.__uncaughtErrors, window.__consoleErrors,
                                 window.__consoleWarnings]) {
                 if (list) list.length = 0;
             }",
            vec![],
        )
        .await
    }

    /// Fail if the page panicked, threw uncaught errors, logged errors, or has elements with
    /// literal `attr:` attributes. Runs after every test and before every navigation.
    async fn expect_no_page_errors(&self) -> Result<(), Report> {
        async {
            let Diagnostics {
                panics,
                uncaught_errors,
                console_errors,
                ..
            } = self.diagnostics().await?;
            let mut problems = Vec::new();
            for (what, messages) in [
                ("panicked", panics),
                ("threw uncaught errors", uncaught_errors),
                ("logged errors", console_errors),
            ] {
                if !messages.is_empty() {
                    problems.push(format!("the page {what}:\n  - {}", messages.join("\n  - ")));
                }
            }
            // `attr:` only means something on components; on elements it becomes a literal attribute.
            let literal: Vec<String> = self
                .eval(
                    "return [...document.querySelectorAll('*')]
                         .flatMap(e => [...e.attributes].map(a => a.name))
                         .filter(n => n.startsWith('attr:'));",
                    vec![],
                )
                .await?;
            if !literal.is_empty() {
                problems.push(format!(
                    "elements have literal `attr:` attributes: {literal:?}"
                ));
            }
            if !problems.is_empty() {
                bail!("{}", problems.join("\n"));
            }
            Ok(())
        }
        .step("check_page_errors")
        .await
    }

    // Lookups in the page (`ElementActions` has the same below an element). `element` waits until
    // its element exists: the one way to wait for an element to appear. The plural reads
    // (`elements`, `count`, `inner_texts`) read now, so they also see "none"; to wait for a list,
    // `wait_for_count` first.

    /// The first element matching `locator`.
    async fn element(&self, locator: impl Into<Locator> + Send) -> Result<WebElement, Report> {
        lookup::element(&**self.driver(), locator.into()).await
    }

    /// The elements matching `locator` now, in document order (maybe none).
    async fn elements(
        &self,
        locator: impl Into<Locator> + Send,
    ) -> Result<Vec<WebElement>, Report> {
        lookup::elements(&**self.driver(), locator.into()).await
    }

    /// The number of elements matching `locator` now.
    async fn count(&self, locator: impl Into<Locator> + Send) -> Result<usize, Report> {
        lookup::count(&**self.driver(), locator.into()).await
    }

    /// The [inner texts](ElementActions::inner_text) of the elements matching `locator` now.
    async fn inner_texts(&self, locator: impl Into<Locator> + Send) -> Result<Vec<String>, Report> {
        lookup::inner_texts(&**self.driver(), locator.into()).await
    }

    /// Wait until exactly `expected` elements match `locator` (`0`: they are gone).
    async fn wait_for_count(
        &self,
        locator: impl Into<Locator> + Send,
        expected: usize,
    ) -> Result<(), Report> {
        lookup::wait_for_count(&**self.driver(), locator.into(), expected).await
    }

    /// Exactly `expected` elements match `locator`, and keep doing so.
    async fn count_stays(
        &self,
        locator: impl Into<Locator> + Send,
        expected: usize,
    ) -> Result<(), Report> {
        lookup::count_stays(&**self.driver(), locator.into(), expected).await
    }

    // Focus: the focused element, compared by identity.

    /// The focused element (`document.activeElement`; `<body>` when nothing has focus).
    async fn focused_element(&self) -> Result<WebElement, Report> {
        Ok(self
            .driver()
            .active_element()
            .await
            .context("failed to get the focused element")?)
    }

    /// Wait until `element` has focus.
    async fn wait_for_focus(&self, element: &WebElement) -> Result<(), Report> {
        let waited = wait_for("the focus")
            .observing(|| async { Ok(self.focused_element().await? == *element) })
            .to_be_equal_to(true)
            .await;
        if waited.is_err() {
            bail!(
                "focus did not move to {} within {TIMEOUT:?}; it is on {}",
                element.describe().await?,
                self.focused_element().await?.describe().await?
            );
        }
        Ok(())
    }

    /// `element` has focus and keeps it.
    async fn focus_stays(&self, element: &WebElement) -> Result<(), Report> {
        let kept = expect("the focus")
            .observing(|| async { Ok(self.focused_element().await? == *element) })
            .to_stay_equal_to(true)
            .await;
        if kept.is_err() {
            bail!(
                "focus left {}; it is on {}",
                element.describe().await?,
                self.focused_element().await?.describe().await?
            );
        }
        Ok(())
    }

    // Keyboard input, to whatever has focus.

    /// Send `keys` to the focused element. Modifiers are held until the end, so
    /// `Key::Shift + Key::Tab` is Shift+Tab.
    async fn send_keys(&self, keys: impl Into<TypingData> + Send) -> Result<(), Report> {
        self.focused_element()
            .await?
            .send_keys(keys)
            .await
            .context("failed to send keys to the focused element")?;
        Ok(())
    }

    /// Type `text` one key at a time, each to the element focused at that moment: for inputs
    /// that move focus while typing (date segments).
    async fn type_text(&self, text: &str) -> Result<(), Report> {
        for key in text.chars() {
            self.send_keys(key.to_string()).await?;
        }
        Ok(())
    }

    /// Hold `key` on the focused element: a `keydown`, `repeats` repeated `keydown`s and a
    /// `keyup`. WebDriver can't send repeated key events.
    async fn hold_key(&self, key: &str, repeats: usize) -> Result<(), Report> {
        let focused = self.focused_element().await?;
        focused
            .dispatch(SyntheticEvent::keyboard("keydown", key))
            .await?;
        for _ in 0..repeats {
            focused
                .dispatch(SyntheticEvent::keyboard("keydown", key).with("repeat", true))
                .await?;
        }
        focused
            .dispatch(SyntheticEvent::keyboard("keyup", key))
            .await?;
        Ok(())
    }

    /// Take focus away from the focused element (`blur()`).
    async fn blur_focused(&self) -> Result<(), Report> {
        self.eval::<()>("document.activeElement.blur();", vec![])
            .await
    }

    // Scripts: for what WebDriver and the helpers above can't do.

    /// Run `script` in the page and return its result as a `T` (`()` for none). `args` are its
    /// `arguments[n]` (`element.to_json()?` for an element); never format values into the
    /// script. A failure names the script.
    async fn eval<T: DeserializeOwned>(
        &self,
        script: &str,
        args: Vec<serde_json::Value>,
    ) -> Result<T, Report> {
        let summary = || {
            let first_line = script.trim().lines().next().unwrap_or_default();
            format!("the script `{first_line} …` failed")
        };
        Ok(self
            .driver()
            .execute(script, args)
            .await
            .context_with(summary)?
            .convert()
            .context_with(summary)?)
    }
}

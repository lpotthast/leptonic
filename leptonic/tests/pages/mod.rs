//! The helpers every browser test uses: [`Page`] on a page, [`ElementActions`] on an element it
//! found, [`Locator`]s to say which elements, [`SyntheticEvent`]s. Page objects for single
//! fixtures live in `crate::fixtures`.
//!
//! Every state has the same three methods: read it (`attr`, `prop`, `inner_text`, `count`), wait
//! until it is a value (`wait_for_*`), check that it stays a value (`*_stays`). Other values are
//! observed with assertr's eventual assertions (`Patience::global()`). Lookups take a
//! [`Locator`] and exist on pages and elements alike (`element`, `elements`, `count`,
//! `inner_texts`, `wait_for_count`, `count_stays`).

mod element;
mod event;
pub mod health;
mod locator;
mod lookup;
pub(crate) use lookup::terminal_lookup_error;
mod platform;
pub(crate) mod script;
mod stopwatch;

use std::time::Duration;

use assertr::{pattern, prelude::*};
pub use browser_test::thirtyfour::WebElement;
use browser_test::{
    StepExt,
    thirtyfour::{TypingData, WebDriver},
};
pub use element::{Dispatched, ElementActions, ScrollExtent};
pub use event::{
    DeltaMode, DragKind, EventKind, KeyKind, Modifier, MouseButton, MouseKind, PointerKind,
    PointerType, SyntheticEvent, interface,
};
pub use locator::{Locator, css, role};
pub use platform::Platform;
use rootcause::{Report, bail, prelude::ResultExt};
use serde::{Deserialize, de::DeserializeOwned};
pub use stopwatch::{Stopwatch, StopwatchEnd, StopwatchStart};

use crate::pages::lookup::Root;

/// A target of [`Page::dispatch_to`] that is no element.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlobalTarget {
    Window,
    Document,
}

impl GlobalTarget {
    fn as_str(self) -> &'static str {
        match self {
            Self::Window => "window",
            Self::Document => "document",
        }
    }
}

/// A page object without page-specific helpers. Tests that only need [`Page`] use this
/// instead of defining their own page type.
pub struct Page<'d> {
    driver: &'d WebDriver,
    base_url: &'d str,
}

impl<'d> Page<'d> {
    pub(crate) fn new(driver: &'d WebDriver, base_url: &'d str) -> Self {
        Self { driver, base_url }
    }
    pub(crate) fn driver(&self) -> &WebDriver {
        self.driver
    }
    pub fn base_url(&self) -> &str {
        self.base_url
    }

    /// Explicit escape hatch for protocol operations and fixture instrumentation.
    pub fn low_level(&self) -> LowLevel<'_> {
        LowLevel {
            driver: self.driver,
        }
    }

    // Lookups in the page (`ElementActions` has the same below an element). `element` waits until
    // its element exists: the one way to wait for an element to appear. The plural reads
    // (`elements`, `count`, `inner_texts`) read now, so they also see "none"; to wait for a list,
    // `wait_for_count` first.

    /// Exactly one element matching `locator`; waits for absence and rejects ambiguity.
    pub async fn element(&self, locator: impl Into<Locator> + Send) -> Result<WebElement, Report> {
        lookup::element(Root::Page(self.driver()), locator.into()).await
    }

    /// The elements matching `locator` now, in document order (maybe none).
    /// Explicitly choose the first match in document order.
    pub async fn first_element(
        &self,
        locator: impl Into<Locator> + Send,
    ) -> Result<WebElement, Report> {
        lookup::first(Root::Page(self.driver()), locator.into()).await
    }

    pub async fn elements(
        &self,
        locator: impl Into<Locator> + Send,
    ) -> Result<Vec<WebElement>, Report> {
        lookup::elements(Root::Page(self.driver()), locator.into()).await
    }

    /// The number of elements matching `locator` now.
    pub async fn count(&self, locator: impl Into<Locator> + Send) -> Result<usize, Report> {
        lookup::count(Root::Page(self.driver()), locator.into()).await
    }

    /// The [inner texts](ElementActions::inner_text) of the elements matching `locator` now.
    pub async fn inner_texts(
        &self,
        locator: impl Into<Locator> + Send,
    ) -> Result<Vec<String>, Report> {
        lookup::inner_texts(Root::Page(self.driver()), locator.into()).await
    }

    /// Wait until exactly `expected` elements match `locator` (`0`: they are gone).
    #[track_caller]
    pub fn wait_for_count(
        &self,
        locator: impl Into<Locator> + Send,
        expected: usize,
    ) -> impl Future<Output = Result<(), Report>> {
        lookup::wait_for_count(Root::Page(self.driver()), locator.into(), expected)
    }

    /// Exactly `expected` elements match `locator`, and keep doing so.
    #[track_caller]
    pub fn count_stays(
        &self,
        locator: impl Into<Locator> + Send,
        expected: usize,
        duration: Duration,
    ) -> impl Future<Output = Result<(), Report>> {
        lookup::count_stays(
            Root::Page(self.driver()),
            locator.into(),
            expected,
            duration,
        )
    }

    /// Wait using a fresh lookup every time, so replacement of the node is supported.
    #[track_caller]
    pub fn wait_for_attr<'a>(
        &'a self,
        locator: impl Into<Locator>,
        name: &'a str,
        expected: Option<&'a str>,
    ) -> impl Future<Output = Result<(), Report>> + 'a {
        lookup::wait_for_attr(Root::Page(self.driver()), locator.into(), name, expected)
    }

    // Focus: the focused element, compared by identity.

    /// The deeply focused element, including inside open shadow roots (`<body>` when none).
    pub async fn focused_element(&self) -> Result<WebElement, Report> {
        self.driver()
            .execute(
                "let element = document.activeElement;
                 while (element?.shadowRoot?.activeElement) element = element.shadowRoot.activeElement;
                 return element;",
                vec![],
            )
            .await
            .context("failed to get the focused element")?
            .element()
            .map_err(Into::into)
    }

    /// Wait for this specific node to receive focus. A detached node fails immediately.
    #[track_caller]
    pub fn wait_for_focus<'a>(
        &'a self,
        element: &'a WebElement,
    ) -> impl Future<Output = Result<(), Report>> + 'a {
        let check = assert_that_owned!(move || async move {
            health::expect_no_panic(self.driver()).await?;
            focus(self, element).await
        })
        .with_subject_name("focus")
        .eventually_ok()
        .giving_up_on_any_error()
        .try_matches(pattern!(Focus::OnTarget));
        async move {
            check.await?;
            Ok(())
        }
        .step("wait_for_focus")
    }

    /// Sample focus immediately and throughout `duration`; does not settle first.
    #[track_caller]
    pub fn focus_stays<'a>(
        &'a self,
        element: &'a WebElement,
        duration: Duration,
    ) -> impl Future<Output = Result<(), Report>> + 'a {
        let check = assert_that_owned!(move || async move {
            health::expect_no_panic(self.driver()).await?;
            focus(self, element).await
        })
        .with_subject_name("focus")
        .consistently_ok()
        .for_at_least(duration)
        .try_matches(pattern!(Focus::OnTarget));
        async move {
            if duration.is_zero() {
                bail!("stability checks require a positive duration");
            }
            check.await?;
            Ok(())
        }
        .step("focus_stays")
    }

    /// Let two animation frames and a task pass. This does not establish quiescence or
    /// replace a wait for the state under test. Stability checks observe without this barrier.
    pub async fn settle(&self) -> Result<(), Report> {
        script::settle(self.driver()).await
    }

    /// Release held input and browser instrumentation after a case, also on failure.
    pub(crate) async fn cleanup(&self) -> Result<(), Report> {
        let input = self.driver().action_chain().reset_actions().await;
        let recording = self.low_level().eval::<()>(
            "for (const recording of window.__attrRecordings?.values() ?? []) recording.observer.disconnect();
             window.__attrRecordings?.clear();
             for (const stopwatch of window.__stopwatches?.values() ?? []) stopwatch.cancel();
             window.__stopwatches?.clear();", vec![]).await;
        input?;
        recording?;
        Ok(())
    }

    // The browser.

    /// Make the browser report `platform` (user agent, `navigator.platform`) from the next page
    /// load on: leptonic detects the platform once per page. Session resets end the emulation.
    pub async fn emulate_platform(&self, platform: Platform) -> Result<(), Report> {
        self.driver()
            .cdp()
            .send(platform.user_agent_override())
            .await
            .context_with(|| format!("failed to emulate {platform:?}"))?;
        Ok(())
    }

    /// Start measuring in the page, on its clock: from the next `event` (a pointer event, or a
    /// plain one like `scroll`) on `target` to `end` (an element appearing or disappearing, an
    /// attribute changing). For behavior behind a timer that must not
    /// happen before it ran out (a tooltip's delay), where WebDriver round trips would blur the
    /// measurement.
    pub async fn start_stopwatch(
        &self,
        target: &WebElement,
        event: impl Into<StopwatchStart>,
        end: StopwatchEnd<'_>,
    ) -> Result<Stopwatch, Report> {
        stopwatch::start(self.driver(), target, event.into(), end).await
    }

    /// Click with the mouse at the point (`x`, `y`) of the viewport (CSS pixels), on whatever is
    /// there: an outside click.
    pub async fn click_at(&self, x: i64, y: i64) -> Result<(), Report> {
        self.driver()
            .action_chain()
            .move_to(x, y)
            .click()
            .perform()
            .await
            .context_with(|| format!("failed to click at ({x}, {y})"))?;
        Ok(())
    }

    /// Dispatch `event` to the window or the document (a `resize` of the window, a `scroll` of the
    /// document), which have no element to dispatch to.
    pub async fn dispatch_to<I: Send>(
        &self,
        target: GlobalTarget,
        event: SyntheticEvent<I>,
    ) -> Result<Dispatched, Report> {
        let description = event.to_string();
        let (interface, kind, init) = event.into_parts();
        let default_prevented = self
            .low_level()
            .eval(
                "const [target, constructor, type, init] = arguments;
                 const event = new window[constructor](type, init);
                 (target === 'window' ? window : document).dispatchEvent(event);
                 return event.defaultPrevented;",
                vec![target.as_str().into(), interface.into(), kind.into(), init],
            )
            .await
            .context_with(|| format!("failed to dispatch {description} to the {target:?}"))?;
        Ok(Dispatched { default_prevented })
    }

    /// Start recording the attribute `name` of the elements matching `selector`, also of those
    /// inserted later (an overlay opening): the value each had when inserted and every value it
    /// took after, in the page (no WebDriver round trips in between). `finish()` returns them.
    pub async fn record_attr_of(
        &self,
        selector: &str,
        name: &str,
    ) -> Result<element::AttrRecording, Report> {
        element::record_attr_of(self.driver(), selector, name).await
    }

    // Keyboard input, to whatever has focus.

    /// Send `keys` to the focused element. Modifiers are held until the end, so
    /// `Key::Shift + Key::Tab` is Shift+Tab.
    pub async fn send_keys(&self, keys: impl Into<TypingData> + Send) -> Result<(), Report> {
        self.focused_element()
            .await?
            .send_keys(keys)
            .await
            .context("failed to send keys to the focused element")?;
        Ok(())
    }

    /// Type `text` one key at a time, each to the element focused at that moment: for inputs
    /// that move focus while typing (date segments).
    pub async fn type_text(&self, text: &str) -> Result<(), Report> {
        for key in text.chars() {
            self.send_keys(key.to_string()).await?;
        }
        Ok(())
    }

    /// Hold `key` on the focused element: a `keydown`, `repeats` repeated `keydown`s and a
    /// `keyup`. WebDriver can't send repeated key events.
    pub async fn hold_key(&self, key: &str, repeats: usize) -> Result<(), Report> {
        let focused = self.focused_element().await?;
        focused
            .dispatch(SyntheticEvent::keyboard(KeyKind::Down, key))
            .await?;
        for _ in 0..repeats {
            focused
                .dispatch(SyntheticEvent::keyboard(KeyKind::Down, key).repeat(true))
                .await?;
        }
        focused
            .dispatch(SyntheticEvent::keyboard(KeyKind::Up, key))
            .await?;
        Ok(())
    }

    /// Take focus away from the focused element (`blur()`).
    pub async fn blur_focused(&self) -> Result<(), Report> {
        self.low_level()
            .eval::<()>("document.activeElement.blur();", vec![])
            .await
    }
}

/// Low-level operations are deliberately visible at call sites; normal actions use Page/WebElement.
#[derive(Clone, Copy)]
pub struct LowLevel<'a> {
    driver: &'a WebDriver,
}
impl LowLevel<'_> {
    pub fn driver(&self) -> &WebDriver {
        self.driver
    }
    // Scripts: for what WebDriver and the helpers above can't do.

    /// Run `script` in the page and return its result as a `T` (`()` for none). `args` are its
    /// `arguments[n]` (`element.to_json()?` for an element); never format values into the
    /// script. A failure names the script.
    pub async fn eval<T: DeserializeOwned>(
        self,
        script: &str,
        args: Vec<serde_json::Value>,
    ) -> Result<T, Report> {
        script::eval(self.driver, script, args).await
    }

    /// Run the asynchronous `script` in the page: it finishes by calling its last argument
    /// (`arguments[arguments.length - 1]`, after `args`) with its result, which is returned as a
    /// `T`. For what takes time in the page: waiting for events, pacing steps with timers. A
    /// failure names the script.
    pub async fn eval_async<T: DeserializeOwned>(
        self,
        script: &str,
        args: Vec<serde_json::Value>,
    ) -> Result<T, Report> {
        script::eval_async(self.driver, script, args).await
    }
}

/// Where the focus is, as focus checks observe it: the elements are described (in the same
/// script) only when the focus is elsewhere, for the failure report.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[expect(
    dead_code,
    reason = "the descriptions are read through `Debug`: failure reports show them"
)]
enum Focus {
    /// On the element the check expects it on.
    OnTarget,
    /// On another element ([described](ElementActions::describe)).
    Elsewhere { focused: String, target: String },
}

/// One script per observation: compares the focused element with the target by identity.
async fn focus(page: &Page<'_>, target: &WebElement) -> Result<Focus, Report> {
    let focused = page.driver().active_element().await?;
    if focused == *target {
        Ok(Focus::OnTarget)
    } else {
        Ok(Focus::Elsewhere {
            focused: focused.describe().await?,
            target: target.describe().await?,
        })
    }
}

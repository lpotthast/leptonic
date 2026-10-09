//! Everything a test does with an element it found: look up below it, read, wait, check
//! stability, act.

use assertr::{matchers::eq, prelude::*};
use browser_test::thirtyfour::{By, WebElement, error::WebDriverErrorInner, prelude::*};
use rootcause::{Report, prelude::ResultExt};
use serde::{Deserialize, de::DeserializeOwned};

use crate::{
    pages::{Locator, event::SyntheticEvent, lookup},
    timing,
};

/// The rectangle of an element in CSS pixels (`getBoundingClientRect()`), not rounded like
/// WebDriver's `WebElement::rect`.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct ClientRect {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub width: f64,
    pub height: f64,
}

/// The outcome of [`ElementActions::dispatch`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dispatched {
    /// Whether a handler called `preventDefault()`.
    pub default_prevented: bool,
}

/// What [`ElementActions`] adds to thirtyfour's `WebElement`. Together with `WebElement`'s own
/// methods (`attr`, `prop`, `is_selected`, `is_enabled`, `is_displayed`, `click`, `focus`,
/// `send_keys`, `scroll_into_view`), this is the one way to do each thing with an element. Never
/// `WebElement::text` (it skips `display: contents` children: use [`Self::inner_text`]) or
/// `find`/`query` (use [`Self::element`]).
#[allow(dead_code)] // Not every test binary uses every helper.
pub trait ElementActions {
    // Lookups below the element (`PageActions` has the same in the page).

    /// The first element below this one matching `locator`, once it exists.
    async fn element(&self, locator: impl Into<Locator> + Send) -> Result<WebElement, Report>;

    /// The elements below this one matching `locator` now, in document order (maybe none).
    async fn elements(&self, locator: impl Into<Locator> + Send)
    -> Result<Vec<WebElement>, Report>;

    /// The number of elements below this one matching `locator` now.
    async fn count(&self, locator: impl Into<Locator> + Send) -> Result<usize, Report>;

    /// The [inner texts](Self::inner_text) of the elements below this one matching `locator`
    /// now.
    async fn inner_texts(&self, locator: impl Into<Locator> + Send) -> Result<Vec<String>, Report>;

    /// Wait until exactly `expected` elements below this one match `locator`.
    #[track_caller]
    fn wait_for_count(
        &self,
        locator: impl Into<Locator> + Send,
        expected: usize,
    ) -> impl Future<Output = Result<(), Report>>;

    /// Exactly `expected` elements below this one match `locator`, and keep doing so.
    #[track_caller]
    fn count_stays(
        &self,
        locator: impl Into<Locator> + Send,
        expected: usize,
    ) -> impl Future<Output = Result<(), Report>>;

    // Reads (thirtyfour has `attr` and `prop`).

    /// The text the element shows: `innerText`, trimmed. The tests' one notion of text.
    async fn inner_text(&self) -> Result<String, Report>;

    /// The text of the elements the id list in the attribute `attr` refers to (e.g.
    /// `aria-describedby`), joined with spaces. Their text content: a referenced element counts for
    /// the accessible name or description also while hidden.
    async fn referenced_text(&self, attr: &str) -> Result<String, Report>;

    /// Whether the form control passes native validation (`validity.valid`; fires nothing).
    async fn is_valid(&self) -> Result<bool, Report>;

    /// The rectangle in CSS pixels, unrounded.
    async fn client_rect(&self) -> Result<ClientRect, Report>;

    /// Names the element in messages: `<span id="x">` by its id, otherwise by tag, ARIA role,
    /// label and the start of its text (`<div role="row"> "Inbox"`).
    async fn describe(&self) -> Result<String, Report>;

    // Waits: until a state is reached, failing after the timeout (`crate::timing`) with the values
    // it went through, or at once when the page removed the element. Waits and stays are
    // `#[track_caller]` and build their assertion when called, so that a failure names the test's
    // line.

    /// Wait until the attribute `name` is `expected` (`None`: absent).
    #[track_caller]
    fn wait_for_attr<'a>(
        &'a self,
        name: &'a str,
        expected: Option<&'a str>,
    ) -> impl Future<Output = Result<(), Report>> + 'a;

    /// Wait until the DOM property `name` is `expected`, e.g. an input's `value` (which the
    /// `value` attribute doesn't follow once the user typed).
    #[track_caller]
    fn wait_for_prop<'a>(
        &'a self,
        name: &'a str,
        expected: &'a str,
    ) -> impl Future<Output = Result<(), Report>> + 'a;

    /// Wait until the [inner text](Self::inner_text) is `expected`.
    #[track_caller]
    fn wait_for_inner_text(&self, expected: &str) -> impl Future<Output = Result<(), Report>>;

    // Stays: once the page settled, the state holds ("nothing happens", `crate::timing`).

    /// The attribute `name` is `expected` (`None`: absent) and stays so.
    #[track_caller]
    fn attr_stays<'a>(
        &'a self,
        name: &'a str,
        expected: Option<&'a str>,
    ) -> impl Future<Output = Result<(), Report>> + 'a;

    /// The DOM property `name` is `expected` and stays so.
    #[track_caller]
    fn prop_stays<'a>(
        &'a self,
        name: &'a str,
        expected: &'a str,
    ) -> impl Future<Output = Result<(), Report>> + 'a;

    /// The [inner text](Self::inner_text) is `expected` and stays so.
    #[track_caller]
    fn inner_text_stays(&self, expected: &str) -> impl Future<Output = Result<(), Report>>;

    // Actions beyond thirtyfour's `click`, `focus`, `send_keys` and `scroll_into_view`.

    /// Move the pointer to the element's center.
    async fn hover(&self) -> Result<(), Report>;

    /// Click from script (`element.click()`), as assistive technology does: no pointer events,
    /// `detail` 0.
    async fn virtual_click(&self) -> Result<(), Report>;

    /// Set the `value` property from script and fire `input`, as assistive technology does (e.g.
    /// a screen reader adjusting a slider).
    async fn virtual_input(&self, value: &str) -> Result<(), Report>;

    /// Dispatch a synthetic `event` on the element: for events WebDriver can't produce (touch and
    /// pen pointers, repeated keys, wheel, drag, `beforematch`, ...).
    async fn dispatch(&self, event: SyntheticEvent) -> Result<Dispatched, Report>;

    /// Scroll the element's content to `top` (CSS pixels).
    async fn scroll_to_top(&self, top: f64) -> Result<(), Report>;

    // Forms.

    /// Run native validation of the form or control (`checkValidity()`, which fires `invalid` on
    /// every invalid control) and return whether it passed.
    async fn check_validity(&self) -> Result<bool, Report>;

    /// The values the form submits for `name` (`FormData.getAll`).
    async fn form_values(&self, name: &str) -> Result<Vec<String>, Report>;

    /// Submit the form as a submit button does (`requestSubmit()`: validates, fires `submit`).
    async fn submit(&self) -> Result<(), Report>;

    /// Reset the form (`reset()`).
    async fn reset(&self) -> Result<(), Report>;
}

impl ElementActions for WebElement {
    async fn element(&self, locator: impl Into<Locator> + Send) -> Result<WebElement, Report> {
        lookup::element(self, locator.into()).await
    }

    async fn elements(
        &self,
        locator: impl Into<Locator> + Send,
    ) -> Result<Vec<WebElement>, Report> {
        lookup::elements(self, locator.into()).await
    }

    async fn count(&self, locator: impl Into<Locator> + Send) -> Result<usize, Report> {
        lookup::count(self, locator.into()).await
    }

    async fn inner_texts(&self, locator: impl Into<Locator> + Send) -> Result<Vec<String>, Report> {
        lookup::inner_texts(self, locator.into()).await
    }

    #[track_caller]
    fn wait_for_count(
        &self,
        locator: impl Into<Locator> + Send,
        expected: usize,
    ) -> impl Future<Output = Result<(), Report>> {
        lookup::wait_for_count(self, locator.into(), expected)
    }

    #[track_caller]
    fn count_stays(
        &self,
        locator: impl Into<Locator> + Send,
        expected: usize,
    ) -> impl Future<Output = Result<(), Report>> {
        lookup::count_stays(self.handle(), self, locator.into(), expected)
    }

    async fn inner_text(&self) -> Result<String, Report> {
        Ok(self
            .prop("innerText")
            .await
            .context("failed to read the element's inner text")?
            .unwrap_or_default()
            .trim()
            .to_owned())
    }

    async fn referenced_text(&self, attr: &str) -> Result<String, Report> {
        let ids = self.attr(attr).await?.unwrap_or_default();
        let mut texts = Vec::new();
        for id in ids.split_whitespace() {
            let referenced = self
                .handle()
                .query(By::Id(id))
                .first()
                .await
                .context_with(|| format!("{attr} refers to #{id}, which doesn't exist"))?;
            let text = referenced.prop("textContent").await?;
            texts.push(text.unwrap_or_default().trim().to_owned());
        }
        Ok(texts.join(" "))
    }

    async fn is_valid(&self) -> Result<bool, Report> {
        script(self, "return arguments[0].validity.valid;", vec![]).await
    }

    async fn client_rect(&self) -> Result<ClientRect, Report> {
        script(
            self,
            "const r = arguments[0].getBoundingClientRect();
             return {left: r.left, top: r.top, right: r.right, bottom: r.bottom,
                     width: r.width, height: r.height};",
            vec![],
        )
        .await
    }

    async fn describe(&self) -> Result<String, Report> {
        let tag = self.tag_name().await?;
        if let Some(id) = self.id().await?.filter(|id| !id.is_empty()) {
            return Ok(format!("<{tag} id={id:?}>"));
        }
        if tag == "body" || tag == "html" {
            return Ok(format!("<{tag}>"));
        }
        let role = self
            .attr("role")
            .await?
            .map(|role| format!(" role={role:?}"))
            .unwrap_or_default();
        let label = self
            .attr("aria-label")
            .await?
            .map(|label| format!(" aria-label={label:?}"))
            .unwrap_or_default();
        let text = self.inner_text().await?;
        let mut text = text.split_whitespace().collect::<Vec<_>>().join(" ");
        if text.chars().count() > 60 {
            text = text.chars().take(60).chain("…".chars()).collect();
        }
        Ok(format!("<{tag}{role}{label}> {text:?}"))
    }

    #[track_caller]
    fn wait_for_attr<'a>(
        &'a self,
        name: &'a str,
        expected: Option<&'a str>,
    ) -> impl Future<Output = Result<(), Report>> + 'a {
        let check = assert_that_owned!(move || self.attr(name))
            .with_subject_name(format!("attribute {name}"))
            .eventually_ok()
            .giving_up_on(RemovedElement::element_was_removed)
            .matches(eq(expected.map(str::to_owned)));
        async move {
            check.await;
            Ok(())
        }
    }

    #[track_caller]
    fn wait_for_prop<'a>(
        &'a self,
        name: &'a str,
        expected: &'a str,
    ) -> impl Future<Output = Result<(), Report>> + 'a {
        let check = assert_that_owned!(move || self.prop(name))
            .with_subject_name(format!("property {name}"))
            .eventually_ok()
            .giving_up_on(RemovedElement::element_was_removed)
            .matches(eq(Some(expected.to_owned())));
        async move {
            check.await;
            Ok(())
        }
    }

    #[track_caller]
    fn wait_for_inner_text(&self, expected: &str) -> impl Future<Output = Result<(), Report>> {
        let check = assert_that_owned!(move || self.inner_text())
            .eventually_ok()
            .giving_up_on(RemovedElement::element_was_removed)
            .matches(eq(expected.to_owned()));
        async move {
            check.await;
            Ok(())
        }
    }

    #[track_caller]
    fn attr_stays<'a>(
        &'a self,
        name: &'a str,
        expected: Option<&'a str>,
    ) -> impl Future<Output = Result<(), Report>> + 'a {
        let check = assert_that_owned!(move || self.attr(name))
            .with_subject_name(format!("attribute {name}"))
            .consistently_ok()
            .matches(eq(expected.map(str::to_owned)));
        async move {
            timing::settle(self.handle()).await?;
            check.await;
            Ok(())
        }
    }

    #[track_caller]
    fn prop_stays<'a>(
        &'a self,
        name: &'a str,
        expected: &'a str,
    ) -> impl Future<Output = Result<(), Report>> + 'a {
        let check = assert_that_owned!(move || self.prop(name))
            .with_subject_name(format!("property {name}"))
            .consistently_ok()
            .matches(eq(Some(expected.to_owned())));
        async move {
            timing::settle(self.handle()).await?;
            check.await;
            Ok(())
        }
    }

    #[track_caller]
    fn inner_text_stays(&self, expected: &str) -> impl Future<Output = Result<(), Report>> {
        let check = assert_that_owned!(move || self.inner_text())
            .consistently_ok()
            .matches(eq(expected.to_owned()));
        async move {
            timing::settle(self.handle()).await?;
            check.await;
            Ok(())
        }
    }

    async fn hover(&self) -> Result<(), Report> {
        self.handle()
            .action_chain()
            .move_to_element_center(self)
            .perform()
            .await
            .context("failed to move the pointer to the element")?;
        Ok(())
    }

    async fn virtual_click(&self) -> Result<(), Report> {
        script(self, "arguments[0].click();", vec![]).await
    }

    async fn virtual_input(&self, value: &str) -> Result<(), Report> {
        script(
            self,
            "arguments[0].value = arguments[1];
             arguments[0].dispatchEvent(new Event('input', {bubbles: true}));",
            vec![value.into()],
        )
        .await
    }

    async fn dispatch(&self, event: SyntheticEvent) -> Result<Dispatched, Report> {
        let description = event.to_string();
        let (interface, kind, init) = event.into_parts();
        let default_prevented = script(
            self,
            "const [target, constructor, type, init] = arguments;
             const event = new window[constructor](type, init);
             target.dispatchEvent(event);
             return event.defaultPrevented;",
            vec![interface.into(), kind.into(), init],
        )
        .await
        .context_with(|| format!("failed to dispatch {description}"))?;
        Ok(Dispatched { default_prevented })
    }

    async fn scroll_to_top(&self, top: f64) -> Result<(), Report> {
        script(
            self,
            "arguments[0].scrollTop = arguments[1];",
            vec![top.into()],
        )
        .await
    }

    async fn check_validity(&self) -> Result<bool, Report> {
        script(self, "return arguments[0].checkValidity();", vec![]).await
    }

    async fn form_values(&self, name: &str) -> Result<Vec<String>, Report> {
        script(
            self,
            "return new FormData(arguments[0]).getAll(arguments[1]);",
            vec![name.into()],
        )
        .await
    }

    async fn submit(&self) -> Result<(), Report> {
        script(self, "arguments[0].requestSubmit();", vec![]).await
    }

    async fn reset(&self) -> Result<(), Report> {
        script(self, "arguments[0].reset();", vec![]).await
    }
}

/// Run `script` with the element as `arguments[0]` and `args` after it, and return its result as
/// a `T` (`()` for none). A failure names the script.
async fn script<T: DeserializeOwned>(
    element: &WebElement,
    script: &str,
    args: Vec<serde_json::Value>,
) -> Result<T, Report> {
    let summary = || {
        let first_line = script.trim().lines().next().unwrap_or_default();
        format!("the script `{first_line} …` failed")
    };
    let mut arguments = vec![element.to_json()?];
    arguments.extend(args);
    Ok(element
        .handle()
        .execute(script, arguments)
        .await
        .context_with(summary)?
        .convert()
        .context_with(summary)?)
}

/// The errors of reads that tell whether the page removed the element read (e.g. re-rendered
/// it): no retry reads it again, so waits give up at once (look the element up after the change).
pub trait RemovedElement {
    fn element_was_removed(&self) -> bool;
}

impl RemovedElement for WebDriverError {
    fn element_was_removed(&self) -> bool {
        matches!(
            self.as_inner(),
            WebDriverErrorInner::StaleElementReference(_)
        )
    }
}

impl RemovedElement for Report {
    fn element_was_removed(&self) -> bool {
        self.iter_reports().any(|report| {
            report
                .downcast_current_context::<WebDriverError>()
                .is_some_and(RemovedElement::element_was_removed)
        })
    }
}

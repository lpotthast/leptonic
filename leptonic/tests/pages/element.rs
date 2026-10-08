//! Everything a test does with an element it found: look up below it, read, wait, check
//! stability, act.

use std::sync::Arc;

use browser_test::{
    StepExt,
    thirtyfour::{By, WebElement, prelude::*},
};
use rootcause::{Report, bail, prelude::ResultExt};
use serde::{Deserialize, de::DeserializeOwned};

use crate::{
    pages::{Locator, event::SyntheticEvent, lookup},
    polling::{TIMEOUT, expect},
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
    async fn wait_for_count(
        &self,
        locator: impl Into<Locator> + Send,
        expected: usize,
    ) -> Result<(), Report>;

    /// Exactly `expected` elements below this one match `locator`, and keep doing so.
    async fn count_stays(
        &self,
        locator: impl Into<Locator> + Send,
        expected: usize,
    ) -> Result<(), Report>;

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

    // Waits: until a state is reached, failing after `polling::TIMEOUT` with the state it was in.

    /// Wait until the attribute `name` is `expected` (`None`: absent).
    async fn wait_for_attr(&self, name: &str, expected: Option<&str>) -> Result<(), Report>;

    /// Wait until the DOM property `name` is `expected`, e.g. an input's `value` (which the
    /// `value` attribute doesn't follow once the user typed).
    async fn wait_for_prop(&self, name: &str, expected: &str) -> Result<(), Report>;

    /// Wait until the [inner text](Self::inner_text) is `expected`.
    async fn wait_for_inner_text(&self, expected: &str) -> Result<(), Report>;

    // Stays: the state holds now and for `polling::STAYS` ("nothing happens").

    /// The attribute `name` is `expected` (`None`: absent) and stays so.
    async fn attr_stays(&self, name: &str, expected: Option<&str>) -> Result<(), Report>;

    /// The DOM property `name` is `expected` and stays so.
    async fn prop_stays(&self, name: &str, expected: &str) -> Result<(), Report>;

    /// The [inner text](Self::inner_text) is `expected` and stays so.
    async fn inner_text_stays(&self, expected: &str) -> Result<(), Report>;

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

    async fn wait_for_count(
        &self,
        locator: impl Into<Locator> + Send,
        expected: usize,
    ) -> Result<(), Report> {
        lookup::wait_for_count(self, locator.into(), expected).await
    }

    async fn count_stays(
        &self,
        locator: impl Into<Locator> + Send,
        expected: usize,
    ) -> Result<(), Report> {
        lookup::count_stays(self, locator.into(), expected).await
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

    async fn wait_for_attr(&self, name: &str, expected: Option<&str>) -> Result<(), Report> {
        let attr: Arc<str> = name.into();
        let wanted = expected.map(str::to_owned);
        let waited = self
            .wait_until()
            .ignore_errors(false)
            .condition(move |element: WebElement| {
                let attr = attr.clone();
                let wanted = wanted.clone();
                async move { Ok(element.attr(&attr).await? == wanted) }
            })
            .step("wait_for_attr")
            .detail(format!("{name} = {}", shown(expected)))
            .await;
        if waited.is_err() {
            let actual = self.attr(name).await;
            explain_timeout(self, &format!("attribute {name}"), expected, actual).await?;
        }
        Ok(())
    }

    async fn wait_for_prop(&self, name: &str, expected: &str) -> Result<(), Report> {
        let waited = self
            .wait_until()
            .ignore_errors(false)
            .has_property(name, expected.to_owned())
            .step("wait_for_prop")
            .detail(format!("{name} = {expected:?}"))
            .await;
        if waited.is_err() {
            let actual = self.prop(name).await;
            explain_timeout(self, &format!("property {name}"), Some(expected), actual).await?;
        }
        Ok(())
    }

    async fn wait_for_inner_text(&self, expected: &str) -> Result<(), Report> {
        let wanted = expected.to_owned();
        let waited = self
            .wait_until()
            .ignore_errors(false)
            .condition(move |element: WebElement| {
                let wanted = wanted.clone();
                async move {
                    let text = element.prop("innerText").await?.unwrap_or_default();
                    Ok(text.trim() == wanted)
                }
            })
            .step("wait_for_inner_text")
            .detail(format!("{expected:?}"))
            .await;
        if waited.is_err() {
            let actual = self.prop("innerText").await;
            let actual = actual.map(|text| text.map(|text| text.trim().to_owned()));
            explain_timeout(self, "the inner text", Some(expected), actual).await?;
        }
        Ok(())
    }

    async fn attr_stays(&self, name: &str, expected: Option<&str>) -> Result<(), Report> {
        expect(format!("attribute {name} of {}", self.describe().await?))
            .observing(|| async { Ok(self.attr(name).await?) })
            .to_stay_equal_to(expected.map(str::to_owned))
            .await
    }

    async fn prop_stays(&self, name: &str, expected: &str) -> Result<(), Report> {
        expect(format!("property {name} of {}", self.describe().await?))
            .observing(|| async { Ok(self.prop(name).await?) })
            .to_stay_equal_to(Some(expected.to_owned()))
            .await
    }

    async fn inner_text_stays(&self, expected: &str) -> Result<(), Report> {
        expect(format!("the inner text of {}", self.describe().await?))
            .observing(|| self.inner_text())
            .to_stay_equal_to(expected)
            .await
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

/// How a value appears in messages: quoted, or `absent` (an attribute that isn't set).
fn shown(value: Option<&str>) -> String {
    value.map_or_else(|| "absent".to_owned(), |value| format!("{value:?}"))
}

/// The error of a wait on `element` that timed out: what it waited for, the element and the value
/// it ended with. A removed element (a stale handle) gets its own explanation.
async fn explain_timeout(
    element: &WebElement,
    what: &str,
    expected: Option<&str>,
    actual: Result<Option<String>, WebDriverError>,
) -> Result<(), Report> {
    if !element.is_present().await.unwrap_or(false) {
        bail!(
            "waited for {what} to become {}, but the element was removed from the page \
             (re-rendered?): look it up after the change",
            shown(expected)
        );
    }
    let actual = actual?;
    bail!(
        "{what} of {} did not become {} within {TIMEOUT:?}; it is {}",
        element.describe().await?,
        shown(expected),
        shown(actual.as_deref())
    )
}

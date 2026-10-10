//! Everything a test does with an element it found: look up below it, read, wait, check
//! stability, act.

use std::{sync::Arc, time::Duration};

use assertr::{matchers::eq, prelude::*};
use browser_test::{
    StepExt,
    thirtyfour::{TypingData, WebElement, session::handle::SessionHandle},
};
use rootcause::{Report, bail, prelude::ResultExt};
use serde::{Deserialize, de::DeserializeOwned};

use crate::pages::{
    Locator,
    event::SyntheticEvent,
    health, keyboard,
    lookup::{self, Root},
    script,
};

/// `describe(element)`: names the element in messages, see [`ElementActions::describe`].
pub(crate) const DESCRIBE: &str = "
    const describe = element => {
        const tag = element.tagName.toLowerCase();
        if (element.id) return `<${tag} id=${JSON.stringify(element.id)}>`;
        if (tag === 'body' || tag === 'html') return `<${tag}>`;
        const role = element.getAttribute('role');
        const label = element.getAttribute('aria-label');
        let text = [...(element.innerText ?? element.textContent).trim().split(/\\s+/).join(' ')];
        text = text.length > 60 ? text.slice(0, 60).join('') + '…' : text.join('');
        return `<${tag}`
            + (role === null ? '' : ` role=${JSON.stringify(role)}`)
            + (label === null ? '' : ` aria-label=${JSON.stringify(label)}`)
            + `> ${JSON.stringify(text)}`;
    };";

/// The viewport-relative rectangle in CSS pixels (`getBoundingClientRect()`), unlike
/// WebDriver's `WebElement::rect`, which reports document-relative coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct ClientRect {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub width: f64,
    pub height: f64,
}

/// How far an element is scrolled and how far it can scroll (vertically), in CSS pixels.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct ScrollExtent {
    /// `scrollTop`.
    pub top: f64,
    /// `clientHeight`: the visible part.
    pub client_height: f64,
    /// `scrollHeight`: the content.
    pub scroll_height: f64,
}

impl ScrollExtent {
    /// How far the element is scrolled from its end.
    pub fn distance_to_end(&self) -> f64 {
        self.scroll_height - self.client_height - self.top
    }
}

/// The pointer [`ElementActions::press_and_hold`] pressed: still pressed until
/// [`release`](Self::release)d, so a test that forgets it leaves the button down.
#[must_use = "the pointer stays pressed until `release`d"]
#[derive(Debug)]
pub struct HeldPointer {
    session: Arc<SessionHandle>,
}

impl HeldPointer {
    /// Move the pressed pointer by (`x`, `y`) CSS pixels.
    pub async fn move_by(&self, x: i64, y: i64) -> Result<(), Report> {
        self.session
            .action_chain()
            .move_by_offset(x, y)
            .perform()
            .await
            .context("failed to move the pressed pointer")?;
        Ok(())
    }

    /// Move the pressed pointer to the center of `element`.
    pub async fn move_to(&self, element: &WebElement) -> Result<(), Report> {
        self.session
            .action_chain()
            .move_to_element_center(element)
            .perform()
            .await
            .context("failed to move the pressed pointer")?;
        Ok(())
    }

    /// Release the pointer where it is.
    pub async fn release(self) -> Result<(), Report> {
        self.session
            .action_chain()
            .release()
            .perform()
            .await
            .context("failed to release the pointer")?;
        Ok(())
    }
}

/// The values an attribute took since [`ElementActions::record_attr`] started recording.
#[must_use = "finish or cancel the recording before leaving the page"]
#[derive(Debug)]
pub struct AttrRecording {
    session: Arc<SessionHandle>,
    id: String,
}

impl AttrRecording {
    /// Stop recording, drain pending records and release the observer and its stored values.
    pub async fn finish(self) -> Result<Vec<Option<String>>, Report> {
        script::eval(
            &self.session,
            "const id = arguments[0]; const recording = window.__attrRecordings?.get(id);
             if (!recording) throw new Error('attribute recording expired or already finished');
             recording.collect(recording.observer.takeRecords()); recording.observer.disconnect();
             window.__attrRecordings.delete(id); return recording.values;",
            vec![self.id.into()],
        )
        .await
    }

    /// Stop without retaining the recorded values.
    pub async fn cancel(self) -> Result<(), Report> {
        self.finish().await?;
        Ok(())
    }
}

/// The number of synthetic events (dispatched by a script, `isTrusted` false: e.g. the focus event
/// react-aria fires to show a focus ring again) of one type an element got since
/// [`ElementActions::count_synthetic_events`] started counting.
#[must_use = "finish the count before leaving the page"]
#[derive(Debug)]
pub struct SyntheticEventCount {
    session: Arc<SessionHandle>,
    id: String,
}

impl SyntheticEventCount {
    /// The number counted so far.
    pub async fn count(&self) -> Result<u64, Report> {
        script::eval(
            &self.session,
            "const counter = window.__eventCounts?.get(arguments[0]);
             if (!counter) throw new Error('event count expired or already finished');
             return counter.count;",
            vec![self.id.clone().into()],
        )
        .await
    }

    /// Stop counting and remove the listener.
    pub async fn finish(self) -> Result<u64, Report> {
        script::eval(
            &self.session,
            "const id = arguments[0]; const counter = window.__eventCounts?.get(id);
             if (!counter) throw new Error('event count expired or already finished');
             counter.stop(); window.__eventCounts.delete(id); return counter.count;",
            vec![self.id.into()],
        )
        .await
    }
}

/// Starts recording the attribute `name` of every element matching `selector` in the document,
/// including elements inserted later: the value each had when inserted, then every value it took
/// (see [`crate::pages::Page::record_attr_of`]).
pub(super) async fn record_attr_of(
    session: &Arc<SessionHandle>,
    selector: &str,
    name: &str,
) -> Result<AttrRecording, Report> {
    let id = script::eval(
        session,
        "const [selector, name] = arguments;
         window.__attrRecordings ??= new Map();
         const id = crypto.randomUUID(); const values = [];
         // A value: the old value of the target's next record of the batch, else the current one.
         const valueAfter = (records, i, element) => {
             const next = records.slice(i + 1).find(r => r.type === 'attributes' && r.target === element);
             return next ? next.oldValue : element.getAttribute(name);
         };
         const collect = records => records.forEach((record, i) => {
             if (record.type === 'childList') {
                 for (const node of record.addedNodes) {
                     if (!(node instanceof Element)) continue;
                     const found = node.matches(selector) ? [node] : [];
                     found.push(...node.querySelectorAll(selector));
                     for (const element of found) values.push(valueAfter(records, i, element));
                 }
             } else if (record.target.matches(selector)) {
                 values.push(valueAfter(records, i, record.target));
             }
         });
         for (const element of document.querySelectorAll(selector)) values.push(element.getAttribute(name));
         const observer = new MutationObserver(collect);
         observer.observe(document.body, {subtree: true, childList: true, attributes: true, attributeFilter: [name], attributeOldValue: true});
         window.__attrRecordings.set(id, {values, observer, collect});
         return id;",
        vec![selector.into(), name.into()],
    )
    .await?;
    Ok(AttrRecording {
        session: Arc::clone(session),
        id,
    })
}

/// The outcome of [`ElementActions::dispatch`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dispatched {
    /// Whether a handler called `preventDefault()`.
    pub default_prevented: bool,
}

/// Shared operations on thirtyfour's WebElement. The handle identifies one node; locator
/// waits on Page re-resolve when a component replaces that node. Raw WebElement methods remain available.
pub trait ElementActions {
    /// Resolve exactly one descendant. Absence is retried; ambiguity fails immediately.
    async fn element(&self, locator: impl Into<Locator> + Send) -> Result<WebElement, Report>;

    /// Explicitly choose the first matching descendant in document order.
    async fn first_element(&self, locator: impl Into<Locator> + Send)
    -> Result<WebElement, Report>;

    /// Read matching descendants now, without waiting for their presence.
    async fn elements(&self, locator: impl Into<Locator> + Send)
    -> Result<Vec<WebElement>, Report>;

    async fn count(&self, locator: impl Into<Locator> + Send) -> Result<usize, Report>;

    async fn inner_texts(&self, locator: impl Into<Locator> + Send) -> Result<Vec<String>, Report>;

    #[track_caller]
    fn wait_for_count(
        &self,
        locator: impl Into<Locator> + Send,
        expected: usize,
    ) -> impl Future<Output = Result<(), Report>>;

    /// Observe the descendant count throughout a positive duration.
    #[track_caller]
    fn count_stays(
        &self,
        locator: impl Into<Locator> + Send,
        expected: usize,
        duration: Duration,
    ) -> impl Future<Output = Result<(), Report>>;

    /// Read trimmed DOM `innerText`, including display-contents children.
    async fn inner_text(&self) -> Result<String, Report>;

    /// Read the browser-computed accessible name through WebDriver.
    async fn accessible_name(&self) -> Result<String, Report>;

    /// The browser-computed role, through the standard WebDriver endpoint.
    async fn accessible_role(&self) -> Result<String, Report>;

    /// Read the browser-computed description through its accessibility tree.
    async fn accessible_description(&self) -> Result<String, Report>;

    async fn number_value(&self) -> Result<f64, Report>;

    async fn is_within(&self, selector: &str) -> Result<bool, Report>;

    async fn is_valid(&self) -> Result<bool, Report>;

    /// Read viewport-relative layout bounds, including fractional CSS pixels.
    async fn client_rect(&self) -> Result<ClientRect, Report>;

    /// The CSS custom property `name` (`--x`) set in the element's `style` (WebDriver's CSS value
    /// command doesn't read custom properties); empty when unset.
    async fn style_property(&self, name: &str) -> Result<String, Report>;

    async fn describe(&self) -> Result<String, Report>;

    /// Wait on this exact node. A stale handle is an error; Page locator waits can re-resolve.
    #[track_caller]
    fn wait_for_attr<'a>(
        &'a self,
        name: &'a str,
        expected: Option<&'a str>,
    ) -> impl Future<Output = Result<(), Report>> + 'a;

    /// Observe immediately and throughout a positive duration. No settling is implicit.
    #[track_caller]
    fn attr_stays<'a>(
        &'a self,
        name: &'a str,
        expected: Option<&'a str>,
        duration: Duration,
    ) -> impl Future<Output = Result<(), Report>> + 'a;

    #[track_caller]
    fn wait_for_prop<'a>(
        &'a self,
        name: &'a str,
        expected: &'a str,
    ) -> impl Future<Output = Result<(), Report>> + 'a;

    /// Observe immediately and throughout a positive duration. No settling is implicit.
    #[track_caller]
    fn prop_stays<'a>(
        &'a self,
        name: &'a str,
        expected: &'a str,
        duration: Duration,
    ) -> impl Future<Output = Result<(), Report>> + 'a;

    #[track_caller]
    fn wait_for_inner_text<'a>(
        &'a self,
        expected: &'a str,
    ) -> impl Future<Output = Result<(), Report>> + 'a;

    /// Observe immediately and throughout a positive duration. No settling is implicit.
    #[track_caller]
    fn inner_text_stays<'a>(
        &'a self,
        expected: &'a str,
        duration: Duration,
    ) -> impl Future<Output = Result<(), Report>> + 'a;

    async fn hover(&self) -> Result<(), Report>;

    /// Double-click the element's center with the mouse.
    async fn double_click(&self) -> Result<(), Report>;

    async fn press_and_hold(&self) -> Result<HeldPointer, Report>;

    async fn press_and_hold_at(&self, x: i64, y: i64) -> Result<HeldPointer, Report>;

    /// Dispatch a DOM click without pointer movement or focus, as assistive technology does.
    async fn virtual_click(&self) -> Result<(), Report>;

    /// Set the value and dispatch an input event, without typing keystrokes.
    async fn virtual_input(&self, value: &str) -> Result<(), Report>;

    /// Focus the element unless it has focus (a text field with the caret at the end of its
    /// value, as WebDriver's send keys does), then send `keys` like `Page::send_keys`: characters
    /// as a US keyboard types them, whatever the host's layout. Use this, never thirtyfour's
    /// `WebElement::send_keys`, which types through the host's layout.
    async fn type_keys(&self, keys: impl Into<TypingData> + Send) -> Result<(), Report>;

    async fn dispatch<I: Send>(&self, event: SyntheticEvent<I>) -> Result<Dispatched, Report>;

    async fn dispatch_both<I: Send, J: Send>(
        &self,
        first: SyntheticEvent<I>,
        second: SyntheticEvent<J>,
    ) -> Result<(Dispatched, Dispatched), Report>;

    async fn scroll_to_top(&self, top: f64) -> Result<(), Report>;

    /// The scroll position and extent, read at once.
    async fn scroll_extent(&self) -> Result<ScrollExtent, Report>;

    /// Record every mutation, including intermediate values in the same task. Finish or cancel to release.
    async fn record_attr(&self, name: &str) -> Result<AttrRecording, Report>;

    /// Starts counting the synthetic `name` events (`isTrusted` false) the element gets.
    async fn count_synthetic_events(&self, name: &str) -> Result<SyntheticEventCount, Report>;

    async fn scroll_by_steps(&self, steps: &[f64], interval: Duration) -> Result<(), Report>;

    async fn check_validity(&self) -> Result<bool, Report>;

    async fn form_values(&self, name: &str) -> Result<Vec<String>, Report>;

    async fn submit(&self) -> Result<(), Report>;

    async fn reset(&self) -> Result<(), Report>;
}

impl ElementActions for WebElement {
    async fn element(&self, locator: impl Into<Locator> + Send) -> Result<WebElement, Report> {
        lookup::element(Root::WebElement(self), locator.into()).await
    }

    /// Explicitly choose the first match in document order.
    async fn first_element(
        &self,
        locator: impl Into<Locator> + Send,
    ) -> Result<WebElement, Report> {
        lookup::first(Root::WebElement(self), locator.into()).await
    }

    async fn elements(
        &self,
        locator: impl Into<Locator> + Send,
    ) -> Result<Vec<WebElement>, Report> {
        lookup::elements(Root::WebElement(self), locator.into()).await
    }

    async fn count(&self, locator: impl Into<Locator> + Send) -> Result<usize, Report> {
        lookup::count(Root::WebElement(self), locator.into()).await
    }

    async fn inner_texts(&self, locator: impl Into<Locator> + Send) -> Result<Vec<String>, Report> {
        lookup::inner_texts(Root::WebElement(self), locator.into()).await
    }

    #[track_caller]
    fn wait_for_count(
        &self,
        locator: impl Into<Locator> + Send,
        expected: usize,
    ) -> impl Future<Output = Result<(), Report>> {
        lookup::wait_for_count(Root::WebElement(self), locator.into(), expected)
    }

    #[track_caller]
    fn count_stays(
        &self,
        locator: impl Into<Locator> + Send,
        expected: usize,
        duration: Duration,
    ) -> impl Future<Output = Result<(), Report>> {
        lookup::count_stays(Root::WebElement(self), locator.into(), expected, duration)
    }

    async fn inner_text(&self) -> Result<String, Report> {
        Ok(assertr::assertions::thirtyfour::read::inner_text(self).await?)
    }

    async fn accessible_name(&self) -> Result<String, Report> {
        Ok(assertr::assertions::thirtyfour::read::accessible_name(self).await?)
    }

    async fn accessible_description(&self) -> Result<String, Report> {
        Ok(assertr::assertions::thirtyfour::read::accessible_description(self).await?)
    }

    async fn accessible_role(&self) -> Result<String, Report> {
        Ok(assertr::assertions::thirtyfour::read::accessible_role(self).await?)
    }

    async fn style_property(&self, name: &str) -> Result<String, Report> {
        script(
            self,
            "return arguments[0].style.getPropertyValue(arguments[1]);",
            vec![name.into()],
        )
        .await
    }

    async fn number_value(&self) -> Result<f64, Report> {
        let value = self.prop("value").await?.unwrap_or_default();
        Ok(value
            .parse()
            .context_with(|| format!("the value {value:?} is no number"))?)
    }

    async fn is_within(&self, selector: &str) -> Result<bool, Report> {
        script(
            self,
            "return arguments[0].closest(arguments[1]) !== null;",
            vec![selector.into()],
        )
        .await
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
        script(
            self,
            &format!("{DESCRIBE} return describe(arguments[0]);"),
            vec![],
        )
        .await
    }

    #[track_caller]
    fn wait_for_attr<'a>(
        &'a self,
        name: &'a str,
        expected: Option<&'a str>,
    ) -> impl Future<Output = Result<(), Report>> + 'a {
        let check = assert_that_owned!(move || async move {
            health::expect_no_panic(self.handle()).await?;
            Ok::<_, Report>(self.attr(name).await?)
        })
        .with_subject_name(format!("attr {name}"))
        .eventually_ok()
        .giving_up_on_any_error()
        .try_matches(eq(expected.map(str::to_owned)));
        async move {
            check.await?;
            Ok(())
        }
        .step("wait_for_attr")
    }

    #[track_caller]
    fn attr_stays<'a>(
        &'a self,
        name: &'a str,
        expected: Option<&'a str>,
        duration: Duration,
    ) -> impl Future<Output = Result<(), Report>> + 'a {
        let check = assert_that_owned!(move || async move {
            health::expect_no_panic(self.handle()).await?;
            Ok::<_, Report>(self.attr(name).await?)
        })
        .with_subject_name(format!("attr {name}"))
        .consistently_ok()
        .for_at_least(duration)
        .try_matches(eq(expected.map(str::to_owned)));
        async move {
            if duration.is_zero() {
                bail!("stability checks require a positive duration");
            }
            check.await?;
            Ok(())
        }
        .step("attr_stays")
    }

    #[track_caller]
    fn wait_for_prop<'a>(
        &'a self,
        name: &'a str,
        expected: &'a str,
    ) -> impl Future<Output = Result<(), Report>> + 'a {
        let check = assert_that_owned!(move || async move {
            health::expect_no_panic(self.handle()).await?;
            Ok::<_, Report>(self.prop(name).await?)
        })
        .with_subject_name(format!("prop {name}"))
        .eventually_ok()
        .giving_up_on_any_error()
        .try_matches(eq(Some(expected.to_owned())));
        async move {
            check.await?;
            Ok(())
        }
        .step("wait_for_prop")
    }

    #[track_caller]
    fn prop_stays<'a>(
        &'a self,
        name: &'a str,
        expected: &'a str,
        duration: Duration,
    ) -> impl Future<Output = Result<(), Report>> + 'a {
        let check = assert_that_owned!(move || async move {
            health::expect_no_panic(self.handle()).await?;
            Ok::<_, Report>(self.prop(name).await?)
        })
        .with_subject_name(format!("prop {name}"))
        .consistently_ok()
        .for_at_least(duration)
        .try_matches(eq(Some(expected.to_owned())));
        async move {
            if duration.is_zero() {
                bail!("stability checks require a positive duration");
            }
            check.await?;
            Ok(())
        }
        .step("prop_stays")
    }

    #[track_caller]
    fn wait_for_inner_text<'a>(
        &'a self,
        expected: &'a str,
    ) -> impl Future<Output = Result<(), Report>> + 'a {
        let check = assert_that_owned!(move || async move {
            health::expect_no_panic(self.handle()).await?;
            self.inner_text().await
        })
        .with_subject_name("inner text")
        .eventually_ok()
        .giving_up_on_any_error()
        .try_matches(eq(expected.to_owned()));
        async move {
            check.await?;
            Ok(())
        }
        .step("wait_for_inner_text")
    }

    #[track_caller]
    fn inner_text_stays<'a>(
        &'a self,
        expected: &'a str,
        duration: Duration,
    ) -> impl Future<Output = Result<(), Report>> + 'a {
        let check = assert_that_owned!(move || async move {
            health::expect_no_panic(self.handle()).await?;
            self.inner_text().await
        })
        .with_subject_name("inner text")
        .consistently_ok()
        .for_at_least(duration)
        .try_matches(eq(expected.to_owned()));
        async move {
            if duration.is_zero() {
                bail!("stability checks require a positive duration");
            }
            check.await?;
            Ok(())
        }
        .step("inner_text_stays")
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

    async fn double_click(&self) -> Result<(), Report> {
        self.handle()
            .action_chain()
            .double_click_element(self)
            .perform()
            .await
            .context("failed to double-click the element")?;
        Ok(())
    }

    async fn press_and_hold(&self) -> Result<HeldPointer, Report> {
        self.handle()
            .action_chain()
            .click_and_hold_element(self)
            .perform()
            .await
            .context("failed to press the pointer on the element")?;
        Ok(HeldPointer {
            session: Arc::clone(self.handle()),
        })
    }

    async fn press_and_hold_at(&self, x: i64, y: i64) -> Result<HeldPointer, Report> {
        self.handle()
            .action_chain()
            .move_to_element_with_offset(self, x, y)
            .click_and_hold()
            .perform()
            .await
            .context("failed to press the pointer on the element")?;
        Ok(HeldPointer {
            session: Arc::clone(self.handle()),
        })
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

    async fn type_keys(&self, keys: impl Into<TypingData> + Send) -> Result<(), Report> {
        let keys = keys.into();
        script::<()>(
            self,
            "const element = arguments[0];
             let focused = document.activeElement;
             while (focused?.shadowRoot?.activeElement) focused = focused.shadowRoot.activeElement;
             if (focused !== element) {
                 element.focus();
                 try {
                     element.setSelectionRange?.(element.value.length, element.value.length);
                 } catch {
                     // Number inputs have no text selection.
                 }
             }",
            vec![],
        )
        .await?;
        keyboard::send_keys(self.handle(), &keys).await
    }

    async fn dispatch<I: Send>(&self, event: SyntheticEvent<I>) -> Result<Dispatched, Report> {
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

    async fn dispatch_both<I: Send, J: Send>(
        &self,
        first: SyntheticEvent<I>,
        second: SyntheticEvent<J>,
    ) -> Result<(Dispatched, Dispatched), Report> {
        let description = format!("{first}, then {second}");
        let events: Vec<serde_json::Value> = [first.into_parts(), second.into_parts()]
            .into_iter()
            .map(|(interface, kind, init)| {
                serde_json::Value::from(vec![interface.into(), kind.into(), init])
            })
            .collect();
        let default_prevented: Vec<bool> = script(
            self,
            "const [target, events] = arguments;
             return events.map(([constructor, type, init]) => {
                 const event = new window[constructor](type, init);
                 target.dispatchEvent(event);
                 return event.defaultPrevented;
             });",
            vec![serde_json::Value::from(events)],
        )
        .await
        .context_with(|| format!("failed to dispatch {description}"))?;
        let [first, second] = default_prevented[..] else {
            bail!("dispatching {description} returned {default_prevented:?}");
        };
        Ok((
            Dispatched {
                default_prevented: first,
            },
            Dispatched {
                default_prevented: second,
            },
        ))
    }

    async fn scroll_to_top(&self, top: f64) -> Result<(), Report> {
        script(
            self,
            "arguments[0].scrollTop = arguments[1];",
            vec![top.into()],
        )
        .await
    }

    async fn scroll_extent(&self) -> Result<ScrollExtent, Report> {
        script(
            self,
            "const e = arguments[0];
             return {top: e.scrollTop, client_height: e.clientHeight, scroll_height: e.scrollHeight};",
            vec![],
        )
        .await
    }

    async fn record_attr(&self, name: &str) -> Result<AttrRecording, Report> {
        let id = script(
            self,
            "const [element, name] = arguments;
             window.__attrRecordings ??= new Map();
             const id = crypto.randomUUID(); const values = [];
             const collect = records => records.forEach((record, i) => {
                 values.push(i + 1 < records.length ? records[i + 1].oldValue : element.getAttribute(name));
             });
             const observer = new MutationObserver(collect);
             observer.observe(element, {attributes: true, attributeFilter: [name], attributeOldValue: true});
             window.__attrRecordings.set(id, {values, observer, collect});
             return id;",
            vec![name.into()],
        )
        .await?;
        Ok(AttrRecording {
            session: Arc::clone(self.handle()),
            id,
        })
    }

    async fn count_synthetic_events(&self, name: &str) -> Result<SyntheticEventCount, Report> {
        let id = script(
            self,
            "const [element, name] = arguments;
             window.__eventCounts ??= new Map();
             const id = crypto.randomUUID();
             const counter = {count: 0};
             const listener = e => { if (!e.isTrusted) counter.count++; };
             element.addEventListener(name, listener);
             counter.stop = () => element.removeEventListener(name, listener);
             window.__eventCounts.set(id, counter);
             return id;",
            vec![name.into()],
        )
        .await?;
        Ok(SyntheticEventCount {
            session: Arc::clone(self.handle()),
            id,
        })
    }

    async fn scroll_by_steps(&self, steps: &[f64], interval: Duration) -> Result<(), Report> {
        let interval = u64::try_from(interval.as_millis()).unwrap_or(u64::MAX);
        script::eval_async(
            self.handle(),
            "const [element, steps, interval, done] = arguments;
             const step = i => {
                 if (i === steps.length) return done();
                 element.scrollTop += steps[i];
                 setTimeout(() => step(i + 1), interval);
             };
             step(0);",
            vec![self.to_json()?, steps.into(), interval.into()],
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
    let mut arguments = vec![element.to_json()?];
    arguments.extend(args);
    script::eval(element.handle(), script, arguments).await
}

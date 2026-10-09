//! Time measured in the page (see `::start_stopwatch`): from an event to a change of
//! the DOM, on the page's clock, so that slow WebDriver round trips don't count.

use std::{sync::Arc, time::Duration};

use browser_test::thirtyfour::session::handle::SessionHandle;
use rootcause::{Report, bail};
use serde::Deserialize;
use serde_json::{Value, json};

use super::{EventKind, PointerKind, WebElement, script};

/// The event starting a [`Stopwatch`]: a pointer event or a plain event (a `scroll`).
#[derive(Debug, Clone, Copy)]
pub struct StopwatchStart(&'static str);

impl From<PointerKind> for StopwatchStart {
    fn from(kind: PointerKind) -> Self {
        Self(kind.as_str())
    }
}

impl From<EventKind> for StopwatchStart {
    fn from(kind: EventKind) -> Self {
        Self(kind.as_str())
    }
}

/// What ends a [`Stopwatch`]'s measurement.
#[derive(Debug, Clone, Copy)]
pub enum StopwatchEnd<'a> {
    /// An element matching the CSS selector is in the document.
    Appears(&'a str),
    /// No element matching the CSS selector is in the document.
    Disappears(&'a str),
    /// The attribute `name` changes on `element` or an element below it.
    AttributeChanges {
        element: &'a WebElement,
        name: &'a str,
    },
}

/// A measurement running in the page, from an event to a change of the DOM.
#[must_use = "finish or cancel the stopwatch before leaving the page"]
pub struct Stopwatch {
    session: Arc<SessionHandle>,
    id: String,
}

/// Starts measuring in the page: from the next `event` on `target` (its time stamp) to `end`.
pub(crate) async fn start(
    session: &Arc<SessionHandle>,
    target: &WebElement,
    event: StopwatchStart,
    end: StopwatchEnd<'_>,
) -> Result<Stopwatch, Report> {
    let end = match end {
        StopwatchEnd::Appears(selector) => json!({ "appears": selector }),
        StopwatchEnd::Disappears(selector) => json!({ "disappears": selector }),
        StopwatchEnd::AttributeChanges { element, name } => {
            json!({ "element": element.to_json()?, "attribute": name })
        }
    };
    let id = script::eval(
        session,
        "const [target, event, end] = arguments;
         window.__stopwatches ??= new Map();
         const id = crypto.randomUUID();
         const stopwatch = {started: null, ended: null};
         const start = e => { stopwatch.started ??= e.timeStamp; };
         target.addEventListener(event, start, {capture: true, once: true});
         const ended = () => end.appears !== undefined ? document.querySelector(end.appears) !== null
             : end.disappears !== undefined ? document.querySelector(end.disappears) === null
             : true;
         const observer = new MutationObserver(() => {
             if (stopwatch.ended === null && ended()) {
                 stopwatch.ended = performance.now(); observer.disconnect();
             }
         });
         stopwatch.cancel = () => { observer.disconnect(); target.removeEventListener(event, start, true); };
         window.__stopwatches.set(id, stopwatch);
         if (end.appears !== undefined || end.disappears !== undefined) {
             observer.observe(document.body, {subtree: true, childList: true, attributes: true});
         } else {
             observer.observe(end.element, {subtree: true, attributes: true, attributeFilter: [end.attribute]});
         }
         return id;",
        vec![target.to_json()?, event.0.into(), end],
    )
    .await?;
    Ok(Stopwatch {
        session: Arc::clone(session),
        id,
    })
}

#[derive(Debug, Deserialize)]
struct Times {
    started: Option<f64>,
    ended: Option<f64>,
}

impl Stopwatch {
    /// Remove the listener, observer and stored measurement even if it never completed.
    pub async fn cancel(self) -> Result<(), Report> {
        script::eval(
            &self.session,
            "const id = arguments[0]; const stopwatch = window.__stopwatches?.get(id);
            if (stopwatch) { stopwatch.cancel(); window.__stopwatches.delete(id); }",
            vec![self.id.into()],
        )
        .await
    }

    /// The time from the event to the end of the measurement; an error when either hasn't
    /// happened (yet) or the end came first.
    pub async fn finish(self) -> Result<Duration, Report> {
        let times: Times = script::eval(
            &self.session,
            "const id = arguments[0]; const stopwatch = window.__stopwatches?.get(id);
             if (!stopwatch) throw new Error('stopwatch expired or already finished');
             stopwatch.cancel(); window.__stopwatches.delete(id);
             return {started: stopwatch.started, ended: stopwatch.ended};",
            vec![Value::from(self.id)],
        )
        .await?;
        let (Some(started), Some(ended)) = (times.started, times.ended) else {
            bail!("the stopwatch didn't see both its start and its end: {times:?}");
        };
        if ended < started {
            bail!(
                "the stopwatch's end came {} ms before its start",
                started - ended
            );
        }
        Ok(Duration::from_secs_f64((ended - started) / 1000.0))
    }
}

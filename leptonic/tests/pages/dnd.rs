use assertr::{matchers::eq, prelude::*};
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;
use serde::de::DeserializeOwned;

use crate::pages::{ElementActions, Page, PageActions, SyntheticEvent, xpath};

/// A script: a `DragEvent` carries a `DataTransfer`, which no event init from WebDriver can.
/// Fires a drag event at the center of an element (`arguments[0]`, type `arguments[1]`) with the
/// page's current `DataTransfer` (`window.__dndTransfer`, created by `dragstart`), modifier
/// keys (`arguments[2]`) and, if given, at an offset from its top left corner (`arguments[3]`);
/// returns whether the default was prevented.
const FIRE_DRAG_EVENT: &str = "
    const [element, type, modifiers, at] = arguments;
    if (type === 'dragstart') {
        // A constructed DataTransfer ignores `effectAllowed` writes (only a real drag's dragstart
        // may set them): keep both effects as plain properties, as the browser's drag data store.
        const transfer = new DataTransfer();
        for (const [name, initial] of [['effectAllowed', 'uninitialized'], ['dropEffect', 'none']]) {
            let value = initial;
            Object.defineProperty(transfer, name, { get: () => value, set: v => { value = v; }, configurable: true });
        }
        window.__dndTransfer = transfer;
    }
    const rect = element.getBoundingClientRect();
    const event = new DragEvent(type, {
        dataTransfer: window.__dndTransfer,
        clientX: rect.left + (at ? at[0] : rect.width / 2),
        clientY: rect.top + (at ? at[1] : rect.height / 2),
        bubbles: true,
        cancelable: true,
        altKey: modifiers.includes('alt'),
        ctrlKey: modifiers.includes('ctrl'),
        shiftKey: modifiers.includes('shift'),
        metaKey: modifiers.includes('meta'),
    });
    element.dispatchEvent(event);
    return event.defaultPrevented;";

/// Actions on the drag and drop fixtures (`/hooks/dnd`, `/hooks/dnd-targets`,
/// `/hooks/dnd-collection`): event logs, the fixture's mid-drag actions, and native drags
/// synthesized with a `DataTransfer` and `DragEvent`s.
pub trait DndActions: PageActions {
    /// The entries of the log list `#<id>`.
    async fn log(&self, id: &str) -> Result<Vec<String>, Report> {
        self.element(format!("#{id}"))
            .await?
            .inner_texts("li")
            .await
    }

    /// Waits until the log `#<id>` is exactly `expected`.
    #[track_caller]
    fn expect_log<'a>(
        &'a self,
        id: &'a str,
        expected: &'a [&'a str],
    ) -> impl Future<Output = Result<(), Report>> + 'a {
        let check = assert_that_owned!(move || self.log(id))
            .with_subject_name(format!("the log #{id}"))
            .eventually_ok()
            .matches(eq(expected));
        async move {
            check.await;
            Ok(())
        }
    }

    /// Waits until the log `#<id>` is exactly `expected`, and checks it stays so (nothing more
    /// is logged).
    #[track_caller]
    fn expect_log_settled<'a>(
        &'a self,
        id: &'a str,
        expected: &'a [&'a str],
    ) -> impl Future<Output = Result<(), Report>> + 'a {
        let reached = self.expect_log(id, expected);
        let kept = assert_that_owned!(move || self.log(id))
            .with_subject_name(format!("the log #{id}"))
            .consistently_ok()
            .matches(eq(expected));
        async move {
            reached.await?;
            self.settle().await?;
            kept.await;
            Ok(())
        }
    }

    /// Dispatches the fixture's custom event `event` with the `action` as its detail on the
    /// element `#<target>`.
    async fn dispatch_action(&self, target: &str, event: &str, action: &str) -> Result<(), Report> {
        self.element(format!("#{target}"))
            .await?
            .dispatch(SyntheticEvent::custom(event, action).with("bubbles", false))
            .await?;
        Ok(())
    }

    /// Fires the native drag event `kind` (`dragstart` creates a new `DataTransfer`) at the
    /// center of `element`. Returns whether its default was prevented.
    async fn fire_drag_event(
        &self,
        element: &WebElement,
        kind: &str,
        modifiers: &[&str],
    ) -> Result<bool, Report> {
        self.fire_drag_event_at(element, kind, modifiers, None)
            .await
    }

    /// [`fire_drag_event`](Self::fire_drag_event) at `at` (x, y) from the element's top left
    /// corner instead of its center.
    async fn fire_drag_event_at(
        &self,
        element: &WebElement,
        kind: &str,
        modifiers: &[&str],
        at: Option<(f64, f64)>,
    ) -> Result<bool, Report> {
        let modifiers: Vec<serde_json::Value> = modifiers.iter().map(|m| (*m).into()).collect();
        self.eval(
            FIRE_DRAG_EVENT,
            vec![
                element.to_json()?,
                kind.into(),
                serde_json::Value::Array(modifiers),
                at.map_or(serde_json::Value::Null, |(x, y)| serde_json::json!([x, y])),
            ],
        )
        .await
    }

    /// A property of the current native drag's `DataTransfer`, e.g. `effectAllowed`, or
    /// `getData('text/plain')` (`expression` is script, evaluated on the transfer).
    async fn transfer<T: DeserializeOwned>(&self, expression: &str) -> Result<T, Report> {
        self.eval(
            &format!("return window.__dndTransfer.{expression};"),
            vec![],
        )
        .await
    }

    /// Whether `element` is inert (itself or through an ancestor).
    async fn is_inert(&self, element: &WebElement) -> Result<bool, Report> {
        Ok(element.count(xpath("ancestor-or-self::*[@inert]")).await? > 0)
    }
}

impl DndActions for Page<'_> {}

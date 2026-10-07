use std::time::Duration;

use browser_test::thirtyfour::{By, WebDriver, WebElement};
use leptos_browser_test::{Report, ResultExt};

use crate::pages::BaseActions;

/// The drag and drop fixtures (`/hooks/dnd`, `/hooks/dnd-targets`, `/hooks/dnd-collection`):
/// event logs, the fixture's mid-drag actions, and native drags synthesized with a `DataTransfer`
/// and `DragEvent`s.
pub struct DndPage<'d> {
    pub driver: &'d WebDriver,
    pub base_url: &'d str,
}

impl BaseActions for DndPage<'_> {
    fn driver(&self) -> &WebDriver {
        self.driver
    }

    fn base_url(&self) -> &str {
        self.base_url
    }
}

/// How long negative checks let the page settle before checking again.
const SETTLE: Duration = Duration::from_millis(300);

/// Fires a drag event at the center of an element (`arguments[0]`, type `arguments[1]`) with the
/// page's current `DataTransfer` (`window.__dndTransfer`, created by `dragstart`), modifier
/// keys (`arguments[2]`) and, if given, at an offset from its top left corner (`arguments[3]`);
/// returns whether the default was prevented.
const FIRE_DRAG_EVENT: &str = "
    const [element, type, modifiers, at] = arguments;
    if (type === 'dragstart') { window.__dndTransfer = new DataTransfer(); }
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

impl DndPage<'_> {
    /// The entries of the log list `#<id>`.
    pub async fn log(&self, id: &str) -> Result<Vec<String>, Report> {
        let mut entries = Vec::new();
        for li in self.element(id).await?.find_all(By::Css("li")).await? {
            entries.push(li.text().await?);
        }
        Ok(entries)
    }

    /// Waits until the log `#<id>` is exactly `expected`.
    pub async fn expect_log(&self, id: &str, expected: &[&str]) -> Result<(), Report> {
        let expected: Vec<String> = expected.iter().map(|e| (*e).to_owned()).collect();
        self.wait_for_value(&format!("the log #{id}"), expected, || self.log(id))
            .await
    }

    /// Waits until the log `#<id>` is exactly `expected`, and checks it still is after the page
    /// settled (nothing more is logged).
    pub async fn expect_log_settled(&self, id: &str, expected: &[&str]) -> Result<(), Report> {
        self.expect_log(id, expected).await?;
        tokio::time::sleep(SETTLE).await;
        let actual = self.log(id).await?;
        if actual != expected {
            leptos_browser_test::bail!("the log #{id} changed to {actual:?}, expected {expected:?}");
        }
        Ok(())
    }

    /// Lets the page settle (for negative checks: something must not happen).
    pub async fn settle(&self) {
        tokio::time::sleep(SETTLE).await;
    }

    /// Dispatches the fixture's custom event `event` with the `action` as its detail on the
    /// element `#<target>`.
    pub async fn dispatch_action(
        &self,
        target: &str,
        event: &str,
        action: &str,
    ) -> Result<(), Report> {
        self.driver
            .execute(
                "document.getElementById(arguments[0]).dispatchEvent(new CustomEvent(arguments[1], {detail: arguments[2]}));",
                vec![target.into(), event.into(), action.into()],
            )
            .await
            .context("failed to dispatch the action")?;
        Ok(())
    }

    /// Fires the native drag event `kind` (`dragstart` creates a new `DataTransfer`) at the
    /// center of `element`. Returns whether its default was prevented.
    pub async fn fire_drag_event(
        &self,
        element: &WebElement,
        kind: &str,
        modifiers: &[&str],
    ) -> Result<bool, Report> {
        self.fire_drag_event_at(element, kind, modifiers, None).await
    }

    /// [`fire_drag_event`](Self::fire_drag_event) at `at` (x, y) from the element's top left
    /// corner instead of its center.
    pub async fn fire_drag_event_at(
        &self,
        element: &WebElement,
        kind: &str,
        modifiers: &[&str],
        at: Option<(f64, f64)>,
    ) -> Result<bool, Report> {
        let modifiers: Vec<serde_json::Value> = modifiers.iter().map(|m| (*m).into()).collect();
        let prevented = self
            .driver
            .execute(
                FIRE_DRAG_EVENT,
                vec![
                    element.to_json()?,
                    kind.into(),
                    serde_json::Value::Array(modifiers),
                    at.map_or(serde_json::Value::Null, |(x, y)| serde_json::json!([x, y])),
                ],
            )
            .await
            .context_with(|| format!("failed to fire {kind}"))?;
        Ok(prevented.json().as_bool().unwrap_or(false))
    }

    /// A property of the current native drag's `DataTransfer`, e.g. `effectAllowed`, or
    /// `getData('text/plain')`.
    pub async fn transfer(&self, expression: &str) -> Result<serde_json::Value, Report> {
        Ok(self
            .driver
            .execute(&format!("return window.__dndTransfer.{expression};"), vec![])
            .await
            .context_with(|| format!("failed to read the transfer's {expression}"))?
            .json()
            .clone())
    }

    /// The text of the elements `aria-describedby` of `element` refers to.
    pub async fn description(&self, element: &WebElement) -> Result<String, Report> {
        let text = self
            .driver
            .execute(
                "return (arguments[0].getAttribute('aria-describedby') ?? '').split(' ').map(id => document.getElementById(id)?.textContent ?? '').join(' ').trim();",
                vec![element.to_json()?],
            )
            .await?;
        Ok(text.json().as_str().unwrap_or_default().to_owned())
    }

    /// Whether `element` is inert (itself or through an ancestor).
    pub async fn is_inert(&self, element: &WebElement) -> Result<bool, Report> {
        Ok(self
            .driver
            .execute(
                "return arguments[0].closest('[inert]') !== null;",
                vec![element.to_json()?],
            )
            .await?
            .json()
            .as_bool()
            .unwrap_or(false))
    }

    /// Focuses `element` from script, as a screen reader's virtual cursor does.
    pub async fn focus_by_script(&self, element: &WebElement) -> Result<(), Report> {
        self.driver
            .execute("arguments[0].focus();", vec![element.to_json()?])
            .await?;
        Ok(())
    }

    /// Clicks `element` from script: a click without a pointer (`detail` 0), as screen readers
    /// click (a "virtual click").
    pub async fn virtual_click(&self, element: &WebElement) -> Result<(), Report> {
        self.driver
            .execute("arguments[0].click();", vec![element.to_json()?])
            .await?;
        Ok(())
    }

    /// Waits until the attribute `name` of `element` is `expected` and stays so after the page
    /// settled.
    pub async fn expect_attr_settled(
        &self,
        element: &WebElement,
        name: &str,
        expected: Option<&str>,
    ) -> Result<(), Report> {
        self.wait_for_attr(element, name, expected).await?;
        self.settle().await;
        let actual = element.attr(name).await?;
        if actual.as_deref() != expected {
            leptos_browser_test::bail!("attribute {name} changed to {actual:?}, expected {expected:?}");
        }
        Ok(())
    }
}

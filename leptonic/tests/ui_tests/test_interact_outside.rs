// Upstream: react-aria/test/interactions/useInteractOutside.test.js @ 99e6102368
//! `use_interact_outside`: a press outside the element (not inside) fires start and the
//! interaction; other buttons and a pointer up without a pointer down don't; nothing while
//! disabled.
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, MouseButton, MouseKind, Page, PointerKind, SyntheticEvent};

const PATH: &str = "/hooks/interact-outside";

/// The log of interactions outside.
async fn log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-interact-outside-log").await
}

/// A click outside the element fires the start and the interaction outside, while a click inside
/// or a virtual click (no pointer events) fires nothing ("should fire interact outside events based
/// on pointer events").
#[browser_test]
pub async fn pointer_events(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let log = log(page).await?;
    page.element("#test-interact-outside-target")
        .await?
        .click()
        .await?;
    log.inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    page.element("#test-interact-outside-away")
        .await?
        .click()
        .await?;
    log.wait_for_inner_text("start,outside").await?;
    // A virtual click has no pointer events: not an interaction outside.
    page.element("#test-interact-outside-away")
        .await?
        .virtual_click()
        .await?;
    log.inner_text_stays("start,outside", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A mouse press with `button` on `<body>`: `pointerdown`, `pointerup` and `click`.
async fn press_body(page: &Page<'_>, button: MouseButton) -> Result<(), Report> {
    let body = page.element("body").await?;
    for kind in [PointerKind::Down, PointerKind::Up] {
        body.dispatch(SyntheticEvent::pointer(kind).button(button))
            .await?;
    }
    body.dispatch(SyntheticEvent::mouse(MouseKind::Click).button(button))
        .await?;
    Ok(())
}

/// A middle-button press outside fires nothing, and a left-button press fires once ("should only
/// listen for the left mouse button").
#[browser_test]
pub async fn left_button_only(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let log = log(page).await?;
    press_body(page, MouseButton::Auxiliary).await?;
    log.inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    press_body(page, MouseButton::Primary).await?;
    log.wait_for_inner_text("start,outside").await?;
    log.inner_text_stays("start,outside", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A `pointerup` and `click` outside without a preceding `pointerdown` fire nothing, a full press
/// does ("should not fire interact outside if there is a pointer up event without a pointer down
/// first").
#[browser_test]
pub async fn pointer_up_without_pointer_down(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let log = log(page).await?;
    let body = page.element("body").await?;
    body.dispatch(SyntheticEvent::pointer(PointerKind::Up))
        .await?;
    body.dispatch(SyntheticEvent::mouse(MouseKind::Click).button(MouseButton::Primary))
        .await?;
    log.inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    // The same events after a `pointerdown` are an interaction outside.
    press_body(page, MouseButton::Primary).await?;
    log.wait_for_inner_text("start,outside").await?;
    Ok(())
}

/// While disabled, a click outside fires nothing ("does not handle pointer events if disabled").
#[browser_test]
pub async fn disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-interact-outside-disable")
        .await?
        .virtual_click()
        .await?;
    page.element("#test-interact-outside-away")
        .await?
        .click()
        .await?;
    log(page)
        .await?
        .inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

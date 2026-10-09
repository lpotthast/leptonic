// Upstream: react-aria/test/interactions/useLongPress.test.js @ 99e6102368
//! Long presses through `use_press`: start, end and the long press after the threshold, which
//! cancels the press; cancelled when released early; a custom threshold; the accessibility
//! description; no context menu on touch (only during the press); nothing for the keyboard.
use std::time::Duration;

use assertr::{matchers::eq, prelude::*};
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions, SyntheticEvent};

const PATH: &str = "/hooks/long-press";

/// The log of press and long press events.
async fn log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-long-press-log").await
}

/// Clears the log, by a script click.
async fn reset(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-long-press-reset")
        .await?
        .virtual_click()
        .await?;
    log(page).await?.wait_for_inner_text("").await?;
    Ok(())
}

/// The long-pressable `#test-long-press-<name>`.
async fn target(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    page.element(format!("#test-long-press-{name}")).await
}

/// A touch pointer event of `kind` (`pointerdown` or `pointerup`).
fn touch(kind: &str) -> SyntheticEvent {
    SyntheticEvent::pointer(kind)
        .with("pointerType", "touch")
        .with("pointerId", 1)
        .with("isPrimary", true)
        .with("button", 0)
        .with("buttons", i32::from(kind == "pointerdown"))
        .with("width", 1)
        .with("height", 1)
}

/// "should perform a long press": start, then end and the long press once the 500 ms threshold
/// passed; the release adds nothing.
pub async fn long_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let log = log(page).await?;
    let basic = target(page, "basic").await?;
    basic.dispatch(touch("pointerdown")).await?;
    // Well before the 500 ms threshold: a window of its own, independent of `BROWSER_TEST_STAYS_MS`.
    page.settle().await?;
    assert_that!(|| log.inner_text())
        .consistently_ok()
        .for_at_least(Duration::from_millis(100))
        .matches(eq("basic:longpressstart:touch"))
        .await;
    log.wait_for_inner_text(
        "basic:longpressstart:touch,basic:longpressend:touch,basic:longpress:touch",
    )
    .await?;
    basic.dispatch(touch("pointerup")).await?;
    // Past the press's click fallback (80 ms after a pointer up without a click).
    page.settle().await?;
    assert_that!(|| log.inner_text())
        .consistently_ok()
        .for_at_least(Duration::from_millis(300))
        .matches(eq(
            "basic:longpressstart:touch,basic:longpressend:touch,basic:longpress:touch",
        ))
        .await;
    reset(page).await?;
    Ok(())
}

/// "should cancel if pointer ends before timeout": no long press, also past the threshold.
pub async fn cancelled_when_released_early(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let log = log(page).await?;
    let basic = target(page, "basic").await?;
    basic.dispatch(touch("pointerdown")).await?;
    basic.dispatch(touch("pointerup")).await?;
    log.wait_for_inner_text("basic:longpressstart:touch,basic:longpressend:touch")
        .await?;
    page.settle().await?;
    assert_that!(|| log.inner_text())
        .consistently_ok()
        .for_at_least(Duration::from_millis(700))
        .matches(eq("basic:longpressstart:touch,basic:longpressend:touch"))
        .await;
    reset(page).await?;
    Ok(())
}

/// "should cancel other press events".
pub async fn cancels_other_press_events(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let with_press = target(page, "with-press").await?;
    with_press.dispatch(touch("pointerdown")).await?;
    log(page)
        .await?
        .wait_for_inner_text(
            "with-press:longpressstart:touch,with-press:pressstart:touch,\
             with-press:longpressend:touch,with-press:pressend:touch,with-press:longpress:touch",
        )
        .await?;
    with_press.dispatch(touch("pointerup")).await?;
    reset(page).await?;
    Ok(())
}

/// "should not cancel press events if pointer ends before timer".
pub async fn keeps_press_events_when_released_early(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let with_press = target(page, "with-press").await?;
    with_press.dispatch(touch("pointerdown")).await?;
    with_press.dispatch(touch("pointerup")).await?;
    log(page)
        .await?
        .wait_for_inner_text(
            "with-press:longpressstart:touch,with-press:pressstart:touch,\
             with-press:longpressend:touch,with-press:pressend:touch,with-press:press:touch",
        )
        .await?;
    reset(page).await?;
    Ok(())
}

/// "allows changing the threshold": with 1500 ms, nothing after 600 ms (beyond the default's
/// 500 ms), the long press later.
pub async fn custom_threshold(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let log = log(page).await?;
    let threshold = target(page, "threshold").await?;
    threshold.dispatch(touch("pointerdown")).await?;
    log.wait_for_inner_text("threshold:longpressstart:touch")
        .await?;
    page.settle().await?;
    assert_that!(|| log.inner_text())
        .consistently_ok()
        .for_at_least(Duration::from_millis(600))
        .matches(eq("threshold:longpressstart:touch"))
        .await;
    log.wait_for_inner_text(
        "threshold:longpressstart:touch,threshold:longpressend:touch,threshold:longpress:touch",
    )
    .await?;
    threshold.dispatch(touch("pointerup")).await?;
    reset(page).await?;
    Ok(())
}

/// "supports accessibilityDescription", "does not show accessibilityDescription if disabled",
/// "... if no onLongPress handler".
pub async fn accessibility_description(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    assert_that!(
        target(page, "description")
            .await?
            .referenced_text("aria-describedby")
            .await?
    )
    .is_equal_to("Long press to open a menu");
    for name in ["description-disabled", "description-no-handler"] {
        assert_that!(target(page, name).await?.attr("aria-describedby").await?)
            .with_detail_message(name)
            .is_none();
    }
    Ok(())
}

/// "prevents context menu events on touch", but only during the press: the blocker goes 100 ms
/// after the pointer up (upstream: "If no contextmenu/click event is fired quickly after
/// pointerup, remove the handler"), so a later context menu opens.
pub async fn prevents_context_menu_during_touch(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let basic = target(page, "basic").await?;
    let context_menu = || SyntheticEvent::mouse("contextmenu");

    basic.dispatch(touch("pointerdown")).await?;
    assert_that!(basic.dispatch(context_menu()).await?.default_prevented).is_true();
    basic.dispatch(touch("pointerup")).await?;
    log(page)
        .await?
        .wait_for_inner_text("basic:longpressstart:touch,basic:longpressend:touch")
        .await?;
    reset(page).await?;

    basic.dispatch(touch("pointerdown")).await?;
    basic.dispatch(touch("pointerup")).await?;
    // A real timer: past the blocker's 100 ms.
    tokio::time::sleep(Duration::from_millis(300)).await;
    assert_that!(basic.dispatch(context_menu()).await?.default_prevented).is_false();
    reset(page).await?;
    Ok(())
}

/// "should not fire any events for keyboard interactions" (long press events, that is), also
/// past the 500 ms threshold.
pub async fn no_long_press_by_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let log = log(page).await?;
    target(page, "with-press").await?.focus().await?;
    page.send_keys(Key::Enter).await?;
    log.wait_for_inner_text(
        "with-press:pressstart:keyboard,with-press:pressend:keyboard,with-press:press:keyboard",
    )
    .await?;
    page.settle().await?;
    assert_that!(|| log.inner_text())
        .consistently_ok()
        .for_at_least(Duration::from_millis(600))
        .matches(eq(
            "with-press:pressstart:keyboard,with-press:pressend:keyboard,with-press:press:keyboard",
        ))
        .await;
    Ok(())
}

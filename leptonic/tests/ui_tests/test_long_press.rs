// Upstream: react-aria/test/interactions/useLongPress.test.js @ 99e6102368
//! Long presses through `use_press`: start, end and the long press after the threshold, which
//! cancels the press; cancelled when released early; a custom threshold; the accessibility
//! description; no context menu on touch (only during the press); nothing for the keyboard.
use std::time::Duration;

use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, MouseKind, Page, PointerKind, PointerType, SyntheticEvent};

const PATH: &str = "/hooks/long-press";

/// The log of press and long press events.
async fn log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-long-press-log").await
}

/// The long-pressable `#test-long-press-<name>`.
async fn target(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    page.element(format!("#test-long-press-{name}")).await
}

/// A touch held past the 500 ms threshold fires long press start, then end and the long press,
/// and the release fires nothing more ("should perform a long press").
#[browser_test]
pub async fn long_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let log = log(page).await?;
    let basic = target(page, "basic").await?;
    basic
        .dispatch(SyntheticEvent::pointer(PointerKind::Down).pointer_type(PointerType::Touch))
        .await?;
    // Well before the 500 ms threshold: an explicit observation window.
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
    basic
        .dispatch(SyntheticEvent::pointer(PointerKind::Up).pointer_type(PointerType::Touch))
        .await?;
    // Past the press's click fallback (80 ms after a pointer up without a click).
    page.settle().await?;
    assert_that!(|| log.inner_text())
        .consistently_ok()
        .for_at_least(Duration::from_millis(300))
        .matches(eq(
            "basic:longpressstart:touch,basic:longpressend:touch,basic:longpress:touch",
        ))
        .await;
    Ok(())
}

/// A touch released before the threshold ends the long press without firing it, also once the
/// threshold has passed ("should cancel if pointer ends before timeout").
#[browser_test]
pub async fn cancelled_when_released_early(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let log = log(page).await?;
    let basic = target(page, "basic").await?;
    basic
        .dispatch(SyntheticEvent::pointer(PointerKind::Down).pointer_type(PointerType::Touch))
        .await?;
    basic
        .dispatch(SyntheticEvent::pointer(PointerKind::Up).pointer_type(PointerType::Touch))
        .await?;
    log.wait_for_inner_text("basic:longpressstart:touch,basic:longpressend:touch")
        .await?;
    page.settle().await?;
    assert_that!(|| log.inner_text())
        .consistently_ok()
        .for_at_least(Duration::from_millis(700))
        .matches(eq("basic:longpressstart:touch,basic:longpressend:touch"))
        .await;
    Ok(())
}

/// A long press ends the element's ongoing press before it fires ("should cancel other press
/// events").
#[browser_test]
pub async fn cancels_other_press_events(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let with_press = target(page, "with-press").await?;
    with_press
        .dispatch(SyntheticEvent::pointer(PointerKind::Down).pointer_type(PointerType::Touch))
        .await?;
    log(page)
        .await?
        .wait_for_inner_text(
            "with-press:longpressstart:touch,with-press:pressstart:touch,\
             with-press:longpressend:touch,with-press:pressend:touch,with-press:longpress:touch",
        )
        .await?;
    with_press
        .dispatch(SyntheticEvent::pointer(PointerKind::Up).pointer_type(PointerType::Touch))
        .await?;
    // No press after the release either (also not from the click fallback, 80 ms after it).
    log(page)
        .await?
        .inner_text_stays(
            "with-press:longpressstart:touch,with-press:pressstart:touch,\
             with-press:longpressend:touch,with-press:pressend:touch,with-press:longpress:touch",
            Duration::from_millis(300),
        )
        .await?;
    Ok(())
}

/// A touch released before the long press threshold still fires a regular press ("should not
/// cancel press events if pointer ends before timer").
#[browser_test]
pub async fn keeps_press_events_when_released_early(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let with_press = target(page, "with-press").await?;
    with_press
        .dispatch(SyntheticEvent::pointer(PointerKind::Down).pointer_type(PointerType::Touch))
        .await?;
    with_press
        .dispatch(SyntheticEvent::pointer(PointerKind::Up).pointer_type(PointerType::Touch))
        .await?;
    log(page)
        .await?
        .wait_for_inner_text(
            "with-press:longpressstart:touch,with-press:pressstart:touch,\
             with-press:longpressend:touch,with-press:pressend:touch,with-press:press:touch",
        )
        .await?;
    Ok(())
}

/// With a 1000 ms threshold, the long press doesn't fire after 600 ms (past the default 500 ms)
/// but fires later ("allows changing the threshold").
#[browser_test]
pub async fn custom_threshold(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let log = log(page).await?;
    let threshold = target(page, "threshold").await?;
    threshold
        .dispatch(SyntheticEvent::pointer(PointerKind::Down).pointer_type(PointerType::Touch))
        .await?;
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
    threshold
        .dispatch(SyntheticEvent::pointer(PointerKind::Up).pointer_type(PointerType::Touch))
        .await?;
    Ok(())
}

/// The accessibility description describes a long-pressable element, but not a disabled one or one
/// without a long press handler ("supports accessibilityDescription", "does not show
/// accessibilityDescription if disabled", "does not show accessibilityDescription if no
/// onLongPress handler").
#[browser_test]
pub async fn accessibility_description(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    assert_that!(target(page, "description").await?)
        .accessible_description()
        .await
        .is_equal_to("Long press to open a menu");
    for name in ["description-disabled", "description-no-handler"] {
        assert_that!(target(page, name).await?)
            .attribute("aria-describedby")
            .await
            .with_detail_message(name)
            .is_none();
    }
    Ok(())
}

/// A touch press prevents the context menu, but only until shortly (100 ms) after the release, so
/// a later context menu opens ("prevents context menu events on touch").
#[browser_test]
pub async fn prevents_context_menu_during_touch(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let basic = target(page, "basic").await?;
    let context_menu = || SyntheticEvent::mouse(MouseKind::ContextMenu);

    basic
        .dispatch(SyntheticEvent::pointer(PointerKind::Down).pointer_type(PointerType::Touch))
        .await?;
    assert_that!(basic.dispatch(context_menu()).await?.default_prevented).is_true();
    basic
        .dispatch(SyntheticEvent::pointer(PointerKind::Up).pointer_type(PointerType::Touch))
        .await?;
    log(page)
        .await?
        .wait_for_inner_text("basic:longpressstart:touch,basic:longpressend:touch")
        .await?;

    basic
        .dispatch(SyntheticEvent::pointer(PointerKind::Down).pointer_type(PointerType::Touch))
        .await?;
    basic
        .dispatch(SyntheticEvent::pointer(PointerKind::Up).pointer_type(PointerType::Touch))
        .await?;
    // A real timer: past the blocker's 100 ms.
    tokio::time::sleep(Duration::from_millis(300)).await;
    assert_that!(basic.dispatch(context_menu()).await?.default_prevented).is_false();
    Ok(())
}

/// Enter fires only regular press events, no long press events, also past the 500 ms threshold
/// ("should not fire any events for keyboard interactions").
#[browser_test]
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

/// The click after a mouse long press is prevented: a long-pressed submit button doesn't submit
/// its form. A later click submits (the prevention ends 100 ms after the release).
#[browser_test]
pub async fn prevents_the_click_after_a_long_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let submit = page.element("#test-long-press-submit").await?;
    let held = submit.press_and_hold().await?;
    log(page)
        .await?
        .wait_for_inner_text("submit:longpress:mouse")
        .await?;
    held.release().await?;
    let submits = page.element("#test-long-press-submits").await?;
    submits
        .inner_text_stays("0", Duration::from_millis(300))
        .await?;
    submit.click().await?;
    submits.wait_for_inner_text("1").await?;
    Ok(())
}

/// Dragging out of a long press ends it, dragging back in starts a new one, which fires once its
/// threshold passes: every long press start has one end.
#[browser_test]
pub async fn dragging_out_and_back_in(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let log = log(page).await?;
    let basic = target(page, "basic").await?;
    let elsewhere = page.element("#test-long-press-elsewhere").await?;
    let held = basic.press_and_hold().await?;
    log.wait_for_inner_text("basic:longpressstart:mouse")
        .await?;
    held.move_to(&elsewhere).await?;
    log.wait_for_inner_text("basic:longpressstart:mouse,basic:longpressend:mouse")
        .await?;
    held.move_to(&basic).await?;
    let expected = "basic:longpressstart:mouse,basic:longpressend:mouse,\
                    basic:longpressstart:mouse,basic:longpressend:mouse,basic:longpress:mouse";
    log.wait_for_inner_text(expected).await?;
    held.release().await?;
    log.inner_text_stays(expected, Duration::from_millis(300))
        .await?;
    Ok(())
}

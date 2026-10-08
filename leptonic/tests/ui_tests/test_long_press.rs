// Upstream: react-aria/test/interactions/useLongPress.test.js @ 99e6102368
use std::{borrow::Cow, time::Duration};

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    pages::{ElementActions, Page, PageActions, SyntheticEvent},
    polling::expect,
};

/// Long presses through `use_press`: start, end and the long press after the threshold, which
/// cancels the press; cancelled when released early; a custom threshold; the accessibility
/// description; no context menu on touch (only during the press); nothing for the keyboard.
pub struct LongPressTests {}

#[async_trait]
impl BrowserTest<str> for LongPressTests {
    fn name(&self) -> Cow<'_, str> {
        "long_press_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/hooks/long-press").await?;

        cases!(
            long_press(&page),
            cancelled_when_released_early(&page),
            cancels_other_press_events(&page),
            keeps_press_events_when_released_early(&page),
            custom_threshold(&page),
            accessibility_description(&page),
            prevents_context_menu_during_touch(&page),
            no_long_press_by_keyboard(&page),
        );
        Ok(())
    }
}

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
async fn long_press(page: &Page<'_>) -> Result<(), Report> {
    let log = log(page).await?;
    let basic = target(page, "basic").await?;
    basic.dispatch(touch("pointerdown")).await?;
    // Well before the threshold.
    log.inner_text_stays("basic:longpressstart:touch").await?;
    log.wait_for_inner_text(
        "basic:longpressstart:touch,basic:longpressend:touch,basic:longpress:touch",
    )
    .await?;
    basic.dispatch(touch("pointerup")).await?;
    log.inner_text_stays(
        "basic:longpressstart:touch,basic:longpressend:touch,basic:longpress:touch",
    )
    .await?;
    reset(page).await?;
    Ok(())
}

/// "should cancel if pointer ends before timeout": no long press, also past the threshold.
async fn cancelled_when_released_early(page: &Page<'_>) -> Result<(), Report> {
    let log = log(page).await?;
    let basic = target(page, "basic").await?;
    basic.dispatch(touch("pointerdown")).await?;
    basic.dispatch(touch("pointerup")).await?;
    log.wait_for_inner_text("basic:longpressstart:touch,basic:longpressend:touch")
        .await?;
    expect("the press log")
        .observing(|| log.inner_text())
        .for_at_least(Duration::from_millis(700))
        .to_stay_equal_to("basic:longpressstart:touch,basic:longpressend:touch")
        .await?;
    reset(page).await?;
    Ok(())
}

/// "should cancel other press events".
async fn cancels_other_press_events(page: &Page<'_>) -> Result<(), Report> {
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
async fn keeps_press_events_when_released_early(page: &Page<'_>) -> Result<(), Report> {
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
async fn custom_threshold(page: &Page<'_>) -> Result<(), Report> {
    let log = log(page).await?;
    let threshold = target(page, "threshold").await?;
    threshold.dispatch(touch("pointerdown")).await?;
    log.wait_for_inner_text("threshold:longpressstart:touch")
        .await?;
    expect("the press log")
        .observing(|| log.inner_text())
        .for_at_least(Duration::from_millis(600))
        .to_stay_equal_to("threshold:longpressstart:touch")
        .await?;
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
async fn accessibility_description(page: &Page<'_>) -> Result<(), Report> {
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
async fn prevents_context_menu_during_touch(page: &Page<'_>) -> Result<(), Report> {
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
async fn no_long_press_by_keyboard(page: &Page<'_>) -> Result<(), Report> {
    let log = log(page).await?;
    target(page, "with-press").await?.focus().await?;
    page.send_keys(Key::Enter).await?;
    log.wait_for_inner_text(
        "with-press:pressstart:keyboard,with-press:pressend:keyboard,with-press:press:keyboard",
    )
    .await?;
    expect("the press log")
        .observing(|| log.inner_text())
        .for_at_least(Duration::from_millis(600))
        .to_stay_equal_to(
            "with-press:pressstart:keyboard,with-press:pressend:keyboard,with-press:press:keyboard",
        )
        .await?;
    Ok(())
}

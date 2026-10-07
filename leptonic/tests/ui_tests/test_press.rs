// Upstream: react-aria/test/interactions/usePress.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{Key, WebDriver},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

pub struct PressTests {}

#[async_trait]
impl BrowserTest<str> for PressTests {
    fn name(&self) -> Cow<'_, str> {
        "press_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/hooks/press").await?;

        mouse_click_fires_all_events_in_order(&page).await?;
        keyboard_enter_and_space_press(&page).await?;
        releasing_outside_does_not_press(&page).await?;
        disabled_element_ignores_presses(&page).await?;
        becoming_disabled_cancels_active_press(&page).await?;
        enter_on_checkbox_submits_form(&page).await?;
        prevent_focus_on_press_keeps_the_focus(&page).await?;
        nested_press_stops_by_default(&page).await?;
        nested_press_propagates_when_continued(&page).await?;

        Ok(())
    }
}

const LOG: &str = "test-press-log";

async fn clear_log(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-press-clear").await?;
    page.wait_for_text(LOG, "").await
}

async fn mouse_click_fires_all_events_in_order(page: &Page<'_>) -> Result<(), Report> {
    clear_log(page).await?;
    page.click_element_with_id("test-press-target").await?;
    page.wait_for_text(LOG, "start:mouse,up:mouse,end:mouse,press:mouse")
        .await?;
    assert_that!(page.read_bool("test-press-is-pressed").await?).is_false();
    // Text selection is disabled on the pressed element (not on the text inside it) and restored
    // (react-aria's `state.target`).
    let label = page.element("test-press-label").await?;
    assert_that!(label.attr("style").await?).is_none();
    let target = page.element("test-press-target").await?;
    assert_that!(target.attr("data-leptonic-saved-user-select").await?).is_none();
    Ok(())
}

async fn keyboard_enter_and_space_press(page: &Page<'_>) -> Result<(), Report> {
    // Focus the target first, then start from a clean log.
    page.click_element_with_id("test-press-target").await?;
    clear_log(page).await?;
    page.element("test-press-target")
        .await?
        .send_keys(Key::Enter)
        .await?;
    page.wait_for_text(
        LOG,
        "start:keyboard,up:keyboard,end:keyboard,press:keyboard",
    )
    .await?;

    clear_log(page).await?;
    page.element("test-press-target")
        .await?
        .send_keys(" ")
        .await?;
    page.wait_for_text(
        LOG,
        "start:keyboard,up:keyboard,end:keyboard,press:keyboard",
    )
    .await?;
    Ok(())
}

async fn releasing_outside_does_not_press(page: &Page<'_>) -> Result<(), Report> {
    clear_log(page).await?;
    let target = page.element("test-press-target").await?;
    let elsewhere = page.element("test-press-elsewhere").await?;

    page.driver
        .action_chain()
        .move_to_element_center(&target)
        .click_and_hold()
        .perform()
        .await?;
    page.wait_for_text(LOG, "start:mouse").await?;
    assert_that!(page.read_bool("test-press-is-pressed").await?).is_true();

    page.driver
        .action_chain()
        .move_to_element_center(&elsewhere)
        .release()
        .perform()
        .await?;
    page.wait_for_text(LOG, "start:mouse,end:mouse").await?;
    assert_that!(page.read_bool("test-press-is-pressed").await?).is_false();
    Ok(())
}

async fn disabled_element_ignores_presses(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-press-toggle-disabled")
        .await?;
    clear_log(page).await?;
    page.click_element_with_id("test-press-target").await?;
    // Give a (wrongly) handled press the chance to show up before asserting its absence.
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    assert_that!(page.read_text_of(LOG).await?).is_equal_to(String::new());
    page.click_element_with_id("test-press-toggle-disabled")
        .await?;
    Ok(())
}

/// Upstream fix (react-spectrum #9813): an element that becomes disabled while pressed must end
/// the press instead of staying pressed (a spin button would otherwise keep spinning).
async fn becoming_disabled_cancels_active_press(page: &Page<'_>) -> Result<(), Report> {
    let target = page.element("test-press-self-disabling").await?;
    page.driver
        .action_chain()
        .move_to_element_center(&target)
        .click_and_hold()
        .perform()
        .await?;

    // The press ends while the pointer is still held down.
    page.wait_for_text("test-press-self-disabling-log", "start:mouse,end:mouse")
        .await?;
    assert_that!(
        page.read_bool("test-press-self-disabling-is-pressed")
            .await?
    )
    .is_false();

    page.driver.action_chain().release().perform().await?;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    assert_that!(page.read_text_of("test-press-self-disabling-log").await?)
        .is_equal_to("start:mouse,end:mouse".to_owned());
    Ok(())
}

/// Upstream fix (react-spectrum #9972): Enter on a checkbox must trigger the form's implicit
/// submission instead of being swallowed.
async fn enter_on_checkbox_submits_form(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-press-checkbox").await?;
    page.element("test-press-checkbox")
        .await?
        .send_keys(Key::Enter)
        .await?;
    page.wait_for_text("test-press-submits", "1").await?;
    Ok(())
}

/// "event bubbling: should stop propagation by default": the outer pressable sees nothing of a
/// press on the inner one.
async fn nested_press_stops_by_default(page: &Page<'_>) -> Result<(), Report> {
    let id = "test-press-nested-stop";
    page.click_element_with_id(&format!("{id}-inner")).await?;
    page.wait_for_text(&format!("{id}-inner-log"), "start,up,end,press")
        .await?;
    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_text(
        &format!("{id}-inner-log"),
        "start,up,end,press,start,up,end,press",
    )
    .await?;
    assert_that!(page.read_text_of(&format!("{id}-outer-log")).await?).is_equal_to(String::new());
    Ok(())
}

/// "event bubbling: should allow propagation if continuePropagation is called": the outer
/// pressable is pressed along with the inner one.
async fn nested_press_propagates_when_continued(page: &Page<'_>) -> Result<(), Report> {
    let id = "test-press-nested-continue";
    page.click_element_with_id(&format!("{id}-inner")).await?;
    page.wait_for_text(&format!("{id}-inner-log"), "start,up,end,press")
        .await?;
    page.wait_for_text(
        &format!("{id}-outer-log"),
        "start:mouse,up:mouse,end:mouse,press:mouse",
    )
    .await?;
    Ok(())
}

/// "should not focus the element on click if preventFocusOnPress is true": the focus stays on
/// the previously focused input, which sees no blur, and the press still happens.
async fn prevent_focus_on_press_keeps_the_focus(page: &Page<'_>) -> Result<(), Report> {
    page.element("test-press-keep-input").await?.focus().await?;
    page.click_element_with_id("test-press-keep").await?;
    page.wait_for_text("test-press-keep-presses", "1").await?;
    page.wait_for_active_id("test-press-keep-input").await?;
    assert_that!(page.read_text_of("test-press-keep-blurs").await?).is_equal_to("0".to_owned());
    Ok(())
}

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

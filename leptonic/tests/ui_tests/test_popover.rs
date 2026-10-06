// Upstream: react-aria-components/test/Popover.test.js @ 99e6102368
// Upstream: react-aria-components/test/Dialog.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// `DialogTrigger` + `Popover` atoms: the trigger opens a dialog in the popover and controls it,
/// focus moves into the dialog and back, Escape and outside clicks close it, presses inside don't
/// toggle the trigger, and a non-modal popover closes when focus moves out.
pub struct PopoverTests {}

#[async_trait]
impl BrowserTest<str> for PopoverTests {
    fn name(&self) -> Cow<'_, str> {
        "popover_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/popover").await?;

        // Works with a dialog: the trigger controls the dialog.
        let trigger = page.element("test-popover-trigger").await?;
        assert_that!(trigger.attr("aria-expanded").await?).is_equal_to(Some("false".to_owned()));
        assert_that!(page.count_matching("[role=dialog]").await?).is_equal_to(0);
        trigger.click().await?;
        page.wait_for_selector("[role=dialog]").await?;
        let dialog = page.css("[role=dialog]").await?;
        let dialog_id = dialog.attr("id").await?;
        let controls = trigger.attr("aria-controls").await?;
        assert_that!(controls).is_equal_to(dialog_id.clone());
        assert_that!(trigger.attr("aria-expanded").await?).is_equal_to(Some("true".to_owned()));
        page.wait_for_selector("[data-placement]").await?;
        // Its title names it, not the trigger (Dialog.test.js).
        let title_id = page.css("[role=dialog] h2").await?.attr("id").await?;
        assert_that!(dialog.attr("aria-labelledby").await?).is_equal_to(title_id);

        // Focus moves into the dialog.
        page.wait_for_active_id(&dialog_id.unwrap_or_default())
            .await?;

        // Pressing a button inside the popover doesn't toggle it through the trigger.
        page.click_element_with_id("test-popover-inner").await?;
        page.wait_for_text("test-popover-inner-presses", "1")
            .await?;
        assert_that!(page.count_matching("[role=dialog]").await?).is_equal_to(1);

        // Escape closes it, focus returns to the trigger.
        page.send_keys_to_active(Key::Escape).await?;
        page.wait_for_no_selector("[role=dialog]").await?;
        page.wait_for_active_id("test-popover-trigger").await?;
        page.wait_for_attr(&trigger, "aria-expanded", Some("false"))
            .await?;

        // A click outside a modal popover (on its underlay, which covers the page) closes it.
        page.click_element_with_id("test-popover-trigger").await?;
        page.wait_for_selector("[role=dialog]").await?;
        let outside = page
            .driver
            .execute("return document.elementFromPoint(5, 5)", vec![])
            .await?
            .element()?;
        outside.click().await?;
        page.wait_for_no_selector("[role=dialog]").await?;
        page.wait_for_active_id("test-popover-trigger").await?;

        // A non-modal popover leaves the page usable and closes when focus moves out.
        page.click_element_with_id("test-popover-non-modal-trigger")
            .await?;
        page.wait_for_selector("[role=dialog][aria-label=Info]")
            .await?;
        page.click_element_with_id("test-popover-outside").await?;
        page.wait_for_no_selector("[role=dialog]").await?;
        page.wait_for_active_id("test-popover-outside").await?;

        // Without a title, the trigger names the dialog ("should get default aria label from
        // trigger"); the trigger gets an id for it.
        let untitled = page
            .driver
            .find(By::XPath("//button[normalize-space(.)='Untitled']"))
            .await?;
        untitled.click().await?;
        page.wait_for_selector("[role=dialog]").await?;
        let untitled_id = untitled.attr("id").await?;
        assert_that!(untitled_id.as_deref().unwrap_or_default()).is_not_empty();
        let dialog = page.css("[role=dialog]").await?;
        assert_that!(dialog.attr("aria-labelledby").await?).is_equal_to(untitled_id);
        page.send_keys_to_active(Key::Escape).await?;
        page.wait_for_no_selector("[role=dialog]").await?;

        // Without a dialog inside, the modal popover is the dialog ("applies overlay id to standalone
        // popover", "should handle focus"): the trigger controls it, it is focused and named by the
        // trigger, and focus returns to the trigger when it closes.
        let standalone = page.element("test-popover-standalone-trigger").await?;
        standalone.click().await?;
        page.wait_for_selector("[role=dialog]").await?;
        let dialog = page.css("[role=dialog]").await?;
        let dialog_id = dialog.attr("id").await?;
        assert_that!(standalone.attr("aria-controls").await?).is_equal_to(dialog_id.clone());
        assert_that!(dialog.attr("aria-labelledby").await?)
            .is_equal_to(Some("test-popover-standalone-trigger".to_owned()));
        assert_that!(dialog.text().await?).contains("Standalone content");
        page.wait_for_active_id(&dialog_id.unwrap_or_default())
            .await?;
        page.send_keys_to_active(Key::Escape).await?;
        page.wait_for_no_selector("[role=dialog]").await?;
        page.wait_for_active_id("test-popover-standalone-trigger")
            .await?;
        page.expect_no_page_errors().await
    }
}

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
/// toggle the trigger, and a non-modal popover closes when focus moves out (it contains the focus
/// only while a dialog is inside, decided per opening). A popover keeps the direction of the
/// subtree its trigger is in.
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
        // With a dialog inside, it contains focus (react-aria's `useOverlayFocusContain`): Tab
        // wraps around instead of leaving (which would close it).
        let info_id = page
            .css("[role=dialog][aria-label=Info]")
            .await?
            .attr("id")
            .await?
            .unwrap_or_default();
        page.wait_for_active_id(&info_id).await?;
        page.send_keys_to_active(Key::Tab).await?;
        page.wait_for_active_id("test-popover-non-modal-first")
            .await?;
        page.send_keys_to_active(Key::Tab).await?;
        page.wait_for_active_id("test-popover-non-modal-second")
            .await?;
        page.send_keys_to_active(Key::Tab).await?;
        page.wait_for_active_id("test-popover-non-modal-first")
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

        // "supports isEntering and isExiting props", with CSS animations: entering while its
        // animation runs, then exiting (and still rendered) until the exit animation ended.
        page.click_element_with_id("test-popover-animated-trigger")
            .await?;
        page.wait_for_selector(".test-animated-popover[data-entering]")
            .await?;
        page.wait_for_selector(".test-animated-popover:not([data-entering])")
            .await?;
        page.send_keys_to_active(Key::Escape).await?;
        page.wait_for_selector(".test-animated-popover[data-exiting]")
            .await?;
        page.wait_for_no_selector(".test-animated-popover").await?;
        page.wait_for_active_id("test-popover-animated-trigger")
            .await?;

        // useOverlayPosition.test.tsx: a non-modal popover stays open when an adjacent region
        // scrolls ("should not close the overlay when an adjacent scrollable region scrolls"), and
        // closes when the page scrolls ("should close the overlay when the body scrolls").
        page.click_element_with_id("test-popover-non-modal-trigger")
            .await?;
        page.wait_for_selector("[role=dialog]").await?;
        page.driver
            .execute(
                "document.getElementById('test-popover-adjacent-scroll').dispatchEvent(new Event('scroll'));",
                vec![],
            )
            .await?;
        assert_that!(page.count_matching("[role=dialog]").await?).is_equal_to(1);
        page.driver
            .execute("document.body.dispatchEvent(new Event('scroll'));", vec![])
            .await?;
        page.wait_for_no_selector("[role=dialog]").await?;

        containment_per_opening(&page).await?;
        direction(&page).await?;
        page.expect_no_page_errors().await
    }
}

/// A non-modal popover contains the focus while a dialog is inside; reopened without one, it
/// doesn't (the containment starts over with each opening): Tab leaves it, which closes it.
async fn containment_per_opening(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-popover-toggled-trigger")
        .await?;
    page.wait_for_selector("[role=dialog][aria-label=Toggled]")
        .await?;
    page.driver
        .execute(
            "document.getElementById('test-popover-toggled-second').focus()",
            vec![],
        )
        .await?;
    page.send_keys_to_active(Key::Tab).await?;
    page.wait_for_active_id("test-popover-toggled-first")
        .await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_no_selector("#test-popover-toggled-first")
        .await?;

    // Without the dialog.
    page.click_element_with_id("test-popover-with-dialog")
        .await?;
    page.click_element_with_id("test-popover-toggled-trigger")
        .await?;
    page.wait_for_selector("#test-popover-toggled-second")
        .await?;
    assert_that!(page.count_matching("[role=dialog]").await?).is_equal_to(0);
    page.driver
        .execute(
            "document.getElementById('test-popover-toggled-second').focus()",
            vec![],
        )
        .await?;
    page.wait_for_active_id("test-popover-toggled-second")
        .await?;
    page.send_keys_to_active(Key::Tab).await?;
    page.wait_for_no_selector("#test-popover-toggled-second")
        .await
}

/// The portalled popover renders `dir` from its trigger's locale (react-aria-components).
async fn direction(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-popover-rtl-trigger")
        .await?;
    page.wait_for_selector(".test-popover-rtl").await?;
    let popover = page.css(".test-popover-rtl").await?;
    assert_that!(popover.attr("dir").await?).is_equal_to(Some("rtl".to_owned()));
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_no_selector(".test-popover-rtl").await?;
    page.click_element_with_id("test-popover-trigger").await?;
    page.wait_for_selector(".leptonic-Popover").await?;
    assert_that!(page.css(".leptonic-Popover").await?.attr("dir").await?)
        .is_equal_to(Some("ltr".to_owned()));
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_no_selector(".leptonic-Popover").await
}

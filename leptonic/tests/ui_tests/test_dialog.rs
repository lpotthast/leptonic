// Upstream: react-aria-components/test/Dialog.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{Key, WebDriver},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// Behavior of the dialog hook (through the `Dialog` atom in a modal): `role="dialog"` named by
/// `aria_label`, closing with Escape (the focused dialog's removal must not fail), restoring focus
/// to the opener, and sibling modals.
/// Spec: react-aria-components `Dialog.test.js`.
pub struct DialogTests {}

#[async_trait]
impl BrowserTest<str> for DialogTests {
    fn name(&self) -> Cow<'_, str> {
        "dialog_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/dialog").await?;

        page.click_element_with_id("test-dialog-open").await?;
        page.wait_for_selector("[role=dialog]").await?;
        let dialog = page.css("[role=dialog]").await?;
        assert_that!(dialog.attr("aria-label").await?).is_equal_to(Some("Settings".to_owned()));
        assert_that!(dialog.attr("aria-labelledby").await?).is_none();

        // Focus moves into the dialog; Escape closes it and focus returns to the opener.
        page.wait_for_active_text("Inside").await?;
        page.send_keys_to_active(Key::Escape).await?;
        page.wait_for_no_selector("[role=dialog]").await?;
        page.wait_for_text("test-dialog-is-open", "false").await?;
        page.wait_for_active_id("test-dialog-open").await?;

        // A second modal in the same owner (an alert dialog with `Button` atoms, opened by a
        // `Button` atom) gets its own backdrop's props: it is the topmost overlay, so Escape
        // closes it.
        page.click_element_with_id("test-dialog-open-other").await?;
        page.wait_for_selector("[role=alertdialog]").await?;
        page.wait_for_active_id("test-dialog-other-close").await?;
        // Named by its `DialogTitle` (an `<h2>`) and, as an alert dialog, described by its
        // `DialogDescription`.
        let alert = page.css("[role=alertdialog]").await?;
        let title = page.css("[role=alertdialog] h2").await?;
        let labelled_by = alert.attr("aria-labelledby").await?;
        let title_id = title.attr("id").await?;
        assert_that!(labelled_by).is_equal_to(title_id);
        assert_that!(title.text().await?).is_equal_to("Other".to_owned());
        let description_id = alert.attr("aria-describedby").await?.unwrap_or_default();
        let description = page.css(&format!("[id='{description_id}']")).await?;
        assert_that!(description.text().await?).is_equal_to("Leave this page?".to_owned());
        page.send_keys_to_active(Key::Escape).await?;
        page.wait_for_no_selector("[role=alertdialog]").await?;
        page.wait_for_active_id("test-dialog-open-other").await?;

        // Opened with the keyboard, closed from inside (a button calling `on_close`).
        page.send_keys_to_active(Key::Enter).await?;
        page.wait_for_active_id("test-dialog-other-close").await?;
        page.send_keys_to_active(Key::Enter).await?;
        page.wait_for_no_selector("[role=alertdialog]").await?;
        page.wait_for_active_id("test-dialog-open-other").await?;

        // Opened and closed with the keyboard.
        page.send_keys_to_active(Key::Enter).await?;
        page.wait_for_active_id("test-dialog-other-close").await?;
        page.send_keys_to_active(Key::Escape).await?;
        page.wait_for_no_selector("[role=alertdialog]").await?;
        page.wait_for_active_id("test-dialog-open-other").await?;

        // A button inside a modal opened by a `DialogTrigger` doesn't press through the trigger's
        // responder: it counts, the modal stays open.
        page.click_element_with_id("test-dialog-trigger").await?;
        page.wait_for_selector("[role=dialog][aria-label=Triggered]")
            .await?;
        page.click_element_with_id("test-dialog-count").await?;
        page.wait_for_text("test-dialog-count", "Count 1").await?;
        assert_that!(
            page.count_matching("[role=dialog][aria-label=Triggered]")
                .await?
        )
        .is_equal_to(1);
        page.send_keys_to_active(Key::Escape).await?;
        page.wait_for_no_selector("[role=dialog][aria-label=Triggered]")
            .await?;
        page.wait_for_active_id("test-dialog-trigger").await?;
        // Backdrop and modal animate in and out; they stay rendered until the exit animations ended,
        // then focus returns to the opener.
        page.click_element_with_id("test-dialog-open-animated")
            .await?;
        page.wait_for_selector(".test-animated-modal[data-entering]")
            .await?;
        page.wait_for_selector(".test-animated-modal:not([data-entering])")
            .await?;
        page.send_keys_to_active(Key::Escape).await?;
        page.wait_for_selector(".test-animated-backdrop[data-exiting]")
            .await?;
        page.wait_for_selector(".test-animated-modal[data-exiting]")
            .await?;
        page.wait_for_no_selector(".test-animated-backdrop").await?;
        page.wait_for_active_id("test-dialog-open-animated").await?;

        page.expect_no_page_errors().await
    }
}

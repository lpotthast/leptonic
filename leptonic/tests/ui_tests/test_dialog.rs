// Upstream: react-aria-components/test/Dialog.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions};

/// Behavior of the dialog hook (through the `Dialog` atom in a modal): `role="dialog"` named by
/// `aria_label`, focused when the modal opens (the first button with `auto_focus`), closing with
/// Escape (the focused dialog's removal must not fail) or the dismiss button of a dismissable
/// modal, restoring focus to the opener, sibling modals, and no `aria-modal` (react-aria-components:
/// the inert page makes the modal modal).
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

        cases!(
            dismiss_button_closes(&page),
            escape_closes(&page),
            alert_dialog(&page),
            keyboard_open_and_close_from_inside(&page),
            keyboard_open_and_escape(&page),
            nested_modals(&page),
            animated_modal(&page),
            auto_focus(&page),
        );

        Ok(())
    }
}

/// "should be focused when opened": the dialog itself takes the focus. A dismissable modal has no
/// `aria-modal` (WebKit bug 211934) and starts with a visually hidden dismiss button for screen
/// reader users (react-aria-components' `Modal`), which closes it and restores focus.
async fn dismiss_button_closes(page: &Page<'_>) -> Result<(), Report> {
    let opener = page.element("#test-dialog-open").await?;
    let is_open = page.element("#test-dialog-is-open").await?;
    opener.click().await?;
    let dialog = page.element("[role=dialog]").await?;
    assert_that!(dialog.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Settings");
    assert_that!(dialog.attr("aria-labelledby").await?).is_none();
    page.wait_for_focus(&dialog).await?;
    assert_that!(page.count(".leptonic-ModalContent[aria-modal]").await?).is_equal_to(0);
    assert_that!(
        page.count(".leptonic-ModalContent button[aria-label=Dismiss]")
            .await?
    )
    .is_equal_to(1);

    page.element(".leptonic-ModalContent button[aria-label=Dismiss]")
        .await?
        .virtual_click()
        .await?;
    page.wait_for_count("[role=dialog]", 0).await?;
    is_open.wait_for_inner_text("false").await?;
    page.wait_for_focus(&opener).await?;
    Ok(())
}

/// Escape closes the dialog and focus returns to the opener.
async fn escape_closes(page: &Page<'_>) -> Result<(), Report> {
    let opener = page.element("#test-dialog-open").await?;
    opener.click().await?;
    let dialog = page.element("[role=dialog]").await?;
    page.wait_for_focus(&dialog).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count("[role=dialog]", 0).await?;
    page.element("#test-dialog-is-open")
        .await?
        .wait_for_inner_text("false")
        .await?;
    page.wait_for_focus(&opener).await?;
    Ok(())
}

/// A second modal in the same owner (an alert dialog with `Button` atoms, opened by a `Button`
/// atom) gets its own backdrop's props: it is the topmost overlay, so Escape closes it. The alert
/// dialog takes the focus, not its first (maybe destructive) button; it is not dismissable, named
/// by its `DialogTitle` and described by its `DialogDescription`.
async fn alert_dialog(page: &Page<'_>) -> Result<(), Report> {
    let opener = page.element("#test-dialog-open-other").await?;
    opener.click().await?;
    let alert = page.element("[role=alertdialog]").await?;
    page.wait_for_focus(&alert).await?;
    assert_that!(
        page.count("[role=alertdialog] button[aria-label=Dismiss]")
            .await?
    )
    .is_equal_to(0);
    let title = alert.element("h2").await?;
    let title_id = title.id().await?;
    assert_that!(alert.attr("aria-labelledby").await?).is_equal_to(title_id);
    assert_that!(title.inner_text().await?).is_equal_to("Other");
    assert_that!(alert.referenced_text("aria-describedby").await?).is_equal_to("Leave this page?");

    page.send_keys(Key::Escape).await?;
    page.wait_for_count("[role=alertdialog]", 0).await?;
    page.wait_for_focus(&opener).await?;
    Ok(())
}

/// Opened with the keyboard, closed from inside (a button calling `on_close`).
async fn keyboard_open_and_close_from_inside(page: &Page<'_>) -> Result<(), Report> {
    let opener = page.element("#test-dialog-open-other").await?;
    page.wait_for_focus(&opener).await?;
    page.send_keys(Key::Enter).await?;
    page.wait_for_focus(&page.element("[role=alertdialog]").await?)
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-dialog-other-close").await?)
        .await?;
    page.send_keys(Key::Enter).await?;
    page.wait_for_count("[role=alertdialog]", 0).await?;
    page.wait_for_focus(&opener).await?;
    Ok(())
}

/// Opened and closed with the keyboard.
async fn keyboard_open_and_escape(page: &Page<'_>) -> Result<(), Report> {
    let opener = page.element("#test-dialog-open-other").await?;
    page.wait_for_focus(&opener).await?;
    page.send_keys(Key::Enter).await?;
    page.wait_for_focus(&page.element("[role=alertdialog]").await?)
        .await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count("[role=alertdialog]", 0).await?;
    page.wait_for_focus(&opener).await?;
    Ok(())
}

/// A button inside a modal opened by a `DialogTrigger` doesn't press through the trigger's
/// responder: it counts, the modal stays open. A modal nested in its markup: Escape closes only
/// the nested one, focus returns to its trigger inside the outer modal.
async fn nested_modals(page: &Page<'_>) -> Result<(), Report> {
    const TRIGGERED: &str = "[role=dialog][aria-label=Triggered]";
    const NESTED: &str = "[role=dialog][aria-label=Nested]";
    let trigger = page.element("#test-dialog-trigger").await?;
    trigger.click().await?;
    page.element(TRIGGERED).await?;
    let count = page.element("#test-dialog-count").await?;
    count.click().await?;
    count.wait_for_inner_text("Count 1").await?;
    assert_that!(page.count(TRIGGERED).await?).is_equal_to(1);

    let nested_trigger = page.element("#test-dialog-nested-trigger").await?;
    nested_trigger.click().await?;
    page.wait_for_focus(&page.element(NESTED).await?).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(NESTED, 0).await?;
    page.wait_for_focus(&nested_trigger).await?;
    assert_that!(page.count(TRIGGERED).await?).is_equal_to(1);

    page.send_keys(Key::Escape).await?;
    page.wait_for_count(TRIGGERED, 0).await?;
    page.wait_for_focus(&trigger).await?;
    Ok(())
}

/// Backdrop and modal animate in and out; they stay rendered until the exit animations ended,
/// then focus returns to the opener.
async fn animated_modal(page: &Page<'_>) -> Result<(), Report> {
    let opener = page.element("#test-dialog-open-animated").await?;
    opener.click().await?;
    page.element(".test-animated-modal[data-entering]").await?;
    page.element(".test-animated-modal:not([data-entering])")
        .await?;
    page.send_keys(Key::Escape).await?;
    page.element(".test-animated-backdrop[data-exiting]")
        .await?;
    page.element(".test-animated-modal[data-exiting]").await?;
    page.wait_for_count(".test-animated-backdrop", 0).await?;
    page.wait_for_focus(&opener).await?;
    Ok(())
}

/// Opting into `auto_focus`: the first button takes the focus instead of the dialog.
async fn auto_focus(page: &Page<'_>) -> Result<(), Report> {
    const DIALOG: &str = "[role=dialog][aria-label='Auto focus']";
    let opener = page.element("#test-dialog-open-autofocus").await?;
    opener.click().await?;
    page.element(DIALOG).await?;
    page.wait_for_focus(&page.element("#test-dialog-autofocus-first").await?)
        .await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(DIALOG, 0).await?;
    page.wait_for_focus(&opener).await?;
    Ok(())
}

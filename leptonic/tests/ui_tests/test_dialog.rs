// Upstream: react-aria-components/test/Dialog.test.js @ 99e6102368
// Upstream: react-aria/test/dialog/useDialog.test.js @ 99e6102368
//! Behavior of the dialog hook (through the `Dialog` atom in a modal): `role="dialog"` named by
//! `aria_label`, focused when the modal opens (the first button with `auto_focus`), closing with
//! Escape (the focused dialog's removal must not fail) or the dismiss button of a dismissable
//! modal, restoring focus to the opener, sibling modals, and no `aria-modal` (react-aria-components:
//! the inert page makes the modal modal); descriptions, the missing-title warning, and focus kept
//! inside a shadow root.
//! Spec: react-aria-components `Dialog.test.js`.
use assertr::{
    matchers::{eq, satisfying},
    prelude::*,
};
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page, WebElement};

const PATH: &str = "/atoms/dialog";

/// Opening a dismissable modal focuses its dialog, named by `aria_label`, without `aria-modal`;
/// its visually hidden Dismiss button closes it and returns focus to the opener ("has dismiss
/// button when isDismissable").
#[browser_test]
pub async fn dismiss_button_closes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let opener = page.element("#test-dialog-open").await?;
    let is_open = page.element("#test-dialog-is-open").await?;
    opener.click().await?;
    let dialog = page.element("[role=dialog]").await?;
    assert_that!(dialog)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Settings");
    assert_that!(dialog)
        .attribute("aria-labelledby")
        .await
        .is_none();
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
#[browser_test]
pub async fn escape_closes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
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

/// An alert dialog takes the focus itself rather than its first button, has no Dismiss button and
/// is named by its title and described by its description; Escape closes it and returns focus to
/// the opener ("works with modal", "should set aria-describedby when Text slot="description" is
/// used in alertdialog").
#[browser_test]
pub async fn alert_dialog(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
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
    assert_that!(alert)
        .attribute("aria-labelledby")
        .await
        .is_equal_to(title_id);
    assert_that!(title).inner_text().await.is_equal_to("Other");
    assert_that!(alert)
        .accessible_description()
        .await
        .is_equal_to("Leave this page?");

    page.send_keys(Key::Escape).await?;
    page.wait_for_count("[role=alertdialog]", 0).await?;
    page.wait_for_focus(&opener).await?;
    Ok(())
}

/// An alert dialog opened with Enter closes when its own close button is pressed with Enter, and
/// focus returns to the opener ("works with modal").
#[browser_test]
pub async fn keyboard_open_and_close_from_inside(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let opener = page.element("#test-dialog-open-other").await?;
    opener.focus().await?;
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

/// An alert dialog opened with Enter takes the focus and closes on Escape, returning focus to the
/// opener.
#[browser_test]
pub async fn keyboard_open_and_escape(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let opener = page.element("#test-dialog-open-other").await?;
    opener.focus().await?;
    page.wait_for_focus(&opener).await?;
    page.send_keys(Key::Enter).await?;
    page.wait_for_focus(&page.element("[role=alertdialog]").await?)
        .await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count("[role=alertdialog]", 0).await?;
    page.wait_for_focus(&opener).await?;
    Ok(())
}

/// A button inside a modal opened by a `DialogTrigger` works without pressing the trigger, so the
/// modal stays open; Escape in a modal nested in it closes only the nested one and returns focus
/// to its trigger.
#[browser_test]
pub async fn nested_modals(page: &Page<'_>) -> Result<(), Report> {
    const TRIGGERED: &str = "[role=dialog][aria-label=Triggered]";
    const NESTED: &str = "[role=dialog][aria-label=Nested]";
    page.goto_path(PATH).await?;
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
#[browser_test]
pub async fn animated_modal(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
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

/// With `auto_focus`, the dialog's first button takes the focus instead of the dialog; Escape
/// closes it and returns focus to the opener.
#[browser_test]
pub async fn auto_focus(page: &Page<'_>) -> Result<(), Report> {
    const DIALOG: &str = "[role=dialog][aria-label='Auto focus']";
    page.goto_path(PATH).await?;
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

/// Opens the modal of `#test-dialog-open-<name>` and returns its dialog.
async fn open(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    page.goto_path(PATH).await?;
    page.element(format!("#test-dialog-open-{name}"))
        .await?
        .click()
        .await?;
    page.element(".leptonic-ModalContent .leptonic-Dialog")
        .await
}

/// A regular dialog isn't described by its description, which still has an id (useDialog.test.js
/// "should not auto-wire aria-describedby on regular dialog, but contentProps.id is still
/// provided").
#[browser_test]
pub async fn regular_dialog_not_described(page: &Page<'_>) -> Result<(), Report> {
    let dialog = open(page, "described").await?;
    assert_that!(dialog)
        .has_attribute("role")
        .await
        .is_equal_to("dialog");
    assert_that!(dialog)
        .attribute("aria-describedby")
        .await
        .is_none();
    let description = dialog.element(".leptonic-DialogDescription").await?;
    assert_that!(description)
        .has_attribute("id")
        .await
        .is_not_empty();
    Ok(())
}

/// An alert dialog's `aria_describedby` replaces its description as what describes it
/// (useDialog.test.js "should allow aria-describedby override on alertdialog").
#[browser_test]
pub async fn alert_dialog_describedby_override(page: &Page<'_>) -> Result<(), Report> {
    let dialog = open(page, "override").await?;
    assert_that!(dialog)
        .has_attribute("aria-describedby")
        .await
        .is_equal_to("test-dialog-custom-description");
    assert_that!(dialog)
        .accessible_description()
        .await
        .is_equal_to("A custom description.");
    Ok(())
}

/// A dialog without title, `aria_label` or `aria_labelledby` logs a warning once rendered
/// (useDialog.test.js "should warn when dialog has no accessible title").
#[browser_test]
pub async fn untitled_dialog_warns(page: &Page<'_>) -> Result<(), Report> {
    open(page, "untitled").await?;
    assert_that!(|| warnings(page))
        .eventually_ok()
        .satisfies(|warnings| {
            warnings.contains_matching(satisfying(|warning: AssertThat<String, Capture>| {
                warning.contains("A dialog must have a title");
            }));
        })
        .await;
    // The warning was expected: the page check must not fail on it.
    crate::fixtures::take_warnings(page, "A dialog must have a title", 1).await?;
    Ok(())
}

/// Dialogs named by `aria_label`, by `aria_labelledby` or by a title log no warning
/// (useDialog.test.js "should not warn when aria-label is provided", "should not warn when
/// aria-labelledby is provided", "should not warn when a title element is rendered").
#[browser_test]
pub async fn named_dialogs_dont_warn(page: &Page<'_>) -> Result<(), Report> {
    for name in ["autofocus", "labelledby", "described"] {
        open(page, name).await?;
        // The check runs once the dialog rendered (an effect after mount).
        page.settle().await?;
        assert_that!(|| warnings(page))
            .consistently_ok()
            .for_at_least(std::time::Duration::from_millis(100))
            .matches(eq(Vec::<String>::new()))
            .await;
    }
    Ok(())
}

/// A dialog whose content focused an element inside a shadow root when it mounted keeps that
/// focus instead of taking it (useDialog.test.js "should not focus the overlay if something inside
/// is auto focused", across a shadow root).
#[browser_test]
pub async fn keeps_focus_inside_a_shadow_root(page: &Page<'_>) -> Result<(), Report> {
    open(page, "shadow").await?;
    let host = page.element("#test-dialog-shadow-host").await?;
    page.wait_for_focus(&host).await?;
    page.focus_stays(&host, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// The console warnings the page logged.
async fn warnings(page: &Page<'_>) -> Result<Vec<String>, Report> {
    Ok(crate::pages::health::diagnostics(page.low_level().driver())
        .await?
        .console_warnings)
}

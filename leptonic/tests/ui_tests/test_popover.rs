// Upstream: react-aria-components/test/Popover.test.js @ 99e6102368
// Upstream: react-aria-components/test/Dialog.test.js @ 99e6102368
//! `DialogTrigger` + `Popover` atoms: the trigger opens a dialog in the popover and controls it,
//! focus moves into the dialog and back, Escape and outside clicks close it, presses inside don't
//! toggle the trigger, and a non-modal popover closes when focus moves out (it contains the focus
//! only while a dialog is inside, decided per opening). A popover keeps the direction of the
//! subtree its trigger is in.
use assertr::prelude::*;
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::Report;

use crate::pages::{ElementActions, EventKind, GlobalTarget, Page, SyntheticEvent, role};

const PATH: &str = "/atoms/popover";

const DIALOG: &str = "[role=dialog]";

/// The trigger opens and controls a focused dialog named by its title, presses inside keep it open,
/// and Escape closes it with the focus back on the trigger ("works with a dialog").
#[browser_test]
pub async fn trigger_controls_the_dialog(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element("#test-popover-trigger").await?;
    assert_that!(trigger)
        .has_attribute("aria-expanded")
        .await
        .is_equal_to("false");
    assert_that!(page.count(DIALOG).await?).is_equal_to(0);

    trigger.click().await?;
    let dialog = page.element(DIALOG).await?;
    let dialog_id = dialog.id().await?;
    assert_that!(trigger)
        .attribute("aria-controls")
        .await
        .is_equal_to(dialog_id);
    assert_that!(trigger)
        .has_attribute("aria-expanded")
        .await
        .is_equal_to("true");
    page.element("[data-placement]").await?;
    let title_id = dialog.element("h2").await?.id().await?;
    assert_that!(dialog)
        .attribute("aria-labelledby")
        .await
        .is_equal_to(title_id);
    page.wait_for_focus(&dialog).await?;

    page.element("#test-popover-inner").await?.click().await?;
    page.element("#test-popover-inner-presses")
        .await?
        .wait_for_inner_text("1")
        .await?;
    assert_that!(page.count(DIALOG).await?).is_equal_to(1);

    page.send_keys(Key::Escape).await?;
    page.wait_for_count(DIALOG, 0).await?;
    page.wait_for_focus(&trigger).await?;
    trigger
        .wait_for_attr("aria-expanded", Some("false"))
        .await?;
    Ok(())
}

/// A click outside a modal popover (on its underlay) closes it and returns the focus to the
/// trigger.
#[browser_test]
pub async fn outside_click_closes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element("#test-popover-trigger").await?;
    trigger.click().await?;
    page.element(DIALOG).await?;
    page.click_at(5, 5).await?;
    page.wait_for_count(DIALOG, 0).await?;
    page.wait_for_focus(&trigger).await?;
    Ok(())
}

/// A non-modal popover with a dialog inside contains the focus (Tab wraps around), and a click on
/// the still usable page outside closes it and focuses the clicked element.
#[browser_test]
pub async fn non_modal_contains_focus_with_a_dialog(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-popover-toggled-trigger")
        .await?
        .click()
        .await?;
    let info = page.element("[role=dialog][aria-label=Toggled]").await?;
    page.wait_for_focus(&info).await?;
    let first = page.element("#test-popover-toggled-first").await?;
    let second = page.element("#test-popover-toggled-second").await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&first).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&second).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&first).await?;

    let outside = page.element("#test-popover-outside").await?;
    outside.click().await?;
    page.wait_for_count(DIALOG, 0).await?;
    page.wait_for_focus(&outside).await?;
    Ok(())
}

/// A dialog without a title is labelled by its trigger, which gets an id for it ("should get
/// default aria label from trigger").
#[browser_test]
pub async fn trigger_names_an_untitled_dialog(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let untitled = page
        .element(role(AriaRole::Button).text("Untitled"))
        .await?;
    untitled.click().await?;
    let dialog = page.element(DIALOG).await?;
    let untitled_id = assert_that!(untitled)
        .has_attribute("id")
        .await
        .is_not_empty()
        .actual()
        .clone();
    assert_that!(dialog)
        .has_attribute("aria-labelledby")
        .await
        .is_equal_to(untitled_id);
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(DIALOG, 0).await?;
    Ok(())
}

/// A modal popover without a dialog inside is itself the dialog, controlled and labelled by the
/// trigger and focused, and the focus returns to the trigger on close ("applies overlay id to
/// standalone popover", "should handle focus").
#[browser_test]
pub async fn standalone_popover_is_the_dialog(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let standalone = page.element("#test-popover-standalone-trigger").await?;
    standalone.click().await?;
    let dialog = page.element(DIALOG).await?;
    let dialog_id = dialog.id().await?;
    assert_that!(standalone)
        .attribute("aria-controls")
        .await
        .is_equal_to(dialog_id);
    assert_that!(dialog)
        .has_attribute("aria-labelledby")
        .await
        .is_equal_to("test-popover-standalone-trigger");
    assert_that!(dialog)
        .inner_text()
        .await
        .is_equal_to("Standalone content");
    page.wait_for_focus(&dialog).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(DIALOG, 0).await?;
    page.wait_for_focus(&standalone).await?;
    Ok(())
}

/// An animated popover has `data-entering` while its entry animation runs and stays rendered with
/// `data-exiting` until its exit animation ends ("supports isEntering and isExiting props").
#[browser_test]
pub async fn animated(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element("#test-popover-animated-trigger").await?;
    trigger.click().await?;
    page.element(".test-animated-popover[data-entering]")
        .await?;
    page.element(".test-animated-popover:not([data-entering])")
        .await?;
    page.send_keys(Key::Escape).await?;
    page.element(".test-animated-popover[data-exiting]").await?;
    page.wait_for_count(".test-animated-popover", 0).await?;
    page.wait_for_focus(&trigger).await?;
    Ok(())
}

/// A non-modal popover stays open when an adjacent region scrolls and closes when the page scrolls
/// (useOverlayPosition.test.tsx "should not close the overlay when an adjacent scrollable region
/// scrolls", "should close the overlay when the body scrolls").
#[browser_test]
pub async fn scrolling(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-popover-toggled-trigger")
        .await?
        .click()
        .await?;
    page.element(DIALOG).await?;
    page.element("#test-popover-adjacent-scroll")
        .await?
        .dispatch(SyntheticEvent::plain(EventKind::Scroll).bubbles(false))
        .await?;
    page.count_stays(DIALOG, 1, std::time::Duration::from_millis(100))
        .await?;
    page.element("body")
        .await?
        .dispatch(SyntheticEvent::plain(EventKind::Scroll).bubbles(false))
        .await?;
    page.wait_for_count(DIALOG, 0).await?;
    Ok(())
}

/// A non-modal popover closes when the document or the window scrolls ("should close the overlay
/// when the document scrolls", "should close the overlay when target is window in a scroll event",
/// useOverlayPosition.test.tsx).
#[browser_test]
pub async fn closes_on_document_and_window_scroll(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element("#test-popover-toggled-trigger").await?;
    for target in [GlobalTarget::Document, GlobalTarget::Window] {
        trigger.click().await?;
        page.element(DIALOG).await?;
        page.dispatch_to(
            target,
            SyntheticEvent::plain(EventKind::Scroll).bubbles(false),
        )
        .await?;
        page.wait_for_count(DIALOG, 0).await?;
    }
    Ok(())
}

/// A modal popover stays open when the page scrolls (usePopover.test.tsx "should not close popover
/// on scroll").
#[browser_test]
pub async fn modal_stays_open_on_scroll(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-popover-trigger").await?.click().await?;
    page.element(DIALOG).await?;
    page.element("body")
        .await?
        .dispatch(SyntheticEvent::plain(EventKind::Scroll).bubbles(false))
        .await?;
    page.dispatch_to(
        GlobalTarget::Document,
        SyntheticEvent::plain(EventKind::Scroll).bubbles(false),
    )
    .await?;
    page.count_stays(DIALOG, 1, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A non-modal popover contains the focus while a dialog is inside, but reopened without one it
/// doesn't, so Shift+Tab leaves and closes it.
#[browser_test]
pub async fn containment_per_opening(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element("#test-popover-toggled-trigger").await?;
    trigger.click().await?;
    page.element("[role=dialog][aria-label=Toggled]").await?;
    page.element("#test-popover-toggled-first")
        .await?
        .focus()
        .await?;
    // Contained: Shift+Tab wraps around.
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-popover-toggled-second").await?)
        .await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count("#test-popover-toggled-first", 0)
        .await?;

    // Without the dialog.
    page.element("#test-popover-with-dialog")
        .await?
        .click()
        .await?;
    trigger.click().await?;
    page.element("#test-popover-toggled-second").await?;
    assert_that!(page.count(DIALOG).await?).is_equal_to(0);
    let first = page.element("#test-popover-toggled-first").await?;
    first.focus().await?;
    page.wait_for_focus(&first).await?;
    // Not contained: Shift+Tab moves to the page before the popover, which closes it.
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_count("#test-popover-toggled-second", 0)
        .await?;
    Ok(())
}

/// The portalled popover gets the `dir` of its trigger's locale: `rtl` in a right-to-left subtree,
/// `ltr` elsewhere.
#[browser_test]
pub async fn direction(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-popover-rtl-trigger")
        .await?
        .click()
        .await?;
    let popover = page.element(".test-popover-rtl").await?;
    assert_that!(popover)
        .has_attribute("dir")
        .await
        .is_equal_to("rtl");
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(".test-popover-rtl", 0).await?;

    page.element("#test-popover-trigger").await?.click().await?;
    let popover = page.element(".leptonic-Popover").await?;
    assert_that!(popover)
        .has_attribute("dir")
        .await
        .is_equal_to("ltr");
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(".leptonic-Popover", 0).await?;
    Ok(())
}

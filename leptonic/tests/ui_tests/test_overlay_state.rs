// Upstream: react-aria-components/test/Popover.test.js @ 99e6102368
// Upstream: react-aria-components/test/Dialog.test.js @ 99e6102368
// Upstream: react-aria/test/overlays/useModalOverlay.test.js @ 99e6102368
//! Overlays with their own open state and their animation callbacks: a popover or modal with its
//! own `is_open` inside a `DialogTrigger` follows that state, a popover works without a
//! `DialogTrigger`, a popover inside a modal closes alone, a modal's outside-interaction filter
//! decides, and `on_enter`/`on_exit` get called (an animation `on_exit` starts keeps the modal).
use std::time::Duration;

use assertr::prelude::*;
use browser_test::{browser_test, thirtyfour::prelude::Key};
use leptonic::AriaRole;
use rootcause::Report;

use crate::pages::{ElementActions, Page, role};

const PATH: &str = "/atoms/overlay-state";
const LOG: &str = "#test-os-log";

/// A popover with its own `is_open` inside a `DialogTrigger` is open from the start, and an outside
/// press reports `false` once to its own `on_open_change` ("isOpen and defaultOpen should override
/// state from context", Popover.test.js).
#[browser_test]
pub async fn popover_state_overrides_the_trigger(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["popover-override"]).await?;
    let dialog = page.element("[role=dialog][aria-label=Popover]").await?;
    assert_that!(dialog)
        .inner_text()
        .await
        .is_equal_to("A popover");
    page.click_at(5, 5).await?;
    page.element(LOG)
        .await?
        .wait_for_inner_text("open:false")
        .await?;
    page.element(LOG)
        .await?
        .inner_text_stays("open:false", Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A dismissable modal with its own `is_open` inside a `DialogTrigger` is open from the start, and
/// an outside press reports `false` once to its own `on_open_change` ("isOpen and defaultOpen
/// should override state from context", Dialog.test.js).
#[browser_test]
pub async fn modal_state_overrides_the_trigger(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["modal-override"]).await?;
    let dialog = page.element("[role=dialog][aria-label=Modal]").await?;
    assert_that!(dialog)
        .inner_text()
        .await
        .is_equal_to("A modal");
    page.click_at(5, 590).await?;
    let log = page.element(LOG).await?;
    log.wait_for_inner_text("open:false").await?;
    log.inner_text_stays("open:false", Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A popover outside any `DialogTrigger`, with its own `trigger` and `is_open`, is open and reports
/// an outside press once to its `on_open_change` ("should support being used standalone").
#[browser_test]
pub async fn standalone_popover(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["standalone"]).await?;
    page.element("[role=dialog][aria-label=Standalone]").await?;
    page.element(".leptonic-Popover[data-placement=bottom]")
        .await?;
    page.click_at(5, 5).await?;
    let log = page.element(LOG).await?;
    log.wait_for_inner_text("open:false").await?;
    log.inner_text_stays("open:false", Duration::from_millis(100))
        .await?;
    Ok(())
}

/// An outside press closes a popover opened inside a modal, but not the modal: the press lands on
/// the modal, outside the popover (Dialog.test.js "should not close Modal when DateRangePicker is
/// dismissed by outside click", with a popover).
#[browser_test]
pub async fn popover_in_a_modal_closes_alone(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["popover-in-modal"]).await?;
    page.element("#test-os-open-modal").await?.click().await?;
    page.element("[role=dialog][aria-label=Outer]").await?;
    let trigger = page.element("#test-os-popover-trigger").await?;
    trigger.click().await?;
    let inner = page.element("[role=dialog][aria-label=Inner]").await?;
    page.wait_for_focus(&inner).await?;
    let text = page.element("#test-os-modal-text").await?;
    // The inner modal popover makes the outer content inert. Press at its position so the
    // browser delivers the outside interaction to the actual hit target.
    page.low_level()
        .driver()
        .action_chain()
        .move_to_element_center(&text)
        .click()
        .perform()
        .await?;
    page.wait_for_count("[role=dialog][aria-label=Inner]", 0)
        .await?;
    page.count_stays(
        "[role=dialog][aria-label=Outer]",
        1,
        Duration::from_millis(100),
    )
    .await?;
    Ok(())
}

/// A dismissable modal closes on an outside press its filter accepts ("should hide the overlay
/// when clicking outside if shouldCloseOnInteractOutside returns true").
#[browser_test]
pub async fn modal_filter_closes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["modal-filter-close"]).await?;
    page.element("[role=dialog][aria-label=Closing]").await?;
    page.click_at(5, 590).await?;
    page.wait_for_count("[role=dialog][aria-label=Closing]", 0)
        .await?;
    page.element(LOG)
        .await?
        .wait_for_inner_text("open:false")
        .await?;
    Ok(())
}

/// A dismissable modal stays open on an outside press its filter rejects ("should not hide the
/// overlay when clicking outside if shouldCloseOnInteractOutside returns false").
#[browser_test]
pub async fn modal_filter_keeps(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["modal-filter-keep"]).await?;
    page.element("[role=dialog][aria-label=Keeping]").await?;
    page.click_at(5, 590).await?;
    page.count_stays(
        "[role=dialog][aria-label=Keeping]",
        1,
        Duration::from_millis(100),
    )
    .await?;
    page.element(LOG)
        .await?
        .inner_text_stays("", Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Opening calls the backdrop's and the modal's `on_enter`, closing their `on_exit`; an animation
/// the modal's `on_exit` starts keeps the modal rendered (`data-exiting`) until it ends ("calls
/// onEnter on ModalOverlay and Modal with their elements when opening", "calls onExit on
/// ModalOverlay and Modal with their elements when closing", "remains mounted until the promise
/// returned by Modal onExit resolves", with an animation for the promise).
#[browser_test]
pub async fn enter_and_exit_callbacks(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["enter-exit"]).await?;
    page.element("#test-os-open-animated")
        .await?
        .click()
        .await?;
    page.element("[role=dialog][aria-label=Animated]").await?;
    let log = page.element(LOG).await?;
    assert_that!(|| async {
        Ok::<_, Report>(
            log.inner_text()
                .await?
                .split(',')
                .map(str::to_owned)
                .collect::<Vec<_>>(),
        )
    })
    .eventually_ok()
    .satisfies(|entries| {
        entries.contains_exactly_in_any_order(["backdrop-enter", "modal-enter"]);
    })
    .await;

    page.click_at(5, 590).await?;
    assert_that!(|| async {
        Ok::<_, Report>(
            log.inner_text()
                .await?
                .split(',')
                .map(str::to_owned)
                .collect::<Vec<_>>(),
        )
    })
    .eventually_ok()
    .satisfies(|entries| {
        entries.contains_exactly_in_any_order([
            "backdrop-enter",
            "modal-enter",
            "backdrop-exit",
            "modal-exit",
        ]);
    })
    .await;
    // Kept by the animation `on_exit` started (300ms).
    page.element(".test-os-animated.test-os-slow-exit[data-exiting]")
        .await?;
    page.wait_for_count(".test-os-animated", 0).await?;
    Ok(())
}

/// Opening a modal from a menu item by keyboard focuses its `auto_focus` field, not the dialog or
/// the menu's trigger (Dialog.test.js "ensure Input autoFocus works when opening Modal from
/// MenuItem via keyboard").
#[browser_test]
pub async fn auto_focus_in_a_modal_opened_from_a_menu(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(PATH, &["menu-modal"]).await?;
    let trigger = page.element("#test-os-menu-trigger").await?;
    trigger.focus().await?;
    page.send_keys(Key::Enter).await?;
    let item = page
        .element(role(AriaRole::Menuitem).text("Add account"))
        .await?;
    page.wait_for_focus(&item).await?;
    page.send_keys(Key::Enter).await?;
    let email = page
        .first_element("[role=dialog][aria-label='Sign up'] input")
        .await?;
    page.wait_for_focus(&email).await?;
    page.focus_stays(&email, Duration::from_millis(100)).await?;
    Ok(())
}

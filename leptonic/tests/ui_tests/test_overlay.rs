// Upstream: react-aria/test/overlays/useOverlay.test.js @ 99e6102368
// Upstream: react-aria/test/overlays/usePreventScroll.test.js @ 99e6102368
//! `use_overlay`: interacting outside closes a dismissable overlay unless
//! `should_close_on_interact_outside` says no, a non-dismissable one only by Escape; with keyboard
//! dismissal disabled, Escape reaches the page; only the top-most overlay closes. Nested modals:
//! only the top one closes, the outer one becomes usable again, and the page stays unscrollable
//! until the last one closed.
use assertr::{matchers::eq, prelude::*};
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions};

const PATH: &str = "/hooks/overlay";

/// The overlay `id`, once it is shown.
async fn shown_overlay(page: &Page<'_>, id: &str) -> Result<WebElement, Report> {
    page.element(format!("#{id}[style*=block]")).await
}

/// Negative check: `overlay` is shown (`display: block`) and stays so.
async fn still_open(page: &Page<'_>, overlay: &WebElement) -> Result<(), Report> {
    let subject = format!("the display of {}", overlay.describe().await?);
    page.settle().await?;
    assert_that!(|| overlay.css_value("display"))
        .with_subject_name(subject)
        .consistently_ok()
        .matches(eq("block"))
        .await;
    Ok(())
}

/// The page's inline `overflow` style, which `use_prevent_scroll` sets while a modal is open.
async fn root_overflow(page: &Page<'_>) -> Result<String, Report> {
    page.eval("return document.documentElement.style.overflow;", vec![])
        .await
}

/// A click near the viewport's top left corner (on the inert page around the modals).
async fn click_page_corner(page: &Page<'_>) -> Result<(), Report> {
    page.driver
        .action_chain()
        .move_to(2, 2)
        .click()
        .perform()
        .await?;
    Ok(())
}

/// "should hide the overlay when clicking outside if isDismissable is true", "... if
/// shouldCloseOnInteractOutside returns true", "should not hide the overlay when clicking outside
/// if shouldCloseOnInteractOutside returns false", "should hide the overlay when pressing the
/// escape key".
pub async fn dismissable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let open = page.element("#test-ov-open-a").await?;
    let closes = page.element("#test-ov-a-closes").await?;
    let outside = page.element("#test-ov-outside").await?;
    open.click().await?;
    let overlay = shown_overlay(page, "test-ov-a").await?;
    // The filter keeps it open for `#test-ov-keep`.
    page.element("#test-ov-keep").await?.click().await?;
    still_open(page, &overlay).await?;
    assert_that!(closes.inner_text().await?).is_equal_to("0");
    outside.click().await?;
    closes.wait_for_inner_text("1").await?;
    overlay
        .wait_until()
        .error("#test-ov-a to hide")
        .not_displayed()
        .await?;

    // Escape closes it.
    open.click().await?;
    page.element("#test-ov-a-inside").await?.click().await?;
    page.send_keys(Key::Escape).await?;
    closes.wait_for_inner_text("2").await?;
    Ok(())
}

/// "should not hide the overlay when clicking outside if isDismissable is false", "should still
/// hide the overlay when pressing the escape key if isDismissable is false".
pub async fn not_dismissable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let closes = page.element("#test-ov-b-closes").await?;
    page.element("#test-ov-open-b").await?.click().await?;
    let overlay = shown_overlay(page, "test-ov-b").await?;
    page.element("#test-ov-outside").await?.click().await?;
    still_open(page, &overlay).await?;
    assert_that!(closes.inner_text().await?).is_equal_to("0");
    page.element("#test-ov-b-inside").await?.click().await?;
    page.send_keys(Key::Escape).await?;
    closes.wait_for_inner_text("1").await?;
    Ok(())
}

/// `isKeyboardDismissDisabled`: Escape doesn't close the overlay and reaches the page.
pub async fn keyboard_dismiss_disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let closes = page.element("#test-ov-c-closes").await?;
    let escapes = page.element("#test-ov-escapes").await?;
    page.element("#test-ov-open-c").await?.click().await?;
    let overlay = shown_overlay(page, "test-ov-c").await?;
    page.element("#test-ov-c-inside").await?.click().await?;
    let escapes_before: u32 = escapes.inner_text().await?.parse()?;
    page.send_keys(Key::Escape).await?;
    escapes
        .wait_for_inner_text(&(escapes_before + 1).to_string())
        .await?;
    still_open(page, &overlay).await?;
    assert_that!(closes.inner_text().await?).is_equal_to("0");
    // Interacting outside still closes it.
    page.element("#test-ov-outside").await?.click().await?;
    closes.wait_for_inner_text("1").await?;
    Ok(())
}

/// "should only hide the top-most overlay".
pub async fn top_most_only(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let a_closes = page.element("#test-ov-a-closes").await?;
    let outside = page.element("#test-ov-outside").await?;
    let a_closes_before: u32 = a_closes.inner_text().await?.parse()?;
    page.element("#test-ov-open-a-d").await?.click().await?;
    shown_overlay(page, "test-ov-d").await?;
    let a = shown_overlay(page, "test-ov-a").await?;
    outside.click().await?;
    page.element("#test-ov-d-closes")
        .await?
        .wait_for_inner_text("1")
        .await?;
    still_open(page, &a).await?;
    assert_that!(a_closes.inner_text().await?.parse::<u32>()?).is_equal_to(a_closes_before);
    outside.click().await?;
    a_closes
        .wait_for_inner_text(&(a_closes_before + 1).to_string())
        .await?;
    Ok(())
}

/// Nested modals: "only hides the top-most overlay" for modals, the outer modal shown and usable
/// again when the inner one closes, and `use_prevent_scroll` counting nested modals ("should work
/// with nested modals" in `usePreventScroll.test.js`).
pub async fn nested_modals(page: &Page<'_>) -> Result<(), Report> {
    const OUTER: &str = "[role=dialog][aria-label=Outer]";
    const INNER: &str = "[role=dialog][aria-label=Inner]";
    const OUTER_INERT: &str = "[inert] .test-ov-outer-backdrop, .test-ov-outer-backdrop[inert]";
    page.goto_path(PATH).await?;
    let open = page.element("#test-ov-modal-open").await?;
    open.click().await?;
    page.element(OUTER).await?;
    assert_that!(root_overflow(page).await?).is_equal_to("hidden");
    let inner_open = page.element("#test-ov-modal-inner-open").await?;
    inner_open.click().await?;
    let inner = page.element(INNER).await?;
    page.wait_for_focus(&inner).await?;
    // The outer modal is hidden behind the inner one.
    page.element(OUTER_INERT).await?;
    assert_that!(root_overflow(page).await?).is_equal_to("hidden");

    // Closed from inside: the outer modal is usable again, focus back on its button.
    page.element("#test-ov-modal-inner-close")
        .await?
        .click()
        .await?;
    page.wait_for_count(INNER, 0).await?;
    page.wait_for_count(OUTER_INERT, 0).await?;
    page.wait_for_focus(&inner_open).await?;
    assert_that!(root_overflow(page).await?)
        .with_detail_message("the outer modal still prevents scrolling")
        .is_equal_to("hidden");

    // Both open again: an interaction outside closes only the inner (top-most) modal.
    inner_open.click().await?;
    page.element(INNER).await?;
    click_page_corner(page).await?;
    page.wait_for_count(INNER, 0).await?;
    page.count_stays(OUTER, 1).await?;
    click_page_corner(page).await?;
    page.wait_for_count(OUTER, 0).await?;
    page.wait_for_focus(&open).await?;
    assert_that!(root_overflow(page).await?).is_empty();
    Ok(())
}

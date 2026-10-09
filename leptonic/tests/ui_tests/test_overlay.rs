// Upstream: react-aria/test/overlays/useOverlay.test.js @ 99e6102368
// Upstream: react-aria/test/overlays/usePreventScroll.test.js @ 99e6102368
//! `use_overlay`: interacting outside closes a dismissable overlay unless
//! `should_close_on_interact_outside` says no, a non-dismissable one only by Escape; with keyboard
//! dismissal disabled, Escape reaches the page; only the top-most overlay closes. Nested modals:
//! only the top one closes, the outer one becomes usable again, and the page stays unscrollable
//! until the last one closed.
use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page};

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
        .for_at_least(std::time::Duration::from_millis(100))
        .matches(eq("block"))
        .await;
    Ok(())
}

/// The page's inline `overflow` style, which `use_prevent_scroll` sets while a modal is open.
async fn root_overflow(page: &Page<'_>) -> Result<String, Report> {
    page.low_level()
        .eval("return document.documentElement.style.overflow;", vec![])
        .await
}

/// A dismissable overlay closes on a click outside, except on elements its outside-interaction
/// filter excludes, and on Escape ("should hide the overlay when clicking outside if isDismissble
/// is true", "should hide the overlay when clicking outside if shouldCloseOnInteractOutside returns
/// true", "should not hide the overlay when clicking outside if shouldCloseOnInteractOutside
/// returns false", "should hide the overlay when pressing the escape key").
#[browser_test]
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
    closes
        .inner_text_stays("0", std::time::Duration::from_millis(100))
        .await?;
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

/// A non-dismissable overlay stays open on a click outside but closes on Escape ("should not hide
/// the overlay when clicking outside if isDismissable is false", "should still hide the overlay
/// when pressing the escape key if isDismissable is false").
#[browser_test]
pub async fn not_dismissable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let closes = page.element("#test-ov-b-closes").await?;
    page.element("#test-ov-open-b").await?.click().await?;
    let overlay = shown_overlay(page, "test-ov-b").await?;
    page.element("#test-ov-outside").await?.click().await?;
    still_open(page, &overlay).await?;
    closes
        .inner_text_stays("0", std::time::Duration::from_millis(100))
        .await?;
    page.element("#test-ov-b-inside").await?.click().await?;
    page.send_keys(Key::Escape).await?;
    closes.wait_for_inner_text("1").await?;
    Ok(())
}

/// With keyboard dismissal disabled, Escape doesn't close the overlay and reaches the page, while a
/// click outside still closes it.
#[browser_test]
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
    closes
        .inner_text_stays("0", std::time::Duration::from_millis(100))
        .await?;
    // Interacting outside still closes it.
    page.element("#test-ov-outside").await?.click().await?;
    closes.wait_for_inner_text("1").await?;
    Ok(())
}

/// With two overlays open, a click outside closes only the top-most one, and a second click the
/// other ("should only hide the top-most overlay").
#[browser_test]
pub async fn top_most_only(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let a_closes = page.element("#test-ov-a-closes").await?;
    let outside = page.element("#test-ov-outside").await?;
    assert_that!(a_closes).inner_text().await.is_equal_to("0");
    page.element("#test-ov-open-a-d").await?.click().await?;
    shown_overlay(page, "test-ov-d").await?;
    let a = shown_overlay(page, "test-ov-a").await?;
    outside.click().await?;
    page.element("#test-ov-d-closes")
        .await?
        .wait_for_inner_text("1")
        .await?;
    still_open(page, &a).await?;
    a_closes
        .inner_text_stays("0", std::time::Duration::from_millis(100))
        .await?;
    outside.click().await?;
    a_closes.wait_for_inner_text("1").await?;
    Ok(())
}

/// Of nested modals, an interaction outside closes only the inner one, the outer one becomes usable
/// again when the inner one closes, and the page stays unscrollable until the last one closed
/// ("should only hide the top-most overlay", "should work with nested modals").
#[browser_test]
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
    // Near the top left corner: on the inert page around the overlays.
    page.click_at(2, 2).await?;
    page.wait_for_count(INNER, 0).await?;
    page.count_stays(OUTER, 1, std::time::Duration::from_millis(100))
        .await?;
    // Near the top left corner: on the inert page around the overlays.
    page.click_at(2, 2).await?;
    page.wait_for_count(OUTER, 0).await?;
    page.wait_for_focus(&open).await?;
    assert_that!(root_overflow(page).await?).is_empty();
    Ok(())
}

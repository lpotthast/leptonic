// Upstream: react-aria/test/overlays/useOverlay.test.js @ 99e6102368
// Upstream: react-aria/test/overlays/usePreventScroll.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{Key, WebDriver},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// `use_overlay`: interacting outside closes a dismissable overlay unless
/// `should_close_on_interact_outside` says no, a non-dismissable one only by Escape; with keyboard
/// dismissal disabled, Escape reaches the page; only the top-most overlay closes. Nested modals:
/// only the top one closes, the outer one becomes usable again, and the page stays unscrollable
/// until the last one closed.
pub struct OverlayTests {}

#[async_trait]
impl BrowserTest<str> for OverlayTests {
    fn name(&self) -> Cow<'_, str> {
        "overlay_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/hooks/overlay").await?;
        dismissable(&page).await?;
        not_dismissable(&page).await?;
        keyboard_dismiss_disabled(&page).await?;
        top_most_only(&page).await?;
        nested_modals(&page).await?;
        page.expect_no_page_errors().await
    }
}

async fn is_open(page: &Page<'_>, id: &str) -> Result<bool, Report> {
    Ok(page.element(id).await?.is_displayed().await?)
}

/// Waits a moment, then checks that `id` is (still) open.
async fn expect_still_open(page: &Page<'_>, id: &str) -> Result<(), Report> {
    stays!(format!("#{id} still open"), true, is_open(page, id).await?);
    Ok(())
}

/// "should hide the overlay when clicking outside if isDismissable is true", "... if
/// shouldCloseOnInteractOutside returns true", "should not hide the overlay when clicking outside
/// if shouldCloseOnInteractOutside returns false", "should hide the overlay when pressing the
/// escape key".
async fn dismissable(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-ov-open-a").await?;
    page.wait_for_selector("#test-ov-a[style*=block]").await?;
    // The filter keeps it open for `#test-ov-keep`.
    page.click_element_with_id("test-ov-keep").await?;
    expect_still_open(page, "test-ov-a").await?;
    assert_that!(page.read_text_of("test-ov-a-closes").await?).is_equal_to("0".to_owned());
    page.click_element_with_id("test-ov-outside").await?;
    page.wait_for_text("test-ov-a-closes", "1").await?;
    assert_that!(is_open(page, "test-ov-a").await?).is_false();

    // Escape closes it.
    page.click_element_with_id("test-ov-open-a").await?;
    page.click_element_with_id("test-ov-a-inside").await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_text("test-ov-a-closes", "2").await
}

/// "should not hide the overlay when clicking outside if isDismissable is false", "should still
/// hide the overlay when pressing the escape key if isDismissable is false".
async fn not_dismissable(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-ov-open-b").await?;
    page.wait_for_selector("#test-ov-b[style*=block]").await?;
    page.click_element_with_id("test-ov-outside").await?;
    expect_still_open(page, "test-ov-b").await?;
    assert_that!(page.read_text_of("test-ov-b-closes").await?).is_equal_to("0".to_owned());
    page.click_element_with_id("test-ov-b-inside").await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_text("test-ov-b-closes", "1").await
}

/// `isKeyboardDismissDisabled`: Escape doesn't close the overlay and reaches the page.
async fn keyboard_dismiss_disabled(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-ov-open-c").await?;
    page.wait_for_selector("#test-ov-c[style*=block]").await?;
    page.click_element_with_id("test-ov-c-inside").await?;
    let escapes: u32 = page.read_u32("test-ov-escapes").await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_text("test-ov-escapes", &(escapes + 1).to_string())
        .await?;
    expect_still_open(page, "test-ov-c").await?;
    assert_that!(page.read_text_of("test-ov-c-closes").await?).is_equal_to("0".to_owned());
    // Interacting outside still closes it.
    page.click_element_with_id("test-ov-outside").await?;
    page.wait_for_text("test-ov-c-closes", "1").await
}

/// "should only hide the top-most overlay".
async fn top_most_only(page: &Page<'_>) -> Result<(), Report> {
    let a_closes = page.read_u32("test-ov-a-closes").await?;
    page.click_element_with_id("test-ov-open-a-d").await?;
    page.wait_for_selector("#test-ov-d[style*=block]").await?;
    page.click_element_with_id("test-ov-outside").await?;
    page.wait_for_text("test-ov-d-closes", "1").await?;
    expect_still_open(page, "test-ov-a").await?;
    assert_that!(page.read_u32("test-ov-a-closes").await?).is_equal_to(a_closes);
    page.click_element_with_id("test-ov-outside").await?;
    page.wait_for_text("test-ov-a-closes", &(a_closes + 1).to_string())
        .await
}

async fn root_overflow(page: &Page<'_>) -> Result<String, Report> {
    Ok(page
        .driver
        .execute("return document.documentElement.style.overflow;", vec![])
        .await?
        .convert()?)
}

/// Nested modals: "only hides the top-most overlay" for modals, the outer modal shown and usable
/// again when the inner one closes, and `use_prevent_scroll` counting nested modals ("should work
/// with nested modals" in `usePreventScroll.test.js`).
async fn nested_modals(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-ov-modal-open").await?;
    page.wait_for_selector("[role=dialog][aria-label=Outer]")
        .await?;
    assert_that!(root_overflow(page).await?).is_equal_to("hidden".to_owned());
    page.click_element_with_id("test-ov-modal-inner-open")
        .await?;
    page.wait_for_selector("[role=dialog][aria-label=Inner]")
        .await?;
    page.wait_for_focus_on(
        &page.css("[role=dialog][aria-label=Inner]").await?,
        "the inner dialog",
    )
    .await?;
    // The outer modal is hidden behind the inner one.
    page.wait_for_selector("[inert] .test-ov-outer-backdrop, .test-ov-outer-backdrop[inert]")
        .await?;
    assert_that!(root_overflow(page).await?).is_equal_to("hidden".to_owned());

    // Closed from inside: the outer modal is usable again, focus back on its button.
    page.click_element_with_id("test-ov-modal-inner-close")
        .await?;
    page.wait_for_no_selector("[role=dialog][aria-label=Inner]")
        .await?;
    page.wait_for_no_selector("[inert] .test-ov-outer-backdrop, .test-ov-outer-backdrop[inert]")
        .await?;
    page.wait_for_focus_on(
        &page.element("test-ov-modal-inner-open").await?,
        "the outer modal's button that opened the inner one",
    )
    .await?;
    assert_that!(root_overflow(page).await?)
        .with_detail_message("the outer modal still prevents scrolling")
        .is_equal_to("hidden".to_owned());

    // Both open again: an interaction outside closes only the inner (top-most) modal.
    page.click_element_with_id("test-ov-modal-inner-open")
        .await?;
    page.wait_for_selector("[role=dialog][aria-label=Inner]")
        .await?;
    click_page_corner(page).await?;
    page.wait_for_no_selector("[role=dialog][aria-label=Inner]")
        .await?;
    stays!(
        "the elements matching [role=dialog][aria-label=Outer]",
        1,
        page.count_matching("[role=dialog][aria-label=Outer]")
            .await?
    );
    click_page_corner(page).await?;
    page.wait_for_no_selector("[role=dialog][aria-label=Outer]")
        .await?;
    page.wait_for_active_id("test-ov-modal-open").await?;
    assert_that!(root_overflow(page).await?).is_equal_to(String::new());
    Ok(())
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

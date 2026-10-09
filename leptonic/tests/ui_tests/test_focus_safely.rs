// Upstream: react-aria/test/interactions/focusSafely.test.js @ 99e6102368
//! `focus_safely` with virtual modality: "should focus on the element if it's connected",
//! "should not focus on the element if it's no longer connected", also within shadow DOM; SVG
//! elements are focused too.
use browser_test::browser_test;
use rootcause::Report;

use crate::pages::{ElementActions, Page};

const PATH: &str = "/hooks/focus-safely";

/// After a virtual click (virtual modality), `focus_safely` focuses the connected element ("should
/// focus on the element if it's connected").
#[browser_test]
pub async fn connected(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-focus-safely-focus")
        .await?
        .virtual_click()
        .await?;
    page.element("#test-focus-safely-modality")
        .await?
        .wait_for_inner_text("Virtual")
        .await?;
    page.wait_for_focus(&page.element("#test-focus-safely-target").await?)
        .await?;
    Ok(())
}

/// An element removed before the deferred focus of `focus_safely` runs is not focused, and focus
/// stays where it was ("should not focus on the element if it's no longer connected").
#[browser_test]
pub async fn no_longer_connected(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let remove = page.element("#test-focus-safely-remove").await?;
    remove.focus().await?;
    remove.virtual_click().await?;
    page.wait_for_count("#test-focus-safely-target", 0).await?;
    page.focus_stays(&remove, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// `focus_safely` focuses an SVG element too.
#[browser_test]
pub async fn svg(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-focus-safely-focus-svg")
        .await?
        .click()
        .await?;
    page.wait_for_focus(&page.element("#test-focus-safely-svg").await?)
        .await?;
    Ok(())
}

/// After a virtual click, `focus_safely` focuses a connected element inside a shadow root
/// ("should focus on the element if it's connected within shadow DOM").
#[browser_test]
pub async fn connected_in_shadow_dom(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-focus-safely-shadow-focus")
        .await?
        .virtual_click()
        .await?;
    page.element("#test-focus-safely-shadow-focused")
        .await?
        .wait_for_inner_text("test-focus-safely-shadow-target")
        .await?;
    Ok(())
}

/// An element inside a shadow root removed before the deferred focus of `focus_safely` runs is not
/// focused ("should not focus on the element if it's no longer connected within shadow DOM").
#[browser_test]
pub async fn no_longer_connected_in_shadow_dom(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let remove = page.element("#test-focus-safely-shadow-remove").await?;
    remove.focus().await?;
    remove.virtual_click().await?;
    page.settle().await?;
    page.focus_stays(&remove, std::time::Duration::from_millis(100))
        .await?;
    page.element("#test-focus-safely-shadow-focused")
        .await?
        .inner_text_stays(
            "test-focus-safely-shadow-remove",
            std::time::Duration::from_millis(100),
        )
        .await?;
    Ok(())
}

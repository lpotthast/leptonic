// Upstream: react-aria/test/interactions/focusSafely.test.js @ 99e6102368
//! `focus_safely` with virtual modality: "should focus on the element if it's connected",
//! "should not focus on the element if it's no longer connected"; SVG elements are focused too.
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions};

const PATH: &str = "/hooks/focus-safely";

/// "should focus on the element if it's connected": a virtual click has no pointer (virtual
/// modality), so focusing is deferred.
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

/// "should not focus on the element if it's no longer connected": removed before the deferred
/// focus, focus stays where it was.
pub async fn no_longer_connected(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let remove = page.element("#test-focus-safely-remove").await?;
    remove.focus().await?;
    remove.virtual_click().await?;
    page.wait_for_count("#test-focus-safely-target", 0).await?;
    page.focus_stays(&remove).await?;
    Ok(())
}

/// SVG elements are focused too.
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

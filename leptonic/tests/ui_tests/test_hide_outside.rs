// No upstream: react-aria tests the effect only through `ariaHideOutside`
// (`test_aria_hide_outside.rs`) and its overlays.
//! `use_hide_outside`: a modal overlay makes the page inert while open, once it is rendered, and
//! restores it on closing; a non-modal one leaves the page alone, and stays usable when opened
//! while a modal hid the page.
use std::time::Duration;

use browser_test::browser_test;
use rootcause::Report;

use crate::pages::{ElementActions, Page};

const PATH: &str = "/hooks/hide-outside";
const CONTENT: &str = "#test-ho-content";

/// Waits until the element `selector` is inert (`true`) or not.
async fn wait_for_inert(page: &Page<'_>, selector: &str, inert: bool) -> Result<(), Report> {
    page.element(selector)
        .await?
        .wait_for_attr("inert", inert.then_some(""))
        .await?;
    Ok(())
}

/// Opening the modal makes the page inert (the modal itself stays usable); closing it restores
/// the page.
#[browser_test]
pub async fn a_modal_makes_the_page_inert_while_open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-ho-open-modal").await?.click().await?;
    wait_for_inert(page, CONTENT, true).await?;
    wait_for_inert(page, "#test-ho-open-modal", true).await?;
    wait_for_inert(page, "#test-ho-modal", false).await?;
    page.element("#test-ho-close-modal").await?.click().await?;
    page.wait_for_count("#test-ho-modal", 0).await?;
    wait_for_inert(page, CONTENT, false).await?;
    wait_for_inert(page, "#test-ho-open-modal", false).await?;
    Ok(())
}

/// A non-modal overlay leaves the page as it is.
#[browser_test]
pub async fn a_non_modal_overlay_leaves_the_page_alone(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-ho-open-nonmodal")
        .await?
        .click()
        .await?;
    page.element("#test-ho-nonmodal").await?;
    page.element(CONTENT)
        .await?
        .attr_stays("inert", None, Duration::from_millis(100))
        .await?;
    page.element("#test-ho-close-nonmodal")
        .await?
        .click()
        .await?;
    page.wait_for_count("#test-ho-nonmodal", 0).await?;
    Ok(())
}

/// A non-modal overlay opened from a modal, outside it (as a portaled popover), stays usable
/// while the page is inert, and closing it leaves the modal's hiding in place.
#[browser_test]
pub async fn a_non_modal_overlay_stays_usable_over_a_modal(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-ho-open-modal").await?.click().await?;
    wait_for_inert(page, CONTENT, true).await?;
    page.element("#test-ho-open-nonmodal-inside")
        .await?
        .click()
        .await?;
    wait_for_inert(page, "#test-ho-nonmodal", false).await?;
    page.element("#test-ho-nonmodal")
        .await?
        .attr_stays("inert", None, Duration::from_millis(100))
        .await?;
    page.element("#test-ho-close-nonmodal")
        .await?
        .click()
        .await?;
    page.wait_for_count("#test-ho-nonmodal", 0).await?;
    wait_for_inert(page, CONTENT, true).await?;
    page.element("#test-ho-close-modal").await?.click().await?;
    wait_for_inert(page, CONTENT, false).await?;
    Ok(())
}

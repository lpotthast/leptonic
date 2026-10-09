// No upstream: react-aria has no tests of `useHasTabbableChild` (a private hook).
//! `use_has_tabbable_child`: whether a container has a tabbable descendant, following removed,
//! re-added, nested and disabled children. Every case starts on a fresh page.
use browser_test::browser_test;
use rootcause::Report;

use crate::pages::{ElementActions, Page};

const PATH: &str = "/hooks/has-tabbable-child";

/// With a button child, the container has a tabbable child.
#[browser_test]
pub async fn with_tabbable_child(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-htc-result")
        .await?
        .wait_for_inner_text("true")
        .await?;
    Ok(())
}

/// Removing the button leaves the container without a tabbable child, and re-adding it restores
/// one.
#[browser_test]
pub async fn child_removed_and_re_added(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let result = page.element("#test-htc-result").await?;
    let toggle = page.element("#test-htc-toggle").await?;
    result.wait_for_inner_text("true").await?;

    toggle.click().await?;
    result.wait_for_inner_text("false").await?;

    toggle.click().await?;
    result.wait_for_inner_text("true").await?;
    Ok(())
}

/// A deeply nested tabbable child (div > div > button) is found.
#[browser_test]
pub async fn deeply_nested_tabbable_child(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-htc-nested-result")
        .await?
        .wait_for_inner_text("true")
        .await?;
    Ok(())
}

/// Disabling the child button leaves the container without a tabbable child, and enabling it
/// restores one.
#[browser_test]
pub async fn child_disabled_attribute_change(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let result = page.element("#test-htc-attr-result").await?;
    let toggle = page.element("#test-htc-attr-toggle").await?;
    result.wait_for_inner_text("true").await?;

    toggle.click().await?;
    result.wait_for_inner_text("false").await?;

    toggle.click().await?;
    result.wait_for_inner_text("true").await?;
    Ok(())
}

/// A container with only non-tabbable elements has no tabbable child, and neither has one whose
/// hook is disabled, despite its button child.
#[browser_test]
pub async fn no_tabbable_children(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-htc-none-result")
        .await?
        .inner_text_stays("false", std::time::Duration::from_millis(100))
        .await?;
    page.element("#test-htc-disabled-result")
        .await?
        .inner_text_stays("false", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

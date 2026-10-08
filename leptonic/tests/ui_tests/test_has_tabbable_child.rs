// No upstream: react-aria has no tests of `useHasTabbableChild` (a private hook).
use std::borrow::Cow;

use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions};

/// `use_has_tabbable_child`: whether a container has a tabbable descendant, following removed,
/// re-added, nested and disabled children. Every case starts on a fresh page.
pub struct HasTabbableChildTests {}

#[async_trait]
impl BrowserTest<str> for HasTabbableChildTests {
    fn name(&self) -> Cow<'_, str> {
        "has_tabbable_child_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        cases!(
            with_tabbable_child(&page),
            child_removed_and_re_added(&page),
            no_tabbable_children(&page),
            deeply_nested_tabbable_child(&page),
            child_disabled_attribute_change(&page),
        );
        Ok(())
    }
}

const PATH: &str = "/hooks/has-tabbable-child";

/// With a button child, the container has a tabbable child.
async fn with_tabbable_child(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-htc-result")
        .await?
        .wait_for_inner_text("true")
        .await?;
    Ok(())
}

/// Removing the button: no tabbable child; re-adding it: a tabbable child again.
async fn child_removed_and_re_added(page: &Page<'_>) -> Result<(), Report> {
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
async fn deeply_nested_tabbable_child(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-htc-nested-result")
        .await?
        .wait_for_inner_text("true")
        .await?;
    Ok(())
}

/// Disabling and enabling the child button (an attribute mutation) is followed.
async fn child_disabled_attribute_change(page: &Page<'_>) -> Result<(), Report> {
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

/// A container with only non-tabbable elements, and a disabled hook (with a button child): no
/// tabbable child.
async fn no_tabbable_children(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-htc-none-result")
        .await?
        .inner_text_stays("false")
        .await?;
    page.element("#test-htc-disabled-result")
        .await?
        .inner_text_stays("false")
        .await?;
    Ok(())
}

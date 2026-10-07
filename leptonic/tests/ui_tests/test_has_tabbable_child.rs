// No upstream: react-aria has no tests of `useHasTabbableChild` (a private hook).
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::WebDriver};
use rootcause::Report;

use crate::pages::{BaseActions, has_tabbable_child::HasTabbableChildPage};

pub struct HasTabbableChildTests {}

#[async_trait]
impl BrowserTest<str> for HasTabbableChildTests {
    fn name(&self) -> Cow<'_, str> {
        "has_tabbable_child_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = HasTabbableChildPage { driver, base_url };

        test_with_tabbable_child(&page).await?;
        test_child_removed(&page).await?;
        test_child_re_added(&page).await?;
        test_no_tabbable_children(&page).await?;
        test_deeply_nested_tabbable_child(&page).await?;
        test_child_disabled_attribute_change(&page).await?;

        Ok(())
    }
}

/// Initial state with button child present: result="true".
async fn test_with_tabbable_child(page: &HasTabbableChildPage<'_>) -> Result<(), Report> {
    tracing::info!("Test: with tabbable child");
    page.goto().await?;

    assert_that!(page.read_result().await?).is_equal_to(true);

    Ok(())
}

/// Click toggle to remove button, then check result="false".
async fn test_child_removed(page: &HasTabbableChildPage<'_>) -> Result<(), Report> {
    tracing::info!("Test: child removed");
    page.goto().await?;

    // Initial: has tabbable child
    assert_that!(page.read_result().await?).is_equal_to(true);

    // Toggle to remove the child button
    page.click_toggle().await?;

    page.wait_for_text("test-htc-result", "false").await?;

    Ok(())
}

/// Click toggle to remove, then toggle again to re-add: result="true".
async fn test_child_re_added(page: &HasTabbableChildPage<'_>) -> Result<(), Report> {
    tracing::info!("Test: child re-added");
    page.goto().await?;

    // Remove
    page.click_toggle().await?;
    page.wait_for_text("test-htc-result", "false").await?;

    // Re-add
    page.click_toggle().await?;
    page.wait_for_text("test-htc-result", "true").await?;

    Ok(())
}

/// Deeply nested tabbable child (div > div > button): TreeWalker subtree traversal finds it.
async fn test_deeply_nested_tabbable_child(page: &HasTabbableChildPage<'_>) -> Result<(), Report> {
    tracing::info!("Test: deeply nested tabbable child");
    page.goto().await?;

    assert_that!(page.read_nested_result().await?).is_equal_to(true);

    Ok(())
}

/// Dynamic attribute mutation: disabling/enabling a child button via MutationObserver.
async fn test_child_disabled_attribute_change(
    page: &HasTabbableChildPage<'_>,
) -> Result<(), Report> {
    tracing::info!("Test: child disabled attribute change");
    page.goto().await?;

    // Initial: button is enabled, has tabbable child
    assert_that!(page.read_attr_result().await?).is_equal_to(true);

    // Toggle to disable the child button
    page.click_attr_toggle().await?;
    page.wait_for_text("test-htc-attr-result", "false").await?;

    // Toggle again to re-enable
    page.click_attr_toggle().await?;
    page.wait_for_text("test-htc-attr-result", "true").await?;

    Ok(())
}

/// Section with no tabbable children: result="false". Disabled section: result="false".
async fn test_no_tabbable_children(page: &HasTabbableChildPage<'_>) -> Result<(), Report> {
    tracing::info!("Test: no tabbable children and disabled hook");
    page.goto().await?;

    // Section with only non-tabbable elements
    assert_that!(page.read_none_result().await?).is_equal_to(false);

    // Disabled hook section (has a button child, but hook is disabled)
    assert_that!(page.read_disabled_result().await?).is_equal_to(false);

    Ok(())
}

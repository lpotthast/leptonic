// Upstream: react-aria/test/visually-hidden/VisuallyHidden.test.tsx @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::WebDriver};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// `VisuallyHidden`: hidden by inline styles; with `is_focusable`, shown while focus is within it
/// (also when `is_focusable` changes at runtime).
pub struct VisuallyHiddenTests {}

#[async_trait]
impl BrowserTest<str> for VisuallyHiddenTests {
    fn name(&self) -> Cow<'_, str> {
        "visually_hidden_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/visually-hidden").await?;

        // "hides element".
        let plain = page.css(".test-vh-plain").await?;
        page.click_element_with_id("test-vh-plain-a").await?;
        let hidden_style = plain.attr("style").await?.unwrap_or_default();
        assert_that!(hidden_style.as_str()).contains("clip-path");
        page.press_tab().await?;
        page.wait_for_active_id("test-vh-plain-b").await?;
        assert_that!(plain.attr("style").await?.unwrap_or_default())
            .is_equal_to(hidden_style.clone());

        // "unhides element if focused and isFocusable".
        let focusable = page.css(".test-vh-focusable").await?;
        page.click_element_with_id("test-vh-focusable-a").await?;
        assert_that!(focusable.attr("style").await?.unwrap_or_default())
            .is_equal_to(hidden_style.clone());
        page.press_tab().await?;
        page.wait_for_active_id("test-vh-focusable-b").await?;
        page.wait_for_attr(&focusable, "style", None).await?;
        page.press_tab().await?;
        page.wait_for_active_id("test-vh-focusable-c").await?;
        page.wait_for_attr(&focusable, "style", Some(&hidden_style))
            .await?;

        // `is_focusable` is reactive.
        page.click_element_with_id("test-vh-toggle").await?;
        page.click_element_with_id("test-vh-plain-a").await?;
        page.press_tab().await?;
        page.wait_for_active_id("test-vh-plain-b").await?;
        page.wait_for_attr(&plain, "style", None).await?;
        Ok(())
    }
}

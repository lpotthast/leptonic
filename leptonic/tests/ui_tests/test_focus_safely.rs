// Upstream: react-aria/test/interactions/focusSafely.test.js @ 99e6102368
use std::borrow::Cow;

use browser_test::{BrowserTest, async_trait, thirtyfour::WebDriver};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// `focus_safely` with virtual modality: "should focus on the element if it's connected",
/// "should not focus on the element if it's no longer connected"; SVG elements are focused too.
pub struct FocusSafelyTests {}

#[async_trait]
impl BrowserTest<str> for FocusSafelyTests {
    fn name(&self) -> Cow<'_, str> {
        "focus_safely_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/hooks/focus-safely").await?;

        // A script click has no pointer: virtual modality, so focusing is deferred.
        js_click(driver, "test-focus-safely-focus").await?;
        page.wait_for_text("test-focus-safely-modality", "Virtual")
            .await?;
        page.wait_for_active_id("test-focus-safely-target").await?;

        // Removed before the deferred focus: focus stays where it was.
        page.element("test-focus-safely-remove")
            .await?
            .focus()
            .await?;
        js_click(driver, "test-focus-safely-remove").await?;
        page.wait_for_no_selector("#test-focus-safely-target")
            .await?;
        stays!(
            "the focused element's id",
            Some("test-focus-safely-remove".to_owned()),
            page.active_element_id().await?
        );

        page.click_element_with_id("test-focus-safely-focus-svg")
            .await?;
        page.wait_for_active_id("test-focus-safely-svg").await?;

        page.expect_no_page_errors().await
    }
}

async fn js_click(driver: &WebDriver, id: &str) -> Result<(), Report> {
    driver
        .execute(&format!("document.getElementById('{id}').click();"), vec![])
        .await?;
    Ok(())
}

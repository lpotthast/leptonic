// Upstream: react-aria/test/interactions/focusSafely.test.js @ 99e6102368
use std::borrow::Cow;

use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions};

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
        cases!(connected(&page), no_longer_connected(&page), svg(&page));
        Ok(())
    }
}

/// "should focus on the element if it's connected": a virtual click has no pointer (virtual
/// modality), so focusing is deferred.
async fn connected(page: &Page<'_>) -> Result<(), Report> {
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
async fn no_longer_connected(page: &Page<'_>) -> Result<(), Report> {
    let remove = page.element("#test-focus-safely-remove").await?;
    remove.focus().await?;
    remove.virtual_click().await?;
    page.wait_for_count("#test-focus-safely-target", 0).await?;
    page.focus_stays(&remove).await?;
    Ok(())
}

/// SVG elements are focused too.
async fn svg(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-focus-safely-focus-svg")
        .await?
        .click()
        .await?;
    page.wait_for_focus(&page.element("#test-focus-safely-svg").await?)
        .await?;
    Ok(())
}

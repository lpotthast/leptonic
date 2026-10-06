// Upstream: react-aria-components/test/Tooltip.test.js @ 99e6102368
use std::{borrow::Cow, time::Duration};

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// `TooltipTrigger` + `Tooltip` atoms on `Button`s: hovering opens the tooltip after the delay,
/// the next one opens right away (warm-up), leaving closes it; focus opens it immediately and
/// Escape closes it. The trigger is described by the tooltip while it is open.
pub struct TooltipTests {}

#[async_trait]
impl BrowserTest<str> for TooltipTests {
    fn name(&self) -> Cow<'_, str> {
        "tooltip_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/tooltip").await?;
        let edit = page.element("test-tooltip-edit").await?;
        let delete = page.element("test-tooltip-delete").await?;
        let away = page.element("test-tooltip-away").await?;

        // Shows on hover, after the delay, describing the trigger, above it. The pointer moves over
        // the page first (as upstream's test): hovering only counts with pointer modality.
        hover(driver, &away).await?;
        assert_that!(edit.attr("aria-describedby").await?).is_none();
        hover(driver, &edit).await?;
        assert_that!(page.count_matching("[role=tooltip]").await?).is_equal_to(0);
        page.wait_for_selector("[role=tooltip]").await?;
        let tooltip = page.css("[role=tooltip]").await?;
        assert_that!(tooltip.text().await?).is_equal_to("Edit the entry".to_owned());
        let tooltip_id = tooltip.attr("id").await?;
        let describedby = edit.attr("aria-describedby").await?;
        assert_that!(describedby).is_equal_to(tooltip_id);
        assert_that!(tooltip.attr("data-placement").await?).is_equal_to(Some("top".to_owned()));

        // Warm: the next tooltip opens right away and replaces the first.
        hover(driver, &delete).await?;
        tokio::time::sleep(Duration::from_millis(100)).await;
        let tooltip = page.css("[role=tooltip]").await?;
        assert_that!(tooltip.text().await?).is_equal_to("Delete the entry".to_owned());
        assert_that!(page.count_matching("[role=tooltip]").await?).is_equal_to(1);

        // Leaving closes it after the close delay.
        hover(driver, &away).await?;
        page.wait_for_no_selector("[role=tooltip]").await?;
        assert_that!(delete.attr("aria-describedby").await?).is_none();

        // Shows on focus right away; Escape closes it.
        page.click_element_with_id("test-tooltip-before").await?;
        page.press_tab().await?;
        page.wait_for_active_id("test-tooltip-edit").await?;
        page.wait_for_selector("[role=tooltip]").await?;
        assert_that!(page.css("[role=tooltip]").await?.text().await?)
            .is_equal_to("Edit the entry".to_owned());
        page.send_keys_to_active(Key::Escape).await?;
        page.wait_for_no_selector("[role=tooltip]").await?;
        page.expect_no_page_errors().await
    }
}

async fn hover(driver: &WebDriver, element: &WebElement) -> Result<(), Report> {
    driver
        .action_chain()
        .move_to_element_center(element)
        .perform()
        .await?;
    Ok(())
}

use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// The slider atoms' value tooltips: `SliderPopover::Always` shows the tooltip
/// (`data-visible`), `SliderPopover::When` shows it while the thumb is hovered.
pub struct SliderTests {}

#[async_trait]
impl BrowserTest<str> for SliderTests {
    fn name(&self) -> Cow<'_, str> {
        "slider_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/slider").await?;

        let always = tooltip(&page, "Always").await?;
        assert_that!(always.attr("data-visible").await?).is_some();
        assert_that!(always.text().await?).is_equal_to("30".to_owned());

        let on_hover = tooltip(&page, "On hover").await?;
        assert_that!(on_hover.attr("data-visible").await?).is_none();
        page.css("[role=group][aria-label='On hover'] .tooltip")
            .await?
            .find(By::XPath(".."))
            .await?
            .scroll_into_view()
            .await?;
        let thumb = on_hover.find(By::XPath("..")).await?;
        page.driver
            .action_chain()
            .move_to_element_center(&thumb)
            .perform()
            .await?;
        page.wait_for_selector("[role=group][aria-label='On hover'] .tooltip[data-visible]")
            .await
    }
}

async fn tooltip(page: &Page<'_>, label: &str) -> Result<WebElement, Report> {
    page.css(&format!("[role=group][aria-label='{label}'] .tooltip"))
        .await
}

use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// The styled `ColorPicker`: its parts show the bound color, a typed channel changes the color
/// (and the other parts), and a color set from outside reaches all of them.
pub struct ColorPickerComponentTests {}

#[async_trait]
impl BrowserTest<str> for ColorPickerComponentTests {
    fn name(&self) -> Cow<'_, str> {
        "color_picker_component_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/components/color-picker").await?;

        let swatch = page.css("#test-ccp [role=img]").await?;
        assert_that!(swatch.attr("aria-label").await?).is_equal_to(Some("vibrant red".to_owned()));
        let red = field(&page, "Red").await?;
        let blue = field(&page, "Blue").await?;
        let hex = field(&page, "Hex").await?;
        assert_that!(red.prop("value").await?).is_equal_to(Some("255".to_owned()));
        assert_that!(hex.prop("value").await?).is_equal_to(Some("#FF0000".to_owned()));

        // Typing a channel changes the color.
        blue.focus().await?;
        page.send_keys_to_active(Key::Control + "a").await?;
        page.send_keys_to_active("255").await?;
        page.send_keys_to_active(Key::Enter).await?;
        page.wait_for_text("test-ccp-value", "FF00FF").await?;
        page.wait_for_attr(&swatch, "aria-label", Some("light vibrant magenta"))
            .await?;
        assert_that!(hex.prop("value").await?).is_equal_to(Some("#FF00FF".to_owned()));
        let hue = field(&page, "Hue").await?;
        assert_that!(hue.prop("value").await?).is_equal_to(Some("300°".to_owned()));

        // Set from outside.
        page.click_element_with_id("test-ccp-set").await?;
        page.wait_for_attr(&swatch, "aria-label", Some("dark vibrant green"))
            .await?;
        assert_that!(red.prop("value").await?).is_equal_to(Some("0".to_owned()));
        assert_that!(hex.prop("value").await?).is_equal_to(Some("#008000".to_owned()));

        page.expect_no_page_errors().await
    }
}

/// The input labelled `label`.
async fn field(page: &Page<'_>, label: &str) -> Result<WebElement, Report> {
    let label = page
        .driver
        .find(By::XPath(format!(
            "//*[@id='test-ccp']//label[normalize-space()='{label}']"
        )))
        .await?;
    let id = label.attr("for").await?.unwrap_or_default();
    Ok(page.driver.find(By::Id(id)).await?)
}

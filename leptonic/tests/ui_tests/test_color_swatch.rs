// Upstream: react-aria-components/test/ColorSwatch.test.js @ 99e6102368
// Upstream: react-aria-components/test/ColorSwatchPicker.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// The `ColorSwatch` atom (named after its color, the label added) and the `ColorSwatchPicker`
/// atoms (a listbox of swatches, picked with the keyboard).
pub struct ColorSwatchTests {}

#[async_trait]
impl BrowserTest<str> for ColorSwatchTests {
    fn name(&self) -> Cow<'_, str> {
        "color_swatch_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/color-swatch").await?;

        // "should render a swatch", "custom aria-label", "custom aria-labelledby", "custom
        // colorName".
        let plain = img(&page, "test-csw-plain").await?;
        assert_that!(attr(&plain, "aria-label").await?).is_equal_to(Some("vibrant red".to_owned()));
        assert_that!(attr(&plain, "aria-roledescription").await?)
            .is_equal_to(Some("color swatch".to_owned()));
        assert_that!(plain.css_value("background-color").await?)
            .is_equal_to("rgba(255, 0, 0, 1)".to_owned());
        let label = img(&page, "test-csw-label").await?;
        assert_that!(attr(&label, "aria-label").await?)
            .is_equal_to(Some("vibrant red, Background".to_owned()));
        let labelledby = img(&page, "test-csw-labelledby").await?;
        let id = attr(&labelledby, "id").await?.unwrap_or_default();
        assert_that!(attr(&labelledby, "aria-labelledby").await?)
            .is_equal_to(Some(format!("{id} test-csw-label-id")));
        let name = img(&page, "test-csw-name").await?;
        assert_that!(attr(&name, "aria-label").await?)
            .is_equal_to(Some("Fire truck red".to_owned()));

        // "renders a listbox", "supports defaultValue".
        let listbox = page.css("#test-csw-default [role=listbox]").await?;
        assert_that!(attr(&listbox, "aria-label").await?)
            .is_equal_to(Some("Color swatches".to_owned()));
        let defaults = options(&page, "test-csw-default").await?;
        assert_that!(defaults.len()).is_equal_to(4);
        assert_that!(attr(&defaults[2], "aria-selected").await?)
            .is_equal_to(Some("true".to_owned()));
        let swatch = defaults[0].find(By::Css("[role=img]")).await?;
        assert_that!(attr(&swatch, "aria-label").await?)
            .is_equal_to(Some("vibrant red".to_owned()));

        // "handles keyboard input".
        let swatches = options(&page, "test-csw-keyboard").await?;
        page.element("test-csw-before").await?.focus().await?;
        page.press_tab().await?;
        page.wait_for_focus_on(&swatches[0], "the first swatch")
            .await?;
        page.send_keys_to_active(Key::Right).await?;
        page.wait_for_focus_on(&swatches[1], "the second swatch")
            .await?;
        page.send_keys_to_active(Key::Enter).await?;
        page.wait_for_text("test-csw-log", "00FF00").await?;
        page.wait_for_attr(&swatches[1], "aria-selected", Some("true"))
            .await?;

        // "isDisabled" items: not selectable, skipped by the arrow keys.
        let items = options(&page, "test-csw-disabled").await?;
        assert_that!(attr(&items[1], "aria-disabled").await?).is_equal_to(Some("true".to_owned()));
        items[0].focus().await?;
        page.wait_for_focus_on(&items[0], "the first swatch")
            .await?;
        page.send_keys_to_active(Key::Right).await?;
        page.wait_for_focus_on(&items[2], "the third swatch (the second is disabled)")
            .await?;

        // A swatch in an item shows the item's color, also inside a `ColorPicker`.
        let in_picker = options(&page, "test-csw-in-picker").await?;
        let swatch = in_picker[1].find(By::Css("[role=img]")).await?;
        assert_that!(attr(&swatch, "aria-label").await?)
            .is_equal_to(Some("very light vibrant green".to_owned()));

        page.expect_no_page_errors().await
    }
}

async fn img(page: &Page<'_>, id: &str) -> Result<WebElement, Report> {
    Ok(page
        .driver
        .find(By::Css(format!("#{id} [role=img]")))
        .await?)
}

async fn options(page: &Page<'_>, id: &str) -> Result<Vec<WebElement>, Report> {
    Ok(page
        .driver
        .find_all(By::Css(format!("#{id} [role=option]")))
        .await?)
}

async fn attr(element: &WebElement, name: &str) -> Result<Option<String>, Report> {
    Ok(element.attr(name).await?)
}

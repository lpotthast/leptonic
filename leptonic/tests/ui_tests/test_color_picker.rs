// Upstream: react-aria-components/test/ColorPicker.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions};

/// The `ColorPicker` atom: a swatch, an HSV area, a hue slider and a hex field without their
/// own values share the picker's color, each in its own color space.
pub struct ColorPickerTests {}

#[async_trait]
impl BrowserTest<str> for ColorPickerTests {
    fn name(&self) -> Cow<'_, str> {
        "color_picker_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/color-picker").await?;

        cases!(shared_color(&page), alpha(&page));

        Ok(())
    }
}

async fn swatch(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-cp-swatch [role=img]").await
}

/// The area's visible input is its x (saturation, 0 to 1 here; react-aria: 0 to 100).
async fn saturation(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-cp-area input[type=range]:not([aria-hidden=true])")
        .await
}

async fn hue(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-cp-hue input[type=range]").await
}

/// "renders"; the parts follow a color typed into the field.
async fn shared_color(page: &Page<'_>) -> Result<(), Report> {
    let swatch = swatch(page).await?;
    assert_that!(swatch.attr("aria-label").await?)
        .get_some()
        .is_equal_to("vibrant red");
    let saturation = saturation(page).await?;
    let hue = hue(page).await?;
    assert_that!(saturation.value().await?)
        .get_some()
        .is_equal_to("1");
    assert_that!(hue.value().await?).get_some().is_equal_to("0");
    let field = page.element("#test-cp-field input").await?;
    assert_that!(field.value().await?)
        .get_some()
        .is_equal_to("#FF0000");

    field.focus().await?;
    // Clear it (End would step to white: the field has a spin button's keys).
    page.send_keys(Key::Control + "a").await?;
    page.send_keys(Key::Backspace).await?;
    page.send_keys("00f").await?;
    page.send_keys(Key::Tab).await?;

    swatch
        .wait_for_attr("aria-label", Some("dark vibrant blue"))
        .await?;
    // The swatch changed: the other parts did in the same update.
    assert_that!(hue.value().await?)
        .get_some()
        .is_equal_to("240");
    assert_that!(saturation.value().await?)
        .get_some()
        .is_equal_to("1");
    assert_that!(field.value().await?)
        .get_some()
        .is_equal_to("#0000FF");
    page.element("#test-cp-log")
        .await?
        .wait_for_inner_text("0000FF")
        .await?;
    Ok(())
}

/// Alpha (react-aria's colors all have one): an alpha slider's value text names no color, the
/// swatch says how transparent the color is, and opaque parts keep the alpha.
async fn alpha(page: &Page<'_>) -> Result<(), Report> {
    let swatch = swatch(page).await?;
    let alpha = page.element("#test-cp-alpha input[type=range]").await?;
    assert_that!(alpha.attr("aria-valuetext").await?)
        .get_some()
        .is_equal_to("100%");
    alpha.focus().await?;
    page.send_keys(Key::PageDown).await?;
    alpha.wait_for_attr("aria-valuetext", Some("90%")).await?;
    swatch
        .wait_for_attr("aria-label", Some("dark vibrant blue, 10% transparent"))
        .await?;
    hue(page).await?.focus().await?;
    page.send_keys(Key::Home).await?;
    swatch
        .wait_for_attr("aria-label", Some("vibrant red, 10% transparent"))
        .await?;
    assert_that!(alpha.attr("aria-valuetext").await?)
        .get_some()
        .is_equal_to("90%");
    Ok(())
}

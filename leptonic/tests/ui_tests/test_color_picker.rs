// Upstream: react-aria-components/test/ColorPicker.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

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

        // "renders".
        let swatch = driver.find(By::Css("#test-cp-swatch [role=img]")).await?;
        assert_that!(swatch.attr("aria-label").await?).is_equal_to(Some("vibrant red".to_owned()));
        // The area's visible input is its x (saturation, 0 to 1 here; react-aria: 0 to 100).
        let saturation = driver
            .find(By::Css(
                "#test-cp-area input[type=range]:not([aria-hidden=true])",
            ))
            .await?;
        let hue = driver
            .find(By::Css("#test-cp-hue input[type=range]"))
            .await?;
        assert_that!(saturation.prop("value").await?).is_equal_to(Some("1".to_owned()));
        assert_that!(hue.prop("value").await?).is_equal_to(Some("0".to_owned()));
        let field = driver.find(By::Css("#test-cp-field input")).await?;
        assert_that!(field.prop("value").await?).is_equal_to(Some("#FF0000".to_owned()));

        field.focus().await?;
        // Clear it (End would step to white: the field has a spin button's keys).
        page.send_keys_to_active(Key::Control + "a").await?;
        page.send_keys_to_active(Key::Backspace).await?;
        page.send_keys_to_active("00f").await?;
        page.send_keys_to_active(Key::Tab).await?;

        page.wait_for_attr(&swatch, "aria-label", Some("dark vibrant blue"))
            .await?;
        // The swatch changed: the other parts did in the same update.
        assert_that!(hue.prop("value").await?).is_equal_to(Some("240".to_owned()));
        assert_that!(saturation.prop("value").await?).is_equal_to(Some("1".to_owned()));
        assert_that!(field.prop("value").await?).is_equal_to(Some("#0000FF".to_owned()));
        page.wait_for_text("test-cp-log", "0000FF").await?;

        // Alpha (react-aria's colors all have one): an alpha slider's value text names no color,
        // the swatch says how transparent the color is, and opaque parts keep the alpha.
        let alpha = driver
            .find(By::Css("#test-cp-alpha input[type=range]"))
            .await?;
        assert_that!(alpha.attr("aria-valuetext").await?).is_equal_to(Some("100%".to_owned()));
        alpha.focus().await?;
        page.send_keys_to_active(Key::PageDown).await?;
        page.wait_for_attr(&alpha, "aria-valuetext", Some("90%"))
            .await?;
        page.wait_for_attr(
            &swatch,
            "aria-label",
            Some("dark vibrant blue, 10% transparent"),
        )
        .await?;
        hue.focus().await?;
        page.send_keys_to_active(Key::Home).await?;
        page.wait_for_attr(&swatch, "aria-label", Some("vibrant red, 10% transparent"))
            .await?;
        assert_that!(alpha.attr("aria-valuetext").await?).is_equal_to(Some("90%".to_owned()));

        page.expect_no_page_errors().await
    }
}

// Upstream: react-aria-components/test/ColorPicker.test.js @ 99e6102368
//! The `ColorPicker` atom: a swatch, an HSV area, a hue slider and a hex field without their
//! own values share the picker's color, each in its own color space.
use assertr::prelude::*;
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page};

const PATH: &str = "/atoms/color-picker";

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

/// Replaces the hex field's text with `hex` and leaves the field (committing it).
async fn enter_hex(page: &Page<'_>, hex: &str) -> Result<(), Report> {
    page.element("#test-cp-field input").await?.focus().await?;
    // Clear it (End would step to white: the field has a spin button's keys).
    page.send_keys(Key::Control + "a").await?;
    page.send_keys(Key::Backspace).await?;
    page.send_keys(hex).await?;
    page.send_keys(Key::Tab).await?;
    Ok(())
}

/// The swatch, area, hue slider and hex field show the picker's color, and all follow a color
/// typed into the field ("renders").
#[browser_test]
pub async fn shared_color(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let swatch = swatch(page).await?;
    assert_that!(swatch)
        .has_attribute("aria-label")
        .await
        .is_equal_to("vibrant red");
    let saturation = saturation(page).await?;
    let hue = hue(page).await?;
    assert_that!(saturation)
        .property("value")
        .await
        .get_some()
        .is_equal_to("1");
    assert_that!(hue)
        .property("value")
        .await
        .get_some()
        .is_equal_to("0");
    let field = page.element("#test-cp-field input").await?;
    assert_that!(field)
        .property("value")
        .await
        .get_some()
        .is_equal_to("#FF0000");

    enter_hex(page, "00f").await?;
    swatch
        .wait_for_attr("aria-label", Some("dark vibrant blue"))
        .await?;
    // Each part follows (they may update in separate effects).
    hue.wait_for_prop("value", "240").await?;
    saturation.wait_for_prop("value", "1").await?;
    field.wait_for_prop("value", "#0000FF").await?;
    page.element("#test-cp-log")
        .await?
        .wait_for_inner_text("0000FF")
        .await?;
    Ok(())
}

/// The alpha slider's value text is a bare percentage, lowering it makes the swatch say how
/// transparent the color is, and changing the hue keeps the alpha.
#[browser_test]
pub async fn alpha(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    // A color whose hue isn't the hue slider's minimum.
    enter_hex(page, "00f").await?;
    let swatch = swatch(page).await?;
    swatch
        .wait_for_attr("aria-label", Some("dark vibrant blue"))
        .await?;
    let alpha = page.element("#test-cp-alpha input[type=range]").await?;
    assert_that!(alpha)
        .has_attribute("aria-valuetext")
        .await
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
    alpha
        .attr_stays(
            "aria-valuetext",
            Some("90%"),
            std::time::Duration::from_millis(100),
        )
        .await?;
    Ok(())
}

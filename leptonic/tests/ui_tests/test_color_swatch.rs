// Upstream: react-aria-components/test/ColorSwatch.test.js @ 99e6102368
// Upstream: react-aria-components/test/ColorSwatchPicker.test.js @ 99e6102368
//! The `ColorSwatch` atom (named after its color, the label added) and the `ColorSwatchPicker`
//! atoms (a listbox of swatches, picked with the keyboard).
use assertr::prelude::*;
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page};

const PATH: &str = "/atoms/color-swatch";

/// The swatch inside `#id`.
async fn img(page: &Page<'_>, id: &str) -> Result<WebElement, Report> {
    page.element(format!("#{id} [role=img]")).await
}

/// The items of the swatch picker inside `#id`.
async fn options(page: &Page<'_>, id: &str) -> Result<Vec<WebElement>, Report> {
    page.elements(format!("#{id} [role=option]")).await
}

/// A swatch is an image named after its color, with an `aria-label` added to that name, an
/// `aria-labelledby` referencing it, or a custom color name instead ("should render a swatch with
/// default class", "should support custom aria-label", "should support custom aria-labelledby",
/// "should support custom colorName").
#[browser_test]
pub async fn swatches(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let plain = img(page, "test-csw-plain").await?;
    assert_that!(plain)
        .has_attribute("aria-label")
        .await
        .is_equal_to("vibrant red");
    assert_that!(plain)
        .has_attribute("aria-roledescription")
        .await
        .is_equal_to("color swatch");
    assert_that!(plain.css_value("background-color").await?).is_equal_to("rgba(255, 0, 0, 1)");
    let label = img(page, "test-csw-label").await?;
    assert_that!(label)
        .has_attribute("aria-label")
        .await
        .is_equal_to("vibrant red, Background");
    let labelledby = img(page, "test-csw-labelledby").await?;
    let id = labelledby.id().await?.unwrap_or_default();
    assert_that!(labelledby)
        .has_attribute("aria-labelledby")
        .await
        .is_equal_to(format!("{id} test-csw-label-id"));
    let name = img(page, "test-csw-name").await?;
    assert_that!(name)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Fire truck red");
    Ok(())
}

/// The swatch picker is a labelled listbox of swatch options with the default value selected
/// ("renders a listbox", "supports defaultValue").
#[browser_test]
pub async fn picker_default_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let listbox = page.element("#test-csw-default [role=listbox]").await?;
    assert_that!(listbox)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Color swatches");
    let defaults = options(page, "test-csw-default").await?;
    assert_that!(defaults).has_length(4);
    assert_that!(defaults[2])
        .has_attribute("aria-selected")
        .await
        .is_equal_to("true");
    let swatch = defaults[0].element("[role=img]").await?;
    assert_that!(swatch)
        .has_attribute("aria-label")
        .await
        .is_equal_to("vibrant red");
    Ok(())
}

/// Tab focuses the first swatch, the arrow keys move between swatches and Enter selects the
/// focused one ("handles keyboard input").
#[browser_test]
pub async fn picker_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let swatches = options(page, "test-csw-keyboard").await?;
    page.element("#test-csw-before").await?.focus().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&swatches[0]).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&swatches[1]).await?;
    page.send_keys(Key::Enter).await?;
    page.element("#test-csw-log")
        .await?
        .wait_for_inner_text("00FF00")
        .await?;
    swatches[1]
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    Ok(())
}

/// A disabled swatch is `aria-disabled` and skipped by the arrow keys.
#[browser_test]
pub async fn picker_disabled_items(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let items = options(page, "test-csw-disabled").await?;
    assert_that!(items[1])
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");
    items[0].focus().await?;
    page.wait_for_focus(&items[0]).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&items[2]).await?;
    Ok(())
}

/// A swatch in an item shows the item's color, also inside a `ColorPicker`.
#[browser_test]
pub async fn swatch_in_item(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let in_picker = options(page, "test-csw-in-picker").await?;
    let swatch = in_picker[1].element("[role=img]").await?;
    assert_that!(swatch)
        .has_attribute("aria-label")
        .await
        .is_equal_to("very light vibrant green");
    Ok(())
}

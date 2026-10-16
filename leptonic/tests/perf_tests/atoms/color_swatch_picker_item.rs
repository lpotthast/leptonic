// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ColorSwatchPickerItem` (`/perf/atoms/color_swatch_picker_item`,
//! the middle of three items of a `ColorSwatchPicker`): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/color_swatch_picker_item.json`): mounting and unmounting it, picking it
//! with the mouse and with the keyboard (focus by an arrow key, then Enter), hover, and disabling
//! it (with a click it ignores).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/color_swatch_picker_item";

/// Registers the performance tests of `ColorSwatchPickerItem`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(PickWithTheMouse))
        .with(fixture_test(PickWithTheKeyboard))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(DisableAndEnable))
}
const OPTIONS: &str = ".leptonic-ColorSwatchPicker [role=option]";
const FIRST: &str = ".leptonic-ColorSwatchPicker [role=option]:nth-child(1)";
const ITEM: &str = ".leptonic-ColorSwatchPicker [role=option]:nth-child(2)";
const RED: f64 = 16_711_680.0;
const GREEN: f64 = 65_280.0;

/// The picked color (`0xRRGGBB`).
async fn value(page: &Page<'_>) -> Result<f64, Report> {
    page.control("value").await
}

/// Mounting the item (with its swatch) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(OPTIONS, 2).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(OPTIONS, 3).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(OPTIONS, 2).await?;
    Ok(())
}

/// A click on the item: its color is picked.
#[browser_test]
pub async fn pick_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let item = page.element(ITEM).await?;
    page.measure("click", item.click()).await?;
    item.wait_for_attr("aria-selected", Some("true")).await?;
    assert_that!(value(page).await?).is_equal_to(GREEN);
    Ok(())
}

/// The right arrow key moving focus from the first item to the item, and Enter picking it.
#[browser_test]
pub async fn pick_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(FIRST).await?).await?;
    let item = page.element(ITEM).await?;
    page.measure("arrow right", page.send_keys(Key::Right))
        .await?;
    page.wait_for_focus(&item).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    item.wait_for_attr("aria-selected", Some("true")).await?;
    assert_that!(value(page).await?).is_equal_to(GREEN);
    Ok(())
}

/// The pointer entering and leaving the item.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let item = page.element(ITEM).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", item.hover()).await?;
    item.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    item.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Disabling the item, a click it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let item = page.element(ITEM).await?;
    page.measure("disable", page.control("disable")).await?;
    item.wait_for_attr("aria-disabled", Some("true")).await?;
    page.measure("ignored click", item.click()).await?;
    assert_that!(value(page).await?).is_equal_to(RED);
    page.measure("enable", page.control("enable")).await?;
    item.wait_for_attr("aria-disabled", None).await?;
    Ok(())
}

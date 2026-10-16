// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ColorSwatchPicker` (`/perf/atoms/color_swatch_picker`, its
//! swatches by `ColorSwatchPickerItems`): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/color_swatch_picker.json`): mounting and unmounting it with 4 and 100
//! colors, picking a color with the mouse and with the keyboard (at both sizes), keyboard focus,
//! and changes of the bound value and of its colors.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/color_swatch_picker";

/// Registers the performance tests of `ColorSwatchPicker`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountMany))
        .with(fixture_test(PickWithTheMouse))
        .with(fixture_test(PickWithTheMouseMany))
        .with(fixture_test(PickWithTheKeyboard))
        .with(fixture_test(PickWithTheKeyboardMany))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(ChangeTheValue))
        .with(fixture_test(ChangeTheColors))
}
const PICKER: &str = ".leptonic-ColorSwatchPicker";
const OPTIONS: &str = ".leptonic-ColorSwatchPicker [role=option]";
/// The second color (`#FE0100`).
const SECOND: f64 = 16_646_400.0;

/// The `n`th swatch (from 1).
fn option(n: usize) -> String {
    format!("{OPTIONS}:nth-child({n})")
}

/// Opens the fixture with 100 colors.
async fn goto_many(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(PICKER, 0).await?;
    page.control("use-many").await?;
    page.control("mount").await?;
    page.wait_for_count(OPTIONS, 100).await
}

/// Mounts and unmounts the picker (unmounted with `count` colors first).
async fn mount_and_unmount_with(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(OPTIONS, count).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(PICKER, 0).await?;
    Ok(())
}

/// A click on the second swatch: it is picked.
async fn pick_with_the_mouse_in(page: &Page<'_>) -> Result<(), Report> {
    let second = page.element(option(2)).await?;
    page.measure("click", second.click()).await?;
    second.wait_for_attr("aria-selected", Some("true")).await?;
    assert_that!(page.control("value").await?).is_equal_to(SECOND);
    assert_that!(page.control("changes").await?).is_equal_to(1.0);
    Ok(())
}

/// Tab to the picked (first) swatch, the right arrow key moving focus to the second, and Enter
/// picking it.
async fn pick_with_the_keyboard_in(page: &Page<'_>) -> Result<(), Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(option(1)).await?).await?;
    let second = page.element(option(2)).await?;
    page.measure("arrow right", page.send_keys(Key::Right))
        .await?;
    page.wait_for_focus(&second).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    second.wait_for_attr("aria-selected", Some("true")).await?;
    assert_that!(page.control("value").await?).is_equal_to(SECOND);
    Ok(())
}

/// Mounting the picker with 4 colors and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(PICKER, 0).await?;
    mount_and_unmount_with(page, 4).await
}

/// Mounting the picker with 100 colors and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_many(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(PICKER, 0).await?;
    page.control("use-many").await?;
    mount_and_unmount_with(page, 100).await
}

/// A click on the second of 4 swatches.
#[browser_test]
pub async fn pick_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    pick_with_the_mouse_in(page).await
}

/// A click on the second of 100 swatches.
#[browser_test]
pub async fn pick_with_the_mouse_many(page: &Page<'_>) -> Result<(), Report> {
    goto_many(page).await?;
    pick_with_the_mouse_in(page).await
}

/// The right arrow key and Enter picking the second of 4 swatches.
#[browser_test]
pub async fn pick_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    pick_with_the_keyboard_in(page).await
}

/// The right arrow key and Enter picking the second of 100 swatches.
#[browser_test]
pub async fn pick_with_the_keyboard_many(page: &Page<'_>) -> Result<(), Report> {
    goto_many(page).await?;
    pick_with_the_keyboard_in(page).await
}

/// Tab moving focus into the picker (its picked swatch) and out again.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let first = page.element(option(1)).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&first).await?;
    let after = page.element("#perf-after").await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&after).await?;
    Ok(())
}

/// Changing the bound value to the second color and back.
#[browser_test]
pub async fn change_the_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("select the second", page.control("select-second"))
        .await?;
    page.wait_for_attr(option(2), "aria-selected", Some("true"))
        .await?;
    page.measure("select the first", page.control("select-first"))
        .await?;
    page.wait_for_attr(option(1), "aria-selected", Some("true"))
        .await?;
    Ok(())
}

/// A color coming to the picker and going again.
#[browser_test]
pub async fn change_the_colors(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(OPTIONS, 4).await?;
    page.measure("add", page.control("add-gray")).await?;
    page.wait_for_count(OPTIONS, 5).await?;
    page.measure("remove", page.control("remove-gray")).await?;
    page.wait_for_count(OPTIONS, 4).await?;
    Ok(())
}

// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ColorSwatchPickerItems` (`/perf/atoms/color_swatch_picker_items`,
//! the items of a `ColorSwatchPicker`): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/color_swatch_picker_items.json`): mounting and unmounting the items of 4
//! and 100 colors, and a color coming and going (at both sizes).
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/atoms/color_swatch_picker_items";

/// Registers the performance tests of `ColorSwatchPickerItems`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountMany))
        .with(fixture_test(ChangeTheColors))
        .with(fixture_test(ChangeTheColorsMany))
}
const OPTIONS: &str = ".leptonic-ColorSwatchPicker [role=option]";

/// Opens the fixture with 100 colors.
async fn goto_many(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("use-many").await?;
    page.wait_for_count(OPTIONS, 100).await
}

/// Mounts and unmounts the items (`count` of them), unmounted first.
async fn mount_and_unmount_with(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(OPTIONS, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(OPTIONS, count).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(OPTIONS, 0).await?;
    Ok(())
}

/// Adds a color and removes it again, `count` colors at first.
async fn change_the_colors_with(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.measure("add", page.control("add-gray")).await?;
    page.wait_for_count(OPTIONS, count + 1).await?;
    page.measure("remove", page.control("remove-gray")).await?;
    page.wait_for_count(OPTIONS, count).await?;
    Ok(())
}

/// Mounting the items of 4 colors and unmounting them.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(OPTIONS, 4).await?;
    mount_and_unmount_with(page, 4).await
}

/// Mounting the items of 100 colors and unmounting them.
#[browser_test]
pub async fn mount_and_unmount_many(page: &Page<'_>) -> Result<(), Report> {
    goto_many(page).await?;
    mount_and_unmount_with(page, 100).await
}

/// A color coming to 4 and going again.
#[browser_test]
pub async fn change_the_colors(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(OPTIONS, 4).await?;
    change_the_colors_with(page, 4).await
}

/// A color coming to 100 and going again.
#[browser_test]
pub async fn change_the_colors_many(page: &Page<'_>) -> Result<(), Report> {
    goto_many(page).await?;
    change_the_colors_with(page, 100).await
}

// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_color_picker_state` (`/perf/hooks/use_color_picker_state`,
//! the shared color of a color picker alone): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_color_picker_state.json`): creating and disposing of it (owning its
//! color and bound to app state), setting the color (`set_color`), and the bound app state
//! changing.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_color_picker_state";

/// Registers the performance tests of `use_color_picker_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountBound))
        .with(fixture_test(SetTheColor))
        .with(fixture_test(SetTheColorBound))
        .with(fixture_test(ChangeTheBoundColor))
}
const COLOR: &str = "#perf-color";

/// Waits until the color is `hex` (`RRGGBB`).
async fn wait_for_color(page: &Page<'_>, hex: &str) -> Result<(), Report> {
    page.element(COLOR).await?.wait_for_inner_text(hex).await
}

/// Opens the fixture with the state bound to the page's color.
async fn goto_bound(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(COLOR, 0).await?;
    page.control("use-binding").await?;
    page.control("mount").await?;
    wait_for_color(page, "00FFFF").await
}

/// Measures unmounting and mounting the state (mounted at first).
async fn unmount_and_mount(page: &Page<'_>) -> Result<(), Report> {
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(COLOR, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    wait_for_color(page, "00FFFF").await
}

/// Measures setting the color to red and back.
async fn set_the_color_twice(page: &Page<'_>) -> Result<(), Report> {
    page.measure("set red", page.control("set-red")).await?;
    wait_for_color(page, "FF0000").await?;
    page.measure("set cyan", page.control("set-cyan")).await?;
    wait_for_color(page, "00FFFF").await?;
    assert_that!(page.control("changes").await?).is_equal_to(2.0);
    Ok(())
}

/// Disposing of the state (owning its color) and creating it again.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    wait_for_color(page, "00FFFF").await?;
    unmount_and_mount(page).await
}

/// Disposing of the state (bound to app state) and creating it again.
#[browser_test]
pub async fn mount_and_unmount_bound(page: &Page<'_>) -> Result<(), Report> {
    goto_bound(page).await?;
    unmount_and_mount(page).await
}

/// Setting the owned color (`set_color`) and back.
#[browser_test]
pub async fn set_the_color(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    set_the_color_twice(page).await
}

/// Setting the bound color (`set_color`) and back.
#[browser_test]
pub async fn set_the_color_bound(page: &Page<'_>) -> Result<(), Report> {
    goto_bound(page).await?;
    set_the_color_twice(page).await
}

/// The app changing the bound color and back.
#[browser_test]
pub async fn change_the_bound_color(page: &Page<'_>) -> Result<(), Report> {
    goto_bound(page).await?;
    page.measure("red", page.control("bind-red")).await?;
    wait_for_color(page, "FF0000").await?;
    page.measure("cyan", page.control("bind-cyan")).await?;
    wait_for_color(page, "00FFFF").await?;
    Ok(())
}

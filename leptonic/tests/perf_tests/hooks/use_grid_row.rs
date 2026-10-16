// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_grid_row` (`/perf/hooks/use_grid_row`, rows of a hook grid):
//! what each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_grid_row.json`): a row mounting and unmounting in its grid,
//! selecting and deselecting it with clicks and with Space, its pressed state, its action (click
//! and Enter), and a click on a disabled row, in grids of 10 rows and of 1,000.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_grid_row";

/// Registers the performance tests of `use_grid_row`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(ClickSmall))
        .with(fixture_test(ClickLarge))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(SpaceSmall))
        .with(fixture_test(SpaceLarge))
        .with(fixture_test(RowAction))
        .with(fixture_test(DisabledRowIgnoresClick))
}

const GRID: &str = "[role=grid]";
const ROWS: &str = "[role=grid] > [role=row]";

/// Row `i` (from 0).
fn row(i: usize) -> String {
    format!("[role=grid] > [role=row]:nth-child({})", i + 1)
}

/// Opens the fixture with its 10 rows.
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ROWS, 10).await?;
    Ok(())
}

/// Opens the fixture and mounts the grid with 1,000 rows (unmeasured).
async fn open_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(GRID, 0).await?;
    page.control("large").await?;
    page.control("mount").await?;
    page.wait_for_count(ROWS, 1_000).await?;
    Ok(())
}

/// Creates the live announcer's regions: its first announcement (of a selection change while
/// the grid has focus) waits 100 ms for them, which would fall into a later measurement.
async fn prepare_announcer(page: &Page<'_>) -> Result<(), Report> {
    page.control("prepare-announcer").await?;
    page.wait_for_count("[data-live-announcer] [aria-live=polite] > div", 1)
        .await?;
    Ok(())
}

/// How many rows are selected.
async fn selected(page: &Page<'_>) -> Result<f64, Report> {
    page.control("selected").await
}

/// A row added at the end of the grid's `rows` and removed again.
async fn mount_and_unmount_row(page: &Page<'_>, rows: usize) -> Result<(), Report> {
    page.measure("mount", page.control("add-row")).await?;
    page.wait_for_count(ROWS, rows + 1).await?;
    page.measure("unmount", page.control("remove-row")).await?;
    page.wait_for_count(ROWS, rows).await?;
    Ok(())
}

/// A row mounting in a grid of 10 rows and unmounting.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    mount_and_unmount_row(page, 10).await
}

/// A row mounting in a grid of 1,000 rows and unmounting.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    mount_and_unmount_row(page, 1_000).await
}

/// A click selecting the second row, and one deselecting it.
async fn click(page: &Page<'_>) -> Result<(), Report> {
    prepare_announcer(page).await?;
    let second = page.element(row(1)).await?;
    page.measure("select", second.click()).await?;
    second.wait_for_attr("aria-selected", Some("true")).await?;
    page.measure("deselect", second.click()).await?;
    second.wait_for_attr("aria-selected", Some("false")).await?;
    assert_that!(selected(page).await?).is_equal_to(0.0);
    Ok(())
}

/// Clicks selecting and deselecting a row of 10.
#[browser_test]
pub async fn click_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    click(page).await
}

/// Clicks selecting and deselecting a row of 1,000.
#[browser_test]
pub async fn click_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    click(page).await
}

/// Holding the pointer down on a row (the pressed state; it selects the row) and releasing it.
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    prepare_announcer(page).await?;
    let second = page.element(row(1)).await?;
    let held = page.measure("hold", second.press_and_hold()).await?;
    second.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("release", held.release()).await?;
    second.wait_for_attr("data-pressed", None).await?;
    second.wait_for_attr("aria-selected", Some("true")).await?;
    Ok(())
}

/// Space selecting the focused first row and deselecting it.
async fn space(page: &Page<'_>) -> Result<(), Report> {
    prepare_announcer(page).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let first = page.element(row(0)).await?;
    page.wait_for_focus(&first).await?;
    page.measure("select", page.send_keys(" ")).await?;
    first.wait_for_attr("aria-selected", Some("true")).await?;
    page.measure("deselect", page.send_keys(" ")).await?;
    first.wait_for_attr("aria-selected", Some("false")).await?;
    Ok(())
}

/// Space selecting and deselecting a row of 10.
#[browser_test]
pub async fn space_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    space(page).await
}

/// Space selecting and deselecting a row of 1,000.
#[browser_test]
pub async fn space_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    space(page).await
}

/// The row action (a grid with `on_row_action`): Enter on the focused first row, and a click on
/// the second (nothing is selected, so a click performs the action).
#[browser_test]
pub async fn row_action(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(GRID, 0).await?;
    page.control("mount-with-actions").await?;
    page.wait_for_count(ROWS, 10).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(row(0)).await?).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    assert_that!(page.control("actions").await?).is_equal_to(1.0);
    let second = page.element(row(1)).await?;
    page.measure("click", second.click()).await?;
    assert_that!(page.control("actions").await?).is_equal_to(2.0);
    assert_that!(selected(page).await?).is_equal_to(0.0);
    Ok(())
}

/// A click on a disabled row, which it ignores.
#[browser_test]
pub async fn disabled_row_ignores_click(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("disable-second").await?;
    let second = page.element(row(1)).await?;
    second.wait_for_attr("aria-disabled", Some("true")).await?;
    page.measure("ignored click", second.click()).await?;
    assert_that!(selected(page).await?).is_equal_to(0.0);
    Ok(())
}

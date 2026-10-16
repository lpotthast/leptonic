// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `GridCell` (`/perf/atoms/grid_cell`): what each thing it offers
//! costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/grid_cell.json`): a row of cells mounting and unmounting, clicks
//! (focusing the cell, selecting its row), its pressed state (with a cell action), keyboard
//! focus, the arrow keys between cells and between the children of a cell whose children take
//! focus, and its action (Enter and click), in grids of 10 rows and of 1,000.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/grid_cell";

/// Registers the performance tests of `GridCell`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(ClickSmall))
        .with(fixture_test(ClickLarge))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(ArrowKeysSmall))
        .with(fixture_test(ArrowKeysLarge))
        .with(fixture_test(CellAction))
}

const GRID: &str = "[role=grid]";
const ROWS: &str = "[role=grid] > [role=row]";

/// Row `i` (from 0).
fn row(i: usize) -> String {
    format!("[role=grid] > [role=row]:nth-child({})", i + 1)
}

/// Cell `column` of row `i` (both from 0).
fn cell(i: usize, column: usize) -> String {
    format!("{} > [role=gridcell]:nth-child({})", row(i), column + 1)
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

/// Focuses the grid with the keyboard (Tab from the plain button before it): its first cell.
async fn focus_with_tab(page: &Page<'_>) -> Result<(), Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(cell(0, 0)).await?)
        .await?;
    Ok(())
}

/// A row of cells added at the end of the grid's `rows` and removed again.
async fn mount_and_unmount_cells(page: &Page<'_>, rows: usize) -> Result<(), Report> {
    page.measure("mount", page.control("add-row")).await?;
    page.wait_for_count(ROWS, rows + 1).await?;
    page.measure("unmount", page.control("remove-row")).await?;
    page.wait_for_count(ROWS, rows).await?;
    Ok(())
}

/// A row of cells mounting in a grid of 10 rows and unmounting.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    mount_and_unmount_cells(page, 10).await
}

/// A row of cells mounting in a grid of 1,000 rows and unmounting.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    mount_and_unmount_cells(page, 1_000).await
}

/// A click on the second cell of the first row: it takes focus, and its row is selected.
async fn click(page: &Page<'_>) -> Result<(), Report> {
    prepare_announcer(page).await?;
    let target = page.element(cell(0, 1)).await?;
    page.measure("click", target.click()).await?;
    target.wait_for_attr("data-focused", Some("true")).await?;
    page.element(row(0))
        .await?
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    Ok(())
}

/// A click on a cell of a grid of 10 rows.
#[browser_test]
pub async fn click_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    click(page).await
}

/// A click on a cell of a grid of 1,000 rows.
#[browser_test]
pub async fn click_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    click(page).await
}

/// Holding the pointer down on a cell (the pressed state) and releasing it, in a grid with a
/// cell action (without one, a cell can't be pressed: its row takes the press): the release
/// performs the action.
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(GRID, 0).await?;
    page.control("mount-with-actions").await?;
    page.wait_for_count(ROWS, 10).await?;
    let target = page.element(cell(0, 1)).await?;
    let held = page.measure("hold", target.press_and_hold()).await?;
    target.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("release", held.release()).await?;
    target.wait_for_attr("data-pressed", None).await?;
    assert_that!(page.control("actions").await?).is_equal_to(1.0);
    Ok(())
}

/// Tab moving focus to the first cell and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element("#perf-before").await?.click().await?;
    let first = page.element(cell(0, 0)).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    first.wait_for_attr("data-focused", Some("true")).await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    first.wait_for_attr("data-focused", None).await?;
    Ok(())
}

/// The arrow keys from the first cell: ArrowRight to the second cell, to the first child of the
/// third ("Edit 0", the cell's children take focus), to its second child ("Delete 0"), ArrowLeft
/// back to the first, and ArrowDown to the next row.
async fn arrow_keys(page: &Page<'_>) -> Result<(), Report> {
    focus_with_tab(page).await?;
    page.measure("arrow right", page.send_keys(Key::Right))
        .await?;
    page.wait_for_focus(&page.element(cell(0, 1)).await?)
        .await?;
    page.measure("arrow right", page.send_keys(Key::Right))
        .await?;
    let edit = page
        .element(format!("{} > button:nth-child(1)", cell(0, 2)))
        .await?;
    page.wait_for_focus(&edit).await?;
    page.measure("arrow right", page.send_keys(Key::Right))
        .await?;
    page.wait_for_focus(
        &page
            .element(format!("{} > button:nth-child(2)", cell(0, 2)))
            .await?,
    )
    .await?;
    page.measure("arrow left", page.send_keys(Key::Left))
        .await?;
    page.wait_for_focus(&edit).await?;
    page.measure("arrow down", page.send_keys(Key::Down))
        .await?;
    page.wait_for_focus(
        &page
            .element(format!("{} > button:nth-child(1)", cell(1, 2)))
            .await?,
    )
    .await?;
    Ok(())
}

/// The arrow keys in a grid of 10 rows.
#[browser_test]
pub async fn arrow_keys_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    arrow_keys(page).await
}

/// The arrow keys in a grid of 1,000 rows.
#[browser_test]
pub async fn arrow_keys_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    arrow_keys(page).await
}

/// The cell action (a grid with `on_cell_action`): Enter on the focused first cell, and a click
/// on the second (nothing is selected, so a click performs the action).
#[browser_test]
pub async fn cell_action(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(GRID, 0).await?;
    page.control("mount-with-actions").await?;
    page.wait_for_count(ROWS, 10).await?;
    focus_with_tab(page).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    assert_that!(page.control("actions").await?).is_equal_to(1.0);
    page.measure("click", page.element(cell(1, 1)).await?.click())
        .await?;
    assert_that!(page.control("actions").await?).is_equal_to(2.0);
    Ok(())
}

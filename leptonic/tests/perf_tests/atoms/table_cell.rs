// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `TableCell` (`/perf/atoms/table_cell`): what each thing it offers
//! costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/table_cell.json`): a row of cells mounting and unmounting, a click on a
//! cell (selecting its row), its hovered and pressed states (pressed with a cell action), the
//! arrow keys between cells, its action (Enter and click), and the selection checkbox cell, in
//! tables of 10 rows and of 1,000.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/table_cell";

/// Registers the performance tests of `TableCell`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(ClickSmall))
        .with(fixture_test(ClickLarge))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(ArrowKeysSmall))
        .with(fixture_test(ArrowKeysLarge))
        .with(fixture_test(CellAction))
        .with(fixture_test(SelectionCheckboxSmall))
        .with(fixture_test(SelectionCheckboxLarge))
}

const TABLE: &str = "[role=grid]";
const ROWS: &str = "[role=grid] > tbody > tr";

/// Row `i` (from 0).
fn row(i: usize) -> String {
    format!("[role=grid] > tbody > tr:nth-child({})", i + 1)
}

/// Cell `column` of row `i` (both from 0; column 0 holds the selection checkbox).
fn cell(i: usize, column: usize) -> String {
    format!("{} > td:nth-child({})", row(i), column + 1)
}

/// Opens the fixture with its 10 rows.
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ROWS, 10).await?;
    Ok(())
}

/// Opens the fixture and mounts the table with 1,000 rows (unmeasured).
async fn open_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(TABLE, 0).await?;
    page.control("large").await?;
    page.control("mount").await?;
    page.wait_for_count(ROWS, 1_000).await?;
    Ok(())
}

/// Opens the fixture with the table with a row and a cell action (10 rows).
async fn open_with_actions(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(TABLE, 0).await?;
    page.control("mount-with-actions").await?;
    page.wait_for_count(ROWS, 10).await?;
    Ok(())
}

/// Creates the live announcer's regions: its first announcement (of a selection change while
/// the table has focus) waits 100 ms for them, which would fall into a later measurement.
async fn prepare_announcer(page: &Page<'_>) -> Result<(), Report> {
    page.control("prepare-announcer").await?;
    page.wait_for_count("[data-live-announcer] [aria-live=polite] > div", 1)
        .await?;
    Ok(())
}

/// Focuses the table with the keyboard (Tab from the plain button before it): its first row.
async fn focus_with_tab(page: &Page<'_>) -> Result<(), Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(row(0)).await?).await?;
    Ok(())
}

/// A row of cells added at the end of the table's `rows` and removed again.
async fn mount_and_unmount_cells(page: &Page<'_>, rows: usize) -> Result<(), Report> {
    page.measure("mount", page.control("add-row")).await?;
    page.wait_for_count(ROWS, rows + 1).await?;
    page.measure("unmount", page.control("remove-row")).await?;
    page.wait_for_count(ROWS, rows).await?;
    Ok(())
}

/// A row of cells mounting in a table of 10 rows and unmounting.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    mount_and_unmount_cells(page, 10).await
}

/// A row of cells mounting in a table of 1,000 rows and unmounting.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    mount_and_unmount_cells(page, 1_000).await
}

/// A click on the cell "Value 0": its row is selected.
async fn click(page: &Page<'_>) -> Result<(), Report> {
    prepare_announcer(page).await?;
    page.measure("click", page.element(cell(0, 2)).await?.click())
        .await?;
    page.element(row(0))
        .await?
        .wait_for_attr("data-selected", Some("true"))
        .await?;
    assert_that!(page.control("selected").await?).is_equal_to(1.0);
    Ok(())
}

/// A click on a cell of a table of 10 rows.
#[browser_test]
pub async fn click_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    click(page).await
}

/// A click on a cell of a table of 1,000 rows.
#[browser_test]
pub async fn click_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    click(page).await
}

/// The pointer entering and leaving a cell.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let target = page.element(cell(0, 2)).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", target.hover()).await?;
    target.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    target.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Holding the pointer down on a cell (the pressed state) and releasing it, in a table with a
/// cell action (without one, a cell can't be pressed: its row takes the press): the release
/// performs the action.
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    open_with_actions(page).await?;
    let target = page.element(cell(0, 2)).await?;
    let held = page.measure("hold", target.press_and_hold()).await?;
    target.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("release", held.release()).await?;
    target.wait_for_attr("data-pressed", None).await?;
    assert_that!(page.control("cell-actions").await?).is_equal_to(1.0);
    Ok(())
}

/// The arrow keys from the first row: ArrowRight to its selection checkbox cell (the checkbox,
/// the cell's child, takes the focus), to "Row 0", to "Value 0", ArrowLeft back to "Row 0",
/// ArrowDown to "Row 1".
async fn arrow_keys(page: &Page<'_>) -> Result<(), Report> {
    focus_with_tab(page).await?;
    page.measure("arrow right", page.send_keys(Key::Right))
        .await?;
    page.wait_for_focus(&page.element(format!("{} input", cell(0, 0))).await?)
        .await?;
    for column in 1..3 {
        page.measure("arrow right", page.send_keys(Key::Right))
            .await?;
        page.wait_for_focus(&page.element(cell(0, column)).await?)
            .await?;
    }
    page.measure("arrow left", page.send_keys(Key::Left))
        .await?;
    page.wait_for_focus(&page.element(cell(0, 1)).await?)
        .await?;
    page.measure("arrow down", page.send_keys(Key::Down))
        .await?;
    page.wait_for_focus(&page.element(cell(1, 1)).await?)
        .await?;
    Ok(())
}

/// The arrow keys in a table of 10 rows.
#[browser_test]
pub async fn arrow_keys_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    arrow_keys(page).await
}

/// The arrow keys in a table of 1,000 rows.
#[browser_test]
pub async fn arrow_keys_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    arrow_keys(page).await
}

/// The cell action (a table with `on_cell_action`): Enter on the focused cell "Row 0", and a
/// click on "Value 1" (nothing is selected, so a click performs the action).
#[browser_test]
pub async fn cell_action(page: &Page<'_>) -> Result<(), Report> {
    open_with_actions(page).await?;
    focus_with_tab(page).await?;
    page.send_keys(Key::Right).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&page.element(cell(0, 1)).await?)
        .await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    assert_that!(page.control("cell-actions").await?).is_equal_to(1.0);
    page.measure("click", page.element(cell(1, 2)).await?.click())
        .await?;
    assert_that!(page.control("cell-actions").await?).is_equal_to(2.0);
    Ok(())
}

/// Clicks on the first row's selection checkbox (in its first cell): the row selected, then
/// deselected.
async fn selection_checkbox(page: &Page<'_>) -> Result<(), Report> {
    prepare_announcer(page).await?;
    let checkbox = page.element(format!("{} input", cell(0, 0))).await?;
    page.measure("select", checkbox.click()).await?;
    checkbox.wait_for_prop("checked", "true").await?;
    assert_that!(page.control("selected").await?).is_equal_to(1.0);
    page.measure("deselect", checkbox.click()).await?;
    checkbox.wait_for_prop("checked", "false").await?;
    assert_that!(page.control("selected").await?).is_equal_to(0.0);
    Ok(())
}

/// A selection checkbox of 10 rows.
#[browser_test]
pub async fn selection_checkbox_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    selection_checkbox(page).await
}

/// A selection checkbox of 1,000 rows.
#[browser_test]
pub async fn selection_checkbox_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    selection_checkbox(page).await
}

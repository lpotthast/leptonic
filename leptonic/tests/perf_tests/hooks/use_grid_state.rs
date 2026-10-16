// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_grid_state` (`/perf/hooks/use_grid_state`, the state alone):
//! what each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_grid_state.json`): creating and disposing it, its selection methods
//! (toggle, replace, extend, select all, clear), focusing rows, rows added and removed (also the
//! focused row: focus moves to the row that took its place) and disabling keyboard navigation,
//! with 10 rows and with 1,000.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/hooks/use_grid_state";

/// Registers the performance tests of `use_grid_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(SelectionSmall))
        .with(fixture_test(SelectionLarge))
        .with(fixture_test(Focus))
        .with(fixture_test(RowsChangeSmall))
        .with(fixture_test(RowsChangeLarge))
        .with(fixture_test(KeyboardNavigation))
}

const STATE: &str = "#perf-selected";

/// Opens the fixture with its 10 rows.
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(STATE, 1).await?;
    Ok(())
}

/// Opens the fixture and creates the state with 1,000 rows (unmeasured).
async fn open_large(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(STATE, 0).await?;
    page.control("large").await?;
    page.control("mount").await?;
    page.wait_for_count(STATE, 1).await?;
    Ok(())
}

/// How many rows are selected (-1: all).
async fn selected(page: &Page<'_>) -> Result<f64, Report> {
    page.control("selected").await
}

/// The focused row (-1: none).
async fn focused(page: &Page<'_>) -> Result<f64, Report> {
    page.control("focused").await
}

/// Creating the state and disposing it.
async fn mount_and_unmount_state(page: &Page<'_>) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(STATE, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(STATE, 1).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(STATE, 0).await?;
    Ok(())
}

/// The state of 10 rows created and disposed.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    mount_and_unmount_state(page).await
}

/// The state of 1,000 rows created and disposed.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    mount_and_unmount_state(page).await
}

/// The selection methods: toggling the first row, replacing the selection with the last,
/// extending it to the first (every row), clearing, selecting all and clearing. (Select all
/// right after the extension would do nothing: every row is selected.)
async fn selection(page: &Page<'_>, rows: f64) -> Result<(), Report> {
    page.measure("toggle", page.control("toggle-first")).await?;
    assert_that!(selected(page).await?).is_equal_to(1.0);
    page.measure("replace", page.control("replace-with-last"))
        .await?;
    assert_that!(selected(page).await?).is_equal_to(1.0);
    page.measure("extend", page.control("extend-to-first"))
        .await?;
    assert_that!(selected(page).await?).is_equal_to(rows);
    page.measure("clear", page.control("clear-selection"))
        .await?;
    assert_that!(selected(page).await?).is_equal_to(0.0);
    page.measure("select all", page.control("select-all"))
        .await?;
    assert_that!(selected(page).await?).is_equal_to(-1.0);
    page.measure("clear", page.control("clear-selection"))
        .await?;
    assert_that!(selected(page).await?).is_equal_to(0.0);
    Ok(())
}

/// The selection methods on 10 rows.
#[browser_test]
pub async fn selection_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    selection(page, 10.0).await
}

/// The selection methods on 1,000 rows.
#[browser_test]
pub async fn selection_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    selection(page, 1_000.0).await
}

/// Focusing the first row, then the last.
#[browser_test]
pub async fn focus(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.measure("focus first", page.control("focus-first"))
        .await?;
    assert_that!(focused(page).await?).is_equal_to(0.0);
    page.measure("focus last", page.control("focus-last"))
        .await?;
    assert_that!(focused(page).await?).is_equal_to(9.0);
    Ok(())
}

/// A row added and removed, then the focused last row removed: focus moves to the row before it.
async fn rows_change(page: &Page<'_>, rows: f64) -> Result<(), Report> {
    page.measure("add", page.control("add-row")).await?;
    page.measure("remove", page.control("remove-last")).await?;
    page.control("focus-last").await?;
    assert_that!(focused(page).await?).is_equal_to(rows - 1.0);
    page.measure("remove focused", page.control("remove-last"))
        .await?;
    assert_that!(focused(page).await?).is_equal_to(rows - 2.0);
    Ok(())
}

/// Rows changing in a state of 10.
#[browser_test]
pub async fn rows_change_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    rows_change(page, 10.0).await
}

/// Rows changing in a state of 1,000.
#[browser_test]
pub async fn rows_change_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    rows_change(page, 1_000.0).await
}

/// Disabling keyboard navigation and enabling it again.
#[browser_test]
pub async fn keyboard_navigation(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.measure("disable", page.control("disable-navigation"))
        .await?;
    assert_that!(page.control("navigation-disabled").await?).is_equal_to(1.0);
    page.measure("enable", page.control("enable-navigation"))
        .await?;
    assert_that!(page.control("navigation-disabled").await?).is_equal_to(0.0);
    Ok(())
}

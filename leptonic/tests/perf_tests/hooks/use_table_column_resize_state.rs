// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_table_column_resize_state`
//! (`/perf/hooks/use_table_column_resize_state`, the state alone): what each thing it offers
//! costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_table_column_resize_state.json`): creating and disposing it, the
//! table width changing (the column widths follow), a column resized, and a resize starting and
//! ending, for tables of 10 rows and of 1,000.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_table_column_resize_state";

/// Registers the performance tests of `use_table_column_resize_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(WidthChangeSmall))
        .with(fixture_test(WidthChangeLarge))
        .with(fixture_test(ResizeColumn))
        .with(fixture_test(StartAndEndResize))
}

const WIDTHS: &str = "#perf-widths";

/// Opens the fixture with its 10 rows.
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element(WIDTHS)
        .await?
        .wait_for_inner_text("300 300 300")
        .await?;
    Ok(())
}

/// Opens the fixture and creates the state for 1,000 rows (unmeasured).
async fn open_large(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(WIDTHS, 0).await?;
    page.control("large").await?;
    page.control("mount").await?;
    page.wait_for_count(WIDTHS, 1).await?;
    Ok(())
}

/// Creating the state and disposing it.
async fn mount_and_unmount_state(page: &Page<'_>) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(WIDTHS, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(WIDTHS)
        .await?
        .wait_for_inner_text("300 300 300")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(WIDTHS, 0).await?;
    Ok(())
}

/// The state for 10 rows created and disposed.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    mount_and_unmount_state(page).await
}

/// The state for 1,000 rows created and disposed.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    mount_and_unmount_state(page).await
}

/// The table narrowing from 900 to 600 pixels (each column from 300 to 200) and widening again.
async fn width_change(page: &Page<'_>) -> Result<(), Report> {
    let widths = page.element(WIDTHS).await?;
    page.measure("narrow", page.control("narrow")).await?;
    widths.wait_for_inner_text("200 200 200").await?;
    page.measure("widen", page.control("wide")).await?;
    widths.wait_for_inner_text("300 300 300").await?;
    Ok(())
}

/// The width of a table of 10 rows changing.
#[browser_test]
pub async fn width_change_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    width_change(page).await
}

/// The width of a table of 1,000 rows changing.
#[browser_test]
pub async fn width_change_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    width_change(page).await
}

/// The column Name resized to 350 pixels (the others share the rest).
#[browser_test]
pub async fn resize_column(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.measure("resize", page.control("resize-name")).await?;
    page.element(WIDTHS)
        .await?
        .wait_for_inner_text("350 275 275")
        .await?;
    Ok(())
}

/// A resize of the column Name starting and ending.
#[browser_test]
pub async fn start_and_end_resize(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let resizing = page.element("#perf-resizing").await?;
    page.measure("start", page.control("start-resize")).await?;
    resizing.wait_for_inner_text("name").await?;
    page.measure("end", page.control("end-resize")).await?;
    resizing.wait_for_inner_text("").await?;
    Ok(())
}

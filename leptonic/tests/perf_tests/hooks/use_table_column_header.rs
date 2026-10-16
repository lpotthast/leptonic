// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_table_column_header` (`/perf/hooks/use_table_column_header`,
//! the column headers of a hook table): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_table_column_header.json`): mounting and unmounting the header,
//! sorting with clicks on a column header and with Enter, its pressed state, and keyboard focus
//! moving onto the column headers, in tables of 10 rows and of 1,000.
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_table_column_header";

/// Registers the performance tests of `use_table_column_header`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(SortByClickSmall))
        .with(fixture_test(SortByClickLarge))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(KeyboardFocusAndSort))
}

const TABLE: &str = "[role=grid]";
const HEADER: &str = "[role=grid] > thead";
const ROWS: &str = "[role=grid] > tbody > tr";
/// The column header of the column "Value".
const VALUE_HEADER: &str = "[role=grid] > thead th:nth-child(2)";
/// The assertive announcements (of sort changes, each removed after 500 ms).
const SORT_ANNOUNCEMENTS: &str = "[data-live-announcer] [aria-live=assertive] > div";

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

/// Creates the live announcer's regions: its first announcement waits 100 ms for them, which
/// would fall into a later measurement.
async fn prepare_announcer(page: &Page<'_>) -> Result<(), Report> {
    page.control("prepare-announcer").await?;
    page.wait_for_count("[data-live-announcer] [aria-live=polite] > div", 1)
        .await?;
    Ok(())
}

/// Mounting the header alone and unmounting it.
async fn mount_and_unmount_header(page: &Page<'_>) -> Result<(), Report> {
    page.control("unmount-header").await?;
    page.wait_for_count(HEADER, 0).await?;
    page.measure("mount", page.control("mount-header")).await?;
    page.wait_for_count(VALUE_HEADER, 1).await?;
    page.measure("unmount", page.control("unmount-header"))
        .await?;
    page.wait_for_count(HEADER, 0).await?;
    Ok(())
}

/// Mounting the header of a table of 10 rows and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    mount_and_unmount_header(page).await
}

/// Mounting the header of a table of 1,000 rows and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    mount_and_unmount_header(page).await
}

/// Clicks on the column header "Value": sorting ascending, then descending. Each measured after
/// the previous sort's announcement was removed (500 ms).
async fn sort_by_click(page: &Page<'_>) -> Result<(), Report> {
    prepare_announcer(page).await?;
    let header = page.element(VALUE_HEADER).await?;
    page.measure("ascending", header.click()).await?;
    header.wait_for_attr("aria-sort", Some("ascending")).await?;
    page.wait_for_count(SORT_ANNOUNCEMENTS, 0).await?;
    page.measure("descending", header.click()).await?;
    header
        .wait_for_attr("aria-sort", Some("descending"))
        .await?;
    page.wait_for_count(SORT_ANNOUNCEMENTS, 0).await?;
    Ok(())
}

/// Sorting a table of 10 rows by clicks.
#[browser_test]
pub async fn sort_by_click_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    sort_by_click(page).await
}

/// Sorting a table of 1,000 rows by clicks.
#[browser_test]
pub async fn sort_by_click_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    sort_by_click(page).await
}

/// Holding the pointer down on a column header (the pressed state) and releasing it (a sort).
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    prepare_announcer(page).await?;
    let header = page.element(VALUE_HEADER).await?;
    let held = page.measure("hold", header.press_and_hold()).await?;
    header.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("release", held.release()).await?;
    header.wait_for_attr("data-pressed", None).await?;
    header.wait_for_attr("aria-sort", Some("ascending")).await?;
    page.wait_for_count(SORT_ANNOUNCEMENTS, 0).await?;
    Ok(())
}

/// ArrowUp from the first row's first cell onto the column header "Name", ArrowRight to
/// "Value", and Enter sorting by it.
#[browser_test]
pub async fn keyboard_focus_and_sort(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    prepare_announcer(page).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(
        &page
            .element("[role=grid] > tbody > tr:nth-child(1)")
            .await?,
    )
    .await?;
    page.send_keys(Key::Right).await?;
    page.measure("arrow up", page.send_keys(Key::Up)).await?;
    page.wait_for_focus(&page.element("[role=grid] > thead th:nth-child(1)").await?)
        .await?;
    page.measure("arrow right", page.send_keys(Key::Right))
        .await?;
    let header = page.element(VALUE_HEADER).await?;
    page.wait_for_focus(&header).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    header.wait_for_attr("aria-sort", Some("ascending")).await?;
    page.wait_for_count(SORT_ANNOUNCEMENTS, 0).await?;
    Ok(())
}

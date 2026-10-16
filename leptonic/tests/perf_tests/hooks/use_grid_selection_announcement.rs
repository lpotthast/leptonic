// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_grid_selection_announcement`
//! (`/perf/hooks/use_grid_selection_announcement`, the hook on a grid state): what each thing it
//! offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_grid_selection_announcement.json`): mounting and unmounting it, and
//! the announcements of selection changes while the grid has focus (one row, all, none, every
//! row by key) and a change while it hasn't (announced after a frame, if focus came back), with
//! 10 rows and with 1,000.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/hooks/use_grid_selection_announcement";

/// Registers the performance tests of `use_grid_selection_announcement`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(AnnouncementsSmall))
        .with(fixture_test(AnnouncementsLarge))
        .with(fixture_test(ChangeWithoutFocus))
}

const HOOK: &str = "#perf-announcer";
/// The polite announcements (the hook's), including the empty one `prepare_announcer` makes.
const ANNOUNCEMENTS: &str = "[data-live-announcer] [aria-live=polite] > div";

/// Opens the fixture with its 10 rows.
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(HOOK, 1).await?;
    Ok(())
}

/// Opens the fixture with 1,000 rows (unmeasured).
async fn open_large(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(HOOK, 0).await?;
    page.control("large").await?;
    page.control("mount").await?;
    page.wait_for_count(HOOK, 1).await?;
    Ok(())
}

/// Creates the live announcer's regions: its first announcement waits 100 ms for them, which
/// would fall into a later measurement.
async fn prepare_announcer(page: &Page<'_>) -> Result<(), Report> {
    page.control("prepare-announcer").await?;
    page.wait_for_count(ANNOUNCEMENTS, 1).await?;
    Ok(())
}

/// Mounting the hook and unmounting it.
async fn mount_and_unmount_hook(page: &Page<'_>) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(HOOK, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(HOOK, 1).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(HOOK, 0).await?;
    Ok(())
}

/// Mounting the hook on 10 rows and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    mount_and_unmount_hook(page).await
}

/// Mounting the hook on 1,000 rows and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    mount_and_unmount_hook(page).await
}

/// Selection changes while the grid has focus, each announced: one row selected, all selected,
/// none, every row selected by key.
async fn announcements(page: &Page<'_>) -> Result<(), Report> {
    prepare_announcer(page).await?;
    page.control("focus").await?;
    page.measure("select one", page.control("toggle-first"))
        .await?;
    page.wait_for_count(ANNOUNCEMENTS, 2).await?;
    page.measure("select all", page.control("select-all"))
        .await?;
    page.wait_for_count(ANNOUNCEMENTS, 3).await?;
    page.measure("clear", page.control("clear-selection"))
        .await?;
    page.wait_for_count(ANNOUNCEMENTS, 4).await?;
    page.measure("select every key", page.control("select-every-key"))
        .await?;
    page.wait_for_count(ANNOUNCEMENTS, 5).await?;
    Ok(())
}

/// The announcements with 10 rows.
#[browser_test]
pub async fn announcements_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    announcements(page).await
}

/// The announcements with 1,000 rows.
#[browser_test]
pub async fn announcements_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    announcements(page).await
}

/// A selection change while the grid doesn't have focus: the hook checks again a frame later
/// (when focus may have come back) and announces nothing.
#[browser_test]
pub async fn change_without_focus(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    prepare_announcer(page).await?;
    page.measure("select one", page.control("toggle-first"))
        .await?;
    page.count_stays(ANNOUNCEMENTS, 1, std::time::Duration::from_millis(200))
        .await?;
    Ok(())
}

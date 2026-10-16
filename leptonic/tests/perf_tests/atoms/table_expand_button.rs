// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `TableExpandButton` (`/perf/atoms/table_expand_button`, the expand
//! buttons of a tree table's parent rows): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/table_expand_button.json`): mounting and unmounting the table, clicks
//! expanding and collapsing a row, its hovered and pressed states, the controlled expanded rows
//! changing (all, none), and the row's ArrowRight and ArrowLeft, which expand and collapse it too
//! (the button shows it), with 5 parent rows (10 rows) and with 500 (1,000 rows).
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/table_expand_button";

/// Registers the performance tests of `TableExpandButton`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(ClickSmall))
        .with(fixture_test(ClickLarge))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(ExpandAllSmall))
        .with(fixture_test(ExpandAllLarge))
        .with(fixture_test(KeyboardSmall))
        .with(fixture_test(KeyboardLarge))
}

const TABLE: &str = "[role=treegrid]";
/// The rows shown (child rows of collapsed rows are hidden or not rendered yet).
const SHOWN_ROWS: &str = "[role=treegrid] > tbody > tr:not([hidden])";
/// The first parent row.
const FIRST_PARENT: &str = "[role=treegrid] > tbody > tr:nth-child(1)";
/// The expand button of the first parent row.
const FIRST_BUTTON: &str = "[role=treegrid] > tbody > tr:nth-child(1) .leptonic-TableExpandButton";
/// The expand button of the last parent row, once every row is rendered (each parent row is
/// followed by its child row).
const LAST_BUTTON: &str =
    "[role=treegrid] > tbody > tr:nth-last-child(2) .leptonic-TableExpandButton";

/// Opens the fixture with its 5 parent rows.
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(SHOWN_ROWS, 5).await?;
    Ok(())
}

/// Opens the fixture and mounts the table with 500 parent rows (unmeasured).
async fn open_large(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(TABLE, 0).await?;
    page.control("large").await?;
    page.control("mount").await?;
    page.wait_for_count(SHOWN_ROWS, 500).await?;
    Ok(())
}

/// Mounting the table with its `parents` (collapsed) parent rows and unmounting it.
async fn mount_and_unmount_table(page: &Page<'_>, parents: usize) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(TABLE, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(SHOWN_ROWS, parents).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TABLE, 0).await?;
    Ok(())
}

/// Mounting the table with 5 parent rows and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    mount_and_unmount_table(page, 5).await
}

/// Mounting the table with 500 parent rows and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    mount_and_unmount_table(page, 500).await
}

/// A click on the first row's expand button expanding it (its child row appears), one collapsing
/// it, and one expanding it again (the child row was rendered before).
async fn click(page: &Page<'_>, parents: usize) -> Result<(), Report> {
    let button = page.element(FIRST_BUTTON).await?;
    page.measure("expand", button.click()).await?;
    button.wait_for_attr("data-expanded", Some("true")).await?;
    page.wait_for_count(SHOWN_ROWS, parents + 1).await?;
    page.measure("collapse", button.click()).await?;
    button.wait_for_attr("data-expanded", None).await?;
    page.wait_for_count(SHOWN_ROWS, parents).await?;
    page.measure("expand again", button.click()).await?;
    page.wait_for_count(SHOWN_ROWS, parents + 1).await?;
    Ok(())
}

/// Clicks on an expand button of a table of 5 parent rows.
#[browser_test]
pub async fn click_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    click(page, 5).await
}

/// Clicks on an expand button of a table of 500 parent rows.
#[browser_test]
pub async fn click_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    click(page, 500).await
}

/// The pointer entering and leaving an expand button.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let button = page.element(FIRST_BUTTON).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", button.hover()).await?;
    button.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    button.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Holding the pointer down on an expand button (the pressed state) and releasing it (the row
/// expands).
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let button = page.element(FIRST_BUTTON).await?;
    let held = page.measure("hold", button.press_and_hold()).await?;
    button.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("release", held.release()).await?;
    button.wait_for_attr("data-pressed", None).await?;
    button.wait_for_attr("data-expanded", Some("true")).await?;
    Ok(())
}

/// The controlled expanded rows changing: all parent rows expanded, then none.
async fn expand_all(page: &Page<'_>, parents: usize) -> Result<(), Report> {
    page.measure("expand all", page.control("expand-all"))
        .await?;
    page.wait_for_count(SHOWN_ROWS, 2 * parents).await?;
    page.element(LAST_BUTTON)
        .await?
        .wait_for_attr("data-expanded", Some("true"))
        .await?;
    page.measure("collapse all", page.control("collapse-all"))
        .await?;
    page.wait_for_count(SHOWN_ROWS, parents).await?;
    Ok(())
}

/// All of 5 parent rows expanded and collapsed.
#[browser_test]
pub async fn expand_all_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    expand_all(page, 5).await
}

/// All of 500 parent rows expanded and collapsed.
#[browser_test]
pub async fn expand_all_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    expand_all(page, 500).await
}

/// ArrowRight on the focused first row expanding it, ArrowLeft collapsing it.
async fn keyboard(page: &Page<'_>, parents: usize) -> Result<(), Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(FIRST_PARENT).await?)
        .await?;
    let button = page.element(FIRST_BUTTON).await?;
    page.measure("expand", page.send_keys(Key::Right)).await?;
    button.wait_for_attr("data-expanded", Some("true")).await?;
    page.wait_for_count(SHOWN_ROWS, parents + 1).await?;
    page.measure("collapse", page.send_keys(Key::Left)).await?;
    button.wait_for_attr("data-expanded", None).await?;
    page.wait_for_count(SHOWN_ROWS, parents).await?;
    Ok(())
}

/// The keys in a table of 5 parent rows.
#[browser_test]
pub async fn keyboard_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    keyboard(page, 5).await
}

/// The keys in a table of 500 parent rows.
#[browser_test]
pub async fn keyboard_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    keyboard(page, 500).await
}

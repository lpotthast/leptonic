// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_tree_item` (`/perf/hooks/use_tree_item`, items of a hook
//! tree): what each thing it offers costs, measured (`Page::measure`) and checked against its
//! budget (`tests/budgets/hooks/use_tree_item.json`): an item mounting and unmounting in its
//! tree, a click expanding and collapsing it (without selection), its expand button, its pressed
//! state, ArrowRight and ArrowLeft expanding and collapsing it, every item expanded at once
//! (their children mount), and clicks and Space selecting it (with selection), with 5 parent
//! items (collapsed: 10 items) and with 500 (1,000 items).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_tree_item";

/// Registers the performance tests of `use_tree_item`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(ClickSmall))
        .with(fixture_test(ClickLarge))
        .with(fixture_test(ExpandButton))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(ArrowKeysSmall))
        .with(fixture_test(ArrowKeysLarge))
        .with(fixture_test(ExpandAllSmall))
        .with(fixture_test(ExpandAllLarge))
        .with(fixture_test(SelectionSmall))
        .with(fixture_test(SelectionLarge))
}

const TREE: &str = "[role=treegrid]";
const ITEMS: &str = "[role=treegrid] > [role=row]";

/// Visible item `i` (from 0).
fn item(i: usize) -> String {
    format!("[role=treegrid] > [role=row]:nth-child({})", i + 1)
}

/// Opens the fixture with its 5 parents (`mount`: `mount` or `mount-selectable`).
async fn open(page: &Page<'_>, mount: &str) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ITEMS, 5).await?;
    if mount != "mount" {
        page.control("unmount").await?;
        page.wait_for_count(TREE, 0).await?;
        page.control(mount).await?;
        page.wait_for_count(ITEMS, 5).await?;
    }
    Ok(())
}

/// Opens the fixture and mounts the tree (`mount`: `mount` or `mount-selectable`) with 500
/// parents (unmeasured).
async fn open_large(page: &Page<'_>, mount: &str) -> Result<(), Report> {
    open(page, "mount").await?;
    page.control("unmount").await?;
    page.wait_for_count(TREE, 0).await?;
    page.control("large").await?;
    page.control(mount).await?;
    page.wait_for_count(ITEMS, 500).await?;
    Ok(())
}

/// Creates the live announcer's regions: its first announcement (of a selection change while
/// the tree has focus) waits 100 ms for them, which would fall into a later measurement.
async fn prepare_announcer(page: &Page<'_>) -> Result<(), Report> {
    page.control("prepare-announcer").await?;
    page.wait_for_count("[data-live-announcer] [aria-live=polite] > div", 1)
        .await?;
    Ok(())
}

/// A parent added at the end of the tree's `parents` and removed again.
async fn mount_and_unmount_item(page: &Page<'_>, parents: usize) -> Result<(), Report> {
    page.measure("mount", page.control("add-parent")).await?;
    page.wait_for_count(ITEMS, parents + 1).await?;
    page.measure("unmount", page.control("remove-parent"))
        .await?;
    page.wait_for_count(ITEMS, parents).await?;
    Ok(())
}

/// An item mounting in a tree of 5 parents and unmounting.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page, "mount").await?;
    mount_and_unmount_item(page, 5).await
}

/// An item mounting in a tree of 500 parents and unmounting.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page, "mount").await?;
    mount_and_unmount_item(page, 500).await
}

/// A click on the first parent expanding it (its child mounts), and one collapsing it.
async fn click(page: &Page<'_>, parents: usize) -> Result<(), Report> {
    let first = page.element(item(0)).await?;
    page.measure("expand", first.click()).await?;
    first.wait_for_attr("aria-expanded", Some("true")).await?;
    page.wait_for_count(ITEMS, parents + 1).await?;
    page.measure("collapse", first.click()).await?;
    first.wait_for_attr("aria-expanded", Some("false")).await?;
    page.wait_for_count(ITEMS, parents).await?;
    Ok(())
}

/// Clicks on a parent of 5.
#[browser_test]
pub async fn click_small(page: &Page<'_>) -> Result<(), Report> {
    open(page, "mount").await?;
    click(page, 5).await
}

/// Clicks on a parent of 500.
#[browser_test]
pub async fn click_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page, "mount").await?;
    click(page, 500).await
}

/// Clicks on the first parent's expand button: expanding it, and collapsing it.
#[browser_test]
pub async fn expand_button(page: &Page<'_>) -> Result<(), Report> {
    open(page, "mount").await?;
    let first = page.element(item(0)).await?;
    let button = first.element("button").await?;
    page.measure("expand", button.click()).await?;
    first.wait_for_attr("aria-expanded", Some("true")).await?;
    page.measure("collapse", button.click()).await?;
    first.wait_for_attr("aria-expanded", Some("false")).await?;
    Ok(())
}

/// Holding the pointer down on a parent (the pressed state) and releasing it (it expands).
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    open(page, "mount").await?;
    let first = page.element(item(0)).await?;
    let held = page.measure("hold", first.press_and_hold()).await?;
    first.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("release", held.release()).await?;
    first.wait_for_attr("data-pressed", None).await?;
    first.wait_for_attr("aria-expanded", Some("true")).await?;
    Ok(())
}

/// ArrowRight on the focused first parent expanding it, ArrowLeft collapsing it.
async fn arrow_keys(page: &Page<'_>, parents: usize) -> Result<(), Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let first = page.element(item(0)).await?;
    page.wait_for_focus(&first).await?;
    page.measure("expand", page.send_keys(Key::Right)).await?;
    first.wait_for_attr("aria-expanded", Some("true")).await?;
    page.wait_for_count(ITEMS, parents + 1).await?;
    page.measure("collapse", page.send_keys(Key::Left)).await?;
    first.wait_for_attr("aria-expanded", Some("false")).await?;
    page.wait_for_count(ITEMS, parents).await?;
    Ok(())
}

/// The keys in a tree of 5 parents.
#[browser_test]
pub async fn arrow_keys_small(page: &Page<'_>) -> Result<(), Report> {
    open(page, "mount").await?;
    arrow_keys(page, 5).await
}

/// The keys in a tree of 500 parents.
#[browser_test]
pub async fn arrow_keys_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page, "mount").await?;
    arrow_keys(page, 500).await
}

/// Every parent expanded at once (each child item mounts), then collapsed.
async fn expand_all(page: &Page<'_>, parents: usize) -> Result<(), Report> {
    page.measure("expand all", page.control("expand-all"))
        .await?;
    page.wait_for_count(ITEMS, 2 * parents).await?;
    page.measure("collapse all", page.control("collapse-all"))
        .await?;
    page.wait_for_count(ITEMS, parents).await?;
    Ok(())
}

/// All of 5 parents expanded and collapsed.
#[browser_test]
pub async fn expand_all_small(page: &Page<'_>) -> Result<(), Report> {
    open(page, "mount").await?;
    expand_all(page, 5).await
}

/// All of 500 parents expanded and collapsed.
#[browser_test]
pub async fn expand_all_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page, "mount").await?;
    expand_all(page, 500).await
}

/// With selection: a click selecting the second parent, and Space (the item focused)
/// deselecting it.
async fn selection(page: &Page<'_>) -> Result<(), Report> {
    prepare_announcer(page).await?;
    let second = page.element(item(1)).await?;
    page.measure("select", second.click()).await?;
    second.wait_for_attr("aria-selected", Some("true")).await?;
    page.wait_for_focus(&second).await?;
    page.measure("deselect", page.send_keys(" ")).await?;
    second.wait_for_attr("aria-selected", Some("false")).await?;
    assert_that!(page.control("selected").await?).is_equal_to(0.0);
    Ok(())
}

/// Selecting an item of 5 parents.
#[browser_test]
pub async fn selection_small(page: &Page<'_>) -> Result<(), Report> {
    open(page, "mount-selectable").await?;
    selection(page).await
}

/// Selecting an item of 500 parents.
#[browser_test]
pub async fn selection_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page, "mount-selectable").await?;
    selection(page).await
}

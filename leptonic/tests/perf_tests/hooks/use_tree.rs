// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_tree` (`/perf/hooks/use_tree`, a tree of hook items on plain
//! elements): what each thing it offers costs, measured (`Page::measure`) and checked against its
//! budget (`tests/budgets/hooks/use_tree.json`): mounting and unmounting the tree and its items,
//! keyboard focus, its navigation keys and type-ahead, with 5 parent items (collapsed: 10 items)
//! and with 500 (1,000 items).
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/hooks/use_tree";

/// Registers the performance tests of `use_tree`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(NavigationKeysSmall))
        .with(fixture_test(NavigationKeysLarge))
        .with(fixture_test(TypeAheadSmall))
        .with(fixture_test(TypeAheadLarge))
}

/// Types `text` one key at a time, each key measured as "type" from an idle page (keys sent at
/// once fall into frames differently from run to run).
async fn type_each_key(page: &Page<'_>, text: &str) -> Result<(), Report> {
    for key in text.chars() {
        page.measure("type", page.send_keys(key.to_string()))
            .await?;
    }
    Ok(())
}

const TREE: &str = "[role=treegrid]";
const ITEMS: &str = "[role=treegrid] > [role=row]";

/// Visible item `i` (from 0).
fn item(i: usize) -> String {
    format!("[role=treegrid] > [role=row]:nth-child({})", i + 1)
}

/// Opens the fixture with its 5 parents.
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ITEMS, 5).await?;
    Ok(())
}

/// Opens the fixture and mounts the tree with 500 parents (unmeasured).
async fn open_large(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(TREE, 0).await?;
    page.control("large").await?;
    page.control("mount").await?;
    page.wait_for_count(ITEMS, 500).await?;
    Ok(())
}

/// Focuses the tree with the keyboard (Tab from the plain button before it): its first item.
async fn focus_with_tab(page: &Page<'_>) -> Result<(), Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(item(0)).await?).await?;
    Ok(())
}

/// Mounting the tree with its `items` visible items and unmounting it.
async fn mount_and_unmount_tree(page: &Page<'_>, items: usize) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(TREE, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(ITEMS, items).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TREE, 0).await?;
    Ok(())
}

/// Mounting the tree of 5 parents and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    mount_and_unmount_tree(page, 5).await
}

/// Mounting the tree of 500 parents and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    mount_and_unmount_tree(page, 500).await
}

/// Tab moving focus into the tree (its first item) and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element("#perf-before").await?.click().await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&page.element(item(0)).await?).await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&page.element("#perf-after").await?)
        .await?;
    Ok(())
}

/// The navigation keys, from the first item: ArrowDown, ArrowUp, End, Home, PageDown and
/// PageUp.
async fn navigation_keys(page: &Page<'_>, last: usize) -> Result<(), Report> {
    focus_with_tab(page).await?;
    page.measure("arrow down", page.send_keys(Key::Down))
        .await?;
    page.wait_for_focus(&page.element(item(1)).await?).await?;
    page.measure("arrow up", page.send_keys(Key::Up)).await?;
    page.wait_for_focus(&page.element(item(0)).await?).await?;
    page.measure("end", page.send_keys(Key::End)).await?;
    page.wait_for_focus(&page.element(item(last)).await?)
        .await?;
    page.measure("home", page.send_keys(Key::Home)).await?;
    page.wait_for_focus(&page.element(item(0)).await?).await?;
    page.measure("page down", page.send_keys(Key::PageDown))
        .await?;
    page.measure("page up", page.send_keys(Key::PageUp)).await?;
    page.wait_for_focus(&page.element(item(0)).await?).await?;
    Ok(())
}

/// The navigation keys in the tree of 5 parents.
#[browser_test]
pub async fn navigation_keys_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    navigation_keys(page, 4).await
}

/// The navigation keys in the tree of 500 parents.
#[browser_test]
pub async fn navigation_keys_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    navigation_keys(page, 499).await
}

/// Typing the text of the last of 5 parents ("Parent 4") moves focus to it (type-ahead).
/// Measured last: the search ends after a timeout.
#[browser_test]
pub async fn type_ahead_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    focus_with_tab(page).await?;
    type_each_key(page, "Parent 4").await?;
    page.wait_for_focus(&page.element(item(4)).await?).await?;
    Ok(())
}

/// Typing the text of the last of 500 parents ("Parent 499") moves focus to it.
#[browser_test]
pub async fn type_ahead_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    focus_with_tab(page).await?;
    type_each_key(page, "Parent 499").await?;
    page.wait_for_focus(&page.element(item(499)).await?).await?;
    Ok(())
}

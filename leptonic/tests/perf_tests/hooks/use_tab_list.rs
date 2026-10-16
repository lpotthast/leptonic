// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_tab_list` (`/perf/hooks/use_tab_list`, on a plain `<div>`
//! with tabs of `use_tab`): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/hooks/use_tab_list.json`): mounting and
//! unmounting it with 3 and 100 tabs, the arrow keys, Home and End moving the selection (at both
//! sizes), keyboard focus entering and leaving it, and changes of its label and orientation.
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/hooks/use_tab_list";

/// Registers the performance tests of `use_tab_list`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountMany))
        .with(fixture_test(ArrowKeys))
        .with(fixture_test(ArrowKeysMany))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(ChangeTheLabel))
        .with(fixture_test(ChangeTheOrientation))
}
const TAB_LIST: &str = ".perf-tab-list";
const ALL_TABS: &str = ".perf-tab-list [role=tab]";

/// The `n`th tab (from 1).
fn tab(n: usize) -> String {
    format!("{TAB_LIST} > :nth-child({n})")
}

/// Mounts the list with 100 tabs, from a fresh page.
async fn goto_many(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(TAB_LIST, 0).await?;
    page.control("use-many").await?;
    page.control("mount").await?;
    page.wait_for_count(ALL_TABS, 100).await?;
    Ok(())
}

/// The arrow keys, End and Home moving the selection through `last` tabs.
async fn arrow_keys_through(page: &Page<'_>, last: usize) -> Result<(), Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let first = page.element(tab(1)).await?;
    page.wait_for_focus(&first).await?;
    page.measure("arrow right", page.send_keys(Key::Right))
        .await?;
    page.wait_for_attr(tab(2), "aria-selected", Some("true"))
        .await?;
    page.measure("arrow left", page.send_keys(Key::Left))
        .await?;
    page.wait_for_attr(tab(1), "aria-selected", Some("true"))
        .await?;
    page.measure("end", page.send_keys(Key::End)).await?;
    page.wait_for_attr(tab(last), "aria-selected", Some("true"))
        .await?;
    page.measure("home", page.send_keys(Key::Home)).await?;
    page.wait_for_attr(tab(1), "aria-selected", Some("true"))
        .await?;
    Ok(())
}

/// Mounting the list with 3 tabs and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(TAB_LIST, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(ALL_TABS, 3).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TAB_LIST, 0).await?;
    // Tabs and panels reference each other: mounted again, the page is valid at the end.
    page.control("mount").await?;
    page.first_element(ALL_TABS).await?;
    Ok(())
}

/// Mounting the list with 100 tabs and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_many(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(TAB_LIST, 0).await?;
    page.control("use-many").await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(ALL_TABS, 100).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TAB_LIST, 0).await?;
    // Tabs and panels reference each other: mounted again, the page is valid at the end.
    page.control("mount").await?;
    page.first_element(ALL_TABS).await?;
    Ok(())
}

/// The arrow keys, End and Home moving the selection through 3 tabs.
#[browser_test]
pub async fn arrow_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    arrow_keys_through(page, 3).await
}

/// The arrow keys, End and Home moving the selection through 100 tabs.
#[browser_test]
pub async fn arrow_keys_many(page: &Page<'_>) -> Result<(), Report> {
    goto_many(page).await?;
    arrow_keys_through(page, 100).await
}

/// Tab moving focus into the list (its selected tab) and on to the panel.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let first = page.element(tab(1)).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&first).await?;
    let panel = page.element("[role=tabpanel]").await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&panel).await?;
    Ok(())
}

/// Labeling the list and removing the label again.
#[browser_test]
pub async fn change_the_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("set", page.control("set-label")).await?;
    page.wait_for_attr(TAB_LIST, "aria-label", Some("Sections"))
        .await?;
    page.measure("unset", page.control("unset-label")).await?;
    page.wait_for_attr(TAB_LIST, "aria-label", None).await?;
    Ok(())
}

/// Making the list vertical and horizontal again.
#[browser_test]
pub async fn change_the_orientation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("vertical", page.control("make-vertical"))
        .await?;
    page.wait_for_attr(TAB_LIST, "aria-orientation", Some("vertical"))
        .await?;
    page.measure("horizontal", page.control("make-horizontal"))
        .await?;
    page.wait_for_attr(TAB_LIST, "aria-orientation", Some("horizontal"))
        .await?;
    Ok(())
}

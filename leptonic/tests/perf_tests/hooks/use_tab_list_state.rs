// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_tab_list_state` (`/perf/hooks/use_tab_list_state`, a tab
//! list's state alone): what each thing it offers costs, measured (`Page::measure`) and checked
//! against its budget (`tests/budgets/hooks/use_tab_list_state.json`): creating and disposing of
//! the state of 3 and 1,000 tabs, selecting tabs (at both sizes), changing the disabled keys, and
//! the selected tab leaving the collection (and coming back).
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_tab_list_state";

/// Registers the performance tests of `use_tab_list_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountMany))
        .with(fixture_test(Select))
        .with(fixture_test(SelectMany))
        .with(fixture_test(DisableATab))
        .with(fixture_test(RemoveTheSelectedTab))
        .with(fixture_test(RemoveTheSelectedTabMany))
}
const SELECTED: &str = "#perf-selected";

/// Waits until the state's selected key is `key`.
async fn wait_for_selected(page: &Page<'_>, key: &str) -> Result<(), Report> {
    page.element(SELECTED).await?.wait_for_inner_text(key).await
}

/// Opens the fixture with the state of 1,000 tabs.
async fn goto_many(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(SELECTED, 0).await?;
    page.control("use-many").await?;
    page.control("mount").await?;
    wait_for_selected(page, "tab-1").await
}

/// Creating the state of 3 tabs (the first selected) and disposing of it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(SELECTED, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    wait_for_selected(page, "tab-1").await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(SELECTED, 0).await?;
    Ok(())
}

/// Creating the state of 1,000 tabs and disposing of it.
#[browser_test]
pub async fn mount_and_unmount_many(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(SELECTED, 0).await?;
    page.control("use-many").await?;
    page.measure("mount", page.control("mount")).await?;
    wait_for_selected(page, "tab-1").await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(SELECTED, 0).await?;
    Ok(())
}

/// Selecting the second of 3 tabs and the first again (`set_selected_key`).
#[browser_test]
pub async fn select(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    wait_for_selected(page, "tab-1").await?;
    page.measure("select the second", page.control("select-second"))
        .await?;
    wait_for_selected(page, "tab-2").await?;
    page.measure("select the first", page.control("select-first"))
        .await?;
    wait_for_selected(page, "tab-1").await?;
    Ok(())
}

/// Selecting the second of 1,000 tabs, the last and the first again.
#[browser_test]
pub async fn select_many(page: &Page<'_>) -> Result<(), Report> {
    goto_many(page).await?;
    page.measure("select the second", page.control("select-second"))
        .await?;
    wait_for_selected(page, "tab-2").await?;
    page.measure("select the last", page.control("select-last"))
        .await?;
    wait_for_selected(page, "tab-1000").await?;
    page.measure("select the first", page.control("select-first"))
        .await?;
    wait_for_selected(page, "tab-1").await?;
    Ok(())
}

/// Disabling the second tab (`disabled_keys`) and enabling it again: the selection stays.
#[browser_test]
pub async fn disable_a_tab(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    wait_for_selected(page, "tab-1").await?;
    page.measure("disable", page.control("disable-second"))
        .await?;
    wait_for_selected(page, "tab-1").await?;
    page.measure("enable", page.control("enable-second"))
        .await?;
    wait_for_selected(page, "tab-1").await?;
    Ok(())
}

/// The selected (first) of 3 tabs leaving the collection (the first remaining tab is selected
/// instead) and coming back (the selection stays).
#[browser_test]
pub async fn remove_the_selected_tab(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    wait_for_selected(page, "tab-1").await?;
    page.measure("remove", page.control("remove-first")).await?;
    wait_for_selected(page, "tab-2").await?;
    page.measure("add", page.control("add-first")).await?;
    wait_for_selected(page, "tab-2").await?;
    Ok(())
}

/// The selected (first) of 1,000 tabs leaving the collection and coming back.
#[browser_test]
pub async fn remove_the_selected_tab_many(page: &Page<'_>) -> Result<(), Report> {
    goto_many(page).await?;
    page.measure("remove", page.control("remove-first")).await?;
    wait_for_selected(page, "tab-2").await?;
    page.measure("add", page.control("add-first")).await?;
    wait_for_selected(page, "tab-2").await?;
    Ok(())
}

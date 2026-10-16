// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_submenu_trigger_state`
//! (`/perf/hooks/use_submenu_trigger_state`): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_submenu_trigger_state.json`): mounting and unmounting it, opening and
//! closing the submenu, toggling it, and closing all menus (the root's, too).
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_submenu_trigger_state";
const OPEN: &str = "#perf-open";

/// Registers the performance tests of `use_submenu_trigger_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(OpenAndClose))
        .with(fixture_test(Toggle))
        .with(fixture_test(CloseAll))
}

/// Mounting the state and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(OPEN, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(OPEN)
        .await?
        .wait_for_inner_text("false")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(OPEN, 0).await?;
    Ok(())
}

/// `open()` (focusing the first item) and `close()`.
#[browser_test]
pub async fn open_and_close(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let open = page.element(OPEN).await?;
    page.measure("open", page.control("open")).await?;
    open.wait_for_inner_text("true").await?;
    page.measure("close", page.control("close")).await?;
    open.wait_for_inner_text("false").await?;
    Ok(())
}

/// `toggle()` twice: opening, then closing.
#[browser_test]
pub async fn toggle(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let open = page.element(OPEN).await?;
    page.measure("toggle", page.control("toggle")).await?;
    open.wait_for_inner_text("true").await?;
    page.measure("toggle", page.control("toggle")).await?;
    open.wait_for_inner_text("false").await?;
    Ok(())
}

/// `close_all()` with the submenu open: the submenu and the root menu close.
#[browser_test]
pub async fn close_all(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let open = page.element(OPEN).await?;
    page.control("open").await?;
    open.wait_for_inner_text("true").await?;
    page.measure("close all", page.control("close-all")).await?;
    open.wait_for_inner_text("false").await?;
    page.element("#perf-root-open")
        .await?
        .wait_for_inner_text("false")
        .await?;
    Ok(())
}

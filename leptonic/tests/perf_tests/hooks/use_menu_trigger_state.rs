// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_menu_trigger_state` (`/perf/hooks/use_menu_trigger_state`):
//! what each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_menu_trigger_state.json`): mounting and unmounting it, opening and
//! closing, toggling, opening and closing a submenu, and closing with a submenu open (closing it,
//! too).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_menu_trigger_state";
const OPEN: &str = "#perf-open";
const SUBMENUS: &str = "#perf-submenus";

/// Registers the performance tests of `use_menu_trigger_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(OpenAndClose))
        .with(fixture_test(Toggle))
        .with(fixture_test(Submenu))
        .with(fixture_test(CloseWithSubmenu))
}

/// How often the state reported a change.
async fn changes(page: &Page<'_>) -> Result<f64, Report> {
    page.control("changes").await
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
    assert_that!(changes(page).await?).is_equal_to(2.0);
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
    assert_that!(changes(page).await?).is_equal_to(2.0);
    Ok(())
}

/// Opening the submenu of `share` and closing it, the menu open.
#[browser_test]
pub async fn submenu(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("open").await?;
    let submenus = page.element(SUBMENUS).await?;
    page.measure("open submenu", page.control("open-submenu"))
        .await?;
    submenus.wait_for_inner_text("share").await?;
    page.measure("close submenu", page.control("close-submenu"))
        .await?;
    submenus.wait_for_inner_text("").await?;
    Ok(())
}

/// Closing the menu with a submenu open: the submenu closes, too.
#[browser_test]
pub async fn close_with_submenu(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("open").await?;
    page.control("open-submenu").await?;
    let submenus = page.element(SUBMENUS).await?;
    submenus.wait_for_inner_text("share").await?;
    page.measure("close", page.control("close")).await?;
    submenus.wait_for_inner_text("").await?;
    page.element(OPEN)
        .await?
        .wait_for_inner_text("false")
        .await?;
    Ok(())
}

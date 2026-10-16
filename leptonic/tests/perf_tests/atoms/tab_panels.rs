// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `TabPanels` (`/perf/atoms/tab_panels`, around three panels of
//! different heights): what each thing it offers costs, measured (`Page::measure`) and checked
//! against its budget (`tests/budgets/atoms/tab_panels.json`): mounting and unmounting it (with
//! its panels), and a selection change resizing it, without and with a size transition.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::WebElement};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/tab_panels";

/// Registers the performance tests of `TabPanels`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(ChangeTheSelection))
        .with(fixture_test(ChangeTheSelectionWithATransition))
}
const PANELS: &str = "#perf-tabs .leptonic-TabPanels";
const PANEL: &str = "#perf-tabs [role=tabpanel]";
/// The lines of the panel shown.
const LINES: &str = "#perf-tabs [role=tabpanel] p";

/// Selects the second tab and the first again, each measured: the panels resize to the new
/// panel (their size variables `auto` again afterwards).
async fn change_the_selection_twice(page: &Page<'_>) -> Result<(), Report> {
    page.measure("select the second", page.control("select-second"))
        .await?;
    page.wait_for_count(LINES, 5).await?;
    let panels = page.element(PANELS).await?;
    assert_auto_size(&panels).await?;
    page.measure("select the first", page.control("select-first"))
        .await?;
    page.wait_for_count(LINES, 1).await?;
    assert_auto_size(&panels).await?;
    Ok(())
}

/// The panels' size variables are `auto` (no size transition running).
async fn assert_auto_size(panels: &WebElement) -> Result<(), Report> {
    assert_that!(panels.style_property("--tab-panel-height").await?).is_equal_to("auto");
    assert_that!(panels.style_property("--tab-panel-width").await?).is_equal_to("auto");
    Ok(())
}

/// Mounting the panels and unmounting them.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(PANELS, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(PANEL, 1).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(PANELS, 0).await?;
    // Tabs and panels reference each other: mounted again, the page is valid at the end.
    page.control("mount").await?;
    page.element(PANEL).await?;
    Ok(())
}

/// A selection change and back: the panels measure the new panel.
#[browser_test]
pub async fn change_the_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element(PANEL).await?;
    change_the_selection_twice(page).await
}

/// A selection change and back with a size transition (of zero duration): the panels set their
/// old size, then the new one, and `auto` once the transition ended.
#[browser_test]
pub async fn change_the_selection_with_a_transition(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(PANELS, 0).await?;
    page.control("use-transition").await?;
    page.control("mount").await?;
    page.element(PANEL).await?;
    change_the_selection_twice(page).await
}

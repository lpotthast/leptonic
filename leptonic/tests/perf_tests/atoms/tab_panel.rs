// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `TabPanel` (`/perf/atoms/tab_panel`, the first of two tabs'
//! panel): what each thing it offers costs, measured (`Page::measure`) and checked against its
//! budget (`tests/budgets/atoms/tab_panel.json`): mounting and unmounting it, its tab's selection
//! ending and starting (hiding and showing it; force-mounted: inert and not), keyboard focus,
//! and changes of its label.
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/tab_panel";

/// Registers the performance tests of `TabPanel`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(HideAndShow))
        .with(fixture_test(HideAndShowForceMounted))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(ChangeTheLabel))
}
const PANEL: &str = "#perf-subject > .leptonic-TabPanel";

/// Mounting the (selected) panel and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(PANEL, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_attr(PANEL, "role", Some("tabpanel")).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(PANEL, 0).await?;
    // Tabs and panels reference each other: mounted again, the page is valid at the end.
    page.control("mount").await?;
    page.element(PANEL).await?;
    Ok(())
}

/// Selecting the other tab (the panel leaves) and its tab again (the panel comes back).
#[browser_test]
pub async fn hide_and_show(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element(PANEL).await?;
    page.measure("hide", page.control("select-second")).await?;
    page.wait_for_count(PANEL, 0).await?;
    page.measure("show", page.control("select-first")).await?;
    page.wait_for_attr(PANEL, "role", Some("tabpanel")).await?;
    Ok(())
}

/// A force-mounted panel: selecting the other tab (it stays, inert) and its tab again.
#[browser_test]
pub async fn hide_and_show_force_mounted(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(PANEL, 0).await?;
    page.control("use-force-mount").await?;
    page.control("mount").await?;
    page.wait_for_attr(PANEL, "role", Some("tabpanel")).await?;
    page.measure("hide", page.control("select-second")).await?;
    page.wait_for_attr(PANEL, "inert", Some("")).await?;
    page.wait_for_attr(PANEL, "role", None).await?;
    page.measure("show", page.control("select-first")).await?;
    page.wait_for_attr(PANEL, "inert", None).await?;
    page.wait_for_attr(PANEL, "role", Some("tabpanel")).await?;
    Ok(())
}

/// Tab moving focus from the selected tab to the panel (a tab stop, with a focus ring) and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let tab = page.element("#perf-tabs [role=tab]:first-child").await?;
    page.wait_for_focus(&tab).await?;
    let panel = page.element(PANEL).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    panel
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    panel.wait_for_attr("data-focus-visible", None).await?;
    Ok(())
}

/// Labeling the panel and removing the label again (labelled by its tab).
#[browser_test]
pub async fn change_the_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let panel = page.element(PANEL).await?;
    page.measure("set", page.control("set-label")).await?;
    panel.wait_for_attr("aria-label", Some("First")).await?;
    page.measure("unset", page.control("unset-label")).await?;
    panel.wait_for_attr("aria-label", None).await?;
    Ok(())
}

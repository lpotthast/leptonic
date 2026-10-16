// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_tab_panel` (`/perf/hooks/use_tab_panel`, on a plain `<div>`
//! after two tabs, showing the selected tab): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget (`tests/budgets/hooks/use_tab_panel.json`):
//! mounting and unmounting it, following a selection change (its id and labelling), keyboard
//! focus, changes of its label, and a tabbable child coming and going (the panel leaving the
//! tab order and joining it again).
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_tab_panel";

/// Registers the performance tests of `use_tab_panel`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(FollowTheSelection))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(ChangeTheLabel))
        .with(fixture_test(TabbableChild))
}
const PANEL: &str = ".perf-panel";

/// Mounting the panel and unmounting it.
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

/// Selecting the second tab and the first again: the panel follows (labelled by the tab).
#[browser_test]
pub async fn follow_the_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let panel = page.element(PANEL).await?;
    let tabs = page.elements("[role=tab]").await?;
    let first_id = tabs[0].attr("id").await?.unwrap_or_default();
    let second_id = tabs[1].attr("id").await?.unwrap_or_default();
    page.measure("select the second", page.control("select-second"))
        .await?;
    panel
        .wait_for_attr("aria-labelledby", Some(&second_id))
        .await?;
    page.measure("select the first", page.control("select-first"))
        .await?;
    panel
        .wait_for_attr("aria-labelledby", Some(&first_id))
        .await?;
    Ok(())
}

/// Tab moving focus from the selected tab to the panel (a tab stop) and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let panel = page.element(PANEL).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&panel).await?;
    let after = page.element("#perf-after").await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&after).await?;
    Ok(())
}

/// Labeling the panel and removing the label again.
#[browser_test]
pub async fn change_the_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let panel = page.element(PANEL).await?;
    page.measure("set", page.control("set-label")).await?;
    panel.wait_for_attr("aria-label", Some("Details")).await?;
    page.measure("unset", page.control("unset-label")).await?;
    panel.wait_for_attr("aria-label", None).await?;
    Ok(())
}

/// A button coming into the panel (it leaves the tab order) and going again (a tab stop again).
#[browser_test]
pub async fn tabbable_child(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let panel = page.element(PANEL).await?;
    panel.wait_for_attr("tabindex", Some("0")).await?;
    page.measure("add", page.control("add-button")).await?;
    panel.wait_for_attr("tabindex", None).await?;
    page.measure("remove", page.control("remove-button"))
        .await?;
    panel.wait_for_attr("tabindex", Some("0")).await?;
    Ok(())
}

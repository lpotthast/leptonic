// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ToggleButtonGroup` (`/perf/atoms/toggle_button_group`, three
//! buttons): what each thing it offers costs, measured (`Page::measure`) and checked against its
//! budget (`tests/budgets/atoms/toggle_button_group.json`): mounting and unmounting it, selecting
//! a button and another one (single selection), the arrow keys moving the focus, Tab leaving it,
//! disabling it, its orientation, and a controlled multiple-selection group's selection by clicks
//! and from outside.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/toggle_button_group";

/// Registers the performance tests of `ToggleButtonGroup`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(Select))
        .with(fixture_test(ArrowKeys))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(Orientation))
        .with(fixture_test(MultipleSelection))
}
const GROUP: &str = "#perf-group";

/// Mounting the group with its buttons and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(GROUP, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element("#perf-c").await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(GROUP, 0).await?;
    Ok(())
}

/// A click selecting the first button, and one selecting the second instead.
#[browser_test]
pub async fn select(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let a = page.element("#perf-a").await?;
    let b = page.element("#perf-b").await?;
    page.measure("select", a.click()).await?;
    a.wait_for_attr("aria-checked", Some("true")).await?;
    page.measure("select another", b.click()).await?;
    b.wait_for_attr("aria-checked", Some("true")).await?;
    a.wait_for_attr("aria-checked", Some("false")).await?;
    Ok(())
}

/// Tab into the group, ArrowRight and ArrowLeft moving the focus, and Tab leaving it.
#[browser_test]
pub async fn arrow_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let a = page.element("#perf-a").await?;
    page.measure("tab in", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&a).await?;
    page.measure("arrow right", page.send_keys(Key::Right))
        .await?;
    page.wait_for_focus(&page.element("#perf-b").await?).await?;
    page.measure("arrow left", page.send_keys(Key::Left))
        .await?;
    page.wait_for_focus(&a).await?;
    page.measure("tab out", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&page.element("#perf-after").await?)
        .await?;
    Ok(())
}

/// Disabling the group, a click it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let group = page.element(GROUP).await?;
    let a = page.element("#perf-a").await?;
    page.measure("disable", page.control("disable")).await?;
    group.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("ignored click", a.click()).await?;
    assert_that!(a.attr("aria-checked").await?).is_equal_to(Some("false".to_owned()));
    page.measure("enable", page.control("enable")).await?;
    group.wait_for_attr("data-disabled", None).await?;
    Ok(())
}

/// Making the group vertical and horizontal again.
#[browser_test]
pub async fn orientation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let group = page.element(GROUP).await?;
    page.measure("vertical", page.control("vertical")).await?;
    group
        .wait_for_attr("data-orientation", Some("vertical"))
        .await?;
    page.measure("horizontal", page.control("horizontal"))
        .await?;
    group
        .wait_for_attr("data-orientation", Some("horizontal"))
        .await?;
    Ok(())
}

/// The controlled multiple-selection group: two clicks selecting two buttons, then all selected
/// and the selection cleared from outside.
#[browser_test]
pub async fn multiple_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let a = page.element("#perf-multiple-a").await?;
    let b = page.element("#perf-multiple-b").await?;
    let c = page.element("#perf-multiple-c").await?;
    page.measure("select", a.click()).await?;
    a.wait_for_attr("aria-pressed", Some("true")).await?;
    page.measure("select another", b.click()).await?;
    b.wait_for_attr("aria-pressed", Some("true")).await?;
    a.wait_for_attr("aria-pressed", Some("true")).await?;
    page.measure("select all", page.control("select-all"))
        .await?;
    c.wait_for_attr("aria-pressed", Some("true")).await?;
    page.measure("clear", page.control("clear")).await?;
    a.wait_for_attr("aria-pressed", Some("false")).await?;
    c.wait_for_attr("aria-pressed", Some("false")).await?;
    Ok(())
}

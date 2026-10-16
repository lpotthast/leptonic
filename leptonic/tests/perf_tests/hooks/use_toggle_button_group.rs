// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_toggle_button_group` (`/perf/hooks/use_toggle_button_group`,
//! a `radiogroup` of three buttons): what each thing it offers costs, measured (`Page::measure`)
//! and checked against its budget (`tests/budgets/hooks/use_toggle_button_group.json`): mounting
//! and unmounting it with its buttons, the arrow keys moving the focus among them (its toolbar),
//! Tab leaving it, disabling it, and its orientation. Selecting its buttons is
//! `use_toggle_button_group_item`'s (measured there).
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_toggle_button_group";

/// Registers the performance tests of `use_toggle_button_group`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(ArrowKeys))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(Orientation))
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

/// Disabling the group (its buttons too) and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let group = page.element(GROUP).await?;
    let a = page.element("#perf-a").await?;
    page.measure("disable", page.control("disable")).await?;
    group.wait_for_attr("aria-disabled", Some("true")).await?;
    a.wait_for_attr("disabled", Some("true")).await?;
    page.measure("enable", page.control("enable")).await?;
    group.wait_for_attr("aria-disabled", None).await?;
    a.wait_for_attr("disabled", None).await?;
    Ok(())
}

/// Making the group vertical and horizontal again.
#[browser_test]
pub async fn orientation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let group = page.element(GROUP).await?;
    page.measure("vertical", page.control("vertical")).await?;
    group
        .wait_for_attr("aria-orientation", Some("vertical"))
        .await?;
    page.measure("horizontal", page.control("horizontal"))
        .await?;
    group
        .wait_for_attr("aria-orientation", Some("horizontal"))
        .await?;
    Ok(())
}

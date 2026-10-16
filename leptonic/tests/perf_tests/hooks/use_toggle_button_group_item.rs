// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_toggle_button_group_item`
//! (`/perf/hooks/use_toggle_button_group_item`, three radios of a single-selection group): what
//! each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_toggle_button_group_item.json`): mounting and unmounting the group
//! with its buttons, selecting a button with the mouse and another with the keyboard, deselecting
//! it, and disabling a button (a click it ignores).
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_toggle_button_group_item";

/// Registers the performance tests of `use_toggle_button_group_item`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(SelectWithTheMouse))
        .with(fixture_test(SelectWithTheKeyboard))
        .with(fixture_test(DisableAndEnable))
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

/// A click selecting the first button, one selecting the second instead, and one deselecting
/// it.
#[browser_test]
pub async fn select_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let a = page.element("#perf-a").await?;
    let b = page.element("#perf-b").await?;
    page.measure("select", a.click()).await?;
    a.wait_for_attr("aria-checked", Some("true")).await?;
    page.measure("select another", b.click()).await?;
    b.wait_for_attr("aria-checked", Some("true")).await?;
    a.wait_for_attr("aria-checked", Some("false")).await?;
    page.measure("deselect", b.click()).await?;
    b.wait_for_attr("aria-checked", Some("false")).await?;
    Ok(())
}

/// Space selecting the focused button, and Enter deselecting it.
#[browser_test]
pub async fn select_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let a = page.element("#perf-a").await?;
    page.wait_for_focus(&a).await?;
    page.measure("space", page.send_keys(" ")).await?;
    a.wait_for_attr("aria-checked", Some("true")).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    a.wait_for_attr("aria-checked", Some("false")).await?;
    Ok(())
}

/// Disabling a button, a click it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let b = page.element("#perf-b").await?;
    page.measure("disable", page.control("disable-b")).await?;
    b.wait_for_attr("disabled", Some("true")).await?;
    page.measure("ignored click", b.click()).await?;
    b.wait_for_attr("aria-checked", Some("false")).await?;
    page.measure("enable", page.control("enable-b")).await?;
    b.wait_for_attr("disabled", None).await?;
    Ok(())
}

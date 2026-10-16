// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_toggle_button` (`/perf/hooks/use_toggle_button`, on a plain
//! `<button>` through `use_button`): what each thing it offers costs, measured (`Page::measure`)
//! and checked against its budget (`tests/budgets/hooks/use_toggle_button.json`): mounting and
//! unmounting it, selecting and deselecting it with the mouse and the keyboard, its state changing
//! from outside, and disabling it (a click it ignores). Its hover, focus and pressed state are
//! `use_button`'s (measured there).
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_toggle_button";

/// Registers the performance tests of `use_toggle_button`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(ToggleWithTheMouse))
        .with(fixture_test(ToggleWithTheKeyboard))
        .with(fixture_test(ToggleFromOutside))
        .with(fixture_test(DisableAndEnable))
}
const TOGGLE: &str = "#perf-toggle";

/// Mounting the toggle button and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(TOGGLE, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(TOGGLE).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TOGGLE, 0).await?;
    Ok(())
}

/// A click selecting it, and one deselecting it.
#[browser_test]
pub async fn toggle_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let toggle = page.element(TOGGLE).await?;
    page.measure("select", toggle.click()).await?;
    toggle.wait_for_attr("aria-pressed", Some("true")).await?;
    page.measure("deselect", toggle.click()).await?;
    toggle.wait_for_attr("aria-pressed", Some("false")).await?;
    Ok(())
}

/// Space selecting it and Enter deselecting it, focused.
#[browser_test]
pub async fn toggle_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let toggle = page.element(TOGGLE).await?;
    page.wait_for_focus(&toggle).await?;
    page.measure("space", page.send_keys(" ")).await?;
    toggle.wait_for_attr("aria-pressed", Some("true")).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    toggle.wait_for_attr("aria-pressed", Some("false")).await?;
    Ok(())
}

/// Its state toggled from outside, twice.
#[browser_test]
pub async fn toggle_from_outside(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let toggle = page.element(TOGGLE).await?;
    page.measure("select", page.control("toggle")).await?;
    toggle.wait_for_attr("aria-pressed", Some("true")).await?;
    page.measure("deselect", page.control("toggle")).await?;
    toggle.wait_for_attr("aria-pressed", Some("false")).await?;
    Ok(())
}

/// Disabling the toggle button, a click it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let toggle = page.element(TOGGLE).await?;
    page.measure("disable", page.control("disable")).await?;
    toggle.wait_for_attr("disabled", Some("true")).await?;
    page.measure("ignored click", toggle.click()).await?;
    toggle.wait_for_attr("aria-pressed", Some("false")).await?;
    page.measure("enable", page.control("enable")).await?;
    toggle.wait_for_attr("disabled", None).await?;
    Ok(())
}

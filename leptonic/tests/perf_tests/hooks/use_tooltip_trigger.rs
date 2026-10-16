// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_tooltip_trigger` (`/perf/hooks/use_tooltip_trigger`): what each
//! thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_tooltip_trigger.json`): mounting and unmounting it, the pointer
//! entering (opening) and leaving (closing), keyboard focus (opening) and blur (closing), Escape,
//! a key press and a pointer press closing, a click (opening, closing, and focus by the pointer
//! opening nothing) and hovering while disabled (ignored). Without delays, each step ends with the
//! tooltip open or closed, no timer pending (one opening per test: the warm-up's cooldown after a
//! close is a timer).
use std::time::Duration;

use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_tooltip_trigger";
const TOOLTIP: &str = ".perf-tooltip";
const TRIGGER: &str = "#perf-trigger";

/// Registers the performance tests of `use_tooltip_trigger`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(FocusAndBlur))
        .with(fixture_test(EscapeCloses))
        .with(fixture_test(KeyPressCloses))
        .with(fixture_test(PointerPressCloses))
        .with(fixture_test(Click))
        .with(fixture_test(IgnoredWhileDisabled))
}

/// Waits until the tooltip is open and describes the trigger.
async fn wait_for_open(page: &Page<'_>) -> Result<(), Report> {
    let tooltip = page.element(TOOLTIP).await?;
    let id = tooltip.attr("id").await?;
    page.element(TRIGGER)
        .await?
        .wait_for_attr("aria-describedby", id.as_deref())
        .await
}

/// Waits until the tooltip is closed (gone, the trigger no longer described by it).
async fn wait_for_closed(page: &Page<'_>) -> Result<(), Report> {
    page.wait_for_count(TOOLTIP, 0).await?;
    page.element(TRIGGER)
        .await?
        .wait_for_attr("aria-describedby", None)
        .await
}

/// Opens the tooltip by keyboard focus (Tab from the plain button before the trigger).
async fn open_by_focus(page: &Page<'_>) -> Result<(), Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(TRIGGER).await?).await?;
    wait_for_open(page).await
}

/// Opens the tooltip by hovering the trigger (the pointer over the page first: hovering counts
/// with the pointer's modality).
async fn open_by_hover(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#perf-after").await?.hover().await?;
    let trigger = page.element(TRIGGER).await?;
    trigger.hover().await?;
    wait_for_open(page).await?;
    Ok(trigger)
}

/// Mounting the trigger (closed) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(TRIGGER, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    wait_for_closed(page).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TRIGGER, 0).await?;
    Ok(())
}

/// The pointer entering the trigger (opening the tooltip) and leaving it (closing it).
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let after = page.element("#perf-after").await?;
    after.hover().await?;
    let trigger = page.element(TRIGGER).await?;
    page.measure("hover", trigger.hover()).await?;
    wait_for_open(page).await?;
    page.measure("leave", after.hover()).await?;
    wait_for_closed(page).await?;
    Ok(())
}

/// Tab moving focus to the trigger (opening the tooltip) and on (closing it).
#[browser_test]
pub async fn focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    wait_for_open(page).await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    wait_for_closed(page).await?;
    Ok(())
}

/// Escape closing the tooltip opened by keyboard focus.
#[browser_test]
pub async fn escape_closes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open_by_focus(page).await?;
    page.measure("escape", page.send_keys(Key::Escape)).await?;
    wait_for_closed(page).await?;
    Ok(())
}

/// A key press on the focused trigger (Enter) closing the tooltip opened by keyboard focus.
#[browser_test]
pub async fn key_press_closes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open_by_focus(page).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    wait_for_closed(page).await?;
    Ok(())
}

/// A pointer press (holding the pointer down) on the trigger closing the tooltip opened by
/// hovering.
#[browser_test]
pub async fn pointer_press_closes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = open_by_hover(page).await?;
    let held = page.measure("press", trigger.press_and_hold()).await?;
    wait_for_closed(page).await?;
    held.release().await?;
    Ok(())
}

/// A click on the trigger: the pointer entering opens the tooltip, the press closes it, and the
/// focus the click gives the trigger (the pointer's modality) opens nothing.
#[browser_test]
pub async fn click(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-after").await?.hover().await?;
    let trigger = page.element(TRIGGER).await?;
    page.measure("click", trigger.click()).await?;
    page.wait_for_focus(&trigger).await?;
    wait_for_closed(page).await?;
    Ok(())
}

/// The pointer entering the trigger while it is disabled: no tooltip.
#[browser_test]
pub async fn ignored_while_disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-after").await?.hover().await?;
    page.control("disable").await?;
    let trigger = page.element(TRIGGER).await?;
    page.measure("ignored hover", trigger.hover()).await?;
    page.count_stays(TOOLTIP, 0, Duration::from_millis(100))
        .await?;
    Ok(())
}

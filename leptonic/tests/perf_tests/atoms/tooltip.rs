// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `Tooltip` (`/perf/atoms/tooltip`): what each thing it offers costs,
//! measured (`Page::measure`) and checked against its budget (`tests/budgets/atoms/tooltip.json`):
//! mounting and unmounting it, opening (rendered and positioned) and closing by the app's state,
//! by hovering and by keyboard focus, Escape, pressing the trigger and scrolling closing it, and
//! the pointer leaving the hovered tooltip. Its trigger has no delays: each step ends with the
//! tooltip open or closed, no timer pending (one opening per test: the warm-up's cooldown after
//! a close is a timer).
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/tooltip";
const TOOLTIP: &str = "[role=tooltip]";
const TRIGGER: &str = "#perf-trigger";

/// Registers the performance tests of `Tooltip`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(OpenAndClose))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(FocusAndBlur))
        .with(fixture_test(EscapeCloses))
        .with(fixture_test(PressCloses))
        .with(fixture_test(ScrollingCloses))
        .with(fixture_test(LeavingTheTooltip))
}

/// Waits until the tooltip is open, placed above the trigger (and done entering), describing it.
async fn wait_for_open(page: &Page<'_>) -> Result<WebElement, Report> {
    let tooltip = page.element(TOOLTIP).await?;
    tooltip.wait_for_attr("data-placement", Some("top")).await?;
    tooltip.wait_for_attr("data-entering", None).await?;
    let id = tooltip.attr("id").await?;
    page.element(TRIGGER)
        .await?
        .wait_for_attr("aria-describedby", id.as_deref())
        .await?;
    Ok(tooltip)
}

/// Waits until the tooltip is closed (gone, the trigger no longer described by it).
async fn wait_for_closed(page: &Page<'_>) -> Result<(), Report> {
    page.wait_for_count(TOOLTIP, 0).await?;
    page.element(TRIGGER)
        .await?
        .wait_for_attr("aria-describedby", None)
        .await
}

/// Focuses the trigger with the keyboard (Tab from the plain button before it), which opens the
/// tooltip.
async fn focus_with_tab(page: &Page<'_>) -> Result<(), Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(TRIGGER).await?).await
}

/// Mounting the (closed) tooltip and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.measure("mount", page.control("mount")).await?;
    page.control("open").await?;
    wait_for_open(page).await?;
    page.control("close").await?;
    wait_for_closed(page).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TOOLTIP, 0).await?;
    Ok(())
}

/// The app opening the tooltip (rendered and positioned) and closing it.
#[browser_test]
pub async fn open_and_close(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("open", page.control("open")).await?;
    wait_for_open(page).await?;
    page.measure("close", page.control("close")).await?;
    wait_for_closed(page).await?;
    Ok(())
}

/// The pointer entering the trigger (opening the tooltip) and leaving it (closing it).
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    // Hovering counts with the pointer's modality: the pointer moves over the page first.
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
    focus_with_tab(page).await?;
    wait_for_open(page).await?;
    page.measure("escape", page.send_keys(Key::Escape)).await?;
    wait_for_closed(page).await?;
    Ok(())
}

/// Clicking the trigger closing the tooltip opened by hovering.
#[browser_test]
pub async fn press_closes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-after").await?.hover().await?;
    let trigger = page.element(TRIGGER).await?;
    trigger.hover().await?;
    wait_for_open(page).await?;
    page.measure("press", trigger.click()).await?;
    wait_for_closed(page).await?;
    Ok(())
}

/// Scrolling the container of the trigger closing the open tooltip.
#[browser_test]
pub async fn scrolling_closes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("open").await?;
    wait_for_open(page).await?;
    let scroller = page.element("#perf-scroller").await?;
    page.measure("scroll", scroller.scroll_to_top(20.0)).await?;
    wait_for_closed(page).await?;
    Ok(())
}

/// The pointer entering the tooltip opened by keyboard focus (keeping it open) and leaving it
/// (closing it).
#[browser_test]
pub async fn leaving_the_tooltip(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_with_tab(page).await?;
    let tooltip = wait_for_open(page).await?;
    page.measure("hover", tooltip.hover()).await?;
    tooltip.wait_for_attr("data-placement", Some("top")).await?;
    page.measure("leave", page.element("#perf-after").await?.hover())
        .await?;
    wait_for_closed(page).await?;
    Ok(())
}

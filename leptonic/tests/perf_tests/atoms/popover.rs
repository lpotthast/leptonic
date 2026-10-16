// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `Popover` (`/perf/atoms/popover`): what each thing it offers costs,
//! measured (`Page::measure`) and checked against its budget (`tests/budgets/atoms/popover.json`):
//! mounting and unmounting it, opening (rendered, positioned, the page outside made inert, the
//! popover focused) and closing it by the app's state and by its trigger, Escape and a click
//! outside closing it (the focus returning to the trigger), Tab wrapping inside it, its placement
//! and offset changing while open, and a non-modal popover closing when its trigger's container
//! scrolls.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/popover";
const POPOVER: &str = ".leptonic-Popover";
const TRIGGER: &str = "#perf-trigger";

/// Registers the performance tests of `Popover`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(OpenAndClose))
        .with(fixture_test(OpenWithTheTrigger))
        .with(fixture_test(EscapeCloses))
        .with(fixture_test(OutsideClickCloses))
        .with(fixture_test(TabWraps))
        .with(fixture_test(PlacementChanges))
        .with(fixture_test(OffsetChanges))
        .with(fixture_test(ScrollingClosesNonModal))
}

/// Waits until the popover is open: placed on `side` of the trigger, done entering, focused.
async fn wait_for_open(page: &Page<'_>, side: &str) -> Result<WebElement, Report> {
    let popover = page.element(POPOVER).await?;
    popover.wait_for_attr("data-placement", Some(side)).await?;
    popover.wait_for_attr("data-entering", None).await?;
    page.wait_for_focus(&popover).await?;
    page.element(TRIGGER)
        .await?
        .wait_for_attr("aria-expanded", Some("true"))
        .await?;
    Ok(popover)
}

/// Waits until the popover is closed and the trigger has the focus back.
async fn wait_for_closed(page: &Page<'_>) -> Result<(), Report> {
    page.wait_for_count(POPOVER, 0).await?;
    let trigger = page.element(TRIGGER).await?;
    trigger
        .wait_for_attr("aria-expanded", Some("false"))
        .await?;
    page.wait_for_focus(&trigger).await
}

/// Focuses the trigger (a click on the plain button before it, then Tab).
async fn focus_trigger(page: &Page<'_>) -> Result<(), Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(TRIGGER).await?).await
}

/// Opens the popover by the app's state, the trigger focused before.
async fn open(page: &Page<'_>) -> Result<WebElement, Report> {
    focus_trigger(page).await?;
    page.control("open").await?;
    wait_for_open(page, "bottom").await
}

/// Mounting the (closed) popover and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.measure("mount", page.control("mount")).await?;
    focus_trigger(page).await?;
    page.control("open").await?;
    wait_for_open(page, "bottom").await?;
    page.control("close").await?;
    wait_for_closed(page).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(POPOVER, 0).await?;
    Ok(())
}

/// The app opening the popover and closing it.
#[browser_test]
pub async fn open_and_close(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_trigger(page).await?;
    page.measure("open", page.control("open")).await?;
    wait_for_open(page, "bottom").await?;
    page.measure("close", page.control("close")).await?;
    wait_for_closed(page).await?;
    Ok(())
}

/// Enter on the focused trigger opening the popover.
#[browser_test]
pub async fn open_with_the_trigger(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_trigger(page).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    wait_for_open(page, "bottom").await?;
    Ok(())
}

/// Escape closing the popover.
#[browser_test]
pub async fn escape_closes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open(page).await?;
    page.measure("escape", page.send_keys(Key::Escape)).await?;
    wait_for_closed(page).await?;
    Ok(())
}

/// A click outside the popover closing it.
#[browser_test]
pub async fn outside_click_closes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open(page).await?;
    page.measure("outside click", page.click_at(5, 5)).await?;
    page.wait_for_count(POPOVER, 0).await?;
    page.element(TRIGGER)
        .await?
        .wait_for_attr("aria-expanded", Some("false"))
        .await?;
    Ok(())
}

/// Tab from the last button wrapping around to the first (the modal popover contains the focus),
/// and Shift+Tab back.
#[browser_test]
pub async fn tab_wraps(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open(page).await?;
    let first = page.element("#perf-first").await?;
    let last = page.element("#perf-last").await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&first).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&last).await?;
    page.measure("tab", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&first).await?;
    page.measure("shift tab", page.send_keys(Key::Shift + Key::Tab))
        .await?;
    page.wait_for_focus(&last).await?;
    Ok(())
}

/// The open popover moving above the trigger and back below it (`placement`).
#[browser_test]
pub async fn placement_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let popover = open(page).await?;
    page.measure("place above", page.control("place-above"))
        .await?;
    popover.wait_for_attr("data-placement", Some("top")).await?;
    page.measure("place below", page.control("place-below"))
        .await?;
    popover
        .wait_for_attr("data-placement", Some("bottom"))
        .await?;
    Ok(())
}

/// The open popover's distance from the trigger growing and shrinking (`offset`).
#[browser_test]
pub async fn offset_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let popover = open(page).await?;
    let trigger = page.element(TRIGGER).await?;
    let gap = async || -> Result<f64, Report> {
        Ok(popover.client_rect().await?.top - trigger.client_rect().await?.bottom)
    };
    let near = gap().await?;
    page.measure("offset far", page.control("offset-far"))
        .await?;
    assert_gap(gap().await?, near + 16.0);
    page.measure("offset near", page.control("offset-near"))
        .await?;
    assert_gap(gap().await?, near);
    Ok(())
}

/// Asserts that the popover's distance from the trigger is `expected` (to a pixel).
fn assert_gap(gap: f64, expected: f64) {
    assert_that!(gap).is_close_to(expected, 1.0);
}

/// Scrolling the trigger's container closing the open non-modal popover.
#[browser_test]
pub async fn scrolling_closes_non_modal(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.control("mount-non-modal").await?;
    focus_trigger(page).await?;
    page.control("open").await?;
    let popover = page.element(POPOVER).await?;
    popover
        .wait_for_attr("data-placement", Some("bottom"))
        .await?;
    popover.wait_for_attr("data-entering", None).await?;
    let scroller = page.element("#perf-scroller").await?;
    page.measure("scroll", scroller.scroll_to_top(20.0)).await?;
    page.wait_for_count(POPOVER, 0).await?;
    Ok(())
}

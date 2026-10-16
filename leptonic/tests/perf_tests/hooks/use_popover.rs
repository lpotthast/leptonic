// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_popover` (`/perf/hooks/use_popover`): what each thing it offers
//! costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_popover.json`): mounting and unmounting it, opening (positioned, the
//! page outside made inert, its scrolling prevented) and closing by the app's state, Escape, a
//! click outside and the focus leaving (non-modal) closing it, its placement changing while open,
//! and a non-modal popover closing when its trigger's container scrolls.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_popover";
const POPOVER: &str = ".perf-popover";
const TRIGGER: &str = "#perf-trigger";

/// Registers the performance tests of `use_popover`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(OpenAndClose))
        .with(fixture_test(EscapeCloses))
        .with(fixture_test(OutsideClickCloses))
        .with(fixture_test(BlurCloses))
        .with(fixture_test(PlacementChanges))
        .with(fixture_test(ScrollingClosesNonModal))
}

/// Waits until the popover is open, placed below the trigger, its button focused.
async fn wait_for_open(page: &Page<'_>) -> Result<(), Report> {
    page.element(POPOVER)
        .await?
        .wait_for_attr("data-placement", Some("bottom"))
        .await?;
    page.wait_for_focus(&page.element("#perf-popover-button").await?)
        .await
}

/// Opens the popover (the trigger focused before) and waits until it is open.
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.element(TRIGGER).await?.click().await?;
    page.control("open").await?;
    wait_for_open(page).await
}

/// Mounting the hook (closed) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(TRIGGER, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(TRIGGER).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TRIGGER, 0).await?;
    Ok(())
}

/// The app opening the popover (the page outside made inert) and closing it.
#[browser_test]
pub async fn open_and_close(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element(TRIGGER).await?.click().await?;
    page.measure("open", page.control("open")).await?;
    wait_for_open(page).await?;
    let before = page.element("#perf-before").await?;
    assert_that!(before.is_within("[inert]").await?).is_true();
    page.measure("close", page.control("close")).await?;
    page.wait_for_count(POPOVER, 0).await?;
    assert_that!(before.is_within("[inert]").await?).is_false();
    Ok(())
}

/// Escape (in the popover) closing it.
#[browser_test]
pub async fn escape_closes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open(page).await?;
    page.measure("escape", page.send_keys(Key::Escape)).await?;
    page.wait_for_count(POPOVER, 0).await?;
    Ok(())
}

/// A click outside the popover closing it.
#[browser_test]
pub async fn outside_click_closes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open(page).await?;
    page.measure("outside click", page.click_at(5, 5)).await?;
    page.wait_for_count(POPOVER, 0).await?;
    Ok(())
}

/// The focus leaving the non-modal popover (Tab, out of its last button) closing it (a modal one
/// makes the page outside inert: there is nowhere to tab to).
#[browser_test]
pub async fn blur_closes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.control("mount-non-modal").await?;
    open(page).await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    page.wait_for_count(POPOVER, 0).await?;
    Ok(())
}

/// The open popover moving above the trigger and back below it.
#[browser_test]
pub async fn placement_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open(page).await?;
    let popover = page.element(POPOVER).await?;
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

/// Scrolling the trigger's container closing the open non-modal popover.
#[browser_test]
pub async fn scrolling_closes_non_modal(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.control("mount-non-modal").await?;
    open(page).await?;
    let scroller = page.element("#perf-scroller").await?;
    page.measure("scroll", scroller.scroll_to_top(20.0)).await?;
    page.wait_for_count(POPOVER, 0).await?;
    Ok(())
}

// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `SelectPopover` (`/perf/atoms/select_popover`, in a `Select` with a
//! trigger): what each thing it offers costs, measured (`Page::measure`) and checked against its
//! budget (`tests/budgets/atoms/select_popover.json`): mounting and unmounting it while the select
//! is open, opening and closing (it renders while open), with 10 options and with 1,000, and its
//! dismissal with Escape and with a click outside.
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/atoms/select_popover";

/// Registers the performance tests of `SelectPopover`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(OpenAndClose))
        .with(fixture_test(OpenAndCloseLarge))
        .with(fixture_test(Dismiss))
}
const TRIGGER: &str = ".leptonic-SelectTrigger";
const POPOVER: &str = ".leptonic-SelectPopover";
/// The fixture's options after the control `large`.
const LARGE: u32 = 1_000;

/// The option "Option `i`".
fn option(i: u32) -> String {
    format!("[data-perf-option='{i}']")
}

/// Switches to [`LARGE`] options (unmeasured).
async fn large(page: &Page<'_>) -> Result<(), Report> {
    page.control("large").await?;
    page.settle().await
}

/// Mounting the popover of the open select (`last + 1` options) and unmounting it.
async fn mount_and_unmount_in(page: &Page<'_>, last: u32) -> Result<(), Report> {
    page.control("open").await?;
    page.element(option(last)).await?;
    page.control("unmount").await?;
    page.wait_for_count(POPOVER, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(option(last)).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(POPOVER, 0).await?;
    // Closed again with the popover (the trigger refers to it while open).
    page.control("mount").await?;
    page.control("close").await?;
    page.wait_for_count(POPOVER, 0).await?;
    Ok(())
}

/// Mounting the popover of 10 options and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_and_unmount_in(page, 9).await
}

/// Mounting the popover of 1,000 options and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    mount_and_unmount_in(page, LARGE - 1).await
}

/// The controlled open state opening the popover of `last + 1` options and closing it.
async fn open_and_close_in(page: &Page<'_>, last: u32) -> Result<(), Report> {
    page.measure("open", page.control("open")).await?;
    page.element(option(last)).await?;
    page.measure("close", page.control("close")).await?;
    page.wait_for_count(POPOVER, 0).await?;
    Ok(())
}

/// Opening and closing the popover of 10 options.
#[browser_test]
pub async fn open_and_close(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open_and_close_in(page, 9).await
}

/// Opening and closing the popover of 1,000 options.
#[browser_test]
pub async fn open_and_close_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    open_and_close_in(page, LARGE - 1).await
}

/// Escape and a click outside closing the popover.
#[browser_test]
pub async fn dismiss(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(TRIGGER).await?;
    trigger.click().await?;
    page.element(option(0)).await?;
    page.measure("escape", page.send_keys(Key::Escape)).await?;
    page.wait_for_count(POPOVER, 0).await?;
    trigger.click().await?;
    page.element(option(0)).await?;
    page.measure("outside click", page.click_at(5, 5)).await?;
    page.wait_for_count(POPOVER, 0).await?;
    Ok(())
}

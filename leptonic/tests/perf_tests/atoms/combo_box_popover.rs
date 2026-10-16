// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ComboBoxPopover` (`/perf/atoms/combo_box_popover`, the fixture of
//! `ComboBox`'s performance test): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/atoms/combo_box_popover.json`): mounting and
//! unmounting it while the combo box is open, opening and closing it, and its options changing
//! while open (typing), with 10 options and with 1,000; its dismissal with Escape and with a click
//! outside.
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use super::combo_box::{
    BUTTON, INPUT, OPTIONS, POPOVER, goto, large, open_and_select_with_the_mouse_in, typing_in,
};
use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/atoms/combo_box_popover";

/// Registers the performance tests of `ComboBoxPopover`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(OpenAndSelect))
        .with(fixture_test(OpenAndSelectLarge))
        .with(fixture_test(Filtering))
        .with(fixture_test(FilteringLarge))
        .with(fixture_test(Dismiss))
}

/// Mounting the popover of the open combo box (`count` options) and unmounting it.
async fn mount_and_unmount_in(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.element(BUTTON).await?.click().await?;
    page.wait_for_count(OPTIONS, count).await?;
    page.control("unmount").await?;
    page.wait_for_count(POPOVER, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(OPTIONS, count).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(POPOVER, 0).await?;
    // Closed again with the popover (the input refers to it while open).
    page.control("mount").await?;
    page.wait_for_count(OPTIONS, count).await?;
    page.click_at(5, 5).await?;
    page.wait_for_count(POPOVER, 0).await?;
    Ok(())
}

/// Mounting the popover of 10 options and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    mount_and_unmount_in(page, 10).await
}

/// Mounting the popover of 1,000 options and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    mount_and_unmount_in(page, 1_000).await
}

/// Opening the popover of 10 options and closing it by selecting one.
#[browser_test]
pub async fn open_and_select(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    open_and_select_with_the_mouse_in(page, 10).await
}

/// Opening the popover of 1,000 options and closing it by selecting one.
#[browser_test]
pub async fn open_and_select_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    open_and_select_with_the_mouse_in(page, 1_000).await
}

/// The popover's options following the typed text, of 10.
#[browser_test]
pub async fn filtering(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    typing_in(page, 1, 0).await
}

/// The popover's options following the typed text, of 1,000.
#[browser_test]
pub async fn filtering_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    typing_in(page, 271, 20).await
}

/// Escape and a click outside closing the popover.
#[browser_test]
pub async fn dismiss(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    let button = page.element(BUTTON).await?;
    button.click().await?;
    page.wait_for_count(OPTIONS, 10).await?;
    page.measure("escape", page.send_keys(Key::Escape)).await?;
    page.wait_for_count(POPOVER, 0).await?;
    button.click().await?;
    page.wait_for_count(OPTIONS, 10).await?;
    page.measure("outside click", page.click_at(5, 5)).await?;
    page.wait_for_count(POPOVER, 0).await?;
    page.element(INPUT).await?;
    Ok(())
}

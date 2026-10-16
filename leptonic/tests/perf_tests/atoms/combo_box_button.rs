// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ComboBoxButton` (`/perf/atoms/combo_box_button`, the fixture of
//! `ComboBox`'s performance test): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/atoms/combo_box_button.json`): mounting and
//! unmounting it in its combo box, clicks opening the popover (10 and 1,000 options) and closing
//! it, its pressed and hovered states, and the combo box's disabled state changing.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use super::combo_box::{BUTTON, OPTIONS, POPOVER, goto, large};
use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/combo_box_button";

/// Registers the performance tests of `ComboBoxButton`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(OpenAndClose))
        .with(fixture_test(OpenAndCloseLarge))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(DisableAndEnable))
}

/// Mounting the button in its combo box and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(BUTTON, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(BUTTON).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(BUTTON, 0).await?;
    Ok(())
}

/// A click opening the popover with `count` options and one closing it.
async fn open_and_close_in(page: &Page<'_>, count: usize) -> Result<(), Report> {
    let button = page.element(BUTTON).await?;
    page.measure("open", button.click()).await?;
    page.wait_for_count(OPTIONS, count).await?;
    button.wait_for_attr("data-open", Some("true")).await?;
    page.measure("close", button.click()).await?;
    page.wait_for_count(POPOVER, 0).await?;
    button.wait_for_attr("data-open", None).await?;
    Ok(())
}

/// Opening and closing the popover of 10 options with clicks.
#[browser_test]
pub async fn open_and_close(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    open_and_close_in(page, 10).await
}

/// Opening and closing the popover of 1,000 options with clicks.
#[browser_test]
pub async fn open_and_close_large(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    large(page).await?;
    open_and_close_in(page, 1_000).await
}

/// Holding the pointer down on the button (its pressed state; the popover opens) and releasing
/// it.
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    let button = page.element(BUTTON).await?;
    let held = page.measure("hold", button.press_and_hold()).await?;
    button.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("release", held.release()).await?;
    page.wait_for_count(OPTIONS, 10).await?;
    Ok(())
}

/// The pointer entering and leaving the button.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    let button = page.element(BUTTON).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", button.hover()).await?;
    button.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    button.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// The combo box disabled (the button too), a click it ignores, and enabled again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    goto(page, PATH).await?;
    let button = page.element(BUTTON).await?;
    page.measure("disable", page.control("disable")).await?;
    button.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("ignored click", button.click()).await?;
    assert_that!(page.count(POPOVER).await?).is_equal_to(0);
    page.measure("enable", page.control("enable")).await?;
    button.wait_for_attr("data-disabled", None).await?;
    Ok(())
}

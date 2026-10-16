// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `SearchFieldClearButton` (`leptonic/src/atoms/search_field.rs`,
//! fixture `/perf/atoms/search_field_clear_button`): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/search_field_clear_button.json`): mounting and unmounting it in its
//! search field, a click clearing the field (focus moves to the input), holding and releasing
//! it, hover, and a click it ignores while the field is disabled.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/search_field_clear_button";
const BUTTON: &str = ".leptonic-SearchFieldClearButton";
const INPUT: &str = "#perf-input";

/// Registers the performance tests of `SearchFieldClearButton`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(PressWithTheMouse))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(IgnoredWhileDisabled))
}

/// How often the field cleared.
async fn clears(page: &Page<'_>) -> Result<f64, Report> {
    page.control("clears").await
}

/// Mounting the button in its field and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(BUTTON, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    let button = page.element(BUTTON).await?;
    button.wait_for_attr("tabindex", Some("-1")).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(BUTTON, 0).await?;
    Ok(())
}

/// A click: the field clears and its input takes focus.
#[browser_test]
pub async fn press_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let button = page.element(BUTTON).await?;
    let input = page.element(INPUT).await?;
    page.measure("click", button.click()).await?;
    input.wait_for_prop("value", "").await?;
    page.wait_for_focus(&input).await?;
    assert_that!(clears(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Holding the pointer down (the pressed state) and releasing it (the field clears).
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let button = page.element(BUTTON).await?;
    let input = page.element(INPUT).await?;
    let held = page.measure("hold", button.press_and_hold()).await?;
    button.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("release", held.release()).await?;
    button.wait_for_attr("data-pressed", None).await?;
    input.wait_for_prop("value", "").await?;
    assert_that!(clears(page).await?).is_equal_to(1.0);
    Ok(())
}

/// The pointer entering and leaving the button.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let button = page.element(BUTTON).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", button.hover()).await?;
    button.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    button.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Disabling the field (the button with it), a click the button ignores, and enabling it again.
#[browser_test]
pub async fn ignored_while_disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let button = page.element(BUTTON).await?;
    let input = page.element(INPUT).await?;
    page.measure("disable", page.control("disable")).await?;
    button.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("ignored click", button.click()).await?;
    input.wait_for_prop("value", "leptos").await?;
    assert_that!(clears(page).await?).is_equal_to(0.0);
    page.measure("enable", page.control("enable")).await?;
    button.wait_for_attr("data-disabled", None).await?;
    Ok(())
}

// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `NumberFieldDecrementButton`
//! (`leptonic/src/atoms/number_field.rs`, fixture `/perf/atoms/number_field_decrement_button`):
//! what each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/number_field_decrement_button.json`): mounting and unmounting it in its
//! number field, a click decrementing the value (the first focuses the input and creates the
//! live announcer), hover, and a click it ignores at the minimum.
//!
//! Holding it down steps the value down repeatedly after 400 ms, every 60 ms until released: a timer
//! no measurement can span deterministically, so the tests click it.
//!
//! A focused number field announces each new value. The first announcement creates the live
//! announcer and shows after 100 ms: the measurement of the first click waits for it, the
//! other tests click once unmeasured and wait for it first.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/number_field_decrement_button";
const BUTTON: &str = ".leptonic-NumberFieldDecrementButton";
const INPUT: &str = "#perf-input";
/// The live announcer's announcement.
const ANNOUNCEMENT: &str = "[data-live-announcer] [aria-live=assertive] div";

/// Registers the performance tests of `NumberFieldDecrementButton`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(FirstClick))
        .with(fixture_test(PressWithTheMouse))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(IgnoredAtTheMinimum))
}

/// The field's value.
async fn value(page: &Page<'_>) -> Result<f64, Report> {
    page.control("value").await
}

/// Mounting the button in its field ("Decrease Quantity") and unmounting it.
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

/// The first click: the input takes focus, the value decrements, and the live announcer is
/// created; the measurement includes its first announcement, 100 ms later.
#[browser_test]
pub async fn first_click(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let button = page.element(BUTTON).await?;
    let input = page.element(INPUT).await?;
    page.measure("click", async {
        button.click().await?;
        page.wait_for_count(ANNOUNCEMENT, 1).await
    })
    .await?;
    page.wait_for_focus(&input).await?;
    assert_that!(value(page).await?).is_equal_to(4.0);
    Ok(())
}

/// A click while the input has focus: the value decrements and is announced.
#[browser_test]
pub async fn press_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let button = page.element(BUTTON).await?;
    let input = page.element(INPUT).await?;
    button.click().await?;
    page.wait_for_focus(&input).await?;
    page.wait_for_count(ANNOUNCEMENT, 1).await?;
    page.measure("click", button.click()).await?;
    input.wait_for_prop("value", "3").await?;
    page.element(ANNOUNCEMENT)
        .await?
        .wait_for_inner_text("3")
        .await?;
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

/// The value reaching the minimum (the button disables) and a click it ignores.
#[browser_test]
pub async fn ignored_at_the_minimum(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let button = page.element(BUTTON).await?;
    page.measure("reach the minimum", page.control("set-minimum"))
        .await?;
    button.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("ignored click", button.click()).await?;
    assert_that!(value(page).await?).is_equal_to(0.0);
    Ok(())
}

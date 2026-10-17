// Upstream: @adobe/react-spectrum/test/color/ColorField.test.js @ 99e6102368
// The other cases pin `use_number_field`'s and `use_color_field`'s conditions (react-aria:
// `isWheelDisabled || isDisabled || isReadOnly || !focusWithin`, the horizontal check).
//! `use_wheel_stepping`: while focus is within the field, the wheel over its input steps the
//! value (down increments, up decrements) and doesn't scroll the page; a mostly horizontal wheel,
//! a wheel over an unfocused field, and a read-only, disabled or wheel-disabled field don't step.
use std::time::Duration;

use assertr::prelude::*;
use browser_test::browser_test;
use rootcause::Report;

use crate::pages::{ElementActions, MouseKind, Page, SyntheticEvent};

const PATH: &str = "/hooks/wheel-stepping";
const INPUT: &str = "#test-ws-input";
const COUNT: &str = "#test-ws-count";

/// Opens the fixture and focuses the input.
async fn open_focused(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    input.click().await?;
    page.wait_for_focus(&input).await?;
    Ok(())
}

/// Checks that the count is and stays `expected`.
async fn count_stays(page: &Page<'_>, selector: &str, expected: &str) -> Result<(), Report> {
    page.element(selector)
        .await?
        .inner_text_stays(expected, Duration::from_millis(100))
        .await
}

/// The wheel down over the focused input increments, up decrements, without scrolling the page
/// ("increment with mouse wheel", "decrement with mouse wheel").
#[browser_test]
pub async fn the_wheel_steps_the_focused_field(page: &Page<'_>) -> Result<(), Report> {
    open_focused(page).await?;
    let input = page.element(INPUT).await?;
    let down = input
        .dispatch(SyntheticEvent::wheel().delta_y(10.0))
        .await?;
    assert_that!(down.default_prevented).is_true();
    page.element(COUNT).await?.wait_for_inner_text("1").await?;
    input
        .dispatch(SyntheticEvent::wheel().delta_y(-10.0))
        .await?;
    input
        .dispatch(SyntheticEvent::wheel().delta_y(-10.0))
        .await?;
    page.element(COUNT).await?.wait_for_inner_text("-1").await?;
    Ok(())
}

/// Focus on another element of the field (a stepper button) lets the wheel over the input step.
#[browser_test]
pub async fn focus_within_the_field_lets_the_wheel_step(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let inner = page.element("#test-ws-inner").await?;
    inner.click().await?;
    page.wait_for_focus(&inner).await?;
    page.element(INPUT)
        .await?
        .dispatch(SyntheticEvent::wheel().delta_y(10.0))
        .await?;
    page.element(COUNT).await?.wait_for_inner_text("1").await?;
    Ok(())
}

/// A wheel over the field while focus is elsewhere scrolls the page and doesn't step.
#[browser_test]
pub async fn an_unfocused_field_ignores_the_wheel(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let wheel = page
        .element(INPUT)
        .await?
        .dispatch(SyntheticEvent::wheel().delta_y(10.0))
        .await?;
    assert_that!(wheel.default_prevented).is_false();
    count_stays(page, COUNT, "0").await
}

/// A mostly horizontal wheel (a trackpad's) doesn't step.
#[browser_test]
pub async fn a_horizontal_wheel_doesnt_step(page: &Page<'_>) -> Result<(), Report> {
    open_focused(page).await?;
    page.element(INPUT)
        .await?
        .dispatch(SyntheticEvent::wheel().delta_x(20.0).delta_y(5.0))
        .await?;
    count_stays(page, COUNT, "0").await
}

/// A read-only field doesn't step.
#[browser_test]
pub async fn a_read_only_field_doesnt_step(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-ws-read-only").await?.click().await?;
    let input = page.element(INPUT).await?;
    input.click().await?;
    page.wait_for_focus(&input).await?;
    input
        .dispatch(SyntheticEvent::wheel().delta_y(10.0))
        .await?;
    count_stays(page, COUNT, "0").await
}

/// A field disabled while focused doesn't step.
#[browser_test]
pub async fn a_disabled_field_doesnt_step(page: &Page<'_>) -> Result<(), Report> {
    open_focused(page).await?;
    // Disabled programmatically: the field keeps the focus (a press on the button would move it).
    page.element("#test-ws-disable")
        .await?
        .dispatch(SyntheticEvent::mouse(MouseKind::Click))
        .await?;
    page.element(INPUT)
        .await?
        .dispatch(SyntheticEvent::wheel().delta_y(10.0))
        .await?;
    count_stays(page, COUNT, "0").await
}

/// A field whose wheel is disabled doesn't step.
#[browser_test]
pub async fn a_wheel_disabled_field_doesnt_step(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#test-ws-off-input").await?;
    input.click().await?;
    page.wait_for_focus(&input).await?;
    let wheel = input
        .dispatch(SyntheticEvent::wheel().delta_y(10.0))
        .await?;
    assert_that!(wheel.default_prevented).is_false();
    count_stays(page, "#test-ws-off-count", "0").await
}

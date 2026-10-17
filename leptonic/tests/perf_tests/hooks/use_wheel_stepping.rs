// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_wheel_stepping` (`/perf/hooks/use_wheel_stepping`, on a plain
//! field): what each thing it offers costs, measured (`Page::measure`) and checked against its
//! budget (`tests/budgets/hooks/use_wheel_stepping.json`): mounting and unmounting it, focus
//! entering and leaving the field, the wheel down and up over the focused input (stepping), a
//! mostly horizontal wheel and a wheel over the unfocused input (both ignored), and making it
//! read-only (a wheel it ignores).
use std::time::Duration;

use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page, SyntheticEvent},
};

const PATH: &str = "/perf/hooks/use_wheel_stepping";
const INPUT: &str = "#perf-input";

/// Registers the performance tests of `use_wheel_stepping`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(FocusAndBlur))
        .with(fixture_test(Wheel))
        .with(fixture_test(HorizontalWheel))
        .with(fixture_test(UnfocusedWheel))
        .with(fixture_test(ReadOnly))
}

/// The count the wheel stepped.
async fn count(page: &Page<'_>) -> Result<f64, Report> {
    page.control("count").await
}

/// Opens the fixture and focuses the input (unmeasured).
async fn open_focused(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    input.click().await?;
    page.wait_for_focus(&input).await?;
    Ok(())
}

/// Mounting the field with its wheel handling and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(INPUT, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(INPUT).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(INPUT, 0).await?;
    Ok(())
}

/// Tab moving focus into the field and out of it.
#[browser_test]
pub async fn focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.measure("enter", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&page.element(INPUT).await?).await?;
    page.measure("leave", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&page.element("#perf-after").await?)
        .await?;
    Ok(())
}

/// The wheel down (incrementing) and up (decrementing) over the focused input.
#[browser_test]
pub async fn wheel(page: &Page<'_>) -> Result<(), Report> {
    open_focused(page).await?;
    let input = page.element(INPUT).await?;
    let down = page
        .measure(
            "down",
            input.dispatch(SyntheticEvent::wheel().delta_y(10.0)),
        )
        .await?;
    assert_that!(down.default_prevented).is_true();
    assert_that!(count(page).await?).is_equal_to(1.0);
    page.measure("up", input.dispatch(SyntheticEvent::wheel().delta_y(-10.0)))
        .await?;
    assert_that!(count(page).await?).is_equal_to(0.0);
    Ok(())
}

/// A mostly horizontal wheel (a trackpad's) over the focused input: ignored.
#[browser_test]
pub async fn horizontal_wheel(page: &Page<'_>) -> Result<(), Report> {
    open_focused(page).await?;
    let input = page.element(INPUT).await?;
    page.measure(
        "horizontal",
        input.dispatch(SyntheticEvent::wheel().delta_x(20.0).delta_y(5.0)),
    )
    .await?;
    assert_that!(count(page).await?).is_equal_to(0.0);
    Ok(())
}

/// A wheel over the input while the field doesn't have focus: the page scrolls as usual.
#[browser_test]
pub async fn unfocused_wheel(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    let wheel = page
        .measure(
            "wheel",
            input.dispatch(SyntheticEvent::wheel().delta_y(10.0)),
        )
        .await?;
    assert_that!(wheel.default_prevented).is_false();
    assert_that!(count(page).await?).is_equal_to(0.0);
    Ok(())
}

/// Making the focused field read-only, a wheel it ignores, and editable again.
#[browser_test]
pub async fn read_only(page: &Page<'_>) -> Result<(), Report> {
    open_focused(page).await?;
    let input = page.element(INPUT).await?;
    page.measure("read-only", page.control("read-only")).await?;
    page.measure(
        "ignored wheel",
        input.dispatch(SyntheticEvent::wheel().delta_y(10.0)),
    )
    .await?;
    assert_that!(count(page).await?).is_equal_to(0.0);
    page.measure("editable", page.control("editable")).await?;
    input
        .dispatch(SyntheticEvent::wheel().delta_y(10.0))
        .await?;
    input
        .prop_stays("value", "1", Duration::from_millis(50))
        .await?;
    Ok(())
}

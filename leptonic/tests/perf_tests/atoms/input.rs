// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `Input` (`leptonic/src/atoms/input.rs`, fixture
//! `/perf/atoms/input`): what each thing it offers costs, measured (`Page::measure`) and checked
//! against its budget (`tests/budgets/atoms/input.json`): in a text field, mounting and
//! unmounting it, typing and hover; standalone, mounting and unmounting it, hover, focus by
//! keyboard (its own focus ring) and changes of its reactive props (disabled, invalid).
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/input";
const INPUT: &str = "#perf-input";
const STANDALONE: &str = ".perf-standalone";

/// Registers the performance tests of `Input`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(Typing))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(MountAndUnmountStandalone))
        .with(fixture_test(HoverAndLeaveStandalone))
        .with(fixture_test(KeyboardFocusAndBlurStandalone))
        .with(fixture_test(DisableAndEnableStandalone))
        .with(fixture_test(InvalidStandalone))
}

/// Mounting the input of a text field (it renders the field's attributes) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(INPUT, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    let input = page.element(INPUT).await?;
    input.wait_for_prop("value", "Ada").await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(INPUT, 0).await?;
    // The field's label refers to the input: mounted again, so that it references something.
    page.control("mount").await?;
    page.element(INPUT).await?;
    Ok(())
}

/// Typing a character into the input of a text field.
#[browser_test]
pub async fn typing(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    input.type_keys(Key::End).await?;
    page.wait_for_focus(&input).await?;
    page.measure("type a character", page.send_keys("m"))
        .await?;
    input.wait_for_prop("value", "Adam").await?;
    Ok(())
}

/// The pointer entering and leaving the input of a text field.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", input.hover()).await?;
    input.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    input.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Mounting a standalone input and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_standalone(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount-standalone").await?;
    page.wait_for_count(STANDALONE, 0).await?;
    page.measure("mount", page.control("mount-standalone"))
        .await?;
    page.element(STANDALONE).await?;
    page.measure("unmount", page.control("unmount-standalone"))
        .await?;
    page.wait_for_count(STANDALONE, 0).await?;
    Ok(())
}

/// The pointer entering and leaving a standalone input.
#[browser_test]
pub async fn hover_and_leave_standalone(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(STANDALONE).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", input.hover()).await?;
    input.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    input.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Tab moving focus to a standalone input (its own focus ring) and on.
#[browser_test]
pub async fn keyboard_focus_and_blur_standalone(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(STANDALONE).await?;
    page.element(INPUT).await?.click().await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    input
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    input.wait_for_attr("data-focused", None).await?;
    Ok(())
}

/// Disabling a standalone input and enabling it again.
#[browser_test]
pub async fn disable_and_enable_standalone(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(STANDALONE).await?;
    page.measure("disable", page.control("disable-standalone"))
        .await?;
    input.wait_for_attr("data-disabled", Some("true")).await?;
    input.wait_for_attr("disabled", Some("true")).await?;
    page.measure("enable", page.control("enable-standalone"))
        .await?;
    input.wait_for_attr("data-disabled", None).await?;
    input.wait_for_attr("disabled", None).await?;
    Ok(())
}

/// Marking a standalone input invalid and valid again.
#[browser_test]
pub async fn invalid_standalone(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(STANDALONE).await?;
    page.measure("invalidate", page.control("invalidate-standalone"))
        .await?;
    input.wait_for_attr("data-invalid", Some("true")).await?;
    input.wait_for_attr("aria-invalid", Some("true")).await?;
    page.measure("uninvalidate", page.control("uninvalidate-standalone"))
        .await?;
    input.wait_for_attr("data-invalid", None).await?;
    input.wait_for_attr("aria-invalid", None).await?;
    Ok(())
}

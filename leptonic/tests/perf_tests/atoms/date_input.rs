// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `DateInput` (`/perf/atoms/date_input`, the segments' group of a
//! date and time field on June 5, 2024, 9:45 AM): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget (`tests/budgets/atoms/date_input.json`):
//! mounting and unmounting it (with its field), hover, the focus entering it (with a focus ring)
//! and leaving it, its segments changing (days only, and minutes again), and the states it shows
//! (disabled, read-only, invalid).
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/date_input";

/// Registers the performance tests of `DateInput`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(FocusWithinAndBlur))
        .with(fixture_test(SegmentsChange))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(ReadOnly))
        .with(fixture_test(Invalid))
}
/// The input (the field's group of segments).
const INPUT: &str = "#perf-field";

/// Mounting the input with its field (eight segments with the literals between them) and
/// unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(INPUT, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element("#perf-field [data-type='minute']")
        .await?
        .wait_for_inner_text("45")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(INPUT, 0).await?;
    Ok(())
}

/// The pointer entering and leaving the input.
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

/// Tab moving focus into the input (its first segment: focus within, with a focus ring) and
/// Shift+Tab out.
#[browser_test]
pub async fn focus_within_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let input = page.element(INPUT).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    input
        .wait_for_attr("data-focus-within", Some("true"))
        .await?;
    input
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("blur", page.send_keys(Key::Shift + Key::Tab))
        .await?;
    input.wait_for_attr("data-focus-within", None).await?;
    Ok(())
}

/// Its segments changing: the date only (no hour, minute, day period), and minutes again.
#[browser_test]
pub async fn segments_change(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let hours = "#perf-field [data-type='hour']";
    page.wait_for_count(hours, 1).await?;
    page.measure("show days", page.control("show-days")).await?;
    page.wait_for_count(hours, 0).await?;
    page.measure("show minutes", page.control("show-minutes"))
        .await?;
    page.wait_for_count(hours, 1).await?;
    Ok(())
}

/// The input showing its field disabled and enabled again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    page.measure("disable", page.control("disable")).await?;
    input.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("enable", page.control("enable")).await?;
    input.wait_for_attr("data-disabled", None).await?;
    Ok(())
}

/// The input showing its field read-only and editable again.
#[browser_test]
pub async fn read_only(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    page.measure("make read-only", page.control("make-read-only"))
        .await?;
    input.wait_for_attr("data-readonly", Some("true")).await?;
    page.measure("make editable", page.control("make-editable"))
        .await?;
    input.wait_for_attr("data-readonly", None).await?;
    Ok(())
}

/// The input showing its field invalid and valid again.
#[browser_test]
pub async fn invalid(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    page.measure("invalidate", page.control("invalidate"))
        .await?;
    input.wait_for_attr("data-invalid", Some("true")).await?;
    page.measure("validate", page.control("validate")).await?;
    input.wait_for_attr("data-invalid", None).await?;
    Ok(())
}

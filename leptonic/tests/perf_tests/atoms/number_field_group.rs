// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `NumberFieldGroup` (`leptonic/src/atoms/number_field.rs`, fixture
//! `/perf/atoms/number_field_group`): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/number_field_group.json`): mounting and unmounting it (with the
//! field's input) in its number field, hover, focus entering and leaving it by keyboard, and
//! the field's states it shows (disabled, invalid).
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/number_field_group";
const GROUP: &str = ".leptonic-NumberFieldGroup";
const INPUT: &str = "#perf-input";

/// Registers the performance tests of `NumberFieldGroup`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(FocusWithinAndOut))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(Invalid))
}

/// Mounting the group with the input and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(GROUP, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(GROUP)
        .await?
        .wait_for_attr("role", Some("group"))
        .await?;
    page.element(INPUT)
        .await?
        .wait_for_prop("value", "5")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(GROUP, 0).await?;
    // The field's label refers to the input: mounted again, so that it references something.
    page.control("mount").await?;
    page.element(GROUP).await?;
    Ok(())
}

/// The pointer entering and leaving the group.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let group = page.element(GROUP).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", group.hover()).await?;
    group.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    group.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Tab moving focus into the group (its input) and out of it.
#[browser_test]
pub async fn focus_within_and_out(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let group = page.element(GROUP).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    group
        .wait_for_attr("data-focus-within", Some("true"))
        .await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    group.wait_for_attr("data-focus-within", None).await?;
    Ok(())
}

/// Disabling the field (the group shows it) and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let group = page.element(GROUP).await?;
    page.measure("disable", page.control("disable")).await?;
    group.wait_for_attr("data-disabled", Some("true")).await?;
    group.wait_for_attr("aria-disabled", Some("true")).await?;
    page.measure("enable", page.control("enable")).await?;
    group.wait_for_attr("data-disabled", None).await?;
    Ok(())
}

/// Marking the field invalid (the group shows it) and valid again.
#[browser_test]
pub async fn invalid(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let group = page.element(GROUP).await?;
    page.measure("invalidate", page.control("invalidate"))
        .await?;
    group.wait_for_attr("data-invalid", Some("true")).await?;
    page.measure("uninvalidate", page.control("uninvalidate"))
        .await?;
    group.wait_for_attr("data-invalid", None).await?;
    Ok(())
}

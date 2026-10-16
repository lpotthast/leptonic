// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `RadioField` (`/perf/atoms/radio_field`): what each thing it
//! offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/radio_field.json`): mounting and unmounting it (with its `RadioButton`)
//! in a mounted `RadioGroup`, the group's selection reaching it and leaving it, and disabling it.
//! The button's interactions are measured in `radio_button.rs`, the group's in `radio_group.rs`.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/atoms/radio_field";

/// Registers the performance tests of `RadioField`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(SelectionChanges))
        .with(fixture_test(DisableAndEnable))
}
const FIELD: &str = "#perf-radio-field";
const OTHER: &str = "#perf-radio-field-other";

/// Mounting the field (with its button) in the group and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(FIELD, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(FIELD).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(FIELD, 0).await?;
    Ok(())
}

/// The app selecting the field's radio, the other radio instead, and none.
#[browser_test]
pub async fn selection_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("select", page.control("select")).await?;
    page.wait_for_attr(FIELD, "data-selected", Some("true"))
        .await?;
    page.measure("select other", page.control("select-other"))
        .await?;
    page.wait_for_attr(FIELD, "data-selected", None).await?;
    page.wait_for_attr(OTHER, "data-selected", Some("true"))
        .await?;
    page.measure("select none", page.control("select-none"))
        .await?;
    page.wait_for_attr(OTHER, "data-selected", None).await?;
    Ok(())
}

/// Disabling the field's radio and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("disable", page.control("disable")).await?;
    page.wait_for_attr(FIELD, "data-disabled", Some("true"))
        .await?;
    page.measure("enable", page.control("enable")).await?;
    page.wait_for_attr(FIELD, "data-disabled", None).await?;
    Ok(())
}

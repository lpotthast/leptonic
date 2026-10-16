// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `CheckboxGroup` (`/perf/atoms/checkbox_group`): what each thing it
//! offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/checkbox_group.json`), with 10 and with 1,000 checkboxes: mounting and
//! unmounting it, a click checking and unchecking one of its checkboxes, the app checking all or
//! none of them (its value controlled by app state), and disabling it; with 10, changes of its
//! other reactive props (read-only, required, invalid). A single checkbox's costs are measured in
//! `checkbox_button.rs` and `checkbox_field.rs`.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/checkbox_group";

/// Registers the performance tests of `CheckboxGroup`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(CheckWithTheMouse))
        .with(fixture_test(CheckWithTheMouseLarge))
        .with(fixture_test(CheckAllAndNone))
        .with(fixture_test(CheckAllAndNoneLarge))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(DisableAndEnableLarge))
        .with(fixture_test(ReadOnlyAndEditable))
        .with(fixture_test(RequiredAndOptional))
        .with(fixture_test(InvalidAndValid))
}
const GROUP: &str = "#perf-checkbox-group";
const CHECKBOXES: &str = "#perf-checkbox-group .leptonic-CheckboxButton";
const FIRST: &str = "#perf-checkbox-group > :first-child .leptonic-CheckboxButton";
const LAST: &str = "#perf-checkbox-group > :last-child .leptonic-CheckboxButton";

/// How many checkboxes are checked.
async fn checked(page: &Page<'_>) -> Result<f64, Report> {
    page.control("checked").await
}

/// Replaces the group of 10 checkboxes with one of 1,000 (not measured).
async fn mount_large(page: &Page<'_>) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(GROUP, 0).await?;
    page.control("mount-large").await?;
    page.wait_for_count(CHECKBOXES, 1_000).await?;
    Ok(())
}

/// Mounting and unmounting the group with the control `mount` (`mount-large`) and `count`
/// checkboxes.
async fn mount_and_unmount_with(page: &Page<'_>, mount: &str, count: usize) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(GROUP, 0).await?;
    page.measure("mount", page.control(mount)).await?;
    page.wait_for_count(CHECKBOXES, count).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(GROUP, 0).await?;
    Ok(())
}

/// Clicks checking and unchecking the checkbox `checkbox`.
async fn check_with_the_mouse_on(page: &Page<'_>, checkbox: &str) -> Result<(), Report> {
    let checkbox = page.element(checkbox).await?;
    page.measure("check", checkbox.click()).await?;
    checkbox
        .wait_for_attr("data-selected", Some("true"))
        .await?;
    assert_that!(checked(page).await?).is_equal_to(1.0);
    page.measure("uncheck", checkbox.click()).await?;
    checkbox.wait_for_attr("data-selected", None).await?;
    assert_that!(checked(page).await?).is_equal_to(0.0);
    Ok(())
}

/// The app checking all `count` checkboxes and none again.
async fn check_all_and_none_of(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.measure("check all", page.control("check-all")).await?;
    page.wait_for_attr(LAST, "data-selected", Some("true"))
        .await?;
    assert_that!(checked(page).await?).is_equal_to(f64::from(u32::try_from(count)?));
    page.measure("check none", page.control("check-none"))
        .await?;
    page.wait_for_attr(LAST, "data-selected", None).await?;
    page.wait_for_attr(FIRST, "data-selected", None).await?;
    Ok(())
}

/// Disabling the group (and so every checkbox) and enabling it again.
async fn disable_and_enable_group(page: &Page<'_>) -> Result<(), Report> {
    page.measure("disable", page.control("disable")).await?;
    page.wait_for_attr(GROUP, "data-disabled", Some("true"))
        .await?;
    page.wait_for_attr(LAST, "data-disabled", Some("true"))
        .await?;
    page.measure("enable", page.control("enable")).await?;
    page.wait_for_attr(GROUP, "data-disabled", None).await?;
    page.wait_for_attr(LAST, "data-disabled", None).await?;
    Ok(())
}

/// Measures the control `on` (named after it), waits until the group has the flag `attr`, then
/// measures the control `off` and waits until the flag is gone.
async fn flag_round_trip(page: &Page<'_>, attr: &str, on: &str, off: &str) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure(on, page.control(on)).await?;
    page.wait_for_attr(GROUP, attr, Some("true")).await?;
    page.measure(off, page.control(off)).await?;
    page.wait_for_attr(GROUP, attr, None).await?;
    Ok(())
}

/// Mounting the group of 10 checkboxes and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount_with(page, "mount", 10).await
}

/// Mounting the group of 1,000 checkboxes and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount_with(page, "mount-large", 1_000).await
}

/// Clicks checking and unchecking the first of 10 checkboxes.
#[browser_test]
pub async fn check_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    check_with_the_mouse_on(page, FIRST).await
}

/// Clicks checking and unchecking the last of 1,000 checkboxes.
#[browser_test]
pub async fn check_with_the_mouse_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_large(page).await?;
    check_with_the_mouse_on(page, LAST).await
}

/// The app checking all 10 checkboxes and none again.
#[browser_test]
pub async fn check_all_and_none(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    check_all_and_none_of(page, 10).await
}

/// The app checking all 1,000 checkboxes and none again.
#[browser_test]
pub async fn check_all_and_none_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_large(page).await?;
    check_all_and_none_of(page, 1_000).await
}

/// Disabling the group of 10 checkboxes and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    disable_and_enable_group(page).await
}

/// Disabling the group of 1,000 checkboxes and enabling it again.
#[browser_test]
pub async fn disable_and_enable_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_large(page).await?;
    disable_and_enable_group(page).await
}

/// Making the group of 10 read-only and editable again.
#[browser_test]
pub async fn read_only_and_editable(page: &Page<'_>) -> Result<(), Report> {
    flag_round_trip(page, "data-readonly", "make-read-only", "make-editable").await
}

/// Requiring a checked checkbox in the group of 10, and making it optional again.
#[browser_test]
pub async fn required_and_optional(page: &Page<'_>) -> Result<(), Report> {
    flag_round_trip(page, "data-required", "require", "make-optional").await
}

/// Marking the group of 10 invalid and valid again.
#[browser_test]
pub async fn invalid_and_valid(page: &Page<'_>) -> Result<(), Report> {
    flag_round_trip(page, "data-invalid", "invalidate", "validate").await
}

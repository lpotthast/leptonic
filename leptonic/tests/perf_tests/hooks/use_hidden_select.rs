// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_hidden_select` (`/perf/hooks/use_hidden_select`, the hidden
//! select of the plain select of `use_select`'s performance test, field `option`): what each thing
//! it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_hidden_select.json`): mounting and unmounting it with 10 options (a
//! native `<select>`) and with 1,000 (a hidden input, above 300), the value changing at both sizes,
//! autofill (the native select changing), and the disabled and required states changing.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use super::use_select::{large, value};
use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_hidden_select";
/// The native select (up to 300 options).
const NATIVE: &str = "select[name=option]";
/// The hidden input (more than 300 options).
const INPUT: &str = "input[name=option]";

/// Registers the performance tests of `use_hidden_select`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(ValueChange))
        .with(fixture_test(ValueChangeLarge))
        .with(fixture_test(Autofill))
        .with(fixture_test(DisabledAndRequired))
}

/// Mounting the hidden select with 10 options (a native select) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(NATIVE, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(format!("{NATIVE} option"), 11).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(NATIVE, 0).await?;
    Ok(())
}

/// Mounting the hidden select with 1,000 options (an input with the value) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("select-first").await?;
    large(page).await?;
    page.element(INPUT).await?;
    page.control("unmount").await?;
    page.wait_for_count(INPUT, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(INPUT)
        .await?
        .wait_for_prop("value", "0")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(INPUT, 0).await?;
    Ok(())
}

/// The value of a select of 10 options changing: the native select follows.
#[browser_test]
pub async fn value_change(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let native = page.element(NATIVE).await?;
    page.measure("select", page.control("select-first")).await?;
    native.wait_for_prop("value", "0").await?;
    page.measure("clear", page.control("clear-value")).await?;
    native.wait_for_prop("value", "").await?;
    Ok(())
}

/// The value of a select of 1,000 options changing: the hidden input follows.
#[browser_test]
pub async fn value_change_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    page.measure("select", page.control("select-first")).await?;
    page.element(INPUT)
        .await?
        .wait_for_prop("value", "0")
        .await?;
    page.measure("clear", page.control("clear-value")).await?;
    page.element(INPUT)
        .await?
        .wait_for_prop("value", "")
        .await?;
    Ok(())
}

/// Autofill: the native select's value changing (with its `change` event) selects the option.
#[browser_test]
pub async fn autofill(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element(NATIVE).await?;
    let fill = page.low_level().eval::<()>(
        "const select = document.querySelector('select[name=option]');
         select.value = '3';
         select.dispatchEvent(new Event('change', {bubbles: true}));",
        vec![],
    );
    page.measure("autofill", fill).await?;
    assert_that!(value(page).await?).is_equal_to(3.0);
    Ok(())
}

/// The select disabled and enabled, required and optional: the native select follows.
#[browser_test]
pub async fn disabled_and_required(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let native = page.element(NATIVE).await?;
    page.measure("disable", page.control("disable")).await?;
    native.wait_for_prop("disabled", "true").await?;
    page.measure("enable", page.control("enable")).await?;
    native.wait_for_prop("disabled", "false").await?;
    page.measure("require", page.control("require")).await?;
    native.wait_for_prop("required", "true").await?;
    page.measure("unrequire", page.control("unrequire")).await?;
    native.wait_for_prop("required", "false").await?;
    Ok(())
}

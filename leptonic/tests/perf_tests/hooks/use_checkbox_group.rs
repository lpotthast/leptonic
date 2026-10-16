// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_checkbox_group`
//! (`leptonic/src/hooks/form/use_checkbox_group.rs`, fixture `/perf/hooks/use_checkbox_group`):
//! what each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_checkbox_group.json`), with 10 and with 1,000 checkboxes: mounting
//! and unmounting the group, checking and unchecking a checkbox, disabling and enabling the
//! group (and a click it then ignores), and focus entering and leaving it by keyboard.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page, css},
};

const PATH: &str = "/perf/hooks/use_checkbox_group";
const GROUP: &str = "#perf-group";
const CHECKBOXES: &str = "#perf-group input";
/// The label of "Item 3" (by its input's value: a text lookup reads every label's text, too slow
/// among 1,000).
const ITEM_3: &str = "#perf-group label:has(input[value='3'])";

/// Registers the performance tests of `use_checkbox_group`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmount1000))
        .with(fixture_test(CheckAndUncheck))
        .with(fixture_test(CheckAndUncheck1000))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(DisableAndEnable1000))
        .with(fixture_test(KeyboardFocusAndBlur))
}

/// How many checkboxes are checked.
async fn checked(page: &Page<'_>) -> Result<f64, Report> {
    page.control("checked").await
}

/// Mounts the group with 1,000 checkboxes (unmeasured).
async fn mount_1000(page: &Page<'_>) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(GROUP, 0).await?;
    page.control("mount-1000").await?;
    page.wait_for_count(CHECKBOXES, 1_000).await
}

/// Mounts the group (`mount` or `mount-1000`) with `size` checkboxes and unmounts it, both
/// measured.
async fn measure_mount_and_unmount(
    page: &Page<'_>,
    mount: &str,
    size: usize,
) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(GROUP, 0).await?;
    page.measure("mount", page.control(mount)).await?;
    page.element(GROUP)
        .await?
        .wait_for_attr("role", Some("group"))
        .await?;
    assert_that!(page.count(CHECKBOXES).await?).is_equal_to(size);
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(GROUP, 0).await
}

/// Clicks "Item 3" twice, measured: checked, then unchecked.
async fn measure_check_and_uncheck(page: &Page<'_>) -> Result<(), Report> {
    let label = page.element(ITEM_3).await?;
    let input = label.element("input").await?;
    page.measure("check", label.click()).await?;
    input.wait_for_prop("checked", "true").await?;
    assert_that!(checked(page).await?).is_equal_to(1.0);
    page.measure("uncheck", label.click()).await?;
    input.wait_for_prop("checked", "false").await?;
    assert_that!(checked(page).await?).is_equal_to(0.0);
    Ok(())
}

/// Disables the group, clicks "Item 3" (ignored) and enables the group, measured.
async fn measure_disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    let group = page.element(GROUP).await?;
    let label = page.element(ITEM_3).await?;
    page.measure("disable", page.control("disable")).await?;
    group.wait_for_attr("aria-disabled", Some("true")).await?;
    page.measure("ignored click", label.click()).await?;
    assert_that!(checked(page).await?).is_equal_to(0.0);
    page.measure("enable", page.control("enable")).await?;
    group.wait_for_attr("aria-disabled", None).await?;
    Ok(())
}

/// Mounting the group with 10 checkboxes and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    measure_mount_and_unmount(page, "mount", 10).await
}

/// Mounting the group with 1,000 checkboxes and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_1000(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    measure_mount_and_unmount(page, "mount-1000", 1_000).await
}

/// Checking and unchecking a checkbox of 10.
#[browser_test]
pub async fn check_and_uncheck(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    measure_check_and_uncheck(page).await
}

/// Checking and unchecking a checkbox of 1,000.
#[browser_test]
pub async fn check_and_uncheck_1000(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_1000(page).await?;
    measure_check_and_uncheck(page).await
}

/// Disabling a group of 10 checkboxes, a click it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    measure_disable_and_enable(page).await
}

/// Disabling a group of 1,000 checkboxes, a click it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable_1000(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_1000(page).await?;
    measure_disable_and_enable(page).await
}

/// Tab moving focus into the group (its first checkbox) and back out.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let first = page.element(css("label").text("Item 0")).await?;
    let first = first.element("input").await?;
    let before = page.element("#perf-before").await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&first).await?;
    page.measure("blur", page.send_keys(Key::Shift + Key::Tab))
        .await?;
    page.wait_for_focus(&before).await?;
    Ok(())
}

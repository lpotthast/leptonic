// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_radio_group`
//! (`leptonic/src/hooks/form/use_radio_group.rs`, fixture `/perf/hooks/use_radio_group`): what
//! each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_radio_group.json`), with 10 and with 1,000 radios: mounting and
//! unmounting the group, the arrow keys moving the selection, selecting a radio with the mouse,
//! and disabling and enabling the group (and a click it then ignores).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_radio_group";
const GROUP: &str = "#perf-group";
const RADIOS: &str = "#perf-group input";
/// The label of option 3.
const OPTION_3: &str = "#perf-group label:has(input[value='3'])";

/// Registers the performance tests of `use_radio_group`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmount1000))
        .with(fixture_test(ArrowKeys))
        .with(fixture_test(ArrowKeys1000))
        .with(fixture_test(SelectWithTheMouse))
        .with(fixture_test(SelectWithTheMouse1000))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(DisableAndEnable1000))
}

/// The selected option (-1: none).
async fn selected(page: &Page<'_>) -> Result<f64, Report> {
    page.control("selected").await
}

/// The radio input of option `option` (by its value: a text lookup reads every label's text, too
/// slow among 1,000).
async fn radio(page: &Page<'_>, option: u32) -> Result<WebElement, Report> {
    page.element(format!("#perf-group input[value='{option}']"))
        .await
}

/// Mounts the group with 1,000 radios (unmeasured).
async fn mount_1000(page: &Page<'_>) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(GROUP, 0).await?;
    page.control("mount-1000").await?;
    page.wait_for_count(RADIOS, 1_000).await
}

/// Mounts the group (`mount` or `mount-1000`) with `size` radios and unmounts it, both measured.
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
        .wait_for_attr("role", Some("radiogroup"))
        .await?;
    assert_that!(page.count(RADIOS).await?).is_equal_to(size);
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(GROUP, 0).await
}

/// Tabs to the selected radio (option 0), then measures Arrow Down (option 1 selected) and
/// Arrow Up (option 0 again).
async fn measure_arrow_keys(page: &Page<'_>) -> Result<(), Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let first = radio(page, 0).await?;
    page.wait_for_focus(&first).await?;
    page.measure("arrow down", page.send_keys(Key::Down))
        .await?;
    let second = radio(page, 1).await?;
    page.wait_for_focus(&second).await?;
    second.wait_for_prop("checked", "true").await?;
    assert_that!(selected(page).await?).is_equal_to(1.0);
    page.measure("arrow up", page.send_keys(Key::Up)).await?;
    page.wait_for_focus(&first).await?;
    assert_that!(selected(page).await?).is_equal_to(0.0);
    Ok(())
}

/// Measures a click on the label of option 3: it is selected.
async fn measure_select_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    let label = page.element(OPTION_3).await?;
    page.measure("click", label.click()).await?;
    radio(page, 3)
        .await?
        .wait_for_prop("checked", "true")
        .await?;
    assert_that!(selected(page).await?).is_equal_to(3.0);
    Ok(())
}

/// Measures disabling the group, a click on option 3 (ignored) and enabling the group.
async fn measure_disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    let group = page.element(GROUP).await?;
    let label = page.element(OPTION_3).await?;
    page.measure("disable", page.control("disable")).await?;
    group.wait_for_attr("aria-disabled", Some("true")).await?;
    page.measure("ignored click", label.click()).await?;
    assert_that!(selected(page).await?).is_equal_to(0.0);
    page.measure("enable", page.control("enable")).await?;
    group.wait_for_attr("aria-disabled", None).await?;
    Ok(())
}

/// Mounting the group with 10 radios and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    measure_mount_and_unmount(page, "mount", 10).await
}

/// Mounting the group with 1,000 radios and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_1000(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    measure_mount_and_unmount(page, "mount-1000", 1_000).await
}

/// The arrow keys moving the selection among 10 radios.
#[browser_test]
pub async fn arrow_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    measure_arrow_keys(page).await
}

/// The arrow keys moving the selection among 1,000 radios.
#[browser_test]
pub async fn arrow_keys_1000(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_1000(page).await?;
    measure_arrow_keys(page).await
}

/// Selecting one of 10 radios with the mouse.
#[browser_test]
pub async fn select_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    measure_select_with_the_mouse(page).await
}

/// Selecting one of 1,000 radios with the mouse.
#[browser_test]
pub async fn select_with_the_mouse_1000(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_1000(page).await?;
    measure_select_with_the_mouse(page).await
}

/// Disabling a group of 10 radios, a click it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    measure_disable_and_enable(page).await
}

/// Disabling a group of 1,000 radios, a click it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable_1000(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_1000(page).await?;
    measure_disable_and_enable(page).await
}

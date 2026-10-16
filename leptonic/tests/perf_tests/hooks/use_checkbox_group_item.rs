// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_checkbox_group_item`
//! (`leptonic/src/hooks/form/use_checkbox_group.rs`, fixture
//! `/perf/hooks/use_checkbox_group_item`): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_checkbox_group_item.json`), for a checkbox in a group of four:
//! mounting and unmounting it, checking and unchecking it with the mouse (on its label) and
//! the keyboard, its pressed state, and disabling it (and a click it then ignores).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page, css},
};

const PATH: &str = "/perf/hooks/use_checkbox_group_item";
const INPUT: &str = "#perf-input";

/// Registers the performance tests of `use_checkbox_group_item`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(PressWithTheMouse))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(PressWithTheKeyboard))
        .with(fixture_test(DisableAndEnable))
}

/// Whether the group checks the checkbox's value (1) or not (0).
async fn checked(page: &Page<'_>) -> Result<f64, Report> {
    page.control("checked").await
}

/// The checkbox's label.
async fn label(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element(css("label").text("Item 3")).await
}

/// Mounting the checkbox in its group and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(INPUT, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    let input = page.element(INPUT).await?;
    input.wait_for_attr("name", Some("items")).await?;
    input.wait_for_attr("value", Some("3")).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(INPUT, 0).await?;
    Ok(())
}

/// Clicks on the label: the checkbox is checked, then unchecked.
#[browser_test]
pub async fn press_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let label = label(page).await?;
    let input = page.element(INPUT).await?;
    page.measure("check", label.click()).await?;
    input.wait_for_prop("checked", "true").await?;
    assert_that!(checked(page).await?).is_equal_to(1.0);
    page.measure("uncheck", label.click()).await?;
    input.wait_for_prop("checked", "false").await?;
    assert_that!(checked(page).await?).is_equal_to(0.0);
    Ok(())
}

/// Holding the pointer down on the label and releasing it (checked).
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let label = label(page).await?;
    let input = page.element(INPUT).await?;
    let held = page.measure("hold", label.press_and_hold()).await?;
    page.measure("release", held.release()).await?;
    input.wait_for_prop("checked", "true").await?;
    assert_that!(checked(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Space checking and unchecking the focused checkbox.
#[browser_test]
pub async fn press_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    input.click().await?;
    input.wait_for_prop("checked", "true").await?;
    page.wait_for_focus(&input).await?;
    page.measure("uncheck", page.send_keys(" ")).await?;
    input.wait_for_prop("checked", "false").await?;
    page.measure("check", page.send_keys(" ")).await?;
    input.wait_for_prop("checked", "true").await?;
    assert_that!(checked(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Disabling the checkbox, a click it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let label = label(page).await?;
    let input = page.element(INPUT).await?;
    page.measure("disable", page.control("disable")).await?;
    input.wait_for_attr("disabled", Some("true")).await?;
    page.measure("ignored click", label.click()).await?;
    assert_that!(checked(page).await?).is_equal_to(0.0);
    page.measure("enable", page.control("enable")).await?;
    input.wait_for_attr("disabled", None).await?;
    Ok(())
}

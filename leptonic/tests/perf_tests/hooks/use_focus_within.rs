// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_focus_within` (`/perf/hooks/use_focus_within`, on a group of
//! two buttons): what each thing it offers costs, measured (`Page::measure`) and checked against
//! its budget (`tests/budgets/hooks/use_focus_within.json`): mounting and unmounting it, focus
//! entering, moving within and leaving by the keyboard and by the pointer, and disabling it
//! (focus it ignores).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_focus_within";

/// Registers the performance tests of `use_focus_within`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(KeyboardEnterMoveAndLeave))
        .with(fixture_test(PointerEnterMoveAndLeave))
        .with(fixture_test(DisableAndEnable))
}
const GROUP: &str = "#perf-group";

/// How often focus entered the group.
async fn entries(page: &Page<'_>) -> Result<f64, Report> {
    page.control("entries").await
}

/// Mounting the group with its focus handling and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(GROUP, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(GROUP).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(GROUP, 0).await?;
    Ok(())
}

/// Tab moving focus into the group, within it, and out of it.
#[browser_test]
pub async fn keyboard_enter_move_and_leave(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let group = page.element(GROUP).await?;
    page.measure("enter", page.send_keys(Key::Tab)).await?;
    group
        .wait_for_attr("data-focus-within", Some("true"))
        .await?;
    page.measure("move within", page.send_keys(Key::Tab))
        .await?;
    page.wait_for_focus(&page.element("#perf-second").await?)
        .await?;
    page.measure("leave", page.send_keys(Key::Tab)).await?;
    group.wait_for_attr("data-focus-within", None).await?;
    assert_that!(entries(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Clicks focusing a button of the group, the other one, and a button outside.
#[browser_test]
pub async fn pointer_enter_move_and_leave(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let group = page.element(GROUP).await?;
    let first = page.element("#perf-first").await?;
    let second = page.element("#perf-second").await?;
    let after = page.element("#perf-after").await?;
    page.measure("enter", first.click()).await?;
    group
        .wait_for_attr("data-focus-within", Some("true"))
        .await?;
    page.measure("move within", second.click()).await?;
    page.wait_for_focus(&second).await?;
    page.measure("leave", after.click()).await?;
    group.wait_for_attr("data-focus-within", None).await?;
    assert_that!(entries(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Disabling the focus handling, focus it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.measure("disable", page.control("disable")).await?;
    page.measure("ignored focus", page.send_keys(Key::Tab))
        .await?;
    page.wait_for_focus(&page.element("#perf-first").await?)
        .await?;
    assert_that!(entries(page).await?).is_equal_to(0.0);
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.measure("enable", page.control("enable")).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_attr(GROUP, "data-focus-within", Some("true"))
        .await?;
    assert_that!(entries(page).await?).is_equal_to(1.0);
    Ok(())
}

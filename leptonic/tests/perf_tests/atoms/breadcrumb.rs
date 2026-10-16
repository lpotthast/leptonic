// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `Breadcrumb` (`/perf/atoms/breadcrumb`, an item alone in its
//! `Breadcrumbs`): what each thing it offers costs, measured (`Page::measure`) and checked
//! against its budget (`tests/budgets/atoms/breadcrumb.json`): mounting and unmounting it,
//! pressing it with the mouse and the keyboard (the breadcrumbs' `on_action` with its key, and
//! the navigation), hover, keyboard focus, becoming the current item and not, and the
//! breadcrumbs around it disabling it.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/breadcrumb";

/// Registers the performance tests of `Breadcrumb`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(PressWithTheMouse))
        .with(fixture_test(PressWithTheKeyboard))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(BecomeCurrent))
        .with(fixture_test(DisabledBreadcrumbs))
}
const ITEM: &str = "#perf-nav li";
/// The item's link (an `<a>`, a `<span>` while disabled).
const LINK: &str = "#perf-nav li > *";

/// How often the item was pressed (`on_action` with its key).
async fn actions(page: &Page<'_>) -> Result<f64, Report> {
    page.control("actions").await
}

/// Mounting the item and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(ITEM, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(ITEM).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(ITEM, 0).await?;
    Ok(())
}

/// A click on the item: the breadcrumbs' `on_action` and the navigation to its link.
#[browser_test]
pub async fn press_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let link = page.element(LINK).await?;
    page.measure("click", link.click()).await?;
    assert_that!(actions(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Enter on the focused item.
#[browser_test]
pub async fn press_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let link = page.element(LINK).await?;
    page.wait_for_focus(&link).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    assert_that!(actions(page).await?).is_equal_to(1.0);
    Ok(())
}

/// The pointer entering and leaving the item.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let link = page.element(LINK).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", link.hover()).await?;
    link.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    link.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Tab moving focus to the item and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let link = page.element(LINK).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    link.wait_for_attr("data-focused", Some("true")).await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    link.wait_for_attr("data-focused", None).await?;
    Ok(())
}

/// The item becoming the current one (its link disabled, `aria-current="page"`), a click it
/// ignores, and back.
#[browser_test]
pub async fn become_current(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("make current", page.control("make-current"))
        .await?;
    page.wait_for_attr(ITEM, "data-current", Some("true"))
        .await?;
    page.wait_for_attr(ITEM, "data-disabled", Some("true"))
        .await?;
    let link = page.element(LINK).await?;
    page.measure("ignored click", link.click()).await?;
    assert_that!(actions(page).await?).is_equal_to(0.0);
    page.measure("make not current", page.control("make-not-current"))
        .await?;
    page.wait_for_attr(ITEM, "data-current", None).await?;
    page.wait_for_attr(ITEM, "data-disabled", None).await?;
    Ok(())
}

/// The breadcrumbs around it disabling the item, a click it ignores, and enabling it again.
#[browser_test]
pub async fn disabled_breadcrumbs(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("disable", page.control("disable-breadcrumbs"))
        .await?;
    page.wait_for_attr(ITEM, "data-disabled", Some("true"))
        .await?;
    let link = page.element(LINK).await?;
    page.measure("ignored click", link.click()).await?;
    assert_that!(actions(page).await?).is_equal_to(0.0);
    page.measure("enable", page.control("enable-breadcrumbs"))
        .await?;
    page.wait_for_attr(ITEM, "data-disabled", None).await?;
    Ok(())
}

// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `Breadcrumbs` (`/perf/atoms/breadcrumbs`): what each thing it
//! offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/breadcrumbs.json`): mounting and unmounting a trail of 3 and of 100
//! items, pressing an item with the mouse (at both sizes) and the keyboard (its `on_action` and
//! the navigation), hover, keyboard focus, and changes of its reactive props (disabled, label,
//! the current item).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/breadcrumbs";

/// Registers the performance tests of `Breadcrumbs`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountMany))
        .with(fixture_test(PressWithTheMouse))
        .with(fixture_test(PressWithTheMouseMany))
        .with(fixture_test(PressWithTheKeyboard))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(ChangeTheCurrentItem))
        .with(fixture_test(ChangeTheLabel))
}
const LIST: &str = "#perf-nav ol";
const FIRST_ITEM: &str = "#perf-nav li:nth-child(1)";
/// The link of the first item (an `<a>`, a `<span>` while disabled).
const FIRST_LINK: &str = "#perf-nav li:nth-child(1) > *";
const LAST_ITEM: &str = "#perf-nav li:last-child";

/// How often an item was pressed (`on_action`).
async fn actions(page: &Page<'_>) -> Result<f64, Report> {
    page.control("actions").await
}

/// Mounting the trail of 3 items and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(LIST, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count("#perf-nav li", 3).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(LIST, 0).await?;
    Ok(())
}

/// Mounting a trail of 100 items and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_many(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(LIST, 0).await?;
    page.control("use-many").await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count("#perf-nav li", 100).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(LIST, 0).await?;
    Ok(())
}

/// A click on the first of 3 items: its `on_action` and the navigation to its link.
#[browser_test]
pub async fn press_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let link = page.element(FIRST_LINK).await?;
    page.measure("click", link.click()).await?;
    assert_that!(actions(page).await?).is_equal_to(1.0);
    Ok(())
}

/// A click on the first of 100 items.
#[browser_test]
pub async fn press_with_the_mouse_many(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(LIST, 0).await?;
    page.control("use-many").await?;
    page.control("mount").await?;
    page.wait_for_count("#perf-nav li", 100).await?;
    let link = page.element(FIRST_LINK).await?;
    page.measure("click", link.click()).await?;
    assert_that!(actions(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Enter on the focused first item.
#[browser_test]
pub async fn press_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let link = page.element(FIRST_LINK).await?;
    page.wait_for_focus(&link).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    assert_that!(actions(page).await?).is_equal_to(1.0);
    Ok(())
}

/// The pointer entering and leaving the first item.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let link = page.element(FIRST_LINK).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", link.hover()).await?;
    link.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    link.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Tab moving focus to the first item, and Shift+Tab back.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let link = page.element(FIRST_LINK).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    link.wait_for_attr("data-focused", Some("true")).await?;
    page.measure("blur", page.send_keys(Key::Shift + Key::Tab))
        .await?;
    link.wait_for_attr("data-focused", None).await?;
    Ok(())
}

/// Disabling all items, a click they ignore, and enabling them again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("disable", page.control("disable")).await?;
    page.wait_for_attr(FIRST_ITEM, "data-disabled", Some("true"))
        .await?;
    let link = page.element(FIRST_LINK).await?;
    page.measure("ignored click", link.click()).await?;
    assert_that!(actions(page).await?).is_equal_to(0.0);
    page.measure("enable", page.control("enable")).await?;
    page.wait_for_attr(FIRST_ITEM, "data-disabled", None)
        .await?;
    Ok(())
}

/// Making the first item the current one (the last one no longer is), a click on it (ignored:
/// the current item is disabled), and back.
#[browser_test]
pub async fn change_the_current_item(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("make the first current", page.control("make-first-current"))
        .await?;
    page.wait_for_attr(FIRST_ITEM, "data-current", Some("true"))
        .await?;
    page.wait_for_attr(LAST_ITEM, "data-current", None).await?;
    let link = page.element(FIRST_LINK).await?;
    page.measure("ignored click", link.click()).await?;
    assert_that!(actions(page).await?).is_equal_to(0.0);
    page.measure("make the last current", page.control("make-last-current"))
        .await?;
    page.wait_for_attr(LAST_ITEM, "data-current", Some("true"))
        .await?;
    page.wait_for_attr(FIRST_ITEM, "data-current", None).await?;
    Ok(())
}

/// Labeling the trail and removing the label again (its default label).
#[browser_test]
pub async fn change_the_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("set", page.control("set-label")).await?;
    page.wait_for_attr(LIST, "aria-label", Some("Trail"))
        .await?;
    page.measure("unset", page.control("unset-label")).await?;
    page.wait_for_attr(LIST, "aria-label", Some("Breadcrumbs"))
        .await?;
    Ok(())
}

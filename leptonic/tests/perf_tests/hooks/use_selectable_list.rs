// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_selectable_list` (`/perf/hooks/use_selectable_list`, on a plain
//! list): what each thing it offers costs, measured (`Page::measure`) and checked against its
//! budget (`tests/budgets/hooks/use_selectable_list.json`): mounting and unmounting the list,
//! keyboard focus coming in and leaving, keyboard navigation, select all and Escape, type-ahead,
//! and an orientation change, with 10 items and with 1,000.
//!
//! The performance tests of `use_selectable_collection`, `use_list_keyboard_delegate` and
//! `use_selectable_item` use the same fixture and the helpers here.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_selectable_list";

/// Registers the performance tests of `use_selectable_list`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(KeyboardNavigation))
        .with(fixture_test(KeyboardNavigationLarge))
        .with(fixture_test(SelectAllAndClear))
        .with(fixture_test(SelectAllAndClearLarge))
        .with(fixture_test(TypeAhead))
        .with(fixture_test(TypeAheadLarge))
        .with(fixture_test(Orientation))
}
pub(crate) const LIST: &str = "#perf-list";
pub(crate) const ITEMS: &str = "[data-perf-option]";
/// The fixture's items after the control `large`.
pub(crate) const LARGE: u32 = 1_000;

/// The item "Option `i`".
pub(crate) fn item(i: u32) -> String {
    format!("[data-perf-option='{i}']")
}

/// Switches to [`LARGE`] items (unmeasured).
pub(crate) async fn large(page: &Page<'_>) -> Result<(), Report> {
    page.control("large").await?;
    page.element(item(LARGE - 1)).await?;
    Ok(())
}

/// Focuses the first item with the keyboard (Tab from the plain button before the list).
pub(crate) async fn tab_in(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let first = page.element(item(0)).await?;
    page.wait_for_focus(&first).await?;
    Ok(first)
}

/// The `data-perf-option` of the focused element.
pub(crate) async fn focused_item(page: &Page<'_>) -> Result<Option<String>, Report> {
    Ok(page
        .focused_element()
        .await?
        .attr("data-perf-option")
        .await?)
}

/// Mounting the list with `count` items and unmounting it.
pub(crate) async fn mount_and_unmount_in(
    page: &Page<'_>,
    container: &str,
    count: usize,
) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(container, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(ITEMS, count).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(container, 0).await?;
    Ok(())
}

/// Mounting the list with 10 items and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_and_unmount_in(page, LIST, 10).await
}

/// Mounting the list with 1,000 items and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    mount_and_unmount_in(page, LIST, 1_000).await
}

/// Tab moving focus into the list (its first item) and on.
pub(crate) async fn keyboard_focus_and_blur_in(page: &Page<'_>) -> Result<(), Report> {
    page.element("#perf-before").await?.click().await?;
    let first = page.element(item(0)).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&first).await?;
    first.wait_for_attr("data-focused", Some("true")).await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&page.element("#perf-after").await?)
        .await?;
    Ok(())
}

/// Keyboard focus coming into the list and leaving it.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    keyboard_focus_and_blur_in(page).await
}

/// ArrowDown, ArrowUp, End, Home, PageDown and PageUp moving the focus, from the first item of
/// `last + 1`.
pub(crate) async fn keyboard_navigation_in(page: &Page<'_>, last: u32) -> Result<(), Report> {
    tab_in(page).await?;
    page.measure("arrow down", page.send_keys(Key::Down))
        .await?;
    page.wait_for_focus(&page.element(item(1)).await?).await?;
    page.measure("arrow up", page.send_keys(Key::Up)).await?;
    page.wait_for_focus(&page.element(item(0)).await?).await?;
    page.measure("end", page.send_keys(Key::End)).await?;
    page.wait_for_focus(&page.element(item(last)).await?)
        .await?;
    page.measure("home", page.send_keys(Key::Home)).await?;
    page.wait_for_focus(&page.element(item(0)).await?).await?;
    page.measure("page down", page.send_keys(Key::PageDown))
        .await?;
    assert_that!(focused_item(page).await?).is_not_equal_to(Some("0".to_owned()));
    page.measure("page up", page.send_keys(Key::PageUp)).await?;
    page.wait_for_focus(&page.element(item(0)).await?).await?;
    Ok(())
}

/// Keyboard navigation through 10 items.
#[browser_test]
pub async fn keyboard_navigation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    keyboard_navigation_in(page, 9).await
}

/// Keyboard navigation through 1,000 items.
#[browser_test]
pub async fn keyboard_navigation_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    keyboard_navigation_in(page, LARGE - 1).await
}

/// Ctrl+A selecting every item and Escape clearing the selection, from the first item of
/// `last + 1`.
pub(crate) async fn select_all_and_clear_in(page: &Page<'_>, last: u32) -> Result<(), Report> {
    tab_in(page).await?;
    let last = page.element(item(last)).await?;
    let select_all = page.primary_modifier().await? + "a";
    page.measure("select all", page.send_keys(select_all))
        .await?;
    last.wait_for_attr("aria-selected", Some("true")).await?;
    page.measure("escape", page.send_keys(Key::Escape)).await?;
    last.wait_for_attr("aria-selected", Some("false")).await?;
    Ok(())
}

/// Select all and clearing it with 10 items.
#[browser_test]
pub async fn select_all_and_clear(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    select_all_and_clear_in(page, 9).await
}

/// Select all and clearing it with 1,000 items.
#[browser_test]
pub async fn select_all_and_clear_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    select_all_and_clear_in(page, LARGE - 1).await
}

/// Type-ahead from the first item: "x" matches no item (a search through all of them), "o"
/// matches the focused item (the search starts at it). (Measured last: the search resets a second later.)
pub(crate) async fn type_ahead_in(page: &Page<'_>) -> Result<(), Report> {
    tab_in(page).await?;
    page.measure("no match", page.send_keys("x")).await?;
    assert_that!(focused_item(page).await?).is_equal_to(Some("0".to_owned()));
    page.measure("match", page.send_keys("o")).await?;
    assert_that!(focused_item(page).await?).is_equal_to(Some("0".to_owned()));
    Ok(())
}

/// Type-ahead through 10 items.
#[browser_test]
pub async fn type_ahead(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    type_ahead_in(page).await
}

/// Type-ahead through 1,000 items.
#[browser_test]
pub async fn type_ahead_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    type_ahead_in(page).await
}

/// Switching to a horizontal orientation (ArrowRight then moves the focus) and back (ArrowDown
/// moves it again).
pub(crate) async fn orientation_in(page: &Page<'_>) -> Result<(), Report> {
    page.measure("horizontal", page.control("horizontal"))
        .await?;
    page.element(LIST)
        .await?
        .wait_for_attr("aria-orientation", Some("horizontal"))
        .await?;
    tab_in(page).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&page.element(item(1)).await?).await?;
    page.measure("vertical", page.control("vertical")).await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&page.element(item(2)).await?).await?;
    Ok(())
}

/// An orientation change.
#[browser_test]
pub async fn orientation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    orientation_in(page).await
}

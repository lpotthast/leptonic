// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_listbox` (`/perf/hooks/use_listbox`, on a plain element): what
//! each thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_listbox.json`): mounting and unmounting the listbox, keyboard focus
//! coming in and leaving, keyboard navigation, select all and Escape, type-ahead, and changes of
//! its reactive input (orientation, label), with 10 options and with 1,000.
//!
//! `use_option`'s performance test uses the same fixture and the helpers here.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_listbox";

/// Registers the performance tests of `use_listbox`.
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
        .with(fixture_test(AriaLabel))
}
pub(crate) const LISTBOX: &str = "[role=listbox]";
pub(crate) const OPTIONS: &str = "[data-perf-option]";
/// The fixture's options after the control `large`.
pub(crate) const LARGE: u32 = 1_000;

/// The option "Option `i`".
pub(crate) fn option(i: u32) -> String {
    format!("[data-perf-option='{i}']")
}

/// Switches to [`LARGE`] options (unmeasured).
pub(crate) async fn large(page: &Page<'_>) -> Result<(), Report> {
    page.control("large").await?;
    page.element(option(LARGE - 1)).await?;
    Ok(())
}

/// Focuses the first option with the keyboard (Tab from the plain button before the listbox).
pub(crate) async fn tab_in(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let first = page.element(option(0)).await?;
    page.wait_for_focus(&first).await?;
    Ok(first)
}

/// The `data-perf-option` of the focused element.
pub(crate) async fn focused_option(page: &Page<'_>) -> Result<Option<String>, Report> {
    Ok(page
        .focused_element()
        .await?
        .attr("data-perf-option")
        .await?)
}

/// Mounting the listbox with `count` options and unmounting it.
async fn mount_and_unmount_in(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.control("unmount").await?;
    page.wait_for_count(LISTBOX, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(OPTIONS, count).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(LISTBOX, 0).await?;
    Ok(())
}

/// Mounting the listbox with 10 options and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    mount_and_unmount_in(page, 10).await
}

/// Mounting the listbox with 1,000 options and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    mount_and_unmount_in(page, 1_000).await
}

/// Tab moving focus into the listbox (its first option, with a focus ring) and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let first = page.element(option(0)).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    first
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&page.element("#perf-after").await?)
        .await?;
    first.wait_for_attr("data-focus-visible", None).await?;
    Ok(())
}

/// ArrowDown, ArrowUp, End, Home, PageDown and PageUp moving the focus, from the first option
/// of `last + 1`.
pub(crate) async fn keyboard_navigation_in(page: &Page<'_>, last: u32) -> Result<(), Report> {
    tab_in(page).await?;
    page.measure("arrow down", page.send_keys(Key::Down))
        .await?;
    page.wait_for_focus(&page.element(option(1)).await?).await?;
    page.measure("arrow up", page.send_keys(Key::Up)).await?;
    page.wait_for_focus(&page.element(option(0)).await?).await?;
    page.measure("end", page.send_keys(Key::End)).await?;
    page.wait_for_focus(&page.element(option(last)).await?)
        .await?;
    page.measure("home", page.send_keys(Key::Home)).await?;
    page.wait_for_focus(&page.element(option(0)).await?).await?;
    page.measure("page down", page.send_keys(Key::PageDown))
        .await?;
    assert_that!(focused_option(page).await?).is_not_equal_to(Some("0".to_owned()));
    page.measure("page up", page.send_keys(Key::PageUp)).await?;
    page.wait_for_focus(&page.element(option(0)).await?).await?;
    Ok(())
}

/// Keyboard navigation through 10 options.
#[browser_test]
pub async fn keyboard_navigation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    keyboard_navigation_in(page, 9).await
}

/// Keyboard navigation through 1,000 options.
#[browser_test]
pub async fn keyboard_navigation_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    keyboard_navigation_in(page, LARGE - 1).await
}

/// Ctrl+A selecting every option and Escape clearing the selection, from the first option of
/// `last + 1`.
pub(crate) async fn select_all_and_clear_in(page: &Page<'_>, last: u32) -> Result<(), Report> {
    tab_in(page).await?;
    let last = page.element(option(last)).await?;
    let select_all = page.primary_modifier().await? + "a";
    page.measure("select all", page.send_keys(select_all))
        .await?;
    last.wait_for_attr("aria-selected", Some("true")).await?;
    page.measure("escape", page.send_keys(Key::Escape)).await?;
    last.wait_for_attr("aria-selected", Some("false")).await?;
    Ok(())
}

/// Select all and clearing it with 10 options.
#[browser_test]
pub async fn select_all_and_clear(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    select_all_and_clear_in(page, 9).await
}

/// Select all and clearing it with 1,000 options.
#[browser_test]
pub async fn select_all_and_clear_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    select_all_and_clear_in(page, LARGE - 1).await
}

/// Type-ahead from the first option: "x" matches no option (a search through all of them), "o"
/// matches the focused option (the search starts at it). (Measured last: the search resets a second later.)
pub(crate) async fn type_ahead_in(page: &Page<'_>) -> Result<(), Report> {
    tab_in(page).await?;
    page.measure("no match", page.send_keys("x")).await?;
    assert_that!(focused_option(page).await?).is_equal_to(Some("0".to_owned()));
    page.measure("match", page.send_keys("o")).await?;
    assert_that!(focused_option(page).await?).is_equal_to(Some("0".to_owned()));
    Ok(())
}

/// Type-ahead through 10 options.
#[browser_test]
pub async fn type_ahead(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    type_ahead_in(page).await
}

/// Type-ahead through 1,000 options.
#[browser_test]
pub async fn type_ahead_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    large(page).await?;
    type_ahead_in(page).await
}

/// Switching to a horizontal orientation (ArrowRight then moves the focus) and back.
#[browser_test]
pub async fn orientation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let listbox = page.element(LISTBOX).await?;
    page.measure("horizontal", page.control("horizontal"))
        .await?;
    listbox
        .wait_for_attr("aria-orientation", Some("horizontal"))
        .await?;
    tab_in(page).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&page.element(option(1)).await?).await?;
    page.measure("vertical", page.control("vertical")).await?;
    listbox
        .wait_for_attr("aria-orientation", Some("vertical"))
        .await?;
    Ok(())
}

/// Changing the listbox's label.
#[browser_test]
pub async fn aria_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let listbox = page.element(LISTBOX).await?;
    page.measure("rename", page.control("rename")).await?;
    listbox.wait_for_attr("aria-label", Some("Choices")).await?;
    Ok(())
}

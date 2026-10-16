// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_selectable_collection`
//! (`/perf/hooks/use_selectable_collection`, the plain list of `use_selectable_list`'s performance
//! test with a `ListKeyboardDelegate` the fixture builds): what each thing it offers costs,
//! measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_selectable_collection.json`): mounting and unmounting the list,
//! keyboard focus coming in and leaving, keyboard navigation, select all and Escape, typing without
//! type-ahead (the delegate has no collator), and an orientation change, with 10 items and with
//! 1,000.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use super::use_selectable_list::{
    LIST, focused_item, keyboard_focus_and_blur_in, keyboard_navigation_in, large,
    mount_and_unmount_in, orientation_in, select_all_and_clear_in, tab_in,
};
use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/hooks/use_selectable_collection";

/// Registers the performance tests of `use_selectable_collection`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(KeyboardNavigation))
        .with(fixture_test(KeyboardNavigationLarge))
        .with(fixture_test(SelectAllAndClear))
        .with(fixture_test(SelectAllAndClearLarge))
        .with(fixture_test(TypingWithoutTypeAhead))
        .with(fixture_test(Orientation))
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

/// Keyboard focus coming into the list and leaving it.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    keyboard_focus_and_blur_in(page).await
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
    keyboard_navigation_in(page, 999).await
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
    select_all_and_clear_in(page, 999).await
}

/// A typed character: without a collator, the delegate finds nothing, the focus stays.
#[browser_test]
pub async fn typing_without_type_ahead(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    tab_in(page).await?;
    page.measure("type", page.send_keys("o")).await?;
    assert_that!(focused_item(page).await?).is_equal_to(Some("0".to_owned()));
    Ok(())
}

/// An orientation change (the delegate is rebuilt).
#[browser_test]
pub async fn orientation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    orientation_in(page).await
}

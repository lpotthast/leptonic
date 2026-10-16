// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_list_keyboard_delegate`
//! (`/perf/hooks/use_list_keyboard_delegate`, the delegate of `use_selectable_collection` on the
//! plain list of `use_selectable_list`'s performance test): what each thing it offers costs,
//! measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_list_keyboard_delegate.json`): mounting and unmounting the list
//! (creating the delegate), keyboard navigation and type-ahead (the delegate's searches), and an
//! orientation change (rebuilding it), with 10 items and with 1,000.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use super::use_selectable_list::{
    LIST, keyboard_navigation_in, large, mount_and_unmount_in, orientation_in, type_ahead_in,
};
use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/hooks/use_list_keyboard_delegate";

/// Registers the performance tests of `use_list_keyboard_delegate`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(KeyboardNavigation))
        .with(fixture_test(KeyboardNavigationLarge))
        .with(fixture_test(TypeAhead))
        .with(fixture_test(TypeAheadLarge))
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

/// An orientation change (the delegate is rebuilt).
#[browser_test]
pub async fn orientation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    orientation_in(page).await
}

// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_draggable_collection` (`/perf/hooks/use_draggable_collection`):
//! a grid list of 10 or 1,000 draggable rows (the fixture of `use_draggable_item`) recording
//! itself as the collection drags come from. Measured (`Page::measure`) and checked against its
//! budget (`tests/budgets/hooks/use_draggable_collection.json`): mounting and unmounting the list,
//! a native drag of a row (its start, a move, its end) and a keyboard drop outside the list.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::Page,
    perf_tests::hooks::use_draggable_item::{
        keyboard_drop_on_the_target, mount_and_unmount, native_drag,
    },
};

const PATH: &str = "/perf/hooks/use_draggable_collection";

/// Registers the performance tests of `use_draggable_collection`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmountSmall))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(NativeDragSmall))
        .with(fixture_test(NativeDragLarge))
        .with(fixture_test(KeyboardDrop))
}

/// Mounting and unmounting the list of 10 rows.
#[browser_test]
pub async fn mount_and_unmount_small(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount(page, PATH, 10).await
}

/// Mounting and unmounting the list of 1,000 rows.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount(page, PATH, 1_000).await
}

/// A native drag of the first of 10 rows.
#[browser_test]
pub async fn native_drag_small(page: &Page<'_>) -> Result<(), Report> {
    native_drag(page, PATH, 10).await
}

/// A native drag of the first of 1,000 rows.
#[browser_test]
pub async fn native_drag_large(page: &Page<'_>) -> Result<(), Report> {
    native_drag(page, PATH, 1_000).await
}

/// A keyboard drop of a row on the target outside the list.
#[browser_test]
pub async fn keyboard_drop(page: &Page<'_>) -> Result<(), Report> {
    keyboard_drop_on_the_target(page, PATH).await
}

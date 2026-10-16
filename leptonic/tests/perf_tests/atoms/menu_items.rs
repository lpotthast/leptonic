// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `MenuItems` (`/perf/atoms/menu_items`): what each thing it offers
//! costs, at 10 and at 1,000 items, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/menu_items.json`): mounting and unmounting the items of a menu, and an
//! item added to the collection and removed again.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/atoms/menu_items";
const ITEMS: &str = "[role=menu] .leptonic-MenuItem";

/// Registers the performance tests of `MenuItems`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmountSmall))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(AddAndRemoveSmall))
        .with(fixture_test(AddAndRemoveLarge))
}

/// The collection's sizes, with the control setting it.
#[derive(Clone, Copy)]
enum Size {
    /// 10 items.
    Small,
    /// 1,000 items.
    Large,
}

impl Size {
    fn items(self) -> usize {
        match self {
            Self::Small => 10,
            Self::Large => 1_000,
        }
    }

    fn control(self) -> &'static str {
        match self {
            Self::Small => "size-small",
            Self::Large => "size-large",
        }
    }
}

/// Mounting the items of a collection of `size` and unmounting them.
async fn mount_and_unmount(page: &Page<'_>, size: Size) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(ITEMS, 0).await?;
    page.control(size.control()).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(ITEMS, size.items()).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(ITEMS, 0).await?;
    Ok(())
}

/// Mounting and unmounting 10 items.
#[browser_test]
pub async fn mount_and_unmount_small(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount(page, Size::Small).await
}

/// Mounting and unmounting 1,000 items.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount(page, Size::Large).await
}

/// An item added to the end of a collection of `size` and removed again.
async fn add_and_remove(page: &Page<'_>, size: Size) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control(size.control()).await?;
    page.wait_for_count(ITEMS, size.items()).await?;
    page.measure("add", page.control("add")).await?;
    page.wait_for_count(ITEMS, size.items() + 1).await?;
    page.measure("remove", page.control("remove")).await?;
    page.wait_for_count(ITEMS, size.items()).await?;
    Ok(())
}

/// An item added to 10 and removed again.
#[browser_test]
pub async fn add_and_remove_small(page: &Page<'_>) -> Result<(), Report> {
    add_and_remove(page, Size::Small).await
}

/// An item added to 1,000 and removed again.
#[browser_test]
pub async fn add_and_remove_large(page: &Page<'_>) -> Result<(), Report> {
    add_and_remove(page, Size::Large).await
}

// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_menu` (`/perf/hooks/use_menu`): what each thing it offers
//! costs, at 10 and at 1,000 items, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_menu.json`): mounting and unmounting it (with its items), moving the
//! focus with the arrow keys, Home and End, type-ahead, and an action (Enter).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{Page, role},
};

const PATH: &str = "/perf/hooks/use_menu";
const MENU: &str = "ul[role=menu]";
const ITEMS: &str = "ul[role=menu] > li";

/// Registers the performance tests of `use_menu`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmountSmall))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(KeyboardNavigationSmall))
        .with(fixture_test(KeyboardNavigationLarge))
        .with(fixture_test(TypeAhead))
        .with(fixture_test(Action))
}

/// The menu's sizes, with the control mounting it.
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

    fn mount_control(self) -> &'static str {
        match self {
            Self::Small => "mount-small",
            Self::Large => "mount-large",
        }
    }
}

/// Loads the page with the menu of `size`, mounted.
async fn open(page: &Page<'_>, size: Size) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    if let Size::Large = size {
        page.control("unmount").await?;
        page.wait_for_count(MENU, 0).await?;
        page.control(size.mount_control()).await?;
    }
    page.wait_for_count(ITEMS, size.items()).await
}

/// The item with the text `text`.
async fn item(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.element(role(AriaRole::Menuitem).text(text)).await
}

/// Focuses the first item with the keyboard (Tab from the plain button before the menu).
async fn focus_first(page: &Page<'_>) -> Result<(), Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&item(page, "Item 0").await?).await
}

/// Mounting the menu of `size` and unmounting it.
async fn mount_and_unmount(page: &Page<'_>, size: Size) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(MENU, 0).await?;
    page.measure("mount", page.control(size.mount_control()))
        .await?;
    page.wait_for_count(ITEMS, size.items()).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(MENU, 0).await?;
    Ok(())
}

/// Mounting the menu of 10 items and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_small(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount(page, Size::Small).await
}

/// Mounting the menu of 1,000 items and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount(page, Size::Large).await
}

/// ArrowDown, ArrowUp, End and Home moving the focus through the menu of `size`.
async fn keyboard_navigation(page: &Page<'_>, size: Size) -> Result<(), Report> {
    open(page, size).await?;
    focus_first(page).await?;
    page.measure("arrow down", page.send_keys(Key::Down))
        .await?;
    page.wait_for_focus(&item(page, "Item 1").await?).await?;
    page.measure("arrow up", page.send_keys(Key::Up)).await?;
    page.wait_for_focus(&item(page, "Item 0").await?).await?;
    page.measure("end", page.send_keys(Key::End)).await?;
    page.wait_for_focus(&item(page, "Zebra").await?).await?;
    page.measure("home", page.send_keys(Key::Home)).await?;
    page.wait_for_focus(&item(page, "Item 0").await?).await?;
    Ok(())
}

/// Keyboard navigation in the menu of 10 items.
#[browser_test]
pub async fn keyboard_navigation_small(page: &Page<'_>) -> Result<(), Report> {
    keyboard_navigation(page, Size::Small).await
}

/// Keyboard navigation in the menu of 1,000 items.
#[browser_test]
pub async fn keyboard_navigation_large(page: &Page<'_>) -> Result<(), Report> {
    keyboard_navigation(page, Size::Large).await
}

/// Type-ahead in the menu of 1,000 items: typing "z" moves the focus from the first item to the
/// last one, the only one starting with it (one key: the typed text resets after a timeout).
#[browser_test]
pub async fn type_ahead(page: &Page<'_>) -> Result<(), Report> {
    open(page, Size::Large).await?;
    focus_first(page).await?;
    page.measure("type", page.send_keys("z")).await?;
    page.wait_for_focus(&item(page, "Zebra").await?).await?;
    Ok(())
}

/// Enter on the focused item: its action.
#[browser_test]
pub async fn action(page: &Page<'_>) -> Result<(), Report> {
    open(page, Size::Small).await?;
    focus_first(page).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    assert_that!(page.control("actions").await?).is_equal_to(1.0);
    Ok(())
}

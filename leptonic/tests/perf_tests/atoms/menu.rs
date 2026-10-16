// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `Menu` (`/perf/atoms/menu`): what each thing it offers costs, at
//! 10 and at 1,000 items, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/menu.json`): mounting and unmounting it, moving the focus with the arrow
//! keys, Home and End, type-ahead, the pointer focusing the items it hovers, actions by click and
//! by Enter, single selection, and a click on a disabled item (ignored).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page, role},
};

const PATH: &str = "/perf/atoms/menu";
const MENU: &str = "[role=menu]";
const ITEMS: &str = "[role=menu] .leptonic-MenuItem";

/// Registers the performance tests of `Menu`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmountSmall))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(KeyboardNavigationSmall))
        .with(fixture_test(KeyboardNavigationLarge))
        .with(fixture_test(TypeAhead))
        .with(fixture_test(HoverFocuses))
        .with(fixture_test(Actions))
        .with(fixture_test(SelectionSmall))
        .with(fixture_test(SelectionLarge))
        .with(fixture_test(IgnoredDisabledItem))
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

/// The pointer hovering an item (focusing it) and the next one.
#[browser_test]
pub async fn hover_focuses(page: &Page<'_>) -> Result<(), Report> {
    open(page, Size::Small).await?;
    // Hovering focuses with the pointer's modality: a click first.
    page.element("#perf-before").await?.click().await?;
    let first = item(page, "Item 0").await?;
    let second = item(page, "Item 1").await?;
    page.measure("hover", first.hover()).await?;
    page.wait_for_focus(&first).await?;
    page.measure("hover next", second.hover()).await?;
    page.wait_for_focus(&second).await?;
    Ok(())
}

/// A click on an item and Enter on the focused one: actions.
#[browser_test]
pub async fn actions(page: &Page<'_>) -> Result<(), Report> {
    open(page, Size::Small).await?;
    let second = item(page, "Item 1").await?;
    page.measure("click", second.click()).await?;
    assert_that!(page.control("actions").await?).is_equal_to(1.0);
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    assert_that!(page.control("actions").await?).is_equal_to(2.0);
    Ok(())
}

/// With single selection: clicks selecting an item, then another one, in the menu of `size`.
async fn selection(page: &Page<'_>, size: Size) -> Result<(), Report> {
    open(page, size).await?;
    page.measure("single selection", page.control("single-selection"))
        .await?;
    // With single selection, the items are radio items.
    let radio = |text: &'static str| page.element(role(AriaRole::Menuitemradio).text(text));
    let first = radio("Item 0").await?;
    let second = radio("Item 1").await?;
    page.measure("select", first.click()).await?;
    first.wait_for_attr("aria-checked", Some("true")).await?;
    page.measure("select another", second.click()).await?;
    second.wait_for_attr("aria-checked", Some("true")).await?;
    first.wait_for_attr("aria-checked", Some("false")).await?;
    page.measure("no selection", page.control("no-selection"))
        .await?;
    Ok(())
}

/// Single selection in the menu of 10 items.
#[browser_test]
pub async fn selection_small(page: &Page<'_>) -> Result<(), Report> {
    selection(page, Size::Small).await
}

/// Single selection in the menu of 1,000 items.
#[browser_test]
pub async fn selection_large(page: &Page<'_>) -> Result<(), Report> {
    selection(page, Size::Large).await
}

/// Disabling the first item, a click on it (ignored), and enabling it again.
#[browser_test]
pub async fn ignored_disabled_item(page: &Page<'_>) -> Result<(), Report> {
    open(page, Size::Small).await?;
    let first = item(page, "Item 0").await?;
    page.measure("disable", page.control("disable-first"))
        .await?;
    first.wait_for_attr("aria-disabled", Some("true")).await?;
    page.measure("ignored click", first.click()).await?;
    assert_that!(page.control("actions").await?).is_equal_to(0.0);
    page.measure("enable", page.control("enable-first")).await?;
    first.wait_for_attr("aria-disabled", None).await?;
    Ok(())
}

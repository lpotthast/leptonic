// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `GridListItem` (`/perf/atoms/grid_list_item`): what each thing it
//! offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/grid_list_item.json`): an item mounting and unmounting in its list,
//! selecting and deselecting it with clicks and with Space, its pressed and hovered states,
//! keyboard focus, the arrow keys into its child (focus visible within) and back, its action
//! (click and Enter), and a click on a disabled item, in lists of 10 items and of 1,000.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/grid_list_item";

/// Registers the performance tests of `GridListItem`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(ClickSmall))
        .with(fixture_test(ClickLarge))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(SpaceSmall))
        .with(fixture_test(SpaceLarge))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(ArrowIntoChild))
        .with(fixture_test(ItemAction))
        .with(fixture_test(DisabledItemIgnoresClick))
}

const LIST: &str = "[role=grid]";
const ITEMS: &str = "[role=grid] > [role=row]";

/// Item `i` (from 0).
fn item(i: usize) -> String {
    format!("[role=grid] > [role=row]:nth-child({})", i + 1)
}

/// Opens the fixture with its 10 items.
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ITEMS, 10).await?;
    Ok(())
}

/// Opens the fixture and mounts the list with 1,000 items (unmeasured).
async fn open_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(LIST, 0).await?;
    page.control("large").await?;
    page.control("mount").await?;
    page.wait_for_count(ITEMS, 1_000).await?;
    Ok(())
}

/// Creates the live announcer's regions: its first announcement (of a selection change while
/// the list has focus) waits 100 ms for them, which would fall into a later measurement.
async fn prepare_announcer(page: &Page<'_>) -> Result<(), Report> {
    page.control("prepare-announcer").await?;
    page.wait_for_count("[data-live-announcer] [aria-live=polite] > div", 1)
        .await?;
    Ok(())
}

/// How many items are selected.
async fn selected(page: &Page<'_>) -> Result<f64, Report> {
    page.control("selected").await
}

/// An item added at the end of the list's `items` and removed again.
async fn mount_and_unmount_item(page: &Page<'_>, items: usize) -> Result<(), Report> {
    page.measure("mount", page.control("add-item")).await?;
    page.wait_for_count(ITEMS, items + 1).await?;
    page.measure("unmount", page.control("remove-item")).await?;
    page.wait_for_count(ITEMS, items).await?;
    Ok(())
}

/// An item mounting in a list of 10 items and unmounting.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    mount_and_unmount_item(page, 10).await
}

/// An item mounting in a list of 1,000 items and unmounting.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    mount_and_unmount_item(page, 1_000).await
}

/// A click selecting the second item, and one deselecting it.
async fn click(page: &Page<'_>) -> Result<(), Report> {
    prepare_announcer(page).await?;
    let second = page.element(item(1)).await?;
    page.measure("select", second.click()).await?;
    second.wait_for_attr("data-selected", Some("true")).await?;
    page.measure("deselect", second.click()).await?;
    second.wait_for_attr("data-selected", None).await?;
    assert_that!(selected(page).await?).is_equal_to(0.0);
    Ok(())
}

/// Clicks selecting and deselecting an item of 10.
#[browser_test]
pub async fn click_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    click(page).await
}

/// Clicks selecting and deselecting an item of 1,000.
#[browser_test]
pub async fn click_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    click(page).await
}

/// Holding the pointer down on an item (the pressed state; it selects the item) and releasing it.
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    prepare_announcer(page).await?;
    let second = page.element(item(1)).await?;
    let held = page.measure("hold", second.press_and_hold()).await?;
    second.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("release", held.release()).await?;
    second.wait_for_attr("data-pressed", None).await?;
    second.wait_for_attr("data-selected", Some("true")).await?;
    Ok(())
}

/// The pointer entering and leaving an item.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let second = page.element(item(1)).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", second.hover()).await?;
    second.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    second.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Space selecting the focused first item and deselecting it.
async fn space(page: &Page<'_>) -> Result<(), Report> {
    prepare_announcer(page).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let first = page.element(item(0)).await?;
    page.wait_for_focus(&first).await?;
    page.measure("select", page.send_keys(" ")).await?;
    first.wait_for_attr("data-selected", Some("true")).await?;
    page.measure("deselect", page.send_keys(" ")).await?;
    first.wait_for_attr("data-selected", None).await?;
    Ok(())
}

/// Space selecting and deselecting an item of 10.
#[browser_test]
pub async fn space_small(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    space(page).await
}

/// Space selecting and deselecting an item of 1,000.
#[browser_test]
pub async fn space_large(page: &Page<'_>) -> Result<(), Report> {
    open_large(page).await?;
    space(page).await
}

/// Tab moving focus to the first item (`data-focus-visible`) and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element("#perf-before").await?.click().await?;
    let first = page.element(item(0)).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    first
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    first.wait_for_attr("data-focus-visible", None).await?;
    Ok(())
}

/// ArrowRight from the focused first item to its child (the button "Edit 0"), ArrowLeft back.
#[browser_test]
pub async fn arrow_into_child(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let first = page.element(item(0)).await?;
    page.wait_for_focus(&first).await?;
    page.measure("arrow right", page.send_keys(Key::Right))
        .await?;
    page.wait_for_focus(&first.element("button").await?).await?;
    first
        .wait_for_attr("data-focus-visible-within", Some("true"))
        .await?;
    page.measure("arrow left", page.send_keys(Key::Left))
        .await?;
    page.wait_for_focus(&first).await?;
    Ok(())
}

/// The action (a list with `on_action`): Enter on the focused first item, and a click on
/// the second (nothing is selected, so a click performs the action).
#[browser_test]
pub async fn item_action(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(LIST, 0).await?;
    page.control("mount-with-actions").await?;
    page.wait_for_count(ITEMS, 10).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(item(0)).await?).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    assert_that!(page.control("actions").await?).is_equal_to(1.0);
    let second = page.element(item(1)).await?;
    page.measure("click", second.click()).await?;
    assert_that!(page.control("actions").await?).is_equal_to(2.0);
    assert_that!(selected(page).await?).is_equal_to(0.0);
    Ok(())
}

/// A click on a disabled item, which it ignores.
#[browser_test]
pub async fn disabled_item_ignores_click(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("disable-second").await?;
    let second = page.element(item(1)).await?;
    second.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("ignored click", second.click()).await?;
    assert_that!(selected(page).await?).is_equal_to(0.0);
    Ok(())
}

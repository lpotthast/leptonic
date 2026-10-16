// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `MenuItem` (`/perf/atoms/menu_item`): what each thing it offers
//! costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/menu_item.json`): mounting and unmounting it, a click (its action), its
//! pressed state, the pointer hovering it (focusing it) and leaving, Enter and Space on it while
//! focused, selecting it, and a click while disabled (ignored).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page, role},
};

const PATH: &str = "/perf/atoms/menu_item";

/// Registers the performance tests of `MenuItem`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(Click))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(Keyboard))
        .with(fixture_test(Selection))
        .with(fixture_test(IgnoredWhileDisabled))
}

/// The item with the text `text`.
async fn item(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.element(role(AriaRole::Menuitem).text(text)).await
}

/// How many actions the menu reported.
async fn actions(page: &Page<'_>) -> Result<f64, Report> {
    page.control("actions").await
}

/// Mounting the item in its menu and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(".leptonic-MenuItem", 2).await?;
    page.measure("mount", page.control("mount")).await?;
    item(page, "Item 1").await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(".leptonic-MenuItem", 2).await?;
    Ok(())
}

/// A click: the item's action.
#[browser_test]
pub async fn click(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = item(page, "Item 1").await?;
    page.measure("click", target.click()).await?;
    assert_that!(actions(page).await?).is_equal_to(1.0);
    Ok(())
}

/// Holding the pointer down on the item (its pressed state) and releasing it (its action).
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = item(page, "Item 1").await?;
    let held = page.measure("hold", target.press_and_hold()).await?;
    target.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("release", held.release()).await?;
    target.wait_for_attr("data-pressed", None).await?;
    assert_that!(actions(page).await?).is_equal_to(1.0);
    Ok(())
}

/// The pointer entering the item (hovering and focusing it) and leaving it for the next item.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    // Hovering focuses with the pointer's modality: a click first.
    page.element("#perf-before").await?.click().await?;
    let target = item(page, "Item 1").await?;
    let next = item(page, "Item 2").await?;
    page.measure("hover", target.hover()).await?;
    target.wait_for_attr("data-hovered", Some("true")).await?;
    page.wait_for_focus(&target).await?;
    page.measure("leave", next.hover()).await?;
    target.wait_for_attr("data-hovered", None).await?;
    target.wait_for_attr("data-focused", None).await?;
    Ok(())
}

/// The focus arriving with ArrowDown (a focus ring), then Enter and Space: its actions.
#[browser_test]
pub async fn keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&item(page, "Item 0").await?).await?;
    let target = item(page, "Item 1").await?;
    page.measure("focus", page.send_keys(Key::Down)).await?;
    target
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    page.measure("space", page.send_keys(" ")).await?;
    assert_that!(actions(page).await?).is_equal_to(2.0);
    Ok(())
}

/// With single selection: a click selecting the item and one deselecting it.
#[browser_test]
pub async fn selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("single-selection").await?;
    // With single selection, the item is a radio item.
    let target = page
        .element(role(AriaRole::Menuitemradio).text("Item 1"))
        .await?;
    target.wait_for_attr("aria-checked", Some("false")).await?;
    page.measure("select", target.click()).await?;
    target.wait_for_attr("aria-checked", Some("true")).await?;
    target.wait_for_attr("data-selected", Some("true")).await?;
    page.measure("deselect", target.click()).await?;
    target.wait_for_attr("aria-checked", Some("false")).await?;
    Ok(())
}

/// Disabling the item, a click on it (ignored), and enabling it again.
#[browser_test]
pub async fn ignored_while_disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let target = item(page, "Item 1").await?;
    page.measure("disable", page.control("disable")).await?;
    target.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("ignored click", target.click()).await?;
    assert_that!(actions(page).await?).is_equal_to(0.0);
    page.measure("enable", page.control("enable")).await?;
    target.wait_for_attr("data-disabled", None).await?;
    Ok(())
}

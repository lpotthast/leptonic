// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `SubmenuTrigger` (`/perf/atoms/submenu_trigger`): what each thing
//! it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/submenu_trigger.json`): mounting and unmounting it, the pointer opening
//! its submenu (hovering, without delay; a click) and closing it (hovering another item), and the
//! keyboard opening it (ArrowRight, Enter) and closing it (ArrowLeft, Escape).
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page, role},
};

const PATH: &str = "/perf/atoms/submenu_trigger";
const MENUS: &str = "[role=menu]";

/// Registers the performance tests of `SubmenuTrigger`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(HoverOpensAndCloses))
        .with(fixture_test(ClickOpens))
        .with(fixture_test(ArrowKeys))
        .with(fixture_test(EnterAndEscape))
}

/// The item with the text `text`.
async fn item(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.element(role(AriaRole::Menuitem).text(text)).await
}

/// Waits until the submenu is open (placed, done entering) and the trigger expanded.
async fn wait_for_open(page: &Page<'_>) -> Result<(), Report> {
    page.wait_for_count(MENUS, 2).await?;
    page.element(".leptonic-Popover[data-placement]").await?;
    page.wait_for_count(".leptonic-Popover[data-entering]", 0)
        .await?;
    item(page, "Share")
        .await?
        .wait_for_attr("aria-expanded", Some("true"))
        .await
}

/// Waits until the submenu is closed and the trigger collapsed.
async fn wait_for_closed(page: &Page<'_>) -> Result<(), Report> {
    page.wait_for_count(MENUS, 1).await?;
    item(page, "Share")
        .await?
        .wait_for_attr("aria-expanded", Some("false"))
        .await
}

/// Focuses the trigger item with the keyboard (Tab into the menu).
async fn focus_trigger(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let share = item(page, "Share").await?;
    page.wait_for_focus(&share).await?;
    Ok(share)
}

/// Mounting the trigger (with its item, closed) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(".leptonic-MenuItem", 2).await?;
    page.measure("mount", page.control("mount")).await?;
    wait_for_closed(page).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(".leptonic-MenuItem", 2).await?;
    Ok(())
}

/// The pointer hovering the trigger (opening the submenu) and the next item (closing it).
#[browser_test]
pub async fn hover_opens_and_closes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    // Hovering focuses with the pointer's modality: a click first.
    page.element("#perf-before").await?.click().await?;
    let share = item(page, "Share").await?;
    page.measure("hover", share.hover()).await?;
    wait_for_open(page).await?;
    let copy = item(page, "Copy").await?;
    page.measure("hover another item", copy.hover()).await?;
    wait_for_closed(page).await?;
    Ok(())
}

/// A click on the trigger opening the submenu (the pointer arriving opens it, the press keeps it
/// open).
#[browser_test]
pub async fn click_opens(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.hover().await?;
    let share = item(page, "Share").await?;
    page.measure("click", share.click()).await?;
    wait_for_open(page).await?;
    Ok(())
}

/// ArrowRight opening the submenu (its first item focused) and ArrowLeft closing it (the focus
/// back on the trigger).
#[browser_test]
pub async fn arrow_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let share = focus_trigger(page).await?;
    page.measure("arrow right", page.send_keys(Key::Right))
        .await?;
    wait_for_open(page).await?;
    page.wait_for_focus(&item(page, "Email").await?).await?;
    page.measure("arrow left", page.send_keys(Key::Left))
        .await?;
    wait_for_closed(page).await?;
    page.wait_for_focus(&share).await?;
    Ok(())
}

/// Enter opening the submenu and Escape closing it (the focus back on the trigger).
#[browser_test]
pub async fn enter_and_escape(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let share = focus_trigger(page).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    wait_for_open(page).await?;
    page.wait_for_focus(&item(page, "Email").await?).await?;
    page.measure("escape", page.send_keys(Key::Escape)).await?;
    wait_for_closed(page).await?;
    page.wait_for_focus(&share).await?;
    Ok(())
}

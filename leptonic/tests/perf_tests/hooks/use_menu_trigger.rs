// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_menu_trigger` (`/perf/hooks/use_menu_trigger`): what each thing
//! it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_menu_trigger.json`): mounting and unmounting it, a mouse press opening
//! the menu (on pointer down), Enter opening it (the first item to focus) and closing it again,
//! and ArrowUp opening it (the last item to focus).
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_menu_trigger";
const TRIGGER: &str = "button[aria-haspopup]";
const MENU: &str = "[role=menu]";
const STRATEGY: &str = "#perf-focus-strategy";

/// Registers the performance tests of `use_menu_trigger`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(PressOpens))
        .with(fixture_test(EnterOpensAndCloses))
        .with(fixture_test(ArrowUpOpens))
}

/// Focuses the trigger (a click on the plain button before it, then Tab).
async fn focus_trigger(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let trigger = page.element(TRIGGER).await?;
    page.wait_for_focus(&trigger).await?;
    Ok(trigger)
}

/// Mounting the trigger (closed) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(TRIGGER, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(TRIGGER)
        .await?
        .wait_for_attr("aria-expanded", Some("false"))
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TRIGGER, 0).await?;
    Ok(())
}

/// A click on the trigger opening the menu (on pointer down; no focus strategy).
#[browser_test]
pub async fn press_opens(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(TRIGGER).await?;
    page.measure("click", trigger.click()).await?;
    trigger.wait_for_attr("aria-expanded", Some("true")).await?;
    page.element(MENU).await?;
    page.element(STRATEGY)
        .await?
        .wait_for_inner_text("")
        .await?;
    Ok(())
}

/// Enter on the focused trigger opening the menu (focusing its first item) and closing it again.
#[browser_test]
pub async fn enter_opens_and_closes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = focus_trigger(page).await?;
    page.measure("open", page.send_keys(Key::Enter)).await?;
    trigger.wait_for_attr("aria-expanded", Some("true")).await?;
    page.element(STRATEGY)
        .await?
        .wait_for_inner_text("first")
        .await?;
    page.measure("close", page.send_keys(Key::Enter)).await?;
    trigger
        .wait_for_attr("aria-expanded", Some("false"))
        .await?;
    page.wait_for_count(MENU, 0).await?;
    Ok(())
}

/// ArrowUp on the focused trigger opening the menu (focusing its last item).
#[browser_test]
pub async fn arrow_up_opens(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = focus_trigger(page).await?;
    page.measure("arrow up", page.send_keys(Key::Up)).await?;
    trigger.wait_for_attr("aria-expanded", Some("true")).await?;
    page.element(STRATEGY)
        .await?
        .wait_for_inner_text("last")
        .await?;
    Ok(())
}

// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `MenuTrigger` (`/perf/atoms/menu_trigger`): what each thing it
//! offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/menu_trigger.json`): mounting and unmounting it, opening its menu with the
//! mouse (the menu focused), with Enter (the first item focused) and ArrowUp (the last), and by
//! the app's state, and closing it by an action, Escape, a click outside and the app's state (the
//! focus returning to the trigger).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page, role},
};

const PATH: &str = "/perf/atoms/menu_trigger";
const MENU: &str = "[role=menu]";
const TRIGGER: &str = "#perf-trigger";

/// Registers the performance tests of `MenuTrigger`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(OpenWithTheMouse))
        .with(fixture_test(OpenWithTheKeyboard))
        .with(fixture_test(OutsideClickCloses))
        .with(fixture_test(OpenAndCloseByTheApp))
}

/// The item with the text `text`.
async fn item(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.element(role(AriaRole::Menuitem).text(text)).await
}

/// Focuses the trigger (a click on the plain button before it, then Tab).
async fn focus_trigger(page: &Page<'_>) -> Result<(), Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(TRIGGER).await?).await
}

/// Waits until the menu is open (the trigger expanded) and placed.
async fn wait_for_open(page: &Page<'_>) -> Result<WebElement, Report> {
    let menu = page.element(MENU).await?;
    page.element(".leptonic-Popover[data-placement]").await?;
    page.wait_for_count(".leptonic-Popover[data-entering]", 0)
        .await?;
    page.element(TRIGGER)
        .await?
        .wait_for_attr("aria-expanded", Some("true"))
        .await?;
    Ok(menu)
}

/// Waits until the menu is closed and the trigger has the focus back.
async fn wait_for_closed(page: &Page<'_>) -> Result<(), Report> {
    page.wait_for_count(MENU, 0).await?;
    let trigger = page.element(TRIGGER).await?;
    trigger
        .wait_for_attr("aria-expanded", Some("false"))
        .await?;
    page.wait_for_focus(&trigger).await
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

/// A click on the trigger opening the menu (focusing it), and a click on an item (its action)
/// closing it.
#[browser_test]
pub async fn open_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(TRIGGER).await?;
    page.measure("click", trigger.click()).await?;
    let menu = wait_for_open(page).await?;
    page.wait_for_focus(&menu).await?;
    let second = item(page, "Item 1").await?;
    page.measure("action", second.click()).await?;
    wait_for_closed(page).await?;
    assert_that!(page.control("actions").await?).is_equal_to(1.0);
    Ok(())
}

/// Enter opening the menu (its first item focused), Escape closing it, and ArrowUp opening it
/// again (its last item focused).
#[browser_test]
pub async fn open_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_trigger(page).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    wait_for_open(page).await?;
    page.wait_for_focus(&item(page, "Item 0").await?).await?;
    page.measure("escape", page.send_keys(Key::Escape)).await?;
    wait_for_closed(page).await?;
    page.measure("arrow up", page.send_keys(Key::Up)).await?;
    wait_for_open(page).await?;
    page.wait_for_focus(&item(page, "Item 9").await?).await?;
    Ok(())
}

/// A click outside the open menu closing it.
#[browser_test]
pub async fn outside_click_closes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_trigger(page).await?;
    page.send_keys(Key::Enter).await?;
    wait_for_open(page).await?;
    page.measure("outside click", page.click_at(5, 5)).await?;
    page.wait_for_count(MENU, 0).await?;
    page.element(TRIGGER)
        .await?
        .wait_for_attr("aria-expanded", Some("false"))
        .await?;
    Ok(())
}

/// The app opening the menu (the trigger focused before) and closing it.
#[browser_test]
pub async fn open_and_close_by_the_app(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_trigger(page).await?;
    page.measure("open", page.control("open")).await?;
    wait_for_open(page).await?;
    page.measure("close", page.control("close")).await?;
    wait_for_closed(page).await?;
    Ok(())
}

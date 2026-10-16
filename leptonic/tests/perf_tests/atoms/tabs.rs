// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `Tabs` (`/perf/atoms/tabs`, with a `TabList`, its `Tab`s and
//! their `TabPanel`s): what each thing it offers costs, measured (`Page::measure`) and checked
//! against its budget (`tests/budgets/atoms/tabs.json`): mounting and unmounting 3 and 100 tabs,
//! selecting a tab with the mouse and with the arrow keys, Home and End (at both sizes), keyboard
//! focus, and changes of its reactive props (the selected key, disabled, orientation, disabled
//! keys).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/tabs";

/// Registers the performance tests of `Tabs`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountMany))
        .with(fixture_test(SelectWithTheMouse))
        .with(fixture_test(SelectWithTheMouseMany))
        .with(fixture_test(SelectWithTheKeyboard))
        .with(fixture_test(SelectWithTheKeyboardMany))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(SelectProgrammatically))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(ChangeTheOrientation))
        .with(fixture_test(DisableATab))
}
const TABS: &str = "#perf-tabs .leptonic-Tabs";
const TAB_LIST: &str = "#perf-tabs [role=tablist]";
const ALL_TABS: &str = "#perf-tabs [role=tab]";
const PANEL: &str = "#perf-tabs [role=tabpanel]";

/// The `n`th tab (from 1).
fn tab(n: usize) -> String {
    format!("{TAB_LIST} > :nth-child({n})")
}

/// How often the selection changed.
async fn changes(page: &Page<'_>) -> Result<f64, Report> {
    page.control("changes").await
}

/// Opens the fixture with 100 tabs.
async fn goto_many(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(TABS, 0).await?;
    page.control("use-many").await?;
    page.control("mount").await?;
    page.wait_for_count(ALL_TABS, 100).await?;
    Ok(())
}

/// Waits until the `n`th tab is selected and its panel shows.
async fn wait_for_selected(page: &Page<'_>, n: usize) -> Result<(), Report> {
    page.wait_for_attr(tab(n), "aria-selected", Some("true"))
        .await?;
    page.element(PANEL)
        .await?
        .wait_for_inner_text(&format!("Panel {n}"))
        .await?;
    Ok(())
}

/// Focuses the first tab with the keyboard (Tab from the plain button before it).
async fn focus_with_tab(page: &Page<'_>) -> Result<(), Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let first = page.element(tab(1)).await?;
    page.wait_for_focus(&first).await?;
    Ok(())
}

/// Mounting 3 tabs with their panels and unmounting them.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(TABS, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(ALL_TABS, 3).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TABS, 0).await?;
    Ok(())
}

/// Mounting 100 tabs with their panels and unmounting them.
#[browser_test]
pub async fn mount_and_unmount_many(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(TABS, 0).await?;
    page.control("use-many").await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(ALL_TABS, 100).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TABS, 0).await?;
    Ok(())
}

/// A click on the second of 3 tabs: it is selected, its panel replaces the first's.
#[browser_test]
pub async fn select_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let second = page.element(tab(2)).await?;
    page.measure("click", second.click()).await?;
    wait_for_selected(page, 2).await?;
    assert_that!(changes(page).await?).is_equal_to(1.0);
    Ok(())
}

/// A click on the second of 100 tabs.
#[browser_test]
pub async fn select_with_the_mouse_many(page: &Page<'_>) -> Result<(), Report> {
    goto_many(page).await?;
    let second = page.element(tab(2)).await?;
    page.measure("click", second.click()).await?;
    wait_for_selected(page, 2).await?;
    assert_that!(changes(page).await?).is_equal_to(1.0);
    Ok(())
}

/// The arrow keys, End and Home moving the selection through 3 tabs (automatic activation).
#[browser_test]
pub async fn select_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_with_tab(page).await?;
    page.measure("arrow right", page.send_keys(Key::Right))
        .await?;
    wait_for_selected(page, 2).await?;
    page.measure("arrow left", page.send_keys(Key::Left))
        .await?;
    wait_for_selected(page, 1).await?;
    page.measure("end", page.send_keys(Key::End)).await?;
    wait_for_selected(page, 3).await?;
    page.measure("home", page.send_keys(Key::Home)).await?;
    wait_for_selected(page, 1).await?;
    assert_that!(changes(page).await?).is_equal_to(4.0);
    Ok(())
}

/// The arrow keys, End and Home moving the selection through 100 tabs.
#[browser_test]
pub async fn select_with_the_keyboard_many(page: &Page<'_>) -> Result<(), Report> {
    goto_many(page).await?;
    focus_with_tab(page).await?;
    page.measure("arrow right", page.send_keys(Key::Right))
        .await?;
    wait_for_selected(page, 2).await?;
    page.measure("arrow left", page.send_keys(Key::Left))
        .await?;
    wait_for_selected(page, 1).await?;
    page.measure("end", page.send_keys(Key::End)).await?;
    wait_for_selected(page, 100).await?;
    page.measure("home", page.send_keys(Key::Home)).await?;
    wait_for_selected(page, 1).await?;
    assert_that!(changes(page).await?).is_equal_to(4.0);
    Ok(())
}

/// Tab moving focus into the tabs (the selected tab), and Shift+Tab out of them again.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    page.wait_for_attr(TABS, "data-focused", Some("true"))
        .await?;
    page.measure("blur", page.send_keys(Key::Shift + Key::Tab))
        .await?;
    page.wait_for_attr(TABS, "data-focused", None).await?;
    Ok(())
}

/// Selecting the second tab through the bound key, and the first again.
#[browser_test]
pub async fn select_programmatically(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("select the second", page.control("select-second"))
        .await?;
    wait_for_selected(page, 2).await?;
    page.measure("select the first", page.control("select-first"))
        .await?;
    wait_for_selected(page, 1).await?;
    Ok(())
}

/// Disabling all tabs, a click they ignore, and enabling them again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("disable", page.control("disable")).await?;
    page.wait_for_attr(TABS, "data-disabled", Some("true"))
        .await?;
    page.wait_for_attr(tab(2), "aria-disabled", Some("true"))
        .await?;
    let second = page.element(tab(2)).await?;
    page.measure("ignored click", second.click()).await?;
    assert_that!(changes(page).await?).is_equal_to(0.0);
    page.measure("enable", page.control("enable")).await?;
    page.wait_for_attr(TABS, "data-disabled", None).await?;
    page.wait_for_attr(tab(2), "aria-disabled", None).await?;
    Ok(())
}

/// Making the tabs vertical and horizontal again.
#[browser_test]
pub async fn change_the_orientation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("vertical", page.control("make-vertical"))
        .await?;
    page.wait_for_attr(TAB_LIST, "aria-orientation", Some("vertical"))
        .await?;
    page.measure("horizontal", page.control("make-horizontal"))
        .await?;
    page.wait_for_attr(TAB_LIST, "aria-orientation", Some("horizontal"))
        .await?;
    Ok(())
}

/// Disabling the second tab (`disabled_keys`), a click it ignores, and enabling it again.
#[browser_test]
pub async fn disable_a_tab(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("disable", page.control("disable-second"))
        .await?;
    page.wait_for_attr(tab(2), "aria-disabled", Some("true"))
        .await?;
    let second = page.element(tab(2)).await?;
    page.measure("ignored click", second.click()).await?;
    assert_that!(changes(page).await?).is_equal_to(0.0);
    page.measure("enable", page.control("enable-second"))
        .await?;
    page.wait_for_attr(tab(2), "aria-disabled", None).await?;
    Ok(())
}

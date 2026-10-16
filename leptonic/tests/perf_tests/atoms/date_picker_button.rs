// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `DatePickerButton` (`/perf/atoms/date_picker_button`, the button
//! of a date picker opening on June 2024): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/date_picker_button.json`): mounting and unmounting it, opening the
//! picker with a click and with Enter (pressed while open), holding and releasing the pointer,
//! hover, keyboard focus, and a click it ignores while the picker is disabled.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/date_picker_button";

/// Registers the performance tests of `DatePickerButton`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(PressWithTheMouse))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(PressWithTheKeyboard))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(DisableAndEnable))
}
const BUTTON: &str = ".leptonic-DatePickerButton";
const DIALOG: &str = "[role=dialog]";
/// The entry the page's first announcement leaves in the live announcer.
const ANNOUNCER_READY: &str = "[data-live-announcer] [aria-live=polite] > div";

/// Loads the page and waits until the live announcer exists (the page creates it on load; the
/// first announcement of a page waits 100 ms for it).
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ANNOUNCER_READY, 1).await
}

/// Whether the picker is open.
async fn is_open(page: &Page<'_>) -> Result<bool, Report> {
    Ok(page.control("is-open").await? == 1.0)
}

/// Mounting the button and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(BUTTON, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(BUTTON).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(BUTTON, 0).await?;
    Ok(())
}

/// A click opens the picker; the button shows pressed while it is open.
#[browser_test]
pub async fn press_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let button = page.element(BUTTON).await?;
    page.measure("click", button.click()).await?;
    page.element(format!("{DIALOG} [role=grid]")).await?;
    button.wait_for_attr("data-pressed", Some("true")).await?;
    assert_that!(is_open(page).await?).is_true();
    Ok(())
}

/// Holding the pointer down on the button (its pressed state) and releasing it (the picker
/// opens).
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let button = page.element(BUTTON).await?;
    let held = page.measure("hold", button.press_and_hold()).await?;
    button.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("release", held.release()).await?;
    page.element(format!("{DIALOG} [role=grid]")).await?;
    assert_that!(is_open(page).await?).is_true();
    Ok(())
}

/// Enter on the focused button opens the picker.
#[browser_test]
pub async fn press_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element("#perf-after").await?.click().await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&page.element(BUTTON).await?).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    page.element(format!("{DIALOG} [role=grid]")).await?;
    assert_that!(is_open(page).await?).is_true();
    Ok(())
}

/// The pointer entering and leaving the button.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let button = page.element(BUTTON).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", button.hover()).await?;
    button.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    button.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Shift+Tab moving focus to the button (with a focus ring) and Tab out.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element("#perf-after").await?.click().await?;
    let button = page.element(BUTTON).await?;
    page.measure("focus", page.send_keys(Key::Shift + Key::Tab))
        .await?;
    button
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    button.wait_for_attr("data-focus-visible", None).await?;
    Ok(())
}

/// Disabling the picker (the button with it), a click it ignores, and enabling it.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let button = page.element(BUTTON).await?;
    page.measure("disable", page.control("disable")).await?;
    button.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("ignored click", button.click()).await?;
    page.count_stays(DIALOG, 0, std::time::Duration::from_millis(100))
        .await?;
    page.measure("enable", page.control("enable")).await?;
    button.wait_for_attr("data-disabled", None).await?;
    Ok(())
}

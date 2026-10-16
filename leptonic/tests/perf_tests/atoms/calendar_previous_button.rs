// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `CalendarPreviousButton` (`/perf/atoms/calendar_previous_button`, in
//! a calendar of June 2024): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/atoms/calendar_previous_button.json`): mounting and
//! unmounting it, paging with a click and with Enter and Space, its pressed state, hover, keyboard
//! focus, and being disabled (at a minimum, with the calendar) with a click it ignores.
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/calendar_previous_button";

/// Registers the performance tests of `CalendarPreviousButton`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(PressWithTheMouse))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(PressWithTheKeyboard))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(NoPreviousPage))
        .with(fixture_test(DisableAndEnable))
}
const BUTTON: &str = "#perf-calendar button[aria-label=Previous]";
const HEADING: &str = "#perf-calendar h2";
/// The entry the page's first announcement leaves in the live announcer.
const ANNOUNCER_READY: &str = "[data-live-announcer] [aria-live=polite] > div";

/// Loads the page and waits until the live announcer exists (the page creates it on load; the
/// first announcement of a page waits 100 ms for it).
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ANNOUNCER_READY, 1).await
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

/// A click shows the previous month.
#[browser_test]
pub async fn press_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let button = page.element(BUTTON).await?;
    page.measure("click", button.click()).await?;
    page.element(HEADING)
        .await?
        .wait_for_inner_text("May 2024")
        .await?;
    Ok(())
}

/// Holding the pointer down (the pressed state) and releasing it (the previous month).
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let button = page.element(BUTTON).await?;
    let held = page.measure("hold", button.press_and_hold()).await?;
    button.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("release", held.release()).await?;
    button.wait_for_attr("data-pressed", None).await?;
    page.element(HEADING)
        .await?
        .wait_for_inner_text("May 2024")
        .await?;
    Ok(())
}

/// Enter and Space on the focused button, each showing the previous month.
#[browser_test]
pub async fn press_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(BUTTON).await?).await?;
    let heading = page.element(HEADING).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    heading.wait_for_inner_text("May 2024").await?;
    page.measure("space", page.send_keys(" ")).await?;
    heading.wait_for_inner_text("April 2024").await?;
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

/// Tab moving focus to the button (with a focus ring) and on.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element("#perf-before").await?.click().await?;
    let button = page.element(BUTTON).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    button
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    button.wait_for_attr("data-focus-visible", None).await?;
    Ok(())
}

/// A minimum in June (no previous page) disables the button, which ignores a click, and clearing
/// it enables it again.
#[browser_test]
pub async fn no_previous_page(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let button = page.element(BUTTON).await?;
    page.measure("set min", page.control("set-min")).await?;
    button.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("ignored click", button.click()).await?;
    page.element(HEADING)
        .await?
        .inner_text_stays("June 2024", std::time::Duration::from_millis(100))
        .await?;
    page.measure("clear min", page.control("clear-min")).await?;
    button.wait_for_attr("data-disabled", None).await?;
    Ok(())
}

/// Disabling the calendar disables the button; enabling it enables the button again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let button = page.element(BUTTON).await?;
    page.measure("disable", page.control("disable")).await?;
    button.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("enable", page.control("enable")).await?;
    button.wait_for_attr("data-disabled", None).await?;
    Ok(())
}

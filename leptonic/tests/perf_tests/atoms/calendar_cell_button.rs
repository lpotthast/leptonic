// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `CalendarCellButton` (`/perf/atoms/calendar_cell_button`, the
//! dates of a calendar of June 2024): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/calendar_cell_button.json`): mounting and unmounting the cells with
//! their buttons, a click, holding and releasing the pointer (its pressed state), hover, keyboard
//! focus, and presses with Enter and Space.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/calendar_cell_button";

/// Registers the performance tests of `CalendarCellButton`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(PressWithTheMouse))
        .with(fixture_test(HoldAndRelease))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(KeyboardFocusAndBlur))
        .with(fixture_test(PressWithTheKeyboard))
}
const BUTTONS: &str = "#perf-calendar [role=gridcell] > [role=button]";
/// The entry the page's first announcement leaves in the live announcer.
const ANNOUNCER_READY: &str = "[data-live-announcer] [aria-live=polite] > div";

/// Loads the page and waits until the live announcer exists (the page creates it on load; the
/// first announcement of a page waits 100 ms for it).
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ANNOUNCER_READY, 1).await
}

/// The button of the date labelled `label` ("Friday, June 14, 2024") in its own month's grid.
async fn date(page: &Page<'_>, label: &str) -> Result<WebElement, Report> {
    page.element(format!(
        "{BUTTONS}:not([data-outside-month])[aria-label^='{label}']"
    ))
    .await
}

/// The calendar's value (`yyyymmdd`, 0 without).
async fn value(page: &Page<'_>) -> Result<f64, Report> {
    page.control("value").await
}

/// Mounting the cells with their buttons (42: six weeks) and unmounting them.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(BUTTONS, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(BUTTONS, 42).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(BUTTONS, 0).await?;
    Ok(())
}

/// A click selects the date.
#[browser_test]
pub async fn press_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let june_14 = date(page, "Friday, June 14, 2024").await?;
    page.measure("click", june_14.click()).await?;
    june_14.wait_for_attr("data-selected", Some("true")).await?;
    assert_that!(value(page).await?).is_equal_to(20_240_614.0);
    Ok(())
}

/// Holding the pointer down (the pressed state) and releasing it (the selection).
#[browser_test]
pub async fn hold_and_release(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let june_14 = date(page, "Friday, June 14, 2024").await?;
    let held = page.measure("hold", june_14.press_and_hold()).await?;
    june_14.wait_for_attr("data-pressed", Some("true")).await?;
    page.measure("release", held.release()).await?;
    june_14.wait_for_attr("data-pressed", None).await?;
    assert_that!(value(page).await?).is_equal_to(20_240_614.0);
    Ok(())
}

/// The pointer entering and leaving a date.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let june_14 = date(page, "Friday, June 14, 2024").await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", june_14.hover()).await?;
    june_14.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    june_14.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Shift+Tab moving focus to the focused date (with a focus ring) and Tab out.
#[browser_test]
pub async fn keyboard_focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element("#perf-after").await?.click().await?;
    let june_12 = date(page, "Wednesday, June 12, 2024").await?;
    page.measure("focus", page.send_keys(Key::Shift + Key::Tab))
        .await?;
    june_12
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    june_12.wait_for_attr("data-focused", Some("true")).await?;
    page.measure("blur", page.send_keys(Key::Tab)).await?;
    june_12.wait_for_attr("data-focus-visible", None).await?;
    Ok(())
}

/// Enter and Space on the focused date select it.
#[browser_test]
pub async fn press_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element("#perf-after").await?.click().await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    let june_12 = date(page, "Wednesday, June 12, 2024").await?;
    page.wait_for_focus(&june_12).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    june_12.wait_for_attr("data-selected", Some("true")).await?;
    page.send_keys(Key::Right).await?;
    let june_13 = date(page, "Thursday, June 13, 2024").await?;
    page.wait_for_focus(&june_13).await?;
    page.measure("space", page.send_keys(" ")).await?;
    june_13.wait_for_attr("data-selected", Some("true")).await?;
    assert_that!(value(page).await?).is_equal_to(20_240_613.0);
    Ok(())
}

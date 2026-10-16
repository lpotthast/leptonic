// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `DatePickerGroup` (`/perf/atoms/date_picker_group`, the group of
//! a date picker opening on June 2024): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/date_picker_group.json`): mounting and unmounting it (with its picker),
//! hover, the focus entering it (with a focus ring) and leaving it, a press between its segments
//! (focusing the one before), the arrow keys between its segments, Alt+ArrowDown opening the picker,
//! and the states it shows (open, disabled, invalid).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/date_picker_group";

/// Registers the performance tests of `DatePickerGroup`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(FocusWithinAndBlur))
        .with(fixture_test(PressBetweenTheSegments))
        .with(fixture_test(MoveBetweenSegments))
        .with(fixture_test(OpenWithTheKeyboard))
        .with(fixture_test(OpenAndClose))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(Invalid))
}
const GROUP: &str = ".leptonic-DatePickerGroup";
const DIALOG: &str = "[role=dialog]";
/// The entry the page's first announcement leaves in the live announcer.
const ANNOUNCER_READY: &str = "[data-live-announcer] [aria-live=polite] > div";

/// Loads the page and waits until the live announcer exists (the page creates it on load; the
/// first announcement of a page waits 100 ms for it).
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ANNOUNCER_READY, 1).await
}

/// The segment of `kind` (`month`, `day`, `year`).
async fn segment(page: &Page<'_>, kind: &str) -> Result<WebElement, Report> {
    page.element(format!("{GROUP} [data-type='{kind}']")).await
}

/// Focuses the month with the keyboard (Tab from the plain button before the picker).
async fn focus_the_month(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let month = segment(page, "month").await?;
    page.wait_for_focus(&month).await?;
    Ok(month)
}

/// Mounting the group with its picker and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(GROUP, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    assert_that!(page.element(GROUP).await?.attr("role").await?)
        .is_equal_to(Some("group".to_owned()));
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(GROUP, 0).await?;
    Ok(())
}

/// The pointer entering and leaving the group.
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let group = page.element(GROUP).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", group.hover()).await?;
    group.wait_for_attr("data-hovered", Some("true")).await?;
    page.measure("leave", after.hover()).await?;
    group.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Tab moving focus into the group (its month: focus within, with a focus ring) and Shift+Tab
/// out.
#[browser_test]
pub async fn focus_within_and_blur(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element("#perf-before").await?.click().await?;
    let group = page.element(GROUP).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    group
        .wait_for_attr("data-focus-within", Some("true"))
        .await?;
    group
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.measure("blur", page.send_keys(Key::Shift + Key::Tab))
        .await?;
    group.wait_for_attr("data-focus-within", None).await?;
    Ok(())
}

/// A press on the group between segments (on the literal after the month: the segments handle
/// their own presses) focuses the segment before it.
#[browser_test]
pub async fn press_between_the_segments(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let literal = page
        .first_element(format!("{GROUP} [data-type='literal']"))
        .await?;
    page.measure("press", literal.click()).await?;
    page.wait_for_focus(&segment(page, "month").await?).await?;
    Ok(())
}

/// Arrow Right and Left move the focus between the segments.
#[browser_test]
pub async fn move_between_segments(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    focus_the_month(page).await?;
    page.measure("right", page.send_keys(Key::Right)).await?;
    page.wait_for_focus(&segment(page, "day").await?).await?;
    page.measure("left", page.send_keys(Key::Left)).await?;
    page.wait_for_focus(&segment(page, "month").await?).await?;
    Ok(())
}

/// Alt+ArrowDown in the group opens the picker.
#[browser_test]
pub async fn open_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    focus_the_month(page).await?;
    page.measure("alt+down", page.send_keys(Key::Alt + Key::Down))
        .await?;
    page.element(format!("{DIALOG} [role=grid]")).await?;
    page.element(GROUP)
        .await?
        .wait_for_attr("data-open", Some("true"))
        .await?;
    Ok(())
}

/// The group showing the picker open and closed again (the app opening and closing it).
#[browser_test]
pub async fn open_and_close(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let group = page.element(GROUP).await?;
    page.measure("open", page.control("open")).await?;
    group.wait_for_attr("data-open", Some("true")).await?;
    page.measure("close", page.control("close")).await?;
    group.wait_for_attr("data-open", None).await?;
    page.wait_for_count(DIALOG, 0).await?;
    Ok(())
}

/// The group showing the picker disabled and enabled again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let group = page.element(GROUP).await?;
    page.measure("disable", page.control("disable")).await?;
    group.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("enable", page.control("enable")).await?;
    group.wait_for_attr("data-disabled", None).await?;
    Ok(())
}

/// The group showing the picker invalid and valid again.
#[browser_test]
pub async fn invalid(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let group = page.element(GROUP).await?;
    page.measure("invalidate", page.control("invalidate"))
        .await?;
    group.wait_for_attr("data-invalid", Some("true")).await?;
    page.measure("validate", page.control("validate")).await?;
    group.wait_for_attr("data-invalid", None).await?;
    Ok(())
}

// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_date_range_picker` (`/perf/hooks/use_date_range_picker`, a
//! date range picker of hooks labelled "Trip", its popover a plain dialog): what each thing it
//! offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_date_range_picker.json`): creating and disposing of it, opening it
//! with its button and with Alt+ArrowDown, closing it, a range selected in its dialog, typing the
//! start (the focus moves on to the end), and changes of its inputs (range, disabled).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_date_range_picker";

/// Registers the performance tests of `use_date_range_picker`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(OpenWithTheButtonAndClose))
        .with(fixture_test(OpenWithTheKeyboard))
        .with(fixture_test(SelectARange))
        .with(fixture_test(TypeTheStart))
        .with(fixture_test(SetAndClearTheRange))
        .with(fixture_test(DisableAndEnable))
}
const GROUP: &str = "#perf-group";
const BUTTON: &str = "#perf-group button";
const DIALOG: &str = "#perf-dialog";
/// The entry the page's first announcement leaves in the live announcer.
const ANNOUNCER_READY: &str = "[data-live-announcer] [aria-live=polite] > div";

/// Loads the page and waits until the live announcer exists (the page creates it on load; the
/// first announcement of a page waits 100 ms for it).
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ANNOUNCER_READY, 1).await
}

/// The segment of `kind` (`month`, `day`, `year`) of the field `end` (`start`, `end`).
async fn segment(page: &Page<'_>, end: &str, kind: &str) -> Result<WebElement, Report> {
    page.element(format!("#perf-{end} [data-type='{kind}']"))
        .await
}

/// The range (`yyyymmdd`, 0 without).
async fn range(page: &Page<'_>) -> Result<(f64, f64), Report> {
    Ok((page.control("start").await?, page.control("end").await?))
}

/// Creating the picker and disposing of it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(GROUP, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    segment(page, "end", "year")
        .await?
        .wait_for_inner_text("yyyy")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(GROUP, 0).await?;
    Ok(())
}

/// Its button opening the picker, and the app closing it.
#[browser_test]
pub async fn open_with_the_button_and_close(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.measure("open", page.element(BUTTON).await?.click())
        .await?;
    page.element(DIALOG).await?;
    page.measure("close", page.control("close")).await?;
    page.wait_for_count(DIALOG, 0).await?;
    Ok(())
}

/// Alt+ArrowDown in a segment opens the picker.
#[browser_test]
pub async fn open_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&segment(page, "start", "month").await?)
        .await?;
    page.measure("alt+down", page.send_keys(Key::Alt + Key::Down))
        .await?;
    page.element(DIALOG).await?;
    Ok(())
}

/// A range selected in the open dialog: the picker takes it and closes.
#[browser_test]
pub async fn select_a_range(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element(BUTTON).await?.click().await?;
    let pick = page.element("#perf-pick").await?;
    page.measure("select", pick.click()).await?;
    page.wait_for_count(DIALOG, 0).await?;
    assert_that!(range(page).await?).is_equal_to((20_240_610.0, 20_240_614.0));
    Ok(())
}

/// Typing the start, June 15, 2024: the focus moves on to the end.
#[browser_test]
pub async fn type_the_start(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&segment(page, "start", "month").await?)
        .await?;
    page.measure("type the month", page.type_text("6")).await?;
    page.wait_for_focus(&segment(page, "start", "day").await?)
        .await?;
    page.measure("type the day", page.type_text("15")).await?;
    page.wait_for_focus(&segment(page, "start", "year").await?)
        .await?;
    page.measure("type the year", page.type_text("2024"))
        .await?;
    page.wait_for_focus(&segment(page, "end", "month").await?)
        .await?;
    Ok(())
}

/// The app setting the range (June 5 to 9, 2024) and clearing it.
#[browser_test]
pub async fn set_and_clear_the_range(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let day = segment(page, "end", "day").await?;
    page.measure("set", page.control("set-value")).await?;
    day.wait_for_inner_text("9").await?;
    page.measure("clear", page.control("clear-value")).await?;
    day.wait_for_inner_text("dd").await?;
    Ok(())
}

/// Disabling the picker (its button too), a click it ignores, and enabling it.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let button = page.element(BUTTON).await?;
    page.measure("disable", page.control("disable")).await?;
    button.wait_for_prop("disabled", "true").await?;
    page.measure("ignored click", button.click()).await?;
    page.count_stays(DIALOG, 0, std::time::Duration::from_millis(100))
        .await?;
    page.measure("enable", page.control("enable")).await?;
    button.wait_for_prop("disabled", "false").await?;
    Ok(())
}

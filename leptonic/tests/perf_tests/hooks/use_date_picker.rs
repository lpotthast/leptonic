// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_date_picker` (`/perf/hooks/use_date_picker`, a date picker of
//! hooks labelled "Departure", its popover a plain dialog): what each thing it offers costs,
//! measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_date_picker.json`): creating and disposing of it, opening it with its
//! button and with Alt+ArrowDown, closing it, a date selected in its dialog, typing a date, and
//! changes of its inputs (value, disabled).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_date_picker";

/// Registers the performance tests of `use_date_picker`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(OpenWithTheButtonAndClose))
        .with(fixture_test(OpenWithTheKeyboard))
        .with(fixture_test(SelectADate))
        .with(fixture_test(TypeADate))
        .with(fixture_test(SetAndClearTheValue))
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

/// The segment of `kind` (`month`, `day`, `year`).
async fn segment(page: &Page<'_>, kind: &str) -> Result<WebElement, Report> {
    page.element(format!("{GROUP} [data-type='{kind}']")).await
}

/// Creating the picker and disposing of it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(GROUP, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    segment(page, "month")
        .await?
        .wait_for_inner_text("mm")
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
    page.wait_for_focus(&segment(page, "month").await?).await?;
    page.measure("alt+down", page.send_keys(Key::Alt + Key::Down))
        .await?;
    page.element(DIALOG).await?;
    Ok(())
}

/// A date selected in the open dialog: the picker takes it and closes.
#[browser_test]
pub async fn select_a_date(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element(BUTTON).await?.click().await?;
    let pick = page.element("#perf-pick").await?;
    page.measure("select", pick.click()).await?;
    page.wait_for_count(DIALOG, 0).await?;
    assert_that!(page.control("value").await?).is_equal_to(20_240_614.0);
    Ok(())
}

/// Typing June 15, 2024 into the picker's segments.
#[browser_test]
pub async fn type_a_date(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&segment(page, "month").await?).await?;
    page.measure("type the month", page.type_text("6")).await?;
    page.wait_for_focus(&segment(page, "day").await?).await?;
    page.measure("type the day", page.type_text("15")).await?;
    page.wait_for_focus(&segment(page, "year").await?).await?;
    page.measure("type the year", page.type_text("2024"))
        .await?;
    assert_that!(page.control("value").await?).is_equal_to(20_240_615.0);
    Ok(())
}

/// The app setting the value (June 5, 2024) and clearing it.
#[browser_test]
pub async fn set_and_clear_the_value(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let year = segment(page, "year").await?;
    page.measure("set", page.control("set-value")).await?;
    year.wait_for_inner_text("2024").await?;
    page.measure("clear", page.control("clear-value")).await?;
    year.wait_for_inner_text("yyyy").await?;
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

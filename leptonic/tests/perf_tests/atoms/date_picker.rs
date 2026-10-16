// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `DatePicker` (`/perf/atoms/date_picker`, a date picker labelled
//! "Departure" opening on June 2024): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget (`tests/budgets/atoms/date_picker.json`):
//! mounting and unmounting it (empty and with a date), opening it with its button and with
//! Alt+ArrowDown, closing it with Escape, selecting a date in its calendar, typing a date, and
//! changes of its reactive props (open state, value, minimum, disabled, read-only, invalid).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/date_picker";

/// Registers the performance tests of `DatePicker`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountWithADate))
        .with(fixture_test(OpenWithTheButtonAndEscape))
        .with(fixture_test(SelectADate))
        .with(fixture_test(OpenWithTheKeyboard))
        .with(fixture_test(TypeADate))
        .with(fixture_test(OpenAndClose))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(ReadOnly))
        .with(fixture_test(Invalid))
        .with(fixture_test(ControlledValue))
        .with(fixture_test(Minimum))
}
const PICKER: &str = ".leptonic-DatePicker";
const GROUP: &str = ".leptonic-DatePickerGroup";
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

/// The segment of `kind` (`month`, `day`, `year`).
async fn segment(page: &Page<'_>, kind: &str) -> Result<WebElement, Report> {
    page.element(format!("{GROUP} [data-type='{kind}']")).await
}

/// The button of the date labelled `label` ("Friday, June 14, 2024") in the open calendar.
async fn date(page: &Page<'_>, label: &str) -> Result<WebElement, Report> {
    page.element(format!(
        "{DIALOG} [role=gridcell] > [role=button]:not([data-outside-month])[aria-label^='{label}']"
    ))
    .await
}

/// The picker's value (`yyyymmdd`, 0 without).
async fn value(page: &Page<'_>) -> Result<f64, Report> {
    page.control("value").await
}

/// Mounting the empty picker and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(PICKER, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    segment(page, "month")
        .await?
        .wait_for_inner_text("mm")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(PICKER, 0).await?;
    Ok(())
}

/// Mounting the picker with a date (June 5, 2024) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_with_a_date(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(PICKER, 0).await?;
    page.control("set-value").await?;
    page.measure("mount", page.control("mount")).await?;
    segment(page, "year")
        .await?
        .wait_for_inner_text("2024")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(PICKER, 0).await?;
    Ok(())
}

/// Opening the picker with its button (the calendar takes the focus) and closing it with
/// Escape.
#[browser_test]
pub async fn open_with_the_button_and_escape(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let button = page.element(BUTTON).await?;
    page.measure("open", button.click()).await?;
    page.wait_for_focus(&date(page, "Wednesday, June 12, 2024").await?)
        .await?;
    page.measure("escape", page.send_keys(Key::Escape)).await?;
    page.wait_for_count(DIALOG, 0).await?;
    assert_that!(page.control("is-open").await?).is_equal_to(0.0);
    Ok(())
}

/// Selecting a date in the open calendar: the picker takes it and closes.
#[browser_test]
pub async fn select_a_date(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element(BUTTON).await?.click().await?;
    let june_14 = date(page, "Friday, June 14, 2024").await?;
    page.wait_for_focus(&date(page, "Wednesday, June 12, 2024").await?)
        .await?;
    page.measure("select", june_14.click()).await?;
    page.wait_for_count(DIALOG, 0).await?;
    segment(page, "day")
        .await?
        .wait_for_inner_text("14")
        .await?;
    assert_that!(value(page).await?).is_equal_to(20_240_614.0);
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
    page.wait_for_focus(&date(page, "Wednesday, June 12, 2024").await?)
        .await?;
    assert_that!(page.control("is-open").await?).is_equal_to(1.0);
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
    assert_that!(value(page).await?).is_equal_to(20_240_615.0);
    Ok(())
}

/// The app opening the picker and closing it.
#[browser_test]
pub async fn open_and_close(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.measure("open", page.control("open")).await?;
    page.element(format!("{DIALOG} [role=grid]")).await?;
    page.measure("close", page.control("close")).await?;
    page.wait_for_count(DIALOG, 0).await?;
    Ok(())
}

/// Disabling the picker, a click on its button it ignores, and enabling it.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let picker = page.element(PICKER).await?;
    let button = page.element(BUTTON).await?;
    page.measure("disable", page.control("disable")).await?;
    picker.wait_for_attr("data-disabled", Some("true")).await?;
    page.measure("ignored click", button.click()).await?;
    page.count_stays(DIALOG, 0, std::time::Duration::from_millis(100))
        .await?;
    page.measure("enable", page.control("enable")).await?;
    picker.wait_for_attr("data-disabled", None).await?;
    Ok(())
}

/// Making the picker read-only and editable again.
#[browser_test]
pub async fn read_only(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let picker = page.element(PICKER).await?;
    page.measure("make read-only", page.control("make-read-only"))
        .await?;
    picker.wait_for_attr("data-readonly", Some("true")).await?;
    page.measure("make editable", page.control("make-editable"))
        .await?;
    picker.wait_for_attr("data-readonly", None).await?;
    Ok(())
}

/// Marking the picker invalid and valid again.
#[browser_test]
pub async fn invalid(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let picker = page.element(PICKER).await?;
    page.measure("invalidate", page.control("invalidate"))
        .await?;
    picker.wait_for_attr("data-invalid", Some("true")).await?;
    page.measure("validate", page.control("validate")).await?;
    picker.wait_for_attr("data-invalid", None).await?;
    Ok(())
}

/// The app setting the picker's date (June 5, 2024) and clearing it.
#[browser_test]
pub async fn controlled_value(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let year = segment(page, "year").await?;
    page.measure("set", page.control("set-value")).await?;
    year.wait_for_inner_text("2024").await?;
    page.measure("clear", page.control("clear-value")).await?;
    year.wait_for_inner_text("yyyy").await?;
    Ok(())
}

/// A minimum after the picker's date (it becomes invalid) and clearing it.
#[browser_test]
pub async fn minimum(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("set-value").await?;
    // Native validation (the default): the field's input reports the error to the form.
    let input = page.first_element(".leptonic-DatePicker > input").await?;
    segment(page, "year")
        .await?
        .wait_for_inner_text("2024")
        .await?;
    page.measure("set min", page.control("set-min")).await?;
    assert_that!(input.is_valid().await?).is_false();
    page.measure("clear min", page.control("clear-min")).await?;
    assert_that!(input.is_valid().await?).is_true();
    Ok(())
}

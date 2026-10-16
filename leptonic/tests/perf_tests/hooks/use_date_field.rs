// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_date_field` (`/perf/hooks/use_date_field`, a date field of
//! hooks labelled "Birthday"): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/hooks/use_date_field.json`): creating and disposing
//! of it, typing a date, the focus entering and leaving its group, a click on its label, its value
//! set and cleared by the app (its hidden input follows), and changes of its inputs (disabled, a
//! minimum).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page, css},
};

const PATH: &str = "/perf/hooks/use_date_field";

/// Registers the performance tests of `use_date_field`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(TypeADate))
        .with(fixture_test(FocusAndBlur))
        .with(fixture_test(ClickTheLabel))
        .with(fixture_test(SetAndClearTheValue))
        .with(fixture_test(DisableAndEnable))
        .with(fixture_test(Minimum))
}
const FIELD: &str = "#perf-field";
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
    page.element(format!("{FIELD} [data-type='{kind}']")).await
}

/// Creating the field and disposing of it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(FIELD, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    segment(page, "month")
        .await?
        .wait_for_inner_text("mm")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(FIELD, 0).await?;
    Ok(())
}

/// Typing June 15, 2024: each full segment moves the focus on; the last makes the date.
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

/// Tab moving focus into the field's group and Shift+Tab out.
#[browser_test]
pub async fn focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element("#perf-before").await?.click().await?;
    let month = segment(page, "month").await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&month).await?;
    page.measure("blur", page.send_keys(Key::Shift + Key::Tab))
        .await?;
    page.wait_for_focus(&page.element("#perf-before").await?)
        .await?;
    Ok(())
}

/// A click on the label focuses the first segment.
#[browser_test]
pub async fn click_the_label(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let label = page.element(css("span").text("Birthday")).await?;
    page.measure("click", label.click()).await?;
    page.wait_for_focus(&segment(page, "month").await?).await?;
    Ok(())
}

/// The app setting the value (June 5, 2024; the hidden input submits it) and clearing it.
#[browser_test]
pub async fn set_and_clear_the_value(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let input = page.element("#perf-input").await?;
    page.measure("set", page.control("set-value")).await?;
    input.wait_for_prop("value", "2024-06-05").await?;
    segment(page, "year")
        .await?
        .wait_for_inner_text("2024")
        .await?;
    page.measure("clear", page.control("clear-value")).await?;
    input.wait_for_prop("value", "").await?;
    Ok(())
}

/// Disabling the field and enabling it.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let field = page.element(FIELD).await?;
    page.measure("disable", page.control("disable")).await?;
    field.wait_for_attr("aria-disabled", Some("true")).await?;
    page.measure("enable", page.control("enable")).await?;
    field.wait_for_attr("aria-disabled", None).await?;
    Ok(())
}

/// A minimum after the value (the segments invalid) and clearing it.
#[browser_test]
pub async fn minimum(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("set-value").await?;
    let month = segment(page, "month").await?;
    month.wait_for_inner_text("6").await?;
    page.measure("set min", page.control("set-min")).await?;
    month.wait_for_attr("aria-invalid", Some("true")).await?;
    page.measure("clear min", page.control("clear-min")).await?;
    month.wait_for_attr("aria-invalid", None).await?;
    Ok(())
}

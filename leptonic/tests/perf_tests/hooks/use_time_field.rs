// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_time_field` (`/perf/hooks/use_time_field`, a time field of
//! hooks labelled "Pickup"): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/hooks/use_time_field.json`): creating and disposing
//! of it, typing a time, the focus entering and leaving its group, its value set and cleared by
//! the app (its hidden input submits the time), and changes of its inputs (disabled, a minimum).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_time_field";

/// Registers the performance tests of `use_time_field`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(TypeATime))
        .with(fixture_test(FocusAndBlur))
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

/// The segment of `kind` (`hour`, `minute`, `dayPeriod`).
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
    segment(page, "dayPeriod").await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(FIELD, 0).await?;
    Ok(())
}

/// Typing 8:30 PM: the hour, the minute and the day period, each full segment moving the focus
/// on.
#[browser_test]
pub async fn type_a_time(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&segment(page, "hour").await?).await?;
    page.measure("type the hour", page.type_text("8")).await?;
    page.wait_for_focus(&segment(page, "minute").await?).await?;
    page.measure("type the minute", page.type_text("30"))
        .await?;
    page.wait_for_focus(&segment(page, "dayPeriod").await?)
        .await?;
    page.measure("type the day period", page.type_text("p"))
        .await?;
    assert_that!(page.control("value").await?).is_equal_to(203_000.0);
    Ok(())
}

/// Tab moving focus into the field's group and Shift+Tab out.
#[browser_test]
pub async fn focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.element("#perf-before").await?.click().await?;
    let hour = segment(page, "hour").await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&hour).await?;
    page.measure("blur", page.send_keys(Key::Shift + Key::Tab))
        .await?;
    page.wait_for_focus(&page.element("#perf-before").await?)
        .await?;
    Ok(())
}

/// The app setting the value (8:30 AM; the hidden input submits it) and clearing it.
#[browser_test]
pub async fn set_and_clear_the_value(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let input = page.element("#perf-input").await?;
    page.measure("set", page.control("set-value")).await?;
    input.wait_for_prop("value", "08:30:00").await?;
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
    let hour = segment(page, "hour").await?;
    hour.wait_for_inner_text("8").await?;
    page.measure("set min", page.control("set-min")).await?;
    hour.wait_for_attr("aria-invalid", Some("true")).await?;
    page.measure("clear min", page.control("clear-min")).await?;
    hour.wait_for_attr("aria-invalid", None).await?;
    Ok(())
}

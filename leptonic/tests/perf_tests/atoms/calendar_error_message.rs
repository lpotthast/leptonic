// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `CalendarErrorMessage` (`/perf/atoms/calendar_error_message`, in
//! a calendar of June 2024 with June 18 selected): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/calendar_error_message.json`): mounting and unmounting it (an invalid
//! selected date refers to it while it is rendered), and the calendar becoming invalid (the
//! selected date then refers to it) and valid again.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/calendar_error_message";

/// Registers the performance tests of `CalendarErrorMessage`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(InvalidAndValid))
}
const MESSAGE: &str = "#perf-calendar .leptonic-CalendarErrorMessage";
/// The entry the page's first announcement leaves in the live announcer.
const ANNOUNCER_READY: &str = "[data-live-announcer] [aria-live=polite] > div";

/// Loads the page and waits until the live announcer exists (the page creates it on load; the
/// first announcement of a page waits 100 ms for it).
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(ANNOUNCER_READY, 1).await
}

/// The button of the selected date, June 18, 2024.
async fn june_18(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element(
        "#perf-calendar [role=gridcell] > [role=button]:not([data-outside-month])\
         [aria-label^='Tuesday, June 18, 2024']",
    )
    .await
}

/// The id of the error message.
async fn message_id(page: &Page<'_>) -> Result<String, Report> {
    Ok(page.element(MESSAGE).await?.id().await?.unwrap_or_default())
}

/// Mounting the error message of an invalid calendar (the selected date then refers to it) and
/// unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.control("invalidate").await?;
    let selected = june_18(page).await?;
    selected.wait_for_attr("data-invalid", Some("true")).await?;
    page.control("unmount").await?;
    page.wait_for_count(MESSAGE, 0).await?;
    selected.wait_for_attr("aria-describedby", None).await?;
    page.measure("mount", page.control("mount")).await?;
    let id = message_id(page).await?;
    selected
        .wait_for_attr("aria-describedby", Some(&id))
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(MESSAGE, 0).await?;
    selected.wait_for_attr("aria-describedby", None).await?;
    Ok(())
}

/// The calendar becoming invalid (the selected date refers to the message) and valid again.
#[browser_test]
pub async fn invalid_and_valid(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let selected = june_18(page).await?;
    let id = message_id(page).await?;
    assert_that!(selected.attr("aria-describedby").await?).is_none();
    page.measure("invalidate", page.control("invalidate"))
        .await?;
    selected
        .wait_for_attr("aria-describedby", Some(&id))
        .await?;
    page.measure("validate", page.control("validate")).await?;
    selected.wait_for_attr("aria-describedby", None).await?;
    Ok(())
}

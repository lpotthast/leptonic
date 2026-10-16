// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_tooltip_trigger_state`
//! (`/perf/hooks/use_tooltip_trigger_state`): what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_tooltip_trigger_state.json`): mounting and unmounting it, opening and
//! closing right away, and opening and closing delayed: the measured call starts the delay's timer
//! (1,500 ms to open, 500 ms to close), which runs out after the measurement (the page is idle
//! within a few frames), unmeasured. One opening per test: the warm-up's cooldown after a close is
//! a timer, too.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_tooltip_trigger_state";
const OPEN: &str = "#perf-open";

/// Registers the performance tests of `use_tooltip_trigger_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(OpenAndClose))
        .with(fixture_test(DelayedOpen))
        .with(fixture_test(DelayedClose))
}

/// How often the state reported a change.
async fn changes(page: &Page<'_>) -> Result<f64, Report> {
    page.control("changes").await
}

/// Mounting the state and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(OPEN, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(OPEN)
        .await?
        .wait_for_inner_text("false")
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(OPEN, 0).await?;
    Ok(())
}

/// Opening right away and closing right away.
#[browser_test]
pub async fn open_and_close(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let open = page.element(OPEN).await?;
    page.measure("open", page.control("open")).await?;
    open.wait_for_inner_text("true").await?;
    page.measure("close", page.control("close")).await?;
    open.wait_for_inner_text("false").await?;
    assert_that!(changes(page).await?).is_equal_to(2.0);
    Ok(())
}

/// A delayed opening: the call (starting the warm-up's timer), the state still closed after it;
/// it opens once the delay ran out.
#[browser_test]
pub async fn delayed_open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let open = page.element(OPEN).await?;
    page.measure("open", page.control("open-delayed")).await?;
    assert_that!(open.inner_text().await?).is_equal_to("false".to_owned());
    open.wait_for_inner_text("true").await?;
    assert_that!(changes(page).await?).is_equal_to(1.0);
    Ok(())
}

/// A delayed closing of the open state: the call (starting the close delay's timer), the state
/// still open after it; it closes once the delay ran out.
#[browser_test]
pub async fn delayed_close(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let open = page.element(OPEN).await?;
    page.control("open").await?;
    open.wait_for_inner_text("true").await?;
    page.measure("close", page.control("close-delayed")).await?;
    assert_that!(open.inner_text().await?).is_equal_to("true".to_owned());
    open.wait_for_inner_text("false").await?;
    assert_that!(changes(page).await?).is_equal_to(2.0);
    Ok(())
}

// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_toast_state` (`/perf/hooks/use_toast_state`, a toast queue
//! alone, showing up to 5 toasts): what each thing it offers costs, measured (`Page::measure`)
//! and checked against its budget (`tests/budgets/hooks/use_toast_state.json`): creating and
//! disposing of the queue, adding and closing a toast in an empty queue and in one of 100,
//! pausing and resuming the timeouts, and clearing it. No timeout runs out while measuring: the
//! timed toasts last an hour.
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_toast_state";

/// Registers the performance tests of `use_toast_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(AddAndClose))
        .with(fixture_test(AddAndCloseMany))
        .with(fixture_test(PauseAndResume))
        .with(fixture_test(Clear))
}
const VISIBLE: &str = "#perf-visible";

/// Waits until the queue shows `count` toasts.
async fn wait_for_visible(page: &Page<'_>, count: &str) -> Result<(), Report> {
    page.element(VISIBLE)
        .await?
        .wait_for_inner_text(count)
        .await
}

/// Creating the queue and disposing of it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(VISIBLE, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    wait_for_visible(page, "0").await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(VISIBLE, 0).await?;
    Ok(())
}

/// Adding a toast to the empty queue (without and with a timeout) and closing them.
#[browser_test]
pub async fn add_and_close(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    wait_for_visible(page, "0").await?;
    page.measure("add", page.control("add")).await?;
    wait_for_visible(page, "1").await?;
    page.measure("close", page.control("close-newest")).await?;
    wait_for_visible(page, "0").await?;
    page.measure("add timed", page.control("add-timed")).await?;
    wait_for_visible(page, "1").await?;
    page.measure("close timed", page.control("close-newest"))
        .await?;
    wait_for_visible(page, "0").await?;
    Ok(())
}

/// Adding a toast to a queue of 100 (5 shown) and closing it.
#[browser_test]
pub async fn add_and_close_many(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("fill").await?;
    wait_for_visible(page, "5").await?;
    page.measure("add", page.control("add")).await?;
    page.measure("close", page.control("close-newest")).await?;
    wait_for_visible(page, "5").await?;
    Ok(())
}

/// Pausing the timeouts of the toasts shown (5 timed toasts) and resuming them.
#[browser_test]
pub async fn pause_and_resume(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    for _ in 0..5 {
        page.control("add-timed").await?;
    }
    wait_for_visible(page, "5").await?;
    page.measure("pause", page.control("pause-all")).await?;
    page.measure("resume", page.control("resume-all")).await?;
    page.measure("pause again", page.control("pause-all"))
        .await?;
    wait_for_visible(page, "5").await?;
    Ok(())
}

/// Clearing a queue of 100 toasts.
#[browser_test]
pub async fn clear(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("fill").await?;
    wait_for_visible(page, "5").await?;
    page.measure("clear", page.control("clear")).await?;
    wait_for_visible(page, "0").await?;
    Ok(())
}

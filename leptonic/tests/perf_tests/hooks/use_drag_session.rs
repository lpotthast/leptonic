// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_drag_session` (`/perf/hooks/use_drag_session`): the keyboard
//! and screen reader drag session, read by one element. Measured (`Page::measure`) and checked
//! against its budget (`tests/budgets/hooks/use_drag_session.json`): mounting and unmounting it,
//! and a session starting (Enter on a `use_drag` source) and ending (Escape; a drop).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_drag_session";
const SESSION: &str = "#perf-session";

/// Registers the performance tests of `use_drag_session`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(StartAndCancel))
        .with(fixture_test(StartAndDrop))
}

/// Opens the fixture once the live announcer exists (keyboard drags announce) and focuses the
/// drag source with the keyboard; returns the session element.
async fn open_focused(page: &Page<'_>) -> Result<WebElement, Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count("[data-live-announcer] [aria-live=polite] > div", 1)
        .await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#perf-draggable").await?)
        .await?;
    page.element(SESSION).await
}

/// Mounting the hook and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count("[data-live-announcer] [aria-live=polite] > div", 1)
        .await?;
    page.control("unmount").await?;
    page.wait_for_count(SESSION, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(SESSION).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(SESSION, 0).await?;
    Ok(())
}

/// A session starting (Enter on the drag source) and ending (Escape).
#[browser_test]
pub async fn start_and_cancel(page: &Page<'_>) -> Result<(), Report> {
    let session = open_focused(page).await?;
    page.measure("start", page.send_keys(Key::Enter)).await?;
    session.wait_for_attr("data-session", Some("true")).await?;
    assert_that!(session)
        .has_attribute("data-items")
        .await
        .is_equal_to("1");
    page.wait_for_focus(&page.element("#perf-drop").await?)
        .await?;
    page.measure("cancel", page.send_keys(Key::Escape)).await?;
    session.wait_for_attr("data-session", None).await?;
    Ok(())
}

/// A session ending with a drop (Enter on the target).
#[browser_test]
pub async fn start_and_drop(page: &Page<'_>) -> Result<(), Report> {
    let session = open_focused(page).await?;
    page.send_keys(Key::Enter).await?;
    session.wait_for_attr("data-session", Some("true")).await?;
    page.wait_for_focus(&page.element("#perf-drop").await?)
        .await?;
    page.measure("drop", page.send_keys(Key::Enter)).await?;
    session.wait_for_attr("data-session", None).await?;
    Ok(())
}

// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_modal_backdrop` (`/perf/hooks/use_modal_backdrop`): what each
//! thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/hooks/use_modal_backdrop.json`): mounting and unmounting it, opening (the page
//! outside the modal made inert, its scrolling prevented) and closing by the app's state, Escape
//! and a click outside the modal closing it, and both ignored while dismissing is disabled.
use std::time::Duration;

use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_modal_backdrop";
const MODAL: &str = ".perf-modal";
const OPENER: &str = "#perf-opener";

/// Registers the performance tests of `use_modal_backdrop`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(OpenAndClose))
        .with(fixture_test(EscapeCloses))
        .with(fixture_test(OutsideClickCloses))
        .with(fixture_test(IgnoredEscape))
        .with(fixture_test(IgnoredOutsideClick))
}

/// Opens the modal (the opener focused before) and waits until it is open.
async fn open(page: &Page<'_>) -> Result<(), Report> {
    page.element(OPENER).await?.click().await?;
    page.control("open").await?;
    wait_for_open(page).await
}

/// Waits until the modal is open: its button has the focus, the opener is inert and the page's
/// scrolling is prevented.
async fn wait_for_open(page: &Page<'_>) -> Result<(), Report> {
    page.wait_for_focus(&page.element("#perf-modal-button").await?)
        .await?;
    let opener = page.element(OPENER).await?;
    assert_that!(opener.is_within("[inert]").await?).is_true();
    Ok(())
}

/// Waits until the modal is closed and the opener is no longer inert.
async fn wait_for_closed(page: &Page<'_>) -> Result<(), Report> {
    page.wait_for_count(MODAL, 0).await?;
    let opener = page.element(OPENER).await?;
    assert_that!(opener.is_within("[inert]").await?).is_false();
    Ok(())
}

/// Mounting the closed backdrop and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.measure("mount", page.control("mount")).await?;
    page.count_stays(MODAL, 0, Duration::from_millis(100))
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.control("open").await?;
    page.count_stays(MODAL, 0, Duration::from_millis(100))
        .await?;
    Ok(())
}

/// The app opening the modal and closing it.
#[browser_test]
pub async fn open_and_close(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element(OPENER).await?.click().await?;
    page.measure("open", page.control("open")).await?;
    wait_for_open(page).await?;
    page.measure("close", page.control("close")).await?;
    wait_for_closed(page).await?;
    Ok(())
}

/// Escape (in the modal) closing it.
#[browser_test]
pub async fn escape_closes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open(page).await?;
    page.measure("escape", page.send_keys(Key::Escape)).await?;
    wait_for_closed(page).await?;
    Ok(())
}

/// A click outside the modal closing it.
#[browser_test]
pub async fn outside_click_closes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open(page).await?;
    page.measure("outside click", page.click_at(5, 5)).await?;
    wait_for_closed(page).await?;
    Ok(())
}

/// Escape while dismissing with the keyboard is disabled: the modal stays open.
#[browser_test]
pub async fn ignored_escape(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure(
        "disable keyboard dismiss",
        page.control("disable-keyboard-dismiss"),
    )
    .await?;
    open(page).await?;
    page.measure("ignored escape", page.send_keys(Key::Escape))
        .await?;
    page.count_stays(MODAL, 1, Duration::from_millis(100))
        .await?;
    page.measure(
        "enable keyboard dismiss",
        page.control("enable-keyboard-dismiss"),
    )
    .await?;
    Ok(())
}

/// A click outside the modal while it isn't dismissable: it stays open.
#[browser_test]
pub async fn ignored_outside_click(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.measure("not dismissable", page.control("not-dismissable"))
        .await?;
    open(page).await?;
    page.measure("ignored outside click", page.click_at(5, 5))
        .await?;
    page.count_stays(MODAL, 1, Duration::from_millis(100))
        .await?;
    page.measure("dismissable", page.control("dismissable"))
        .await?;
    Ok(())
}

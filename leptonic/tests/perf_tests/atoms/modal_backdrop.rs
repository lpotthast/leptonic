// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ModalBackdrop` (`/perf/atoms/modal_backdrop`): what each thing it
//! offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/modal_backdrop.json`): mounting and unmounting it (closed and open),
//! opening (the page outside made inert, its scrolling prevented, the dialog focused) and closing
//! by the app's state, Escape and a click outside the modal closing it (the focus returning to
//! the opener), and both ignored while dismissing is disabled. Every measurement after an opening
//! starts after the dialog's refocusing ran (`use_dialog` blurs and focuses the dialog again
//! 500 ms after focusing it, a VoiceOver workaround).
use std::time::Duration;

use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/modal_backdrop";
const BACKDROP: &str = ".leptonic-ModalBackdrop";
const DIALOG: &str = "[role=dialog]";
const OPENER: &str = "#perf-opener";

/// Registers the performance tests of `ModalBackdrop`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountOpen))
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

/// Waits until the modal is open: the dialog has the focus (past its refocusing, 500 ms after it
/// took the focus) and the opener is inert.
async fn wait_for_open(page: &Page<'_>) -> Result<(), Report> {
    let dialog = page.element(DIALOG).await?;
    page.wait_for_focus(&dialog).await?;
    // Past the dialog's refocusing.
    page.focus_stays(&dialog, Duration::from_millis(600))
        .await?;
    let opener = page.element(OPENER).await?;
    assert_that!(opener.is_within("[inert]").await?).is_true();
    Ok(())
}

/// Waits until the modal is closed and the opener has the focus back.
async fn wait_for_closed(page: &Page<'_>) -> Result<(), Report> {
    page.wait_for_count(BACKDROP, 0).await?;
    let opener = page.element(OPENER).await?;
    page.wait_for_focus(&opener).await?;
    assert_that!(opener.is_within("[inert]").await?).is_false();
    Ok(())
}

/// Mounting the closed backdrop and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.measure("mount", page.control("mount")).await?;
    page.count_stays(BACKDROP, 0, Duration::from_millis(100))
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.control("open").await?;
    page.count_stays(BACKDROP, 0, Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Mounting the open backdrop (the modal opening with it) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount_open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.element(OPENER).await?.click().await?;
    page.control("open").await?;
    page.measure("mount", page.control("mount")).await?;
    wait_for_open(page).await?;
    page.measure("unmount", page.control("unmount")).await?;
    wait_for_closed(page).await?;
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

/// Escape closing the modal.
#[browser_test]
pub async fn escape_closes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open(page).await?;
    page.measure("escape", page.send_keys(Key::Escape)).await?;
    wait_for_closed(page).await?;
    Ok(())
}

/// A click outside the modal (on the backdrop) closing it.
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
    page.count_stays(DIALOG, 1, Duration::from_millis(100))
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
    page.count_stays(DIALOG, 1, Duration::from_millis(100))
        .await?;
    page.measure("dismissable", page.control("dismissable"))
        .await?;
    Ok(())
}

// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ModalContent` (`/perf/atoms/modal_content`): what each thing it
//! offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/modal_content.json`): mounting it in its open backdrop (its dialog
//! focused) and unmounting it, Tab and Shift+Tab wrapping around inside it (it contains the
//! focus), its dismiss button closing the modal, and containing the focus stopped and started.
//! Every measurement after a mount starts after the dialog's refocusing ran (`use_dialog` blurs and
//! focuses the dialog again 500 ms after focusing it, a VoiceOver workaround).
use std::time::Duration;

use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/modal_content";
const CONTENT: &str = ".leptonic-ModalContent";
const DIALOG: &str = "[role=dialog]";

/// Registers the performance tests of `ModalContent`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(TabWraps))
        .with(fixture_test(DismissButtonCloses))
        .with(fixture_test(ContainFocus))
}

/// Waits until the dialog has the focus, past its refocusing (500 ms after it took the focus).
async fn wait_for_focused_dialog(page: &Page<'_>) -> Result<(), Report> {
    let dialog = page.element(DIALOG).await?;
    page.wait_for_focus(&dialog).await?;
    // Past the dialog's refocusing.
    page.focus_stays(&dialog, Duration::from_millis(600)).await
}

/// Mounting the content in the open backdrop (its dialog taking the focus) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    wait_for_focused_dialog(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(CONTENT, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    wait_for_focused_dialog(page).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(CONTENT, 0).await?;
    Ok(())
}

/// Tab from the last button wrapping around to the first, and Shift+Tab from the first to the
/// last.
#[browser_test]
pub async fn tab_wraps(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    wait_for_focused_dialog(page).await?;
    let first = page.element("#perf-first").await?;
    let last = page.element("#perf-last").await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&first).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&last).await?;
    page.measure("tab", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&first).await?;
    page.measure("shift tab", page.send_keys(Key::Shift + Key::Tab))
        .await?;
    page.wait_for_focus(&last).await?;
    Ok(())
}

/// The dismiss button (visually hidden, for screen readers) closing the modal.
#[browser_test]
pub async fn dismiss_button_closes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    wait_for_focused_dialog(page).await?;
    let dismiss = page
        .element(".leptonic-ModalContent button[aria-label=Dismiss]")
        .await?;
    page.measure("dismiss", dismiss.virtual_click()).await?;
    page.wait_for_count(CONTENT, 0).await?;
    Ok(())
}

/// Containing the focus stopped and started again.
#[browser_test]
pub async fn contain_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    wait_for_focused_dialog(page).await?;
    page.measure("uncontain", page.control("uncontain")).await?;
    page.measure("contain", page.control("contain")).await?;
    // Contained again: Tab from the last button wraps around.
    page.element("#perf-last").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#perf-first").await?)
        .await?;
    Ok(())
}

// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `DialogTrigger` (`/perf/atoms/dialog_trigger`): what each thing it
//! offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/dialog_trigger.json`): mounting and unmounting it, opening its modal
//! dialog with the mouse, with Enter and Space, and by the app's state, and closing it (Escape,
//! the app's state), the focus returning to the trigger. Every measurement after an opening starts
//! after the dialog's refocusing ran (`use_dialog` blurs and focuses the dialog again 500 ms after
//! focusing it, a VoiceOver workaround).
use std::time::Duration;

use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/dialog_trigger";
const DIALOG: &str = "[role=dialog]";
const TRIGGER: &str = "#perf-trigger";

/// Registers the performance tests of `DialogTrigger`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(OpenWithTheMouse))
        .with(fixture_test(OpenWithTheKeyboard))
        .with(fixture_test(OpenAndCloseByTheApp))
}

/// Waits until the dialog is open: it has the focus (past its refocusing, 500 ms after it took
/// the focus), the trigger is expanded and controls it.
async fn wait_for_open(page: &Page<'_>) -> Result<(), Report> {
    let dialog = page.element(DIALOG).await?;
    page.wait_for_focus(&dialog).await?;
    // Past the dialog's refocusing.
    page.focus_stays(&dialog, Duration::from_millis(600))
        .await?;
    let trigger = page.element(TRIGGER).await?;
    trigger.wait_for_attr("aria-expanded", Some("true")).await?;
    let id = dialog.attr("id").await?;
    trigger.wait_for_attr("aria-controls", id.as_deref()).await
}

/// Waits until the dialog is closed and the trigger has the focus back.
async fn wait_for_closed(page: &Page<'_>) -> Result<(), Report> {
    page.wait_for_count(DIALOG, 0).await?;
    let trigger = page.element(TRIGGER).await?;
    trigger
        .wait_for_attr("aria-expanded", Some("false"))
        .await?;
    page.wait_for_focus(&trigger).await
}

/// Mounting the trigger (closed) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(TRIGGER, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(TRIGGER)
        .await?
        .wait_for_attr("aria-expanded", Some("false"))
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TRIGGER, 0).await?;
    Ok(())
}

/// A click on the trigger opening the dialog, and Escape closing it.
#[browser_test]
pub async fn open_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = page.element(TRIGGER).await?;
    page.measure("click", trigger.click()).await?;
    wait_for_open(page).await?;
    page.measure("escape", page.send_keys(Key::Escape)).await?;
    wait_for_closed(page).await?;
    Ok(())
}

/// Enter opening the dialog from the focused trigger, Escape closing it, and Space opening it
/// again.
#[browser_test]
pub async fn open_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(TRIGGER).await?).await?;
    page.measure("enter", page.send_keys(Key::Enter)).await?;
    wait_for_open(page).await?;
    page.measure("escape", page.send_keys(Key::Escape)).await?;
    wait_for_closed(page).await?;
    page.measure("space", page.send_keys(" ")).await?;
    wait_for_open(page).await?;
    Ok(())
}

/// The app opening the dialog (the trigger focused before) and closing it.
#[browser_test]
pub async fn open_and_close_by_the_app(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(TRIGGER).await?).await?;
    page.measure("open", page.control("open")).await?;
    wait_for_open(page).await?;
    page.measure("close", page.control("close")).await?;
    wait_for_closed(page).await?;
    Ok(())
}

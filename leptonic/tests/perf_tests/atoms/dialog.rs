// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `Dialog` (`/perf/atoms/dialog`): what each thing it offers costs,
//! measured (`Page::measure`) and checked against its budget (`tests/budgets/atoms/dialog.json`):
//! mounting it (it focuses itself) and unmounting it, its name and description changing, and the
//! focus leaving it. Every measurement starts after the dialog's refocusing ran (`use_dialog`
//! blurs and focuses the dialog again 500 ms after focusing it, a VoiceOver workaround).
use std::time::Duration;

use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/dialog";
const DIALOG: &str = "[role=dialog]";

/// Registers the performance tests of `Dialog`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(LabelChanges))
        .with(fixture_test(DescriptionChanges))
        .with(fixture_test(TabThrough))
}

/// Waits until the dialog has the focus and its refocusing (500 ms after it took the focus) ran:
/// it keeps the focus throughout.
async fn wait_for_focused_dialog(page: &Page<'_>) -> Result<WebElement, Report> {
    let dialog = page.element(DIALOG).await?;
    page.wait_for_focus(&dialog).await?;
    // Past the refocusing's 500 ms timer.
    page.focus_stays(&dialog, Duration::from_millis(600))
        .await?;
    Ok(dialog)
}

/// The id of the dialog's title.
async fn title_id(page: &Page<'_>) -> Result<Option<String>, Report> {
    Ok(page
        .element(".leptonic-DialogTitle")
        .await?
        .attr("id")
        .await?)
}

/// Mounting the dialog (it takes the focus) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    wait_for_focused_dialog(page).await?;
    page.control("unmount").await?;
    page.wait_for_count(DIALOG, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    let dialog = wait_for_focused_dialog(page).await?;
    let title = title_id(page).await?;
    assert_that!(dialog.attr("aria-labelledby").await?).is_equal_to(title);
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(DIALOG, 0).await?;
    Ok(())
}

/// An `aria_label` naming the dialog instead of its title, and back.
#[browser_test]
pub async fn label_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let dialog = wait_for_focused_dialog(page).await?;
    page.measure("label", page.control("label")).await?;
    dialog
        .wait_for_attr("aria-label", Some("Settings dialog"))
        .await?;
    dialog.wait_for_attr("aria-labelledby", None).await?;
    page.measure("unlabel", page.control("unlabel")).await?;
    dialog.wait_for_attr("aria-label", None).await?;
    let title = title_id(page).await?;
    dialog
        .wait_for_attr("aria-labelledby", title.as_deref())
        .await?;
    Ok(())
}

/// An `aria_describedby` describing the dialog by an element outside it, and unset again.
#[browser_test]
pub async fn description_changes(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let dialog = wait_for_focused_dialog(page).await?;
    page.measure("describe", page.control("describe")).await?;
    dialog
        .wait_for_attr("aria-describedby", Some("perf-external-description"))
        .await?;
    page.measure("undescribe", page.control("undescribe"))
        .await?;
    dialog.wait_for_attr("aria-describedby", None).await?;
    Ok(())
}

/// Tab moving the focus from the dialog to its button, and out of the dialog.
#[browser_test]
pub async fn tab_through(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    wait_for_focused_dialog(page).await?;
    page.measure("tab to its button", page.send_keys(Key::Tab))
        .await?;
    page.wait_for_focus(&page.element("#perf-dialog-button").await?)
        .await?;
    page.measure("tab out", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&page.element("#perf-after").await?)
        .await?;
    Ok(())
}

// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `DialogTitle` (`/perf/atoms/dialog_title`): what it costs, measured
//! (`Page::measure`) and checked against its budget (`tests/budgets/atoms/dialog_title.json`):
//! mounting it (it names its dialog) and unmounting it. Every measurement starts after the
//! dialog's refocusing ran (`use_dialog` blurs and focuses the dialog again 500 ms after focusing
//! it, a VoiceOver workaround).
use std::time::Duration;

use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/dialog_title";
const DIALOG: &str = "[role=dialog]";
const TITLE: &str = ".leptonic-DialogTitle";

/// Registers the performance tests of `DialogTitle`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group.with(fixture_test(MountAndUnmount))
}

/// Mounting the title (naming the dialog) and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let dialog = page.element(DIALOG).await?;
    page.wait_for_focus(&dialog).await?;
    // Past the dialog's refocusing (500 ms after it took the focus).
    page.focus_stays(&dialog, Duration::from_millis(600))
        .await?;
    page.control("unmount").await?;
    dialog.wait_for_attr("aria-labelledby", None).await?;
    page.measure("mount", page.control("mount")).await?;
    let title = page.element(TITLE).await?;
    let id = title.attr("id").await?;
    assert_that!(id.is_some()).is_true();
    dialog
        .wait_for_attr("aria-labelledby", id.as_deref())
        .await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(TITLE, 0).await?;
    dialog.wait_for_attr("aria-labelledby", None).await?;
    Ok(())
}

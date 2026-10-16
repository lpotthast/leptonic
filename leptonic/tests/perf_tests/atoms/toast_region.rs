// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `ToastRegion` (`/perf/atoms/toast_region`, with toasts of
//! `Toast`, `ToastContent`, `ToastTitle`, `ToastDescription` and `ToastCloseButton`): what each
//! thing it offers costs, measured (`Page::measure`) and checked against its budget
//! (`tests/budgets/atoms/toast_region.json`): appearing with the first toast and leaving with
//! the last, mounting and unmounting it with 1 and 50 toasts, adding and closing a toast among
//! 50, hover and focus (pausing the timeouts), moving focus to the remaining toast when the
//! focused one closes, and changes of its label. No timeout runs out while measuring: the timed
//! toasts last an hour.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/atoms/toast_region";

/// Registers the performance tests of `ToastRegion`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(FirstToastAndLastClose))
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(MountAndUnmountMany))
        .with(fixture_test(AddAndCloseMany))
        .with(fixture_test(HoverAndLeave))
        .with(fixture_test(KeyboardFocusAndLeave))
        .with(fixture_test(CloseTheFocusedToast))
        .with(fixture_test(ChangeTheLabel))
}
const REGION: &str = ".leptonic-ToastRegion";
const TOAST: &str = "[role=alertdialog]";

/// Adds a toast without a timeout and waits until the region shows `count` toasts.
async fn add(page: &Page<'_>, count: usize) -> Result<(), Report> {
    page.control("add").await?;
    page.wait_for_count(TOAST, count).await
}

/// The region appearing with the first toast, and leaving with the last.
#[browser_test]
pub async fn first_toast_and_last_close(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count(REGION, 0).await?;
    page.measure("first toast", page.control("add")).await?;
    page.wait_for_count(TOAST, 1).await?;
    page.measure("last close", page.control("close-newest"))
        .await?;
    page.wait_for_count(REGION, 0).await?;
    assert_that!(page.control("closes").await?).is_equal_to(1.0);
    Ok(())
}

/// Unmounting the region with a toast, and mounting it again.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    add(page, 1).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(REGION, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(TOAST, 1).await?;
    Ok(())
}

/// Unmounting the region with 50 toasts, and mounting it again.
#[browser_test]
pub async fn mount_and_unmount_many(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("fill").await?;
    page.wait_for_count(TOAST, 50).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(REGION, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_count(TOAST, 50).await?;
    Ok(())
}

/// Adding a toast to 50 and closing it again.
#[browser_test]
pub async fn add_and_close_many(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("fill").await?;
    page.wait_for_count(TOAST, 50).await?;
    page.measure("add", page.control("add")).await?;
    page.wait_for_count(TOAST, 51).await?;
    page.measure("close", page.control("close-newest")).await?;
    page.wait_for_count(TOAST, 50).await?;
    Ok(())
}

/// The pointer entering the region (pausing the timeout of its toast) and leaving it (resuming
/// it).
#[browser_test]
pub async fn hover_and_leave(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("add-timed").await?;
    let toast = page.element(TOAST).await?;
    let after = page.element("#perf-after").await?;
    page.measure("hover", toast.hover()).await?;
    page.measure("leave", after.hover()).await?;
    page.wait_for_count(TOAST, 1).await?;
    Ok(())
}

/// F6 moving focus to the region (pausing the timeout of its toast), and a click elsewhere
/// taking it away (resuming it).
#[browser_test]
pub async fn keyboard_focus_and_leave(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("add-timed").await?;
    let region = page.element(REGION).await?;
    page.element("#perf-before").await?.click().await?;
    page.measure("f6", page.send_keys(Key::F6)).await?;
    page.wait_for_focus(&region).await?;
    region.wait_for_attr("data-focused", Some("true")).await?;
    let after = page.element("#perf-after").await?;
    page.measure("leave", after.click()).await?;
    page.wait_for_focus(&after).await?;
    region.wait_for_attr("data-focused", None).await?;
    Ok(())
}

/// Closing the focused (newest) of two toasts with its close button and Enter: the focus moves
/// to the remaining toast.
#[browser_test]
pub async fn close_the_focused_toast(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    add(page, 1).await?;
    add(page, 2).await?;
    let toasts = page.elements(TOAST).await?;
    let (newest, remaining) = (&toasts[0], &toasts[1]);
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::F6).await?;
    page.wait_for_focus(&page.element(REGION).await?).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(newest).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&newest.element("button").await?)
        .await?;
    page.measure("close", page.send_keys(Key::Enter)).await?;
    page.wait_for_count(TOAST, 1).await?;
    page.wait_for_focus(remaining).await?;
    Ok(())
}

/// Labeling the region and removing the label again (its default label, counting its toasts).
#[browser_test]
pub async fn change_the_label(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    add(page, 1).await?;
    page.measure("set", page.control("set-label")).await?;
    page.wait_for_attr(REGION, "aria-label", Some("Alerts"))
        .await?;
    page.measure("unset", page.control("unset-label")).await?;
    page.wait_for_attr(REGION, "aria-label", Some("1 notification."))
        .await?;
    Ok(())
}

// Upstream: react-aria-components/test/Toast.test.js @ 99e6102368
//! The toast atoms on one queue: a region (landmark, "n notifications.") showing alert dialogs
//! labelled by their title and described by their description; closing by button, timeout
//! (paused while hovered or focused) or programmatically; F6 reaches the region; the focus moves
//! to a remaining toast and finally back to where it came from.
use std::time::Duration;

use assertr::{
    matchers::{eq, one_of},
    prelude::*,
};
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page, PointerKind, StopwatchEnd};

const TOAST: &str = "[role=alertdialog]";
const REGION: &str = "[role=region]";
const PATH: &str = "/atoms/toast";
const SINGLE: &str = "/atoms/toast-single";

/// Wait until `toast` is named `title` (its title mounts after the toast).
async fn wait_for_title(toast: &WebElement, title: &str) -> Result<(), Report> {
    assert_that!(|| toast.accessible_name())
        .eventually_ok()
        .matches(eq(title))
        .await;
    Ok(())
}

/// Each added toast shows first in a region counting the notifications, as an alert dialog named by
/// its title and described by its description; closing the toasts with their "Close" buttons
/// returns focus to the trigger ("should trigger a toast", "should restore focus when removing with
/// the mouse").
#[browser_test]
pub async fn trigger_and_close(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let add = page.element("#test-toast-add").await?;
    assert_that!(page.count(TOAST).await?).is_equal_to(0);
    assert_that!(page.count(REGION).await?).is_equal_to(0);
    add.click().await?;
    let toast = page.element(TOAST).await?;
    let region = page.element(REGION).await?;
    assert_that!(region)
        .has_attribute("aria-label")
        .await
        .is_equal_to("1 notification.");
    assert_that!(region)
        .has_attribute("class")
        .await
        .contains("test-toast-region");
    assert_that!(toast)
        .has_attribute("aria-modal")
        .await
        .is_equal_to("false");
    wait_for_title(&toast, "Toast 1").await?;
    assert_that!(toast)
        .accessible_description()
        .await
        .is_equal_to("Description");
    let alert = toast.element("[role=alert]").await?;
    alert.wait_for_attr("aria-hidden", None).await?;
    assert_that!(alert).displayed().await.is_true();

    // A second toast: the newest comes first; the region counts them.
    add.click().await?;
    region
        .wait_for_attr("aria-label", Some("2 notifications."))
        .await?;
    let newest = page.first_element(TOAST).await?;
    wait_for_title(&newest, "Toast 2").await?;

    // Closing with the mouse returns the focus to where it came from ("should restore focus when
    // removing with the mouse").
    let close = newest.element("button").await?;
    assert_that!(close)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Close");
    close.click().await?;
    region
        .wait_for_attr("aria-label", Some("1 notification."))
        .await?;
    page.wait_for_focus(&add).await?;
    // Keep the original toast's identity while the newest may still be exiting.
    toast.element("button").await?.click().await?;
    page.wait_for_count(TOAST, 0).await?;
    page.wait_for_count(REGION, 0).await?;
    page.wait_for_focus(&add).await?;
    page.element("#test-toast-closed")
        .await?
        .wait_for_inner_text("2")
        .await?;
    Ok(())
}

/// A toast with a timeout closes on its own, but not while it is hovered or focused ("removes a
/// toast via timeout", "pauses timers when hovering", "pauses timers when focusing").
#[browser_test]
pub async fn timeouts(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let add_timed = page.element("#test-toast-add-timed").await?;
    let closed = page.element("#test-toast-closed").await?;
    add_timed.click().await?;
    page.element(TOAST).await?;
    page.wait_for_count(TOAST, 0).await?;
    closed.wait_for_inner_text("1").await?;

    // Hovered, it stays (twice its timeout of 500 ms); left, it closes after the rest of its time.
    add_timed.click().await?;
    page.element(TOAST).await?.hover().await?;
    page.settle().await?;
    assert_that!(|| page.count(TOAST))
        .consistently_ok()
        .for_at_least(Duration::from_millis(1000))
        .matches(eq(1))
        .await;
    closed.hover().await?;
    page.wait_for_count(TOAST, 0).await?;

    // Focused, it stays; blurred, it closes.
    add_timed.click().await?;
    page.element(format!("{TOAST} button"))
        .await?
        .focus()
        .await?;
    page.settle().await?;
    assert_that!(|| page.count(TOAST))
        .consistently_ok()
        .for_at_least(Duration::from_millis(1000))
        .matches(eq(1))
        .await;
    page.element("#test-toast-add").await?.focus().await?;
    page.wait_for_count(TOAST, 0).await?;
    closed.wait_for_inner_text("3").await?;
    Ok(())
}

/// F6 focuses the toast region, and closing a toast by keyboard moves focus to the remaining toast,
/// then back to the trigger ("can focus toast region using F6", "should move focus to remaining
/// toast when a toast exits and there are more").
#[browser_test]
pub async fn keyboard_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let add = page.element("#test-toast-add").await?;
    add.click().await?;
    let remaining = page.element(TOAST).await?;
    page.wait_for_focus(&add).await?;
    page.send_keys(Key::Enter).await?;
    let region = page.element(REGION).await?;
    region
        .wait_for_attr("aria-label", Some("2 notifications."))
        .await?;

    page.send_keys(Key::F6).await?;
    page.wait_for_focus(&region).await?;
    page.send_keys(Key::Tab).await?;
    let newest = page.first_element(TOAST).await?;
    page.wait_for_focus(&newest).await?;
    page.send_keys(Key::Tab).await?;
    let close = newest.element("button").await?;
    page.wait_for_focus(&close).await?;
    page.send_keys(Key::Enter).await?;

    // The focus moves to the remaining toast.
    region
        .wait_for_attr("aria-label", Some("1 notification."))
        .await?;
    page.wait_for_focus(&remaining).await?;
    wait_for_title(&remaining, "Toast 1").await?;
    remaining
        .wait_for_attr("data-focused", Some("true"))
        .await?;
    remaining
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Enter).await?;

    // And then back to where it came from.
    page.wait_for_count(TOAST, 0).await?;
    page.wait_for_focus(&add).await?;
    Ok(())
}

/// Closing toasts programmatically removes them, and the region with the last one ("should support
/// programmatically closing toasts").
#[browser_test]
pub async fn programmatic_close(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let add = page.element("#test-toast-add").await?;
    let close_newest = page.element("#test-toast-close-newest").await?;
    add.click().await?;
    add.click().await?;
    let region = page.element(REGION).await?;
    region
        .wait_for_attr("aria-label", Some("2 notifications."))
        .await?;
    close_newest.click().await?;
    region
        .wait_for_attr("aria-label", Some("1 notification."))
        .await?;
    wait_for_title(&page.element(TOAST).await?, "Toast 1").await?;
    page.element("#test-toast-closed")
        .await?
        .wait_for_inner_text("1")
        .await?;
    close_newest.click().await?;
    page.wait_for_count(REGION, 0).await?;
    Ok(())
}

/// After a hover pause, a toast's timeout continues with the time that was left instead of starting
/// over ("pauses timers when hovering").
#[browser_test]
pub async fn remaining_time_after_pause(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-toast-add-slow").await?.click().await?;
    let toast = page.element(TOAST).await?;
    // A real timer: a second of the toast's timeout runs before the pause.
    tokio::time::sleep(Duration::from_millis(1000)).await;
    toast.hover().await?;
    page.settle().await?;
    // Together with the 1.2 s after leaving, longer than the 2 s left: a timer not paused would
    // close the toast within these checks.
    assert_that!(|| page.count(TOAST))
        .consistently_ok()
        .for_at_least(Duration::from_millis(1200))
        .matches(eq(1))
        .await;
    // From the pointer leaving the toasts to the toast closing, on the page's clock.
    let stopwatch = page
        .start_stopwatch(&toast, PointerKind::Leave, StopwatchEnd::Disappears(TOAST))
        .await?;
    page.element("#test-toast-closed").await?.hover().await?;
    page.settle().await?;
    assert_that!(|| page.count(TOAST))
        .consistently_ok()
        .for_at_least(Duration::from_millis(1200))
        .matches(eq(1))
        .await;
    page.wait_for_count(TOAST, 0).await?;
    // At most 2 seconds were left; a restarted timeout would take 3.
    assert_that!(stopwatch.finish().await?)
        .with_detail_message("a restarted timeout would take 3 s")
        .is_less_than(Duration::from_millis(2500));
    Ok(())
}

/// With one toast at a time, the region (named "Alerts") shows only the newest toast, and closing
/// it by keyboard shows and focuses the next ("moves focus to the next toast when it appears",
/// "should support custom aria-label").
#[browser_test]
pub async fn one_at_a_time(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(SINGLE).await?;
    let add = page.element("#test-toast-single-add").await?;
    add.click().await?;
    add.click().await?;
    let shown = page.element(TOAST).await?;
    let region = page.element(REGION).await?;
    assert_that!(region)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Alerts");
    wait_for_title(&shown, "Alert 2").await?;
    page.count_stays(TOAST, 1, std::time::Duration::from_millis(100))
        .await?;

    // Closing it by keyboard shows the next one and focuses it.
    page.send_keys(Key::F6).await?;
    page.wait_for_focus(&region).await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Enter).await?;
    assert_that!(|| shown_toast_title(page))
        .eventually()
        .matches(eq(Some("Alert 1".to_owned())))
        .await;
    page.wait_for_focus(&page.element(TOAST).await?).await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Enter).await?;
    page.wait_for_count(TOAST, 0).await?;
    page.wait_for_focus(&add).await?;
    Ok(())
}

/// The title of the toast shown now, if any. While the toasts are being replaced, a toast or title
/// that is gone while reading counts as none.
async fn shown_toast_title(page: &Page<'_>) -> Option<String> {
    let toast = page.elements(TOAST).await.ok()?.into_iter().next()?;
    let title = toast.accessible_name().await.ok()?;
    Some(title).filter(|title| !title.is_empty())
}

/// A focused toast keeps focus when a new toast arrives above it, and when it closes, focus moves
/// to a remaining toast ("should move focus to remaining toast when a toast exits and there are
/// more").
#[browser_test]
pub async fn focused_toast_after_new_toast(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let add = page.element("#test-toast-add").await?;
    add.click().await?;
    add.click().await?;
    page.wait_for_count(TOAST, 2).await?;
    // By keyboard (the focus moves to the next toast in keyboard modality only): schedule the new
    // toast and the closing, then focus the oldest toast ("Toast 1").
    page.element("#test-toast-add-then-close-oldest")
        .await?
        .focus()
        .await?;
    page.send_keys(Key::Enter).await?;
    page.send_keys(Key::F6).await?;
    page.wait_for_focus(&page.element(REGION).await?).await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Tab).await?;
    let toasts = page.elements(TOAST).await?;
    assert_that!(&toasts)
        .with_detail_message("when focusing the oldest toast, before the new one arrives")
        .has_length(2);
    let oldest = &toasts[1];
    page.wait_for_focus(oldest).await?;
    assert_that!(page.count(TOAST).await?)
        .with_detail_message("the new toast must arrive after the focus moved (test too slow?)")
        .is_equal_to(2);
    wait_for_title(oldest, "Toast 1").await?;

    // A new toast arrives above; the focus stays on "Toast 1".
    page.wait_for_count(TOAST, 3).await?;
    page.wait_for_focus(oldest).await?;
    // "Toast 1" closes: the focus moves to a remaining toast instead of being lost.
    page.wait_for_count(TOAST, 2).await?;
    let mut remaining = Vec::new();
    for toast in page.elements(TOAST).await? {
        remaining.push(toast.describe().await?);
    }
    assert_that!(|| async { page.focused_element().await?.describe().await })
        .eventually_ok()
        // One of the remaining toasts.
        .matches(one_of(&remaining))
        .await;
    page.element("#test-toast-closed")
        .await?
        .wait_for_inner_text("1")
        .await?;
    Ok(())
}

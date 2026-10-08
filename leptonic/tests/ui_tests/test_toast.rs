// Upstream: react-aria-components/test/Toast.test.js @ 99e6102368
//! The toast atoms on one queue: a region (landmark, "n notifications.") showing alert dialogs
//! labelled by their title and described by their description; closing by button, timeout
//! (paused while hovered or focused) or programmatically; F6 reaches the region; the focus moves
//! to a remaining toast and finally back to where it came from.
use std::time::{Duration, Instant};

use assertr::prelude::*;
use browser_test::thirtyfour::prelude::*;
use rootcause::{Report, bail};

use crate::{
    pages::{ElementActions, Page, PageActions},
    polling::{expect, wait_for},
};

const TOAST: &str = "[role=alertdialog]";
const REGION: &str = "[role=region]";

/// Wait until `toast` is named `title` (its title mounts after the toast).
async fn wait_for_title(toast: &WebElement, title: &str) -> Result<(), Report> {
    wait_for("the toast's title")
        .observing(|| toast.referenced_text("aria-labelledby"))
        .to_be_equal_to(title)
        .await?;
    Ok(())
}

/// "should trigger a toast", "should restore focus when a toast exits".
pub async fn trigger_and_close(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/toast").await?;
    let add = page.element("#test-toast-add").await?;
    assert_that!(page.count(TOAST).await?).is_equal_to(0);
    assert_that!(page.count(REGION).await?).is_equal_to(0);
    add.click().await?;
    let toast = page.element(TOAST).await?;
    let region = page.element(REGION).await?;
    assert_that!(region.attr("aria-label").await?)
        .get_some()
        .is_equal_to("1 notification.");
    assert_that!(region.class_name().await?)
        .get_some()
        .contains("test-toast-region");
    assert_that!(toast.attr("aria-modal").await?)
        .get_some()
        .is_equal_to("false");
    wait_for_title(&toast, "Toast 1").await?;
    assert_that!(toast.referenced_text("aria-describedby").await?).is_equal_to("Description");
    let alert = toast.element("[role=alert]").await?;
    alert.wait_for_attr("aria-hidden", None).await?;
    assert_that!(alert.is_displayed().await?).is_true();

    // A second toast: the newest comes first; the region counts them.
    add.click().await?;
    region
        .wait_for_attr("aria-label", Some("2 notifications."))
        .await?;
    let newest = page.element(TOAST).await?;
    wait_for_title(&newest, "Toast 2").await?;

    // Closing with the mouse returns the focus to where it came from ("should restore focus when
    // removing with the mouse").
    let close = newest.element("button").await?;
    assert_that!(close.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Close");
    close.click().await?;
    region
        .wait_for_attr("aria-label", Some("1 notification."))
        .await?;
    page.wait_for_focus(&add).await?;
    page.element(format!("{TOAST} button"))
        .await?
        .click()
        .await?;
    page.wait_for_count(TOAST, 0).await?;
    page.wait_for_count(REGION, 0).await?;
    page.wait_for_focus(&add).await?;
    page.element("#test-toast-closed")
        .await?
        .wait_for_inner_text("2")
        .await?;
    Ok(())
}

/// "removes a toast via timeout", "pauses timers when hovering", "pauses timers when focusing".
pub async fn timeouts(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/toast").await?;
    let add_timed = page.element("#test-toast-add-timed").await?;
    let closed = page.element("#test-toast-closed").await?;
    add_timed.click().await?;
    page.element(TOAST).await?;
    page.wait_for_count(TOAST, 0).await?;
    closed.wait_for_inner_text("1").await?;

    // Hovered, it stays; left, it closes after the rest of its time.
    add_timed.click().await?;
    page.element(TOAST).await?.hover().await?;
    expect("the number of open toasts (hovered)")
        .observing(|| page.count(TOAST))
        .for_at_least(Duration::from_millis(2500))
        .to_stay_equal_to(1)
        .await?;
    closed.hover().await?;
    page.wait_for_count(TOAST, 0).await?;

    // Focused, it stays; blurred, it closes.
    add_timed.click().await?;
    page.element(format!("{TOAST} button"))
        .await?
        .focus()
        .await?;
    expect("the number of open toasts (focused)")
        .observing(|| page.count(TOAST))
        .for_at_least(Duration::from_millis(2500))
        .to_stay_equal_to(1)
        .await?;
    page.element("#test-toast-add").await?.focus().await?;
    page.wait_for_count(TOAST, 0).await?;
    closed.wait_for_inner_text("3").await?;
    Ok(())
}

/// "can focus toast region using F6", "should move focus to remaining toast when a toast exits
/// and there are more".
pub async fn keyboard_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/toast").await?;
    let add = page.element("#test-toast-add").await?;
    add.click().await?;
    page.element(TOAST).await?;
    page.wait_for_focus(&add).await?;
    page.send_keys(Key::Enter).await?;
    let region = page.element(REGION).await?;
    region
        .wait_for_attr("aria-label", Some("2 notifications."))
        .await?;

    page.send_keys(Key::F6).await?;
    page.wait_for_focus(&region).await?;
    page.send_keys(Key::Tab).await?;
    let newest = page.element(TOAST).await?;
    page.wait_for_focus(&newest).await?;
    page.send_keys(Key::Tab).await?;
    let close = newest.element("button").await?;
    page.wait_for_focus(&close).await?;
    page.send_keys(Key::Enter).await?;

    // The focus moves to the remaining toast.
    region
        .wait_for_attr("aria-label", Some("1 notification."))
        .await?;
    let remaining = page.element(TOAST).await?;
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

/// "should support programmatically closing toasts".
pub async fn programmatic_close(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/toast").await?;
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

/// "pauses timers when hovering", with upstream's timing: the timeout continues with the time that
/// was left, it doesn't start over.
pub async fn remaining_time_after_pause(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/toast").await?;
    page.element("#test-toast-add-slow").await?.click().await?;
    let toast = page.element(TOAST).await?;
    // A real timer: a second of the toast's timeout runs before the pause.
    tokio::time::sleep(Duration::from_millis(1000)).await;
    toast.hover().await?;
    expect("the number of open toasts (hovered)")
        .observing(|| page.count(TOAST))
        .for_at_least(Duration::from_millis(2500))
        .to_stay_equal_to(1)
        .await?;
    page.element("#test-toast-closed").await?.hover().await?;
    let left = Instant::now();
    expect("the number of open toasts (left, time remaining)")
        .observing(|| page.count(TOAST))
        .for_at_least(Duration::from_millis(1200))
        .to_stay_equal_to(1)
        .await?;
    page.wait_for_count(TOAST, 0).await?;
    // About 2 seconds were left; a restarted timeout would take 3.
    assert_that!(left.elapsed())
        .with_detail_message("a restarted timeout would take 3 s")
        .is_less_than(Duration::from_millis(2700));
    Ok(())
}

/// react-aria's `useToast.test.js` "moves focus to the next toast when it appears", and "should
/// support custom aria-label": one toast at a time (`use_toast_state`'s default) in a region named
/// "Alerts".
pub async fn one_at_a_time(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/toast-single").await?;
    let add = page.element("#test-toast-single-add").await?;
    add.click().await?;
    add.click().await?;
    let shown = page.element(TOAST).await?;
    let region = page.element(REGION).await?;
    assert_that!(region.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Alerts");
    assert_that!(page.count(TOAST).await?).is_equal_to(1);
    wait_for_title(&shown, "Alert 2").await?;

    // Closing it by keyboard shows the next one and focuses it.
    page.send_keys(Key::F6).await?;
    page.wait_for_focus(&region).await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Enter).await?;
    wait_for("the shown toast's title")
        .observing(|| async { Ok(shown_toast_title(page).await) })
        .to_be_equal_to(Some("Alert 1".to_owned()))
        .await?;
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
    let title = toast.referenced_text("aria-labelledby").await.ok()?;
    Some(title).filter(|title| !title.is_empty())
}

/// The focused toast is tracked by its key, not its index: a new toast above it doesn't make the
/// region lose it, so when it closes the focus still moves to a remaining toast ("should move focus
/// to remaining toast when a toast exits and there are more").
pub async fn focused_toast_after_new_toast(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/toast").await?;
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
    let [_, oldest] = toasts.as_slice() else {
        bail!(
            "expected 2 toasts when focusing the oldest (the new one must arrive after the focus \
             moved), found {}",
            toasts.len()
        );
    };
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
    wait_for("the focused element")
        .observing(|| async { page.focused_element().await?.describe().await })
        .to_be(
            &format!("one of the remaining toasts {remaining:?}"),
            |focused| remaining.contains(focused),
        )
        .await?;
    page.element("#test-toast-closed")
        .await?
        .wait_for_inner_text("1")
        .await?;
    Ok(())
}

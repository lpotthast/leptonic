// Upstream: react-aria-components/test/Toast.test.js @ 99e6102368
use std::{borrow::Cow, time::Duration};

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// The toast atoms on one queue: a region (landmark, "n notifications.") showing alert dialogs
/// labelled by their title and described by their description; closing by button, timeout
/// (paused while hovered or focused) or programmatically; F6 reaches the region; the focus moves
/// to a remaining toast and finally back to where it came from.
pub struct ToastTests {}

#[async_trait]
impl BrowserTest<str> for ToastTests {
    fn name(&self) -> Cow<'_, str> {
        "toast_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        trigger_and_close(&page).await?;
        timeouts(&page).await?;
        keyboard_focus(&page).await?;
        programmatic_close(&page).await?;
        remaining_time_after_pause(&page).await?;
        one_at_a_time(&page).await?;
        page.expect_no_page_errors().await
    }
}

/// "should trigger a toast", "should restore focus when a toast exits".
async fn trigger_and_close(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/toast").await?;
    assert_that!(page.count_matching("[role=alertdialog]").await?).is_equal_to(0);
    assert_that!(page.count_matching("[role=region]").await?).is_equal_to(0);
    page.click_element_with_id("test-toast-add").await?;
    page.wait_for_selector("[role=alertdialog]").await?;

    let region = page.css("[role=region]").await?;
    assert_that!(region.attr("aria-label").await?).is_equal_to(Some("1 notification.".to_owned()));
    assert_that!(region.class_name().await?.unwrap_or_default()).contains("test-toast-region");
    let toast = page.css("[role=alertdialog]").await?;
    assert_that!(toast.attr("aria-modal").await?).is_equal_to(Some("false".to_owned()));
    assert_that!(referenced_text(page, &toast, "aria-labelledby").await?)
        .is_equal_to("Toast 1".to_owned());
    let describedby = wait_for_some_attr(&toast, "aria-describedby").await?;
    assert_that!(page.element(&describedby).await?.text().await?)
        .is_equal_to("Description".to_owned());
    let alert = toast.find(By::Css("[role=alert]")).await?;
    page.wait_for_attr(&alert, "aria-hidden", None).await?;
    assert_that!(alert.is_displayed().await?).is_true();

    // A second toast: the newest comes first; the region counts them.
    page.click_element_with_id("test-toast-add").await?;
    page.wait_for_attr(&region, "aria-label", Some("2 notifications."))
        .await?;
    let first = page.css("[role=alertdialog]").await?;
    assert_that!(referenced_text(page, &first, "aria-labelledby").await?)
        .is_equal_to("Toast 2".to_owned());

    // Closing with the mouse returns the focus to where it came from ("should restore focus when
    // removing with the mouse").
    let close = first.find(By::Css("button")).await?;
    assert_that!(close.attr("aria-label").await?).is_equal_to(Some("Close".to_owned()));
    close.click().await?;
    page.wait_for_attr(&region, "aria-label", Some("1 notification."))
        .await?;
    page.wait_for_active_id("test-toast-add").await?;
    page.css("[role=alertdialog] button").await?.click().await?;
    page.wait_for_no_selector("[role=alertdialog]").await?;
    page.wait_for_no_selector("[role=region]").await?;
    page.wait_for_active_id("test-toast-add").await?;
    page.wait_for_text("test-toast-closed", "2").await
}

/// "removes a toast via timeout", "pauses timers when hovering", "pauses timers when focusing".
async fn timeouts(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/toast").await?;
    page.click_element_with_id("test-toast-add-timed").await?;
    page.wait_for_selector("[role=alertdialog]").await?;
    page.wait_for_no_selector("[role=alertdialog]").await?;
    page.wait_for_text("test-toast-closed", "1").await?;

    // Hovered, it stays; left, it closes after the rest of its time.
    page.click_element_with_id("test-toast-add-timed").await?;
    page.wait_for_selector("[role=alertdialog]").await?;
    hover(page, &page.css("[role=alertdialog]").await?).await?;
    tokio::time::sleep(Duration::from_millis(2500)).await;
    assert_that!(page.count_matching("[role=alertdialog]").await?).is_equal_to(1);
    hover(page, &page.element("test-toast-closed").await?).await?;
    page.wait_for_no_selector("[role=alertdialog]").await?;

    // Focused, it stays; blurred, it closes.
    page.click_element_with_id("test-toast-add-timed").await?;
    page.wait_for_selector("[role=alertdialog]").await?;
    page.driver
        .execute(
            "document.querySelector('[role=alertdialog] button').focus()",
            vec![],
        )
        .await?;
    tokio::time::sleep(Duration::from_millis(2500)).await;
    assert_that!(page.count_matching("[role=alertdialog]").await?).is_equal_to(1);
    page.driver
        .execute("document.getElementById('test-toast-add').focus()", vec![])
        .await?;
    page.wait_for_no_selector("[role=alertdialog]").await?;
    page.wait_for_text("test-toast-closed", "3").await
}

/// "can focus toast region using F6", "should move focus to remaining toast when a toast exits
/// and there are more".
async fn keyboard_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/toast").await?;
    page.click_element_with_id("test-toast-add").await?;
    page.wait_for_selector("[role=alertdialog]").await?;
    page.wait_for_active_id("test-toast-add").await?;
    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_attr(
        &page.css("[role=region]").await?,
        "aria-label",
        Some("2 notifications."),
    )
    .await?;

    page.send_keys_to_active(Key::F6).await?;
    page.wait_for_focus("region", None).await?;
    page.press_tab().await?;
    let newest = page.css("[role=alertdialog]").await?;
    page.wait_for_focus_on(&newest, "the newest toast").await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&newest.find(By::Css("button")).await?, "its close button")
        .await?;
    page.send_keys_to_active(Key::Enter).await?;

    // The focus moves to the remaining toast.
    page.wait_for_attr(
        &page.css("[role=region]").await?,
        "aria-label",
        Some("1 notification."),
    )
    .await?;
    let remaining = page.css("[role=alertdialog]").await?;
    page.wait_for_focus_on(&remaining, "the remaining toast")
        .await?;
    assert_that!(referenced_text(page, &remaining, "aria-labelledby").await?)
        .is_equal_to("Toast 1".to_owned());
    page.wait_for_attr(&remaining, "data-focused", Some("true"))
        .await?;
    page.wait_for_attr(&remaining, "data-focus-visible", Some("true"))
        .await?;
    page.press_tab().await?;
    page.send_keys_to_active(Key::Enter).await?;

    // And then back to where it came from.
    page.wait_for_no_selector("[role=alertdialog]").await?;
    page.wait_for_active_id("test-toast-add").await
}

/// "should support programmatically closing toasts".
async fn programmatic_close(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/toast").await?;
    page.click_element_with_id("test-toast-add").await?;
    page.click_element_with_id("test-toast-add").await?;
    page.wait_for_attr(
        &page.css("[role=region]").await?,
        "aria-label",
        Some("2 notifications."),
    )
    .await?;
    page.click_element_with_id("test-toast-close-newest")
        .await?;
    page.wait_for_attr(
        &page.css("[role=region]").await?,
        "aria-label",
        Some("1 notification."),
    )
    .await?;
    let remaining = page.css("[role=alertdialog]").await?;
    assert_that!(referenced_text(page, &remaining, "aria-labelledby").await?)
        .is_equal_to("Toast 1".to_owned());
    page.wait_for_text("test-toast-closed", "1").await?;
    page.click_element_with_id("test-toast-close-newest")
        .await?;
    page.wait_for_no_selector("[role=region]").await
}

async fn referenced_text(
    page: &Page<'_>,
    element: &WebElement,
    attr: &str,
) -> Result<String, Report> {
    let id = wait_for_some_attr(element, attr).await?;
    Ok(page.element(&id).await?.text().await?)
}

/// The attribute once set (references to slots appear after mounting).
async fn wait_for_some_attr(element: &WebElement, attr: &str) -> Result<String, Report> {
    for _ in 0..50 {
        if let Some(value) = element.attr(attr).await? {
            return Ok(value);
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    rootcause::bail!("{attr} never set")
}

async fn hover(page: &Page<'_>, element: &WebElement) -> Result<(), Report> {
    page.driver
        .action_chain()
        .move_to_element_center(element)
        .perform()
        .await?;
    Ok(())
}

/// "pauses timers when hovering", with upstream's timing: the timeout continues with the time that
/// was left, it doesn't start over.
async fn remaining_time_after_pause(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/toast").await?;
    page.click_element_with_id("test-toast-add-slow").await?;
    page.wait_for_selector("[role=alertdialog]").await?;
    tokio::time::sleep(Duration::from_millis(1000)).await;
    hover(page, &page.css("[role=alertdialog]").await?).await?;
    tokio::time::sleep(Duration::from_millis(2500)).await;
    assert_that!(page.count_matching("[role=alertdialog]").await?).is_equal_to(1);
    hover(page, &page.element("test-toast-closed").await?).await?;
    let left = std::time::Instant::now();
    tokio::time::sleep(Duration::from_millis(1200)).await;
    assert_that!(page.count_matching("[role=alertdialog]").await?).is_equal_to(1);
    page.wait_for_no_selector("[role=alertdialog]").await?;
    // About 2 seconds were left; a restarted timeout would take 3.
    assert_that!(left.elapsed() < Duration::from_millis(2700)).is_true();
    Ok(())
}

/// react-aria's `useToast.test.js` "moves focus to the next toast when it appears", and "should
/// support custom aria-label": one toast at a time (`use_toast_state`'s default) in a region named
/// "Alerts".
async fn one_at_a_time(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/toast-single").await?;
    page.click_element_with_id("test-toast-single-add").await?;
    page.click_element_with_id("test-toast-single-add").await?;
    page.wait_for_selector("[role=alertdialog]").await?;
    let region = page.css("[role=region]").await?;
    assert_that!(region.attr("aria-label").await?).is_equal_to(Some("Alerts".to_owned()));
    let shown = page.css("[role=alertdialog]").await?;
    assert_that!(page.count_matching("[role=alertdialog]").await?).is_equal_to(1);
    assert_that!(referenced_text(page, &shown, "aria-labelledby").await?)
        .is_equal_to("Alert 2".to_owned());

    // Closing it by keyboard shows the next one and focuses it.
    page.send_keys_to_active(Key::F6).await?;
    page.wait_for_focus("region", None).await?;
    page.press_tab().await?;
    page.press_tab().await?;
    page.send_keys_to_active(Key::Enter).await?;
    wait_for_toast_title(page, "Alert 1").await?;
    let next = page.css("[role=alertdialog]").await?;
    page.wait_for_focus_on(&next, "the next toast").await?;
    page.press_tab().await?;
    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_no_selector("[role=alertdialog]").await?;
    page.wait_for_active_id("test-toast-single-add").await
}

/// Waits until the (first) toast is named `title`.
async fn wait_for_toast_title(page: &Page<'_>, title: &str) -> Result<(), Report> {
    for _ in 0..100 {
        if let Ok(toast) = page.css("[role=alertdialog]").await
            && let Ok(text) = referenced_text(page, &toast, "aria-labelledby").await
            && text == title
        {
            return Ok(());
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    rootcause::bail!("no toast named {title}")
}

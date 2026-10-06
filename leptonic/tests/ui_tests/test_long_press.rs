// Upstream: react-aria/test/interactions/useLongPress.test.js @ 99e6102368
use std::{borrow::Cow, time::Duration};

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{Key, WebDriver},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// Long presses through `use_press`: start, end and the long press after the threshold, which
/// cancels the press; cancelled when released early; a custom threshold; the accessibility
/// description; no context menu on touch; nothing for the keyboard.
pub struct LongPressTests {}

#[async_trait]
impl BrowserTest<str> for LongPressTests {
    fn name(&self) -> Cow<'_, str> {
        "long_press_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/hooks/long-press").await?;

        // "should perform a long press".
        fire(driver, "basic", "pointerdown").await?;
        tokio::time::sleep(Duration::from_millis(300)).await;
        expect_log(&page, "basic:longpressstart:touch").await?;
        page.wait_for_text(
            "test-long-press-log",
            "basic:longpressstart:touch,basic:longpressend:touch,basic:longpress:touch",
        )
        .await?;
        fire(driver, "basic", "pointerup").await?;
        expect_log(
            &page,
            "basic:longpressstart:touch,basic:longpressend:touch,basic:longpress:touch",
        )
        .await?;
        reset(&page).await?;

        // "should cancel if pointer ends before timeout".
        fire(driver, "basic", "pointerdown").await?;
        fire(driver, "basic", "pointerup").await?;
        tokio::time::sleep(Duration::from_millis(700)).await;
        expect_log(&page, "basic:longpressstart:touch,basic:longpressend:touch").await?;
        reset(&page).await?;

        // "should cancel other press events".
        fire(driver, "with-press", "pointerdown").await?;
        page.wait_for_text(
            "test-long-press-log",
            "with-press:longpressstart:touch,with-press:pressstart:touch,\
             with-press:longpressend:touch,with-press:pressend:touch,with-press:longpress:touch",
        )
        .await?;
        fire(driver, "with-press", "pointerup").await?;
        reset(&page).await?;

        // "should not cancel press events if pointer ends before timer".
        fire(driver, "with-press", "pointerdown").await?;
        fire(driver, "with-press", "pointerup").await?;
        page.wait_for_text(
            "test-long-press-log",
            "with-press:longpressstart:touch,with-press:pressstart:touch,\
             with-press:longpressend:touch,with-press:pressend:touch,with-press:press:touch",
        )
        .await?;
        reset(&page).await?;

        // "allows changing the threshold".
        fire(driver, "threshold", "pointerdown").await?;
        tokio::time::sleep(Duration::from_millis(600)).await;
        expect_log(&page, "threshold:longpressstart:touch").await?;
        page.wait_for_text(
            "test-long-press-log",
            "threshold:longpressstart:touch,threshold:longpressend:touch,\
             threshold:longpress:touch",
        )
        .await?;
        fire(driver, "threshold", "pointerup").await?;
        reset(&page).await?;

        // "supports accessibilityDescription", "does not show accessibilityDescription if
        // disabled", "... if no onLongPress handler".
        let description = driver
            .execute(
                "const id = document.getElementById('test-long-press-description')
                     .getAttribute('aria-describedby');
                 return id ? document.getElementById(id)?.textContent ?? null : null;",
                vec![],
            )
            .await?;
        assert_that!(description.json().as_str()).is_equal_to(Some("Long press to open a menu"));
        for name in ["description-disabled", "description-no-handler"] {
            let el = page.element(&format!("test-long-press-{name}")).await?;
            assert_that!(el.attr("aria-describedby").await?).is_none();
        }

        // "prevents context menu events on touch".
        fire(driver, "basic", "pointerdown").await?;
        let prevented = driver
            .execute(
                "const e = new MouseEvent('contextmenu', { bubbles: true, cancelable: true });
                 document.getElementById('test-long-press-basic').dispatchEvent(e);
                 return e.defaultPrevented;",
                vec![],
            )
            .await?;
        assert_that!(prevented.json().as_bool()).is_equal_to(Some(true));
        fire(driver, "basic", "pointerup").await?;
        page.wait_for_text(
            "test-long-press-log",
            "basic:longpressstart:touch,basic:longpressend:touch",
        )
        .await?;
        reset(&page).await?;

        // "should not fire any events for keyboard interactions" (long press events, that is).
        page.element("test-long-press-with-press")
            .await?
            .focus()
            .await?;
        page.send_keys_to_active(Key::Enter).await?;
        page.wait_for_text(
            "test-long-press-log",
            "with-press:pressstart:keyboard,with-press:pressend:keyboard,with-press:press:keyboard",
        )
        .await?;
        tokio::time::sleep(Duration::from_millis(600)).await;
        expect_log(
            &page,
            "with-press:pressstart:keyboard,with-press:pressend:keyboard,with-press:press:keyboard",
        )
        .await?;

        page.expect_no_page_errors().await
    }
}

/// Dispatches a synthetic touch pointer event on `#test-long-press-<name>`.
async fn fire(driver: &WebDriver, name: &str, kind: &str) -> Result<(), Report> {
    let buttons = i32::from(kind == "pointerdown");
    driver
        .execute(
            &format!(
                "document.getElementById('test-long-press-{name}').dispatchEvent(new PointerEvent(\
                 '{kind}', {{ bubbles: true, cancelable: true, composed: true, \
                 pointerType: 'touch', pointerId: 1, isPrimary: true, button: 0, \
                 buttons: {buttons}, width: 1, height: 1 }}));"
            ),
            vec![],
        )
        .await?;
    Ok(())
}

async fn expect_log(page: &Page<'_>, expected: &str) -> Result<(), Report> {
    tokio::time::sleep(Duration::from_millis(50)).await;
    assert_that!(page.element("test-long-press-log").await?.text().await?)
        .is_equal_to(expected.to_owned());
    Ok(())
}

async fn reset(page: &Page<'_>) -> Result<(), Report> {
    page.driver
        .execute(
            "document.getElementById('test-long-press-reset').click();",
            vec![],
        )
        .await?;
    page.wait_for_text("test-long-press-log", "").await
}

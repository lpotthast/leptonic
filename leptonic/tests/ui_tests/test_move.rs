// Upstream: react-aria/test/interactions/useMove.test.js @ 99e6102368
use std::{borrow::Cow, time::Duration};

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{Key, WebDriver},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// `use_move`: pointer movement (start on the first move, deltas, end on pointer up or cancel),
/// nothing for right clicks, taps or further pointers, no bubbling to a movable parent, arrow
/// keys, other keys passed on.
pub struct MoveTests {}

#[async_trait]
impl BrowserTest<str> for MoveTests {
    fn name(&self) -> Cow<'_, str> {
        "move_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/hooks/move").await?;

        // "responds to pointer events".
        fire(driver, "single", "pointerdown", 1, 1, 30, 0).await?;
        expect_log(&page, "").await?;
        fire(driver, "single", "pointermove", 1, 10, 25, 0).await?;
        page.wait_for_text("test-move-log", "single:start:pen,single:move:pen:9:-5")
            .await?;
        fire(driver, "single", "pointerup", 1, 10, 25, 0).await?;
        page.wait_for_text(
            "test-move-log",
            "single:start:pen,single:move:pen:9:-5,single:end:pen",
        )
        .await?;
        reset(&page).await?;

        // "ends with pointercancel".
        fire(driver, "single", "pointerdown", 1, 1, 30, 0).await?;
        fire(driver, "single", "pointermove", 1, 10, 25, 0).await?;
        fire(driver, "single", "pointercancel", 1, 10, 25, 0).await?;
        page.wait_for_text(
            "test-move-log",
            "single:start:pen,single:move:pen:9:-5,single:end:pen",
        )
        .await?;
        reset(&page).await?;

        // "doesn't respond to right click", "doesn't fire anything when tapping".
        fire(driver, "single", "pointerdown", 1, 1, 30, 2).await?;
        fire(driver, "single", "pointermove", 1, 10, 25, 2).await?;
        fire(driver, "single", "pointerup", 1, 10, 25, 2).await?;
        fire(driver, "single", "pointerdown", 1, 1, 30, 0).await?;
        fire(driver, "single", "pointerup", 1, 1, 30, 0).await?;
        expect_log(&page, "").await?;

        // "ignores any additional pointers".
        fire(driver, "single", "pointerdown", 1, 1, 30, 0).await?;
        fire(driver, "single", "pointerdown", 3, 1, 30, 0).await?;
        fire(driver, "single", "pointermove", 3, 1, 40, 0).await?;
        fire(driver, "single", "pointerup", 3, 1, 40, 0).await?;
        expect_log(&page, "").await?;
        fire(driver, "single", "pointermove", 1, 10, 25, 0).await?;
        fire(driver, "single", "pointerup", 1, 10, 25, 0).await?;
        page.wait_for_text(
            "test-move-log",
            "single:start:pen,single:move:pen:9:-5,single:end:pen",
        )
        .await?;
        reset(&page).await?;

        // "doesn't bubble to useMove on parent elements".
        fire(driver, "child", "pointerdown", 1, 1, 30, 0).await?;
        fire(driver, "child", "pointermove", 1, 10, 25, 0).await?;
        fire(driver, "child", "pointerup", 1, 10, 25, 0).await?;
        page.wait_for_text(
            "test-move-log",
            "child:start:pen,child:move:pen:9:-5,child:end:pen",
        )
        .await?;
        reset(&page).await?;

        // "responds to keypresses", "allows handling other key events".
        page.element("test-move-single").await?.focus().await?;
        for key in [Key::Up, Key::Down, Key::Left, Key::Right] {
            page.send_keys_to_active(key).await?;
        }
        page.send_keys_to_active(Key::PageUp).await?;
        page.wait_for_text(
            "test-move-log",
            "single:start:keyboard,single:move:keyboard:0:-1,single:end:keyboard,\
             single:start:keyboard,single:move:keyboard:0:1,single:end:keyboard,\
             single:start:keyboard,single:move:keyboard:-1:0,single:end:keyboard,\
             single:start:keyboard,single:move:keyboard:1:0,single:end:keyboard,\
             keydown:PageUp",
        )
        .await?;

        page.expect_no_page_errors().await
    }
}

/// Dispatches a synthetic pen pointer event on `#test-move-<name>` (it bubbles to the document,
/// where the hook listens while moving).
#[allow(clippy::too_many_arguments)]
async fn fire(
    driver: &WebDriver,
    name: &str,
    kind: &str,
    pointer_id: i32,
    x: i32,
    y: i32,
    button: i32,
) -> Result<(), Report> {
    let buttons = if kind == "pointerup" || kind == "pointercancel" {
        0
    } else {
        1 << button.min(1)
    };
    driver
        .execute(
            &format!(
                "document.getElementById('test-move-{name}').dispatchEvent(new PointerEvent('{kind}', {{
                     bubbles: true, cancelable: true, composed: true, pointerType: 'pen',
                     pointerId: {pointer_id}, isPrimary: {pointer_id} === 1, clientX: {x},
                     clientY: {y}, button: {button}, buttons: {buttons}
                 }}));"
            ),
            vec![],
        )
        .await?;
    Ok(())
}

/// The log stays as it is (waiting briefly for wrong events).
async fn expect_log(page: &Page<'_>, expected: &str) -> Result<(), Report> {
    tokio::time::sleep(Duration::from_millis(50)).await;
    assert_that!(page.element("test-move-log").await?.text().await?)
        .is_equal_to(expected.to_owned());
    Ok(())
}

async fn reset(page: &Page<'_>) -> Result<(), Report> {
    page.driver
        .execute(
            "document.getElementById('test-move-reset').click();",
            vec![],
        )
        .await?;
    page.wait_for_text("test-move-log", "").await
}

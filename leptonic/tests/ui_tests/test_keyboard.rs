// Upstream: react-aria/test/interactions/useKeyboard.test.js @ 99e6102368
use std::{borrow::Cow, time::Duration};

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{Key, TypingData, WebDriver},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// `use_keyboard`: handlers, disabled, propagation (stopped by default, continued on request or
/// by unhandled shortcuts), shortcuts ignoring repeats/composing/keyup unless allowed, and two
/// hooks on one element stopping propagation if any of them does.
pub struct KeyboardTests {}

#[async_trait]
impl BrowserTest<str> for KeyboardTests {
    fn name(&self) -> Cow<'_, str> {
        "keyboard_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/hooks/keyboard").await?;

        let cases: [(&str, TypingData, &str); 11] = [
            // "should handle keyboard events", "events do not bubble by default".
            ("basic", "a".into(), "basic:keydown:a,basic:keyup:a"),
            // "should not handle events when disabled".
            (
                "disabled",
                "a".into(),
                "disabled-wrapper:keydown:a,disabled-wrapper:keyup:a",
            ),
            // "events bubble when continuePropagation is called".
            (
                "continue",
                "a".into(),
                "continue:keydown:a,continue-wrapper:keydown:a,continue:keyup:a,\
                 continue-wrapper:keyup:a",
            ),
            // "chains a user-provided onKeyDown and onKeyUp with shortcuts".
            (
                "shortcut",
                "a".into(),
                "shortcut:keydown:a,shortcut:action,shortcut:keyup:a",
            ),
            // "passes event to handler", "continues propagation if the function did not handle
            // the event".
            (
                "ignored",
                Key::Escape.into(),
                "ignored:action,ignored-wrapper:keydown:Escape,ignored-wrapper:keyup:Escape",
            ),
            // "prevent default and stop propagation can both be finely controlled".
            (
                "custom",
                Key::Escape.into(),
                "custom:action,custom-wrapper:keydown:Escape,custom-wrapper:keyup:Escape",
            ),
            // "should stop propagation if any shortcut handling that key stops propagation".
            (
                "stop-other-key",
                Key::Left.into(),
                "stop-other-key:action,stop-other-key-wrapper:keyup:ArrowLeft",
            ),
            // "should continue propagation if all shortcuts that handle that key agree to
            // continue propagation".
            (
                "continue-other-key",
                Key::Left.into(),
                "continue-other-key:action,continue-other-key-wrapper:keydown:ArrowLeft:prevented,\
                 continue-other-key-wrapper:keyup:ArrowLeft",
            ),
            // "should stop propagation if any shortcut stops propagation".
            (
                "stop-any",
                Key::Left.into(),
                "stop-any:action,stop-any:action,stop-any-wrapper:keyup:ArrowLeft",
            ),
            // "should continue propagation if all shortcuts agree to continue propagation".
            (
                "continue-all",
                Key::Left.into(),
                "continue-all:action,continue-all:action,\
                 continue-all-wrapper:keydown:ArrowLeft:prevented,continue-all-wrapper:keyup:ArrowLeft",
            ),
            // A key no shortcut handles bubbles.
            (
                "repeats",
                "b".into(),
                "repeats-wrapper:keydown:b,repeats-wrapper:keyup:b",
            ),
        ];
        for (name, keys, expected) in cases {
            page.element(&format!("test-keyboard-{name}"))
                .await?
                .focus()
                .await?;
            page.send_keys_to_active(keys).await?;
            page.wait_for_text("test-keyboard-log", expected).await?;
            reset(&page).await?;
        }

        // "ignores repeated keydown events by default", "handles repeated keydown events when
        // allowRepeats is true", the same for composing, "does not run shortcuts on keyup".
        for (name, init, expected) in [
            ("shortcut", "repeat: true", "shortcut:keydown:a"),
            ("repeats", "repeat: true", "repeats:action"),
            ("shortcut", "isComposing: true", "shortcut:keydown:a"),
            ("composing", "isComposing: true", "composing:action"),
        ] {
            dispatch(driver, name, "keydown", init).await?;
            page.wait_for_text("test-keyboard-log", expected).await?;
            reset(&page).await?;
        }
        for init in ["repeat: false", "repeat: true", "isComposing: true"] {
            dispatch(driver, "repeats", "keyup", init).await?;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
        assert_that!(page.element("test-keyboard-log").await?.text().await?)
            .does_not_contain("action");

        page.expect_no_page_errors().await
    }
}

/// Dispatches a synthetic `a` key event on `#test-keyboard-<name>`.
async fn dispatch(driver: &WebDriver, name: &str, kind: &str, init: &str) -> Result<(), Report> {
    driver
        .execute(
            &format!(
                "document.getElementById('test-keyboard-{name}').dispatchEvent(new KeyboardEvent(\
                 '{kind}', {{ key: 'a', bubbles: true, cancelable: true, {init} }}));"
            ),
            vec![],
        )
        .await?;
    Ok(())
}

async fn reset(page: &Page<'_>) -> Result<(), Report> {
    page.driver
        .execute(
            "document.getElementById('test-keyboard-reset').click();",
            vec![],
        )
        .await?;
    page.wait_for_text("test-keyboard-log", "").await
}

// Upstream: react-aria/test/interactions/Pressable.test.js @ 99e6102368
// Upstream: react-aria/test/interactions/PressResponder.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{Key, WebDriver},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// The `Pressable` atom: its press handling goes onto the child itself (no wrapper), merged with
/// the child's own handlers; the child becomes focusable unless disabled; a surrounding
/// `PressResponder` applies to it.
pub struct PressableTests {}

#[async_trait]
impl BrowserTest<str> for PressableTests {
    fn name(&self) -> Cow<'_, str> {
        "pressable_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/pressable").await?;

        // "should should merge with existing props, not overwrite".
        page.element("test-pressable-button").await?.click().await?;
        page.wait_for_text("test-pressable-log", "button click, button press")
            .await?;

        // "should automatically make child focusable"; the element itself is pressable (also
        // by keyboard).
        let span = page.element("test-pressable-span").await?;
        page.wait_for_attr(&span, "tabindex", Some("0")).await?;
        let parent_tag = page
            .driver
            .execute(
                "return document.getElementById('test-pressable-span').parentElement.dataset.pressable ?? null;",
                vec![],
            )
            .await?;
        assert_that!(parent_tag.json().is_null()).is_true();
        span.focus().await?;
        page.send_keys_to_active(Key::Enter).await?;
        page.wait_for_text(
            "test-pressable-log",
            "button click, button press, span press",
        )
        .await?;

        // "supports isDisabled": not focusable, no press.
        let disabled = page.element("test-pressable-disabled").await?;
        assert_that!(disabled.attr("tabindex").await?).is_none();
        disabled.click().await?;

        // Inside a `PressResponder`: the responder's handler runs first.
        page.element("test-pressable-responder")
            .await?
            .click()
            .await?;
        page.wait_for_text(
            "test-pressable-log",
            "button click, button press, span press, responder press, inner press",
        )
        .await?;

        // A `PressResponder` warns once without a pressable child, not with one.
        driver
            .execute(
                "window.__warnings = [];
                 const warn = console.warn;
                 console.warn = (...args) => { window.__warnings.push(args.join(' ')); warn(...args); };",
                vec![],
            )
            .await?;
        page.element("test-pressable-mount").await?.click().await?;
        page.element("test-responder-pressable").await?;
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        let warnings = driver
            .execute(
                "return window.__warnings.filter(w => w.includes('PressResponder')).length;",
                vec![],
            )
            .await?;
        assert_that!(warnings.json().as_i64()).is_equal_to(Some(1));

        page.expect_no_page_errors().await
    }
}

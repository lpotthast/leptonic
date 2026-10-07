// Upstream: react-aria-components/test/Toolbar.test.tsx @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{Key, WebDriver},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// The toolbar atom ("supports keyboard navigation"): one tab stop, arrow keys along its
/// orientation across nested toolbars and dividers without wrapping, Tab leaving and re-entering
/// at the control focused last; nested toolbars are groups; vertical and right-to-left toolbars;
/// toolbars of toggle buttons, checkboxes and links.
pub struct ToolbarTests {}

#[async_trait]
impl BrowserTest<str> for ToolbarTests {
    fn name(&self) -> Cow<'_, str> {
        "toolbar_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/toolbar").await?;

        let tools = page.css("[aria-label=Tools]").await?;
        assert_that!(tools.attr("role").await?).is_equal_to(Some("toolbar".to_owned()));
        assert_that!(tools.attr("aria-orientation").await?)
            .is_equal_to(Some("horizontal".to_owned()));
        page.wait_for_selector("[aria-label='Align text'][role=group]")
            .await?;

        page.by_role_and_text("button", "Before")
            .await?
            .click()
            .await?;
        page.press_tab().await?;
        page.wait_for_active_text("Align left").await?;
        page.send_keys_to_active(Key::Right).await?;
        page.wait_for_active_text("Align center").await?;
        // Down does nothing in a horizontal toolbar.
        page.send_keys_to_active(Key::Down).await?;
        page.wait_for_active_text("Align center").await?;
        page.send_keys_to_active(Key::Right).await?;
        // Across the divider into the next group.
        page.send_keys_to_active(Key::Right).await?;
        page.wait_for_active_text("Zoom in").await?;
        page.send_keys_to_active(Key::Left).await?;
        page.wait_for_active_text("Align right").await?;

        // Tab leaves; Shift+Tab re-enters at the control focused last.
        page.press_tab().await?;
        page.wait_for_active_text("After").await?;
        page.press_shift_tab().await?;
        page.wait_for_active_text("Align right").await?;
        page.press_shift_tab().await?;
        page.wait_for_active_text("Before").await?;
        page.press_tab().await?;
        page.wait_for_active_text("Align right").await?;

        // No wrapping.
        page.send_keys_to_active(Key::Left).await?;
        page.send_keys_to_active(Key::Left).await?;
        page.wait_for_active_text("Align left").await?;
        page.send_keys_to_active(Key::Left).await?;
        page.wait_for_active_text("Align left").await?;
        page.by_role_and_text("button", "Zoom out")
            .await?
            .click()
            .await?;
        page.send_keys_to_active(Key::Right).await?;
        page.wait_for_active_text("Zoom out").await?;

        // "supports keyboard navigation with orientation vertical".
        let vertical = page.css("[aria-label=Vertical]").await?;
        assert_that!(vertical.attr("aria-orientation").await?)
            .is_equal_to(Some("vertical".to_owned()));
        page.by_role_and_text("button", "Up 1")
            .await?
            .click()
            .await?;
        page.send_keys_to_active(Key::Down).await?;
        page.wait_for_active_text("Up 2").await?;
        page.send_keys_to_active(Key::Right).await?;
        page.wait_for_active_text("Up 2").await?;
        page.send_keys_to_active(Key::Up).await?;
        page.wait_for_active_text("Up 1").await?;

        // "supports RTL": the arrow keys follow the reading direction.
        page.by_role_and_text("button", "RTL 1")
            .await?
            .click()
            .await?;
        page.send_keys_to_active(Key::Left).await?;
        page.wait_for_active_text("RTL 2").await?;
        page.send_keys_to_active(Key::Right).await?;
        page.wait_for_active_text("RTL 1").await?;

        // "supports RTL with orientation vertical": up and down move; left and right don't.
        page.by_role_and_text("button", "RV 1")
            .await?
            .click()
            .await?;
        page.send_keys_to_active(Key::Down).await?;
        page.wait_for_active_text("RV 2").await?;
        page.send_keys_to_active(Key::Up).await?;
        page.wait_for_active_text("RV 1").await?;
        page.send_keys_to_active(Key::Left).await?;
        page.send_keys_to_active(Key::Right).await?;
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        page.wait_for_active_text("RV 1").await?;

        // "supports all the aria example children": toggle buttons, a checkbox and a link, without
        // wrapping at the end.
        page.element("test-toolbar-input-before")
            .await?
            .click()
            .await?;
        page.press_tab().await?;
        page.wait_for_active_text("B").await?;
        page.send_keys_to_active(Key::Right).await?;
        page.wait_for_active_text("U").await?;
        page.send_keys_to_active(Key::Right).await?;
        page.wait_for_active_text("I").await?;
        page.send_keys_to_active(Key::Right).await?;
        let checkbox_focused = || async {
            let focused: bool = page
                .driver
                .execute(
                    "const el = document.activeElement;
                     return el.type === 'checkbox' && el.closest('label').innerText.includes('Night Mode');",
                    vec![],
                )
                .await?
                .convert()?;
            Ok::<bool, Report>(focused)
        };
        for _ in 0..100 {
            if checkbox_focused().await? {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
        assert_that!(checkbox_focused().await?)
            .with_detail_message("the Night Mode checkbox is focused")
            .is_true();
        page.send_keys_to_active(Key::Right).await?;
        page.wait_for_active_text("Help").await?;
        page.send_keys_to_active(Key::Right).await?;
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        page.wait_for_active_text("Help").await?;

        page.expect_no_page_errors().await
    }
}

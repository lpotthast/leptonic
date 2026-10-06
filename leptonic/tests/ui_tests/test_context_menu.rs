// Upstream: react-aria/test/interactions/useContextMenu.test.tsx @ 99e6102368
use std::{borrow::Cow, time::Duration};

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{Key, WebDriver},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// `use_context_menu` on a non-Apple platform: a right click requests the menu at its position
/// relative to the element, prevents the browser's menu and stops propagation; without a handler
/// nothing happens; Ctrl+Enter is macOS-only. (The macOS and iOS paths need those platforms.)
pub struct ContextMenuTests {}

#[async_trait]
impl BrowserTest<str> for ContextMenuTests {
    fn name(&self) -> Cow<'_, str> {
        "context_menu_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/hooks/context-menu").await?;

        // "calls onContextMenu on right click", "prevents default and stops propagation on
        // contextmenu event".
        let (prevented, x, y) = context_menu(driver, "handler").await?;
        assert_that!(prevented).is_true();
        page.wait_for_text(
            "test-context-menu-log",
            &format!("menu:{x}:{y}:test-context-menu-handler"),
        )
        .await?;
        reset(&page).await?;

        // "does not call onContextMenu when prop is not provided".
        let (prevented, ..) = context_menu(driver, "none").await?;
        assert_that!(prevented).is_false();
        page.wait_for_text("test-context-menu-log", "none-wrapper")
            .await?;
        reset(&page).await?;

        // "does not trigger on Ctrl+Enter on non-macOS".
        page.element("test-context-menu-handler")
            .await?
            .focus()
            .await?;
        page.send_keys_to_active(Key::Control + Key::Enter).await?;
        tokio::time::sleep(Duration::from_millis(100)).await;
        assert_that!(page.element("test-context-menu-log").await?.text().await?)
            .is_equal_to(String::new());

        page.expect_no_page_errors().await
    }
}

/// Dispatches a `contextmenu` event about 20px right of and below the element's corner. Returns
/// whether its default was prevented, and the position relative to the element (the event's
/// coordinates are whole pixels, the element's need not be).
async fn context_menu(driver: &WebDriver, name: &str) -> Result<(bool, f64, f64), Report> {
    let result = driver
        .execute(
            &format!(
                "const el = document.getElementById('test-context-menu-{name}');
                 const rect = el.getBoundingClientRect();
                 const e = new MouseEvent('contextmenu', {{ bubbles: true, cancelable: true,
                     clientX: rect.x + 20, clientY: rect.y + 20, button: 2 }});
                 el.dispatchEvent(e);
                 return [e.defaultPrevented, e.clientX - rect.x, e.clientY - rect.y];"
            ),
            vec![],
        )
        .await?;
    let values = result.json();
    Ok((
        values[0].as_bool().unwrap_or_default(),
        values[1].as_f64().unwrap_or_default(),
        values[2].as_f64().unwrap_or_default(),
    ))
}

async fn reset(page: &Page<'_>) -> Result<(), Report> {
    page.driver
        .execute(
            "document.getElementById('test-context-menu-reset').click();",
            vec![],
        )
        .await?;
    page.wait_for_text("test-context-menu-log", "").await
}

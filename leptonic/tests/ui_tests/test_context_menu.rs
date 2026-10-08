// Upstream: react-aria/test/interactions/useContextMenu.test.tsx @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{Dispatched, ElementActions, Page, PageActions, SyntheticEvent};

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
        cases!(
            right_click_requests_the_menu(&page),
            without_a_handler_nothing_happens(&page),
            ctrl_enter_is_mac_only(&page),
        );
        Ok(())
    }
}

/// The fixture's log of handled context menus.
async fn log(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-context-menu-log").await
}

/// Empties the log (a virtual click, so that the reset itself causes no pointer events).
async fn reset(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-context-menu-reset")
        .await?
        .virtual_click()
        .await?;
    log(page).await?.wait_for_inner_text("").await?;
    Ok(())
}

/// A `contextmenu` event about 20px right of and below the corner of `element`, and the position
/// relative to the element the handler gets (the event's coordinates are whole pixels, the
/// element's need not be).
async fn right_click(element: &WebElement) -> Result<(Dispatched, f64, f64), Report> {
    let rect = element.client_rect().await?;
    let (x, y) = ((rect.left + 20.0).trunc(), (rect.top + 20.0).trunc());
    let dispatched = element
        .dispatch(
            SyntheticEvent::mouse("contextmenu")
                .with("clientX", x)
                .with("clientY", y)
                .with("button", 2),
        )
        .await?;
    Ok((dispatched, x - rect.left, y - rect.top))
}

/// "calls onContextMenu on right click", "prevents default and stops propagation on contextmenu
/// event".
async fn right_click_requests_the_menu(page: &Page<'_>) -> Result<(), Report> {
    let element = page.element("#test-context-menu-handler").await?;
    let (dispatched, x, y) = right_click(&element).await?;
    assert_that!(dispatched.default_prevented).is_true();
    log(page)
        .await?
        .wait_for_inner_text(&format!("menu:{x}:{y}:test-context-menu-handler"))
        .await?;
    reset(page).await?;
    Ok(())
}

/// "does not call onContextMenu when prop is not provided": the event reaches the wrapper, the
/// browser's menu isn't prevented.
async fn without_a_handler_nothing_happens(page: &Page<'_>) -> Result<(), Report> {
    let element = page.element("#test-context-menu-none").await?;
    let (dispatched, ..) = right_click(&element).await?;
    assert_that!(dispatched.default_prevented).is_false();
    log(page).await?.wait_for_inner_text("none-wrapper").await?;
    reset(page).await?;
    Ok(())
}

/// "does not trigger on Ctrl+Enter on non-macOS".
async fn ctrl_enter_is_mac_only(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-context-menu-handler")
        .await?
        .focus()
        .await?;
    page.send_keys(Key::Control + Key::Enter).await?;
    log(page).await?.inner_text_stays("").await?;
    Ok(())
}

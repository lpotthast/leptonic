// Upstream: react-aria/test/menu/useMenuTrigger.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions};

/// `useMenuTrigger`: the trigger's ARIA attributes, opening on mouse down (once), and the focus
/// strategy of keyboard opening.
pub struct MenuTriggerTests {}

#[async_trait]
impl BrowserTest<str> for MenuTriggerTests {
    fn name(&self) -> Cow<'_, str> {
        "menu_trigger_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/hooks/menu-trigger").await?;
        cases!(
            aria_attributes(&page),
            mouse_press_opens_once(&page),
            keyboard_opens_with_focus_strategy(&page),
        );
        Ok(())
    }
}

const TRIGGER: &str = "[data-testid=trigger]";

/// Closes the menu with the fixture's close button.
async fn close(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-mt-close").await?.click().await?;
    page.element("#test-mt-is-open")
        .await?
        .wait_for_inner_text("false")
        .await?;
    Ok(())
}

/// The trigger announces a collapsed menu and has the id that labels the menu.
async fn aria_attributes(page: &Page<'_>) -> Result<(), Report> {
    let trigger = page.element(TRIGGER).await?;
    assert_that!(trigger.attr("aria-haspopup").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(trigger.attr("aria-expanded").await?)
        .get_some()
        .is_equal_to("false");
    assert_that!(trigger.id().await?)
        .get_some()
        .starts_with("menu-trigger-");
    Ok(())
}

/// The trigger opens on mouse down. With a single press state machine on the element, the
/// following click does not toggle it closed again. Mouse users get the menu focused, not its
/// first item.
async fn mouse_press_opens_once(page: &Page<'_>) -> Result<(), Report> {
    let trigger = page.element(TRIGGER).await?;
    let is_open = page.element("#test-mt-is-open").await?;
    trigger.click().await?;
    is_open.wait_for_inner_text("true").await?;
    is_open.inner_text_stays("true").await?;
    assert_that!(trigger.attr("aria-expanded").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(
        page.element("#test-mt-strategy")
            .await?
            .inner_text()
            .await?
    )
    .is_equal_to("none");
    close(page).await?;
    Ok(())
}

/// ArrowDown and Enter open with the first item focused, ArrowUp with the last; Space opens.
async fn keyboard_opens_with_focus_strategy(page: &Page<'_>) -> Result<(), Report> {
    let trigger = page.element(TRIGGER).await?;
    let before = page.element("#test-mt-before").await?;
    let is_open = page.element("#test-mt-is-open").await?;
    let strategy = page.element("#test-mt-strategy").await?;
    for (key, expected) in [
        (Key::Down, "first"),
        (Key::Up, "last"),
        (Key::Enter, "first"),
    ] {
        before.click().await?;
        page.send_keys(Key::Tab).await?;
        page.wait_for_focus(&trigger).await?;
        page.send_keys(key).await?;
        is_open.wait_for_inner_text("true").await?;
        assert_that!(strategy.inner_text().await?).is_equal_to(expected);
        close(page).await?;
    }

    before.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&trigger).await?;
    page.send_keys(" ").await?;
    is_open.wait_for_inner_text("true").await?;
    close(page).await?;
    Ok(())
}

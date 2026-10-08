// Upstream: react-aria/test/menu/useMenuTrigger.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

pub struct MenuTriggerTests {}

#[async_trait]
impl BrowserTest<str> for MenuTriggerTests {
    fn name(&self) -> Cow<'_, str> {
        "menu_trigger_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/hooks/menu-trigger").await?;

        aria_attributes(&page).await?;
        mouse_press_opens_once(&page).await?;
        keyboard_opens_with_focus_strategy(&page).await?;

        Ok(())
    }
}

async fn trigger_attr(page: &Page<'_>, name: &str) -> Result<Option<String>, Report> {
    Ok(page
        .driver
        .find(By::Css("[data-testid=trigger]"))
        .await?
        .attr(name)
        .await?)
}

async fn close(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-mt-close").await?;
    page.wait_for_text("test-mt-is-open", "false").await
}

async fn aria_attributes(page: &Page<'_>) -> Result<(), Report> {
    assert_that!(trigger_attr(page, "aria-haspopup").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(trigger_attr(page, "aria-expanded").await?).is_equal_to(Some("false".to_owned()));
    // The trigger labels the menu.
    let trigger_id = trigger_attr(page, "id").await?;
    assert_that!(trigger_id.as_deref().unwrap_or_default()).starts_with("menu-trigger-");
    Ok(())
}

/// The trigger opens on mouse down. With a single press state machine on the element, the
/// following click does not toggle it closed again.
async fn mouse_press_opens_once(page: &Page<'_>) -> Result<(), Report> {
    page.driver
        .find(By::Css("[data-testid=trigger]"))
        .await?
        .click()
        .await?;
    page.wait_for_text("test-mt-is-open", "true").await?;
    stays!(
        "the text of #test-mt-is-open",
        "true".to_owned(),
        page.read_text_of("test-mt-is-open").await?
    );
    assert_that!(trigger_attr(page, "aria-expanded").await?).is_equal_to(Some("true".to_owned()));
    // Mouse users get the menu focused, not its first item.
    assert_that!(page.read_text_of("test-mt-strategy").await?).is_equal_to("none".to_owned());
    close(page).await
}

async fn keyboard_opens_with_focus_strategy(page: &Page<'_>) -> Result<(), Report> {
    let trigger = page.driver.find(By::Css("[data-testid=trigger]")).await?;
    page.click_element_with_id("test-mt-before").await?;
    page.press_tab().await?;

    for (key, strategy) in [
        (Key::Down, "first"),
        (Key::Up, "last"),
        (Key::Enter, "first"),
    ] {
        trigger.send_keys(key).await?;
        page.wait_for_text("test-mt-is-open", "true").await?;
        assert_that!(page.read_text_of("test-mt-strategy").await?).is_equal_to(strategy.to_owned());
        close(page).await?;
        page.click_element_with_id("test-mt-before").await?;
        page.press_tab().await?;
    }

    trigger.send_keys(" ").await?;
    page.wait_for_text("test-mt-is-open", "true").await?;
    close(page).await
}

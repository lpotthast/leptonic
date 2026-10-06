// Upstream: react-aria-components/test/Tabs.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// Behavior of the tabs hooks (through the `Tabs` atoms): ARIA structure (tabs control their
/// panel, the panel is labelled by its tab), selection by press and by arrow keys (automatic and
/// manual activation, wrapping, vertical orientation), and disabled tabs.
/// Spec: react-aria-components `Tabs.test.js`.
pub struct TabsTests {}

#[async_trait]
impl BrowserTest<str> for TabsTests {
    fn name(&self) -> Cow<'_, str> {
        "tabs_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/tabs").await?;

        aria_structure(&page).await?;
        selection_by_press(&page).await?;
        keyboard_navigation(&page).await?;
        disabled_tab(&page).await?;
        disabled_first_tab(&page).await?;
        all_tabs_disabled(&page).await?;
        vertical(&page).await?;
        manual_activation(&page).await?;

        Ok(())
    }
}

async fn tablist(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    page.css(&format!("[role=tablist][aria-label='{name}']"))
        .await
}

async fn tabs(page: &Page<'_>, name: &str) -> Result<Vec<WebElement>, Report> {
    Ok(tablist(page, name)
        .await?
        .find_all(By::Css("[role=tab]"))
        .await?)
}

/// The panel of the tabs `name` (the tab list's following sibling panel).
async fn panel(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    tablist(page, name)
        .await?
        .find(By::XPath("following-sibling::*[@role='tabpanel']"))
        .await
        .map_err(Into::into)
}

async fn attr(element: &WebElement, name: &str) -> Result<Option<String>, Report> {
    Ok(element.attr(name).await?)
}

async fn enter(page: &Page<'_>, name: &str) -> Result<(), Report> {
    page.click_element_with_id(&format!("test-tabs-{name}-before"))
        .await?;
    page.press_tab().await
}

async fn expect_focus(page: &Page<'_>, name: &str, index: usize) -> Result<(), Report> {
    let tab = tabs(page, name).await?.swap_remove(index);
    page.wait_for_focus_on(&tab, &format!("{name}: tab {index}"))
        .await
}

/// Wait until the tab at `index` is the selected one.
async fn expect_selected(page: &Page<'_>, name: &str, index: usize) -> Result<(), Report> {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    loop {
        let mut selected = Vec::new();
        for tab in tabs(page, name).await? {
            selected.push(attr(&tab, "aria-selected").await?.as_deref() == Some("true"));
        }
        if selected.iter().position(|s| *s) == Some(index)
            && selected.iter().filter(|s| **s).count() == 1
        {
            return Ok(());
        }
        if std::time::Instant::now() > deadline {
            leptos_browser_test::bail!("{name}: expected tab {index} selected, got {selected:?}");
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
}

async fn aria_structure(page: &Page<'_>) -> Result<(), Report> {
    let list = tablist(page, "basic").await?;
    assert_that!(attr(&list, "aria-orientation").await?).is_equal_to(Some("horizontal".to_owned()));
    let tabs = tabs(page, "basic").await?;
    assert_that!(tabs.len()).is_equal_to(3);
    expect_selected(page, "basic", 0).await?;
    // The selected tab controls the panel, which its tab labels.
    let panel = panel(page, "basic").await?;
    assert_that!(panel.text().await?).is_equal_to("Panel A".to_owned());
    let panel_id = attr(&panel, "id").await?;
    assert_that!(attr(&tabs[0], "aria-controls").await?).is_equal_to(panel_id);
    assert_that!(attr(&tabs[1], "aria-controls").await?).is_none();
    let tab_id = attr(&tabs[0], "id").await?;
    assert_that!(attr(&panel, "aria-labelledby").await?).is_equal_to(tab_id);
    // Only the selected tab's panel is rendered.
    let panels = list
        .find_all(By::XPath("following-sibling::*[@role='tabpanel']"))
        .await?;
    assert_that!(panels.len()).is_equal_to(1);
    Ok(())
}

async fn selection_by_press(page: &Page<'_>) -> Result<(), Report> {
    tabs(page, "basic").await?[1].click().await?;
    page.wait_for_text("test-tabs-basic-selection", "b").await?;
    expect_selected(page, "basic", 1).await?;
    let panel = panel(page, "basic").await?;
    assert_that!(panel.text().await?).is_equal_to("Panel B".to_owned());
    let id = attr(&panel, "id").await?.unwrap_or_default();
    assert_that!(id.ends_with("-b")).is_true();
    let tab_id = attr(&tabs(page, "basic").await?[1], "id").await?;
    assert_that!(attr(&panel, "aria-labelledby").await?).is_equal_to(tab_id);

    tabs(page, "basic").await?[0].click().await?;
    page.wait_for_text("test-tabs-basic-selection", "a").await?;
    expect_selected(page, "basic", 0).await
}

/// Arrow keys select the next/previous tab (wrapping); Home/End the first/last.
async fn keyboard_navigation(page: &Page<'_>) -> Result<(), Report> {
    enter(page, "basic").await?;
    expect_focus(page, "basic", 0).await?;
    page.send_keys_to_active(Key::Right).await?;
    expect_focus(page, "basic", 1).await?;
    expect_selected(page, "basic", 1).await?;
    page.send_keys_to_active(Key::Right).await?;
    page.send_keys_to_active(Key::Right).await?;
    expect_focus(page, "basic", 0).await?;
    expect_selected(page, "basic", 0).await?;
    page.send_keys_to_active(Key::Left).await?;
    expect_selected(page, "basic", 2).await?;
    page.send_keys_to_active(Key::Home).await?;
    expect_selected(page, "basic", 0).await?;
    page.send_keys_to_active(Key::End).await?;
    expect_selected(page, "basic", 2).await?;
    // Down/Up don't move in a horizontal tab list.
    page.send_keys_to_active(Key::Down).await?;
    expect_focus(page, "basic", 2).await
}

async fn disabled_tab(page: &Page<'_>) -> Result<(), Report> {
    let tabs = tabs(page, "disabled-tab").await?;
    assert_that!(attr(&tabs[1], "aria-disabled").await?).is_equal_to(Some("true".to_owned()));
    enter(page, "disabled-tab").await?;
    expect_focus(page, "disabled-tab", 0).await?;
    page.send_keys_to_active(Key::Right).await?;
    expect_focus(page, "disabled-tab", 2).await
}

/// Without a default, the first enabled tab is selected.
async fn disabled_first_tab(page: &Page<'_>) -> Result<(), Report> {
    let tabs = tabs(page, "first-disabled").await?;
    assert_that!(attr(&tabs[0], "aria-disabled").await?).is_equal_to(Some("true".to_owned()));
    expect_selected(page, "first-disabled", 1).await?;
    enter(page, "first-disabled").await?;
    expect_focus(page, "first-disabled", 1).await?;
    page.send_keys_to_active(Key::Right).await?;
    expect_focus(page, "first-disabled", 2).await
}

/// With all tabs disabled, the first is selected, and Tab moves past the tabs to the panel.
async fn all_tabs_disabled(page: &Page<'_>) -> Result<(), Report> {
    expect_selected(page, "all-disabled", 0).await?;
    enter(page, "all-disabled").await?;
    let panel = panel(page, "all-disabled").await?;
    page.wait_for_focus_on(&panel, "the all-disabled panel")
        .await
}

async fn vertical(page: &Page<'_>) -> Result<(), Report> {
    let list = tablist(page, "vertical").await?;
    assert_that!(attr(&list, "aria-orientation").await?).is_equal_to(Some("vertical".to_owned()));
    enter(page, "vertical").await?;
    page.send_keys_to_active(Key::Down).await?;
    expect_selected(page, "vertical", 1).await?;
    page.send_keys_to_active(Key::Up).await?;
    expect_selected(page, "vertical", 0).await?;
    page.send_keys_to_active(Key::Right).await?;
    expect_selected(page, "vertical", 1).await?;
    page.send_keys_to_active(Key::Left).await?;
    expect_selected(page, "vertical", 0).await
}

/// With manual activation, arrow keys only move focus; Enter selects.
async fn manual_activation(page: &Page<'_>) -> Result<(), Report> {
    enter(page, "manual").await?;
    expect_focus(page, "manual", 0).await?;
    page.send_keys_to_active(Key::Right).await?;
    expect_focus(page, "manual", 1).await?;
    page.send_keys_to_active(Key::Right).await?;
    expect_focus(page, "manual", 2).await?;
    expect_selected(page, "manual", 0).await?;
    page.send_keys_to_active(Key::Enter).await?;
    expect_selected(page, "manual", 2).await?;
    page.wait_for_text("test-tabs-manual-selection", "c").await
}

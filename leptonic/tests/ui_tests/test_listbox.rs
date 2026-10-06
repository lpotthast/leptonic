// Upstream: react-aria-components/test/ListBox.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// Behavior of the `ListBox` atom, asserted on the DOM/ARIA level so that these tests keep
/// passing while the collection hooks underneath are rewritten. Elements are found by role and
/// text, as users perceive them.
pub struct ListBoxTests {}

#[async_trait]
impl BrowserTest<str> for ListBoxTests {
    fn name(&self) -> Cow<'_, str> {
        "listbox_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/listbox").await?;

        aria_structure(&page).await?;
        keyboard_navigation_skips_disabled_items(&page).await?;
        selection(&page).await?;
        tab_in_and_out(&page).await?;
        type_ahead(&page).await?;
        select_all_and_clear(&page).await?;
        shift_arrow_extends_selection(&page).await?;

        Ok(())
    }
}

async fn option(page: &Page<'_>, fruit: &str) -> Result<WebElement, Report> {
    page.by_role_and_text("option", fruit).await
}

async fn attr(element: &WebElement, name: &str) -> Result<Option<String>, Report> {
    Ok(element.attr(name).await?)
}

async fn aria_structure(page: &Page<'_>) -> Result<(), Report> {
    let listbox = page.css("[role=listbox]").await?;
    assert_that!(attr(&listbox, "aria-label").await?).is_equal_to(Some("Fruits".to_owned()));
    assert_that!(attr(&listbox, "aria-multiselectable").await?)
        .is_equal_to(Some("true".to_owned()));
    for fruit in ["Apple", "Banana", "Cherry", "Durian", "Elderberry"] {
        let option = option(page, fruit).await?;
        assert_that!(attr(&option, "aria-selected").await?).is_equal_to(Some("false".to_owned()));
    }
    assert_that!(attr(&option(page, "Cherry").await?, "aria-disabled").await?)
        .is_equal_to(Some("true".to_owned()));
    Ok(())
}

async fn expect_focus(page: &Page<'_>, fruit: &str) -> Result<(), Report> {
    page.wait_for_active_text(fruit).await
}

async fn keyboard_navigation_skips_disabled_items(page: &Page<'_>) -> Result<(), Report> {
    option(page, "Apple").await?.click().await?;
    expect_focus(page, "Apple").await?;
    for (key, expected) in [
        (Key::Down, "Banana"),
        (Key::Down, "Durian"),
        (Key::Up, "Banana"),
        (Key::End, "Elderberry"),
        // No wrapping by default.
        (Key::Down, "Elderberry"),
        (Key::Home, "Apple"),
        (Key::Up, "Apple"),
    ] {
        page.send_keys_to_active(key.clone()).await?;
        expect_focus(page, expected)
            .await
            .map_err(|e| e.context(format!("after pressing {key:?}")).into_dynamic())?;
    }
    Ok(())
}

async fn selection(page: &Page<'_>) -> Result<(), Report> {
    // Clicking the first option above selected it (multiple selection, toggle behavior).
    page.wait_for_text("test-lb-selection", "Apple").await?;
    assert_that!(attr(&option(page, "Apple").await?, "aria-selected").await?)
        .is_equal_to(Some("true".to_owned()));

    option(page, "Durian").await?.click().await?;
    page.wait_for_text("test-lb-selection", "Apple,Durian")
        .await?;

    // Space toggles the focused option.
    page.send_keys_to_active(" ").await?;
    page.wait_for_text("test-lb-selection", "Apple").await?;

    // Disabled options can't be selected.
    option(page, "Cherry").await?.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    assert_that!(page.read_text_of("test-lb-selection").await?).is_equal_to("Apple".to_owned());
    Ok(())
}

/// The listbox is a single tab stop; focus returns to the last focused option.
async fn tab_in_and_out(page: &Page<'_>) -> Result<(), Report> {
    option(page, "Banana").await?.click().await?;
    page.press_tab().await?;
    assert_that!(page.active_element_id().await?).is_equal_to(Some("test-lb-after".to_owned()));
    page.press_shift_tab().await?;
    expect_focus(page, "Banana").await
}

/// Typing moves focus to the next option starting with the typed text, skipping disabled ones.
async fn type_ahead(page: &Page<'_>) -> Result<(), Report> {
    expect_focus(page, "Banana").await?;
    page.send_keys_to_active("d").await?;
    expect_focus(page, "Durian").await?;
    // Wait for the search to expire, then search again: "Cherry" is disabled.
    tokio::time::sleep(std::time::Duration::from_millis(1100)).await;
    page.send_keys_to_active("c").await?;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    assert_that!(page.active_element_text().await?).is_equal_to("Durian".to_owned());
    // Typing continues the search within a second: "e" then "l" finds "Elderberry".
    tokio::time::sleep(std::time::Duration::from_millis(1100)).await;
    page.send_keys_to_active("el").await?;
    expect_focus(page, "Elderberry").await
}

/// Ctrl+A selects everything, Escape clears the selection.
async fn select_all_and_clear(page: &Page<'_>) -> Result<(), Report> {
    page.send_keys_to_active(Key::Control + "a").await?;
    page.wait_for_text("test-lb-selection", "all").await?;
    assert_that!(attr(&option(page, "Durian").await?, "aria-selected").await?)
        .is_equal_to(Some("true".to_owned()));
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_text("test-lb-selection", "").await?;
    assert_that!(attr(&option(page, "Durian").await?, "aria-selected").await?)
        .is_equal_to(Some("false".to_owned()));
    Ok(())
}

/// Shift+Arrow extends the selection from the anchor (skipping the disabled option).
async fn shift_arrow_extends_selection(page: &Page<'_>) -> Result<(), Report> {
    option(page, "Banana").await?.click().await?;
    page.wait_for_text("test-lb-selection", "Banana").await?;
    page.send_keys_to_active(Key::Shift + Key::Down).await?;
    expect_focus(page, "Durian").await?;
    page.wait_for_text("test-lb-selection", "Banana,Durian")
        .await?;
    page.send_keys_to_active(Key::Shift + Key::Down).await?;
    page.wait_for_text("test-lb-selection", "Banana,Durian,Elderberry")
        .await
}

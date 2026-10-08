// Upstream: react-aria-components/test/ListBox.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::{Report, prelude::ResultExt};

use crate::pages::{ElementActions, Page, PageActions, role};

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
        cases!(
            aria_structure(&page),
            keyboard_navigation_skips_disabled_items(&page),
            selection(&page),
            tab_in_and_out(&page),
            type_ahead(&page),
            select_all_and_clear(&page),
            shift_arrow_extends_selection(&page),
        );
        Ok(())
    }
}

/// The option with the text `fruit`.
async fn option(page: &Page<'_>, fruit: &str) -> Result<WebElement, Report> {
    page.element(role("option").text(fruit)).await
}

/// The fixture's output of the selected keys, comma-separated (`all` for everything).
async fn selection_output(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-lb-selection").await
}

/// The listbox is labelled and multi-selectable; nothing is selected; Cherry is disabled.
async fn aria_structure(page: &Page<'_>) -> Result<(), Report> {
    let listbox = page.element("[role=listbox]").await?;
    assert_that!(listbox.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Fruits");
    assert_that!(listbox.attr("aria-multiselectable").await?)
        .get_some()
        .is_equal_to("true");
    for fruit in ["Apple", "Banana", "Cherry", "Durian", "Elderberry"] {
        let option = option(page, fruit).await?;
        assert_that!(option.attr("aria-selected").await?)
            .with_detail_message(fruit)
            .get_some()
            .is_equal_to("false");
        let disabled = option.attr("aria-disabled").await?;
        if fruit == "Cherry" {
            assert_that!(disabled).get_some().is_equal_to("true");
        } else {
            assert_that!(disabled).with_detail_message(fruit).is_none();
        }
    }
    Ok(())
}

/// Arrow keys, Home and End move focus, skipping the disabled option, without wrapping.
async fn keyboard_navigation_skips_disabled_items(page: &Page<'_>) -> Result<(), Report> {
    let apple = option(page, "Apple").await?;
    apple.click().await?;
    page.wait_for_focus(&apple).await?;
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
        page.send_keys(key.clone()).await?;
        page.wait_for_focus(&option(page, expected).await?)
            .await
            .context_with(|| format!("after pressing {key:?}"))?;
    }
    Ok(())
}

/// Clicking toggles an option (multiple selection, toggle behavior), Space toggles the focused
/// one, a disabled option can't be selected.
async fn selection(page: &Page<'_>) -> Result<(), Report> {
    let selection = selection_output(page).await?;
    // Clicking the first option above selected it.
    selection.wait_for_inner_text("Apple").await?;
    assert_that!(option(page, "Apple").await?.attr("aria-selected").await?)
        .get_some()
        .is_equal_to("true");

    option(page, "Durian").await?.click().await?;
    selection.wait_for_inner_text("Apple,Durian").await?;

    page.send_keys(" ").await?;
    selection.wait_for_inner_text("Apple").await?;

    option(page, "Cherry").await?.click().await?;
    selection.inner_text_stays("Apple").await?;
    Ok(())
}

/// The listbox is a single tab stop; focus returns to the last focused option.
async fn tab_in_and_out(page: &Page<'_>) -> Result<(), Report> {
    let banana = option(page, "Banana").await?;
    banana.click().await?;
    page.wait_for_focus(&banana).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-lb-after").await?)
        .await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&banana).await?;
    Ok(())
}

/// Typing moves focus to the next option starting with the typed text, skipping disabled ones.
async fn type_ahead(page: &Page<'_>) -> Result<(), Report> {
    page.wait_for_focus(&option(page, "Banana").await?).await?;
    page.send_keys("d").await?;
    let durian = option(page, "Durian").await?;
    page.wait_for_focus(&durian).await?;
    // A real timer: let the type-ahead search (1 s) expire, then search again: "Cherry" is
    // disabled.
    tokio::time::sleep(std::time::Duration::from_millis(1100)).await;
    page.send_keys("c").await?;
    page.focus_stays(&durian).await?;
    // A real timer: let the search expire again. Typing continues the search within a second:
    // "e" then "l" finds "Elderberry".
    tokio::time::sleep(std::time::Duration::from_millis(1100)).await;
    page.send_keys("el").await?;
    page.wait_for_focus(&option(page, "Elderberry").await?)
        .await?;
    Ok(())
}

/// Ctrl+A selects everything, Escape clears the selection.
async fn select_all_and_clear(page: &Page<'_>) -> Result<(), Report> {
    let selection = selection_output(page).await?;
    let durian = option(page, "Durian").await?;
    page.send_keys(Key::Control + "a").await?;
    selection.wait_for_inner_text("all").await?;
    assert_that!(durian.attr("aria-selected").await?)
        .get_some()
        .is_equal_to("true");
    page.send_keys(Key::Escape).await?;
    selection.wait_for_inner_text("").await?;
    assert_that!(durian.attr("aria-selected").await?)
        .get_some()
        .is_equal_to("false");
    Ok(())
}

/// Shift+Arrow extends the selection from the anchor (skipping the disabled option).
async fn shift_arrow_extends_selection(page: &Page<'_>) -> Result<(), Report> {
    let selection = selection_output(page).await?;
    option(page, "Banana").await?.click().await?;
    selection.wait_for_inner_text("Banana").await?;
    page.send_keys(Key::Shift + Key::Down).await?;
    page.wait_for_focus(&option(page, "Durian").await?).await?;
    selection.wait_for_inner_text("Banana,Durian").await?;
    page.send_keys(Key::Shift + Key::Down).await?;
    selection
        .wait_for_inner_text("Banana,Durian,Elderberry")
        .await?;
    Ok(())
}

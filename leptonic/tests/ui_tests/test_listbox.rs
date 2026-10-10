// Upstream: react-aria-components/test/ListBox.test.js @ 99e6102368
//! Behavior of the `ListBox` atom, asserted on the DOM/ARIA level so that these tests keep
//! passing while the collection hooks underneath are rewritten. Elements are found by role and
//! text, as users perceive them.
use assertr::prelude::*;
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::{Report, prelude::ResultExt};

use crate::pages::{ElementActions, Page, role};

const PATH: &str = "/atoms/listbox";

/// The option with the text `fruit`.
async fn option(page: &Page<'_>, fruit: &str) -> Result<WebElement, Report> {
    page.element(role(AriaRole::Option).text(fruit)).await
}

/// The fixture's output of the selected keys, comma-separated (`all` for everything).
async fn selection_output(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-lb-selection").await
}

/// Tab from the button before the listbox into it: focus lands on the first option, nothing gets
/// selected.
async fn tab_in(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-lb-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&option(page, "Apple").await?).await?;
    Ok(())
}

/// The listbox has its `aria-label` and is multiselectable, every option starts unselected and
/// only the disabled one has `aria-disabled`.
#[browser_test]
pub async fn aria_structure(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let listbox = page.element("[role=listbox]").await?;
    assert_that!(listbox)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Fruits");
    assert_that!(listbox)
        .has_attribute("aria-multiselectable")
        .await
        .is_equal_to("true");
    for fruit in ["Apple", "Banana", "Cherry", "Durian", "Elderberry"] {
        let option = option(page, fruit).await?;
        assert_that!(option)
            .with_detail_message(fruit)
            .has_attribute("aria-selected")
            .await
            .is_equal_to("false");
        if fruit == "Cherry" {
            assert_that!(option)
                .has_attribute("aria-disabled")
                .await
                .is_equal_to("true");
        } else {
            assert_that!(option)
                .with_detail_message(fruit)
                .attribute("aria-disabled")
                .await
                .is_none();
        }
    }
    Ok(())
}

/// Arrow keys, Home and End move the focus, skipping the disabled option and not wrapping at
/// either end.
#[browser_test]
pub async fn keyboard_navigation_skips_disabled_items(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
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

/// With multiple toggle selection, clicking an option and Space on the focused one toggle it, while
/// clicking a disabled option selects nothing ("should support selection state").
#[browser_test]
pub async fn selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let selection = selection_output(page).await?;
    option(page, "Apple").await?.click().await?;
    selection.wait_for_inner_text("Apple").await?;
    assert_that!(option(page, "Apple").await?)
        .has_attribute("aria-selected")
        .await
        .is_equal_to("true");

    option(page, "Durian").await?.click().await?;
    selection.wait_for_inner_text("Apple,Durian").await?;

    page.send_keys(" ").await?;
    selection.wait_for_inner_text("Apple").await?;

    option(page, "Cherry").await?.click().await?;
    selection
        .inner_text_stays("Apple", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// The listbox is a single tab stop: Tab leaves it, and Shift+Tab returns to the last focused
/// option.
#[browser_test]
pub async fn tab_in_and_out(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
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

/// Typing moves the focus to the next option starting with the typed text, skipping disabled
/// ones, and text matching no option ends the search so the next key starts a new one.
#[browser_test]
pub async fn type_ahead(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    tab_in(page).await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&option(page, "Banana").await?).await?;
    page.send_keys("d").await?;
    let durian = option(page, "Durian").await?;
    page.wait_for_focus(&durian).await?;
    // "dx" matches nothing: the search ends. "c" alone matches only Cherry, which is disabled:
    // the focus stays, and that search ends too.
    page.send_keys("x").await?;
    page.send_keys("c").await?;
    page.focus_stays(&durian, std::time::Duration::from_millis(100))
        .await?;
    // Typing continues a search: "e" then "l" finds "Elderberry".
    page.send_keys("el").await?;
    page.wait_for_focus(&option(page, "Elderberry").await?)
        .await?;
    Ok(())
}

/// Ctrl+A selects every option and Escape clears the selection ("selects all with Mod+A
/// (Control) in multiple selection mode").
#[browser_test]
pub async fn select_all_and_clear(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let selection = selection_output(page).await?;
    let durian = option(page, "Durian").await?;
    tab_in(page).await?;
    page.send_keys(page.primary_modifier().await? + "a").await?;
    selection.wait_for_inner_text("all").await?;
    assert_that!(durian)
        .has_attribute("aria-selected")
        .await
        .is_equal_to("true");
    page.send_keys(Key::Escape).await?;
    selection.wait_for_inner_text("").await?;
    assert_that!(durian)
        .has_attribute("aria-selected")
        .await
        .is_equal_to("false");
    Ok(())
}

/// Shift+ArrowDown extends the selection from the clicked option, skipping the disabled one
/// ("extends selection when Shift is held").
#[browser_test]
pub async fn shift_arrow_extends_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
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

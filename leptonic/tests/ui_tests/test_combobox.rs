// Upstream: react-aria-components/test/ComboBox.test.js @ 99e6102368
use std::{borrow::Cow, time::Duration};

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

const INPUT: &str = "[role=combobox]";
const LISTBOX: &str = "[role=listbox]";

/// Behavior of the `ComboBox` atoms: filtering while typing, virtual focus (DOM focus stays in
/// the input, `aria-activedescendant` points at the focused option), keyboard and pointer
/// selection, reverting with Escape, a controlled value changed from outside, and a combo box
/// in a modal dialog.
pub struct ComboBoxTests {}

#[async_trait]
impl BrowserTest<str> for ComboBoxTests {
    fn name(&self) -> Cow<'_, str> {
        "combobox_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/combobox").await?;

        aria_structure(&page).await?;
        typing_filters_and_keyboard_selects(&page).await?;
        escape_reverts_the_input(&page).await?;
        button_shows_all_options_and_click_selects(&page).await?;
        arrow_down_opens_with_the_selected_option_focused(&page).await?;
        clearing_the_input_clears_the_value(&page).await?;
        externally_changed_value_shows_in_the_input(&page).await?;
        popover_in_a_modal_stays_interactive(&page).await?;

        Ok(())
    }
}

async fn input(page: &Page<'_>) -> Result<WebElement, Report> {
    page.css(INPUT).await
}

async fn input_attr(page: &Page<'_>, name: &str) -> Result<Option<String>, Report> {
    Ok(input(page).await?.attr(name).await?)
}

async fn input_value(page: &Page<'_>) -> Result<String, Report> {
    Ok(input(page).await?.prop("value").await?.unwrap_or_default())
}

async fn option_texts(page: &Page<'_>) -> Result<Vec<String>, Report> {
    let mut texts = Vec::new();
    for option in page.driver.find_all(By::Css("[role=option]")).await? {
        texts.push(option.text().await?);
    }
    Ok(texts)
}

/// Waits until the listbox shows exactly `expected`.
async fn expect_options(page: &Page<'_>, expected: &[&str]) -> Result<(), Report> {
    let deadline = std::time::Instant::now() + Duration::from_secs(10);
    loop {
        let texts = option_texts(page).await?;
        if texts == expected {
            return Ok(());
        }
        if std::time::Instant::now() > deadline {
            leptos_browser_test::bail!("expected options {expected:?}, got {texts:?}");
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

/// The focused option is the input's active descendant; DOM focus stays in the input.
async fn expect_virtual_focus(page: &Page<'_>, option: &str) -> Result<(), Report> {
    let deadline = std::time::Instant::now() + Duration::from_secs(10);
    loop {
        let active_descendant = input_attr(page, "aria-activedescendant").await?;
        let focused = match &active_descendant {
            Some(id) if !id.is_empty() => {
                Some(page.css(&format!("[id='{id}']")).await?.text().await?)
            }
            _ => None,
        };
        if focused.as_deref() == Some(option) {
            break;
        }
        if std::time::Instant::now() > deadline {
            leptos_browser_test::bail!(
                "expected {option:?} to be the active descendant, got {focused:?}"
            );
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    let active = page.driver.active_element().await?;
    assert_that!(active == input(page).await?)
        .with_detail_message("DOM focus stays in the input")
        .is_true();
    Ok(())
}

async fn aria_structure(page: &Page<'_>) -> Result<(), Report> {
    assert_that!(input_attr(page, "aria-expanded").await?).is_equal_to(Some("false".to_owned()));
    assert_that!(input_attr(page, "aria-autocomplete").await?).is_equal_to(Some("list".to_owned()));
    assert_that!(input_attr(page, "aria-controls").await?).is_none();
    let labelled_by = input_attr(page, "aria-labelledby")
        .await?
        .unwrap_or_default();
    let label = page.css(&format!("[id='{labelled_by}']")).await?;
    assert_that!(label.text().await?).is_equal_to("Fruit".to_owned());
    // The button isn't in the tab order.
    let button = page.driver.find(By::Css("button[aria-haspopup]")).await?;
    assert_that!(button.attr("tabindex").await?).is_equal_to(Some("-1".to_owned()));
    Ok(())
}

async fn typing_filters_and_keyboard_selects(page: &Page<'_>) -> Result<(), Report> {
    let input = input(page).await?;
    input.click().await?;
    input.send_keys("an").await?;
    page.wait_for_selector(LISTBOX).await?;
    expect_options(page, &["Banana", "Durian"]).await?;
    assert_that!(input_attr(page, "aria-expanded").await?).is_equal_to(Some("true".to_owned()));
    let listbox_id = page.css(LISTBOX).await?.attr("id").await?;
    assert_that!(input_attr(page, "aria-controls").await?).is_equal_to(listbox_id);
    // The combo box hides the rest of the page from screen readers (react-aria's `useComboBox`),
    // but not its input, and leaves the page usable: `aria-hidden`, nothing inert.
    assert_that!(page.count_matching("[aria-hidden=true]").await?).is_greater_than(0);
    assert_that!(within(page, &input, "[aria-hidden=true]").await?).is_false();
    assert_that!(page.count_matching("[inert]").await?).is_equal_to(0);

    input.send_keys(Key::Down).await?;
    expect_virtual_focus(page, "Banana").await?;
    input.send_keys(Key::Down).await?;
    expect_virtual_focus(page, "Durian").await?;
    input.send_keys(Key::Up).await?;
    expect_virtual_focus(page, "Banana").await?;

    input.send_keys(Key::Enter).await?;
    page.wait_for_text("test-cb-value", "Banana").await?;
    page.wait_for_no_selector(LISTBOX).await?;
    assert_that!(input_value(page).await?).is_equal_to("Banana".to_owned());
    Ok(())
}

async fn escape_reverts_the_input(page: &Page<'_>) -> Result<(), Report> {
    let input = input(page).await?;
    input.send_keys("x").await?;
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert_that!(input_value(page).await?).is_equal_to("Bananax".to_owned());
    input.send_keys(Key::Escape).await?;
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    while input_value(page).await? != "Banana" {
        if std::time::Instant::now() > deadline {
            leptos_browser_test::bail!("Escape did not revert the input");
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    assert_that!(page.read_text_of("test-cb-value").await?).is_equal_to("Banana".to_owned());
    Ok(())
}

/// The button shows all options (not just the matching ones); clicking an option selects it and
/// keeps focus in the input.
async fn button_shows_all_options_and_click_selects(page: &Page<'_>) -> Result<(), Report> {
    page.driver
        .find(By::Css("button[aria-haspopup]"))
        .await?
        .click()
        .await?;
    page.wait_for_selector(LISTBOX).await?;
    expect_options(page, &["Apple", "Banana", "Cherry", "Durian", "Elderberry"]).await?;
    page.by_role_and_text("option", "Durian")
        .await?
        .click()
        .await?;
    page.wait_for_text("test-cb-value", "Durian").await?;
    page.wait_for_no_selector(LISTBOX).await?;
    assert_that!(input_value(page).await?).is_equal_to("Durian".to_owned());
    let active = page.driver.active_element().await?;
    assert_that!(active == input(page).await?).is_true();
    Ok(())
}

/// ArrowDown opens all options. The selected option gets focus; it takes precedence over the
/// "first" focus strategy (react-aria `useSelectableCollection` auto focus).
async fn arrow_down_opens_with_the_selected_option_focused(page: &Page<'_>) -> Result<(), Report> {
    input(page).await?.send_keys(Key::Down).await?;
    page.wait_for_selector(LISTBOX).await?;
    expect_options(page, &["Apple", "Banana", "Cherry", "Durian", "Elderberry"]).await?;
    expect_virtual_focus(page, "Durian").await?;
    // Count the synthetic focus events the input gets from now on.
    page.driver
        .execute(
            "window.__virtualInputFocus = 0; arguments[0].addEventListener('focus', e => { \
             if (!e.isTrusted) window.__virtualInputFocus++; });",
            vec![input(page).await?.to_json()?],
        )
        .await?;
    input(page).await?.send_keys(Key::Escape).await?;
    page.wait_for_no_selector(LISTBOX).await?;
    // No option is virtually focused any more: a virtual focus event on the input lets its focus
    // ring show again (react-aria's "re-show focus ring" effect).
    let count = page
        .driver
        .execute("return window.__virtualInputFocus;", vec![])
        .await?;
    assert_that!(count.json().as_u64()).is_equal_to(Some(1));
    Ok(())
}

/// Single selection: emptying the input clears the value.
async fn clearing_the_input_clears_the_value(page: &Page<'_>) -> Result<(), Report> {
    let input = input(page).await?;
    input.send_keys(Key::Control + "a").await?;
    input.send_keys(Key::Backspace).await?;
    page.wait_for_text("test-cb-value", "").await
}

/// The input of the combo box labelled `label`.
async fn input_labelled(page: &Page<'_>, label: &str) -> Result<WebElement, Report> {
    let label = page
        .driver
        .find(By::XPath(format!("//label[normalize-space()='{label}']")))
        .await?;
    let id = label.attr("for").await?.unwrap_or_default();
    page.element(&id).await
}

async fn wait_for_input_value(input: &WebElement, expected: &str) -> Result<(), Report> {
    let deadline = std::time::Instant::now() + Duration::from_secs(10);
    loop {
        let value = input.prop("value").await?.unwrap_or_default();
        if value == expected {
            return Ok(());
        }
        if std::time::Instant::now() > deadline {
            leptos_browser_test::bail!("expected input value {expected:?}, got {value:?}");
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

/// A controlled value changed from outside resets the input to the selected item's text
/// (upstream resets it whenever the selected key changes), also for an item added to the
/// collection in the same update.
async fn externally_changed_value_shows_in_the_input(page: &Page<'_>) -> Result<(), Report> {
    let input = input_labelled(page, "Controlled fruit").await?;
    wait_for_input_value(&input, "Apple").await?;
    page.click_element_with_id("test-cb-controlled-set").await?;
    wait_for_input_value(&input, "Banana").await?;
    page.click_element_with_id("test-cb-controlled-add").await?;
    wait_for_input_value(&input, "Fig").await?;
    page.click_element_with_id("test-cb-controlled-select-then-add")
        .await?;
    wait_for_input_value(&input, "Grape").await?;

    // Value and items derived from one app signal; the previous item leaves as the new one
    // comes (agnite dev-ui's log source picker).
    let input = input_labelled(page, "Logs of").await?;
    wait_for_input_value(&input, "Orchestration").await?;
    page.click_element_with_id("test-cb-derived-1").await?;
    wait_for_input_value(&input, "Process 1").await?;
    page.click_element_with_id("test-cb-derived-2").await?;
    wait_for_input_value(&input, "Process 2").await?;
    // All options show: the input's text is the selection, not a filter.
    input
        .find(By::XPath("following-sibling::button"))
        .await?
        .click()
        .await?;
    expect_options(page, &["Orchestration", "Process 2"]).await?;
    input.send_keys(Key::Escape).await?;

    // Changed by a timer, with no event around it (agnite dev-ui's minimal case): the input
    // follows, and the next button press opens the popover.
    let input = input_labelled(page, "Timed fruit").await?;
    page.click_element_with_id("test-cb-timed-start").await?;
    wait_for_input_value(&input, "Banana").await?;
    input
        .find(By::XPath("following-sibling::button"))
        .await?
        .click()
        .await?;
    page.wait_for_selector(LISTBOX).await?;
    input.send_keys(Key::Escape).await?;
    page.wait_for_no_selector(LISTBOX).await?;
    // Written while an effect runs.
    page.click_element_with_id("test-cb-timed-effect").await?;
    wait_for_input_value(&input, "Cherry").await?;
    input
        .find(By::XPath("following-sibling::button"))
        .await?
        .click()
        .await?;
    page.wait_for_selector(LISTBOX).await?;
    input.send_keys(Key::Escape).await?;
    Ok(())
}

/// A combo box in a modal: its popover is portaled next to the modal, which hides everything
/// outside it, but the popover opened from inside stays interactive (agnite dev-ui's report).
async fn popover_in_a_modal_stays_interactive(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-cb-modal-open").await?;
    page.wait_for_selector("[role=dialog] [role=combobox]")
        .await?;
    page.css("[role=dialog] button").await?.click().await?;
    expect_options(page, &["Apple", "Banana", "Cherry", "Durian", "Elderberry"]).await?;
    let option = page.by_role_and_text("option", "Durian").await?;
    assert_that!(within(page, &option, "[inert]").await?).is_false();
    option.click().await?;
    page.wait_for_text("test-cb-modal-value", "Durian").await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_no_selector("[role=dialog]").await
}

/// Whether `element` is or is inside an element matching `selector`.
async fn within(page: &Page<'_>, element: &WebElement, selector: &str) -> Result<bool, Report> {
    Ok(page
        .driver
        .execute(
            "return arguments[0].closest(arguments[1]) !== null;",
            vec![element.to_json()?, serde_json::Value::from(selector)],
        )
        .await?
        .convert::<bool>()?)
}

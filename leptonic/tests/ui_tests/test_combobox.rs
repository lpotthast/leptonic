// Upstream: react-aria-components/test/ComboBox.test.js @ 99e6102368
//! Behavior of the `ComboBox` atoms: filtering while typing, virtual focus (DOM focus stays in
//! the input, `aria-activedescendant` points at the focused option), keyboard and pointer
//! selection, reverting with Escape, a controlled value changed from outside, and a combo box
//! in a modal dialog.
use assertr::{matchers::eq, prelude::*};
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions, role, xpath};

const PATH: &str = "/atoms/combobox";

const INPUT: &str = "[role=combobox]";
const LISTBOX: &str = "[role=listbox]";

/// Waits until the listbox shows exactly `expected`.
async fn expect_options(page: &Page<'_>, expected: &[&str]) -> Result<(), Report> {
    assert_that!(|| page.inner_texts("[role=option]"))
        .eventually_ok()
        .matches(eq(expected))
        .await;
    Ok(())
}

/// Waits until the option with the text `option` is the input's active descendant (virtual
/// focus); DOM focus stays in the input.
async fn expect_virtual_focus(page: &Page<'_>, option: &str) -> Result<(), Report> {
    let input = page.element(INPUT).await?;
    assert_that!(|| async {
        Ok::<_, Report>(match input.attr("aria-activedescendant").await? {
            Some(id) if !id.is_empty() => {
                Some(page.element(format!("#{id}")).await?.inner_text().await?)
            }
            _ => None,
        })
    })
    .eventually_ok()
    .matches(eq(Some(option.to_owned())))
    .await;
    page.focus_stays(&input).await?;
    Ok(())
}

/// Whether `element` is or is inside an element with the attribute `attr` (`name='value'` or
/// `name`).
async fn within(element: &WebElement, attr: &str) -> Result<bool, Report> {
    Ok(element
        .count(xpath(format!("ancestor-or-self::*[@{attr}]")))
        .await?
        > 0)
}

/// The input of the combo box labelled `label`.
async fn input_labelled(page: &Page<'_>, label: &str) -> Result<WebElement, Report> {
    let label = page
        .element(xpath(format!("//label[normalize-space()='{label}']")))
        .await?;
    let id = label.attr("for").await?.unwrap_or_default();
    page.element(format!("#{id}")).await
}

/// The button of the combo box whose input is `input`.
async fn button_of(input: &WebElement) -> Result<WebElement, Report> {
    input.element(xpath("following-sibling::button")).await
}

/// Selects `option` of the first combo box with its button and a click on the option: the
/// options close, focus stays in the input.
async fn select(page: &Page<'_>, option: &str) -> Result<(), Report> {
    let input = page.element(INPUT).await?;
    page.element("button[aria-haspopup]").await?.click().await?;
    page.element(role("option").text(option))
        .await?
        .click()
        .await?;
    page.element("#test-cb-value")
        .await?
        .wait_for_inner_text(option)
        .await?;
    page.wait_for_count(LISTBOX, 0).await?;
    input.wait_for_prop("value", option).await?;
    page.wait_for_focus(&input).await?;
    Ok(())
}

/// Closed: a list autocomplete labelled by its label, the button out of the tab order.
pub async fn aria_structure(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    assert_that!(input.attr("aria-expanded").await?)
        .get_some()
        .is_equal_to("false");
    assert_that!(input.attr("aria-autocomplete").await?)
        .get_some()
        .is_equal_to("list");
    assert_that!(input.attr("aria-controls").await?).is_none();
    let labelled_by = input.attr("aria-labelledby").await?.unwrap_or_default();
    let label = page.element(format!("#{labelled_by}")).await?;
    assert_that!(label.inner_text().await?).is_equal_to("Fruit");
    let button = page.element("button[aria-haspopup]").await?;
    assert_that!(button.attr("tabindex").await?)
        .get_some()
        .is_equal_to("-1");
    Ok(())
}

/// Typing opens the filtered options (the rest of the page hidden from screen readers, nothing
/// inert); arrow keys move virtual focus; Enter selects.
pub async fn typing_filters_and_keyboard_selects(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    input.click().await?;
    input.send_keys("an").await?;
    let listbox = page.element(LISTBOX).await?;
    expect_options(page, &["Banana", "Durian"]).await?;
    input.wait_for_attr("aria-expanded", Some("true")).await?;
    let listbox_id = listbox.attr("id").await?;
    assert_that!(input.attr("aria-controls").await?).is_equal_to(listbox_id);
    // The combo box hides the rest of the page from screen readers (react-aria's `useComboBox`),
    // but not its input, and leaves the page usable: `aria-hidden`, nothing inert.
    assert_that!(page.count("[aria-hidden=true]").await?).is_greater_than(0);
    assert_that!(within(&input, "aria-hidden='true'").await?).is_false();
    assert_that!(page.count("[inert]").await?).is_equal_to(0);

    input.send_keys(Key::Down).await?;
    expect_virtual_focus(page, "Banana").await?;
    input.send_keys(Key::Down).await?;
    expect_virtual_focus(page, "Durian").await?;
    input.send_keys(Key::Up).await?;
    expect_virtual_focus(page, "Banana").await?;

    input.send_keys(Key::Enter).await?;
    page.element("#test-cb-value")
        .await?
        .wait_for_inner_text("Banana")
        .await?;
    page.wait_for_count(LISTBOX, 0).await?;
    assert_that!(input.value().await?)
        .get_some()
        .is_equal_to("Banana");
    Ok(())
}

/// Escape reverts the typed text to the selection's.
pub async fn escape_reverts_the_input(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    select(page, "Banana").await?;
    let input = page.element(INPUT).await?;
    input.send_keys("x").await?;
    input.wait_for_prop("value", "Bananax").await?;
    input.send_keys(Key::Escape).await?;
    input.wait_for_prop("value", "Banana").await?;
    page.element("#test-cb-value")
        .await?
        .inner_text_stays("Banana")
        .await?;
    Ok(())
}

/// The button shows all options (not just the matching ones); clicking an option selects it and
/// keeps focus in the input.
pub async fn button_shows_all_options_and_click_selects(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element(INPUT).await?;
    page.element("button[aria-haspopup]").await?.click().await?;
    expect_options(page, &["Apple", "Banana", "Cherry", "Durian", "Elderberry"]).await?;
    page.element(role("option").text("Durian"))
        .await?
        .click()
        .await?;
    page.element("#test-cb-value")
        .await?
        .wait_for_inner_text("Durian")
        .await?;
    page.wait_for_count(LISTBOX, 0).await?;
    assert_that!(input.value().await?)
        .get_some()
        .is_equal_to("Durian");
    page.wait_for_focus(&input).await?;
    Ok(())
}

/// ArrowDown opens all options. The selected option gets focus; it takes precedence over the
/// "first" focus strategy (react-aria `useSelectableCollection` auto focus). Closing fires one
/// virtual focus event on the input, so that its focus ring shows again (react-aria's "re-show
/// focus ring" effect).
pub async fn arrow_down_opens_with_the_selected_option_focused(
    page: &Page<'_>,
) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    select(page, "Durian").await?;
    let input = page.element(INPUT).await?;
    input.send_keys(Key::Down).await?;
    expect_options(page, &["Apple", "Banana", "Cherry", "Durian", "Elderberry"]).await?;
    expect_virtual_focus(page, "Durian").await?;
    // Count the synthetic focus events the input gets from now on.
    page.eval::<()>(
        "window.__virtualInputFocus = 0;
         arguments[0].addEventListener('focus', e => {
             if (!e.isTrusted) window.__virtualInputFocus++;
         });",
        vec![input.to_json()?],
    )
    .await?;
    input.send_keys(Key::Escape).await?;
    page.wait_for_count(LISTBOX, 0).await?;
    assert_that!(|| page.eval::<u64>("return window.__virtualInputFocus;", vec![]))
        .eventually_ok()
        .matches(eq(1))
        .await;
    Ok(())
}

/// Single selection: emptying the input clears the value. The changed input opens the options
/// (react-stately opens on every input change while focused); closing them keeps the cleared
/// value.
pub async fn clearing_the_input_clears_the_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    select(page, "Durian").await?;
    let input = page.element(INPUT).await?;
    let value = page.element("#test-cb-value").await?;
    input.send_keys(Key::Control + "a").await?;
    input.send_keys(Key::Backspace).await?;
    value.wait_for_inner_text("").await?;
    page.element(LISTBOX).await?;
    input.send_keys(Key::Escape).await?;
    page.wait_for_count(LISTBOX, 0).await?;
    value.inner_text_stays("").await?;
    Ok(())
}

/// A controlled value changed from outside resets the input to the selected item's text
/// (upstream resets it whenever the selected key changes), also for an item added to the
/// collection in the same update.
pub async fn externally_changed_value_shows_in_the_input(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_labelled(page, "Controlled fruit").await?;
    input.wait_for_prop("value", "Apple").await?;
    page.element("#test-cb-controlled-set")
        .await?
        .click()
        .await?;
    input.wait_for_prop("value", "Banana").await?;
    page.element("#test-cb-controlled-add")
        .await?
        .click()
        .await?;
    input.wait_for_prop("value", "Fig").await?;
    page.element("#test-cb-controlled-select-then-add")
        .await?
        .click()
        .await?;
    input.wait_for_prop("value", "Grape").await?;

    // Value and items derived from one app signal; the previous item leaves as the new one
    // comes (agnite dev-ui's log source picker).
    let input = input_labelled(page, "Logs of").await?;
    input.wait_for_prop("value", "Orchestration").await?;
    page.element("#test-cb-derived-1").await?.click().await?;
    input.wait_for_prop("value", "Process 1").await?;
    page.element("#test-cb-derived-2").await?.click().await?;
    input.wait_for_prop("value", "Process 2").await?;
    // All options show: the input's text is the selection, not a filter.
    button_of(&input).await?.click().await?;
    expect_options(page, &["Orchestration", "Process 2"]).await?;
    input.send_keys(Key::Escape).await?;
    page.wait_for_count(LISTBOX, 0).await?;

    // Changed by a timer, with no event around it (agnite dev-ui's minimal case): the input
    // follows, and the next button press opens the popover.
    let input = input_labelled(page, "Timed fruit").await?;
    page.element("#test-cb-timed-start").await?.click().await?;
    input.wait_for_prop("value", "Banana").await?;
    button_of(&input).await?.click().await?;
    page.element(LISTBOX).await?;
    input.send_keys(Key::Escape).await?;
    page.wait_for_count(LISTBOX, 0).await?;
    // Written while an effect runs.
    page.element("#test-cb-timed-effect").await?.click().await?;
    input.wait_for_prop("value", "Cherry").await?;
    button_of(&input).await?.click().await?;
    page.element(LISTBOX).await?;
    input.send_keys(Key::Escape).await?;
    page.wait_for_count(LISTBOX, 0).await?;
    Ok(())
}

/// A combo box in a modal: its popover is portaled next to the modal, which hides everything
/// outside it, but the popover opened from inside stays interactive (agnite dev-ui's report).
pub async fn popover_in_a_modal_stays_interactive(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-cb-modal-open").await?.click().await?;
    page.element("[role=dialog] [role=combobox]").await?;
    page.element("[role=dialog] button").await?.click().await?;
    expect_options(page, &["Apple", "Banana", "Cherry", "Durian", "Elderberry"]).await?;
    let option = page.element(role("option").text("Durian")).await?;
    assert_that!(within(&option, "inert").await?).is_false();
    option.click().await?;
    page.element("#test-cb-modal-value")
        .await?
        .wait_for_inner_text("Durian")
        .await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count("[role=dialog]", 0).await?;
    Ok(())
}

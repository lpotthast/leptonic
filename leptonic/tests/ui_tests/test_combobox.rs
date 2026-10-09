// Upstream: react-aria-components/test/ComboBox.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/combobox/ComboBox.test.js @ 99e6102368
//! Behavior of the `ComboBox` atoms: filtering while typing, virtual focus (DOM focus stays in
//! the input, `aria-activedescendant` points at the focused option), keyboard and pointer
//! selection, reverting with Escape, a controlled value changed from outside, and a combo box
//! in a modal dialog.
use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::Report;

use crate::pages::{ElementActions, EventKind, KeyKind, Page, SyntheticEvent, css, role};

const PATH: &str = "/atoms/combobox";

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
    let input = input_labelled(page, "Fruit").await?;
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
    page.focus_stays(&input, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// The input of the combo box labelled `label`.
async fn input_labelled(page: &Page<'_>, label: &str) -> Result<WebElement, Report> {
    let label = page.element(css("label").text(label)).await?;
    let id = label.attr("for").await?.unwrap_or_default();
    page.element(format!("#{id}")).await
}

/// The button of the combo box whose input is `input`.
async fn button_of(input: &WebElement) -> Result<WebElement, Report> {
    input.parent().await?.element(":scope > button").await
}

/// Selects `option` of the Fruit combo box with its button and a click on the option: the
/// options close, focus stays in the input.
async fn select(page: &Page<'_>, option: &str) -> Result<(), Report> {
    let input = input_labelled(page, "Fruit").await?;
    button_of(&input).await?.click().await?;
    page.element(role(AriaRole::Option).text(option))
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

/// The closed combo box is a collapsed list autocomplete labelled by its label, and its button is
/// out of the tab order.
#[browser_test]
pub async fn aria_structure(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_labelled(page, "Fruit").await?;
    assert_that!(input)
        .has_attribute("aria-expanded")
        .await
        .is_equal_to("false");
    assert_that!(input)
        .has_attribute("aria-autocomplete")
        .await
        .is_equal_to("list");
    assert_that!(input)
        .attribute("aria-controls")
        .await
        .is_none();
    let labelled_by = input.attr("aria-labelledby").await?.unwrap_or_default();
    let label = page.element(format!("#{labelled_by}")).await?;
    assert_that!(label).inner_text().await.is_equal_to("Fruit");
    let button = button_of(&input).await?;
    assert_that!(button)
        .has_attribute("tabindex")
        .await
        .is_equal_to("-1");
    Ok(())
}

/// Typing opens the filtered options and hides the rest of the page from screen readers without
/// making it inert; arrow keys move virtual focus and Enter selects ("should support selecting an
/// option via keyboard").
#[browser_test]
pub async fn typing_filters_and_keyboard_selects(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_labelled(page, "Fruit").await?;
    input.click().await?;
    input.send_keys("an").await?;
    let listbox = page.element(LISTBOX).await?;
    expect_options(page, &["Banana", "Durian"]).await?;
    input.wait_for_attr("aria-expanded", Some("true")).await?;
    let listbox_id = listbox.attr("id").await?;
    assert_that!(input)
        .attribute("aria-controls")
        .await
        .is_equal_to(listbox_id);
    // The combo box hides the rest of the page from screen readers (react-aria's `useComboBox`),
    // but not its input and options, and leaves the page usable: `aria-hidden`, nothing inert.
    let outside = page.element("#test-cb-value").await?;
    assert_that!(|| outside.is_within("[aria-hidden='true']"))
        .eventually_ok()
        .matches(eq(true))
        .await;
    page.settle().await?;
    assert_that!(|| async {
        Ok::<_, Report>((
            input.is_within("[aria-hidden='true']").await?,
            listbox.is_within("[aria-hidden='true']").await?,
            page.count("[inert]").await?,
        ))
    })
    .with_detail_message("(input hidden, listbox hidden, inert elements)")
    .consistently_ok()
    .for_at_least(std::time::Duration::from_millis(100))
    .matches(eq((false, false, 0)))
    .await;

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
    assert_that!(input)
        .property("value")
        .await
        .get_some()
        .is_equal_to("Banana");
    Ok(())
}

/// Escape reverts the typed text to the selection's.
#[browser_test]
pub async fn escape_reverts_the_input(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    select(page, "Banana").await?;
    let input = input_labelled(page, "Fruit").await?;
    input.send_keys("x").await?;
    input.wait_for_prop("value", "Bananax").await?;
    input.send_keys(Key::Escape).await?;
    input.wait_for_prop("value", "Banana").await?;
    page.element("#test-cb-value")
        .await?
        .inner_text_stays("Banana", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// The button shows all options (not just the matching ones); clicking an option selects it and
/// keeps focus in the input.
#[browser_test]
pub async fn button_shows_all_options_and_click_selects(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_labelled(page, "Fruit").await?;
    button_of(&input).await?.click().await?;
    expect_options(page, &["Apple", "Banana", "Cherry", "Durian", "Elderberry"]).await?;
    page.element(role(AriaRole::Option).text("Durian"))
        .await?
        .click()
        .await?;
    page.element("#test-cb-value")
        .await?
        .wait_for_inner_text("Durian")
        .await?;
    page.wait_for_count(LISTBOX, 0).await?;
    assert_that!(input)
        .property("value")
        .await
        .get_some()
        .is_equal_to("Durian");
    page.wait_for_focus(&input).await?;
    Ok(())
}

/// ArrowDown opens all options with the selected one focused instead of the first; closing them
/// fires exactly one synthetic focus event on the input, so its focus ring shows again.
#[browser_test]
pub async fn arrow_down_opens_with_the_selected_option_focused(
    page: &Page<'_>,
) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    select(page, "Durian").await?;
    let input = input_labelled(page, "Fruit").await?;
    input.send_keys(Key::Down).await?;
    expect_options(page, &["Apple", "Banana", "Cherry", "Durian", "Elderberry"]).await?;
    expect_virtual_focus(page, "Durian").await?;
    let focus_events = input.count_synthetic_events("focus").await?;
    input.send_keys(Key::Escape).await?;
    page.wait_for_count(LISTBOX, 0).await?;
    assert_that!(|| focus_events.count())
        .eventually_ok()
        .matches(eq(1))
        .await;
    // Exactly one.
    page.settle().await?;
    assert_that!(|| focus_events.count())
        .consistently_ok()
        .for_at_least(std::time::Duration::from_millis(100))
        .matches(eq(1))
        .await;
    focus_events.finish().await?;
    Ok(())
}

/// Emptying the input clears the selection and opens the options; closing them with Escape keeps
/// the selection cleared.
#[browser_test]
pub async fn clearing_the_input_clears_the_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    select(page, "Durian").await?;
    let input = input_labelled(page, "Fruit").await?;
    let value = page.element("#test-cb-value").await?;
    input.send_keys(Key::Control + "a").await?;
    input.send_keys(Key::Backspace).await?;
    value.wait_for_inner_text("").await?;
    page.element(LISTBOX).await?;
    input.send_keys(Key::Escape).await?;
    page.wait_for_count(LISTBOX, 0).await?;
    value
        .inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A controlled value changed from outside resets the input to the selected item's text
/// (upstream resets it whenever the selected key changes).
#[browser_test]
pub async fn externally_changed_value_shows_in_the_input(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_labelled(page, "Controlled fruit").await?;
    input.wait_for_prop("value", "Apple").await?;
    page.element("#test-cb-controlled-set")
        .await?
        .click()
        .await?;
    input.wait_for_prop("value", "Banana").await?;
    Ok(())
}

/// A controlled value selecting an item added to the collection in the same update shows in the
/// input.
#[browser_test]
pub async fn externally_selected_added_item_shows_in_the_input(
    page: &Page<'_>,
) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_labelled(page, "Controlled fruit").await?;
    input.wait_for_prop("value", "Apple").await?;
    page.element("#test-cb-controlled-add")
        .await?
        .click()
        .await?;
    input.wait_for_prop("value", "Fig").await?;
    Ok(())
}

/// A controlled value selecting an item before it is added to the collection shows in the input
/// once it is.
#[browser_test]
pub async fn item_added_after_its_selection_shows_in_the_input(
    page: &Page<'_>,
) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_labelled(page, "Controlled fruit").await?;
    input.wait_for_prop("value", "Apple").await?;
    page.element("#test-cb-controlled-select-then-add")
        .await?
        .click()
        .await?;
    input.wait_for_prop("value", "Grape").await?;
    Ok(())
}

/// With the value and the items derived from one app signal (the previous item leaves as the new
/// one comes), the input follows every change and the button shows all options, unfiltered.
#[browser_test]
pub async fn value_and_items_derived_from_one_signal(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_labelled(page, "Logs of").await?;
    input.wait_for_prop("value", "Orchestration").await?;
    page.element("#test-cb-derived-1").await?.click().await?;
    input.wait_for_prop("value", "Process 1").await?;
    page.element("#test-cb-derived-2").await?.click().await?;
    input.wait_for_prop("value", "Process 2").await?;
    button_of(&input).await?.click().await?;
    expect_options(page, &["Orchestration", "Process 2"]).await?;
    Ok(())
}

/// A value changed by a timer, outside any event, shows in the input, and the next button press
/// opens the popover.
#[browser_test]
pub async fn value_changed_by_a_timer(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_labelled(page, "Controlled fruit").await?;
    page.element("#test-cb-timed-start").await?.click().await?;
    input.wait_for_prop("value", "Banana").await?;
    button_of(&input).await?.click().await?;
    page.element(LISTBOX).await?;
    Ok(())
}

/// A value written while an effect runs shows in the input, and the next button press opens the
/// popover.
#[browser_test]
pub async fn value_written_in_an_effect(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_labelled(page, "Controlled fruit").await?;
    page.element("#test-cb-timed-effect").await?.click().await?;
    input.wait_for_prop("value", "Cherry").await?;
    button_of(&input).await?.click().await?;
    page.element(LISTBOX).await?;
    Ok(())
}

/// The popover of a combo box in a modal dialog stays interactive although it is portaled
/// outside the modal: it is not inert, and clicking an option selects it.
#[browser_test]
pub async fn popover_in_a_modal_stays_interactive(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-cb-modal-open").await?.click().await?;
    page.element("[role=dialog] [role=combobox]").await?;
    page.element("[role=dialog] button").await?.click().await?;
    expect_options(page, &["Apple", "Banana", "Cherry", "Durian", "Elderberry"]).await?;
    let option = page.element(role(AriaRole::Option).text("Durian")).await?;
    assert_that!(option.is_within("[inert]").await?).is_false();
    option.click().await?;
    page.element("#test-cb-modal-value")
        .await?
        .wait_for_inner_text("Durian")
        .await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count("[role=dialog]", 0).await?;
    Ok(())
}

/// The button counts as pressed while the popover is open ("should apply isPressed state to
/// button when expanded").
#[browser_test]
pub async fn button_is_pressed_while_open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let button = button_of(&input_labelled(page, "Fruit").await?).await?;
    assert_that!(button)
        .attribute("data-pressed")
        .await
        .is_none();
    button.click().await?;
    page.element(LISTBOX).await?;
    button.wait_for_attr("data-pressed", Some("true")).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(LISTBOX, 0).await?;
    button.wait_for_attr("data-pressed", None).await?;
    Ok(())
}

/// Clicking the button of the open combo box closes it, and clicking it again reopens it ("should
/// close the combobox when clicking on the button, and it should reopen if clicked again").
#[browser_test]
pub async fn button_toggles_the_popover(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_labelled(page, "Fruit").await?;
    let button = button_of(&input).await?;
    button.click().await?;
    page.element(LISTBOX).await?;
    button.click().await?;
    page.wait_for_count(LISTBOX, 0).await?;
    button.click().await?;
    page.element(LISTBOX).await?;
    page.wait_for_focus(&input).await?;
    Ok(())
}

/// Clicking the input of the open combo box keeps it open ("should not close the combobox when
/// clicking on the input").
#[browser_test]
pub async fn clicking_the_input_keeps_it_open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_labelled(page, "Fruit").await?;
    button_of(&input).await?.click().await?;
    page.element(LISTBOX).await?;
    input.click().await?;
    page.count_stays(LISTBOX, 1, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// ArrowUp opens the options with the last one focused ("opens the menu on up arrow press").
#[browser_test]
pub async fn arrow_up_opens_on_the_last_option(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_labelled(page, "Fruit").await?;
    input.click().await?;
    input.send_keys(Key::Up).await?;
    expect_options(page, &["Apple", "Banana", "Cherry", "Durian", "Elderberry"]).await?;
    expect_virtual_focus(page, "Elderberry").await?;
    Ok(())
}

/// Picking the selected option again resets the edited text to the option's ("resets input text
/// if reselecting a selected option with click").
#[browser_test]
pub async fn picking_the_selected_option_resets_the_text(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    select(page, "Banana").await?;
    let input = input_labelled(page, "Fruit").await?;
    input.send_keys(Key::Backspace).await?;
    input.wait_for_prop("value", "Banan").await?;
    page.element(role(AriaRole::Option).text("Banana"))
        .await?
        .click()
        .await?;
    input.wait_for_prop("value", "Banana").await?;
    page.wait_for_count(LISTBOX, 0).await?;
    page.element("#test-cb-value")
        .await?
        .inner_text_stays("Banana", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Tab selects the virtually focused option, closes the popover and moves focus on ("closes and
/// commits selection on tab").
#[browser_test]
pub async fn tab_commits_the_focused_option(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_labelled(page, "Fruit").await?;
    input.click().await?;
    input.send_keys("an").await?;
    expect_options(page, &["Banana", "Durian"]).await?;
    input.send_keys(Key::Down).await?;
    expect_virtual_focus(page, "Banana").await?;
    input.send_keys(Key::Tab).await?;
    page.element("#test-cb-value")
        .await?
        .wait_for_inner_text("Banana")
        .await?;
    page.wait_for_count(LISTBOX, 0).await?;
    page.wait_for_focus(&page.element("#test-cb-after").await?)
        .await?;
    Ok(())
}

/// A read-only combo box opens neither on focus nor by its button, and its text can't change
/// ("should not open the menu when isReadOnly", "should support read-only state").
#[browser_test]
pub async fn read_only(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_labelled(page, "Read-only fruit").await?;
    assert_that!(input).has_attribute("readonly").await;
    input.click().await?;
    button_of(&input).await?.click().await?;
    input.send_keys(Key::Down).await?;
    page.count_stays(LISTBOX, 0, std::time::Duration::from_millis(100))
        .await?;
    assert_that!(input)
        .property("value")
        .await
        .is_equal_to(Some("Apple".to_owned()));
    Ok(())
}

/// A disabled combo box can't be focused or opened: its input and button are disabled.
#[browser_test]
pub async fn disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_labelled(page, "Disabled fruit").await?;
    let button = button_of(&input).await?;
    assert_that!(input).enabled().await.is_false();
    assert_that!(button).enabled().await.is_false();
    button.click().await?;
    page.count_stays(LISTBOX, 0, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// The open popover closes when the page scrolls, but not when the input scrolls its text ("should
/// close on scroll", "should not close on input scrolling for cursor placement").
#[browser_test]
pub async fn closes_when_the_page_scrolls(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_labelled(page, "Fruit").await?;
    button_of(&input).await?.click().await?;
    page.element(LISTBOX).await?;
    input
        .dispatch(SyntheticEvent::plain(EventKind::Scroll).bubbles(false))
        .await?;
    page.count_stays(LISTBOX, 1, std::time::Duration::from_millis(100))
        .await?;
    page.element("body")
        .await?
        .dispatch(SyntheticEvent::plain(EventKind::Scroll).bubbles(false))
        .await?;
    page.wait_for_count(LISTBOX, 0).await?;
    Ok(())
}

/// The popover is as wide as the input and the button together.
#[browser_test]
pub async fn popover_spans_the_input_and_the_button(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_labelled(page, "Fruit").await?;
    let button = button_of(&input).await?;
    button.click().await?;
    let popover = page.element(".leptonic-ComboBoxPopover").await?;
    let (input_rect, button_rect) = (input.client_rect().await?, button.client_rect().await?);
    let expected = input_rect.right.max(button_rect.right) - input_rect.left.min(button_rect.left);
    assert_that!(|| popover.style_property("--trigger-width"))
        .eventually_ok()
        .matches(eq(format!("{expected}px")))
        .await;
    Ok(())
}

/// A label, input and description inside the popover don't belong to the combo box: the label
/// labels nothing, the input renders nothing, and the combo box isn't described by the text
/// ("should clear contexts inside popover").
#[browser_test]
pub async fn popover_content_isnt_part_of_the_combo_box(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_labelled(page, "Context fruit").await?;
    button_of(&input).await?.click().await?;
    let popover = page.element(".test-cb-contexts-popover").await?;
    popover.element(role(AriaRole::Listbox)).await?;
    let label = popover.element(css("label").text("Hello")).await?;
    assert_that!(label).attribute("for").await.is_none();
    assert_that!(popover.count("input").await?).is_equal_to(0);
    assert_that!(input)
        .attribute("aria-describedby")
        .await
        .is_none();
    let combobox_count = page.count("[role=combobox]").await?;
    let label_count = page.count("label").await?;
    assert_that!(combobox_count).is_equal_to(label_count - 1);
    crate::fixtures::take_warnings(page, "An <Input> needs a field", 1).await?;
    crate::fixtures::take_warnings(page, "A <Description> describes nothing", 1).await?;
    Ok(())
}

/// A combo box filtered by the app opens without options and shows them once loaded; when a
/// later result is empty it closes, and reopens when results arrive again ("should re-open the
/// menu with useAsyncList after an empty async result then backspace").
#[browser_test]
pub async fn server_filtered_options_reopen(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_labelled(page, "Character").await?;
    button_of(&input).await?.click().await?;
    expect_options(
        page,
        &[
            "Luke Skywalker",
            "Leia Organa",
            "Han Solo",
            "Lando Calrissian",
        ],
    )
    .await?;
    input.send_keys("luk").await?;
    expect_options(page, &["Luke Skywalker"]).await?;
    input.send_keys("a").await?;
    page.wait_for_count(LISTBOX, 0).await?;
    input.send_keys(Key::Backspace).await?;
    expect_options(page, &["Luke Skywalker"]).await?;
    Ok(())
}

/// A combo box selecting several options lists the selected ones in its `ComboBoxValue`, else
/// shows its placeholder ("should support multiple selection").
#[browser_test]
pub async fn combo_box_value_lists_the_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_labelled(page, "Fruits").await?;
    let value = page.element(".test-cb-multiple-value").await?;
    assert_that!(value)
        .inner_text()
        .await
        .is_equal_to("No fruits selected");
    assert_that!(value)
        .attribute("data-placeholder")
        .await
        .is_equal_to(Some("true".to_owned()));
    input.click().await?;
    input.send_keys("an").await?;
    page.element(role(AriaRole::Option).text("Banana"))
        .await?
        .click()
        .await?;
    value.wait_for_inner_text("Banana").await?;
    page.element(role(AriaRole::Option).text("Durian"))
        .await?
        .click()
        .await?;
    value.wait_for_inner_text("Banana and Durian").await?;
    assert_that!(value)
        .attribute("data-placeholder")
        .await
        .is_none();
    Ok(())
}

/// ArrowLeft and ArrowRight move the text cursor, so the virtually focused option loses focus
/// (`aria-activedescendant` goes); ArrowDown focuses an option again ("clears
/// aria-activedescendant when user presses left/right arrow (NVDA fix)").
#[browser_test]
pub async fn left_and_right_clear_the_virtual_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_labelled(page, "Fruit").await?;
    input.click().await?;
    input.send_keys("Dur").await?;
    expect_options(page, &["Durian"]).await?;
    for key in [Key::Left, Key::Right] {
        input.send_keys(Key::Down).await?;
        expect_virtual_focus(page, "Durian").await?;
        input.send_keys(key).await?;
        input.wait_for_attr("aria-activedescendant", None).await?;
    }
    Ok(())
}

/// Escape reverts the combo box without preventing the key's default action (react-aria's
/// `useComboBox` returns no `shouldPreventDefault` for it).
#[browser_test]
pub async fn escape_doesnt_prevent_the_default(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_labelled(page, "Fruit").await?;
    input.click().await?;
    input.send_keys("an").await?;
    page.element(LISTBOX).await?;
    let escape = input
        .dispatch(SyntheticEvent::keyboard(KeyKind::Down, "Escape"))
        .await?;
    assert_that!(escape.default_prevented).is_false();
    page.wait_for_count(LISTBOX, 0).await?;
    input.wait_for_prop("value", "").await?;
    Ok(())
}

/// Held arrow keys keep moving the virtual focus: auto-repeated ArrowDown presses count
/// (react-aria: `allowRepeats` for the arrow keys).
#[browser_test]
pub async fn held_arrow_keys_repeat(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_labelled(page, "Fruit").await?;
    input.click().await?;
    input.send_keys(Key::Down).await?;
    expect_virtual_focus(page, "Apple").await?;
    input
        .dispatch(SyntheticEvent::keyboard(KeyKind::Down, "ArrowDown").repeat(true))
        .await?;
    expect_virtual_focus(page, "Banana").await?;
    Ok(())
}

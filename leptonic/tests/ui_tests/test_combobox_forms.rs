// Upstream: react-aria-components/test/ComboBox.test.js @ 99e6102368
// Upstream: react-stately/test/combobox/useComboBoxState.test.js @ 99e6102368
//! The combo boxes of `/atoms/combobox-forms` with custom values: committing typed text on blur,
//! Enter and Escape keeps it and clears the selection (react-stately's `commitCustomValue`), and
//! the text is submitted with the form.
//!
//! "should support validation errors" (native validation reaching the combo box's input), and
//! ARIA validation with `validate`.
//!
//! "should support multiple selection", "should support deselection if multiple selection is
//! enabled", "should support isRequired with multiple selection", "should support formValue".
//!
//! `menuTrigger` focus and manual (react-stately's `useComboBoxState`: `open` with a trigger,
//! showing all items), and `on_open_change` with what opened the popover.
//!
//! "should support filtering sections", disabled keys, Enter without a focused option, and the
//! option count announcement.
use assertr::prelude::*;
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::{
    pages::{ElementActions, Page, PageActions, role},
    polling::wait_for,
};

const LISTBOX: &str = "[role=listbox]";

/// The combo box input inside `container`.
async fn input_in(page: &Page<'_>, container: &str) -> Result<WebElement, Report> {
    page.element(format!("{container} [role=combobox]")).await
}

/// Waits until the open listbox shows exactly `expected`.
async fn expect_options(page: &Page<'_>, expected: &[&str]) -> Result<(), Report> {
    wait_for("the options")
        .observing(|| page.inner_texts("[role=listbox] [role=option]"))
        .to_be_equal_to(expected)
        .await?;
    Ok(())
}

/// Opens the popover of the combo box in `container` with its button.
async fn open_with_button(page: &Page<'_>, container: &str) -> Result<(), Report> {
    page.element(format!("{container} button[aria-haspopup]"))
        .await?
        .click()
        .await?;
    page.element(LISTBOX).await?;
    Ok(())
}

/// The text of the input's active descendant (virtual focus), `""` without one.
async fn active_descendant_text(page: &Page<'_>, input: &WebElement) -> Result<String, Report> {
    match input.attr("aria-activedescendant").await? {
        Some(id) if !id.is_empty() => page.element(format!("#{id}")).await?.inner_text().await,
        _ => Ok(String::new()),
    }
}

/// Takes focus away from the focused element, in the next animation frame: until then, a press
/// that keeps focus where it is (an option of the popover) swallows blurs of the focused element
/// (react-aria's `preventFocus`), and a user can't blur within the frame of a click.
async fn blur_in_the_next_frame(page: &Page<'_>) -> Result<(), Report> {
    page.driver
        .execute_async(
            "const done = arguments[0];
             requestAnimationFrame(() => { document.activeElement.blur(); done(); });",
            vec![],
        )
        .await?;
    Ok(())
}

/// The names and values of the hidden inputs inside `container`.
async fn hidden_values(page: &Page<'_>, container: &str) -> Result<Vec<(String, String)>, Report> {
    let mut values = Vec::new();
    for input in page
        .elements(format!("{container} input[type=hidden]"))
        .await?
    {
        values.push((
            input.attr("name").await?.unwrap_or_default(),
            input.value().await?.unwrap_or_default(),
        ));
    }
    Ok(values)
}

/// Selecting an option with the keyboard (Kangaroo, key 3).
pub async fn select_an_option(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/combobox-forms").await?;
    let input = input_in(page, "#cbf-custom").await?;
    input.click().await?;
    input.send_keys("Kan").await?;
    expect_options(page, &["Kangaroo"]).await?;
    input.send_keys(Key::Down).await?;
    input.send_keys(Key::Enter).await?;
    page.element("#cbf-custom-changes")
        .await?
        .wait_for_inner_text("[3]")
        .await?;
    input.wait_for_prop("value", "Kangaroo").await?;
    Ok(())
}

/// Typed text matching no option is kept when focus leaves and clears the selection; the form
/// submits the text (`allows_custom_value` submits the text, not the key).
pub async fn custom_text_on_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/combobox-forms").await?;
    select_an_option(page).await?;
    let input = input_in(page, "#cbf-custom").await?;
    input.send_keys(Key::Control + "a").await?;
    input.send_keys("Wombat").await?;
    page.wait_for_count(LISTBOX, 0).await?;
    page.send_keys(Key::Tab).await?;
    page.element("#cbf-custom-changes")
        .await?
        .wait_for_inner_text("[3]|[]")
        .await?;
    input.prop_stays("value", "Wombat").await?;
    assert_that!(
        page.element("#cbf-custom")
            .await?
            .form_values("animal")
            .await?
    )
    .contains_exactly(["Wombat"]);
    Ok(())
}

/// Escape without a selection keeps the custom text (react-stately `revert`).
pub async fn escape_keeps_custom_text(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/combobox-forms").await?;
    let input = input_in(page, "#cbf-custom").await?;
    input.click().await?;
    input.send_keys("x").await?;
    input.send_keys(Key::Escape).await?;
    input.prop_stays("value", "x").await?;
    Ok(())
}

/// Enter commits custom text too; the value (no selection) doesn't change.
pub async fn enter_commits_custom_text(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/combobox-forms").await?;
    let input = input_in(page, "#cbf-custom").await?;
    input.send_keys(Key::Control + "a").await?;
    input.send_keys("Emu").await?;
    input.send_keys(Key::Enter).await?;
    input.prop_stays("value", "Emu").await?;
    page.element("#cbf-custom-changes")
        .await?
        .inner_text_stays("")
        .await?;
    Ok(())
}

/// Native: `required` on the input, the error once the form is checked; it stays until the
/// value is committed (focus leaves).
pub async fn native_validation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/combobox-forms").await?;
    let input = input_in(page, "#cbf-required").await?;
    let root = page.element("#cbf-required .leptonic-ComboBox").await?;
    assert_that!(input.attr("required").await?).is_some();
    assert_that!(input.attr("aria-required").await?).is_none();
    assert_that!(input.attr("aria-describedby").await?).is_none();
    assert_that!(input.is_valid().await?).is_false();
    assert_that!(root.attr("data-invalid").await?).is_none();
    assert_that!(root.attr("data-required").await?)
        .get_some()
        .is_equal_to("true");

    assert_that!(
        page.element("#cbf-required")
            .await?
            .check_validity()
            .await?
    )
    .is_false();
    page.wait_for_focus(&input).await?;
    root.wait_for_attr("data-invalid", Some("true")).await?;
    // The browser's validation message.
    assert_that!(input.referenced_text("aria-describedby").await?).is_not_blank();

    input.send_keys("C").await?;
    page.element(role("option").text("Cat"))
        .await?
        .click()
        .await?;
    input.wait_for_prop("value", "Cat").await?;
    assert_that!(input.is_valid().await?).is_true();
    assert_that!(input.attr("aria-describedby").await?).is_some();
    page.send_keys(Key::Tab).await?;
    input.wait_for_attr("aria-describedby", None).await?;
    root.wait_for_attr("data-invalid", None).await?;
    Ok(())
}

/// ARIA: `validate` runs on the value, its message shows right away.
pub async fn aria_validation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/combobox-forms").await?;
    let input = input_in(page, "#cbf-validate").await?;
    input.click().await?;
    input.send_keys("Do").await?;
    page.element(role("option").text("Dog"))
        .await?
        .click()
        .await?;
    input.wait_for_prop("value", "Dog").await?;
    input.wait_for_attr("aria-invalid", Some("true")).await?;
    wait_for("the error")
        .observing(|| input.referenced_text("aria-describedby"))
        .to_be_equal_to("Dogs are not allowed")
        .await?;
    input.send_keys(Key::Control + "a").await?;
    input.send_keys("Ca").await?;
    page.element(role("option").text("Cat"))
        .await?
        .click()
        .await?;
    input.wait_for_attr("aria-invalid", None).await?;
    Ok(())
}

/// Multiple selection: the popover stays open, the input stays empty, the form submits every
/// key; pressing a selected option deselects it.
pub async fn multiple_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/combobox-forms").await?;
    let input = input_in(page, "#cbf-multiple").await?;
    let changes = page.element("#cbf-multiple-changes").await?;
    open_with_button(page, "#cbf-multiple").await?;
    let listbox = page.element(LISTBOX).await?;
    assert_that!(listbox.attr("aria-multiselectable").await?)
        .get_some()
        .is_equal_to("true");
    expect_options(page, &["Cat", "Dog", "Kangaroo"]).await?;
    let cat = page.element(role("option").text("Cat")).await?;
    cat.click().await?;
    cat.wait_for_attr("aria-selected", Some("true")).await?;
    page.element(role("option").text("Dog"))
        .await?
        .click()
        .await?;
    changes.wait_for_inner_text("[1]|[1,2]").await?;
    assert_that!(page.count(LISTBOX).await?).is_equal_to(1);
    assert_that!(input.value().await?).get_some().is_empty();
    assert_that!(
        page.element("#cbf-multiple")
            .await?
            .form_values("animals")
            .await?
    )
    .contains_exactly(["1", "2"]);
    cat.click().await?;
    changes.wait_for_inner_text("[1]|[1,2]|[2]").await?;
    input.send_keys(Key::Escape).await?;
    page.wait_for_count(LISTBOX, 0).await?;
    Ok(())
}

/// Form reset restores the (empty) default.
pub async fn multiple_form_reset(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/combobox-forms").await?;
    let form = page.element("#cbf-multiple").await?;
    open_with_button(page, "#cbf-multiple").await?;
    page.element(role("option").text("Cat"))
        .await?
        .click()
        .await?;
    wait_for("the submitted animals")
        .observing(|| form.form_values("animals"))
        .to_be_equal_to(vec!["1".to_owned()])
        .await?;
    input_in(page, "#cbf-multiple")
        .await?
        .send_keys(Key::Escape)
        .await?;
    page.wait_for_count(LISTBOX, 0).await?;
    page.element("#cbf-multiple-reset").await?.click().await?;
    wait_for("the submitted animals")
        .observing(|| form.form_values("animals"))
        .to_be_equal_to(vec![String::new()])
        .await?;
    Ok(())
}

/// Required with multiple selection: required only while nothing is selected.
pub async fn required_with_multiple_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/combobox-forms").await?;
    let input = input_in(page, "#cbf-multiple-required").await?;
    let root = page
        .element("#cbf-multiple-required .leptonic-ComboBox")
        .await?;
    assert_that!(input.attr("required").await?).is_some();
    assert_that!(input.is_valid().await?).is_false();
    assert_that!(
        page.element("#cbf-multiple-required")
            .await?
            .check_validity()
            .await?
    )
    .is_false();
    root.wait_for_attr("data-invalid", Some("true")).await?;
    open_with_button(page, "#cbf-multiple-required").await?;
    page.element(role("option").text("Cat"))
        .await?
        .click()
        .await?;
    blur_in_the_next_frame(page).await?;
    input.wait_for_attr("required", None).await?;
    assert_that!(input.is_valid().await?).is_true();
    root.wait_for_attr("data-invalid", None).await?;
    assert_that!(hidden_values(page, "#cbf-multiple-required").await?)
        .contains_exactly([("required-animals".to_owned(), "1".to_owned())]);
    open_with_button(page, "#cbf-multiple-required").await?;
    page.element(role("option").text("Cat"))
        .await?
        .click()
        .await?;
    blur_in_the_next_frame(page).await?;
    input.wait_for_attr("required", Some("true")).await?;
    assert_that!(input.is_valid().await?).is_false();
    assert_that!(hidden_values(page, "#cbf-multiple-required").await?)
        .contains_exactly([("required-animals".to_owned(), String::new())]);
    Ok(())
}

/// `form_value`: the key in a hidden input (the input has no name), or the text.
pub async fn form_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/combobox-forms").await?;
    let input = input_in(page, "#cbf-key").await?;
    assert_that!(input.attr("name").await?).is_none();
    assert_that!(input.value().await?)
        .get_some()
        .is_equal_to("Dog");
    assert_that!(hidden_values(page, "#cbf-key").await?)
        .contains_exactly([("key-animal".to_owned(), "2".to_owned())]);
    let input = input_in(page, "#cbf-text").await?;
    assert_that!(input.attr("name").await?)
        .get_some()
        .is_equal_to("text-animal");
    assert_that!(hidden_values(page, "#cbf-text").await?).is_empty();
    assert_that!(
        page.element("#cbf-text")
            .await?
            .form_values("text-animal")
            .await?
    )
    .contains_exactly(["Dog"]);
    Ok(())
}

/// Focus opens the popover with all options, though the input holds "Do"; typing filters again.
pub async fn focus_trigger(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/combobox-forms").await?;
    let input = input_in(page, "#cbf-focus").await?;
    let open_changes = page.element("#cbf-focus-open").await?;
    input.click().await?;
    expect_options(page, &["Cat", "Dog", "Kangaroo"]).await?;
    open_changes.wait_for_inner_text("true:Some(Focus)").await?;
    input.send_keys("g").await?;
    expect_options(page, &["Dog"]).await?;
    input.send_keys(Key::Escape).await?;
    page.wait_for_count(LISTBOX, 0).await?;
    open_changes
        .wait_for_inner_text("true:Some(Focus)|false:None")
        .await?;
    Ok(())
}

/// Manual: typing doesn't open, ArrowDown does (with all options).
pub async fn manual_trigger(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/combobox-forms").await?;
    let input = input_in(page, "#cbf-manual").await?;
    input.click().await?;
    input.send_keys("a").await?;
    page.count_stays(LISTBOX, 0).await?;
    input.send_keys(Key::Down).await?;
    expect_options(page, &["Cat", "Dog", "Kangaroo"]).await?;
    page.element("#cbf-manual-open")
        .await?
        .wait_for_inner_text("true:Some(Manual)")
        .await?;
    input.send_keys(Key::Escape).await?;
    page.wait_for_count(LISTBOX, 0).await?;
    Ok(())
}

/// One group per section with matches, named by its heading; sections without matches are gone.
pub async fn filtering_sections(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/combobox-forms").await?;
    let input = input_in(page, "#cbf-sections").await?;
    input.click().await?;
    input.send_keys("o").await?;
    expect_options(page, &["Dog", "Owl", "Parrot"]).await?;
    let mut names = Vec::new();
    for group in page.elements("[role=listbox] [role=group]").await? {
        names.push(group.referenced_text("aria-labelledby").await?);
    }
    assert_that!(names).contains_exactly(["Animals", "Birds"]);
    input.send_keys("w").await?;
    expect_options(page, &["Owl"]).await?;
    assert_that!(page.count("[role=listbox] [role=group]").await?).is_equal_to(1);
    Ok(())
}

/// The disabled option is skipped by the keyboard.
pub async fn disabled_option_is_skipped(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/combobox-forms").await?;
    let input = input_in(page, "#cbf-sections").await?;
    input.click().await?;
    input.send_keys("o").await?;
    expect_options(page, &["Dog", "Owl", "Parrot"]).await?;
    let dog = page.element(role("option").text("Dog")).await?;
    assert_that!(dog.attr("aria-disabled").await?)
        .get_some()
        .is_equal_to("true");
    input.send_keys(Key::Down).await?;
    wait_for("the active descendant's text")
        .observing(|| active_descendant_text(page, &input))
        .to_be_equal_to("Owl")
        .await?;
    input.send_keys(Key::Enter).await?;
    input.wait_for_prop("value", "Owl").await?;
    page.wait_for_count(LISTBOX, 0).await?;
    Ok(())
}

/// Enter with the popover closed submits the form; with the popover open but no focused option
/// it only commits (reverting the text). The number of options is announced when the popover
/// opens without a focused option.
pub async fn enter_without_a_focused_option(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/combobox-forms").await?;
    let input = input_in(page, "#cbf-submit").await?;
    let submits = page.element("#cbf-submits").await?;
    input.click().await?;
    input.send_keys(Key::Enter).await?;
    submits.wait_for_inner_text("1").await?;
    input.send_keys("Ca").await?;
    page.element(LISTBOX).await?;
    page.element("[data-live-announcer] [aria-live=assertive]")
        .await?
        .wait_for_inner_text("1 option available.")
        .await?;
    input.send_keys(Key::Enter).await?;
    page.wait_for_count(LISTBOX, 0).await?;
    input.wait_for_prop("value", "").await?;
    submits.inner_text_stays("1").await?;
    Ok(())
}

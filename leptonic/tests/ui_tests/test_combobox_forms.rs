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
use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::Report;

use crate::pages::{ElementActions, Page, css, role};

const LISTBOX: &str = "[role=listbox]";
const PATH: &str = "/atoms/combobox-forms";

/// The combo box input inside `container`.
async fn input_in(page: &Page<'_>, container: &str) -> Result<WebElement, Report> {
    page.element(format!("{container} [role=combobox]")).await
}

/// Waits until the open listbox shows exactly `expected`.
async fn expect_options(page: &Page<'_>, expected: &[&str]) -> Result<(), Report> {
    assert_that!(|| page.inner_texts("[role=listbox] [role=option]"))
        .eventually_ok()
        .matches(eq(expected))
        .await;
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
    page.low_level()
        .eval_async::<()>(
            "const done = arguments[arguments.length - 1];
         requestAnimationFrame(() => { document.activeElement.blur(); done(); });",
            vec![],
        )
        .await
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

/// Selects Kangaroo (key 3) in the custom-value combo box with the keyboard: the popover closes.
async fn select_kangaroo(page: &Page<'_>) -> Result<(), Report> {
    let input = input_in(page, "#cbf-custom").await?;
    input.click().await?;
    input.type_keys("Kan").await?;
    expect_options(page, &["Kangaroo"]).await?;
    input.type_keys(Key::Down).await?;
    input.type_keys(Key::Enter).await?;
    page.element("#cbf-custom-changes")
        .await?
        .wait_for_inner_text("[3]")
        .await?;
    input.wait_for_prop("value", "Kangaroo").await?;
    page.wait_for_count(LISTBOX, 0).await?;
    Ok(())
}

/// Typing "Kan", ArrowDown and Enter select Kangaroo, fill the input with its text and close the
/// popover ("should support selecting an option via keyboard").
#[browser_test]
pub async fn select_an_option(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    select_kangaroo(page).await?;
    Ok(())
}

/// Typed text matching no option is kept when focus leaves and clears the selection; the form
/// submits the text (`allows_custom_value` submits the text, not the key).
#[browser_test]
pub async fn custom_text_on_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    select_kangaroo(page).await?;
    let input = input_in(page, "#cbf-custom").await?;
    input
        .type_keys(page.primary_modifier().await? + "a")
        .await?;
    // Text matching options opens the popover, text matching none closes it.
    input.type_keys("Ca").await?;
    expect_options(page, &["Cat"]).await?;
    input
        .type_keys(page.primary_modifier().await? + "a")
        .await?;
    input.type_keys("Wombat").await?;
    page.wait_for_count(LISTBOX, 0).await?;
    page.send_keys(Key::Tab).await?;
    page.element("#cbf-custom-changes")
        .await?
        .wait_for_inner_text("[3]|[]")
        .await?;
    input
        .prop_stays("value", "Wombat", std::time::Duration::from_millis(100))
        .await?;
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
#[browser_test]
pub async fn escape_keeps_custom_text(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_in(page, "#cbf-custom").await?;
    input.click().await?;
    input.type_keys("x").await?;
    input.type_keys(Key::Escape).await?;
    input
        .prop_stays("value", "x", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Enter in the open popover without a focused option commits the custom text: the popover
/// closes, the text stays, the selection clears.
#[browser_test]
pub async fn enter_commits_custom_text(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    select_kangaroo(page).await?;
    let input = input_in(page, "#cbf-custom").await?;
    input
        .type_keys(page.primary_modifier().await? + "a")
        .await?;
    input.type_keys("Ca").await?;
    expect_options(page, &["Cat"]).await?;
    assert_that!(active_descendant_text(page, &input).await?).is_empty();
    input.type_keys(Key::Enter).await?;
    page.wait_for_count(LISTBOX, 0).await?;
    let changes = page.element("#cbf-custom-changes").await?;
    changes.wait_for_inner_text("[3]|[]").await?;
    changes
        .inner_text_stays("[3]|[]", std::time::Duration::from_millis(100))
        .await?;
    input
        .prop_stays("value", "Ca", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A required combo box shows no error until the form is checked, then the browser's message;
/// selecting an option makes it valid, and the error goes once focus leaves ("should support
/// validation errors").
#[browser_test]
pub async fn native_validation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_in(page, "#cbf-required").await?;
    let root = page.element("#cbf-required .leptonic-ComboBox").await?;
    assert_that!(input).has_attribute("required").await;
    assert_that!(input)
        .attribute("aria-required")
        .await
        .is_none();
    assert_that!(input)
        .attribute("aria-describedby")
        .await
        .is_none();
    assert_that!(input.is_valid().await?).is_false();
    assert_that!(root).attribute("data-invalid").await.is_none();
    assert_that!(root)
        .has_attribute("data-required")
        .await
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
    let message = assert_that!(input)
        .property("validationMessage")
        .await
        .some()
        .is_not_blank()
        .actual()
        .clone();
    assert_that!(|| input.accessible_description())
        .eventually_ok()
        .matches(eq(message))
        .await;

    input.type_keys("C").await?;
    page.element(role(AriaRole::Option).text("Cat"))
        .await?
        .click()
        .await?;
    input.wait_for_prop("value", "Cat").await?;
    assert_that!(input.is_valid().await?).is_true();
    assert_that!(input).has_attribute("aria-describedby").await;
    page.send_keys(Key::Tab).await?;
    input.wait_for_attr("aria-describedby", None).await?;
    root.wait_for_attr("data-invalid", None).await?;
    Ok(())
}

/// Selecting Dog makes the input invalid right away with the message of `validate` ("Dogs are
/// not allowed"); selecting Cat makes it valid again.
#[browser_test]
pub async fn aria_validation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_in(page, "#cbf-validate").await?;
    input.click().await?;
    input.type_keys("Do").await?;
    page.element(role(AriaRole::Option).text("Dog"))
        .await?
        .click()
        .await?;
    input.wait_for_prop("value", "Dog").await?;
    input.wait_for_attr("aria-invalid", Some("true")).await?;
    assert_that!(|| input.accessible_description())
        .eventually_ok()
        .matches(eq("Dogs are not allowed"))
        .await;
    input
        .type_keys(page.primary_modifier().await? + "a")
        .await?;
    input.type_keys("Ca").await?;
    page.element(role(AriaRole::Option).text("Cat"))
        .await?
        .click()
        .await?;
    input.wait_for_attr("aria-invalid", None).await?;
    Ok(())
}

/// With multiple selection, pressing options selects them while the popover stays open and the
/// input stays empty, and the form submits every key; pressing a selected option deselects it
/// ("should support multiple selection", "should support deselection if multiple selection is
/// enabled").
#[browser_test]
pub async fn multiple_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_in(page, "#cbf-multiple").await?;
    let changes = page.element("#cbf-multiple-changes").await?;
    open_with_button(page, "#cbf-multiple").await?;
    let listbox = page.element(LISTBOX).await?;
    assert_that!(listbox)
        .has_attribute("aria-multiselectable")
        .await
        .is_equal_to("true");
    expect_options(page, &["Cat", "Dog", "Kangaroo"]).await?;
    let cat = page.element(role(AriaRole::Option).text("Cat")).await?;
    cat.click().await?;
    cat.wait_for_attr("aria-selected", Some("true")).await?;
    page.element(role(AriaRole::Option).text("Dog"))
        .await?
        .click()
        .await?;
    changes.wait_for_inner_text("[1]|[1,2]").await?;
    page.count_stays(LISTBOX, 1, std::time::Duration::from_millis(100))
        .await?;
    input
        .prop_stays("value", "", std::time::Duration::from_millis(100))
        .await?;
    assert_that!(
        page.element("#cbf-multiple")
            .await?
            .form_values("animals")
            .await?
    )
    .contains_exactly(["1", "2"]);
    cat.click().await?;
    changes.wait_for_inner_text("[1]|[1,2]|[2]").await?;
    input.type_keys(Key::Escape).await?;
    page.wait_for_count(LISTBOX, 0).await?;
    Ok(())
}

/// Resetting the form clears a multiple selection, so the form submits an empty value again
/// ("should support multiple selection").
#[browser_test]
pub async fn multiple_form_reset(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let form = page.element("#cbf-multiple").await?;
    open_with_button(page, "#cbf-multiple").await?;
    page.element(role(AriaRole::Option).text("Cat"))
        .await?
        .click()
        .await?;
    assert_that!(|| form.form_values("animals"))
        .eventually_ok()
        .matches(eq(vec!["1".to_owned()]))
        .await;
    input_in(page, "#cbf-multiple")
        .await?
        .type_keys(Key::Escape)
        .await?;
    page.wait_for_count(LISTBOX, 0).await?;
    page.element("#cbf-multiple-reset").await?.click().await?;
    assert_that!(|| form.form_values("animals"))
        .eventually_ok()
        .matches(eq(vec![String::new()]))
        .await;
    Ok(())
}

/// A required combo box with multiple selection is invalid while nothing is selected, valid once
/// an option is selected and invalid again after deselecting it ("should support isRequired with
/// multiple selection").
#[browser_test]
pub async fn required_with_multiple_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_in(page, "#cbf-multiple-required").await?;
    let root = page
        .element("#cbf-multiple-required .leptonic-ComboBox")
        .await?;
    assert_that!(input).has_attribute("required").await;
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
    page.element(role(AriaRole::Option).text("Cat"))
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
    page.element(role(AriaRole::Option).text("Cat"))
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

/// The form submits the selected key from a hidden input (the visible input has no name), or the
/// input's text with `ComboBoxFormValue::Text` ("should support formValue").
#[browser_test]
pub async fn form_value(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_in(page, "#cbf-key").await?;
    assert_that!(input).attribute("name").await.is_none();
    assert_that!(input)
        .property("value")
        .await
        .some()
        .is_equal_to("Dog");
    assert_that!(hidden_values(page, "#cbf-key").await?)
        .contains_exactly([("key-animal".to_owned(), "2".to_owned())]);
    let input = input_in(page, "#cbf-text").await?;
    assert_that!(input)
        .has_attribute("name")
        .await
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

/// With the focus menu trigger, focusing the input opens the popover with all options although
/// it holds "Do", reporting focus as the reason; typing filters again ("onOpenChange should
/// return the reason that open was called").
#[browser_test]
pub async fn focus_trigger(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_in(page, "#cbf-focus").await?;
    let open_changes = page.element("#cbf-focus-open").await?;
    input.click().await?;
    expect_options(page, &["Cat", "Dog", "Kangaroo"]).await?;
    open_changes.wait_for_inner_text("true:Some(Focus)").await?;
    input.type_keys("g").await?;
    expect_options(page, &["Dog"]).await?;
    input.type_keys(Key::Escape).await?;
    page.wait_for_count(LISTBOX, 0).await?;
    open_changes
        .wait_for_inner_text("true:Some(Focus)|false:None")
        .await?;
    Ok(())
}

/// With the manual menu trigger, typing doesn't open the popover but ArrowDown does, with all
/// options and the manual reason ("onOpenChange should return the reason that open was called").
#[browser_test]
pub async fn manual_trigger(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_in(page, "#cbf-manual").await?;
    input.click().await?;
    input.type_keys("a").await?;
    page.count_stays(LISTBOX, 0, std::time::Duration::from_millis(100))
        .await?;
    input.type_keys(Key::Down).await?;
    expect_options(page, &["Cat", "Dog", "Kangaroo"]).await?;
    page.element("#cbf-manual-open")
        .await?
        .wait_for_inner_text("true:Some(Manual)")
        .await?;
    input.type_keys(Key::Escape).await?;
    page.wait_for_count(LISTBOX, 0).await?;
    Ok(())
}

/// Filtering shows one group per section with matches, named by its heading, and hides sections
/// without matches ("should support filtering sections").
#[browser_test]
pub async fn filtering_sections(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_in(page, "#cbf-sections").await?;
    input.click().await?;
    input.type_keys("o").await?;
    expect_options(page, &["Dog", "Owl", "Parrot"]).await?;
    let mut names = Vec::new();
    for group in page.elements("[role=listbox] [role=group]").await? {
        names.push(group.accessible_name().await?);
    }
    assert_that!(names).contains_exactly(["Animals", "Birds"]);
    input.type_keys("w").await?;
    expect_options(page, &["Owl"]).await?;
    assert_that!(page.count("[role=listbox] [role=group]").await?).is_equal_to(1);
    Ok(())
}

/// ArrowDown skips the disabled option Dog and focuses Owl, which Enter then selects.
#[browser_test]
pub async fn disabled_option_is_skipped(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_in(page, "#cbf-sections").await?;
    input.click().await?;
    input.type_keys("o").await?;
    expect_options(page, &["Dog", "Owl", "Parrot"]).await?;
    let dog = page.element(role(AriaRole::Option).text("Dog")).await?;
    assert_that!(dog)
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");
    input.type_keys(Key::Down).await?;
    assert_that!(|| active_descendant_text(page, &input))
        .eventually_ok()
        .matches(eq("Owl"))
        .await;
    input.type_keys(Key::Enter).await?;
    input.wait_for_prop("value", "Owl").await?;
    page.wait_for_count(LISTBOX, 0).await?;
    Ok(())
}

/// Enter with the popover closed submits the form; with the popover open and no focused option it
/// only closes the popover and reverts the text. Opening announces the number of options.
#[browser_test]
pub async fn enter_without_a_focused_option(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_in(page, "#cbf-submit").await?;
    let submits = page.element("#cbf-submits").await?;
    input.click().await?;
    input.type_keys(Key::Enter).await?;
    submits.wait_for_inner_text("1").await?;
    input.type_keys("Ca").await?;
    page.element(LISTBOX).await?;
    page.element("[data-live-announcer] [aria-live=assertive]")
        .await?
        .wait_for_inner_text("1 option available.")
        .await?;
    input.type_keys(Key::Enter).await?;
    page.wait_for_count(LISTBOX, 0).await?;
    input.wait_for_prop("value", "").await?;
    submits
        .inner_text_stays("1", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Resetting the form restores the default selection and its text ("should support form reset").
#[browser_test]
pub async fn single_selection_form_reset(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = input_in(page, "#cbf-key").await?;
    open_with_button(page, "#cbf-key").await?;
    page.element(role(AriaRole::Option).text("Cat"))
        .await?
        .click()
        .await?;
    input.wait_for_prop("value", "Cat").await?;
    assert_that!(|| hidden_values(page, "#cbf-key"))
        .eventually_ok()
        .matches(eq(vec![("key-animal".to_owned(), "1".to_owned())]))
        .await;
    page.element("#cbf-key").await?.reset().await?;
    input.wait_for_prop("value", "Dog").await?;
    assert_that!(|| hidden_values(page, "#cbf-key"))
        .eventually_ok()
        .matches(eq(vec![("key-animal".to_owned(), "2".to_owned())]))
        .await;
    Ok(())
}

/// Clicking a section's header keeps the popover open ("should not close the combobox when
/// clicking on a section header").
#[browser_test]
pub async fn clicking_a_section_header_keeps_it_open(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    open_with_button(page, "#cbf-sections").await?;
    page.element(css("[role=listbox] header").text("Animals"))
        .await?
        .click()
        .await?;
    page.settle().await?;
    page.count_stays(LISTBOX, 1, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Leaving the field after picking an option reports no second change ("only commits on blur if
/// the value changed").
#[browser_test]
pub async fn blur_after_picking_reports_no_change(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let changes = page.element("#cbf-custom-changes").await?;
    open_with_button(page, "#cbf-custom").await?;
    page.element(role(AriaRole::Option).text("Cat"))
        .await?
        .click()
        .await?;
    changes.wait_for_inner_text("[1]").await?;
    blur_in_the_next_frame(page).await?;
    page.settle().await?;
    changes
        .inner_text_stays("[1]", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

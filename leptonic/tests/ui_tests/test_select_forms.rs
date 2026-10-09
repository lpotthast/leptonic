// Upstream: react-aria-components/test/Select.test.js @ 99e6102368
// Upstream: react-aria/test/select/HiddenSelect.test.tsx @ 99e6102368
//! "should support multiple selection", "should support deselection if multiple selection is
//! enabled", "supports placeholder", and the open state bound to app state.
//!
//! "supports validation errors", "should not submit if required and selectedKey is null", "should
//! send disabled prop to the hidden field", the root's data attributes, and autofill (a `change`
//! of the hidden `<select>`).
//!
//! "shouldn't allow the user to open the select if there are no items", "should support empty
//! state", "should support multiple selection form integration with many items" (hidden inputs
//! instead of a `<select>`, with validation and form reset).
use assertr::{matchers::eq, prelude::*};
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions, role};

const LISTBOX: &str = "[role=listbox]";

/// The select trigger inside `container`.
async fn trigger_in(page: &Page<'_>, container: &str) -> Result<WebElement, Report> {
    page.element(format!("{container} [aria-haspopup=listbox]"))
        .await
}

/// "should support hover" on the trigger; the default placeholder.
pub async fn trigger_hover_and_placeholder(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/select-forms").await?;
    let trigger = trigger_in(page, "#sf-multiple").await?;
    trigger.hover().await?;
    trigger.wait_for_attr("data-hovered", Some("true")).await?;
    page.element("h1").await?.hover().await?;
    trigger.wait_for_attr("data-hovered", None).await?;
    assert_that!(trigger.inner_text().await?).is_equal_to("Select an item");
    Ok(())
}

/// Options toggle while the popover stays open; the trigger lists the selection, the form
/// submits every value; pressing a selected option deselects it.
pub async fn multiple_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/select-forms").await?;
    let trigger = trigger_in(page, "#sf-multiple").await?;
    let changes = page.element("#sf-multiple-changes").await?;
    trigger.click().await?;
    let listbox = page.element(LISTBOX).await?;
    assert_that!(listbox.attr("aria-multiselectable").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(page.inner_texts("[role=listbox] [role=option]").await?)
        .contains_exactly(["Cat", "Dog", "Kangaroo"]);
    let cat = page.element(role("option").text("Cat")).await?;
    cat.click().await?;
    page.element(role("option").text("Dog"))
        .await?
        .click()
        .await?;
    changes.wait_for_inner_text("[cat]|[cat,dog]").await?;
    trigger.wait_for_inner_text("Cat and Dog").await?;
    // Multiple selection keeps the popover open.
    assert_that!(page.count(LISTBOX).await?).is_equal_to(1);
    assert_that!(
        page.element("#sf-multiple")
            .await?
            .form_values("select")
            .await?
    )
    .contains_exactly(["cat", "dog"]);
    cat.click().await?;
    changes.wait_for_inner_text("[cat]|[cat,dog]|[dog]").await?;
    cat.wait_for_attr("aria-selected", Some("false")).await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(LISTBOX, 0).await?;
    Ok(())
}

/// The open state bound to app state: opening from outside, Escape writes it back.
pub async fn open_state_bound_to_app_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/select-forms").await?;
    let open_state = page.element("#sf-open-state").await?;
    page.element("#sf-open-toggle").await?.click().await?;
    page.element(LISTBOX).await?;
    open_state.wait_for_inner_text("true").await?;
    let root = page.element("#sf-open .leptonic-Select").await?;
    assert_that!(root.attr("data-open").await?)
        .get_some()
        .is_equal_to("true");
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(LISTBOX, 0).await?;
    open_state.wait_for_inner_text("false").await?;
    Ok(())
}

/// Native validation: the hidden select is required and labelled for autofill; validating the
/// form marks the select invalid, describes the trigger with the error and focuses it; picking
/// a value clears the error.
pub async fn native_validation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/select-forms").await?;
    let root = page.element("#sf-required .leptonic-Select").await?;
    let trigger = trigger_in(page, "#sf-required").await?;
    let select = page.element("#sf-required select").await?;
    assert_that!(select.attr("required").await?).is_some();
    assert_that!(trigger.attr("aria-describedby").await?).is_none();
    assert_that!(select.is_valid().await?).is_false();
    assert_that!(root.attr("data-invalid").await?).is_none();
    assert_that!(root.attr("data-required").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(
        page.count("#sf-required [aria-hidden=true] label select")
            .await?
    )
    .is_equal_to(1);

    assert_that!(page.element("#sf-required").await?.check_validity().await?).is_false();
    root.wait_for_attr("data-invalid", Some("true")).await?;
    page.wait_for_focus(&trigger).await?;
    // The browser's validation message.
    assert_that!(trigger.referenced_text("aria-describedby").await?).is_not_blank();
    // Focus within the select: `data-focused` (focused by the script).
    root.wait_for_attr("data-focused", Some("true")).await?;
    trigger.click().await?;
    page.element(role("option").text("Cat"))
        .await?
        .click()
        .await?;
    page.wait_for_count(LISTBOX, 0).await?;
    trigger.wait_for_attr("aria-describedby", None).await?;
    root.wait_for_attr("data-invalid", None).await?;
    Ok(())
}

/// A required select without a value blocks submission; with one, the form submits.
pub async fn required_blocks_submission(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/select-forms").await?;
    let trigger = trigger_in(page, "#sf-submit").await?;
    let submit = page.element("#sf-submit-button").await?;
    let submits = page.element("#sf-submits").await?;
    assert_that!(trigger.inner_text().await?).is_equal_to("Select an item");
    trigger.click().await?;
    page.element(role("option").text("Cat"))
        .await?
        .click()
        .await?;
    trigger.wait_for_inner_text("Cat").await?;
    submit.click().await?;
    submits.wait_for_inner_text("1").await?;
    page.element("#sf-submit-clear").await?.click().await?;
    trigger.wait_for_inner_text("Select an item").await?;
    submit.click().await?;
    submits.inner_text_stays("1").await?;
    assert_that!(page.element("#sf-submit select").await?.value().await?)
        .get_some()
        .is_empty();
    Ok(())
}

/// Disabled: the hidden select too, and the trigger doesn't open.
pub async fn disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/select-forms").await?;
    let select = page.element("#sf-disabled select").await?;
    assert_that!(select.is_enabled().await?).is_false();
    let trigger = trigger_in(page, "#sf-disabled").await?;
    assert_that!(trigger.is_enabled().await?).is_false();
    trigger.virtual_click().await?;
    page.count_stays(LISTBOX, 0).await?;
    Ok(())
}

/// Autofill picks an option of the hidden select (a `change` event).
pub async fn autofill(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/select-forms").await?;
    let trigger = trigger_in(page, "#sf-required").await?;
    let select = page.element("#sf-required select").await?;
    page.eval::<()>(
        "arguments[0].value = 'kangaroo';
         arguments[0].dispatchEvent(new Event('change', {bubbles: true}));",
        vec![select.to_json()?],
    )
    .await?;
    trigger.wait_for_inner_text("Kangaroo").await?;
    Ok(())
}

/// "shouldn't allow the user to open the select if there are no items".
pub async fn no_items(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/select-forms").await?;
    trigger_in(page, "#sf-empty").await?.click().await?;
    page.count_stays(LISTBOX, 0).await?;
    Ok(())
}

/// "should support empty state": with empty content allowed, it opens and shows it.
pub async fn empty_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/select-forms").await?;
    trigger_in(page, "#sf-empty-allowed").await?.click().await?;
    let listbox = page.element(LISTBOX).await?;
    assert_that!(listbox.attr("data-empty").await?)
        .get_some()
        .is_equal_to("true");
    let empty_option = page.element("[role=listbox] [role=option]").await?;
    assert_that!(empty_option.inner_text().await?).is_equal_to("No results");
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(LISTBOX, 0).await?;
    Ok(())
}

/// More than 300 options: hidden inputs instead of a `<select>`; the first one is required
/// (native validation blocks the submission and shows the error).
pub async fn many_items_validation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/select-forms").await?;
    assert_that!(page.count("#sf-many select").await?).is_equal_to(0);
    page.element("#sf-many-submit").await?.click().await?;
    let error = page.element("#sf-many .leptonic-FieldError").await?;
    assert_that!(error.inner_text().await?).is_not_blank();
    page.element("#sf-many-submits")
        .await?
        .inner_text_stays("0")
        .await?;
    Ok(())
}

/// Selecting with the keyboard fills the hidden inputs, the form submits and its reset clears
/// them.
pub async fn many_items_selection_and_reset(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/select-forms").await?;
    let trigger = trigger_in(page, "#sf-many").await?;
    let form = page.element("#sf-many").await?;
    // Open with the keyboard, which focuses the first option (a pointer resting over the
    // popover would focus the option under it: `should_focus_on_hover`).
    page.element("h1").await?.hover().await?;
    trigger.focus().await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&page.element(role("option").text("item0")).await?)
        .await?;
    page.send_keys(" ").await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&page.element(role("option").text("item1")).await?)
        .await?;
    page.send_keys(" ").await?;
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(LISTBOX, 0).await?;
    trigger.wait_for_inner_text("item0 and item1").await?;
    assert_that!(form.form_values("many").await?).contains_exactly(["0", "1"]);
    page.element("#sf-many-submit").await?.click().await?;
    page.element("#sf-many-submits")
        .await?
        .wait_for_inner_text("1")
        .await?;
    page.wait_for_count("#sf-many .leptonic-FieldError", 0)
        .await?;
    page.element("#sf-many-reset").await?.click().await?;
    assert_that!(|| form.form_values("many"))
        .eventually_ok()
        .matches(eq(vec![String::new()]))
        .await;
    trigger.wait_for_inner_text("Select an item").await?;
    Ok(())
}

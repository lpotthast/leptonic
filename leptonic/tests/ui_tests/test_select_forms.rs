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
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::Report;

use crate::pages::{ElementActions, Page, role};

const LISTBOX: &str = "[role=listbox]";
const PATH: &str = "/atoms/select-forms";

/// The select trigger inside `container`.
async fn trigger_in(page: &Page<'_>, container: &str) -> Result<WebElement, Report> {
    page.element(format!("{container} [aria-haspopup=listbox]"))
        .await
}

/// Hovering the trigger marks it `data-hovered` until the pointer leaves, and without a value it
/// shows the default placeholder "Select an item".
#[browser_test]
pub async fn trigger_hover_and_placeholder(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = trigger_in(page, "#sf-multiple").await?;
    trigger.hover().await?;
    trigger.wait_for_attr("data-hovered", Some("true")).await?;
    page.element("h1").await?.hover().await?;
    trigger.wait_for_attr("data-hovered", None).await?;
    assert_that!(trigger)
        .inner_text()
        .await
        .is_equal_to("Select an item");
    Ok(())
}

/// In a multi-select, pressing options toggles them while the popover stays open; the trigger
/// lists the selection and the form submits every value ("should support multiple selection",
/// "should support deselection if multiple selection is enabled").
#[browser_test]
pub async fn multiple_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = trigger_in(page, "#sf-multiple").await?;
    let changes = page.element("#sf-multiple-changes").await?;
    trigger.click().await?;
    let listbox = page.element(LISTBOX).await?;
    assert_that!(listbox)
        .has_attribute("aria-multiselectable")
        .await
        .is_equal_to("true");
    assert_that!(page.inner_texts("[role=listbox] [role=option]").await?)
        .contains_exactly(["Cat", "Dog", "Kangaroo"]);
    let cat = page
        .element(LISTBOX)
        .await?
        .element(role(AriaRole::Option).text("Cat"))
        .await?;
    cat.click().await?;
    page.element(LISTBOX)
        .await?
        .element(role(AriaRole::Option).text("Dog"))
        .await?
        .click()
        .await?;
    changes.wait_for_inner_text("[cat]|[cat,dog]").await?;
    trigger.wait_for_inner_text("Cat and Dog").await?;
    // Multiple selection keeps the popover open.
    page.count_stays(LISTBOX, 1, std::time::Duration::from_millis(100))
        .await?;
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

/// Setting a select's bound open state opens its popover (`data-open`), and Escape writes the
/// closed state back.
#[browser_test]
pub async fn open_state_bound_to_app_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let open_state = page.element("#sf-open-state").await?;
    page.element("#sf-open-toggle").await?.click().await?;
    page.element(LISTBOX).await?;
    open_state.wait_for_inner_text("true").await?;
    let root = page.element("#sf-open .leptonic-Select").await?;
    assert_that!(root)
        .has_attribute("data-open")
        .await
        .is_equal_to("true");
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(LISTBOX, 0).await?;
    open_state.wait_for_inner_text("false").await?;
    Ok(())
}

/// Validating the form with an empty required select marks it invalid, focuses the trigger and
/// describes it with the browser's message; selecting an option clears the error ("supports
/// validation errors").
#[browser_test]
pub async fn native_validation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let root = page.element("#sf-required .leptonic-Select").await?;
    let trigger = trigger_in(page, "#sf-required").await?;
    let select = page.element("#sf-required select").await?;
    assert_that!(select).has_attribute("required").await;
    assert_that!(trigger)
        .attribute("aria-describedby")
        .await
        .is_none();
    assert_that!(select.is_valid().await?).is_false();
    assert_that!(root).attribute("data-invalid").await.is_none();
    assert_that!(root)
        .has_attribute("data-required")
        .await
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
    let message = assert_that!(select)
        .property("validationMessage")
        .await
        .some()
        .is_not_blank()
        .actual()
        .clone();
    assert_that!(|| trigger.accessible_description())
        .eventually_ok()
        .matches(eq(message))
        .await;
    // Focus within the select: `data-focused` (focused by the script).
    root.wait_for_attr("data-focused", Some("true")).await?;
    trigger.click().await?;
    page.element(LISTBOX)
        .await?
        .element(role(AriaRole::Option).text("Cat"))
        .await?
        .click()
        .await?;
    page.wait_for_count(LISTBOX, 0).await?;
    trigger.wait_for_attr("aria-describedby", None).await?;
    root.wait_for_attr("data-invalid", None).await?;
    Ok(())
}

/// A required select lets its form submit once it has a value and blocks submission again after
/// its value is cleared ("should not submit if required and selectedKey is null").
#[browser_test]
pub async fn required_blocks_submission(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = trigger_in(page, "#sf-submit").await?;
    let submit = page.element("#sf-submit-button").await?;
    let submits = page.element("#sf-submits").await?;
    assert_that!(trigger)
        .inner_text()
        .await
        .is_equal_to("Select an item");
    trigger.click().await?;
    page.element(LISTBOX)
        .await?
        .element(role(AriaRole::Option).text("Cat"))
        .await?
        .click()
        .await?;
    trigger.wait_for_inner_text("Cat").await?;
    submit.click().await?;
    submits.wait_for_inner_text("1").await?;
    page.element("#sf-submit-clear").await?.click().await?;
    trigger.wait_for_inner_text("Select an item").await?;
    submit.click().await?;
    submits
        .inner_text_stays("1", std::time::Duration::from_millis(100))
        .await?;
    assert_that!(page.element("#sf-submit select").await?)
        .property("value")
        .await
        .some()
        .is_empty();
    Ok(())
}

/// A disabled select disables its hidden select and its trigger, which doesn't open on a virtual
/// click ("should send disabled prop to the hidden field").
#[browser_test]
pub async fn disabled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let select = page.element("#sf-disabled select").await?;
    assert_that!(select).enabled().await.is_false();
    let trigger = trigger_in(page, "#sf-disabled").await?;
    assert_that!(trigger).enabled().await.is_false();
    trigger.virtual_click().await?;
    page.count_stays(LISTBOX, 0, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// When autofill picks an option of the hidden select (a `change` event), the trigger shows it
/// ("should trigger on onSelectionChange when select onchange is triggered (autofill)").
#[browser_test]
pub async fn autofill(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = trigger_in(page, "#sf-required").await?;
    let select = page.element("#sf-required select").await?;
    page.low_level()
        .eval::<()>(
            "arguments[0].value = 'kangaroo';
         arguments[0].dispatchEvent(new Event('change', {bubbles: true}));",
            vec![select.to_json()?],
        )
        .await?;
    trigger.wait_for_inner_text("Kangaroo").await?;
    Ok(())
}

/// Pressing the trigger of a select without items doesn't open it ("shouldn't allow the user to
/// open the select if there are no items").
#[browser_test]
pub async fn no_items(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    trigger_in(page, "#sf-unloaded").await?.click().await?;
    page.count_stays(LISTBOX, 0, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A select without items that allows empty content opens on press and shows its empty state, a
/// "No results" option in a `data-empty` listbox ("should support empty state").
#[browser_test]
pub async fn empty_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    trigger_in(page, "#sf-empty-allowed").await?.click().await?;
    let listbox = page.element(LISTBOX).await?;
    assert_that!(listbox)
        .has_attribute("data-empty")
        .await
        .is_equal_to("true");
    let empty_option = page.element("[role=listbox] [role=option]").await?;
    assert_that!(empty_option)
        .inner_text()
        .await
        .is_equal_to("No results");
    page.send_keys(Key::Escape).await?;
    page.wait_for_count(LISTBOX, 0).await?;
    Ok(())
}

/// A required select with more than 300 options uses hidden inputs instead of a `<select>`, and
/// submitting it empty is blocked and shows the browser's message ("should support multiple
/// selection form integration with many items").
#[browser_test]
pub async fn many_items_validation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    assert_that!(page.count("#sf-many select").await?).is_equal_to(0);
    page.element("#sf-many-submit").await?.click().await?;
    // The browser's message for the first hidden input, which is required.
    let required = page.element("#sf-many input[required]").await?;
    let message = assert_that!(required)
        .property("validationMessage")
        .await
        .some()
        .is_not_blank()
        .actual()
        .clone();
    page.element("#sf-many .leptonic-FieldError")
        .await?
        .wait_for_inner_text(&message)
        .await?;
    page.element("#sf-many-submits")
        .await?
        .inner_text_stays("0", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Options selected with the keyboard fill the hidden inputs of a select with more than 300
/// options; the form submits them and its reset clears them ("should support multiple selection
/// form integration with many items").
#[browser_test]
pub async fn many_items_selection_and_reset(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let trigger = trigger_in(page, "#sf-many").await?;
    let form = page.element("#sf-many").await?;
    // Open with the keyboard, which focuses the first option (a pointer resting over the
    // popover would focus the option under it: `should_focus_on_hover`).
    page.element("h1").await?.hover().await?;
    trigger.focus().await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(
        &page
            .element(LISTBOX)
            .await?
            .element(role(AriaRole::Option).text("item0"))
            .await?,
    )
    .await?;
    page.send_keys(" ").await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(
        &page
            .element(LISTBOX)
            .await?
            .element(role(AriaRole::Option).text("item1"))
            .await?,
    )
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

/// A select whose value isn't among its options (not loaded yet) submits the value with its form
/// right away ("should have form value after initial render", "has a value immediately after
/// rendering").
#[browser_test]
pub async fn value_outside_the_options_is_submitted(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let form = page.element("#sf-unloaded").await?;
    assert_that!(form.form_values("unloaded").await?).contains_exactly(["cat"]);
    Ok(())
}

/// A select whose value isn't among its options shows the placeholder and is marked
/// `data-placeholder`, as without a value.
#[browser_test]
pub async fn value_outside_the_options_shows_the_placeholder(
    page: &Page<'_>,
) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let value = page.element("#sf-unloaded .leptonic-SelectValue").await?;
    assert_that!(value)
        .inner_text()
        .await
        .is_equal_to("Select an item");
    assert_that!(value)
        .has_attribute("data-placeholder")
        .await
        .is_equal_to("true");
    Ok(())
}

// Upstream: react-aria-components/test/RadioGroup.test.js @ 99e6102368
//! Behavior of the radio hooks (through the `RadioGroup` and `Radio` atoms): ARIA structure,
//! selection by press, the group as one tab stop (roving tabindex), arrow keys moving the
//! selection (wrapping, skipping disabled radios, both orientations), disabled and read-only
//! groups, and native required validation. Spec: react-aria-components `RadioGroup.test.js`.
use assertr::{matchers::eq, prelude::*};
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions, css};

const PATH: &str = "/atoms/radio-group";

/// The `<label>` of the radio with the visible text `text`.
async fn label(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.element(css("label").text(text)).await
}

/// The fixture's mirror of the main group's value.
async fn value(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-rg-value").await
}

/// The native radio inside the label with the visible text `text`.
async fn radio(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    label(page, text).await?.element("input").await
}

pub async fn structure(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let group = page.element("[role=radiogroup]").await?;
    assert_that!(group.referenced_text("aria-labelledby").await?).is_equal_to("Favorite pet");
    assert_that!(group.attr("aria-orientation").await?)
        .get_some()
        .is_equal_to("vertical");
    assert_that!(group.attr("data-orientation").await?)
        .get_some()
        .is_equal_to("vertical");

    let dogs = radio(page, "Dogs").await?;
    let cats = radio(page, "Cats").await?;
    assert_that!(dogs.attr("type").await?)
        .get_some()
        .is_equal_to("radio");
    let name = dogs.attr("name").await?;
    assert_that!(&name).is_some();
    assert_that!(cats.attr("name").await?).is_equal_to(name);
    assert_that!(cats.value().await?)
        .get_some()
        .is_equal_to("cats");
    // The group's description describes each radio.
    assert_that!(dogs.referenced_text("aria-describedby").await?).is_equal_to("Pick one.");
    Ok(())
}

/// Tab enters the group at a radio and leaves it with the next Tab (react-aria: "should not
/// navigate within the group using Tab").
pub async fn tab_enters_and_leaves_the_group(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-rg-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let dogs = radio(page, "Dogs").await?;
    page.wait_for_focus(&dogs).await?;
    label(page, "Dogs")
        .await?
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-rg-after").await?)
        .await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&dogs).await?;
    // Focusing doesn't select.
    value(page).await?.inner_text_stays("").await?;
    Ok(())
}

pub async fn selection_by_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let cats = label(page, "Cats").await?;
    cats.click().await?;
    value(page).await?.wait_for_inner_text("cats").await?;
    cats.wait_for_attr("data-selected", Some("true")).await?;
    assert_that!(radio(page, "Cats").await?.is_selected().await?).is_true();
    let dogs = label(page, "Dogs").await?;
    dogs.click().await?;
    value(page).await?.wait_for_inner_text("dogs").await?;
    cats.wait_for_attr("data-selected", None).await?;
    Ok(())
}

/// A virtual click on a radio's label selects it.
pub async fn virtual_label_click(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    label(page, "Dragons").await?.virtual_click().await?;
    value(page).await?.wait_for_inner_text("dragons").await?;
    // Another virtual click replaces the selection.
    label(page, "Dogs").await?.virtual_click().await?;
    value(page).await?.wait_for_inner_text("dogs").await?;
    Ok(())
}

/// Arrow keys select the next/previous radio, wrapping around.
pub async fn arrow_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let dogs = radio(page, "Dogs").await?;
    let cats = radio(page, "Cats").await?;
    let dragons = radio(page, "Dragons").await?;
    page.element("#test-rg-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&dogs).await?;

    let value = value(page).await?;
    // Down from Dogs wraps around to the first radio, Up back to the last.
    for (key, radio, expected) in [
        (Key::Down, &cats, "cats"),
        (Key::Right, &dragons, "dragons"),
        (Key::Down, &dogs, "dogs"),
        (Key::Up, &dragons, "dragons"),
        (Key::Left, &cats, "cats"),
    ] {
        page.send_keys(key).await?;
        page.wait_for_focus(radio).await?;
        value.wait_for_inner_text(expected).await?;
    }
    Ok(())
}

/// With a selection, only the selected radio is a tab stop.
pub async fn selected_radio_is_the_tab_stop(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    label(page, "Cats").await?.click().await?;
    value(page).await?.wait_for_inner_text("cats").await?;
    let cats = radio(page, "Cats").await?;
    assert_that!(cats.attr("tabindex").await?)
        .get_some()
        .is_equal_to("0");
    assert_that!(radio(page, "Dogs").await?.attr("tabindex").await?)
        .get_some()
        .is_equal_to("-1");
    page.element("#test-rg-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&cats).await?;
    Ok(())
}

pub async fn skips_disabled_radios(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    label(page, "Skip A").await?.click().await?;
    page.wait_for_focus(&radio(page, "Skip A").await?).await?;
    assert_that!(radio(page, "Skip B").await?.is_enabled().await?).is_false();
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&radio(page, "Skip C").await?).await?;
    label(page, "Skip C")
        .await?
        .wait_for_attr("data-selected", Some("true"))
        .await?;
    Ok(())
}

pub async fn horizontal(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let group = page
        .element("[role=radiogroup][aria-label='Horizontal']")
        .await?;
    assert_that!(group.attr("aria-orientation").await?)
        .get_some()
        .is_equal_to("horizontal");
    let b = label(page, "Horizontal B").await?;
    assert_that!(b.attr("data-selected").await?)
        .get_some()
        .is_equal_to("true");
    b.click().await?;
    page.send_keys(Key::Right).await?;
    label(page, "Horizontal C")
        .await?
        .wait_for_attr("data-selected", Some("true"))
        .await?;
    page.send_keys(Key::Left).await?;
    b.wait_for_attr("data-selected", Some("true")).await?;
    Ok(())
}

pub async fn disabled_group(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let group = page
        .element("[role=radiogroup][aria-label='Disabled group']")
        .await?;
    assert_that!(group.attr("aria-disabled").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(group.attr("data-disabled").await?)
        .get_some()
        .is_equal_to("true");
    let a = label(page, "Disabled group A").await?;
    assert_that!(a.attr("data-disabled").await?)
        .get_some()
        .is_equal_to("true");
    assert_that!(radio(page, "Disabled group A").await?.is_enabled().await?).is_false();
    a.click().await?;
    a.attr_stays("data-selected", None).await?;
    Ok(())
}

pub async fn read_only_group(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let group = page
        .element("[role=radiogroup][aria-label='Read-only group']")
        .await?;
    assert_that!(group.attr("aria-readonly").await?)
        .get_some()
        .is_equal_to("true");
    let a = label(page, "Read-only A").await?;
    let b = label(page, "Read-only B").await?;
    let a_input = radio(page, "Read-only A").await?;
    let b_input = radio(page, "Read-only B").await?;
    b.click().await?;
    a.attr_stays("data-selected", Some("true")).await?;
    assert_that!(b.attr("data-selected").await?).is_none();
    assert_that!(a_input.is_selected().await?).is_true();
    assert_that!(b_input.is_selected().await?).is_false();

    // Space on another radio: the browser unchecks A natively, the group restores it.
    a_input.focus().await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&b_input).await?;
    page.send_keys(Key::Space).await?;
    a_input.prop_stays("checked", "true").await?;
    assert_that!(b_input.is_selected().await?).is_false();
    assert_that!(a.attr("data-selected").await?)
        .get_some()
        .is_equal_to("true");
    Ok(())
}

/// Native required validation, also with the last radio disabled (react-aria: "supports
/// validation errors when last radio is disabled").
pub async fn validation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let form = page.element("#test-rg-form").await?;
    let group = form.element("[role=radiogroup]").await?;
    let a = radio(page, "Required A").await?;
    let b = radio(page, "Required B").await?;
    for input in [&a, &b] {
        assert_that!(input.attr("required").await?).is_some();
        assert_that!(input.attr("aria-required").await?).is_none();
    }
    assert_that!(group.attr("data-invalid").await?).is_none();
    let described = group.attr("aria-describedby").await?;
    let description = group.referenced_text("aria-describedby").await?;

    // The error (the browser's validation message) is added to the group's description.
    assert_that!(form.check_validity().await?).is_false();
    group.wait_for_attr("data-invalid", Some("true")).await?;
    let message = a.prop("validationMessage").await?.unwrap_or_default();
    assert_that!(&message).is_not_blank();
    let expected = [description.as_str(), message.as_str()]
        .into_iter()
        .filter(|text| !text.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    assert_that!(|| group.referenced_text("aria-describedby"))
        .eventually_ok()
        .matches(eq(expected))
        .await;

    label(page, "Required A").await?.click().await?;
    group.wait_for_attr("data-invalid", None).await?;
    group
        .wait_for_attr("aria-describedby", described.as_deref())
        .await?;
    Ok(())
}

/// "should support controlled value": the group shows its bound value, reports changes to it and
/// follows it; without a setter, selecting changes nothing.
pub async fn controlled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    assert_that!(radio(page, "Bound B").await?.is_selected().await?).is_true();
    label(page, "Bound A").await?.click().await?;
    page.element("#test-rg-bound-value")
        .await?
        .wait_for_inner_text("a")
        .await?;
    assert_that!(radio(page, "Bound A").await?.is_selected().await?).is_true();

    page.element("#test-rg-bound-set-c").await?.click().await?;
    page.element("#test-rg-bound-value")
        .await?
        .wait_for_inner_text("c")
        .await?;
    let bound_a = radio(page, "Bound A").await?;
    let bound_c = radio(page, "Bound C").await?;
    bound_c.wait_for_prop("checked", "true").await?;
    assert_that!(bound_a.is_selected().await?).is_false();

    let fixed_a = radio(page, "Fixed A").await?;
    let fixed_b = radio(page, "Fixed B").await?;
    label(page, "Fixed B").await?.click().await?;
    fixed_a.prop_stays("checked", "true").await?;
    assert_that!(fixed_b.is_selected().await?).is_false();
    Ok(())
}

/// A `Label` after a group (outside it) is a plain label: the group's label context doesn't leak
/// to its siblings.
pub async fn label_context_stays_inside(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let standalone = label(page, "Standalone label").await?;
    assert_that!(standalone.id().await?).is_none();
    Ok(())
}

/// A group with enum values (`selection_value!`): the app's signal holds the enum, the form
/// submits the variant's key.
pub async fn typed_values(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let value = page.element("#test-rg-typed-value").await?;
    value.wait_for_inner_text("Some(Small)").await?;
    label(page, "Typed large").await?.click().await?;
    value.wait_for_inner_text("Some(Large)").await?;
    let form = page.element("#test-rg-typed-form").await?;
    assert_that!(form.form_values("size").await?).contains_exactly(["l"]);
    Ok(())
}

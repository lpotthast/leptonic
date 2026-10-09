// Upstream: react-aria-components/test/RadioGroup.test.js @ 99e6102368
//! Behavior of the radio hooks (through the `RadioGroup` and `Radio` atoms): ARIA structure,
//! selection by press, the group as one tab stop (roving tabindex), arrow keys moving the
//! selection (wrapping, skipping disabled radios, both orientations), disabled and read-only
//! groups, and native required validation. Spec: react-aria-components `RadioGroup.test.js`.
use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, KeyKind, Page, SyntheticEvent, css};

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

/// The group is a vertical radiogroup named by its label; its radios are native radio inputs that
/// share one name, carry their values and are described by the group's description ("supports
/// help text").
#[browser_test]
pub async fn structure(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let group = page
        .element(css("[role=radiogroup]").has(css("label").text("Dogs")))
        .await?;
    assert_that!(group)
        .accessible_name()
        .await
        .is_equal_to("Favorite pet");
    assert_that!(group)
        .has_attribute("aria-orientation")
        .await
        .is_equal_to("vertical");
    assert_that!(group)
        .has_attribute("data-orientation")
        .await
        .is_equal_to("vertical");

    let dogs = radio(page, "Dogs").await?;
    let cats = radio(page, "Cats").await?;
    assert_that!(dogs)
        .has_attribute("type")
        .await
        .is_equal_to("radio");
    let name = assert_that!(dogs)
        .has_attribute("name")
        .await
        .actual()
        .clone();
    assert_that!(cats)
        .has_attribute("name")
        .await
        .is_equal_to(name);
    assert_that!(cats)
        .property("value")
        .await
        .get_some()
        .is_equal_to("cats");
    // The group's description describes each radio.
    assert_that!(dogs)
        .accessible_description()
        .await
        .is_equal_to("Pick one.");
    Ok(())
}

/// Tab enters the group at its first radio (focus-visible) and the next Tab leaves it; Shift+Tab
/// returns without selecting ("should not navigate within the group using Tab").
#[browser_test]
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
    value(page)
        .await?
        .inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Pressing a radio's label selects it (`data-selected`, checked input) and pressing another one
/// moves the selection ("should support selected state").
#[browser_test]
pub async fn selection_by_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let cats = label(page, "Cats").await?;
    cats.click().await?;
    value(page).await?.wait_for_inner_text("cats").await?;
    cats.wait_for_attr("data-selected", Some("true")).await?;
    assert_that!(radio(page, "Cats").await?)
        .selected()
        .await
        .is_true();
    let dogs = label(page, "Dogs").await?;
    dogs.click().await?;
    value(page).await?.wait_for_inner_text("dogs").await?;
    cats.wait_for_attr("data-selected", None).await?;
    Ok(())
}

/// A virtual click on a radio's label selects it, and one on another label moves the selection.
#[browser_test]
pub async fn virtual_label_click(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    label(page, "Dragons").await?.virtual_click().await?;
    value(page).await?.wait_for_inner_text("dragons").await?;
    // Another virtual click replaces the selection.
    label(page, "Dogs").await?.virtual_click().await?;
    value(page).await?.wait_for_inner_text("dogs").await?;
    Ok(())
}

/// Down and Right focus and select the next radio, Up and Left the previous one, wrapping around
/// at both ends.
#[browser_test]
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

/// With a selection, only the selected radio is a tab stop (`tabindex` 0, the others -1), so Tab
/// into the group focuses it.
#[browser_test]
pub async fn selected_radio_is_the_tab_stop(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    label(page, "Cats").await?.click().await?;
    value(page).await?.wait_for_inner_text("cats").await?;
    let cats = radio(page, "Cats").await?;
    assert_that!(cats)
        .has_attribute("tabindex")
        .await
        .is_equal_to("0");
    assert_that!(radio(page, "Dogs").await?)
        .has_attribute("tabindex")
        .await
        .is_equal_to("-1");
    page.element("#test-rg-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&cats).await?;
    Ok(())
}

/// The arrow keys skip a disabled radio: Down from the radio before it selects the one after it
/// ("should support disabled state on radio").
#[browser_test]
pub async fn skips_disabled_radios(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    label(page, "Skip A").await?.click().await?;
    page.wait_for_focus(&radio(page, "Skip A").await?).await?;
    assert_that!(radio(page, "Skip B").await?)
        .enabled()
        .await
        .is_false();
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&radio(page, "Skip C").await?).await?;
    label(page, "Skip C")
        .await?
        .wait_for_attr("data-selected", Some("true"))
        .await?;
    Ok(())
}

/// A horizontal group has `aria-orientation=horizontal`, and Right and Left select the next and
/// previous radio ("should support orientation").
#[browser_test]
pub async fn horizontal(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let group = page
        .element("[role=radiogroup][aria-label='Horizontal']")
        .await?;
    assert_that!(group)
        .has_attribute("aria-orientation")
        .await
        .is_equal_to("horizontal");
    let b = label(page, "Horizontal B").await?;
    assert_that!(b)
        .has_attribute("data-selected")
        .await
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

/// A disabled group has `aria-disabled` and `data-disabled`, its radios `data-disabled` and
/// disabled inputs, and pressing one doesn't select it ("should support disabled state on group").
#[browser_test]
pub async fn disabled_group(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let group = page
        .element("[role=radiogroup][aria-label='Disabled group']")
        .await?;
    assert_that!(group)
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");
    assert_that!(group)
        .has_attribute("data-disabled")
        .await
        .is_equal_to("true");
    let a = label(page, "Disabled group A").await?;
    assert_that!(a)
        .has_attribute("data-disabled")
        .await
        .is_equal_to("true");
    assert_that!(radio(page, "Disabled group A").await?)
        .enabled()
        .await
        .is_false();
    a.click().await?;
    a.attr_stays("data-selected", None, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A read-only group (`aria-readonly`) keeps its selection when another radio is pressed, or
/// focused with an arrow key and selected with Space ("should support read only state").
#[browser_test]
pub async fn read_only_group(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let group = page
        .element("[role=radiogroup][aria-label='Read-only group']")
        .await?;
    assert_that!(group)
        .has_attribute("aria-readonly")
        .await
        .is_equal_to("true");
    let a = label(page, "Read-only A").await?;
    let b = label(page, "Read-only B").await?;
    let a_input = radio(page, "Read-only A").await?;
    let b_input = radio(page, "Read-only B").await?;
    b.click().await?;
    a.attr_stays(
        "data-selected",
        Some("true"),
        std::time::Duration::from_millis(100),
    )
    .await?;
    assert_that!(b).attribute("data-selected").await.is_none();
    assert_that!(a_input).selected().await.is_true();
    assert_that!(b_input).selected().await.is_false();

    // Space on another radio: the browser unchecks A natively, the group restores it.
    a_input.focus().await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&b_input).await?;
    page.send_keys(Key::Space).await?;
    a_input
        .prop_stays("checked", "true", std::time::Duration::from_millis(100))
        .await?;
    assert_that!(b_input).selected().await.is_false();
    assert_that!(a)
        .has_attribute("data-selected")
        .await
        .is_equal_to("true");
    Ok(())
}

/// A required group whose last radio is disabled turns invalid on form validation, adds the
/// browser's message to its description and focuses its first radio; selecting a radio makes every
/// radio valid and clears the error ("supports validation errors when last radio is disabled").
#[browser_test]
pub async fn validation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let form = page.element("#test-rg-form").await?;
    let group = form.element("[role=radiogroup]").await?;
    let a = radio(page, "Required A").await?;
    let b = radio(page, "Required B").await?;
    for input in [&a, &b] {
        assert_that!(input).has_attribute("required").await;
        assert_that!(input)
            .attribute("aria-required")
            .await
            .is_none();
    }
    assert_that!(group)
        .attribute("data-invalid")
        .await
        .is_none();
    let described = group.attr("aria-describedby").await?;
    let description = group.accessible_description().await?;

    // The error (the browser's validation message) is added to the group's description.
    assert_that!(form.check_validity().await?).is_false();
    group.wait_for_attr("data-invalid", Some("true")).await?;
    let message = assert_that!(a)
        .property("validationMessage")
        .await
        .get_some()
        .is_not_blank()
        .actual()
        .clone();
    let expected = [description.as_str(), message.as_str()]
        .into_iter()
        .filter(|text| !text.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    assert_that!(|| group.accessible_description())
        .eventually_ok()
        .matches(eq(expected))
        .await;
    page.wait_for_focus(&a).await?;

    label(page, "Required A").await?.click().await?;
    group.wait_for_attr("data-invalid", None).await?;
    group
        .wait_for_attr("aria-describedby", described.as_deref())
        .await?;
    for input in [&a, &b] {
        assert_that!(input.is_valid().await?).is_true();
    }
    Ok(())
}

/// Selecting a radio of a required group with the arrow keys after a failed validation makes every
/// radio valid and removes the error from the group's description ("updates validation state with
/// the keyboard").
#[browser_test]
pub async fn validation_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let form = page.element("#test-rg-keyboard-form").await?;
    let group = form.element("[role=radiogroup]").await?;
    let radios = [
        radio(page, "Keyboard dogs").await?,
        radio(page, "Keyboard cats").await?,
        radio(page, "Keyboard dragons").await?,
    ];
    assert_that!(group)
        .attribute("aria-describedby")
        .await
        .is_none();
    for input in &radios {
        assert_that!(input).has_attribute("required").await;
        assert_that!(input.is_valid().await?).is_false();
    }

    assert_that!(form.check_validity().await?).is_false();
    group.wait_for_attr("data-invalid", Some("true")).await?;
    assert_that!(group).has_attribute("aria-describedby").await;
    page.wait_for_focus(&radios[0]).await?;

    page.send_keys(Key::Down).await?;
    group.wait_for_attr("aria-describedby", None).await?;
    for input in &radios {
        assert_that!(input.is_valid().await?).is_true();
    }
    Ok(())
}

/// Hovering a radio's label sets `data-hovered`, and leaving it clears it ("should support hover").
#[browser_test]
pub async fn hover(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let dogs = label(page, "Dogs").await?;
    assert_that!(dogs).attribute("data-hovered").await.is_none();
    dogs.hover().await?;
    dogs.wait_for_attr("data-hovered", Some("true")).await?;
    page.element("h1").await?.hover().await?;
    dogs.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Holding the pointer down on a radio's label sets `data-pressed` until it is released, which
/// selects it ("should support press state").
#[browser_test]
pub async fn press_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let dogs = label(page, "Dogs").await?;
    assert_that!(dogs).attribute("data-pressed").await.is_none();
    let held = dogs.press_and_hold().await?;
    dogs.wait_for_attr("data-pressed", Some("true")).await?;
    held.release().await?;
    dogs.wait_for_attr("data-pressed", None).await?;
    value(page).await?.wait_for_inner_text("dogs").await?;
    Ok(())
}

/// Holding Space on a focused radio sets `data-pressed` until the key is released ("should support
/// press state with keyboard").
#[browser_test]
pub async fn press_state_with_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let dogs = label(page, "Dogs").await?;
    let input = radio(page, "Dogs").await?;
    input.focus().await?;
    input
        .dispatch(SyntheticEvent::keyboard(KeyKind::Down, " "))
        .await?;
    dogs.wait_for_attr("data-pressed", Some("true")).await?;
    input
        .dispatch(SyntheticEvent::keyboard(KeyKind::Up, " "))
        .await?;
    dogs.wait_for_attr("data-pressed", None).await?;
    Ok(())
}

/// A group bound to a signal shows its value, writes selections to it and follows changes to it;
/// a group with a value but no setter keeps its selection when another radio is pressed.
#[browser_test]
pub async fn controlled(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    assert_that!(radio(page, "Bound B").await?)
        .selected()
        .await
        .is_true();
    label(page, "Bound A").await?.click().await?;
    page.element("#test-rg-bound-value")
        .await?
        .wait_for_inner_text("a")
        .await?;
    assert_that!(radio(page, "Bound A").await?)
        .selected()
        .await
        .is_true();

    page.element("#test-rg-bound-set-c").await?.click().await?;
    page.element("#test-rg-bound-value")
        .await?
        .wait_for_inner_text("c")
        .await?;
    let bound_a = radio(page, "Bound A").await?;
    let bound_c = radio(page, "Bound C").await?;
    bound_c.wait_for_prop("checked", "true").await?;
    assert_that!(bound_a).selected().await.is_false();

    let fixed_a = radio(page, "Fixed A").await?;
    let fixed_b = radio(page, "Fixed B").await?;
    label(page, "Fixed B").await?.click().await?;
    fixed_a
        .prop_stays("checked", "true", std::time::Duration::from_millis(100))
        .await?;
    assert_that!(fixed_b).selected().await.is_false();
    Ok(())
}

/// A `Label` after a group (outside it) is a plain label: the group's label context doesn't leak
/// to its siblings.
#[browser_test]
pub async fn label_context_stays_inside(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let standalone = label(page, "Standalone label").await?;
    assert_that!(standalone).attribute("id").await.is_none();
    Ok(())
}

/// Selecting a radio of a group with enum values (`selection_value!`) writes the variant to the
/// app's signal, and the form submits the variant's key.
#[browser_test]
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

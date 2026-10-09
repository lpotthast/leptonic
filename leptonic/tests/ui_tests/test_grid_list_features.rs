// Upstream: react-aria-components/test/GridList.test.js @ 99e6102368
//! Keyboard navigation into rows' children: "ArrowLeft/Right cycles through children and row
//! element" (`focusMode="child"`), "ArrowDown from child navigates to first child of next item",
//! Tab navigation ("Tab into list focuses row, Tab from row enters child, Shift+Tab returns to
//! row"), "allowsArrowNavigation allows arrow key row navigation when focused on child", and
//! text inputs in rows keeping their keys.
//!
//! "should support onAction on items", `selectionBehavior="replace"` (Ctrl toggles, the action
//! on double click), links, type-ahead, sections ("should support sections") and descriptions.
use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::{Report, prelude::ResultExt};

use crate::pages::{ElementActions, Page, css, role};

const PATH: &str = "/atoms/grid-list-features";

/// Focuses a fresh focusable element right before the grid list in `container` (so Tab enters
/// the grid list).
async fn focus_before(page: &Page<'_>, container: &str) -> Result<(), Report> {
    page.low_level()
        .eval::<()>(
            "const container = document.querySelector(arguments[0]);
         const before = document.createElement('button');
         before.textContent = 'Before';
         container.prepend(before);
         before.focus();",
            vec![container.into()],
        )
        .await
}

/// The button labelled `name`.
async fn button(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    page.element(format!("button[aria-label='{name}']")).await
}

/// The row labelled `label` (its text value) in the grid list inside `container`.
async fn row_labelled(page: &Page<'_>, container: &str, label: &str) -> Result<WebElement, Report> {
    page.element(format!("{container} [role=row][aria-label='{label}']"))
        .await
}

/// The row with the text `text` in the grid list inside `container`.
async fn row(page: &Page<'_>, container: &str, text: &str) -> Result<WebElement, Report> {
    let container = page.element(container).await?;
    container.element(role(AriaRole::Row).text(text)).await
}

/// In rows whose first child takes focus, ArrowRight walks the children and then the row, ArrowLeft
/// back, and ArrowDown and ArrowUp move to the first child of the next and previous row
/// ("ArrowLeft/Right cycles through children and row element", "ArrowDown from child navigates to
/// first child of next item", "ArrowUp from child navigates to first child of previous item").
#[browser_test]
pub async fn arrows_cycle_through_children_and_row(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let item_1 = row_labelled(page, "#glf-children", "Item 1").await?;
    let first = button(page, "Item 1 first").await?;
    let last = button(page, "Item 1 last").await?;
    focus_before(page, "#glf-children").await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&first).await?;
    for (key, target) in [
        (Key::Right, &last),
        (Key::Right, &item_1),
        (Key::Left, &last),
        (Key::Left, &first),
        (Key::Left, &item_1),
        (Key::Right, &first),
    ] {
        page.send_keys(key.clone()).await?;
        page.wait_for_focus(target)
            .await
            .context_with(|| format!("after pressing {key:?}"))?;
    }
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&button(page, "Item 2 first").await?)
        .await?;
    page.send_keys(Key::Up).await?;
    page.wait_for_focus(&first).await?;
    Ok(())
}

/// In right-to-left text, ArrowLeft moves forward through a row's children and the row, and
/// ArrowRight back ("ArrowLeft/Right RTL cycle: ArrowLeft advances forward, ArrowRight wraps to
/// last child").
#[browser_test]
pub async fn arrows_are_mirrored_right_to_left(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let row = row_labelled(page, "#glf-children-rtl", "RTL 1").await?;
    let first = button(page, "RTL first").await?;
    let last = button(page, "RTL last").await?;
    focus_before(page, "#glf-children-rtl").await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&first).await?;
    for (key, target) in [(Key::Left, &last), (Key::Left, &row), (Key::Right, &last)] {
        page.send_keys(key.clone()).await?;
        page.wait_for_focus(target)
            .await
            .context_with(|| format!("after pressing {key:?}"))?;
    }
    Ok(())
}

/// Tab focuses the row and then walks its children, and Shift+Tab goes back to the row ("Tab into
/// list focuses row, Tab from row enters child, Shift+Tab returns to row").
#[browser_test]
pub async fn tab_walks_the_children(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let row = row_labelled(page, "#glf-tab", "Tab 1").await?;
    let first = button(page, "Tab first").await?;
    let last = button(page, "Tab last").await?;
    focus_before(page, "#glf-tab").await?;
    for (keys, target) in [
        (TypingData::from(Key::Tab), &row),
        (TypingData::from(Key::Tab), &first),
        (TypingData::from(Key::Tab), &last),
        (Key::Shift + Key::Tab, &first),
        (Key::Shift + Key::Tab, &row),
    ] {
        let description = format!("after pressing {keys:?}");
        page.send_keys(keys).await?;
        page.wait_for_focus(target)
            .await
            .context_with(|| description.clone())?;
    }
    Ok(())
}

/// Rows whose child takes focus and that allow arrow navigation are no tab stops, and ArrowDown
/// moves from one row's child to the next row's ("allowsArrowNavigation allows arrow key row
/// navigation when focused on child").
#[browser_test]
pub async fn arrows_move_between_children_of_rows(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let rows = page.elements("#glf-child-arrows [role=row]").await?;
    assert_that!(&rows).has_length(3);
    for row in &rows {
        assert_that!(row)
            .has_attribute("tabindex")
            .await
            .is_equal_to("-1");
    }
    focus_before(page, "#glf-child-arrows").await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&button(page, "Arrow 1").await?).await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&button(page, "Arrow 2").await?).await?;
    Ok(())
}

/// A text input in a row keeps its arrow keys, typed text, Space and Enter, which neither move
/// focus nor select ("should not navigate rows when arrow keys are pressed while a text input
/// child has focus (%s)", "should not trigger typeahead when typing in a text input child (%s)",
/// "should not trigger selection when pressing Space or Enter in a text input child (%s)").
#[browser_test]
pub async fn text_input_keeps_its_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let apple = row_labelled(page, "#glf-input", "Apple").await?;
    let input = page.element("#glf-input input").await?;
    focus_before(page, "#glf-input").await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&apple).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&input).await?;
    for key in [Key::Down, Key::Up, Key::Right, Key::Left] {
        page.send_keys(key).await?;
    }
    page.send_keys("b ").await?;
    page.focus_stays(&input, std::time::Duration::from_millis(100))
        .await?;
    input.wait_for_prop("value", "b ").await?;
    page.send_keys(Key::Enter).await?;
    page.element("#glf-input-selection")
        .await?
        .inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Clicking the text input in a row of a Tab-navigated grid list focuses the input and selects
/// nothing, while clicking a row without tabbable children selects it ("should not trigger
/// selection when clicking on a tabbable child element (keyboardNavigationBehavior="tab")",
/// "should still trigger selection when clicking on a row with no tabbable children").
#[browser_test]
pub async fn clicking_a_tabbable_child_in_tab_navigation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = page.element("#glf-input input").await?;
    let selection = page.element("#glf-input-selection").await?;
    input.click().await?;
    page.wait_for_focus(&input).await?;
    page.focus_stays(&input, std::time::Duration::from_millis(100))
        .await?;
    selection
        .inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    row(page, "#glf-input", "Banana").await?.click().await?;
    selection.wait_for_inner_text("Banana").await?;
    Ok(())
}

/// Rows with an action show hover, while rows without selection or action don't ("should not show
/// hover state when item is not interactive").
#[browser_test]
pub async fn hover_on_rows_with_an_action(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let apple = row(page, "#glf-action", "Apple").await?;
    apple.hover().await?;
    apple.wait_for_attr("data-hovered", Some("true")).await?;
    let plain = page.first_element("#glf-children [role=row]").await?;
    plain.hover().await?;
    apple.wait_for_attr("data-hovered", None).await?;
    plain
        .attr_stays("data-hovered", None, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// In a list without selection, clicking a row or pressing Enter on it runs its action, and typing
/// a letter moves focus to the matching row ("should support onAction on items").
#[browser_test]
pub async fn actions_without_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let actions = page.element("#glf-action-actions").await?;
    let apple = row(page, "#glf-action", "Apple").await?;
    apple.click().await?;
    actions.wait_for_inner_text("Apple").await?;
    page.wait_for_focus(&apple).await?;
    // Type-ahead.
    page.send_keys("c").await?;
    page.wait_for_focus(&row(page, "#glf-action", "Cherry").await?)
        .await?;
    page.send_keys(Key::Enter).await?;
    actions.wait_for_inner_text("Apple,Cherry").await?;
    Ok(())
}

/// With replace selection behavior, a click replaces the selection, Ctrl+click toggles a row, and
/// a double click runs the action and selects only that row ("should perform replace selection in
/// highlight mode when not using modifier keys", "should perform toggle selection in highlight
/// mode when using modifier keys").
#[browser_test]
pub async fn replace_selection_behavior(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let selection = page.element("#glf-replace-selection").await?;
    row(page, "#glf-replace", "Apple").await?.click().await?;
    selection.wait_for_inner_text("Apple").await?;
    row(page, "#glf-replace", "Banana").await?.click().await?;
    selection.wait_for_inner_text("Banana").await?;
    let cherry = row(page, "#glf-replace", "Cherry").await?;
    page.low_level()
        .driver()
        .action_chain()
        .key_down(Key::Control)
        .click_element(&cherry)
        .key_up(Key::Control)
        .perform()
        .await?;
    selection.wait_for_inner_text("Banana,Cherry").await?;
    let apple = row(page, "#glf-replace", "Apple").await?;
    page.low_level()
        .driver()
        .action_chain()
        .double_click_element(&apple)
        .perform()
        .await?;
    page.element("#glf-replace-actions")
        .await?
        .wait_for_inner_text("Apple")
        .await?;
    selection.wait_for_inner_text("Apple").await?;
    Ok(())
}

/// Clicking a link row navigates to its URL ("should support links with selectionMode="none"").
#[browser_test]
pub async fn links_open_on_press(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    row(page, "#glf-links", "One").await?.click().await?;
    assert_that!(|| async {
        Ok::<_, Report>(
            page.low_level()
                .driver()
                .current_url()
                .await?
                .fragment()
                .map(str::to_owned),
        )
    })
    .eventually_ok()
    .matches(eq(Some("glf-one".to_owned())))
    .await;
    Ok(())
}

/// Sections are row groups labelled by their header rows, which the arrow keys skip, and a row with
/// a description is labelled by its text and then the description ("should support sections").
#[browser_test]
pub async fn sections_and_descriptions(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let groups = page.elements("#glf-sections [role=rowgroup]").await?;
    assert_that!(&groups).has_length(2);
    assert_that!(groups[0])
        .accessible_name()
        .await
        .is_equal_to("Fruit");
    let header_id = groups[0].attr("aria-labelledby").await?.unwrap_or_default();
    let header = page.element(format!("#{header_id}")).await?;
    assert_that!(header)
        .has_attribute("role")
        .await
        .is_equal_to("rowheader");

    let banana = row_labelled(page, "#glf-sections", "Banana").await?;
    banana.click().await?;
    page.wait_for_focus(&banana).await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&row_labelled(page, "#glf-sections", "Carrot").await?)
        .await?;
    // The description labels the row, after its text.
    let description = banana.element(css("*").text("Yellow")).await?;
    let description_id = description.id().await?.unwrap_or_default();
    assert_that!(banana)
        .has_attribute("aria-labelledby")
        .await
        .derive_owned(|labelledby| labelledby.split_whitespace().last())
        .get_some()
        .is_equal_to(description_id.as_str());
    // The row's own `aria-label` stands for the row in its `aria-labelledby`.
    assert_that!(banana)
        .accessible_name()
        .await
        .is_equal_to("Banana Yellow");
    Ok(())
}

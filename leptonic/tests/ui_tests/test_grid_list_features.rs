// Upstream: react-aria-components/test/GridList.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::{Report, prelude::ResultExt};

use crate::{
    pages::{ElementActions, Page, PageActions, xpath},
    polling::wait_for,
};

/// Keyboard navigation into rows' children: "ArrowLeft/Right cycles through children and row
/// element" (`focusMode="child"`), "ArrowDown from child navigates to first child of next item",
/// Tab navigation ("Tab into list focuses row, Tab from row enters child, Shift+Tab returns to
/// row"), "allowsArrowNavigation allows arrow key row navigation when focused on child", and
/// text inputs in rows keeping their keys.
pub struct GridListChildNavigationTests {}

#[async_trait]
impl BrowserTest<str> for GridListChildNavigationTests {
    fn name(&self) -> Cow<'_, str> {
        "grid_list_child_navigation_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/grid-list-features").await?;

        cases!(
            arrows_cycle_through_children_and_row(&page),
            arrows_are_mirrored_right_to_left(&page),
            tab_walks_the_children(&page),
            arrows_move_between_children_of_rows(&page),
            text_input_keeps_its_keys(&page),
        );

        Ok(())
    }
}

/// "should support onAction on items", `selectionBehavior="replace"` (Ctrl toggles, the action
/// on double click), links, type-ahead, sections ("should support sections") and descriptions.
pub struct GridListActionsTests {}

#[async_trait]
impl BrowserTest<str> for GridListActionsTests {
    fn name(&self) -> Cow<'_, str> {
        "grid_list_actions_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/grid-list-features").await?;

        cases!(
            hover_on_rows_with_an_action(&page),
            actions_without_selection(&page),
            replace_selection_behavior(&page),
            links_open_on_press(&page),
            sections_and_descriptions(&page),
        );

        Ok(())
    }
}

/// Focuses a fresh focusable element right before the grid list in `container` (so Tab enters
/// the grid list).
async fn focus_before(page: &Page<'_>, container: &str) -> Result<(), Report> {
    page.eval::<()>(
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
    container
        .element(xpath(format!(
            ".//*[@role='row'][normalize-space(.)='{text}']"
        )))
        .await
}

/// Rows whose first child takes focus: ArrowRight walks the children, then the row; ArrowLeft
/// back; ArrowDown/Up move to the first child of the next/previous row.
async fn arrows_cycle_through_children_and_row(page: &Page<'_>) -> Result<(), Report> {
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

/// Right to left ("ArrowLeft/Right RTL cycle"): ArrowLeft moves forward, ArrowRight back.
async fn arrows_are_mirrored_right_to_left(page: &Page<'_>) -> Result<(), Report> {
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

/// Tab enters the row, then walks the row's children; Shift+Tab goes back to the row.
async fn tab_walks_the_children(page: &Page<'_>) -> Result<(), Report> {
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

/// Rows whose child takes focus and that allow arrow navigation are no tab stops; arrows move
/// between the rows' children.
async fn arrows_move_between_children_of_rows(page: &Page<'_>) -> Result<(), Report> {
    let rows = page.elements("#glf-child-arrows [role=row]").await?;
    assert_that!(&rows).has_length(3);
    for row in &rows {
        assert_that!(row.attr("tabindex").await?)
            .get_some()
            .is_equal_to("-1");
    }
    focus_before(page, "#glf-child-arrows").await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&button(page, "Arrow 1").await?).await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&button(page, "Arrow 2").await?).await?;
    Ok(())
}

/// A text input in a row keeps its arrow keys, typed text, Space and Enter.
async fn text_input_keeps_its_keys(page: &Page<'_>) -> Result<(), Report> {
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
    page.focus_stays(&input).await?;
    input.wait_for_prop("value", "b ").await?;
    page.send_keys(Key::Enter).await?;
    page.element("#glf-input-selection")
        .await?
        .inner_text_stays("")
        .await?;
    Ok(())
}

/// Rows with an action show hover; rows without selection or action don't.
async fn hover_on_rows_with_an_action(page: &Page<'_>) -> Result<(), Report> {
    let apple = row(page, "#glf-action", "Apple").await?;
    apple.hover().await?;
    apple.wait_for_attr("data-hovered", Some("true")).await?;
    let plain = page.element("#glf-children [role=row]").await?;
    plain.hover().await?;
    apple.wait_for_attr("data-hovered", None).await?;
    plain.attr_stays("data-hovered", None).await?;
    Ok(())
}

/// Actions without selection: by press and by Enter; type-ahead moves focus.
async fn actions_without_selection(page: &Page<'_>) -> Result<(), Report> {
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

/// Replace selection behavior: a press replaces the selection, Ctrl+press toggles, a double
/// click runs the action (and selects the row).
async fn replace_selection_behavior(page: &Page<'_>) -> Result<(), Report> {
    let selection = page.element("#glf-replace-selection").await?;
    row(page, "#glf-replace", "Apple").await?.click().await?;
    selection.wait_for_inner_text("Apple").await?;
    row(page, "#glf-replace", "Banana").await?.click().await?;
    selection.wait_for_inner_text("Banana").await?;
    let cherry = row(page, "#glf-replace", "Cherry").await?;
    page.driver
        .action_chain()
        .key_down(Key::Control)
        .click_element(&cherry)
        .key_up(Key::Control)
        .perform()
        .await?;
    selection.wait_for_inner_text("Banana,Cherry").await?;
    let apple = row(page, "#glf-replace", "Apple").await?;
    page.driver
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

/// Link rows navigate on press.
async fn links_open_on_press(page: &Page<'_>) -> Result<(), Report> {
    row(page, "#glf-links", "One").await?.click().await?;
    wait_for("the URL's fragment")
        .observing(|| async {
            Ok(page
                .driver
                .current_url()
                .await?
                .fragment()
                .map(str::to_owned))
        })
        .to_be_equal_to(Some("glf-one".to_owned()))
        .await?;
    Ok(())
}

/// Sections are row groups labelled by their header rows, which arrow keys skip; a row with a
/// description is labelled by its text and the description.
async fn sections_and_descriptions(page: &Page<'_>) -> Result<(), Report> {
    let groups = page.elements("#glf-sections [role=rowgroup]").await?;
    assert_that!(&groups).has_length(2);
    assert_that!(groups[0].referenced_text("aria-labelledby").await?).is_equal_to("Fruit");
    let header_id = groups[0].attr("aria-labelledby").await?.unwrap_or_default();
    let header = page.element(format!("#{header_id}")).await?;
    assert_that!(header.attr("role").await?)
        .get_some()
        .is_equal_to("rowheader");

    let banana = row_labelled(page, "#glf-sections", "Banana").await?;
    banana.click().await?;
    page.wait_for_focus(&banana).await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&row_labelled(page, "#glf-sections", "Carrot").await?)
        .await?;
    // The description labels the row, after its text.
    let description = banana
        .element(xpath(".//*[normalize-space(.)='Yellow']"))
        .await?;
    let description_id = description.id().await?.unwrap_or_default();
    let labelledby = banana.attr("aria-labelledby").await?.unwrap_or_default();
    assert_that!(labelledby.split_whitespace().last())
        .get_some()
        .is_equal_to(description_id.as_str());
    Ok(())
}

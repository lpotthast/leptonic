// Upstream: react-aria-components/test/GridList.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

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

        // Arrow navigation, rows whose first child takes focus.
        focus_before(&page, "#glf-children").await?;
        page.press_tab().await?;
        expect_focus_on_button(&page, "Item 1 first").await?;
        page.send_keys_to_active(Key::Right).await?;
        expect_focus_on_button(&page, "Item 1 last").await?;
        page.send_keys_to_active(Key::Right).await?;
        page.wait_for_focus("row", None).await?;
        page.send_keys_to_active(Key::Left).await?;
        expect_focus_on_button(&page, "Item 1 last").await?;
        page.send_keys_to_active(Key::Left).await?;
        expect_focus_on_button(&page, "Item 1 first").await?;
        page.send_keys_to_active(Key::Down).await?;
        expect_focus_on_button(&page, "Item 2 first").await?;
        page.send_keys_to_active(Key::Up).await?;
        expect_focus_on_button(&page, "Item 1 first").await?;

        // Tab navigation: Tab enters and walks the row's children, Shift+Tab goes back.
        focus_before(&page, "#glf-tab").await?;
        page.press_tab().await?;
        page.wait_for_focus("row", None).await?;
        page.press_tab().await?;
        expect_focus_on_button(&page, "Tab first").await?;
        page.press_tab().await?;
        expect_focus_on_button(&page, "Tab last").await?;
        page.press_shift_tab().await?;
        expect_focus_on_button(&page, "Tab first").await?;
        page.press_shift_tab().await?;
        page.wait_for_focus("row", None).await?;

        // Tab navigation, rows whose child takes focus and that allow arrow navigation: the rows
        // are no tab stops, arrows move between the rows' children.
        let rows = page
            .driver
            .find_all(By::Css("#glf-child-arrows [role=row]"))
            .await?;
        for row in &rows {
            assert_that!(row.attr("tabindex").await?).is_equal_to(Some("-1".to_owned()));
        }
        focus_before(&page, "#glf-child-arrows").await?;
        page.press_tab().await?;
        expect_focus_on_button(&page, "Arrow 1").await?;
        page.send_keys_to_active(Key::Down).await?;
        expect_focus_on_button(&page, "Arrow 2").await?;

        // A text input in a row keeps its arrow keys, typed text and Space.
        focus_before(&page, "#glf-input").await?;
        page.press_tab().await?;
        page.wait_for_focus("row", Some("Apple")).await?;
        page.press_tab().await?;
        let input = page.css("#glf-input input").await?;
        page.wait_for_focus_on(&input, "the row's text input")
            .await?;
        for key in [Key::Down, Key::Up, Key::Right, Key::Left] {
            page.send_keys_to_active(key).await?;
        }
        page.send_keys_to_active("b ").await?;
        stays!(
            "the focus on the input",
            true,
            page.driver.active_element().await? == input
        );
        wait_for!(
            "the input value",
            "b ".to_owned(),
            input_value(&input).await?
        );
        page.send_keys_to_active(Key::Enter).await?;
        stays!(
            "the selection",
            String::new(),
            page.read_text_of("glf-input-selection").await?
        );
        page.expect_no_page_errors().await
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

        // Hover on rows with an action, none on rows without selection or action.
        let apple = row(&page, "#glf-action", "Apple").await?;
        hover(&page, &apple).await?;
        page.wait_for_attr(&apple, "data-hovered", Some("true"))
            .await?;
        let plain = page.css("#glf-children [role=row]").await?;
        hover(&page, &plain).await?;
        page.wait_for_attr(&apple, "data-hovered", None).await?;
        stays!("data-hovered", None, plain.attr("data-hovered").await?);

        // Actions without selection: press and Enter.
        row(&page, "#glf-action", "Apple").await?.click().await?;
        page.wait_for_text("glf-action-actions", "Apple").await?;
        page.wait_for_focus("row", Some("Apple")).await?;
        // Type-ahead.
        page.send_keys_to_active("c").await?;
        page.wait_for_focus("row", Some("Cherry")).await?;
        page.send_keys_to_active(Key::Enter).await?;
        page.wait_for_text("glf-action-actions", "Apple,Cherry")
            .await?;

        // Replace selection behavior.
        let selection = "glf-replace-selection";
        row(&page, "#glf-replace", "Apple").await?.click().await?;
        page.wait_for_text(selection, "Apple").await?;
        row(&page, "#glf-replace", "Banana").await?.click().await?;
        page.wait_for_text(selection, "Banana").await?;
        let cherry = row(&page, "#glf-replace", "Cherry").await?;
        page.driver
            .action_chain()
            .key_down(Key::Control)
            .click_element(&cherry)
            .key_up(Key::Control)
            .perform()
            .await?;
        page.wait_for_text(selection, "Banana,Cherry").await?;
        page.driver
            .action_chain()
            .double_click_element(&row(&page, "#glf-replace", "Apple").await?)
            .perform()
            .await?;
        page.wait_for_text("glf-replace-actions", "Apple").await?;
        page.wait_for_text(selection, "Apple").await?;

        // Links open on press.
        row(&page, "#glf-links", "One").await?.click().await?;
        wait_for!(
            "the location hash",
            "#glf-one".to_owned(),
            hash(&page).await?
        );

        // Sections: row groups labelled by their header rows.
        let groups = page
            .driver
            .find_all(By::Css("#glf-sections [role=rowgroup]"))
            .await?;
        assert_that!(groups.len()).is_equal_to(2);
        let header_id = groups[0].attr("aria-labelledby").await?.unwrap_or_default();
        let header = page.element(&header_id).await?;
        assert_that!(header.attr("role").await?).is_equal_to(Some("rowheader".to_owned()));
        assert_that!(header.text().await?).is_equal_to("Fruit".to_owned());
        // Arrow keys skip the header rows.
        let banana = page
            .css("#glf-sections [role=row][aria-label=Banana]")
            .await?;
        banana.click().await?;
        page.wait_for_focus_on(&banana, "Banana").await?;
        page.send_keys_to_active(Key::Down).await?;
        page.wait_for_focus("row", Some("Carrot")).await?;
        // A description: the row is labelled by its text and the description.
        let labelledby = banana.attr("aria-labelledby").await?.unwrap_or_default();
        let mut texts = Vec::new();
        for id in labelledby.split_whitespace() {
            texts.push(
                page.element(id)
                    .await?
                    .prop("textContent")
                    .await?
                    .unwrap_or_default(),
            );
        }
        assert_that!(texts.last().cloned()).is_equal_to(Some("Yellow".to_owned()));
        page.expect_no_page_errors().await
    }
}

/// Focuses a fresh focusable element right before the grid list in `container` (so Tab enters
/// the grid list).
async fn focus_before(page: &Page<'_>, container: &str) -> Result<(), Report> {
    page.driver
        .execute(
            "let container = document.querySelector(arguments[0]); \
             let before = document.createElement('button'); \
             before.textContent = 'Before'; \
             container.prepend(before); \
             before.focus();",
            vec![serde_json::Value::from(container)],
        )
        .await?;
    Ok(())
}

async fn expect_focus_on_button(page: &Page<'_>, name: &str) -> Result<(), Report> {
    let button = page.css(&format!("button[aria-label='{name}']")).await?;
    page.wait_for_focus_on(&button, name).await
}

/// The row with the text `text` in the grid list inside `container`.
async fn row(page: &Page<'_>, container: &str, text: &str) -> Result<WebElement, Report> {
    let container = page.css(container).await?;
    Ok(container
        .find(By::XPath(format!(
            ".//*[@role='row'][normalize-space(.)='{text}']"
        )))
        .await?)
}

async fn input_value(input: &WebElement) -> Result<String, Report> {
    Ok(input.prop("value").await?.unwrap_or_default())
}

async fn hash(page: &Page<'_>) -> Result<String, Report> {
    Ok(page
        .driver
        .execute("return window.location.hash;", vec![])
        .await?
        .convert::<String>()?)
}

/// Moves the pointer over `element`.
async fn hover(page: &Page<'_>, element: &WebElement) -> Result<(), Report> {
    page.driver
        .action_chain()
        .move_to_element_center(element)
        .perform()
        .await?;
    Ok(())
}

// Upstream: react-aria-components/test/Table.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

const PATH: &str = "/atoms/table-selection";

/// Selection and actions of the table atoms: `selectionBehavior="replace"` (mouse and
/// keyboard), `escapeKeyBehavior="none"`, `shouldSelectOnPressUp`, row actions, and columns that
/// change while rows stay ("supports removing a column and adding it back", plus renaming,
/// reordering, sortability and the selection mode). Spec: react-aria-components
/// `Table.test.js`.
pub struct TableSelectionTests {}

#[async_trait]
impl BrowserTest<str> for TableSelectionTests {
    fn name(&self) -> Cow<'_, str> {
        "table_selection_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path(PATH).await?;

        replace_selection_with_the_mouse(&page).await?;
        replace_selection_in_single_mode(&page).await?;
        replace_selection_with_the_keyboard(&page).await?;
        escape_without_clearing(&page).await?;
        select_on_press_down_or_up(&page).await?;
        row_actions(&page).await?;
        changing_columns(&page).await?;
        hover_and_focus_states(&page).await?;

        page.expect_no_page_errors().await
    }
}

async fn table(page: &Page<'_>, label: &str) -> Result<WebElement, Report> {
    page.css(&format!("[role=grid][aria-label='{label}']"))
        .await
}

/// The body row of the table `label` whose row header is `name`.
async fn row(page: &Page<'_>, label: &str, name: &str) -> Result<WebElement, Report> {
    Ok(table(page, label)
        .await?
        .find(By::XPath(format!(
            ".//tbody/*[@role='row'][.//*[@role='rowheader'][normalize-space(.)='{name}']]"
        )))
        .await?)
}

/// The texts of the column headers of the table `label`.
async fn column_headers(page: &Page<'_>, label: &str) -> Result<Vec<String>, Report> {
    let mut texts = Vec::new();
    for header in table(page, label)
        .await?
        .find_all(By::Css("[role=columnheader]"))
        .await?
    {
        texts.push(header.text().await?.trim().to_owned());
    }
    Ok(texts)
}

/// The texts of the cells of `row`.
async fn cell_texts(row: &WebElement) -> Result<Vec<String>, Report> {
    let mut texts = Vec::new();
    for cell in row.find_all(By::Css("td")).await? {
        texts.push(cell.text().await?.trim().to_owned());
    }
    Ok(texts)
}

async fn click_with_control(page: &Page<'_>, element: &WebElement) -> Result<(), Report> {
    page.driver
        .action_chain()
        .key_down(Key::Control)
        .click_element(element)
        .key_up(Key::Control)
        .perform()
        .await?;
    Ok(())
}

/// Wait until the selection of the table `id` is `expected`, then check that it took
/// `changes` selection changes.
async fn expect_selection(
    page: &Page<'_>,
    id: &str,
    expected: &str,
    changes: usize,
) -> Result<(), Report> {
    page.wait_for_text(&format!("test-ts-{id}-selection"), expected)
        .await?;
    page.wait_for_text(&format!("test-ts-{id}-changes"), &changes.to_string())
        .await
}

/// "should not render checkboxes for selection with selectionBehavior=replace" and "should
/// perform replace selection in highlight mode when not using modifier keys" / "should
/// perform toggle selection in highlight mode when using modifier keys" (mouse).
async fn replace_selection_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    const REPLACE: &str = "Replace table";
    let checkboxes = page
        .count_matching("[role=grid][aria-label='Replace table'] input[type=checkbox]")
        .await?;
    assert_that!(checkboxes).is_equal_to(0);

    let bootmgr = row(page, REPLACE, "bootmgr").await?;
    let program_files = row(page, REPLACE, "Program Files").await?;
    assert_that!(bootmgr.attr("aria-selected").await?).is_equal_to(Some("false".to_owned()));
    bootmgr.click().await?;
    expect_selection(page, "replace", "3", 1).await?;
    page.wait_for_attr(&bootmgr, "data-selected", Some("true"))
        .await?;
    // Without modifiers: replaced.
    program_files.click().await?;
    expect_selection(page, "replace", "2", 2).await?;
    page.wait_for_attr(&bootmgr, "aria-selected", Some("false"))
        .await?;
    // Pressing it again doesn't deselect it.
    program_files.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    expect_selection(page, "replace", "2", 2).await?;
    // With a modifier: toggled.
    click_with_control(page, &bootmgr).await?;
    expect_selection(page, "replace", "2,3", 3).await?;
    click_with_control(page, &program_files).await?;
    expect_selection(page, "replace", "3", 4).await
}

/// "should perform selection with single selection" (mouse).
async fn replace_selection_in_single_mode(page: &Page<'_>) -> Result<(), Report> {
    const SINGLE: &str = "Single replace table";
    let bootmgr = row(page, SINGLE, "bootmgr").await?;
    let program_files = row(page, SINGLE, "Program Files").await?;
    click_with_control(page, &bootmgr).await?;
    expect_selection(page, "single-replace", "3", 1).await?;
    click_with_control(page, &program_files).await?;
    expect_selection(page, "single-replace", "2", 2).await?;
    click_with_control(page, &program_files).await?;
    expect_selection(page, "single-replace", "", 3).await
}

/// Replace selection follows keyboard focus, from the first focused row on; Shift extends it
/// ("keyboard" variants).
async fn replace_selection_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.click_element_with_id("test-ts-before-replace").await?;
    page.press_tab().await?;
    let games = row(page, "Replace table", "Games").await?;
    page.wait_for_focus_on(&games, "the first row").await?;
    // Focused by keyboard: a focus ring.
    page.wait_for_attr(&games, "data-focus-visible", Some("true"))
        .await?;
    page.wait_for_text("test-ts-replace-selection", "1").await?;
    page.send_keys_to_active(Key::Down).await?;
    page.wait_for_text("test-ts-replace-selection", "2").await?;
    page.send_keys_to_active(Key::Shift + Key::Down).await?;
    page.wait_for_text("test-ts-replace-selection", "2,3").await
}

/// "should prevent Esc from clearing selection if escapeKeyBehavior is "none"".
async fn escape_without_clearing(page: &Page<'_>) -> Result<(), Report> {
    const ESCAPE: &str = "Escape table";
    for (name, expected, changes) in [("Games", "1", 1), ("Program Files", "1,2", 2)] {
        row(page, ESCAPE, name)
            .await?
            .find(By::Css("input[type=checkbox]"))
            .await?
            .click()
            .await?;
        expect_selection(page, "escape", expected, changes).await?;
    }
    page.send_keys_to_active(Key::Escape).await?;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    expect_selection(page, "escape", "1,2", 2).await
}

/// "shouldSelectOnPressUp": without it, the press start selects; with it, the press end. With
/// it, a row the browser would drag doesn't lose focus to the pressed cell: the cell drops its
/// tabindex during the pointer down (useGridCell).
async fn select_on_press_down_or_up(page: &Page<'_>) -> Result<(), Report> {
    const UP: &str = "Press up table";
    let down = row(page, "Press down table", "Games").await?;
    page.driver
        .action_chain()
        .click_and_hold_element(&down)
        .perform()
        .await?;
    expect_selection(page, "press-down", "1", 1).await?;
    page.driver.action_chain().release().perform().await?;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    expect_selection(page, "press-down", "1", 1).await?;

    let up = row(page, UP, "Games").await?;
    page.driver
        .action_chain()
        .click_and_hold_element(&up)
        .perform()
        .await?;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    expect_selection(page, "press-up", "", 0).await?;
    page.driver.action_chain().release().perform().await?;
    expect_selection(page, "press-up", "1", 1).await?;

    // A draggable row: the browser's default focus on pointer down skips the pressed cell.
    let program_files = row(page, UP, "Program Files").await?;
    page.driver
        .execute(
            "arguments[0].setAttribute('draggable', 'true')",
            vec![program_files.to_json()?],
        )
        .await?;
    let type_cell = program_files
        .find(By::XPath(
            ".//*[@role='gridcell'][normalize-space(.)='File folder']",
        ))
        .await?;
    page.driver
        .action_chain()
        .click_and_hold_element(&type_cell)
        .perform()
        .await?;
    page.wait_for_focus_on(&program_files, "the pressed row")
        .await?;
    page.driver.action_chain().release().perform().await?;
    expect_selection(page, "press-up", "2", 2).await?;
    page.wait_for_focus_on(&program_files, "the pressed row")
        .await?;
    // The cell gets its tabindex back.
    page.wait_for_attr(&type_cell, "tabindex", Some("-1")).await
}

/// "should support row actions": the row is pressed while the pointer is down, and the press
/// runs the action; Enter runs it too.
async fn row_actions(page: &Page<'_>) -> Result<(), Report> {
    let games = row(page, "Action table", "Games").await?;
    assert_that!(games.attr("data-pressed").await?).is_none();
    page.driver
        .action_chain()
        .click_and_hold_element(&games)
        .perform()
        .await?;
    page.wait_for_attr(&games, "data-pressed", Some("true"))
        .await?;
    page.driver.action_chain().release().perform().await?;
    page.wait_for_attr(&games, "data-pressed", None).await?;
    page.wait_for_text("test-ts-action", "1").await?;
    page.wait_for_text("test-ts-action-count", "1").await?;

    page.send_keys_to_active(Key::Down).await?;
    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_text("test-ts-action", "2").await?;
    page.wait_for_text("test-ts-action-count", "2").await
}

/// Columns change while the rows stay: hidden and shown again ("supports removing a column and
/// adding it back"), renamed, moved, made sortable; the selection mode switches. Headers, kept
/// cells and navigation follow.
async fn changing_columns(page: &Page<'_>) -> Result<(), Report> {
    const COLUMNS: &str = "Columns table";
    let expect_headers = |expected: Vec<&'static str>| async move {
        wait_for!(
            "the column headers",
            expected,
            column_headers(page, COLUMNS).await?
        );
        Ok::<(), Report>(())
    };
    let expect_cells = |expected: Vec<&'static str>| async move {
        wait_for!(
            "the cells of Games",
            expected,
            cell_texts(&row(page, COLUMNS, "Games").await?).await?
        );
        Ok::<(), Report>(())
    };
    expect_headers(vec!["", "Name", "Type", "Date Modified"]).await?;
    expect_cells(vec!["", "Games", "File folder", "6/7/2020"]).await?;

    page.click_element_with_id("test-ts-hide-type").await?;
    expect_headers(vec!["", "Name", "Date Modified"]).await?;
    expect_cells(vec!["", "Games", "6/7/2020"]).await?;
    // The kept date cell is the row's third cell now: navigation reaches it.
    let games = row(page, COLUMNS, "Games").await?;
    page.driver
        .execute("arguments[0].focus()", vec![games.to_json()?])
        .await?;
    page.wait_for_focus_on(&games, "the Games row").await?;
    page.send_keys_to_active(Key::Left).await?;
    page.wait_for_focus("gridcell", Some("6/7/2020")).await?;
    page.send_keys_to_active(Key::Left).await?;
    page.wait_for_focus("rowheader", Some("Games")).await?;

    page.click_element_with_id("test-ts-hide-type").await?;
    expect_headers(vec!["", "Name", "Type", "Date Modified"]).await?;
    expect_cells(vec!["", "Games", "File folder", "6/7/2020"]).await?;

    page.click_element_with_id("test-ts-rename-type").await?;
    expect_headers(vec!["", "Name", "Kind", "Date Modified"]).await?;

    page.click_element_with_id("test-ts-move-type").await?;
    expect_headers(vec!["", "Name", "Date Modified", "Kind"]).await?;
    expect_cells(vec!["", "Games", "6/7/2020", "File folder"]).await?;
    // The two swapped cells swapped their keys: navigation reaches both.
    let games = row(page, COLUMNS, "Games").await?;
    page.driver
        .execute("arguments[0].focus()", vec![games.to_json()?])
        .await?;
    page.wait_for_focus_on(&games, "the Games row").await?;
    page.send_keys_to_active(Key::Left).await?;
    page.wait_for_focus("gridcell", Some("File folder")).await?;
    page.send_keys_to_active(Key::Left).await?;
    page.wait_for_focus("gridcell", Some("6/7/2020")).await?;
    page.send_keys_to_active(Key::Left).await?;
    page.wait_for_focus("rowheader", Some("Games")).await?;

    // Sortability follows the column ("should support column hover when sorting is allowed",
    // "should not show column hover state when column is not sortable").
    let date = table(page, COLUMNS)
        .await?
        .find(By::XPath(
            ".//*[@role='columnheader'][normalize-space(.)='Date Modified']",
        ))
        .await?;
    assert_that!(date.attr("aria-sort").await?).is_none();
    assert_that!(date.attr("data-allows-sorting").await?).is_none();
    hover(page, &date).await?;
    stays!(
        "data-hovered of the date",
        None,
        date.attr("data-hovered").await?
    );
    page.click_element_with_id("test-ts-sort-date").await?;
    page.wait_for_attr(&date, "aria-sort", Some("none")).await?;
    page.wait_for_attr(&date, "data-allows-sorting", Some("true"))
        .await?;
    hover(page, &date).await?;
    page.wait_for_attr(&date, "data-hovered", Some("true"))
        .await?;
    page.click_element_with_id("test-ts-sort-date").await?;
    page.wait_for_attr(&date, "aria-sort", None).await?;
    page.wait_for_attr(&date, "data-allows-sorting", None)
        .await?;

    // Select all only in multiple selection mode.
    let select_all = "[role=grid][aria-label='Columns table'] thead input[type=checkbox]";
    assert_that!(page.count_matching(select_all).await?).is_equal_to(1);
    page.click_element_with_id("test-ts-single").await?;
    wait_for!("select all checkboxes", 0, page.count_matching(select_all).await?);
    page.click_element_with_id("test-ts-single").await?;
    wait_for!("select all checkboxes", 1, page.count_matching(select_all).await?);
    Ok(())
}

async fn hover(page: &Page<'_>, element: &WebElement) -> Result<(), Report> {
    page.driver
        .action_chain()
        .move_to_element_center(element)
        .perform()
        .await?;
    Ok(())
}

/// `data-hovered` on interactive rows and their cells, and
/// `data-focus-visible` on cells focused by keyboard (react-aria-components' `Row`, `Cell` and
/// `Column` render states).
async fn hover_and_focus_states(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let games = row(page, "Replace table", "Games").await?;
    let name_cell = games.find(By::Css("[role=rowheader]")).await?;
    hover(page, &name_cell).await?;
    page.wait_for_attr(&games, "data-hovered", Some("true"))
        .await?;
    page.wait_for_attr(&name_cell, "data-hovered", Some("true"))
        .await?;
    let action_row = row(page, "Action table", "bootmgr").await?;
    hover(page, &action_row).await?;
    page.wait_for_attr(&action_row, "data-hovered", Some("true"))
        .await?;
    page.wait_for_attr(&games, "data-hovered", None).await?;
    page.wait_for_attr(&name_cell, "data-hovered", None).await?;

    // Keyboard focus on a cell (and a column header): focus rings.
    page.click_element_with_id("test-ts-before-escape").await?;
    page.press_tab().await?;
    page.wait_for_focus("row", None).await?;
    page.send_keys_to_active(Key::Right).await?;
    page.send_keys_to_active(Key::Right).await?;
    page.wait_for_focus("rowheader", Some("Games")).await?;
    let focused = page.driver.active_element().await?;
    page.wait_for_attr(&focused, "data-focus-visible", Some("true"))
        .await?;
    page.send_keys_to_active(Key::Up).await?;
    page.wait_for_focus("columnheader", Some("Name")).await?;
    let focused = page.driver.active_element().await?;
    page.wait_for_attr(&focused, "data-focus-visible", Some("true"))
        .await
}

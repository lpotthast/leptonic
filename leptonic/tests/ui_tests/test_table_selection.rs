// Upstream: react-aria-components/test/Table.test.js @ 99e6102368
//! Selection and actions of the table atoms: `selectionBehavior="replace"` (mouse and
//! keyboard), `escapeKeyBehavior="none"`, `shouldSelectOnPressUp`, row actions, and columns that
//! change while rows stay ("supports removing a column and adding it back", plus renaming,
//! reordering, sortability and the selection mode). Spec: react-aria-components
//! `Table.test.js`.
use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::Report;

use crate::pages::{ElementActions, Page, css, role};

const PATH: &str = "/atoms/table-selection";

async fn table(page: &Page<'_>, label: &str) -> Result<WebElement, Report> {
    page.element(format!("[role=grid][aria-label='{label}']"))
        .await
}

/// The body row of the table `label` whose row header is `name`.
async fn row(page: &Page<'_>, label: &str, name: &str) -> Result<WebElement, Report> {
    table(page, label)
        .await?
        .element(css("tbody > [role=row]").has(role(AriaRole::Rowheader).text(name)))
        .await
}

/// The texts of the column headers of the table `label`.
async fn column_headers(page: &Page<'_>, label: &str) -> Result<Vec<String>, Report> {
    table(page, label)
        .await?
        .inner_texts("[role=columnheader]")
        .await
}

/// The cell or row header with `text` in `row`.
async fn cell_in(row: &WebElement, text: &str) -> Result<WebElement, Report> {
    row.element(css("[role=gridcell], [role=rowheader]").text(text))
        .await
}

/// The column header with `text` of the table `label`.
async fn column_header(page: &Page<'_>, label: &str, text: &str) -> Result<WebElement, Report> {
    table(page, label)
        .await?
        .element(role(AriaRole::Columnheader).text(text))
        .await
}

async fn click_with_control(page: &Page<'_>, element: &WebElement) -> Result<(), Report> {
    page.low_level()
        .driver()
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
    page.element(format!("#test-ts-{id}-selection"))
        .await?
        .wait_for_inner_text(expected)
        .await?;
    page.element(format!("#test-ts-{id}-changes"))
        .await?
        .wait_for_inner_text(&changes.to_string())
        .await?;
    Ok(())
}

/// Negative check: the selection of the table `id` is `expected` after `changes` selection
/// changes, and stays so.
async fn selection_stays(
    page: &Page<'_>,
    id: &str,
    expected: &str,
    changes: usize,
) -> Result<(), Report> {
    page.element(format!("#test-ts-{id}-selection"))
        .await?
        .inner_text_stays(expected, std::time::Duration::from_millis(100))
        .await?;
    page.element(format!("#test-ts-{id}-changes"))
        .await?
        .inner_text_stays(&changes.to_string(), std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// With replace selection there are no checkboxes, and clicking a row replaces the selection while
/// Ctrl+click toggles the row ("should not render checkboxes for selection with
/// selectionBehavior=replace", "should perform replace selection in highlight mode when not using
/// modifier keys", "should perform toggle selection in highlight mode when using modifier keys").
#[browser_test]
pub async fn replace_selection_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    const REPLACE: &str = "Replace table";
    page.goto_path(PATH).await?;
    let checkboxes = page
        .count("[role=grid][aria-label='Replace table'] input[type=checkbox]")
        .await?;
    assert_that!(checkboxes).is_zero();

    let bootmgr = row(page, REPLACE, "bootmgr").await?;
    let program_files = row(page, REPLACE, "Program Files").await?;
    assert_that!(bootmgr)
        .has_attribute("aria-selected")
        .await
        .is_equal_to("false");
    bootmgr.click().await?;
    expect_selection(page, "replace", "3", 1).await?;
    bootmgr.wait_for_attr("data-selected", Some("true")).await?;
    // Without modifiers: replaced.
    program_files.click().await?;
    expect_selection(page, "replace", "2", 2).await?;
    bootmgr
        .wait_for_attr("aria-selected", Some("false"))
        .await?;
    // Pressing it again doesn't deselect it.
    program_files.click().await?;
    selection_stays(page, "replace", "2", 2).await?;
    // With a modifier: toggled.
    click_with_control(page, &bootmgr).await?;
    expect_selection(page, "replace", "2,3", 3).await?;
    click_with_control(page, &program_files).await?;
    expect_selection(page, "replace", "3", 4).await?;
    Ok(())
}

/// With single replace selection, Ctrl+click selects a row in place of the selected one and
/// Ctrl+click on the selected row deselects it ("should perform selection with single selection").
#[browser_test]
pub async fn replace_selection_in_single_mode(page: &Page<'_>) -> Result<(), Report> {
    const SINGLE: &str = "Single replace table";
    page.goto_path(PATH).await?;
    let bootmgr = row(page, SINGLE, "bootmgr").await?;
    let program_files = row(page, SINGLE, "Program Files").await?;
    click_with_control(page, &bootmgr).await?;
    expect_selection(page, "single-replace", "3", 1).await?;
    click_with_control(page, &program_files).await?;
    expect_selection(page, "single-replace", "2", 2).await?;
    click_with_control(page, &program_files).await?;
    expect_selection(page, "single-replace", "", 3).await?;
    Ok(())
}

/// Replace selection follows keyboard focus from the first row tabbed into on, and Shift+ArrowDown
/// extends it ("should perform replace selection in highlight mode when not using modifier keys").
#[browser_test]
pub async fn replace_selection_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-ts-before-replace")
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    let games = row(page, "Replace table", "Games").await?;
    page.wait_for_focus(&games).await?;
    // Focused by keyboard: a focus ring.
    games
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    let selection = page.element("#test-ts-replace-selection").await?;
    selection.wait_for_inner_text("1").await?;
    page.send_keys(Key::Down).await?;
    selection.wait_for_inner_text("2").await?;
    page.send_keys(Key::Shift + Key::Down).await?;
    selection.wait_for_inner_text("2,3").await?;
    Ok(())
}

/// With `escapeKeyBehavior` "none", Escape keeps the selection ("should prevent Esc from clearing
/// selection if escapeKeyBehavior is "none"").
#[browser_test]
pub async fn escape_without_clearing(page: &Page<'_>) -> Result<(), Report> {
    const ESCAPE: &str = "Escape table";
    page.goto_path(PATH).await?;
    for (name, expected, changes) in [("Games", "1", 1), ("Program Files", "1,2", 2)] {
        row(page, ESCAPE, name)
            .await?
            .element("input[type=checkbox]")
            .await?
            .click()
            .await?;
        expect_selection(page, "escape", expected, changes).await?;
    }
    page.send_keys(Key::Escape).await?;
    selection_stays(page, "escape", "1,2", 2).await?;
    Ok(())
}

/// A row is selected on pointer down, or on pointer up with `shouldSelectOnPressUp`, where a
/// draggable row keeps focus over its pressed cell ("should select an item on pressing down when
/// shouldSelectOnPressUp is not provided", "should select an item on pressing up when
/// shouldSelectOnPressUp is true").
#[browser_test]
pub async fn select_on_press_down_or_up(page: &Page<'_>) -> Result<(), Report> {
    const UP: &str = "Press up table";
    page.goto_path(PATH).await?;
    let down = row(page, "Press down table", "Games").await?;
    let held = down.press_and_hold().await?;
    expect_selection(page, "press-down", "1", 1).await?;
    held.release().await?;
    selection_stays(page, "press-down", "1", 1).await?;

    let up = row(page, UP, "Games").await?;
    let held = up.press_and_hold().await?;
    selection_stays(page, "press-up", "", 0).await?;
    held.release().await?;
    expect_selection(page, "press-up", "1", 1).await?;

    // A draggable row: the browser's default focus on pointer down skips the pressed cell.
    let program_files = row(page, UP, "Program Files").await?;
    page.low_level()
        .eval::<()>(
            "arguments[0].setAttribute('draggable', 'true');",
            vec![program_files.to_json()?],
        )
        .await?;
    let type_cell = program_files
        .element(role(AriaRole::Gridcell).text("File folder"))
        .await?;
    let held = type_cell.press_and_hold().await?;
    page.wait_for_focus(&program_files).await?;
    held.release().await?;
    expect_selection(page, "press-up", "2", 2).await?;
    page.wait_for_focus(&program_files).await?;
    // The cell gets its tabindex back.
    type_cell.wait_for_attr("tabindex", Some("-1")).await?;
    Ok(())
}

/// A row is pressed while the pointer is down, and pressing it or Enter on it runs its action
/// ("should support row actions").
#[browser_test]
pub async fn row_actions(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let games = row(page, "Action table", "Games").await?;
    assert_that!(games)
        .attribute("data-pressed")
        .await
        .is_none();
    let held = games.press_and_hold().await?;
    games.wait_for_attr("data-pressed", Some("true")).await?;
    held.release().await?;
    games.wait_for_attr("data-pressed", None).await?;
    let action = page.element("#test-ts-action").await?;
    let action_count = page.element("#test-ts-action-count").await?;
    action.wait_for_inner_text("1").await?;
    action_count.wait_for_inner_text("1").await?;

    page.send_keys(Key::Down).await?;
    page.send_keys(Key::Enter).await?;
    action.wait_for_inner_text("2").await?;
    action_count.wait_for_inner_text("2").await?;
    Ok(())
}

/// Headers, cells and arrow-key navigation follow columns that are hidden and shown again, renamed,
/// moved or made sortable, and "select all" follows the selection mode ("supports removing a
/// column and adding it back static").
#[browser_test]
pub async fn changing_columns(page: &Page<'_>) -> Result<(), Report> {
    const COLUMNS: &str = "Columns table";
    page.goto_path(PATH).await?;
    let expect_headers = |expected: Vec<&'static str>| async move {
        assert_that!(|| column_headers(page, COLUMNS))
            .eventually_ok()
            .matches(eq(expected))
            .await;
        Ok::<(), Report>(())
    };
    let expect_cells = |expected: Vec<&'static str>| async move {
        assert_that!(|| async { row(page, COLUMNS, "Games").await?.inner_texts("td").await })
            .eventually_ok()
            .matches(eq(expected.as_slice()))
            .await;
        Ok::<(), Report>(())
    };
    expect_headers(vec!["", "Name", "Type", "Date Modified"]).await?;
    expect_cells(vec!["", "Games", "File folder", "6/7/2020"]).await?;

    page.element("#test-ts-hide-type").await?.click().await?;
    expect_headers(vec!["", "Name", "Date Modified"]).await?;
    expect_cells(vec!["", "Games", "6/7/2020"]).await?;
    // The kept date cell is the row's third cell now: navigation reaches it.
    let games = row(page, COLUMNS, "Games").await?;
    games.focus().await?;
    page.wait_for_focus(&games).await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&cell_in(&games, "6/7/2020").await?)
        .await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&cell_in(&games, "Games").await?)
        .await?;

    page.element("#test-ts-hide-type").await?.click().await?;
    expect_headers(vec!["", "Name", "Type", "Date Modified"]).await?;
    expect_cells(vec!["", "Games", "File folder", "6/7/2020"]).await?;

    page.element("#test-ts-rename-type").await?.click().await?;
    expect_headers(vec!["", "Name", "Kind", "Date Modified"]).await?;

    page.element("#test-ts-move-type").await?.click().await?;
    expect_headers(vec!["", "Name", "Date Modified", "Kind"]).await?;
    expect_cells(vec!["", "Games", "6/7/2020", "File folder"]).await?;
    // The two swapped cells swapped their keys: navigation reaches both.
    let games = row(page, COLUMNS, "Games").await?;
    games.focus().await?;
    page.wait_for_focus(&games).await?;
    for text in ["File folder", "6/7/2020", "Games"] {
        page.send_keys(Key::Left).await?;
        page.wait_for_focus(&cell_in(&games, text).await?).await?;
    }

    // Sortability follows the column ("should support column hover when sorting is allowed",
    // "should not show column hover state when column is not sortable").
    let date = column_header(page, COLUMNS, "Date Modified").await?;
    assert_that!(date).attribute("aria-sort").await.is_none();
    assert_that!(date)
        .attribute("data-allows-sorting")
        .await
        .is_none();
    date.hover().await?;
    date.attr_stays("data-hovered", None, std::time::Duration::from_millis(100))
        .await?;
    page.element("#test-ts-sort-date").await?.click().await?;
    date.wait_for_attr("aria-sort", Some("none")).await?;
    date.wait_for_attr("data-allows-sorting", Some("true"))
        .await?;
    date.hover().await?;
    date.wait_for_attr("data-hovered", Some("true")).await?;
    page.element("#test-ts-sort-date").await?.click().await?;
    date.wait_for_attr("aria-sort", None).await?;
    date.wait_for_attr("data-allows-sorting", None).await?;

    // Select all only in multiple selection mode.
    let select_all = "[role=grid][aria-label='Columns table'] thead input[type=checkbox]";
    assert_that!(page.count(select_all).await?).is_equal_to(1);
    page.element("#test-ts-single").await?.click().await?;
    page.wait_for_count(select_all, 0).await?;
    page.element("#test-ts-single").await?.click().await?;
    page.wait_for_count(select_all, 1).await?;
    Ok(())
}

/// In single selection mode, Ctrl+A selects nothing ("should not select all with Mod+A when
/// selection mode is single").
#[browser_test]
pub async fn select_all_shortcut_in_single_mode(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let games = row(page, "Press down table", "Games").await?;
    games.focus().await?;
    page.wait_for_focus(&games).await?;
    page.send_keys(Key::Control + "a").await?;
    page.element("#test-ts-press-down-changes")
        .await?
        .inner_text_stays("0", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// The selection checkbox column shows only while rows can be selected: switching the selection
/// mode to none removes the column and every row's checkbox cell, switching back restores them
/// (react-stately's `useTableState`: `showSelectionCheckboxes && selectionMode !== 'none'`).
#[browser_test]
pub async fn the_selection_column_follows_the_selection_mode(
    page: &Page<'_>,
) -> Result<(), Report> {
    const COLUMNS: &str = "Columns table";
    page.goto_path(PATH).await?;
    let expect = |headers: Vec<&'static str>, cells: Vec<&'static str>| async move {
        assert_that!(|| column_headers(page, COLUMNS))
            .eventually_ok()
            .matches(eq(headers))
            .await;
        assert_that!(|| async { row(page, COLUMNS, "Games").await?.inner_texts("td").await })
            .eventually_ok()
            .matches(eq(cells.as_slice()))
            .await;
        Ok::<(), Report>(())
    };
    expect(
        vec!["", "Name", "Type", "Date Modified"],
        vec!["", "Games", "File folder", "6/7/2020"],
    )
    .await?;
    let toggle = page.element("#test-ts-no-selection").await?;
    toggle.click().await?;
    expect(
        vec!["Name", "Type", "Date Modified"],
        vec!["Games", "File folder", "6/7/2020"],
    )
    .await?;
    assert_that!(
        page.count(css("[aria-label='Columns table'] input[type=checkbox]"))
            .await?
    )
    .is_equal_to(0);
    toggle.click().await?;
    expect(
        vec!["", "Name", "Type", "Date Modified"],
        vec!["", "Games", "File folder", "6/7/2020"],
    )
    .await?;
    Ok(())
}

/// Removing the focused column header moves the focus to the same column of the first row
/// (react-stately's `useGridState`: a removed column header refocuses the nearest row's cell).
#[browser_test]
pub async fn removing_the_focused_column_header(page: &Page<'_>) -> Result<(), Report> {
    const COLUMNS: &str = "Columns table";
    page.goto_path(PATH).await?;
    let games = row(page, COLUMNS, "Games").await?;
    games.focus().await?;
    page.wait_for_focus(&games).await?;
    // Games' type cell, then up to the "Type" header.
    page.send_keys(Key::Right).await?;
    page.send_keys(Key::Right).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&cell_in(&games, "File folder").await?)
        .await?;
    page.send_keys(Key::Up).await?;
    page.wait_for_focus(&column_header(page, COLUMNS, "Type").await?)
        .await?;
    // Hide the column from script, as an app would: the focus stays in the table.
    page.element("#test-ts-hide-type")
        .await?
        .virtual_click()
        .await?;
    // Games is the first row: its cell now in the Type column's place.
    page.wait_for_focus(&cell_in(&games, "6/7/2020").await?)
        .await?;
    Ok(())
}

/// Hovering an interactive row marks it and its cells `data-hovered`, and cells and column headers
/// focused by keyboard get `data-focus-visible` ("should support hover", "should support focus
/// ring").
#[browser_test]
pub async fn hover_and_focus_states(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let games = row(page, "Replace table", "Games").await?;
    let name_cell = games.element("[role=rowheader]").await?;
    name_cell.hover().await?;
    games.wait_for_attr("data-hovered", Some("true")).await?;
    name_cell
        .wait_for_attr("data-hovered", Some("true"))
        .await?;
    let action_row = row(page, "Action table", "bootmgr").await?;
    action_row.hover().await?;
    action_row
        .wait_for_attr("data-hovered", Some("true"))
        .await?;
    games.wait_for_attr("data-hovered", None).await?;
    name_cell.wait_for_attr("data-hovered", None).await?;

    // Keyboard focus on a cell (and a column header): focus rings.
    let escape_games = row(page, "Escape table", "Games").await?;
    let games_header = cell_in(&escape_games, "Games").await?;
    let name_header = column_header(page, "Escape table", "Name").await?;
    page.element("#test-ts-before-escape")
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&escape_games).await?;
    page.send_keys(Key::Right).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&games_header).await?;
    games_header
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    page.send_keys(Key::Up).await?;
    page.wait_for_focus(&name_header).await?;
    name_header
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    Ok(())
}

// Upstream: react-aria/test/grid/useGrid.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// Behavior of the grid hooks (through the `Grid` atoms): focus movement in every combination of
/// grid focus mode (row/cell) and cell focus mode (cell/child), restoring the last focused child
/// of a cell, two-dimensional navigation with disabled rows and column spans, and row selection.
/// Spec: react-aria `useGrid.test.js`; the multi-column checks follow `GridKeyboardDelegate`.
pub struct GridTests {}

#[async_trait]
impl BrowserTest<str> for GridTests {
    fn name(&self) -> Cow<'_, str> {
        "grid_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/grid").await?;

        aria_structure(&page).await?;
        row_focus_cell_focus(&page).await?;
        row_focus_child_focus(&page).await?;
        cell_focus_child_focus(&page).await?;
        cell_focus_cell_focus(&page).await?;
        restores_the_last_focused_child(&page).await?;
        focusing_a_child_from_outside_keeps_it(&page).await?;
        two_dimensional_navigation(&page).await?;
        row_selection(&page).await?;
        cell_focus_mode_selects_rows(&page).await?;
        cell_actions(&page).await?;

        Ok(())
    }
}

/// The rows, cells and switches of the switch grid labelled `label`.
struct SwitchGrid {
    rows: Vec<WebElement>,
    cells: Vec<WebElement>,
    switches: Vec<WebElement>,
}

async fn switch_grid(page: &Page<'_>, label: &str) -> Result<SwitchGrid, Report> {
    let grid = page
        .css(&format!("[role=grid][aria-label='{label}']"))
        .await?;
    Ok(SwitchGrid {
        rows: grid.find_all(By::Css("[role=row]")).await?,
        cells: grid.find_all(By::Css("[role=gridcell]")).await?,
        switches: grid.find_all(By::Css("[role=switch]")).await?,
    })
}

/// The row of the "Users" grid whose first cell is `name`.
async fn row(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    page.driver
        .find(By::XPath(format!(
            "//*[@role='row'][.//*[@role='gridcell'][normalize-space(.)='{name}']]"
        )))
        .await
        .map_err(Into::into)
}

/// The cell with `text` in the row of `name`.
async fn cell(page: &Page<'_>, name: &str, text: &str) -> Result<WebElement, Report> {
    row(page, name)
        .await?
        .find(By::XPath(format!(
            ".//*[@role='gridcell'][normalize-space(.)='{text}']"
        )))
        .await
        .map_err(Into::into)
}

async fn expect_focus_on_row(page: &Page<'_>, name: &str) -> Result<(), Report> {
    let row = row(page, name).await?;
    page.wait_for_focus_on(&row, &format!("row {name}")).await
}

async fn expect_focus_on_cell(page: &Page<'_>, name: &str, text: &str) -> Result<(), Report> {
    let cell = cell(page, name, text).await?;
    page.wait_for_focus_on(&cell, &format!("cell {text} of {name}"))
        .await
}

async fn attr(element: &WebElement, name: &str) -> Result<Option<String>, Report> {
    Ok(element.attr(name).await?)
}

async fn focus(page: &Page<'_>, element: &WebElement) -> Result<(), Report> {
    page.driver
        .execute("arguments[0].focus()", vec![element.to_json()?])
        .await?;
    Ok(())
}

async fn press(page: &Page<'_>, key: Key) -> Result<(), Report> {
    page.send_keys_to_active(key).await
}

async fn aria_structure(page: &Page<'_>) -> Result<(), Report> {
    let users = page.css("[role=grid][aria-label='Users']").await?;
    assert_that!(attr(&users, "aria-multiselectable").await?).is_equal_to(Some("true".to_owned()));
    let groups = users.find_all(By::Css("[role=rowgroup]")).await?;
    assert_that!(groups.len()).is_equal_to(1);

    let alice = row(page, "Alice").await?;
    assert_that!(attr(&alice, "aria-selected").await?).is_equal_to(Some("false".to_owned()));
    let bob = row(page, "Bob").await?;
    assert_that!(attr(&bob, "aria-disabled").await?).is_equal_to(Some("true".to_owned()));

    let on_leave = cell(page, "Carol", "On leave").await?;
    assert_that!(attr(&on_leave, "aria-colspan").await?).is_equal_to(Some("2".to_owned()));
    assert_that!(attr(&on_leave, "aria-colindex").await?).is_equal_to(Some("2".to_owned()));
    Ok(())
}

async fn row_focus_cell_focus(page: &Page<'_>) -> Result<(), Report> {
    let g = switch_grid(page, "Row-Cell").await?;
    page.click_element_with_id("test-grid-before").await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&g.rows[0], "Row-Cell: row 1")
        .await?;
    // Keyboard focus is visible on rows and cells.
    page.wait_for_attr(&g.rows[0], "data-focus-visible", Some("true"))
        .await?;

    press(page, Key::Right).await?;
    page.wait_for_focus_on(&g.cells[0], "Row-Cell: cell 1")
        .await?;
    page.wait_for_attr(&g.cells[0], "data-focused", Some("true"))
        .await?;
    page.wait_for_attr(&g.cells[0], "data-focus-visible", Some("true"))
        .await?;
    page.wait_for_attr(&g.rows[0], "data-focus-visible", None)
        .await?;
    press(page, Key::Right).await?;
    page.wait_for_focus_on(&g.switches[0], "Row-Cell: switch 1")
        .await?;
    press(page, Key::Right).await?;
    page.wait_for_focus_on(&g.switches[1], "Row-Cell: switch 2")
        .await?;
    press(page, Key::Right).await?;
    page.wait_for_focus_on(&g.rows[0], "Row-Cell: row 1")
        .await?;

    press(page, Key::Left).await?;
    page.wait_for_focus_on(&g.cells[0], "Row-Cell: cell 1")
        .await?;

    focus(page, &g.switches[1]).await?;
    press(page, Key::Left).await?;
    page.wait_for_focus_on(&g.switches[0], "Row-Cell: switch 1")
        .await?;
    press(page, Key::Left).await?;
    page.wait_for_focus_on(&g.cells[0], "Row-Cell: cell 1")
        .await?;
    press(page, Key::Left).await?;
    page.wait_for_focus_on(&g.rows[0], "Row-Cell: row 1").await
}

async fn row_focus_child_focus(page: &Page<'_>) -> Result<(), Report> {
    let g = switch_grid(page, "Row-Child").await?;
    page.click_element_with_id("test-grid-before-row-child")
        .await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&g.rows[0], "Row-Child: row 1")
        .await?;

    press(page, Key::Right).await?;
    page.wait_for_focus_on(&g.switches[0], "Row-Child: switch 1")
        .await?;
    press(page, Key::Right).await?;
    page.wait_for_focus_on(&g.switches[1], "Row-Child: switch 2")
        .await?;
    press(page, Key::Right).await?;
    page.wait_for_focus_on(&g.rows[0], "Row-Child: row 1")
        .await?;

    press(page, Key::Left).await?;
    page.wait_for_focus_on(&g.switches[1], "Row-Child: switch 2")
        .await?;
    press(page, Key::Left).await?;
    page.wait_for_focus_on(&g.switches[0], "Row-Child: switch 1")
        .await?;
    press(page, Key::Left).await?;
    page.wait_for_focus_on(&g.rows[0], "Row-Child: row 1")
        .await?;
    press(page, Key::Left).await?;
    page.wait_for_focus_on(&g.switches[1], "Row-Child: switch 2")
        .await
}

async fn cell_focus_child_focus(page: &Page<'_>) -> Result<(), Report> {
    let g = switch_grid(page, "Cell-Child").await?;
    page.click_element_with_id("test-grid-before-cell-child")
        .await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&g.switches[0], "Cell-Child: switch 1")
        .await?;

    press(page, Key::Right).await?;
    page.wait_for_focus_on(&g.switches[1], "Cell-Child: switch 2")
        .await?;
    press(page, Key::Right).await?;
    page.wait_for_focus_on(&g.switches[0], "Cell-Child: switch 1")
        .await?;

    press(page, Key::Left).await?;
    page.wait_for_focus_on(&g.switches[1], "Cell-Child: switch 2")
        .await?;
    press(page, Key::Left).await?;
    page.wait_for_focus_on(&g.switches[0], "Cell-Child: switch 1")
        .await?;
    press(page, Key::Left).await?;
    page.wait_for_focus_on(&g.switches[1], "Cell-Child: switch 2")
        .await
}

async fn cell_focus_cell_focus(page: &Page<'_>) -> Result<(), Report> {
    let g = switch_grid(page, "Cell-Cell").await?;
    page.click_element_with_id("test-grid-before-cell-cell")
        .await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&g.cells[0], "Cell-Cell: cell 1")
        .await?;

    press(page, Key::Right).await?;
    page.wait_for_focus_on(&g.switches[0], "Cell-Cell: switch 1")
        .await?;
    press(page, Key::Right).await?;
    page.wait_for_focus_on(&g.switches[1], "Cell-Cell: switch 2")
        .await?;
    press(page, Key::Right).await?;
    page.wait_for_focus_on(&g.cells[0], "Cell-Cell: cell 1")
        .await?;

    press(page, Key::Left).await?;
    page.wait_for_focus_on(&g.switches[1], "Cell-Cell: switch 2")
        .await?;
    press(page, Key::Left).await?;
    page.wait_for_focus_on(&g.switches[0], "Cell-Cell: switch 1")
        .await?;
    press(page, Key::Left).await?;
    page.wait_for_focus_on(&g.cells[0], "Cell-Cell: cell 1")
        .await
}

/// A cell child focused from outside the grid keeps focus, as when a dialog opened from a row's
/// button restores focus to it (crudkit): after a mouse press on the child (the cell becomes the
/// focused key) and with pointer or keyboard modality (closing the dialog with Escape).
async fn focusing_a_child_from_outside_keeps_it(page: &Page<'_>) -> Result<(), Report> {
    let g = switch_grid(page, "Row-Child").await?;
    let target = &g.switches[4];
    target.click().await?;
    page.wait_for_focus_on(target, "Row-Child: switch 5 after clicking it")
        .await?;

    for keyboard in [false, true] {
        page.click_element_with_id("test-grid-before-row-child")
            .await?;
        if keyboard {
            page.send_keys_to_active(Key::Shift).await?;
        }
        focus(page, target).await?;
        // Focus must stay: give deferred focus handling (microtasks, frames) time to run.
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        let what = if keyboard {
            "Row-Child: switch 5 refocused (keyboard modality)"
        } else {
            "Row-Child: switch 5 refocused (pointer modality)"
        };
        page.wait_for_focus_on(target, what).await?;
    }
    Ok(())
}

/// Focusing a cell (in child focus mode) restores focus to the child that was focused last,
/// not the first child: when tabbing back into the grid, and when the cell itself is focused.
/// The previous check left focus on the second switch.
async fn restores_the_last_focused_child(page: &Page<'_>) -> Result<(), Report> {
    let g = switch_grid(page, "Cell-Child").await?;
    page.click_element_with_id("test-grid-before-cell-child")
        .await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&g.switches[1], "restore Cell-Child: switch 2 (tabbing in)")
        .await?;

    focus(page, &g.cells[0]).await?;
    page.wait_for_focus_on(
        &g.switches[1],
        "restore Cell-Child: switch 2 (focusing the cell)",
    )
    .await
}

/// Up/Down keep the column (respecting column spans) and skip the disabled row; Home/End stay in
/// the row, Ctrl+Home/End go to the first/last row.
async fn two_dimensional_navigation(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-grid-before-users").await?;
    page.press_tab().await?;
    expect_focus_on_row(page, "Alice").await?;

    press(page, Key::Down).await?;
    expect_focus_on_row(page, "Carol").await?;
    press(page, Key::Right).await?;
    expect_focus_on_cell(page, "Carol", "Carol").await?;
    press(page, Key::Right).await?;
    expect_focus_on_cell(page, "Carol", "On leave").await?;
    press(page, Key::Down).await?;
    expect_focus_on_cell(page, "Dave", "40").await?;
    press(page, Key::Up).await?;
    expect_focus_on_cell(page, "Carol", "On leave").await?;
    press(page, Key::Up).await?;
    expect_focus_on_cell(page, "Alice", "30").await?;

    press(page, Key::Home).await?;
    expect_focus_on_cell(page, "Alice", "Alice").await?;
    press(page, Key::End).await?;
    expect_focus_on_cell(page, "Alice", "Admin").await?;
    page.send_keys_to_active(Key::Control + Key::End).await?;
    expect_focus_on_cell(page, "Dave", "User").await?;
    page.send_keys_to_active(Key::Control + Key::Home).await?;
    expect_focus_on_cell(page, "Alice", "Alice").await
}

/// Rows are selected by Space and by pressing (a row, or any of its cells).
async fn row_selection(page: &Page<'_>) -> Result<(), Report> {
    // Focus is on Alice's first cell: Space selects her row.
    page.send_keys_to_active(Key::Space).await?;
    page.wait_for_text("test-grid-selection", "Alice").await?;
    let alice = row(page, "Alice").await?;
    assert_that!(attr(&alice, "aria-selected").await?).is_equal_to(Some("true".to_owned()));

    cell(page, "Dave", "40").await?.click().await?;
    page.wait_for_text("test-grid-selection", "Alice,Dave")
        .await?;

    // The disabled row can't be selected.
    cell(page, "Bob", "Bob").await?.click().await?;
    page.wait_for_text("test-grid-selection", "Alice,Dave")
        .await?;

    row(page, "Alice").await?.click().await?;
    page.wait_for_text("test-grid-selection", "Dave").await
}

/// The cell with `text` of the grid labelled `grid`.
async fn grid_cell(page: &Page<'_>, grid: &str, text: &str) -> Result<WebElement, Report> {
    page.css(&format!("[role=grid][aria-label='{grid}']"))
        .await?
        .find(By::XPath(format!(
            ".//*[@role='gridcell'][normalize-space(.)='{text}']"
        )))
        .await
        .map_err(Into::into)
}

/// In cell focus mode, cells can't be selected themselves (no cell selection): Space and presses
/// on a cell select its row.
async fn cell_focus_mode_selects_rows(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-grid-before-fruits")
        .await?;
    page.press_tab().await?;
    let apple = grid_cell(page, "Fruits", "Apple").await?;
    page.wait_for_focus_on(&apple, "Fruits: Apple").await?;

    page.send_keys_to_active(Key::Space).await?;
    page.wait_for_text("test-grid-fruits-selection", "Apple")
        .await?;

    grid_cell(page, "Fruits", "Yellow").await?.click().await?;
    page.wait_for_text("test-grid-fruits-selection", "Apple,Banana")
        .await
}

/// With `on_cell_action`, activating a cell runs the action with the cell's key instead of
/// selecting the row. Only Enter is an action key: Space on such a cell does nothing (cells can't
/// be selected, and the cell's press handling keeps the key from the row).
async fn cell_actions(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-grid-before-actions")
        .await?;
    page.press_tab().await?;
    let apple = grid_cell(page, "Actions", "Apple").await?;
    page.wait_for_focus_on(&apple, "Actions: Apple").await?;

    page.send_keys_to_active(Key::Space).await?;
    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_text("test-grid-actions-action", "Apple-0")
        .await?;
    press(page, Key::Right).await?;
    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_text("test-grid-actions-action", "Apple-1")
        .await?;

    grid_cell(page, "Actions", "Yellow").await?.click().await?;
    page.wait_for_text("test-grid-actions-action", "Banana-1")
        .await?;
    assert_that!(page.read_text_of("test-grid-actions-selection").await?)
        .is_equal_to(String::new());
    Ok(())
}

// Upstream: react-aria/test/grid/useGrid.test.js @ 99e6102368
//! Behavior of the grid hooks (through the `Grid` atoms): focus movement in every combination of
//! grid focus mode (row/cell) and cell focus mode (cell/child), restoring the last focused child
//! of a cell, two-dimensional navigation with disabled rows and column spans, and row selection.
//! Spec: react-aria `useGrid.test.js`; the multi-column checks follow `GridKeyboardDelegate`.
use assertr::prelude::*;
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::{Report, prelude::ResultExt};

use crate::pages::{ElementActions, Page, role};

const PATH: &str = "/atoms/grid";

/// The rows, cells and switches of the switch grid labelled `label`.
struct SwitchGrid {
    rows: Vec<WebElement>,
    cells: Vec<WebElement>,
    switches: Vec<WebElement>,
}

async fn switch_grid(page: &Page<'_>, label: &str) -> Result<SwitchGrid, Report> {
    let grid = page
        .element(format!("[role=grid][aria-label='{label}']"))
        .await?;
    Ok(SwitchGrid {
        rows: grid.elements("[role=row]").await?,
        cells: grid.elements("[role=gridcell]").await?,
        switches: grid.elements("[role=switch]").await?,
    })
}

/// The row of the "Users" grid whose first cell is `name`.
async fn row(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    page.element(role(AriaRole::Row).has(role(AriaRole::Gridcell).text(name)))
        .await
}

/// The cell with `text` in the row of `name`.
async fn cell(page: &Page<'_>, name: &str, text: &str) -> Result<WebElement, Report> {
    row(page, name)
        .await?
        .element(role(AriaRole::Gridcell).text(text))
        .await
}

async fn expect_focus_on_row(page: &Page<'_>, name: &str) -> Result<(), Report> {
    let row = row(page, name).await?;
    page.wait_for_focus(&row).await?;
    Ok(())
}

async fn expect_focus_on_cell(page: &Page<'_>, name: &str, text: &str) -> Result<(), Report> {
    let cell = cell(page, name, text).await?;
    page.wait_for_focus(&cell).await?;
    Ok(())
}

/// The grid is multiselectable with one row group, its rows have `aria-selected` (and
/// `aria-disabled` when disabled), and a cell spanning two columns has `aria-colspan` and its
/// `aria-colindex`.
#[browser_test]
pub async fn aria_structure(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let users = page.element("[role=grid][aria-label='Users']").await?;
    assert_that!(users)
        .has_attribute("aria-multiselectable")
        .await
        .is_equal_to("true");
    let groups = users.elements("[role=rowgroup]").await?;
    assert_that!(groups).has_length(1);

    let alice = row(page, "Alice").await?;
    assert_that!(alice)
        .has_attribute("aria-selected")
        .await
        .is_equal_to("false");
    let bob = row(page, "Bob").await?;
    assert_that!(bob)
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");

    let on_leave = cell(page, "Carol", "On leave").await?;
    assert_that!(on_leave)
        .has_attribute("aria-colspan")
        .await
        .is_equal_to("2");
    assert_that!(on_leave)
        .has_attribute("aria-colindex")
        .await
        .is_equal_to("2");
    Ok(())
}

/// In row focus mode with cell focus mode, Tab focuses the row and the arrow keys move from it to
/// its cell, through the cell's switches and back, in both directions. Only the focused row or cell
/// shows visible focus ("gridFocusMode = row, cellFocusMode = cell").
#[browser_test]
pub async fn row_focus_cell_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let g = switch_grid(page, "Row-Cell").await?;
    page.element("#test-grid-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&g.rows[0]).await?;
    // Keyboard focus is visible on rows and cells.
    g.rows[0]
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;

    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&g.cells[0]).await?;
    g.cells[0]
        .wait_for_attr("data-focused", Some("true"))
        .await?;
    g.cells[0]
        .wait_for_attr("data-focus-visible", Some("true"))
        .await?;
    g.rows[0].wait_for_attr("data-focus-visible", None).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&g.switches[0]).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&g.switches[1]).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&g.rows[0]).await?;

    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&g.cells[0]).await?;

    g.switches[1].focus().await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&g.switches[0]).await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&g.cells[0]).await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&g.rows[0]).await?;
    Ok(())
}

/// In row focus mode with child focus mode, Tab focuses the row and the arrow keys skip the cell,
/// moving through its switches and back to the row in both directions ("gridFocusMode = row,
/// cellFocusMode = child").
#[browser_test]
pub async fn row_focus_child_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let g = switch_grid(page, "Row-Child").await?;
    page.element("#test-grid-before-row-child")
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&g.rows[0]).await?;

    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&g.switches[0]).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&g.switches[1]).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&g.rows[0]).await?;

    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&g.switches[1]).await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&g.switches[0]).await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&g.rows[0]).await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&g.switches[1]).await?;
    Ok(())
}

/// In cell focus mode with child focus mode, Tab focuses the cell's first switch and the arrow keys
/// cycle through its switches in both directions ("gridFocusMode = cell, cellFocusMode = child").
#[browser_test]
pub async fn cell_focus_child_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let g = switch_grid(page, "Cell-Child").await?;
    page.element("#test-grid-before-cell-child")
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&g.switches[0]).await?;

    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&g.switches[1]).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&g.switches[0]).await?;

    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&g.switches[1]).await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&g.switches[0]).await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&g.switches[1]).await?;
    Ok(())
}

/// In cell focus mode with cell focus mode, Tab focuses the cell and the arrow keys move from it
/// through its switches and back, in both directions ("gridFocusMode = cell, cellFocusMode =
/// cell").
#[browser_test]
pub async fn cell_focus_cell_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let g = switch_grid(page, "Cell-Cell").await?;
    page.element("#test-grid-before-cell-cell")
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&g.cells[0]).await?;

    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&g.switches[0]).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&g.switches[1]).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&g.cells[0]).await?;

    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&g.switches[1]).await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&g.switches[0]).await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&g.cells[0]).await?;
    Ok(())
}

/// Focusing a cell's child from outside the grid, as a dialog restoring focus to a row's button
/// does, keeps focus on the child in pointer and in keyboard modality.
#[browser_test]
pub async fn focusing_a_child_from_outside_keeps_it(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let g = switch_grid(page, "Row-Child").await?;
    let target = &g.switches[4];
    target.click().await?;
    page.wait_for_focus(target).await?;

    for keyboard in [false, true] {
        let modality = if keyboard { "keyboard" } else { "pointer" };
        page.element("#test-grid-before-row-child")
            .await?
            .click()
            .await?;
        if keyboard {
            page.send_keys(Key::Shift).await?;
        }
        target.focus().await?;
        page.wait_for_focus(target)
            .await
            .context_with(|| format!("{modality} modality"))?;
        // Focus must stay: deferred focus handling (microtasks, frames) must not move it.
        page.focus_stays(target, std::time::Duration::from_millis(100))
            .await
            .context_with(|| format!("{modality} modality"))?;
    }
    Ok(())
}

/// In child focus mode, tabbing back into the grid or focusing the cell from outside focuses the
/// cell's last focused child, not its first ("should restore focus to the child that was last
/// focused within a cell, not the first child").
#[browser_test]
pub async fn restores_the_last_focused_child(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let g = switch_grid(page, "Cell-Child").await?;
    let before = page.element("#test-grid-before-cell-child").await?;
    before.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&g.switches[0]).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&g.switches[1]).await?;

    before.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&g.switches[1]).await?;

    // Upstream focuses the cell from its own child. In a browser that keeps focus on the cell
    // (`useGridCell`'s `onFocus` ignores focus from the cell's children; jsdom only passes through
    // its fake timers), so the cell is focused from outside the grid.
    before.click().await?;
    g.cells[0].focus().await?;
    page.wait_for_focus(&g.switches[1]).await?;
    Ok(())
}

/// Up and Down keep the column (respecting column spans) and skip the disabled row; Home and End
/// stay in the row, Ctrl+Home and Ctrl+End go to the first and last row.
#[browser_test]
pub async fn two_dimensional_navigation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-grid-before-users")
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    expect_focus_on_row(page, "Alice").await?;

    page.send_keys(Key::Down).await?;
    expect_focus_on_row(page, "Carol").await?;
    page.send_keys(Key::Right).await?;
    expect_focus_on_cell(page, "Carol", "Carol").await?;
    page.send_keys(Key::Right).await?;
    expect_focus_on_cell(page, "Carol", "On leave").await?;
    page.send_keys(Key::Down).await?;
    expect_focus_on_cell(page, "Dave", "40").await?;
    page.send_keys(Key::Up).await?;
    expect_focus_on_cell(page, "Carol", "On leave").await?;
    page.send_keys(Key::Up).await?;
    expect_focus_on_cell(page, "Alice", "30").await?;

    page.send_keys(Key::Home).await?;
    expect_focus_on_cell(page, "Alice", "Alice").await?;
    page.send_keys(Key::End).await?;
    expect_focus_on_cell(page, "Alice", "Admin").await?;
    page.send_keys(Key::Control + Key::End).await?;
    expect_focus_on_cell(page, "Dave", "User").await?;
    page.send_keys(Key::Control + Key::Home).await?;
    expect_focus_on_cell(page, "Alice", "Alice").await?;
    Ok(())
}

/// Space on a row's cell and clicks on a row or any of its cells toggle the row's selection, and
/// selectable rows show hover, while the disabled row can't be selected.
#[browser_test]
pub async fn row_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let selection = page.element("#test-grid-selection").await?;
    page.element("#test-grid-before-users")
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    expect_focus_on_row(page, "Alice").await?;
    page.send_keys(Key::Right).await?;
    expect_focus_on_cell(page, "Alice", "Alice").await?;
    // Focus is on Alice's first cell: Space selects her row.
    page.send_keys(Key::Space).await?;
    selection.wait_for_inner_text("Alice").await?;
    let alice = row(page, "Alice").await?;
    assert_that!(alice)
        .has_attribute("aria-selected")
        .await
        .is_equal_to("true");

    // Selectable rows show hover.
    let dave = row(page, "Dave").await?;
    let dave_age = cell(page, "Dave", "40").await?;
    dave_age.hover().await?;
    dave.wait_for_attr("data-hovered", Some("true")).await?;
    dave_age.click().await?;
    selection.wait_for_inner_text("Alice,Dave").await?;

    // The disabled row can't be selected.
    cell(page, "Bob", "Bob").await?.click().await?;
    selection
        .inner_text_stays("Alice,Dave", std::time::Duration::from_millis(100))
        .await?;

    alice.click().await?;
    selection.wait_for_inner_text("Dave").await?;
    Ok(())
}

/// The cell with `text` of the grid labelled `grid`.
async fn grid_cell(page: &Page<'_>, grid: &str, text: &str) -> Result<WebElement, Report> {
    page.element(format!("[role=grid][aria-label='{grid}']"))
        .await?
        .element(role(AriaRole::Gridcell).text(text))
        .await
}

/// In cell focus mode, Space on a cell and clicking a cell select the cell's row, since cells can't
/// be selected themselves.
#[browser_test]
pub async fn cell_focus_mode_selects_rows(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-grid-before-fruits")
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    let apple = grid_cell(page, "Fruits", "Apple").await?;
    page.wait_for_focus(&apple).await?;

    let selection = page.element("#test-grid-fruits-selection").await?;
    page.send_keys(Key::Space).await?;
    selection.wait_for_inner_text("Apple").await?;

    grid_cell(page, "Fruits", "Yellow").await?.click().await?;
    selection.wait_for_inner_text("Apple,Banana").await?;
    Ok(())
}

/// With `on_cell_action`, Enter on a cell and clicking a cell run the action with the cell's key
/// instead of selecting the row, while Space on a cell does nothing.
#[browser_test]
pub async fn cell_actions(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-grid-before-actions")
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    let apple = grid_cell(page, "Actions", "Apple").await?;
    page.wait_for_focus(&apple).await?;

    let action = page.element("#test-grid-actions-action").await?;
    let selection = page.element("#test-grid-actions-selection").await?;
    page.send_keys(Key::Space).await?;
    selection
        .inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    action
        .inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    page.send_keys(Key::Enter).await?;
    action.wait_for_inner_text("Apple-0").await?;
    page.send_keys(Key::Right).await?;
    page.send_keys(Key::Enter).await?;
    action.wait_for_inner_text("Apple-1").await?;

    grid_cell(page, "Actions", "Yellow").await?.click().await?;
    action.wait_for_inner_text("Banana-1").await?;
    selection
        .inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

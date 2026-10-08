// Upstream: react-aria/test/grid/useGrid.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::{Report, prelude::ResultExt};

use crate::pages::{ElementActions, Page, PageActions, xpath};

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

        cases!(
            aria_structure(&page),
            row_focus_cell_focus(&page),
            row_focus_child_focus(&page),
            cell_focus_child_focus(&page),
            cell_focus_cell_focus(&page),
            restores_the_last_focused_child(&page),
            focusing_a_child_from_outside_keeps_it(&page),
            two_dimensional_navigation(&page),
            row_selection(&page),
            cell_focus_mode_selects_rows(&page),
            cell_actions(&page),
        );

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
    page.element(xpath(format!(
        "//*[@role='row'][.//*[@role='gridcell'][normalize-space(.)='{name}']]"
    )))
    .await
}

/// The cell with `text` in the row of `name`.
async fn cell(page: &Page<'_>, name: &str, text: &str) -> Result<WebElement, Report> {
    row(page, name)
        .await?
        .element(xpath(format!(
            ".//*[@role='gridcell'][normalize-space(.)='{text}']"
        )))
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

async fn aria_structure(page: &Page<'_>) -> Result<(), Report> {
    let users = page.element("[role=grid][aria-label='Users']").await?;
    assert_that!(users.attr("aria-multiselectable").await?)
        .get_some()
        .is_equal_to("true");
    let groups = users.elements("[role=rowgroup]").await?;
    assert_that!(groups).has_length(1);

    let alice = row(page, "Alice").await?;
    assert_that!(alice.attr("aria-selected").await?)
        .get_some()
        .is_equal_to("false");
    let bob = row(page, "Bob").await?;
    assert_that!(bob.attr("aria-disabled").await?)
        .get_some()
        .is_equal_to("true");

    let on_leave = cell(page, "Carol", "On leave").await?;
    assert_that!(on_leave.attr("aria-colspan").await?)
        .get_some()
        .is_equal_to("2");
    assert_that!(on_leave.attr("aria-colindex").await?)
        .get_some()
        .is_equal_to("2");
    Ok(())
}

async fn row_focus_cell_focus(page: &Page<'_>) -> Result<(), Report> {
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

async fn row_focus_child_focus(page: &Page<'_>) -> Result<(), Report> {
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

async fn cell_focus_child_focus(page: &Page<'_>) -> Result<(), Report> {
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

async fn cell_focus_cell_focus(page: &Page<'_>) -> Result<(), Report> {
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

/// A cell child focused from outside the grid keeps focus, as when a dialog opened from a row's
/// button restores focus to it (crudkit): after a mouse press on the child (the cell becomes the
/// focused key) and with pointer or keyboard modality (closing the dialog with Escape).
async fn focusing_a_child_from_outside_keeps_it(page: &Page<'_>) -> Result<(), Report> {
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
        page.focus_stays(target)
            .await
            .context_with(|| format!("{modality} modality"))?;
    }
    Ok(())
}

/// Focusing a cell (in child focus mode) restores focus to the child that was focused last,
/// not the first child: when tabbing back into the grid, and when the cell itself is focused
/// from outside the grid. The previous check left focus on the second switch.
///
/// react-aria's version ("should restore focus to the child that was last focused within a
/// cell") focuses the cell from its own child. In a browser, that keeps focus on the cell
/// (`useGridCell`'s `onFocus` ignores focus coming from the cell's children, and setting the
/// already focused key again changes nothing); the jsdom test only passes because its fake
/// timers still hold the frame callback queued when tabbing in.
async fn restores_the_last_focused_child(page: &Page<'_>) -> Result<(), Report> {
    let g = switch_grid(page, "Cell-Child").await?;
    page.element("#test-grid-before-cell-child")
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&g.switches[1]).await?;

    page.element("#test-grid-before-cell-child")
        .await?
        .click()
        .await?;
    g.cells[0].focus().await?;
    page.wait_for_focus(&g.switches[1]).await?;
    Ok(())
}

/// Up/Down keep the column (respecting column spans) and skip the disabled row; Home/End stay in
/// the row, Ctrl+Home/End go to the first/last row.
async fn two_dimensional_navigation(page: &Page<'_>) -> Result<(), Report> {
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

/// Rows are selected by Space and by pressing (a row, or any of its cells).
async fn row_selection(page: &Page<'_>) -> Result<(), Report> {
    let selection = page.element("#test-grid-selection").await?;
    // Focus is on Alice's first cell: Space selects her row.
    page.send_keys(Key::Space).await?;
    selection.wait_for_inner_text("Alice").await?;
    let alice = row(page, "Alice").await?;
    assert_that!(alice.attr("aria-selected").await?)
        .get_some()
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
    selection.inner_text_stays("Alice,Dave").await?;

    alice.click().await?;
    selection.wait_for_inner_text("Dave").await?;
    Ok(())
}

/// The cell with `text` of the grid labelled `grid`.
async fn grid_cell(page: &Page<'_>, grid: &str, text: &str) -> Result<WebElement, Report> {
    page.element(format!("[role=grid][aria-label='{grid}']"))
        .await?
        .element(xpath(format!(
            ".//*[@role='gridcell'][normalize-space(.)='{text}']"
        )))
        .await
}

/// In cell focus mode, cells can't be selected themselves (no cell selection): Space and presses
/// on a cell select its row.
async fn cell_focus_mode_selects_rows(page: &Page<'_>) -> Result<(), Report> {
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

/// With `on_cell_action`, activating a cell runs the action with the cell's key instead of
/// selecting the row. Only Enter is an action key: Space on such a cell does nothing (cells can't
/// be selected, and the cell's press handling keeps the key from the row).
async fn cell_actions(page: &Page<'_>) -> Result<(), Report> {
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
    selection.inner_text_stays("").await?;
    action.inner_text_stays("").await?;
    page.send_keys(Key::Enter).await?;
    action.wait_for_inner_text("Apple-0").await?;
    page.send_keys(Key::Right).await?;
    page.send_keys(Key::Enter).await?;
    action.wait_for_inner_text("Apple-1").await?;

    grid_cell(page, "Actions", "Yellow").await?.click().await?;
    action.wait_for_inner_text("Banana-1").await?;
    selection.inner_text_stays("").await?;
    Ok(())
}

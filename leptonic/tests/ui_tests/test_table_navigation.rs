// Upstream: react-aria-components/test/Table.test.js @ 99e6102368
//! Keyboard navigation of the table atoms: `KeyboardNavigationBehavior::Tab` with text inputs
//! in cells ("keyboardNavigationBehavior='tab' and textfields in row"), arrow navigation into
//! cells with focusable children, right-to-left, PageUp/PageDown into the column headers,
//! column spans ("colSpan") and an empty table. Spec: react-aria-components `Table.test.js`.
use assertr::prelude::*;
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::{
    pages::{ElementActions, Page, PageActions, xpath},
    polling::wait_for,
};

const PATH: &str = "/atoms/table-navigation";

async fn table(page: &Page<'_>, label: &str) -> Result<WebElement, Report> {
    page.element(format!("[role=grid][aria-label='{label}']"))
        .await
}

/// The cell, row header or column header with `text` in the table `label`.
async fn cell(page: &Page<'_>, label: &str, text: &str) -> Result<WebElement, Report> {
    table(page, label)
        .await?
        .element(xpath(format!(
            ".//*[@role='gridcell' or @role='rowheader' or @role='columnheader'][normalize-space(.)='{text}']"
        )))
        .await
}

/// The row whose row header is `text` in the table `label`.
async fn row(page: &Page<'_>, label: &str, text: &str) -> Result<WebElement, Report> {
    table(page, label)
        .await?
        .element(xpath(format!(
            ".//*[@role='row'][.//*[@role='rowheader'][normalize-space(.)='{text}']]"
        )))
        .await
}

/// The last cell of the row whose row header is `text`.
async fn last_cell(page: &Page<'_>, label: &str, text: &str) -> Result<WebElement, Report> {
    row(page, label, text)
        .await?
        .element(xpath("./*[last()]"))
        .await
}

/// The element with `aria-label` `name` (a button or input) in the table `label`.
async fn labelled(page: &Page<'_>, label: &str, name: &str) -> Result<WebElement, Report> {
    table(page, label)
        .await?
        .element(xpath(format!(".//*[@aria-label='{name}']")))
        .await
}

/// On a freshly loaded page (no focused rows yet, as each upstream test renders anew), focus
/// the first row of the table after the "Before" button `before`.
async fn enter(page: &Page<'_>, before: &str, label: &str, first_row: &str) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element(before).await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&row(page, label, first_row).await?)
        .await?;
    Ok(())
}

const TAB: &str = "Tab mode table";

/// "Tab from a focused cell moves focus to the first tabbable child".
pub async fn tab_from_a_cell_focuses_its_first_tabbable_child(
    page: &Page<'_>,
) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    enter(page, "#test-tn-before-tab", TAB, "Games").await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&last_cell(page, TAB, "Games").await?)
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&labelled(page, TAB, "Games notes").await?)
        .await?;
    Ok(())
}

/// "Tab from a cell with no tabbable children or from the last child in a cell exits the
/// table".
pub async fn tab_from_a_cell_without_children_exits_the_table(
    page: &Page<'_>,
) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    enter(page, "#test-tn-before-tab", TAB, "Games").await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&cell(page, TAB, "Games").await?)
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-tn-after-tab").await?)
        .await?;

    // Back into the table: the row header again.
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&cell(page, TAB, "Games").await?)
        .await?;
    page.send_keys(Key::Left).await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&last_cell(page, TAB, "Games").await?)
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&labelled(page, TAB, "Games notes").await?)
        .await?;
    page.send_keys(Key::Tab).await?;
    let button = table(page, TAB)
        .await?
        .element(xpath(
            ".//button[normalize-space(.)='Button next to input']",
        ))
        .await?;
    page.wait_for_focus(&button).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-tn-after-tab").await?)
        .await?;
    Ok(())
}

/// "Shift+Tab from a child returns focus to the cell".
pub async fn shift_tab_from_a_child_returns_to_the_cell(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    enter(page, "#test-tn-before-tab", TAB, "Games").await?;
    page.send_keys(Key::Left).await?;
    let notes = last_cell(page, TAB, "Games").await?;
    page.wait_for_focus(&notes).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&labelled(page, TAB, "Games notes").await?)
        .await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&notes).await?;
    Ok(())
}

/// "should not navigate to next cell when arrow keys are pressed while a text input child has
/// focus", "should not trigger typeahead when typing in a text input child" and "should not
/// trigger selection when pressing Space or Enter in a text input child".
pub async fn keys_in_a_text_input_stay_there(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    enter(page, "#test-tn-before-tab", TAB, "Games").await?;
    page.send_keys(Key::Left).await?;
    page.send_keys(Key::Tab).await?;
    let input = labelled(page, TAB, "Games notes").await?;
    page.wait_for_focus(&input).await?;
    input.virtual_input("").await?;
    for key in [Key::Down, Key::Up, Key::Right, Key::Left] {
        page.send_keys(key).await?;
    }
    page.send_keys("Games").await?;
    page.send_keys(" ").await?;
    page.send_keys(Key::Enter).await?;
    // Nothing moved or got selected.
    page.focus_stays(&input).await?;
    input.wait_for_prop("value", "Games ").await?;
    page.element("#test-tn-tab-selection")
        .await?
        .inner_text_stays("")
        .await?;
    Ok(())
}

/// "should not trigger selection when clicking on a tabbable child element" and "should still
/// trigger selection when clicking on a row with no tabbable children".
pub async fn clicking_a_child_or_a_row(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let input = labelled(page, TAB, "Program Files notes").await?;
    input.click().await?;
    page.wait_for_focus(&input).await?;
    page.element("#test-tn-tab-selection")
        .await?
        .inner_text_stays("")
        .await?;

    cell(page, TAB, "System file").await?.click().await?;
    page.element("#test-tn-tab-selection")
        .await?
        .wait_for_inner_text("3")
        .await?;
    Ok(())
}

/// `focusMode="child"` in tab navigation: arrowing onto the cell focuses its last child (the
/// strategy of ArrowLeft); with `allowsArrowNavigation`, ArrowDown moves from the child to the
/// next row; Shift+Tab from the child skips the cell.
pub async fn child_focus_mode_in_tab_navigation(page: &Page<'_>) -> Result<(), Report> {
    const ARROWS: &str = "Tab mode arrows table";
    const CHILD: &str = "Tab mode child table";
    page.goto_path(PATH).await?;
    enter(page, "#test-tn-before-child", CHILD, "Games").await?;
    page.send_keys(Key::Left).await?;
    let button = labelled_button(page, CHILD).await?;
    page.wait_for_focus(&button).await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&labelled(page, CHILD, "Games notes").await?)
        .await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-tn-before-child").await?)
        .await?;

    enter(page, "#test-tn-before-arrows", ARROWS, "Games").await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&labelled_button(page, ARROWS).await?)
        .await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&labelled(page, ARROWS, "Program Files notes").await?)
        .await?;
    Ok(())
}

async fn labelled_button(page: &Page<'_>, label: &str) -> Result<WebElement, Report> {
    table(page, label)
        .await?
        .element(xpath(
            ".//button[normalize-space(.)='Button next to input']",
        ))
        .await
}

/// "default focusMode: ArrowRight crosses from last child to first child of next cell,
/// ArrowLeft reverses".
pub async fn arrow_navigation_through_cell_children(page: &Page<'_>) -> Result<(), Report> {
    const ARROW: &str = "Arrow mode table";
    page.goto_path(PATH).await?;
    enter(page, "#test-tn-before-arrow-mode", ARROW, "Row 1").await?;
    // Without selection or actions, rows show no hover; cells do (react-aria-components).
    let first_row = row(page, ARROW, "Row 1").await?;
    let row_header = cell(page, ARROW, "Row 1").await?;
    row_header.hover().await?;
    row_header
        .wait_for_attr("data-hovered", Some("true"))
        .await?;
    assert_that!(first_row.attr("data-hovered").await?).is_none();
    page.send_keys(Key::Right).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&labelled(page, ARROW, "R1C2 first").await?)
        .await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&labelled(page, ARROW, "R1C2 last").await?)
        .await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&labelled(page, ARROW, "R1C3 first").await?)
        .await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&labelled(page, ARROW, "R1C2 last").await?)
        .await?;
    Ok(())
}

/// "arrow navigation with focusMode="cell": cell element stays focused on navigate, arrows
/// enter/exit children within cell".
pub async fn arrow_navigation_with_cell_focus_mode(page: &Page<'_>) -> Result<(), Report> {
    const ARROW: &str = "Arrow cell table";
    page.goto_path(PATH).await?;
    enter(page, "#test-tn-before-arrow-cell", ARROW, "Row 1").await?;
    let cells = row(page, ARROW, "Row 1")
        .await?
        .elements("[role=gridcell]")
        .await?;
    let (col2, col3) = (&cells[0], &cells[1]);
    let first = |n: u8| format!("R1C{n} first");
    page.send_keys(Key::Right).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(col2).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&labelled(page, ARROW, &first(2)).await?)
        .await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&labelled(page, ARROW, "R1C2 last").await?)
        .await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&labelled(page, ARROW, &first(2)).await?)
        .await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(col2).await?;
    page.send_keys(Key::Right).await?;
    page.send_keys(Key::Right).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(col3).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&labelled(page, ARROW, &first(3)).await?)
        .await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(col3).await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(col2).await?;
    Ok(())
}

/// In a right-to-left table, ArrowLeft moves forward: from the row into its first cell, on to
/// the next cell, and between column headers.
pub async fn right_to_left(page: &Page<'_>) -> Result<(), Report> {
    const RTL: &str = "RTL table";
    page.goto_path(PATH).await?;
    enter(page, "#test-tn-before-rtl", RTL, "Games").await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&cell(page, RTL, "Games").await?)
        .await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&cell(page, RTL, "File folder").await?)
        .await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&cell(page, RTL, "Games").await?)
        .await?;
    page.send_keys(Key::Up).await?;
    page.wait_for_focus(&cell(page, RTL, "Name").await?).await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&cell(page, RTL, "Type").await?).await?;
    Ok(())
}

/// PageDown moves a page down; PageUp moves up through the rows into the column headers
/// (react-aria's paging steps with the table's `getKeyAbove`).
pub async fn page_up_reaches_the_column_headers(page: &Page<'_>) -> Result<(), Report> {
    const PAGED: &str = "Paged table";
    page.goto_path(PATH).await?;
    enter(page, "#test-tn-before-paged", PAGED, "Row 1").await?;
    page.send_keys(Key::PageDown).await?;
    wait_for("the focused row's number")
        .observing(|| focused_row_number(page))
        .to_be("past 5 (a page down)", |row| *row > 5)
        .await?;
    // Each PageUp moves focus up (a page of rows, then into the column headers), until it
    // reaches the column header.
    let header = cell(page, PAGED, "Name").await?;
    for _ in 0..3 {
        let before = page.focused_element().await?;
        if before == header {
            break;
        }
        let before = before.describe().await?;
        page.send_keys(Key::PageUp).await?;
        wait_for("the focused element")
            .observing(|| async { page.focused_element().await?.describe().await })
            .to_be(&format!("another than {before} (moved up)"), |focused| {
                *focused != before
            })
            .await?;
    }
    page.wait_for_focus(&header).await?;
    Ok(())
}

/// The number of the focused row ("Row 7"), 0 if focus is not on a row.
async fn focused_row_number(page: &Page<'_>) -> Result<u32, Report> {
    Ok(page
        .focused_element()
        .await?
        .inner_text()
        .await?
        .strip_prefix("Row ")
        .and_then(|rest| rest.split_whitespace().next())
        .and_then(|n| n.parse().ok())
        .unwrap_or(0))
}

/// "should render table with colSpans" and "should focus to the same colIndex when moving
/// focus up or down".
pub async fn column_spans(page: &Page<'_>) -> Result<(), Report> {
    const SPANS: &str = "Table with various colspans";
    page.goto_path(PATH).await?;
    let span2 = cell(page, SPANS, "R1 span 2").await?;
    assert_that!(span2.attr("colspan").await?)
        .get_some()
        .is_equal_to("2");
    assert_that!(
        cell(page, SPANS, "R1C4")
            .await?
            .attr("aria-colindex")
            .await?
    )
    .get_some()
    .is_equal_to("4");
    assert_that!(
        cell(page, SPANS, "R3 span 4")
            .await?
            .attr("colspan")
            .await?
    )
    .get_some()
    .is_equal_to("4");
    assert_that!(cell(page, SPANS, "R3 span 4").await?.attr("role").await?)
        .get_some()
        .is_equal_to("rowheader");

    enter(page, "#test-tn-before-colspan", SPANS, "R1C1").await?;
    // Each step: the keys, then the cell that must have focus.
    let steps: [(&[Key], &str); 17] = [
        (&[Key::Right], "R1C1"),
        (&[Key::Right], "R1 span 2"),
        (&[Key::Down], "R2C2"),
        (&[Key::Down], "R3 span 4"),
        (&[Key::Down, Key::Right], "R4C2"),
        (&[Key::Down], "R5 span 3"),
        (&[Key::Down, Key::Right], "R6C2"),
        (&[Key::Down], "R7 span 3"),
        (&[Key::Up], "R6C2"),
        (&[Key::Right, Key::Right, Key::Up], "R5C4"),
        (&[Key::Up], "R4C4"),
        (&[Key::Up], "R3 span 4"),
        (&[Key::Up], "R2C1"),
        (&[Key::Right, Key::Up], "R1 span 2"),
        (&[Key::Down], "R2C2"),
        (&[Key::Left], "R2C1"),
        (&[Key::Up], "R1C1"),
    ];
    for (keys, expected) in steps {
        for key in keys {
            page.send_keys(key.clone()).await?;
        }
        page.wait_for_focus(&cell(page, SPANS, expected).await?)
            .await?;
    }
    Ok(())
}

/// An empty table is a tab stop itself; its column headers aren't, and arrow keys don't move
/// focus into them (react-stately disables keyboard navigation while `collection.size === 0`).
/// Select all is disabled.
pub async fn an_empty_table(page: &Page<'_>) -> Result<(), Report> {
    const EMPTY: &str = "Empty table";
    page.goto_path(PATH).await?;
    let empty = table(page, EMPTY).await?;
    page.element("#test-tn-before-empty").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&empty).await?;
    for key in [Key::Down, Key::Up, Key::Right, Key::End] {
        page.send_keys(key).await?;
    }
    page.focus_stays(&empty).await?;
    let select_all = empty.element("input[type=checkbox]").await?;
    assert_that!(select_all.is_enabled().await?).is_false();
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-tn-after-empty").await?)
        .await?;
    Ok(())
}

/// Enter and clicks on a `Button` atom that isn't its cell's first child (`CellFocusMode::Child`)
/// press that button, and focus stays on it (crudkit: Enter first moved focus to the first child).
pub async fn enter_on_a_button_that_is_not_the_first_child(page: &Page<'_>) -> Result<(), Report> {
    const ACTIONS: &str = "Actions table";
    page.goto_path(PATH).await?;
    enter(page, "#test-tn-before-actions", ACTIONS, "Alice").await?;
    page.send_keys(Key::Right).await?;
    page.send_keys(Key::Right).await?;
    let edit = labelled(page, ACTIONS, "Edit Alice").await?;
    page.wait_for_focus(&edit).await?;
    page.send_keys(Key::Right).await?;
    let delete = labelled(page, ACTIONS, "Delete Alice").await?;
    page.wait_for_focus(&delete).await?;

    page.send_keys(Key::Enter).await?;
    page.element("#test-tn-actions-log")
        .await?
        .wait_for_inner_text("delete Alice")
        .await?;
    page.wait_for_focus(&delete).await?;
    page.send_keys(Key::Space).await?;
    page.element("#test-tn-actions-log")
        .await?
        .wait_for_inner_text("delete Alice, delete Alice")
        .await?;
    page.wait_for_focus(&delete).await?;

    let delete_bob = labelled(page, ACTIONS, "Delete Bob").await?;
    delete_bob.click().await?;
    page.element("#test-tn-actions-log")
        .await?
        .wait_for_inner_text("delete Alice, delete Alice, delete Bob")
        .await?;
    page.wait_for_focus(&delete_bob).await?;
    page.send_keys(Key::Enter).await?;
    page.element("#test-tn-actions-log")
        .await?
        .wait_for_inner_text("delete Alice, delete Alice, delete Bob, delete Bob")
        .await?;
    page.wait_for_focus(&delete_bob).await?;
    Ok(())
}

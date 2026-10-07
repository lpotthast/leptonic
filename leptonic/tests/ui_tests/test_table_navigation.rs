// Upstream: react-aria-components/test/Table.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

const PATH: &str = "/atoms/table-navigation";

/// Keyboard navigation of the table atoms: `KeyboardNavigationBehavior::Tab` with text inputs
/// in cells ("keyboardNavigationBehavior='tab' and textfields in row"), arrow navigation into
/// cells with focusable children, right-to-left, PageUp/PageDown into the column headers,
/// column spans ("colSpan") and an empty table. Spec: react-aria-components `Table.test.js`.
pub struct TableNavigationTests {}

#[async_trait]
impl BrowserTest<str> for TableNavigationTests {
    fn name(&self) -> Cow<'_, str> {
        "table_navigation_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path(PATH).await?;

        tab_from_a_cell_focuses_its_first_tabbable_child(&page).await?;
        tab_from_a_cell_without_children_exits_the_table(&page).await?;
        shift_tab_from_a_child_returns_to_the_cell(&page).await?;
        keys_in_a_text_input_stay_there(&page).await?;
        clicking_a_child_or_a_row(&page).await?;
        child_focus_mode_in_tab_navigation(&page).await?;
        arrow_navigation_through_cell_children(&page).await?;
        arrow_navigation_with_cell_focus_mode(&page).await?;
        right_to_left(&page).await?;
        page_up_reaches_the_column_headers(&page).await?;
        column_spans(&page).await?;
        an_empty_table(&page).await?;

        page.expect_no_page_errors().await
    }
}

async fn table(page: &Page<'_>, label: &str) -> Result<WebElement, Report> {
    page.css(&format!("[role=grid][aria-label='{label}']"))
        .await
}

/// The cell, row header or column header with `text` in the table `label`.
async fn cell(page: &Page<'_>, label: &str, text: &str) -> Result<WebElement, Report> {
    Ok(table(page, label)
        .await?
        .find(By::XPath(format!(
            ".//*[@role='gridcell' or @role='rowheader' or @role='columnheader'][normalize-space(.)='{text}']"
        )))
        .await?)
}

/// The row whose row header is `text` in the table `label`.
async fn row(page: &Page<'_>, label: &str, text: &str) -> Result<WebElement, Report> {
    Ok(table(page, label)
        .await?
        .find(By::XPath(format!(
            ".//*[@role='row'][.//*[@role='rowheader'][normalize-space(.)='{text}']]"
        )))
        .await?)
}

/// The last cell of the row whose row header is `text`.
async fn last_cell(page: &Page<'_>, label: &str, text: &str) -> Result<WebElement, Report> {
    Ok(row(page, label, text)
        .await?
        .find(By::XPath("./*[last()]"))
        .await?)
}

/// The element with `aria-label` `name` (a button or input) in the table `label`.
async fn labelled(page: &Page<'_>, label: &str, name: &str) -> Result<WebElement, Report> {
    Ok(table(page, label)
        .await?
        .find(By::XPath(format!(".//*[@aria-label='{name}']")))
        .await?)
}

async fn expect_focus(page: &Page<'_>, element: &WebElement, what: &str) -> Result<(), Report> {
    page.wait_for_focus_on(element, what).await
}

async fn press(page: &Page<'_>, key: Key) -> Result<(), Report> {
    page.send_keys_to_active(key).await
}

/// On a freshly loaded page (no focused rows yet, as each upstream test renders anew), focus
/// the first row of the table after the "Before" button `before`.
async fn enter(page: &Page<'_>, before: &str, label: &str, first_row: &str) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.click_element_with_id(before).await?;
    page.press_tab().await?;
    expect_focus(page, &row(page, label, first_row).await?, "the first row").await
}

const TAB: &str = "Tab mode table";

/// "Tab from a focused cell moves focus to the first tabbable child".
async fn tab_from_a_cell_focuses_its_first_tabbable_child(page: &Page<'_>) -> Result<(), Report> {
    enter(page, "test-tn-before-tab", TAB, "Games").await?;
    press(page, Key::Left).await?;
    expect_focus(
        page,
        &last_cell(page, TAB, "Games").await?,
        "the notes cell",
    )
    .await?;
    page.press_tab().await?;
    expect_focus(
        page,
        &labelled(page, TAB, "Games notes").await?,
        "the notes input",
    )
    .await
}

/// "Tab from a cell with no tabbable children or from the last child in a cell exits the
/// table".
async fn tab_from_a_cell_without_children_exits_the_table(page: &Page<'_>) -> Result<(), Report> {
    enter(page, "test-tn-before-tab", TAB, "Games").await?;
    press(page, Key::Right).await?;
    expect_focus(page, &cell(page, TAB, "Games").await?, "the row header").await?;
    page.press_tab().await?;
    page.wait_for_active_id("test-tn-after-tab").await?;

    // Back into the table: the row header again.
    page.press_shift_tab().await?;
    expect_focus(page, &cell(page, TAB, "Games").await?, "the row header").await?;
    press(page, Key::Left).await?;
    press(page, Key::Left).await?;
    expect_focus(
        page,
        &last_cell(page, TAB, "Games").await?,
        "the notes cell",
    )
    .await?;
    page.press_tab().await?;
    expect_focus(
        page,
        &labelled(page, TAB, "Games notes").await?,
        "the notes input",
    )
    .await?;
    page.press_tab().await?;
    let button = table(page, TAB)
        .await?
        .find(By::XPath(
            ".//button[normalize-space(.)='Button next to input']",
        ))
        .await?;
    expect_focus(page, &button, "the button next to the input").await?;
    page.press_tab().await?;
    page.wait_for_active_id("test-tn-after-tab").await
}

/// "Shift+Tab from a child returns focus to the cell".
async fn shift_tab_from_a_child_returns_to_the_cell(page: &Page<'_>) -> Result<(), Report> {
    enter(page, "test-tn-before-tab", TAB, "Games").await?;
    press(page, Key::Left).await?;
    let notes = last_cell(page, TAB, "Games").await?;
    expect_focus(page, &notes, "the notes cell").await?;
    page.press_tab().await?;
    expect_focus(
        page,
        &labelled(page, TAB, "Games notes").await?,
        "the notes input",
    )
    .await?;
    page.press_shift_tab().await?;
    expect_focus(page, &notes, "the notes cell").await
}

/// "should not navigate to next cell when arrow keys are pressed while a text input child has
/// focus", "should not trigger typeahead when typing in a text input child" and "should not
/// trigger selection when pressing Space or Enter in a text input child".
async fn keys_in_a_text_input_stay_there(page: &Page<'_>) -> Result<(), Report> {
    enter(page, "test-tn-before-tab", TAB, "Games").await?;
    press(page, Key::Left).await?;
    page.press_tab().await?;
    let input = labelled(page, TAB, "Games notes").await?;
    expect_focus(page, &input, "the notes input").await?;
    page.driver
        .execute("arguments[0].value = ''", vec![input.to_json()?])
        .await?;
    for key in [Key::Down, Key::Up, Key::Right, Key::Left] {
        press(page, key).await?;
    }
    page.send_keys_to_active("Games").await?;
    page.send_keys_to_active(" ").await?;
    press(page, Key::Enter).await?;
    // Settle, then check nothing moved or got selected.
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    expect_focus(page, &input, "the notes input").await?;
    assert_that!(input.prop("value").await?).is_equal_to(Some("Games ".to_owned()));
    assert_that!(page.read_text_of("test-tn-tab-selection").await?).is_equal_to(String::new());
    Ok(())
}

/// "should not trigger selection when clicking on a tabbable child element" and "should still
/// trigger selection when clicking on a row with no tabbable children".
async fn clicking_a_child_or_a_row(page: &Page<'_>) -> Result<(), Report> {
    let input = labelled(page, TAB, "Program Files notes").await?;
    input.click().await?;
    expect_focus(page, &input, "the clicked input").await?;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    assert_that!(page.read_text_of("test-tn-tab-selection").await?).is_equal_to(String::new());

    cell(page, TAB, "System file").await?.click().await?;
    page.wait_for_text("test-tn-tab-selection", "3").await
}

/// `focusMode="child"` in tab navigation: arrowing onto the cell focuses its last child (the
/// strategy of ArrowLeft); with `allowsArrowNavigation`, ArrowDown moves from the child to the
/// next row; Shift+Tab from the child skips the cell.
async fn child_focus_mode_in_tab_navigation(page: &Page<'_>) -> Result<(), Report> {
    const ARROWS: &str = "Tab mode arrows table";
    const CHILD: &str = "Tab mode child table";
    enter(page, "test-tn-before-child", CHILD, "Games").await?;
    press(page, Key::Left).await?;
    let button = labelled_button(page, CHILD).await?;
    expect_focus(page, &button, "the button next to the input").await?;
    page.press_shift_tab().await?;
    expect_focus(
        page,
        &labelled(page, CHILD, "Games notes").await?,
        "the notes input",
    )
    .await?;
    page.press_shift_tab().await?;
    page.wait_for_active_id("test-tn-before-child").await?;

    enter(page, "test-tn-before-arrows", ARROWS, "Games").await?;
    press(page, Key::Left).await?;
    expect_focus(page, &labelled_button(page, ARROWS).await?, "the button").await?;
    press(page, Key::Down).await?;
    expect_focus(
        page,
        &labelled(page, ARROWS, "Program Files notes").await?,
        "the next row's notes input",
    )
    .await
}

async fn labelled_button(page: &Page<'_>, label: &str) -> Result<WebElement, Report> {
    Ok(table(page, label)
        .await?
        .find(By::XPath(
            ".//button[normalize-space(.)='Button next to input']",
        ))
        .await?)
}

/// "default focusMode: ArrowRight crosses from last child to first child of next cell,
/// ArrowLeft reverses".
async fn arrow_navigation_through_cell_children(page: &Page<'_>) -> Result<(), Report> {
    const ARROW: &str = "Arrow mode table";
    enter(page, "test-tn-before-arrow-mode", ARROW, "Row 1").await?;
    // Without selection or actions, rows show no hover; cells do (react-aria-components).
    let first_row = row(page, ARROW, "Row 1").await?;
    let row_header = cell(page, ARROW, "Row 1").await?;
    page.driver
        .action_chain()
        .move_to_element_center(&row_header)
        .perform()
        .await?;
    page.wait_for_attr(&row_header, "data-hovered", Some("true"))
        .await?;
    assert_that!(first_row.attr("data-hovered").await?).is_none();
    press(page, Key::Right).await?;
    press(page, Key::Right).await?;
    expect_focus(
        page,
        &labelled(page, ARROW, "R1C2 first").await?,
        "R1C2 first",
    )
    .await?;
    press(page, Key::Right).await?;
    expect_focus(
        page,
        &labelled(page, ARROW, "R1C2 last").await?,
        "R1C2 last",
    )
    .await?;
    press(page, Key::Right).await?;
    expect_focus(
        page,
        &labelled(page, ARROW, "R1C3 first").await?,
        "R1C3 first",
    )
    .await?;
    press(page, Key::Left).await?;
    expect_focus(
        page,
        &labelled(page, ARROW, "R1C2 last").await?,
        "R1C2 last",
    )
    .await
}

/// "arrow navigation with focusMode="cell": cell element stays focused on navigate, arrows
/// enter/exit children within cell".
async fn arrow_navigation_with_cell_focus_mode(page: &Page<'_>) -> Result<(), Report> {
    const ARROW: &str = "Arrow cell table";
    enter(page, "test-tn-before-arrow-cell", ARROW, "Row 1").await?;
    let cells = row(page, ARROW, "Row 1")
        .await?
        .find_all(By::Css("[role=gridcell]"))
        .await?;
    let (col2, col3) = (&cells[0], &cells[1]);
    let first = |n: u8| format!("R1C{n} first");
    press(page, Key::Right).await?;
    press(page, Key::Right).await?;
    expect_focus(page, col2, "column 2's cell").await?;
    press(page, Key::Right).await?;
    expect_focus(page, &labelled(page, ARROW, &first(2)).await?, "R1C2 first").await?;
    press(page, Key::Right).await?;
    expect_focus(
        page,
        &labelled(page, ARROW, "R1C2 last").await?,
        "R1C2 last",
    )
    .await?;
    press(page, Key::Left).await?;
    expect_focus(page, &labelled(page, ARROW, &first(2)).await?, "R1C2 first").await?;
    press(page, Key::Left).await?;
    expect_focus(page, col2, "column 2's cell").await?;
    press(page, Key::Right).await?;
    press(page, Key::Right).await?;
    press(page, Key::Right).await?;
    expect_focus(page, col3, "column 3's cell").await?;
    press(page, Key::Right).await?;
    expect_focus(page, &labelled(page, ARROW, &first(3)).await?, "R1C3 first").await?;
    press(page, Key::Left).await?;
    expect_focus(page, col3, "column 3's cell").await?;
    press(page, Key::Left).await?;
    expect_focus(page, col2, "column 2's cell").await
}

/// In a right-to-left table, ArrowLeft moves forward: from the row into its first cell, on to
/// the next cell, and between column headers.
async fn right_to_left(page: &Page<'_>) -> Result<(), Report> {
    const RTL: &str = "RTL table";
    enter(page, "test-tn-before-rtl", RTL, "Games").await?;
    press(page, Key::Left).await?;
    expect_focus(page, &cell(page, RTL, "Games").await?, "the row header").await?;
    press(page, Key::Left).await?;
    expect_focus(
        page,
        &cell(page, RTL, "File folder").await?,
        "the type cell",
    )
    .await?;
    press(page, Key::Right).await?;
    expect_focus(page, &cell(page, RTL, "Games").await?, "the row header").await?;
    press(page, Key::Up).await?;
    expect_focus(
        page,
        &cell(page, RTL, "Name").await?,
        "the Name column header",
    )
    .await?;
    press(page, Key::Left).await?;
    expect_focus(
        page,
        &cell(page, RTL, "Type").await?,
        "the Type column header",
    )
    .await
}

/// PageDown moves a page down; PageUp moves up through the rows into the column headers
/// (react-aria's paging steps with the table's `getKeyAbove`).
async fn page_up_reaches_the_column_headers(page: &Page<'_>) -> Result<(), Report> {
    const PAGED: &str = "Paged table";
    enter(page, "test-tn-before-paged", PAGED, "Row 1").await?;
    press(page, Key::PageDown).await?;
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    loop {
        let text = page.active_element_text().await?;
        let number: u32 = text
            .trim()
            .strip_prefix("Row ")
            .and_then(|rest| rest.split_whitespace().next())
            .and_then(|n| n.parse().ok())
            .unwrap_or(0);
        if number > 5 {
            break;
        }
        if std::time::Instant::now() > deadline {
            rootcause::bail!("PageDown did not move a page down; focus is on {text:?}");
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    for _ in 0..3 {
        press(page, Key::PageUp).await?;
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    expect_focus(
        page,
        &cell(page, PAGED, "Name").await?,
        "the Name column header",
    )
    .await
}

/// "should render table with colSpans" and "should focus to the same colIndex when moving
/// focus up or down".
async fn column_spans(page: &Page<'_>) -> Result<(), Report> {
    const SPANS: &str = "Table with various colspans";
    let span2 = cell(page, SPANS, "R1 span 2").await?;
    assert_that!(span2.attr("colspan").await?).is_equal_to(Some("2".to_owned()));
    assert_that!(
        cell(page, SPANS, "R1C4")
            .await?
            .attr("aria-colindex")
            .await?
    )
    .is_equal_to(Some("4".to_owned()));
    assert_that!(
        cell(page, SPANS, "R3 span 4")
            .await?
            .attr("colspan")
            .await?
    )
    .is_equal_to(Some("4".to_owned()));
    assert_that!(cell(page, SPANS, "R3 span 4").await?.attr("role").await?)
        .is_equal_to(Some("rowheader".to_owned()));

    enter(page, "test-tn-before-colspan", SPANS, "R1C1").await?;
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
            press(page, key.clone()).await?;
        }
        expect_focus(page, &cell(page, SPANS, expected).await?, expected).await?;
    }
    Ok(())
}

/// An empty table is a tab stop itself; its column headers aren't, and arrow keys don't move
/// focus into them (react-stately disables keyboard navigation while `collection.size === 0`).
/// Select all is disabled.
async fn an_empty_table(page: &Page<'_>) -> Result<(), Report> {
    const EMPTY: &str = "Empty table";
    let empty = table(page, EMPTY).await?;
    page.click_element_with_id("test-tn-before-empty").await?;
    page.press_tab().await?;
    expect_focus(page, &empty, "the empty table").await?;
    for key in [Key::Down, Key::Up, Key::Right, Key::End] {
        press(page, key).await?;
    }
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    expect_focus(page, &empty, "the empty table").await?;
    let select_all = empty.find(By::Css("input[type=checkbox]")).await?;
    assert_that!(select_all.is_enabled().await?).is_false();
    page.press_tab().await?;
    page.wait_for_active_id("test-tn-after-empty").await
}

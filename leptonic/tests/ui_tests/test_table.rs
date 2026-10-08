// Upstream: react-aria-components/test/Table.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// Behavior of the table hooks (through the `Table` atoms): ARIA structure (column headers,
/// row headers labelling rows, `aria-sort`, column groups), navigation between body and column
/// headers, sorting, select all, disabled rows, type-ahead and refocusing after removing the
/// focused row. Spec: react-aria-components `Table.test.js`.
pub struct TableTests {}

#[async_trait]
impl BrowserTest<str> for TableTests {
    fn name(&self) -> Cow<'_, str> {
        "table_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/table").await?;

        aria_structure(&page).await?;
        column_groups(&page).await?;
        navigation_into_the_column_headers(&page).await?;
        sorting(&page).await?;
        select_all(&page).await?;
        disabled_rows(&page).await?;
        type_ahead(&page).await?;
        removing_the_focused_row(&page).await?;
        localized(&page).await?;

        Ok(())
    }
}

async fn grid(page: &Page<'_>, label: &str) -> Result<WebElement, Report> {
    page.css(&format!("[role=grid][aria-label='{label}']"))
        .await
}

/// The row of the "Files" table whose row header is `name`.
async fn row(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    grid(page, "Files")
        .await?
        .find(By::XPath(format!(
            ".//*[@role='row'][.//*[@role='rowheader'][normalize-space(.)='{name}']]"
        )))
        .await
        .map_err(Into::into)
}

async fn row_header(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    row(page, name)
        .await?
        .find(By::Css("[role=rowheader]"))
        .await
        .map_err(Into::into)
}

async fn column_header(page: &Page<'_>, table: &str, text: &str) -> Result<WebElement, Report> {
    grid(page, table)
        .await?
        .find(By::XPath(format!(
            ".//*[@role='columnheader'][normalize-space(.)='{text}']"
        )))
        .await
        .map_err(Into::into)
}

/// Focus the row of `name` (programmatically, without pressing it).
async fn focus_row(page: &Page<'_>, name: &str) -> Result<(), Report> {
    let row = row(page, name).await?;
    page.driver
        .execute("arguments[0].focus()", vec![row.to_json()?])
        .await?;
    expect_focus_on_row(page, name).await
}

async fn attr(element: &WebElement, name: &str) -> Result<Option<String>, Report> {
    Ok(element.attr(name).await?)
}

/// The row header texts of the "Files" table, top to bottom.
async fn row_names(page: &Page<'_>) -> Result<Vec<String>, Report> {
    let mut names = Vec::new();
    for cell in grid(page, "Files")
        .await?
        .find_all(By::Css("[role=rowheader]"))
        .await?
    {
        names.push(cell.text().await?);
    }
    Ok(names)
}

async fn expect_rows(page: &Page<'_>, expected: &[&str]) -> Result<(), Report> {
    wait_for!("the rows", expected, row_names(page).await?);
    Ok(())
}

async fn expect_focus_on_row(page: &Page<'_>, name: &str) -> Result<(), Report> {
    let row = row(page, name).await?;
    page.wait_for_focus_on(&row, &format!("row {name}")).await
}

async fn expect_focus_on_row_header(page: &Page<'_>, name: &str) -> Result<(), Report> {
    let cell = row_header(page, name).await?;
    page.wait_for_focus_on(&cell, &format!("row header {name}"))
        .await
}

async fn select_all_checkbox(page: &Page<'_>) -> Result<WebElement, Report> {
    grid(page, "Files")
        .await?
        .find(By::Css("thead input[type=checkbox]"))
        .await
        .map_err(Into::into)
}

async fn expect_focus_on_select_all(page: &Page<'_>) -> Result<(), Report> {
    let checkbox = select_all_checkbox(page).await?;
    page.wait_for_focus_on(&checkbox, "the select all checkbox")
        .await
}

async fn aria_structure(page: &Page<'_>) -> Result<(), Report> {
    let files = grid(page, "Files").await?;
    assert_that!(attr(&files, "aria-multiselectable").await?).is_equal_to(Some("true".to_owned()));
    expect_rows(page, &["bootmgr", "Games", "log.txt", "Program Files"]).await?;

    // The selection column, then Name, Type and Date Modified.
    let headers = files.find_all(By::Css("[role=columnheader]")).await?;
    assert_that!(headers.len()).is_equal_to(4);
    let name = column_header(page, "Files", "Name").await?;
    assert_that!(attr(&name, "aria-sort").await?).is_equal_to(Some("ascending".to_owned()));
    let kind = column_header(page, "Files", "Type").await?;
    assert_that!(attr(&kind, "aria-sort").await?).is_equal_to(Some("none".to_owned()));

    // Rows are labelled by their row header cell.
    let games = row(page, "Games").await?;
    let header = row_header(page, "Games").await?;
    let row_labelledby = attr(&header, "id").await?;
    assert_that!(attr(&games, "aria-labelledby").await?).is_equal_to(row_labelledby);
    assert_that!(attr(&games, "aria-selected").await?).is_equal_to(Some("false".to_owned()));
    let checkboxes = games.find_all(By::Css("input[type=checkbox]")).await?;
    assert_that!(checkboxes.len()).is_equal_to(1);
    // The checkbox is labelled "Select" plus the row header: `aria-labelledby` = its own id
    // (with `aria-label`) and the row header cell.
    let checkbox_id = attr(&checkboxes[0], "id").await?.unwrap_or_default();
    let labelledby = attr(&checkboxes[0], "aria-labelledby").await?;
    let header_id = attr(&header, "id").await?.unwrap_or_default();
    assert_that!(labelledby).is_equal_to(Some(format!("{checkbox_id} {header_id}")));
    assert_that!(attr(&checkboxes[0], "aria-label").await?).is_equal_to(Some("Select".to_owned()));
    Ok(())
}

/// A column group spans its columns in a header row above them; the rest of that row is filled
/// with placeholders.
async fn column_groups(page: &Page<'_>) -> Result<(), Report> {
    let contacts = grid(page, "Contacts").await?;
    let header_rows = contacts.find_all(By::Css("thead [role=row]")).await?;
    assert_that!(header_rows.len()).is_equal_to(2);
    let contact = column_header(page, "Contacts", "Contact").await?;
    assert_that!(attr(&contact, "aria-colspan").await?).is_equal_to(Some("2".to_owned()));
    assert_that!(attr(&contact, "aria-colindex").await?).is_equal_to(Some("2".to_owned()));
    let placeholders = header_rows[0].find_all(By::Css("[role=gridcell]")).await?;
    assert_that!(placeholders.len()).is_equal_to(2);

    // Up from a cell reaches its column, then the group.
    page.click_element_with_id("test-table-before-contacts")
        .await?;
    page.press_tab().await?;
    page.send_keys_to_active(Key::Right).await?;
    page.send_keys_to_active(Key::Right).await?;
    page.wait_for_focus("gridcell", Some("alice@example.com"))
        .await?;
    page.send_keys_to_active(Key::Up).await?;
    page.wait_for_focus("columnheader", Some("Email")).await?;
    page.send_keys_to_active(Key::Up).await?;
    page.wait_for_focus("columnheader", Some("Contact")).await?;
    page.send_keys_to_active(Key::Down).await?;
    page.wait_for_focus("columnheader", Some("Email")).await?;
    page.send_keys_to_active(Key::Down).await?;
    page.wait_for_focus("gridcell", Some("alice@example.com"))
        .await
}

/// ArrowUp from the first row moves into the column headers; ArrowLeft/Right move between them
/// (wrapping), ArrowDown back into the body.
async fn navigation_into_the_column_headers(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-table-before").await?;
    page.press_tab().await?;
    expect_focus_on_row(page, "bootmgr").await?;

    // The selection column's header holds the "select all" checkbox, which gets focus.
    page.send_keys_to_active(Key::Up).await?;
    expect_focus_on_select_all(page).await?;
    page.send_keys_to_active(Key::Right).await?;
    page.wait_for_focus("columnheader", Some("Name")).await?;
    page.send_keys_to_active(Key::Left).await?;
    expect_focus_on_select_all(page).await?;
    page.send_keys_to_active(Key::Left).await?;
    page.wait_for_focus("columnheader", Some("Date Modified"))
        .await?;
    page.send_keys_to_active(Key::Down).await?;
    page.wait_for_focus("gridcell", Some("11/20/2010")).await
}

/// Pressing a sortable column header sorts by it, pressing it again reverses the direction.
/// Focus stays on the header.
async fn sorting(page: &Page<'_>) -> Result<(), Report> {
    column_header(page, "Files", "Type").await?.click().await?;
    page.wait_for_text("test-table-sort", "type ascending")
        .await?;
    expect_rows(page, &["Games", "Program Files", "bootmgr", "log.txt"]).await?;
    let kind = column_header(page, "Files", "Type").await?;
    assert_that!(attr(&kind, "aria-sort").await?).is_equal_to(Some("ascending".to_owned()));
    let name = column_header(page, "Files", "Name").await?;
    assert_that!(attr(&name, "aria-sort").await?).is_equal_to(Some("none".to_owned()));

    // The sort describes the table and is announced ("sortable" in useTable's tests).
    let expected = "sorted by column Type in ascending order";
    page.wait_for_selector("[data-live-announcer] [aria-live=assertive] div")
        .await?;
    // Visually hidden: read the text content.
    let announced = page
        .css("[data-live-announcer] [aria-live=assertive]")
        .await?
        .prop("textContent")
        .await?
        .unwrap_or_default();
    assert_that!(announced.as_str()).contains(expected);
    let files = grid(page, "Files").await?;
    let describedby = attr(&files, "aria-describedby").await?.unwrap_or_default();
    let description = page
        .element(&describedby)
        .await?
        .prop("textContent")
        .await?
        .unwrap_or_default();
    assert_that!(description.as_str()).is_equal_to(expected);

    // With the keyboard: Enter on the focused header.
    page.wait_for_focus("columnheader", Some("Type")).await?;
    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_text("test-table-sort", "type descending")
        .await?;
    expect_rows(page, &["log.txt", "bootmgr", "Program Files", "Games"]).await?;
    page.wait_for_focus("columnheader", Some("Type")).await?;

    // Back to sorting by name.
    column_header(page, "Files", "Name").await?.click().await?;
    page.wait_for_text("test-table-sort", "name ascending")
        .await?;
    expect_rows(page, &["bootmgr", "Games", "log.txt", "Program Files"]).await
}

/// Ctrl+A selects all rows; the "select all" checkbox selects all and clears.
async fn select_all(page: &Page<'_>) -> Result<(), Report> {
    focus_row(page, "bootmgr").await?;
    page.send_keys_to_active(Key::Control + "a").await?;
    page.wait_for_text("test-table-selection", "all")
        .await
        .map_err(|e| e.context("after pressing Ctrl+A").into_dynamic())?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_text("test-table-selection", "").await?;

    let select_all = select_all_checkbox(page).await?;
    select_all.click().await?;
    page.wait_for_text("test-table-selection", "all")
        .await
        .map_err(|e| e.context("after checking select all").into_dynamic())?;
    // The disabled row isn't selected.
    let log = row(page, "log.txt").await?;
    assert_that!(attr(&log, "aria-selected").await?).is_equal_to(Some("false".to_owned()));
    select_all.click().await?;
    page.wait_for_text("test-table-selection", "").await?;

    // A row's checkbox selects just that row; "select all" then shows a partial selection.
    row(page, "Games")
        .await?
        .find(By::Css("input[type=checkbox]"))
        .await?
        .click()
        .await?;
    page.wait_for_text("test-table-selection", "Games").await?;
    let indeterminate = page
        .driver
        .execute(
            "return arguments[0].indeterminate",
            vec![select_all.to_json()?],
        )
        .await?
        .json()
        .as_bool();
    assert_that!(indeterminate).is_equal_to(Some(true));
    select_all.click().await?;
    page.wait_for_text("test-table-selection", "all").await?;
    select_all.click().await?;
    page.wait_for_text("test-table-selection", "").await
}

/// Disabled rows (with the table default `DisabledBehavior::Selection`) can be focused, but not
/// selected.
async fn disabled_rows(page: &Page<'_>) -> Result<(), Report> {
    let log = row(page, "log.txt").await?;
    assert_that!(attr(&log, "aria-disabled").await?).is_none();
    let checkbox = log.find(By::Css("input[type=checkbox]")).await?;
    assert_that!(checkbox.is_enabled().await?).is_false();

    focus_row(page, "Games").await?;
    page.send_keys_to_active(Key::Down).await?;
    expect_focus_on_row(page, "log.txt").await?;
    page.send_keys_to_active(Key::Space).await?;
    page.send_keys_to_active(Key::Down).await?;
    expect_focus_on_row(page, "Program Files").await?;
    page.send_keys_to_active(Key::Space).await?;
    page.wait_for_text("test-table-selection", "Program Files")
        .await?;
    page.send_keys_to_active(Key::Space).await?;
    page.wait_for_text("test-table-selection", "").await
}

/// Typing finds rows by their row header.
async fn type_ahead(page: &Page<'_>) -> Result<(), Report> {
    focus_row(page, "bootmgr").await?;
    page.send_keys_to_active("ga").await?;
    expect_focus_on_row(page, "Games").await
}

/// When the focused row is removed, focus moves to the row that took its place, in the same
/// column.
async fn removing_the_focused_row(page: &Page<'_>) -> Result<(), Report> {
    focus_row(page, "Games").await?;
    page.send_keys_to_active(Key::Right).await?;
    page.send_keys_to_active(Key::Right).await?;
    expect_focus_on_row_header(page, "Games").await?;

    let remove = page.element("test-table-remove-games").await?;
    page.driver
        .execute("arguments[0].click()", vec![remove.to_json()?])
        .await?;
    expect_rows(page, &["bootmgr", "log.txt", "Program Files"]).await?;
    expect_focus_on_row_header(page, "log.txt").await
}

/// The table's labels and descriptions follow the locale ("Alles auswählen" in de-DE), also when
/// it changes (fr-FR).
async fn localized(page: &Page<'_>) -> Result<(), Report> {
    let table = grid(page, "Localized").await?;
    let select_all = table.find(By::Css("[role=columnheader] input")).await?;
    let select_row = table
        .find(By::Css("[role=row] [role=gridcell] input"))
        .await?;
    page.wait_for_attr(&select_all, "aria-label", Some("Alles auswählen"))
        .await?;
    page.wait_for_attr(&select_row, "aria-label", Some("Auswählen"))
        .await?;
    wait_for!(
        "the German sort description",
        "sortiert nach Spalte Name in aufsteigender Reihenfolge",
        description(page, &table).await?
    );

    page.click_element_with_id("test-table-to-french").await?;
    page.wait_for_attr(&select_all, "aria-label", Some("Sélectionner tout"))
        .await?;
    page.wait_for_attr(&select_row, "aria-label", Some("Sélectionner"))
        .await?;
    wait_for!(
        "the French sort description",
        "trié en fonction de la colonne\u{a0}Name par ordre croissant",
        description(page, &table).await?
    );
    Ok(())
}

/// The text of the element describing `element` (`aria-describedby`).
async fn description(page: &Page<'_>, element: &WebElement) -> Result<String, Report> {
    let describedby = attr(element, "aria-describedby").await?.unwrap_or_default();
    Ok(page
        .element(&describedby)
        .await?
        .prop("textContent")
        .await?
        .unwrap_or_default())
}

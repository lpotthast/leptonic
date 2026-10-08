// Upstream: react-aria-components/test/Table.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::{Report, prelude::ResultExt};

use crate::{
    pages::{ElementActions, Page, PageActions, xpath},
    polling::wait_for,
};

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

        cases!(
            aria_structure(&page),
            column_groups(&page),
            navigation_into_the_column_headers(&page),
            sorting(&page),
            select_all(&page),
            disabled_rows(&page),
            type_ahead(&page),
            removing_the_focused_row(&page),
            localized(&page),
        );

        Ok(())
    }
}

async fn grid(page: &Page<'_>, label: &str) -> Result<WebElement, Report> {
    page.element(format!("[role=grid][aria-label='{label}']"))
        .await
}

/// The row of the "Files" table whose row header is `name`.
async fn row(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    grid(page, "Files")
        .await?
        .element(xpath(format!(
            ".//*[@role='row'][.//*[@role='rowheader'][normalize-space(.)='{name}']]"
        )))
        .await
}

async fn row_header(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    row(page, name).await?.element("[role=rowheader]").await
}

async fn column_header(page: &Page<'_>, table: &str, text: &str) -> Result<WebElement, Report> {
    grid(page, table)
        .await?
        .element(xpath(format!(
            ".//*[@role='columnheader'][normalize-space(.)='{text}']"
        )))
        .await
}

/// The cell with `text` of the table labelled `table`.
async fn cell(page: &Page<'_>, table: &str, text: &str) -> Result<WebElement, Report> {
    grid(page, table)
        .await?
        .element(xpath(format!(
            ".//*[@role='gridcell'][normalize-space(.)='{text}']"
        )))
        .await
}

/// Focus the row of `name` (programmatically, without pressing it).
async fn focus_row(page: &Page<'_>, name: &str) -> Result<(), Report> {
    row(page, name).await?.focus().await?;
    expect_focus_on_row(page, name).await?;
    Ok(())
}

/// The row header texts of the "Files" table, top to bottom.
async fn row_names(page: &Page<'_>) -> Result<Vec<String>, Report> {
    grid(page, "Files")
        .await?
        .inner_texts("[role=rowheader]")
        .await
}

async fn expect_rows(page: &Page<'_>, expected: &[&str]) -> Result<(), Report> {
    wait_for("the rows")
        .observing(|| row_names(page))
        .to_be_equal_to(expected)
        .await?;
    Ok(())
}

async fn expect_focus_on_row(page: &Page<'_>, name: &str) -> Result<(), Report> {
    let row = row(page, name).await?;
    page.wait_for_focus(&row).await?;
    Ok(())
}

async fn expect_focus_on_row_header(page: &Page<'_>, name: &str) -> Result<(), Report> {
    let header = row_header(page, name).await?;
    page.wait_for_focus(&header).await?;
    Ok(())
}

async fn select_all_checkbox(page: &Page<'_>) -> Result<WebElement, Report> {
    grid(page, "Files")
        .await?
        .element("thead input[type=checkbox]")
        .await
}

async fn expect_focus_on_select_all(page: &Page<'_>) -> Result<(), Report> {
    let checkbox = select_all_checkbox(page).await?;
    page.wait_for_focus(&checkbox).await?;
    Ok(())
}

async fn aria_structure(page: &Page<'_>) -> Result<(), Report> {
    let files = grid(page, "Files").await?;
    assert_that!(files.attr("aria-multiselectable").await?)
        .get_some()
        .is_equal_to("true");
    expect_rows(page, &["bootmgr", "Games", "log.txt", "Program Files"]).await?;

    // The selection column, then Name, Type and Date Modified.
    let headers = files.elements("[role=columnheader]").await?;
    assert_that!(headers).has_length(4);
    let name = column_header(page, "Files", "Name").await?;
    assert_that!(name.attr("aria-sort").await?)
        .get_some()
        .is_equal_to("ascending");
    let kind = column_header(page, "Files", "Type").await?;
    assert_that!(kind.attr("aria-sort").await?)
        .get_some()
        .is_equal_to("none");

    // Rows are labelled by their row header cell.
    let games = row(page, "Games").await?;
    let header = row_header(page, "Games").await?;
    let row_labelledby = header.attr("id").await?;
    assert_that!(games.attr("aria-labelledby").await?).is_equal_to(row_labelledby);
    assert_that!(games.attr("aria-selected").await?)
        .get_some()
        .is_equal_to("false");
    let checkboxes = games.elements("input[type=checkbox]").await?;
    assert_that!(checkboxes.as_slice()).has_length(1);
    // The checkbox is labelled "Select" plus the row header: `aria-labelledby` = its own id
    // (with `aria-label`) and the row header cell.
    let checkbox_id = checkboxes[0].attr("id").await?.unwrap_or_default();
    let labelledby = checkboxes[0].attr("aria-labelledby").await?;
    let header_id = header.attr("id").await?.unwrap_or_default();
    assert_that!(labelledby)
        .get_some()
        .is_equal_to(format!("{checkbox_id} {header_id}"));
    assert_that!(checkboxes[0].attr("aria-label").await?)
        .get_some()
        .is_equal_to("Select");
    Ok(())
}

/// A column group spans its columns in a header row above them; the rest of that row is filled
/// with placeholders.
async fn column_groups(page: &Page<'_>) -> Result<(), Report> {
    let contacts = grid(page, "Contacts").await?;
    let header_rows = contacts.elements("thead [role=row]").await?;
    assert_that!(header_rows.as_slice()).has_length(2);
    let contact = column_header(page, "Contacts", "Contact").await?;
    assert_that!(contact.attr("aria-colspan").await?)
        .get_some()
        .is_equal_to("2");
    assert_that!(contact.attr("aria-colindex").await?)
        .get_some()
        .is_equal_to("2");
    let placeholders = header_rows[0].elements("[role=gridcell]").await?;
    assert_that!(placeholders).has_length(2);

    // Up from a cell reaches its column, then the group.
    let email = cell(page, "Contacts", "alice@example.com").await?;
    let email_column = column_header(page, "Contacts", "Email").await?;
    page.element("#test-table-before-contacts")
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Right).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&email).await?;
    for (key, target) in [
        (Key::Up, &email_column),
        (Key::Up, &contact),
        (Key::Down, &email_column),
        (Key::Down, &email),
    ] {
        page.send_keys(key.clone()).await?;
        page.wait_for_focus(target)
            .await
            .context_with(|| format!("after pressing {key:?}"))?;
    }
    Ok(())
}

/// ArrowUp from the first row moves into the column headers; ArrowLeft/Right move between them
/// (wrapping), ArrowDown back into the body.
async fn navigation_into_the_column_headers(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-table-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    expect_focus_on_row(page, "bootmgr").await?;

    // The selection column's header holds the "select all" checkbox, which gets focus.
    page.send_keys(Key::Up).await?;
    expect_focus_on_select_all(page).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&column_header(page, "Files", "Name").await?)
        .await?;
    page.send_keys(Key::Left).await?;
    expect_focus_on_select_all(page).await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&column_header(page, "Files", "Date Modified").await?)
        .await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&cell(page, "Files", "11/20/2010").await?)
        .await?;
    Ok(())
}

/// Pressing a sortable column header sorts by it, pressing it again reverses the direction.
/// Focus stays on the header.
async fn sorting(page: &Page<'_>) -> Result<(), Report> {
    let sort = page.element("#test-table-sort").await?;
    let kind = column_header(page, "Files", "Type").await?;
    let name = column_header(page, "Files", "Name").await?;
    kind.click().await?;
    sort.wait_for_inner_text("type ascending").await?;
    expect_rows(page, &["Games", "Program Files", "bootmgr", "log.txt"]).await?;
    assert_that!(kind.attr("aria-sort").await?)
        .get_some()
        .is_equal_to("ascending");
    assert_that!(name.attr("aria-sort").await?)
        .get_some()
        .is_equal_to("none");

    // The sort describes the table and is announced ("sortable" in useTable's tests).
    let expected = "sorted by column Type in ascending order";
    let files = grid(page, "Files").await?;
    assert_that!(files.referenced_text("aria-describedby").await?).is_equal_to(expected);
    // The live region keeps earlier announcements: the newest one is among them. Visually
    // hidden: its text content.
    let announcer = page
        .element("[data-live-announcer] [aria-live=assertive]")
        .await?;
    wait_for("the assertive announcements")
        .observing(|| async { Ok(announcer.prop("textContent").await?.unwrap_or_default()) })
        .to_be(&format!("containing {expected:?}"), |text| {
            text.contains(expected)
        })
        .await?;

    // With the keyboard: Enter on the focused header.
    page.wait_for_focus(&kind).await?;
    page.send_keys(Key::Enter).await?;
    sort.wait_for_inner_text("type descending").await?;
    expect_rows(page, &["log.txt", "bootmgr", "Program Files", "Games"]).await?;
    page.focus_stays(&kind).await?;

    // Back to sorting by name.
    name.click().await?;
    sort.wait_for_inner_text("name ascending").await?;
    expect_rows(page, &["bootmgr", "Games", "log.txt", "Program Files"]).await?;
    Ok(())
}

/// Ctrl+A selects all rows; the "select all" checkbox selects all and clears.
async fn select_all(page: &Page<'_>) -> Result<(), Report> {
    let selection = page.element("#test-table-selection").await?;
    focus_row(page, "bootmgr").await?;
    page.send_keys(Key::Control + "a").await?;
    selection
        .wait_for_inner_text("all")
        .await
        .context("after pressing Ctrl+A")?;
    page.send_keys(Key::Escape).await?;
    selection.wait_for_inner_text("").await?;

    let select_all = select_all_checkbox(page).await?;
    select_all.click().await?;
    selection
        .wait_for_inner_text("all")
        .await
        .context("after checking select all")?;
    // The disabled row isn't selected.
    let log = row(page, "log.txt").await?;
    assert_that!(log.attr("aria-selected").await?)
        .get_some()
        .is_equal_to("false");
    select_all.click().await?;
    selection.wait_for_inner_text("").await?;

    // A row's checkbox selects just that row; "select all" then shows a partial selection.
    row(page, "Games")
        .await?
        .element("input[type=checkbox]")
        .await?
        .click()
        .await?;
    selection.wait_for_inner_text("Games").await?;
    select_all.wait_for_prop("indeterminate", "true").await?;
    select_all.click().await?;
    selection.wait_for_inner_text("all").await?;
    select_all.click().await?;
    selection.wait_for_inner_text("").await?;
    Ok(())
}

/// Disabled rows (with the table default `DisabledBehavior::Selection`) can be focused, but not
/// selected.
async fn disabled_rows(page: &Page<'_>) -> Result<(), Report> {
    let log = row(page, "log.txt").await?;
    assert_that!(log.attr("aria-disabled").await?).is_none();
    let checkbox = log.element("input[type=checkbox]").await?;
    assert_that!(checkbox.is_enabled().await?).is_false();

    focus_row(page, "Games").await?;
    page.send_keys(Key::Down).await?;
    expect_focus_on_row(page, "log.txt").await?;
    let selection = page.element("#test-table-selection").await?;
    page.send_keys(Key::Space).await?;
    selection.inner_text_stays("").await?;
    page.send_keys(Key::Down).await?;
    expect_focus_on_row(page, "Program Files").await?;
    page.send_keys(Key::Space).await?;
    selection.wait_for_inner_text("Program Files").await?;
    page.send_keys(Key::Space).await?;
    selection.wait_for_inner_text("").await?;
    Ok(())
}

/// Typing finds rows by their row header.
async fn type_ahead(page: &Page<'_>) -> Result<(), Report> {
    focus_row(page, "bootmgr").await?;
    page.send_keys("ga").await?;
    expect_focus_on_row(page, "Games").await?;
    Ok(())
}

/// When the focused row is removed, focus moves to the row that took its place, in the same
/// column.
async fn removing_the_focused_row(page: &Page<'_>) -> Result<(), Report> {
    focus_row(page, "Games").await?;
    page.send_keys(Key::Right).await?;
    page.send_keys(Key::Right).await?;
    expect_focus_on_row_header(page, "Games").await?;

    let remove = page.element("#test-table-remove-games").await?;
    remove.virtual_click().await?;
    expect_rows(page, &["bootmgr", "log.txt", "Program Files"]).await?;
    expect_focus_on_row_header(page, "log.txt").await?;
    Ok(())
}

/// The table's labels and descriptions follow the locale ("Alles auswählen" in de-DE), also when
/// it changes (fr-FR).
async fn localized(page: &Page<'_>) -> Result<(), Report> {
    let table = grid(page, "Localized").await?;
    let select_all = table.element("[role=columnheader] input").await?;
    let select_row = table.element("[role=row] [role=gridcell] input").await?;
    select_all
        .wait_for_attr("aria-label", Some("Alles auswählen"))
        .await?;
    select_row
        .wait_for_attr("aria-label", Some("Auswählen"))
        .await?;
    wait_for("the German sort description")
        .observing(|| table.referenced_text("aria-describedby"))
        .to_be_equal_to("sortiert nach Spalte Name in aufsteigender Reihenfolge")
        .await?;

    page.element("#test-table-to-french").await?.click().await?;
    select_all
        .wait_for_attr("aria-label", Some("Sélectionner tout"))
        .await?;
    select_row
        .wait_for_attr("aria-label", Some("Sélectionner"))
        .await?;
    wait_for("the French sort description")
        .observing(|| table.referenced_text("aria-describedby"))
        .to_be_equal_to("trié en fonction de la colonne\u{a0}Name par ordre croissant")
        .await?;
    Ok(())
}

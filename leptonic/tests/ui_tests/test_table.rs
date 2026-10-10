// Upstream: react-aria-components/test/Table.test.js @ 99e6102368
//! Behavior of the table hooks (through the `Table` atoms): ARIA structure (column headers,
//! row headers labelling rows, `aria-sort`, column groups), navigation between body and column
//! headers, sorting, select all, disabled rows, type-ahead and refocusing after removing the
//! focused row. Spec: react-aria-components `Table.test.js`.
use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::{Report, prelude::ResultExt};

use crate::pages::{ElementActions, Page, role};

const PATH: &str = "/atoms/table";

async fn grid(page: &Page<'_>, label: &str) -> Result<WebElement, Report> {
    page.element(format!("[role=grid][aria-label='{label}']"))
        .await
}

/// The row of the "Files" table whose row header is `name`.
async fn row(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    grid(page, "Files")
        .await?
        .element(role(AriaRole::Row).has(role(AriaRole::Rowheader).text(name)))
        .await
}

async fn row_header(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    row(page, name).await?.element("[role=rowheader]").await
}

async fn column_header(page: &Page<'_>, table: &str, text: &str) -> Result<WebElement, Report> {
    grid(page, table)
        .await?
        .element(role(AriaRole::Columnheader).text(text))
        .await
}

/// The cell with `text` of the table labelled `table`.
async fn cell(page: &Page<'_>, table: &str, text: &str) -> Result<WebElement, Report> {
    grid(page, table)
        .await?
        .element(role(AriaRole::Gridcell).text(text))
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
    assert_that!(|| row_names(page))
        .eventually_ok()
        .matches(eq(expected))
        .await;
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

/// The table is a multiselectable grid whose column headers carry `aria-sort` and whose rows are
/// labelled by their row header, each with a selection checkbox labelled "Select" plus that header.
#[browser_test]
pub async fn aria_structure(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let files = grid(page, "Files").await?;
    assert_that!(files)
        .has_attribute("aria-multiselectable")
        .await
        .is_equal_to("true");
    expect_rows(page, &["bootmgr", "Games", "log.txt", "Program Files"]).await?;

    // The selection column, then Name, Type and Date Modified.
    let headers = files.elements("[role=columnheader]").await?;
    assert_that!(headers).has_length(4);
    let name = column_header(page, "Files", "Name").await?;
    assert_that!(name)
        .has_attribute("aria-sort")
        .await
        .is_equal_to("ascending");
    let kind = column_header(page, "Files", "Type").await?;
    assert_that!(kind)
        .has_attribute("aria-sort")
        .await
        .is_equal_to("none");

    // Rows are labelled by their row header cell.
    let games = row(page, "Games").await?;
    let header = row_header(page, "Games").await?;
    let row_labelledby = header.attr("id").await?;
    assert_that!(games)
        .attribute("aria-labelledby")
        .await
        .is_equal_to(row_labelledby);
    assert_that!(games)
        .has_attribute("aria-selected")
        .await
        .is_equal_to("false");
    let checkboxes = games.elements("input[type=checkbox]").await?;
    assert_that!(checkboxes.as_slice()).has_length(1);
    // The checkbox is labelled "Select" plus the row header: `aria-labelledby` = its own id
    // (with `aria-label`) and the row header cell.
    let checkbox_id = checkboxes[0].attr("id").await?.unwrap_or_default();
    let header_id = header.attr("id").await?.unwrap_or_default();
    assert_that!(checkboxes[0])
        .has_attribute("aria-labelledby")
        .await
        .is_equal_to(format!("{checkbox_id} {header_id}"));
    assert_that!(checkboxes[0])
        .has_attribute("aria-label")
        .await
        .is_equal_to("Select");
    assert_that!(checkboxes[0])
        .accessible_name()
        .await
        .is_equal_to("Select Games");

    // As in react-aria-components, rows and cells of a flat table are at level 1 (`data-level`
    // and `--table-row-level`), without the treegrid's `aria-level`.
    assert_that!(games)
        .has_attribute("data-level")
        .await
        .is_equal_to("1");
    assert_that!(header)
        .has_attribute("data-level")
        .await
        .is_equal_to("1");
    assert_that!(games).attribute("aria-level").await.is_none();
    let row_level: String = page
        .low_level()
        .eval(
            "return arguments[0].style.getPropertyValue('--table-row-level');",
            vec![games.to_json()?],
        )
        .await?;
    assert_that!(row_level).is_equal_to("1".to_owned());
    Ok(())
}

/// A column group spans its columns in a header row above them, the rest of that row filled with
/// placeholders. ArrowUp from a cell reaches its column, then the group, and ArrowDown goes back.
#[browser_test]
pub async fn column_groups(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let contacts = grid(page, "Contacts").await?;
    let header_rows = contacts.elements("thead [role=row]").await?;
    assert_that!(header_rows.as_slice()).has_length(2);
    let contact = column_header(page, "Contacts", "Contact").await?;
    assert_that!(contact)
        .has_attribute("aria-colspan")
        .await
        .is_equal_to("2");
    assert_that!(contact)
        .has_attribute("aria-colindex")
        .await
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
#[browser_test]
pub async fn navigation_into_the_column_headers(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
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

/// Hovering the table header marks it `data-hovered` ("should support hover events on the
/// TableHeader").
#[browser_test]
pub async fn hover_on_the_table_header(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let header = grid(page, "Files").await?.element("thead").await?;
    assert_that!(header)
        .attribute("data-hovered")
        .await
        .is_none();
    column_header(page, "Files", "Type").await?.hover().await?;
    header.wait_for_attr("data-hovered", Some("true")).await?;
    page.element("h1").await?.hover().await?;
    header.wait_for_attr("data-hovered", None).await?;
    Ok(())
}

/// Sortable column headers are described as "sortable column"; the others (the selection column,
/// unsortable columns) have no description ("should set the proper aria-describedby and aria-sort
/// on sortable column headers", @adobe/react-spectrum `TableTests.js`).
#[browser_test]
pub async fn sortable_columns_are_described(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    for column in ["Name", "Type", "Date Modified"] {
        let header = column_header(page, "Files", column).await?;
        assert_that!(|| header.accessible_description())
            .eventually_ok()
            .matches(eq("sortable column".to_owned()))
            .await;
    }
    let notes = column_header(page, "Contacts", "Notes").await?;
    assert_that!(notes)
        .attribute("aria-describedby")
        .await
        .is_none();
    Ok(())
}

/// Pressing a sortable column header (or Enter on it) sorts by it and pressing it again reverses
/// the direction; the sort describes the table and is announced ("should support sorting").
#[browser_test]
pub async fn sorting(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let sort = page.element("#test-table-sort").await?;
    let kind = column_header(page, "Files", "Type").await?;
    let name = column_header(page, "Files", "Name").await?;
    kind.click().await?;
    sort.wait_for_inner_text("type ascending").await?;
    expect_rows(page, &["Games", "Program Files", "bootmgr", "log.txt"]).await?;
    assert_that!(kind)
        .has_attribute("aria-sort")
        .await
        .is_equal_to("ascending");
    assert_that!(name)
        .has_attribute("aria-sort")
        .await
        .is_equal_to("none");

    // The sort describes the table and is announced ("sortable" in useTable's tests).
    let expected = "sorted by column Type in ascending order";
    let files = grid(page, "Files").await?;
    assert_that!(files)
        .accessible_description()
        .await
        .is_equal_to(expected);
    // The live region keeps earlier announcements: the newest one is among them. Visually
    // hidden: its text content.
    let announcer = page
        .element("[data-live-announcer] [aria-live=assertive]")
        .await?;
    assert_that!(|| async {
        Ok::<_, Report>(announcer.prop("textContent").await?.unwrap_or_default())
    })
    .eventually_ok()
    .satisfies(|text| {
        text.contains(expected);
    })
    .await;

    // With the keyboard: Enter on the focused header.
    page.wait_for_focus(&kind).await?;
    page.send_keys(Key::Enter).await?;
    sort.wait_for_inner_text("type descending").await?;
    expect_rows(page, &["log.txt", "bootmgr", "Program Files", "Games"]).await?;
    page.focus_stays(&kind, std::time::Duration::from_millis(100))
        .await?;

    // Back to sorting by name.
    name.click().await?;
    sort.wait_for_inner_text("name ascending").await?;
    expect_rows(page, &["bootmgr", "Games", "log.txt", "Program Files"]).await?;
    Ok(())
}

/// Ctrl+A selects all rows and Escape clears them ("should support select all with Mod+A"). The
/// "select all" checkbox selects all enabled rows, shows a partial selection and clears.
#[browser_test]
pub async fn select_all(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let selection = page.element("#test-table-selection").await?;
    focus_row(page, "bootmgr").await?;
    page.send_keys(page.primary_modifier().await? + "a").await?;
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
    assert_that!(log)
        .has_attribute("aria-selected")
        .await
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

/// Disabled rows (with the table default `DisabledBehavior::Selection`) can be focused with the
/// arrow keys, but neither Space nor their disabled checkbox selects them.
#[browser_test]
pub async fn disabled_rows(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let log = row(page, "log.txt").await?;
    assert_that!(log).attribute("aria-disabled").await.is_none();
    let checkbox = log.element("input[type=checkbox]").await?;
    assert_that!(checkbox).enabled().await.is_false();

    focus_row(page, "Games").await?;
    page.send_keys(Key::Down).await?;
    expect_focus_on_row(page, "log.txt").await?;
    let selection = page.element("#test-table-selection").await?;
    page.send_keys(Key::Space).await?;
    selection
        .inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    page.send_keys(Key::Down).await?;
    expect_focus_on_row(page, "Program Files").await?;
    page.send_keys(Key::Space).await?;
    selection.wait_for_inner_text("Program Files").await?;
    page.send_keys(Key::Space).await?;
    selection.wait_for_inner_text("").await?;
    Ok(())
}

/// Typing focuses the row whose row header starts with the typed text ("should support
/// columnHeader typeahead").
#[browser_test]
pub async fn type_ahead(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_row(page, "bootmgr").await?;
    page.send_keys("ga").await?;
    expect_focus_on_row(page, "Games").await?;
    Ok(())
}

/// When the focused row is removed, focus moves to the row that took its place, in the same
/// column ("supports removing rows").
#[browser_test]
pub async fn removing_the_focused_row(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
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
#[browser_test]
pub async fn localized(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let table = grid(page, "Localized").await?;
    let select_all = table.element("[role=columnheader] input").await?;
    let select_row = table.element("[role=row] [role=gridcell] input").await?;
    select_all
        .wait_for_attr("aria-label", Some("Alles auswählen"))
        .await?;
    select_row
        .wait_for_attr("aria-label", Some("Auswählen"))
        .await?;
    assert_that!(|| table.accessible_description())
        .eventually_ok()
        .matches(eq("sortiert nach Spalte Name in aufsteigender Reihenfolge"))
        .await;

    page.element("#test-table-to-french").await?.click().await?;
    select_all
        .wait_for_attr("aria-label", Some("Sélectionner tout"))
        .await?;
    select_row
        .wait_for_attr("aria-label", Some("Sélectionner"))
        .await?;
    assert_that!(|| table.accessible_description())
        .eventually_ok()
        .matches(eq(
            "trié en fonction de la colonne\u{a0}Name par ordre croissant",
        ))
        .await;
    Ok(())
}

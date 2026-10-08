// Upstream: react-aria-components/test/Treeble.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// Tree tables (react-aria-components' `Treeble.test.js`): the treegrid structure, expanding and
/// collapsing rows by mouse and keyboard (also right to left), default and controlled expanded
/// keys, keyboard navigation of the flattened rows and into cells, and selection.
pub struct TableTreeTests {}

const PATH: &str = "/atoms/table-tree";

#[async_trait]
impl BrowserTest<str> for TableTreeTests {
    fn name(&self) -> Cow<'_, str> {
        "table_tree_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path(PATH).await?;

        renders_a_treegrid(&page).await?;
        expands_a_row_with_the_mouse(&page).await?;
        expands_a_row_with_the_keyboard(&page, "files", Key::Right, Key::Left).await?;
        expands_a_row_with_the_keyboard(&page, "rtl", Key::Left, Key::Right).await?;
        default_expanded_keys(&page).await?;
        controlled_expanded_keys(&page).await?;
        keyboard_navigation_of_flattened_rows(&page).await?;
        keyboard_navigation_of_cells(&page).await?;
        selection(&page).await?;
        type_ahead_searches_the_rows_shown(&page).await?;
        arrow_left_moves_from_a_child_row_to_its_parent(&page).await?;
        collapsing_from_outside_moves_focus_to_the_parent(&page).await?;
        leaf_rows_are_never_expanded(&page).await?;

        page.expect_no_page_errors().await
    }
}

/// The visible body rows of the table `id`.
async fn rows(page: &Page<'_>, id: &str) -> Result<Vec<WebElement>, Report> {
    Ok(page
        .driver
        .find_all(By::Css(format!(
            "#test-tt-{id} tbody [role=row]:not([hidden])"
        )))
        .await?)
}

async fn attr(element: &WebElement, name: &str) -> Result<Option<String>, Report> {
    Ok(element.attr(name).await?)
}

/// The row header's text of each visible row.
async fn row_names(page: &Page<'_>, id: &str) -> Result<Vec<String>, Report> {
    let mut names = Vec::new();
    for row in rows(page, id).await? {
        names.push(
            row.find(By::Css("[role=rowheader]"))
                .await?
                .text()
                .await?
                .trim_start_matches('>')
                .trim()
                .to_owned(),
        );
    }
    Ok(names)
}

/// Checks a visible row's tree attributes: `aria-expanded` (`None`: none), level, position and
/// set size.
async fn expect_row(
    row: &WebElement,
    expanded: Option<&str>,
    level: usize,
    position: usize,
    set_size: usize,
) -> Result<(), Report> {
    let name = row.text().await?;
    assert_that!(attr(row, "aria-expanded").await?)
        .with_detail_message(format!("aria-expanded of {name}"))
        .is_equal_to(expanded.map(str::to_owned));
    assert_that!(attr(row, "aria-level").await?)
        .with_detail_message(format!("aria-level of {name}"))
        .is_equal_to(Some(level.to_string()));
    assert_that!(attr(row, "aria-posinset").await?)
        .with_detail_message(format!("aria-posinset of {name}"))
        .is_equal_to(Some(position.to_string()));
    assert_that!(attr(row, "aria-setsize").await?)
        .with_detail_message(format!("aria-setsize of {name}"))
        .is_equal_to(Some(set_size.to_string()));
    assert_that!(
        attr(row, "style")
            .await?
            .unwrap_or_default()
            .replace(' ', "")
    )
    .with_detail_message(format!("style of {name}"))
    .contains(format!("--table-row-level:{level}"));
    Ok(())
}

/// The visible rows of the table `id` once there are `count` of them.
async fn wait_for_rows(page: &Page<'_>, id: &str, count: u64) -> Result<(), Report> {
    page.wait_for_count(
        &format!("#test-tt-{id} tbody [role=row]:not([hidden])"),
        count,
    )
    .await
}

/// "renders a treegrid".
async fn renders_a_treegrid(page: &Page<'_>) -> Result<(), Report> {
    let table = page.css("#test-tt-files table").await?;
    assert_that!(attr(&table, "role").await?).is_equal_to(Some("treegrid".to_owned()));
    wait_for_rows(page, "files", 4).await?;
    let rows = rows(page, "files").await?;
    expect_row(&rows[0], Some("false"), 1, 1, 4).await?;
    assert_that!(attr(&rows[0], "data-expanded").await?).is_none();
    assert_that!(attr(&rows[0], "data-has-child-items").await?).is_some();
    assert_that!(attr(&rows[0], "data-level").await?).is_equal_to(Some("1".to_owned()));
    expect_row(&rows[1], Some("false"), 1, 2, 4).await?;
    expect_row(&rows[2], None, 1, 3, 4).await?;
    assert_that!(attr(&rows[2], "data-has-child-items").await?).is_none();
    expect_row(&rows[3], None, 1, 4, 4).await?;
    assert_that!(row_names(page, "files").await?).is_equal_to(
        [
            "Games",
            "Applications",
            "2024 Financial Report",
            "Job Posting",
        ]
        .map(str::to_owned)
        .to_vec(),
    );

    // The tree column's cells are marked; the row's cells share its state.
    let row_header = rows[0].find(By::Css("[role=rowheader]")).await?;
    assert_that!(attr(&row_header, "data-tree-column").await?).is_some();
    for cell in rows[0].find_all(By::Css("[role=gridcell]")).await? {
        assert_that!(attr(&cell, "data-tree-column").await?).is_none();
        assert_that!(attr(&cell, "data-has-child-items").await?).is_some();
        assert_that!(attr(&cell, "data-level").await?).is_equal_to(Some("1".to_owned()));
    }

    // The expand button: "Expand" plus the row, out of the tab order; none on leaf rows.
    let button = row_header.find(By::Css("button")).await?;
    assert_that!(attr(&button, "aria-label").await?).is_equal_to(Some("Expand".to_owned()));
    let button_id = attr(&button, "id").await?.unwrap_or_default();
    let row_header_id = attr(&row_header, "id").await?.unwrap_or_default();
    assert_that!(attr(&button, "aria-labelledby").await?)
        .is_equal_to(Some(format!("{button_id} {row_header_id}")));
    assert_that!(attr(&button, "tabindex").await?).is_equal_to(Some("-1".to_owned()));
    let leaf_button = rows[2].find(By::Css("button")).await?;
    assert_that!(leaf_button.is_displayed().await?).is_false();
    Ok(())
}

/// Checks the rows of a table with Games expanded (7 visible rows).
async fn expect_games_expanded(page: &Page<'_>, id: &str) -> Result<(), Report> {
    wait_for_rows(page, id, 7).await?;
    let rows = rows(page, id).await?;
    page.wait_for_attr(&rows[0], "aria-expanded", Some("true"))
        .await?;
    expect_row(&rows[0], Some("true"), 1, 1, 4).await?;
    assert_that!(attr(&rows[0], "data-expanded").await?).is_some();
    expect_row(&rows[1], None, 2, 1, 3).await?;
    expect_row(&rows[2], None, 2, 2, 3).await?;
    expect_row(&rows[3], None, 2, 3, 3).await?;
    expect_row(&rows[4], Some("false"), 1, 2, 4).await?;
    expect_row(&rows[5], None, 1, 3, 4).await?;
    assert_that!(row_names(page, id).await?[1..4].to_vec()).is_equal_to(
        ["Mario Kart", "Tetris", "Pac-Man"]
            .map(str::to_owned)
            .to_vec(),
    );
    Ok(())
}

/// "should expand a row with mouse": the expand button toggles the row.
async fn expands_a_row_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let button = rows(page, "files").await?[0]
        .find(By::Css("button"))
        .await?;
    button.click().await?;
    expect_games_expanded(page, "files").await?;
    assert_that!(attr(&button, "aria-label").await?).is_equal_to(Some("Collapse".to_owned()));
    button.click().await?;
    wait_for_rows(page, "files", 4).await
}

/// "should expand a row with keyboard" (`ltr`, `rtl`): the expand key on the focused row
/// expands it, the collapse key collapses it.
async fn expands_a_row_with_the_keyboard(
    page: &Page<'_>,
    id: &str,
    expand: Key,
    collapse: Key,
) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.click_element_with_id(&format!("test-tt-before-{id}"))
        .await?;
    page.press_tab().await?;
    let first = rows(page, id).await?[0].clone();
    page.wait_for_focus_on(&first, "the first row").await?;
    page.send_keys_to_active(expand).await?;
    expect_games_expanded(page, id).await?;
    page.wait_for_focus_on(&first, "the first row").await?;
    page.send_keys_to_active(collapse).await?;
    wait_for_rows(page, id, 4).await
}

/// "should support defaultExpandedKeys": Games starts expanded; expanding and collapsing
/// Applications reports the expanded keys.
async fn default_expanded_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    expect_games_expanded(page, "default").await?;
    let apps = rows(page, "default").await?[4].clone();
    apps.find(By::Css("button")).await?.click().await?;
    page.wait_for_text("test-tt-default-expanded", "apps,games")
        .await?;
    wait_for_rows(page, "default", 10).await?;
    let rows_now = rows(page, "default").await?;
    expect_row(&rows_now[4], Some("true"), 1, 2, 4).await?;
    expect_row(&rows_now[5], None, 2, 1, 3).await?;
    expect_row(&rows_now[7], None, 2, 3, 3).await?;
    expect_row(&rows_now[8], None, 1, 3, 4).await?;
    apps.find(By::Css("button")).await?.click().await?;
    page.wait_for_text("test-tt-default-expanded", "games")
        .await?;
    wait_for_rows(page, "default", 7).await
}

/// "should support expandedKeys": controlled without a setter, a press reports the change but
/// the rows stay.
async fn controlled_expanded_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    expect_games_expanded(page, "controlled").await?;
    rows(page, "controlled").await?[4]
        .find(By::Css("button"))
        .await?
        .click()
        .await?;
    page.wait_for_text("test-tt-controlled-expanded", "apps,games")
        .await?;
    stays!(
        "the controlled table's visible rows",
        7,
        rows(page, "controlled").await?.len()
    );
    Ok(())
}

/// "supports keyboard navigation of flattened rows": ArrowDown walks every visible row, Home
/// and End reach the first and the last.
async fn keyboard_navigation_of_flattened_rows(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.click_element_with_id("test-tt-before-default").await?;
    page.press_tab().await?;
    let visible = rows(page, "default").await?;
    for (index, row) in visible.iter().enumerate() {
        page.wait_for_focus_on(row, &format!("row {index}")).await?;
        page.send_keys_to_active(Key::Down).await?;
    }
    page.send_keys_to_active(Key::Home).await?;
    page.wait_for_focus_on(&visible[0], "the first row").await?;
    page.send_keys_to_active(Key::End).await?;
    page.wait_for_focus_on(&visible[visible.len() - 1], "the last row")
        .await
}

/// "supports keyboard navigation of cells": ArrowRight first expands the row, then walks its
/// cells and back to the row; ArrowLeft first collapses it, then walks the cells backwards.
async fn keyboard_navigation_of_cells(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.click_element_with_id("test-tt-before-files").await?;
    page.press_tab().await?;
    let first = rows(page, "files").await?[0].clone();
    page.wait_for_focus_on(&first, "the first row").await?;
    page.send_keys_to_active(Key::Right).await?;
    page.wait_for_attr(&first, "aria-expanded", Some("true"))
        .await?;
    page.wait_for_focus_on(&first, "the first row").await?;
    let mut cells = vec![first.find(By::Css("[role=rowheader]")).await?];
    cells.extend(first.find_all(By::Css("[role=gridcell]")).await?);
    for cell in &cells {
        page.send_keys_to_active(Key::Right).await?;
        page.wait_for_focus_on(cell, "the next cell").await?;
    }
    page.send_keys_to_active(Key::Right).await?;
    page.wait_for_focus_on(&first, "the row").await?;
    page.send_keys_to_active(Key::Left).await?;
    page.wait_for_attr(&first, "aria-expanded", Some("false"))
        .await?;
    page.wait_for_focus_on(&first, "the row").await?;
    for cell in cells.iter().rev() {
        page.send_keys_to_active(Key::Left).await?;
        page.wait_for_focus_on(cell, "the previous cell").await?;
    }
    page.send_keys_to_active(Key::Left).await?;
    page.wait_for_focus_on(&first, "the row").await
}

/// "supports selection": a row and a range across levels.
async fn selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let visible = rows(page, "default").await?;
    visible[0]
        .find(By::Css("[role=gridcell]"))
        .await?
        .click()
        .await?;
    page.wait_for_text("test-tt-default-selection", "games")
        .await?;
    let target = visible[2].find(By::Css("[role=gridcell]")).await?;
    page.driver
        .action_chain()
        .key_down(Key::Shift)
        .click_element(&target)
        .key_up(Key::Shift)
        .perform()
        .await?;
    page.wait_for_text("test-tt-default-selection", "games,mario,tetris")
        .await
}

/// Tabs from the "Before" button into the table `id`, onto its first row.
async fn enter(page: &Page<'_>, id: &str) -> Result<Vec<WebElement>, Report> {
    page.goto_path(PATH).await?;
    page.click_element_with_id(&format!("test-tt-before-{id}"))
        .await?;
    page.press_tab().await?;
    let visible = rows(page, id).await?;
    page.wait_for_focus_on(&visible[0], "the first row").await?;
    Ok(visible)
}

/// Type-ahead (react-aria's `TableKeyboardDelegate.getKeyForSearch`, which steps with
/// `getKeyBelow`) walks the rows shown: child rows of expanded rows, not those of collapsed ones.
async fn type_ahead_searches_the_rows_shown(page: &Page<'_>) -> Result<(), Report> {
    let visible = enter(page, "default").await?;
    page.send_keys_to_active("te").await?;
    page.wait_for_focus_on(&visible[2], "Tetris").await?;

    // From a child row, on to a later child row.
    let visible = enter(page, "default").await?;
    page.send_keys_to_active(Key::Down).await?;
    page.wait_for_focus_on(&visible[1], "Mario Kart").await?;
    page.send_keys_to_active("p").await?;
    page.wait_for_focus_on(&visible[3], "Pac-Man").await?;

    // Lightroom is under the collapsed Applications; no row shown starts with "l".
    let visible = enter(page, "default").await?;
    page.send_keys_to_active("l").await?;
    stays!(
        "focus on Games",
        true,
        page.driver.active_element().await? == visible[0]
    );
    Ok(())
}

/// ArrowLeft on a child row (a leaf) moves focus to its parent row, which stays expanded.
async fn arrow_left_moves_from_a_child_row_to_its_parent(page: &Page<'_>) -> Result<(), Report> {
    let visible = enter(page, "default").await?;
    page.send_keys_to_active(Key::Down).await?;
    page.send_keys_to_active(Key::Down).await?;
    page.wait_for_focus_on(&visible[2], "Tetris").await?;
    page.send_keys_to_active(Key::Left).await?;
    page.wait_for_focus_on(&visible[0], "Games").await?;
    assert_that!(attr(&visible[0], "aria-expanded").await?).is_equal_to(Some("true".to_owned()));
    Ok(())
}

/// Collapsing a row from outside the table (the app's bound expanded keys) while one of its
/// child rows has focus moves focus to the row: Tab back into the table lands on Games, not on
/// whichever row took the hidden one's place.
async fn collapsing_from_outside_moves_focus_to_the_parent(page: &Page<'_>) -> Result<(), Report> {
    let visible = enter(page, "bound").await?;
    for _ in 0..3 {
        page.send_keys_to_active(Key::Down).await?;
    }
    page.wait_for_focus_on(&visible[3], "Pac-Man").await?;
    page.click_element_with_id("test-tt-collapse-all").await?;
    wait_for_rows(page, "bound", 4).await?;
    page.press_tab().await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&visible[0], "Games").await
}

/// A leaf row whose key is among the expanded keys is not expanded (react-aria-components:
/// `hasChildItems && expandedKeys.has(key)`).
async fn leaf_rows_are_never_expanded(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    wait_for_rows(page, "bound", 7).await?;
    let report = rows(page, "bound").await?[5].clone();
    assert_that!(row_names(page, "bound").await?[5].clone())
        .is_equal_to("2024 Financial Report".to_owned());
    expect_row(&report, None, 1, 3, 4).await?;
    assert_that!(attr(&report, "data-expanded").await?).is_none();
    Ok(())
}

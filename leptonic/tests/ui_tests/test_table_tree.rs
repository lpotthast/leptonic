// Upstream: react-aria-components/test/Treeble.test.js @ 99e6102368
//! Tree tables (react-aria-components' `Treeble.test.js`): the treegrid structure, expanding and
//! collapsing rows by mouse and keyboard (also right to left), default and controlled expanded
//! keys, keyboard navigation of the flattened rows and into cells, and selection.
use assertr::prelude::*;
use browser_test::{browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::pages::{ElementActions, Page};

const PATH: &str = "/atoms/table-tree";

/// The visible body rows of the table `id` (at least one).
async fn rows(page: &Page<'_>, id: &str) -> Result<Vec<WebElement>, Report> {
    page.elements(format!("#test-tt-{id} tbody [role=row]:not([hidden])"))
        .await
}

/// The row header's text of each visible row.
async fn row_names(page: &Page<'_>, id: &str) -> Result<Vec<String>, Report> {
    let mut names = Vec::new();
    for row in rows(page, id).await? {
        names.push(
            row.element("[role=rowheader]")
                .await?
                .inner_text()
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
    page: &Page<'_>,
    row: &WebElement,
    expanded: Option<&str>,
    level: usize,
    position: usize,
    set_size: usize,
) -> Result<(), Report> {
    let name = row.inner_text().await?;
    assert_that!(row)
        .attribute("aria-expanded")
        .await
        .derive_owned(|value| value.as_deref())
        .with_detail_message(format!("aria-expanded of {name}"))
        .is_equal_to(expanded);
    assert_that!(row)
        .with_detail_message(format!("aria-level of {name}"))
        .has_attribute("aria-level")
        .await
        .is_equal_to(level.to_string());
    assert_that!(row)
        .with_detail_message(format!("aria-posinset of {name}"))
        .has_attribute("aria-posinset")
        .await
        .is_equal_to(position.to_string());
    assert_that!(row)
        .with_detail_message(format!("aria-setsize of {name}"))
        .has_attribute("aria-setsize")
        .await
        .is_equal_to(set_size.to_string());
    let row_level: String = page
        .low_level()
        .eval(
            "return arguments[0].style.getPropertyValue('--table-row-level');",
            vec![row.to_json()?],
        )
        .await?;
    assert_that!(row_level)
        .with_detail_message(format!("--table-row-level of {name}"))
        .is_equal_to(level.to_string());
    Ok(())
}

/// The visible rows of the table `id` once there are `count` of them.
async fn wait_for_rows(page: &Page<'_>, id: &str, count: usize) -> Result<(), Report> {
    page.wait_for_count(
        &format!("#test-tt-{id} tbody [role=row]:not([hidden])"),
        count,
    )
    .await?;
    Ok(())
}

/// The table is a treegrid whose rows carry their level, position and expanded state, with an
/// "Expand" button outside the tab order on rows with children only ("renders a treegrid").
#[browser_test]
pub async fn renders_a_treegrid(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let table = page.element("#test-tt-files table").await?;
    assert_that!(table)
        .has_attribute("role")
        .await
        .is_equal_to("treegrid");
    wait_for_rows(page, "files", 4).await?;
    let rows = rows(page, "files").await?;
    expect_row(page, &rows[0], Some("false"), 1, 1, 4).await?;
    assert_that!(rows[0])
        .attribute("data-expanded")
        .await
        .is_none();
    assert_that!(rows[0])
        .has_attribute("data-has-child-items")
        .await;
    assert_that!(rows[0])
        .has_attribute("data-level")
        .await
        .is_equal_to("1");
    expect_row(page, &rows[1], Some("false"), 1, 2, 4).await?;
    expect_row(page, &rows[2], None, 1, 3, 4).await?;
    assert_that!(rows[2])
        .attribute("data-has-child-items")
        .await
        .is_none();
    expect_row(page, &rows[3], None, 1, 4, 4).await?;
    assert_that!(row_names(page, "files").await?).contains_exactly([
        "Games",
        "Applications",
        "2024 Financial Report",
        "Job Posting",
    ]);

    // The tree column's cells are marked; the row's cells share its state.
    let row_header = rows[0].element("[role=rowheader]").await?;
    assert_that!(row_header)
        .has_attribute("data-tree-column")
        .await;
    for cell in rows[0].elements("[role=gridcell]").await? {
        assert_that!(cell)
            .attribute("data-tree-column")
            .await
            .is_none();
        assert_that!(cell)
            .has_attribute("data-has-child-items")
            .await;
        assert_that!(cell)
            .has_attribute("data-level")
            .await
            .is_equal_to("1");
    }

    // The expand button: "Expand" plus the row, out of the tab order; none on leaf rows. (Labelled
    // by the row's text: react-aria's reference to the row header, which contains the button,
    // makes browsers name it "Expand Expand Games".)
    let button = row_header.element("button").await?;
    assert_that!(button)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Expand Games");
    assert_that!(button)
        .attribute("aria-labelledby")
        .await
        .is_none();
    assert_that!(button)
        .accessible_name()
        .await
        .is_equal_to("Expand Games");
    assert_that!(button)
        .has_attribute("tabindex")
        .await
        .is_equal_to("-1");
    let leaf_button = rows[2].element("button").await?;
    assert_that!(leaf_button).displayed().await.is_false();
    Ok(())
}

/// Checks the rows of a table with Games expanded (7 visible rows).
async fn expect_games_expanded(page: &Page<'_>, id: &str) -> Result<(), Report> {
    wait_for_rows(page, id, 7).await?;
    let rows = rows(page, id).await?;
    rows[0].wait_for_attr("aria-expanded", Some("true")).await?;
    expect_row(page, &rows[0], Some("true"), 1, 1, 4).await?;
    assert_that!(rows[0]).has_attribute("data-expanded").await;
    expect_row(page, &rows[1], None, 2, 1, 3).await?;
    expect_row(page, &rows[2], None, 2, 2, 3).await?;
    expect_row(page, &rows[3], None, 2, 3, 3).await?;
    expect_row(page, &rows[4], Some("false"), 1, 2, 4).await?;
    expect_row(page, &rows[5], None, 1, 3, 4).await?;
    assert_that!(row_names(page, id).await?[1..4].to_vec()).contains_exactly([
        "Mario Kart",
        "Tetris",
        "Pac-Man",
    ]);
    Ok(())
}

/// Clicking a row's expand button shows its child rows, relabels it "Collapse Games" and focuses the
/// row (also on a page where nothing had focus); clicking again hides them ("should expand a row
/// with mouse (ltr)").
#[browser_test]
pub async fn expands_a_row_with_the_mouse(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let games = rows(page, "files").await?[0].clone();
    let button = games.element("button").await?;
    button.click().await?;
    expect_games_expanded(page, "files").await?;
    assert_that!(button)
        .accessible_name()
        .await
        .is_equal_to("Collapse Games");
    page.wait_for_focus(&games).await?;
    button.click().await?;
    wait_for_rows(page, "files", 4).await?;
    Ok(())
}

/// ArrowRight expands the focused row and ArrowLeft collapses it ("should expand a row with
/// keyboard (ltr)").
#[browser_test]
pub async fn expands_a_row_with_the_keyboard(page: &Page<'_>) -> Result<(), Report> {
    expand_and_collapse_with_the_keyboard(page, "files", Key::Right, Key::Left).await
}

/// Right to left, ArrowLeft expands the focused row and ArrowRight collapses it ("should expand a
/// row with keyboard (rtl)").
#[browser_test]
pub async fn expands_a_row_with_the_keyboard_rtl(page: &Page<'_>) -> Result<(), Report> {
    expand_and_collapse_with_the_keyboard(page, "rtl", Key::Left, Key::Right).await
}

/// The expand key on the focused row of the table `id` expands it, the collapse key collapses
/// it.
async fn expand_and_collapse_with_the_keyboard(
    page: &Page<'_>,
    id: &str,
    expand: Key,
    collapse: Key,
) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element(format!("#test-tt-before-{id}"))
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    let first = rows(page, id).await?[0].clone();
    page.wait_for_focus(&first).await?;
    page.send_keys(expand).await?;
    expect_games_expanded(page, id).await?;
    page.wait_for_focus(&first).await?;
    page.send_keys(collapse).await?;
    wait_for_rows(page, id, 4).await?;
    Ok(())
}

/// A row in the default expanded keys starts expanded, and expanding and collapsing another row
/// reports the new expanded keys ("should support defaultExpandedKeys").
#[browser_test]
pub async fn default_expanded_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    expect_games_expanded(page, "default").await?;
    let apps = rows(page, "default").await?[4].clone();
    let expanded = page.element("#test-tt-default-expanded").await?;
    apps.element("button").await?.click().await?;
    expanded.wait_for_inner_text("apps,games").await?;
    wait_for_rows(page, "default", 10).await?;
    let rows_now = rows(page, "default").await?;
    expect_row(page, &rows_now[4], Some("true"), 1, 2, 4).await?;
    expect_row(page, &rows_now[5], None, 2, 1, 3).await?;
    expect_row(page, &rows_now[7], None, 2, 3, 3).await?;
    expect_row(page, &rows_now[8], None, 1, 3, 4).await?;
    apps.element("button").await?.click().await?;
    expanded.wait_for_inner_text("games").await?;
    wait_for_rows(page, "default", 7).await?;
    Ok(())
}

/// With controlled expanded keys and no setter, pressing an expand button reports the change but
/// the rows stay as they are ("should support expandedKeys").
#[browser_test]
pub async fn controlled_expanded_keys(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    expect_games_expanded(page, "controlled").await?;
    rows(page, "controlled").await?[4]
        .element("button")
        .await?
        .click()
        .await?;
    page.element("#test-tt-controlled-expanded")
        .await?
        .wait_for_inner_text("apps,games")
        .await?;
    page.count_stays(
        "#test-tt-controlled tbody [role=row]:not([hidden])",
        7,
        std::time::Duration::from_millis(100),
    )
    .await?;
    Ok(())
}

/// ArrowDown walks every visible row across levels, and Home and End reach the first and the last
/// ("supports keyboard navigation of flattened rows").
#[browser_test]
pub async fn keyboard_navigation_of_flattened_rows(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-tt-before-default")
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    let visible = rows(page, "default").await?;
    for row in &visible {
        page.wait_for_focus(row).await?;
        page.send_keys(Key::Down).await?;
    }
    page.send_keys(Key::Home).await?;
    page.wait_for_focus(&visible[0]).await?;
    page.send_keys(Key::End).await?;
    page.wait_for_focus(&visible[visible.len() - 1]).await?;
    Ok(())
}

/// On a collapsed row, ArrowRight first expands it, then walks its cells and back to the row;
/// ArrowLeft first collapses it, then walks the cells backwards ("supports keyboard navigation of
/// cells").
#[browser_test]
pub async fn keyboard_navigation_of_cells(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-tt-before-files").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let first = rows(page, "files").await?[0].clone();
    page.wait_for_focus(&first).await?;
    page.send_keys(Key::Right).await?;
    first.wait_for_attr("aria-expanded", Some("true")).await?;
    page.wait_for_focus(&first).await?;
    let mut cells = vec![first.element("[role=rowheader]").await?];
    cells.extend(first.elements("[role=gridcell]").await?);
    for cell in &cells {
        page.send_keys(Key::Right).await?;
        page.wait_for_focus(cell).await?;
    }
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&first).await?;
    page.send_keys(Key::Left).await?;
    first.wait_for_attr("aria-expanded", Some("false")).await?;
    page.wait_for_focus(&first).await?;
    for cell in cells.iter().rev() {
        page.send_keys(Key::Left).await?;
        page.wait_for_focus(cell).await?;
    }
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&first).await?;
    Ok(())
}

/// Clicking a row selects it, and Shift+click selects the range up to a child row, across levels
/// ("supports selection").
#[browser_test]
pub async fn selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let visible = rows(page, "default").await?;
    visible[0]
        .first_element("[role=gridcell]")
        .await?
        .click()
        .await?;
    let selection = page.element("#test-tt-default-selection").await?;
    selection.wait_for_inner_text("games").await?;
    let target = visible[2].first_element("[role=gridcell]").await?;
    page.low_level()
        .driver()
        .action_chain()
        .key_down(Key::Shift)
        .click_element(&target)
        .key_up(Key::Shift)
        .perform()
        .await?;
    selection.wait_for_inner_text("games,mario,tetris").await?;
    Ok(())
}

/// Tabs from the "Before" button into the table `id`, onto its first row.
async fn enter(page: &Page<'_>, id: &str) -> Result<Vec<WebElement>, Report> {
    page.goto_path(PATH).await?;
    page.element(format!("#test-tt-before-{id}"))
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    let visible = rows(page, id).await?;
    page.wait_for_focus(&visible[0]).await?;
    Ok(visible)
}

/// Typing finds rows among the rows shown, including child rows of expanded rows, but not child
/// rows of collapsed ones.
#[browser_test]
pub async fn type_ahead_searches_the_rows_shown(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let visible = enter(page, "default").await?;
    page.send_keys("te").await?;
    page.wait_for_focus(&visible[2]).await?;

    // From a child row, on to a later child row.
    let visible = enter(page, "default").await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&visible[1]).await?;
    page.send_keys("p").await?;
    page.wait_for_focus(&visible[3]).await?;

    // Lightroom is under the collapsed Applications; no row shown starts with "l".
    let visible = enter(page, "default").await?;
    page.send_keys("l").await?;
    page.focus_stays(&visible[0], std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// ArrowLeft on a child row (a leaf) moves focus to its parent row, which stays expanded.
#[browser_test]
pub async fn arrow_left_moves_from_a_child_row_to_its_parent(
    page: &Page<'_>,
) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let visible = enter(page, "default").await?;
    page.send_keys(Key::Down).await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&visible[2]).await?;
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&visible[0]).await?;
    assert_that!(visible[0])
        .has_attribute("aria-expanded")
        .await
        .is_equal_to("true");
    Ok(())
}

/// When the app collapses a row through its bound expanded keys while a child row has focus,
/// focus moves to the collapsed row, so tabbing back into the table lands on it.
#[browser_test]
pub async fn collapsing_from_outside_moves_focus_to_the_parent(
    page: &Page<'_>,
) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let visible = enter(page, "bound").await?;
    for _ in 0..3 {
        page.send_keys(Key::Down).await?;
    }
    page.wait_for_focus(&visible[3]).await?;
    page.element("#test-tt-collapse-all").await?.click().await?;
    wait_for_rows(page, "bound", 4).await?;
    page.send_keys(Key::Tab).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&visible[0]).await?;
    Ok(())
}

/// A leaf row whose key is among the expanded keys is not marked expanded.
#[browser_test]
pub async fn leaf_rows_are_never_expanded(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    wait_for_rows(page, "bound", 7).await?;
    let report = rows(page, "bound").await?[5].clone();
    assert_that!(row_names(page, "bound").await?.get(5))
        .get_some()
        .is_equal_to("2024 Financial Report");
    expect_row(page, &report, None, 1, 3, 4).await?;
    assert_that!(report)
        .attribute("data-expanded")
        .await
        .is_none();
    Ok(())
}

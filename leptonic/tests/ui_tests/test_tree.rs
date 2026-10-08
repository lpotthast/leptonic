// Upstream: react-aria-components/test/Tree.test.tsx @ 99e6102368
// Upstream: react-aria-components/test/AriaTree.test-util.tsx @ 99e6102368
//! Behavior of the tree hooks: `treegrid` structure with levels and positions, expanding and
//! collapsing with the keyboard, the expand button and by pressing a parent row.
//!
//! Trees with a disabled item, right to left, and app-bound expansion (`/hooks/tree-cases`):
//! react-aria-components' `AriaTreeTests` ("can select items", "should not be able to interact
//! with the tree"), RTL expansion keys, a focused row hidden by collapsing its parent, and an
//! item getting children.
use assertr::prelude::*;
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::{
    pages::{ElementActions, Page, PageActions, xpath},
    polling::wait_for,
};

async fn row(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.element(xpath(format!(
        "//*[@role='row'][.//*[@role='gridcell'][contains(normalize-space(.), '{text}')]]"
    )))
    .await
}

async fn visible_rows(page: &Page<'_>) -> Result<Vec<String>, Report> {
    Ok(page
        .inner_texts("[role=row]")
        .await?
        .iter()
        .map(|text| text.trim_start_matches('›').trim().to_owned())
        .collect())
}

async fn expect_rows(page: &Page<'_>, expected: &[&str]) -> Result<(), Report> {
    wait_for("the visible rows")
        .observing(|| visible_rows(page))
        .to_be_equal_to(expected)
        .await?;
    Ok(())
}

async fn expect_focus(page: &Page<'_>, text: &str) -> Result<(), Report> {
    let expected = row(page, text).await?;
    page.wait_for_focus(&expected).await?;
    Ok(())
}

/// Tabs into the tree from the button before it: focus lands on its first row, Documents.
async fn tab_into_the_tree(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-tree-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    expect_focus(page, "Documents").await
}

pub async fn aria_structure(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/tree").await?;
    let tree = page.element("[role=treegrid]").await?;
    assert_that!(tree.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Files");
    expect_rows(page, &["Documents", "Photos", "Notes"]).await?;
    let documents = row(page, "Documents").await?;
    assert_that!(documents.attr("aria-expanded").await?)
        .get_some()
        .is_equal_to("false");
    assert_that!(documents.attr("aria-level").await?)
        .get_some()
        .is_equal_to("1");
    assert_that!(documents.attr("aria-posinset").await?)
        .get_some()
        .is_equal_to("1");
    assert_that!(documents.attr("aria-setsize").await?)
        .get_some()
        .is_equal_to("3");
    // Leaves aren't expandable.
    assert_that!(row(page, "Notes").await?.attr("aria-expanded").await?).is_none();
    Ok(())
}

pub async fn keyboard_expansion(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/tree").await?;
    tab_into_the_tree(page).await?;

    page.send_keys(Key::Right).await?;
    expect_rows(page, &["Documents", "Project", "CV", "Photos", "Notes"]).await?;
    assert_that!(row(page, "Documents").await?.attr("aria-expanded").await?)
        .get_some()
        .is_equal_to("true");
    let project = row(page, "Project").await?;
    assert_that!(project.attr("aria-level").await?)
        .get_some()
        .is_equal_to("2");
    assert_that!(project.attr("aria-setsize").await?)
        .get_some()
        .is_equal_to("2");

    page.send_keys(Key::Down).await?;
    expect_focus(page, "Project").await?;
    // ArrowLeft on a collapsed child moves to its parent ...
    page.send_keys(Key::Left).await?;
    expect_focus(page, "Documents").await?;
    // ... and on an expanded parent collapses it.
    page.send_keys(Key::Left).await?;
    expect_rows(page, &["Documents", "Photos", "Notes"]).await?;
    expect_focus(page, "Documents").await?;
    Ok(())
}

/// ArrowRight on an expanded row walks into the row's focusable children, but the expand button
/// isn't one of them (react-aria's `data-react-aria-prevent-focus` on it): focus stays on the row.
pub async fn arrow_right_on_an_expanded_row_keeps_the_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/tree").await?;
    tab_into_the_tree(page).await?;
    page.send_keys(Key::Right).await?;
    expect_rows(page, &["Documents", "Project", "CV", "Photos", "Notes"]).await?;
    expect_focus(page, "Documents").await?;
    page.send_keys(Key::Right).await?;
    let documents = row(page, "Documents").await?;
    page.focus_stays(&documents).await?;
    // ArrowLeft collapses it again, the focus staying on it.
    page.send_keys(Key::Left).await?;
    expect_rows(page, &["Documents", "Photos", "Notes"]).await?;
    expect_focus(page, "Documents").await?;
    Ok(())
}

/// Pressing a row's expand button toggles the row and focuses it; the button never takes focus
/// (upstream's `preventFocusOnPress`). Known issue: on a page where the tree never had focus, the
/// focus stays on the button (registered with the known issues in `ui_tests::all()`).
pub async fn expand_button(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/tree").await?;
    let button = row(page, "Photos").await?.element("button").await?;
    assert_that!(button.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Expand");
    button.click().await?;
    expect_rows(page, &["Documents", "Photos", "Cat", "Notes"]).await?;
    let button = row(page, "Photos").await?.element("button").await?;
    assert_that!(button.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Collapse");
    expect_focus(page, "Photos").await?;
    button.click().await?;
    expect_rows(page, &["Documents", "Photos", "Notes"]).await?;
    Ok(())
}

/// Without selection or an action, pressing a parent row toggles it.
pub async fn pressing_a_parent_toggles_it(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/tree").await?;
    tab_into_the_tree(page).await?;
    page.send_keys(Key::Enter).await?;
    expect_rows(page, &["Documents", "Project", "CV", "Photos", "Notes"]).await?;
    row(page, "Documents").await?.click().await?;
    expect_rows(page, &["Documents", "Photos", "Notes"]).await?;
    Ok(())
}

async fn tree_row(page: &Page<'_>, tree: &str, text: &str) -> Result<WebElement, Report> {
    page.element(format!("[role=treegrid][aria-label='{tree}']"))
        .await?
        .element(xpath(format!(
            ".//*[@role='row'][.//span[normalize-space(.)='{text}']]"
        )))
        .await
}

async fn tree_rows(page: &Page<'_>, tree: &str) -> Result<Vec<String>, Report> {
    page.element(format!("[role=treegrid][aria-label='{tree}']"))
        .await?
        .inner_texts("[role=row] span")
        .await
}

async fn expect_tree_rows(page: &Page<'_>, tree: &str, expected: &[&str]) -> Result<(), Report> {
    wait_for("the visible rows")
        .observing(|| tree_rows(page, tree))
        .to_be_equal_to(expected)
        .await?;
    Ok(())
}

async fn expect_tree_focus(page: &Page<'_>, tree: &str, text: &str) -> Result<(), Report> {
    let row = tree_row(page, tree, text).await?;
    page.wait_for_focus(&row).await?;
    Ok(())
}

/// "can select items" (`DisabledBehavior::Selection`): a disabled item can't be selected, but
/// focused and expanded, and its children can be selected.
pub async fn disabled_items_can_be_expanded_but_not_selected(
    page: &Page<'_>,
) -> Result<(), Report> {
    const TREE: &str = "Selection tree";
    page.goto_path("/hooks/tree-cases").await?;
    let selection = page.element("#test-tc-selection-selection").await?;
    tree_row(page, TREE, "Photos").await?.click().await?;
    selection.wait_for_inner_text("photos").await?;
    tree_row(page, TREE, "Projects").await?.click().await?;
    selection.wait_for_inner_text("projects").await?;
    tree_row(page, TREE, "School").await?.click().await?;
    selection.inner_text_stays("projects").await?;

    // Focusable with the keyboard, and expandable.
    let school = tree_row(page, TREE, "School").await?;
    page.element("#test-tc-before-selection")
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    // The pressed (disabled) row is the focused one.
    page.wait_for_focus(&school).await?;
    page.send_keys(Key::Home).await?;
    expect_tree_focus(page, TREE, "Photos").await?;
    page.send_keys(Key::Down).await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&school).await?;
    page.send_keys(Key::Right).await?;
    page.element("#test-tc-selection-expanded")
        .await?
        .wait_for_inner_text("school")
        .await?;
    expect_tree_rows(
        page,
        TREE,
        &[
            "Photos",
            "Projects",
            "School",
            "Homework-1",
            "Homework-2",
            "Homework-3",
            "Notes",
        ],
    )
    .await?;
    tree_row(page, TREE, "Homework-2").await?.click().await?;
    selection.wait_for_inner_text("homework-2").await?;
    Ok(())
}

/// "should not be able to interact with the tree" (`DisabledBehavior::All`): the disabled item
/// can't be expanded or selected, and keyboard navigation skips it.
pub async fn disabled_items_cannot_be_used(page: &Page<'_>) -> Result<(), Report> {
    const TREE: &str = "Disabled tree";
    page.goto_path("/hooks/tree-cases").await?;
    let school = tree_row(page, TREE, "School").await?;
    assert_that!(school.attr("aria-expanded").await?)
        .get_some()
        .is_equal_to("false");
    school.element("button").await?.click().await?;
    school.click().await?;
    page.element("#test-tc-all-expanded")
        .await?
        .inner_text_stays("")
        .await?;
    page.element("#test-tc-all-selection")
        .await?
        .inner_text_stays("")
        .await?;
    assert_that!(school.attr("aria-expanded").await?)
        .get_some()
        .is_equal_to("false");

    page.element("#test-tc-before-all").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    expect_tree_focus(page, TREE, "Photos").await?;
    page.send_keys(Key::Down).await?;
    expect_tree_focus(page, TREE, "Projects").await?;
    page.send_keys(Key::Down).await?;
    expect_tree_focus(page, TREE, "Notes").await?;
    Ok(())
}

/// Right to left, ArrowLeft expands and ArrowRight collapses (or moves to the parent).
pub async fn right_to_left_expansion_keys(page: &Page<'_>) -> Result<(), Report> {
    const TREE: &str = "RTL tree";
    page.goto_path("/hooks/tree-cases").await?;
    page.element("#test-tc-before-rtl").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    expect_tree_focus(page, TREE, "Photos").await?;
    page.send_keys(Key::Down).await?;
    expect_tree_focus(page, TREE, "Projects").await?;
    let expanded = page.element("#test-tc-rtl-expanded").await?;
    page.send_keys(Key::Left).await?;
    expanded.wait_for_inner_text("projects").await?;
    page.send_keys(Key::Down).await?;
    expect_tree_focus(page, TREE, "Projects-1").await?;
    page.send_keys(Key::Right).await?;
    expect_tree_focus(page, TREE, "Projects").await?;
    page.send_keys(Key::Right).await?;
    expanded.wait_for_inner_text("").await?;
    expect_tree_rows(page, TREE, &["Photos", "Projects", "School", "Notes"]).await?;
    Ok(())
}

/// A focused row hidden by collapsing its parent (here: the app clears the bound expanded keys)
/// is no longer the focused key: tabbing back in focuses a visible row (the last, coming from
/// after the tree).
pub async fn collapsing_the_parent_of_the_focused_row(page: &Page<'_>) -> Result<(), Report> {
    const TREE: &str = "Selection tree";
    page.goto_path("/hooks/tree-cases").await?;
    page.goto_path("/hooks/tree-cases").await?;
    page.element("#test-tc-before-selection")
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    expect_tree_focus(page, TREE, "Photos").await?;
    page.send_keys(Key::Down).await?;
    page.send_keys(Key::Right).await?;
    page.element("#test-tc-selection-expanded")
        .await?
        .wait_for_inner_text("projects")
        .await?;
    page.send_keys(Key::Down).await?;
    expect_tree_focus(page, TREE, "Projects-1").await?;

    page.element("#test-tc-selection-collapse-all")
        .await?
        .click()
        .await?;
    expect_tree_rows(page, TREE, &["Photos", "Projects", "School", "Notes"]).await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    expect_tree_focus(page, TREE, "Notes").await?;
    Ok(())
}

/// An item that gets children becomes expandable: it gets an expand button and `aria-expanded`.
pub async fn an_item_getting_children(page: &Page<'_>) -> Result<(), Report> {
    const TREE: &str = "Selection tree";
    page.goto_path("/hooks/tree-cases").await?;
    let notes = tree_row(page, TREE, "Notes").await?;
    assert_that!(notes.attr("aria-expanded").await?).is_none();
    assert_that!(notes.elements("button").await?).is_empty();
    page.element("#test-tc-selection-add-child")
        .await?
        .click()
        .await?;
    let notes = tree_row(page, TREE, "Notes").await?;
    notes.wait_for_count("button", 1).await?;
    notes.wait_for_attr("aria-expanded", Some("false")).await?;
    let button = notes.element("button").await?;
    assert_that!(button.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Expand");
    button.click().await?;
    expect_tree_rows(
        page,
        TREE,
        &["Photos", "Projects", "School", "Notes", "Draft"],
    )
    .await?;
    notes.wait_for_attr("aria-expanded", Some("true")).await?;
    Ok(())
}

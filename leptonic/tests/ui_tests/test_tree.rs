// Upstream: react-aria-components/test/Tree.test.tsx @ 99e6102368
// Upstream: react-aria-components/test/AriaTree.test-util.tsx @ 99e6102368
//! Behavior of the tree hooks: `treegrid` structure with levels and positions, expanding and
//! collapsing with the keyboard, the expand button and by pressing a parent row.
//!
//! Trees with a disabled item, right to left, and app-bound expansion (`/hooks/tree-cases`):
//! react-aria-components' `AriaTreeTests` ("can select items", "should not be able to interact
//! with the tree"), RTL expansion keys, a focused row hidden by collapsing its parent, and an
//! item getting children.
use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::Report;

use crate::pages::{ElementActions, Page, css, role};

const PATH: &str = "/hooks/tree";
const CASES: &str = "/hooks/tree-cases";

async fn row(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.element(role(AriaRole::Row).has(role(AriaRole::Gridcell).has(css("span").text(text))))
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
    assert_that!(|| visible_rows(page))
        .eventually_ok()
        .matches(eq(expected))
        .await;
    Ok(())
}

/// Tabs into the tree from the button before it: focus lands on its first row, Documents.
async fn tab_into_the_tree(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-tree-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&row(page, "Documents").await?).await
}

/// The tree is a labelled treegrid whose rows carry their level, position and set size, with
/// `aria-expanded` on collapsed parents but not on leaves ("should have the expected attributes on
/// the rows").
#[browser_test]
pub async fn aria_structure(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let tree = page.element("[role=treegrid]").await?;
    assert_that!(tree)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Files");
    expect_rows(page, &["Documents", "Photos", "Notes"]).await?;
    let documents = row(page, "Documents").await?;
    assert_that!(documents)
        .has_attribute("aria-expanded")
        .await
        .is_equal_to("false");
    assert_that!(documents)
        .has_attribute("aria-level")
        .await
        .is_equal_to("1");
    assert_that!(documents)
        .has_attribute("aria-posinset")
        .await
        .is_equal_to("1");
    assert_that!(documents)
        .has_attribute("aria-setsize")
        .await
        .is_equal_to("3");
    // Leaves aren't expandable.
    assert_that!(row(page, "Notes").await?)
        .attribute("aria-expanded")
        .await
        .is_none();
    Ok(())
}

/// Right expands the focused parent (its children show with level 2 and their set size); Left on a
/// collapsed child moves focus to its parent, and on an expanded parent collapses it ("should
/// support collapse key to navigate to parent").
#[browser_test]
pub async fn keyboard_expansion(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    tab_into_the_tree(page).await?;

    page.send_keys(Key::Right).await?;
    expect_rows(page, &["Documents", "Project", "CV", "Photos", "Notes"]).await?;
    assert_that!(row(page, "Documents").await?)
        .has_attribute("aria-expanded")
        .await
        .is_equal_to("true");
    let project = row(page, "Project").await?;
    assert_that!(project)
        .has_attribute("aria-level")
        .await
        .is_equal_to("2");
    assert_that!(project)
        .has_attribute("aria-setsize")
        .await
        .is_equal_to("2");

    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&row(page, "Project").await?).await?;
    // ArrowLeft on a collapsed child moves to its parent ...
    page.send_keys(Key::Left).await?;
    page.wait_for_focus(&row(page, "Documents").await?).await?;
    // ... and on an expanded parent collapses it.
    page.send_keys(Key::Left).await?;
    expect_rows(page, &["Documents", "Photos", "Notes"]).await?;
    page.wait_for_focus(&row(page, "Documents").await?).await?;
    Ok(())
}

/// ArrowRight on an expanded row keeps focus on the row, since its expand button doesn't take
/// focus, and ArrowLeft collapses the row again.
#[browser_test]
pub async fn arrow_right_on_an_expanded_row_keeps_the_focus(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    tab_into_the_tree(page).await?;
    page.send_keys(Key::Right).await?;
    expect_rows(page, &["Documents", "Project", "CV", "Photos", "Notes"]).await?;
    page.wait_for_focus(&row(page, "Documents").await?).await?;
    page.send_keys(Key::Right).await?;
    let documents = row(page, "Documents").await?;
    page.focus_stays(&documents, std::time::Duration::from_millis(100))
        .await?;
    // ArrowLeft collapses it again, the focus staying on it.
    page.send_keys(Key::Left).await?;
    expect_rows(page, &["Documents", "Photos", "Notes"]).await?;
    page.wait_for_focus(&row(page, "Documents").await?).await?;
    Ok(())
}

/// Pressing a row's "Expand" button toggles the row (relabelling the button "Collapse") and focuses
/// the row, never the button, also on a page where nothing had focus before ("should toggle the
/// row's expansion when clicking the expand button").
#[browser_test]
pub async fn expand_button(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let button = row(page, "Photos").await?.element("button").await?;
    assert_that!(button)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Expand");
    button.click().await?;
    expect_rows(page, &["Documents", "Photos", "Cat", "Notes"]).await?;
    let button = row(page, "Photos").await?.element("button").await?;
    assert_that!(button)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Collapse");
    assert_that!(button)
        .accessible_name()
        .await
        .is_equal_to("Collapse Photos");
    page.wait_for_focus(&row(page, "Photos").await?).await?;
    button.click().await?;
    expect_rows(page, &["Documents", "Photos", "Notes"]).await?;
    Ok(())
}

/// Without selection or an action, Enter on a parent row or clicking it toggles it ("should
/// expand/collapse a row when clicking/using Enter on the row itself and there arent any other
/// primary actions").
#[browser_test]
pub async fn pressing_a_parent_toggles_it(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
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
        .element(role(AriaRole::Row).has(css("span").text(text)))
        .await
}

async fn tree_rows(page: &Page<'_>, tree: &str) -> Result<Vec<String>, Report> {
    page.element(format!("[role=treegrid][aria-label='{tree}']"))
        .await?
        .inner_texts("[role=row] span")
        .await
}

async fn expect_tree_rows(page: &Page<'_>, tree: &str, expected: &[&str]) -> Result<(), Report> {
    assert_that!(|| tree_rows(page, tree))
        .eventually_ok()
        .matches(eq(expected))
        .await;
    Ok(())
}

async fn expect_tree_focus(page: &Page<'_>, tree: &str, text: &str) -> Result<(), Report> {
    let row = tree_row(page, tree, text).await?;
    page.wait_for_focus(&row).await?;
    Ok(())
}

/// With `DisabledBehavior::Selection`, a disabled item can't be selected but can be focused and
/// expanded, and its children can be selected ("can select items").
#[browser_test]
pub async fn disabled_items_can_be_expanded_but_not_selected(
    page: &Page<'_>,
) -> Result<(), Report> {
    const TREE: &str = "Selection tree";
    page.goto_path(CASES).await?;
    let selection = page.element("#test-tc-selection-selection").await?;
    tree_row(page, TREE, "Photos").await?.click().await?;
    selection.wait_for_inner_text("photos").await?;
    tree_row(page, TREE, "Projects").await?.click().await?;
    selection.wait_for_inner_text("projects").await?;
    tree_row(page, TREE, "School").await?.click().await?;
    selection
        .inner_text_stays("projects", std::time::Duration::from_millis(100))
        .await?;

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

/// With `DisabledBehavior::All`, a disabled item can't be expanded or selected and keyboard
/// navigation skips it ("should not be able to interact with the tree").
#[browser_test]
pub async fn disabled_items_cannot_be_used(page: &Page<'_>) -> Result<(), Report> {
    const TREE: &str = "Disabled tree";
    page.goto_path(CASES).await?;
    let school = tree_row(page, TREE, "School").await?;
    assert_that!(school)
        .has_attribute("aria-expanded")
        .await
        .is_equal_to("false");
    school.element("button").await?.click().await?;
    school.click().await?;
    page.element("#test-tc-all-expanded")
        .await?
        .inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    page.element("#test-tc-all-selection")
        .await?
        .inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    assert_that!(school)
        .has_attribute("aria-expanded")
        .await
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
#[browser_test]
pub async fn right_to_left_expansion_keys(page: &Page<'_>) -> Result<(), Report> {
    const TREE: &str = "RTL tree";
    page.goto_path(CASES).await?;
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

/// When the app collapses the parent of the focused row, the hidden row stops being the focused
/// one, so Shift+Tab back into the tree lands on a visible row.
#[browser_test]
pub async fn collapsing_the_parent_of_the_focused_row(page: &Page<'_>) -> Result<(), Report> {
    const TREE: &str = "Selection tree";
    page.goto_path(CASES).await?;
    page.goto_path(CASES).await?;
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
#[browser_test]
pub async fn an_item_getting_children(page: &Page<'_>) -> Result<(), Report> {
    const TREE: &str = "Selection tree";
    page.goto_path(CASES).await?;
    let notes = tree_row(page, TREE, "Notes").await?;
    assert_that!(notes)
        .attribute("aria-expanded")
        .await
        .is_none();
    assert_that!(notes.elements("button").await?).is_empty();
    page.element("#test-tc-selection-add-child")
        .await?
        .click()
        .await?;
    let notes = tree_row(page, TREE, "Notes").await?;
    notes.wait_for_count("button", 1).await?;
    notes.wait_for_attr("aria-expanded", Some("false")).await?;
    let button = notes.element("button").await?;
    assert_that!(button)
        .has_attribute("aria-label")
        .await
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

/// Typing the start of a row's name focuses that row, but only among the visible rows: the
/// children of a collapsed row are skipped ("should support type ahead").
#[browser_test]
pub async fn type_ahead_searches_the_visible_rows(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    tab_into_the_tree(page).await?;
    let documents = row(page, "Documents").await?;
    // "Cat" is under the collapsed Photos.
    page.send_keys("Cat").await?;
    page.focus_stays(&documents, std::time::Duration::from_millis(100))
        .await?;
    // Past the type-ahead's reset delay (1 s).
    std::thread::sleep(std::time::Duration::from_millis(1100));
    page.send_keys("Not").await?;
    page.wait_for_focus(&row(page, "Notes").await?).await
}

/// Home and End move to the first and last visible row ("should navigate between visible rows when
/// using Home/End").
#[browser_test]
pub async fn home_and_end_move_between_the_visible_rows(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    tab_into_the_tree(page).await?;
    page.send_keys(Key::Right).await?;
    expect_rows(page, &["Documents", "Project", "CV", "Photos", "Notes"]).await?;
    page.send_keys(Key::End).await?;
    page.wait_for_focus(&row(page, "Notes").await?).await?;
    page.send_keys(Key::Home).await?;
    page.wait_for_focus(&row(page, "Documents").await?).await?;
    Ok(())
}

/// In a selectable tree, clicking a parent row or pressing Enter on it selects it instead of
/// expanding it ("should not expand when clicking/using Enter on the row if the row is
/// selectable").
#[browser_test]
pub async fn selectable_rows_are_not_expanded_by_pressing_them(
    page: &Page<'_>,
) -> Result<(), Report> {
    const TREE: &str = "Selection tree";
    page.goto_path(CASES).await?;
    let selection = page.element("#test-tc-selection-selection").await?;
    let expanded = page.element("#test-tc-selection-expanded").await?;
    tree_row(page, TREE, "Projects").await?.click().await?;
    selection.wait_for_inner_text("projects").await?;
    page.send_keys(Key::Enter).await?;
    expanded
        .inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    // The expand button still expands it.
    tree_row(page, TREE, "Projects")
        .await?
        .element("button")
        .await?
        .click()
        .await?;
    expanded.wait_for_inner_text("projects").await?;
    Ok(())
}

/// In a tree with an action, clicking a parent row or pressing Enter on it runs the action instead
/// of expanding it ("should not expand when clicking/using Enter on the row if the row has an
/// action").
#[browser_test]
pub async fn rows_with_an_action_are_not_expanded_by_pressing_them(
    page: &Page<'_>,
) -> Result<(), Report> {
    const TREE: &str = "Action tree";
    page.goto_sections(CASES, &["action"]).await?;
    let actions = page.element("#test-tc-action-actions").await?;
    let projects = tree_row(page, TREE, "Projects").await?;
    projects.click().await?;
    actions.wait_for_inner_text("projects").await?;
    page.wait_for_focus(&projects).await?;
    page.send_keys(Key::Enter).await?;
    actions.wait_for_inner_text("projects,projects").await?;
    page.element("#test-tc-action-expanded")
        .await?
        .inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Tab moves the focus to a tree without items, and on past it ("should allow the user to tab to
/// the empty tree").
#[browser_test]
pub async fn tab_into_an_empty_tree(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(CASES, &["empty"]).await?;
    page.element("#test-tc-before-empty").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let tree = page
        .element("[role=treegrid][aria-label='Empty tree']")
        .await?;
    page.wait_for_focus(&tree).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-tc-after-empty").await?)
        .await
}

/// With `EscapeKeyBehavior::None`, Escape keeps the selection ("should prevent Esc from clearing
/// selection if escapeKeyBehavior is "none"").
#[browser_test]
pub async fn escape_keeps_the_selection(page: &Page<'_>) -> Result<(), Report> {
    const TREE: &str = "Escape tree";
    page.goto_sections(CASES, &["escape"]).await?;
    let selection = page.element("#test-tc-escape-selection").await?;
    tree_row(page, TREE, "Photos").await?.click().await?;
    selection.wait_for_inner_text("photos").await?;
    tree_row(page, TREE, "Notes").await?.click().await?;
    selection.wait_for_inner_text("notes,photos").await?;
    page.send_keys(Key::Escape).await?;
    selection
        .inner_text_stays("notes,photos", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// With `should_select_on_press_up`, a row is selected when the press ends, not when it starts
/// ("should select an item on pressing up when shouldSelectOnPressUp is true").
#[browser_test]
pub async fn selects_on_press_up(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(CASES, &["press-up"]).await?;
    let selection = page.element("#test-tc-press-up-selection").await?;
    let held = tree_row(page, "Press up tree", "Photos")
        .await?
        .press_and_hold()
        .await?;
    selection
        .inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    held.release().await?;
    selection.wait_for_inner_text("photos").await?;
    Ok(())
}

/// In a Tab-navigated tree, keys typed into a row's text input stay there: arrow keys don't move
/// between rows, letters don't search, Space doesn't select, also in a parent row ("should not
/// navigate rows when arrow keys are pressed while a text input child has focus", "should not
/// trigger typeahead when typing in a text input child", "should not trigger selection when
/// pressing Space in a text input child of a leaf row", "should allow typing space in the text
/// input child of a parent row").
#[browser_test]
pub async fn keys_in_a_text_input_stay_there(page: &Page<'_>) -> Result<(), Report> {
    page.goto_sections(CASES, &["input"]).await?;
    let selection = page.element("#test-tc-input-selection").await?;
    for name in ["Photos", "Projects"] {
        let input = page
            .element(css(format!("input[aria-label='{name} input']")))
            .await?;
        input.click().await?;
        page.wait_for_focus(&input).await?;
        for key in [Key::Down, Key::Up, Key::Left, Key::Right] {
            page.send_keys(key).await?;
        }
        page.send_keys("No te").await?;
        page.focus_stays(&input, std::time::Duration::from_millis(100))
            .await?;
        input.wait_for_prop("value", "No te").await?;
        selection
            .inner_text_stays("", std::time::Duration::from_millis(100))
            .await?;
    }
    page.element("#test-tc-input-expanded")
        .await?
        .inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

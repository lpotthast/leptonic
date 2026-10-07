// Upstream: react-aria-components/test/Tree.test.tsx @ 99e6102368
// Upstream: react-aria-components/test/AriaTree.test-util.tsx @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// Behavior of the tree hooks: `treegrid` structure with levels and positions, expanding and
/// collapsing with the keyboard, the expand button and by pressing a parent row.
pub struct TreeTests {}

#[async_trait]
impl BrowserTest<str> for TreeTests {
    fn name(&self) -> Cow<'_, str> {
        "tree_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/hooks/tree").await?;

        aria_structure(&page).await?;
        keyboard_expansion(&page).await?;
        arrow_right_on_an_expanded_row_keeps_the_focus(&page).await?;
        expand_button(&page).await?;
        pressing_a_parent_toggles_it(&page).await?;

        Ok(())
    }
}

async fn row(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.driver
        .find(By::XPath(format!(
            "//*[@role='row'][.//*[@role='gridcell'][contains(normalize-space(.), '{text}')]]"
        )))
        .await
        .map_err(Into::into)
}

async fn attr(element: &WebElement, name: &str) -> Result<Option<String>, Report> {
    Ok(element.attr(name).await?)
}

async fn visible_rows(page: &Page<'_>) -> Result<Vec<String>, Report> {
    let mut texts = Vec::new();
    for row in page.driver.find_all(By::Css("[role=row]")).await? {
        texts.push(row.text().await?.trim_start_matches('›').trim().to_owned());
    }
    Ok(texts)
}

async fn expect_rows(page: &Page<'_>, expected: &[&str]) -> Result<(), Report> {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    loop {
        let rows = visible_rows(page).await?;
        if rows == expected {
            return Ok(());
        }
        if std::time::Instant::now() > deadline {
            leptos_browser_test::bail!("expected rows {expected:?}, got {rows:?}");
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
}

async fn expect_focus(page: &Page<'_>, text: &str) -> Result<(), Report> {
    let expected = row(page, text).await?;
    page.wait_for_focus_on(&expected, text).await
}

async fn aria_structure(page: &Page<'_>) -> Result<(), Report> {
    let tree = page.css("[role=treegrid]").await?;
    assert_that!(attr(&tree, "aria-label").await?).is_equal_to(Some("Files".to_owned()));
    expect_rows(page, &["Documents", "Photos", "Notes"]).await?;
    let documents = row(page, "Documents").await?;
    assert_that!(attr(&documents, "aria-expanded").await?).is_equal_to(Some("false".to_owned()));
    assert_that!(attr(&documents, "aria-level").await?).is_equal_to(Some("1".to_owned()));
    assert_that!(attr(&documents, "aria-posinset").await?).is_equal_to(Some("1".to_owned()));
    assert_that!(attr(&documents, "aria-setsize").await?).is_equal_to(Some("3".to_owned()));
    // Leaves aren't expandable.
    assert_that!(attr(&row(page, "Notes").await?, "aria-expanded").await?).is_none();
    Ok(())
}

async fn keyboard_expansion(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-tree-before").await?;
    page.press_tab().await?;
    expect_focus(page, "Documents").await?;

    page.send_keys_to_active(Key::Right).await?;
    expect_rows(page, &["Documents", "Project", "CV", "Photos", "Notes"]).await?;
    assert_that!(attr(&row(page, "Documents").await?, "aria-expanded").await?)
        .is_equal_to(Some("true".to_owned()));
    let project = row(page, "Project").await?;
    assert_that!(attr(&project, "aria-level").await?).is_equal_to(Some("2".to_owned()));
    assert_that!(attr(&project, "aria-setsize").await?).is_equal_to(Some("2".to_owned()));

    page.send_keys_to_active(Key::Down).await?;
    expect_focus(page, "Project").await?;
    // ArrowLeft on a collapsed child moves to its parent ...
    page.send_keys_to_active(Key::Left).await?;
    expect_focus(page, "Documents").await?;
    // ... and on an expanded parent collapses it.
    page.send_keys_to_active(Key::Left).await?;
    expect_rows(page, &["Documents", "Photos", "Notes"]).await?;
    expect_focus(page, "Documents").await
}

/// ArrowRight on an expanded row walks into the row's focusable children, but the expand button
/// isn't one of them (react-aria's `data-react-aria-prevent-focus` on it): focus stays on the row.
async fn arrow_right_on_an_expanded_row_keeps_the_focus(page: &Page<'_>) -> Result<(), Report> {
    page.send_keys_to_active(Key::Right).await?;
    expect_rows(page, &["Documents", "Project", "CV", "Photos", "Notes"]).await?;
    expect_focus(page, "Documents").await?;
    page.send_keys_to_active(Key::Right).await?;
    // Let a wrong focus move happen, then check that it didn't.
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    expect_focus(page, "Documents").await?;
    assert_that!(page.describe_active_element().await?).does_not_contain("<button>");
    // Back to the state the next steps start from.
    page.send_keys_to_active(Key::Left).await?;
    expect_rows(page, &["Documents", "Photos", "Notes"]).await?;
    expect_focus(page, "Documents").await
}

async fn expand_button(page: &Page<'_>) -> Result<(), Report> {
    let button = row(page, "Photos").await?.find(By::Css("button")).await?;
    assert_that!(attr(&button, "aria-label").await?).is_equal_to(Some("Expand".to_owned()));
    button.click().await?;
    expect_rows(page, &["Documents", "Photos", "Cat", "Notes"]).await?;
    let button = row(page, "Photos").await?.find(By::Css("button")).await?;
    assert_that!(attr(&button, "aria-label").await?).is_equal_to(Some("Collapse".to_owned()));
    expect_focus(page, "Photos").await?;
    button.click().await?;
    expect_rows(page, &["Documents", "Photos", "Notes"]).await
}

/// Without selection or an action, pressing a parent row toggles it.
async fn pressing_a_parent_toggles_it(page: &Page<'_>) -> Result<(), Report> {
    page.send_keys_to_active(Key::Up).await?;
    expect_focus(page, "Documents").await?;
    page.send_keys_to_active(Key::Enter).await?;
    expect_rows(page, &["Documents", "Project", "CV", "Photos", "Notes"]).await?;
    row(page, "Documents").await?.click().await?;
    expect_rows(page, &["Documents", "Photos", "Notes"]).await
}

/// Trees with a disabled item, right to left, and app-bound expansion (`/hooks/tree-cases`):
/// react-aria-components' `AriaTreeTests` ("can select items", "should not be able to interact
/// with the tree"), RTL expansion keys, a focused row hidden by collapsing its parent, and an
/// item getting children.
pub struct TreeCasesTests {}

#[async_trait]
impl BrowserTest<str> for TreeCasesTests {
    fn name(&self) -> Cow<'_, str> {
        "tree_cases_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/hooks/tree-cases").await?;

        disabled_items_can_be_expanded_but_not_selected(&page).await?;
        disabled_items_cannot_be_used(&page).await?;
        right_to_left_expansion_keys(&page).await?;
        collapsing_the_parent_of_the_focused_row(&page).await?;
        an_item_getting_children(&page).await?;

        page.expect_no_page_errors().await
    }
}

async fn tree_row(page: &Page<'_>, tree: &str, text: &str) -> Result<WebElement, Report> {
    Ok(page
        .css(&format!("[role=treegrid][aria-label='{tree}']"))
        .await?
        .find(By::XPath(format!(
            ".//*[@role='row'][.//span[normalize-space(.)='{text}']]"
        )))
        .await?)
}

async fn tree_rows(page: &Page<'_>, tree: &str) -> Result<Vec<String>, Report> {
    let mut texts = Vec::new();
    for row in page
        .css(&format!("[role=treegrid][aria-label='{tree}']"))
        .await?
        .find_all(By::Css("[role=row] span"))
        .await?
    {
        texts.push(row.text().await?.trim().to_owned());
    }
    Ok(texts)
}

async fn expect_tree_rows(page: &Page<'_>, tree: &str, expected: &[&str]) -> Result<(), Report> {
    let expected: Vec<String> = expected.iter().map(|s| (*s).to_owned()).collect();
    page.wait_for_value("the visible rows", expected, || tree_rows(page, tree))
        .await
}

async fn expect_tree_focus(page: &Page<'_>, tree: &str, text: &str) -> Result<(), Report> {
    let row = tree_row(page, tree, text).await?;
    page.wait_for_focus_on(&row, text).await
}

/// "can select items" (`DisabledBehavior::Selection`): a disabled item can't be selected, but
/// focused and expanded, and its children can be selected.
async fn disabled_items_can_be_expanded_but_not_selected(page: &Page<'_>) -> Result<(), Report> {
    const TREE: &str = "Selection tree";
    tree_row(page, TREE, "Photos").await?.click().await?;
    page.wait_for_text("test-tc-selection-selection", "photos")
        .await?;
    tree_row(page, TREE, "Projects").await?.click().await?;
    page.wait_for_text("test-tc-selection-selection", "projects")
        .await?;
    tree_row(page, TREE, "School").await?.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    page.wait_for_text("test-tc-selection-selection", "projects")
        .await?;

    // Focusable with the keyboard, and expandable.
    page.click_element_with_id("test-tc-before-selection")
        .await?;
    page.press_tab().await?;
    page.wait_for_focus("row", None).await?;
    page.send_keys_to_active(Key::Home).await?;
    expect_tree_focus(page, TREE, "Photos").await?;
    page.send_keys_to_active(Key::Down).await?;
    page.send_keys_to_active(Key::Down).await?;
    expect_tree_focus(page, TREE, "School").await?;
    page.send_keys_to_active(Key::Right).await?;
    page.wait_for_text("test-tc-selection-expanded", "school")
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
    page.wait_for_text("test-tc-selection-selection", "homework-2")
        .await
}

/// "should not be able to interact with the tree" (`DisabledBehavior::All`): the disabled item
/// can't be expanded or selected, and keyboard navigation skips it.
async fn disabled_items_cannot_be_used(page: &Page<'_>) -> Result<(), Report> {
    const TREE: &str = "Disabled tree";
    let school = tree_row(page, TREE, "School").await?;
    assert_that!(school.attr("aria-expanded").await?).is_equal_to(Some("false".to_owned()));
    school.find(By::Css("button")).await?.click().await?;
    school.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    assert_that!(page.read_text_of("test-tc-all-expanded").await?).is_equal_to(String::new());
    assert_that!(page.read_text_of("test-tc-all-selection").await?).is_equal_to(String::new());
    assert_that!(school.attr("aria-expanded").await?).is_equal_to(Some("false".to_owned()));

    page.click_element_with_id("test-tc-before-all").await?;
    page.press_tab().await?;
    expect_tree_focus(page, TREE, "Photos").await?;
    page.send_keys_to_active(Key::Down).await?;
    expect_tree_focus(page, TREE, "Projects").await?;
    page.send_keys_to_active(Key::Down).await?;
    expect_tree_focus(page, TREE, "Notes").await
}

/// Right to left, ArrowLeft expands and ArrowRight collapses (or moves to the parent).
async fn right_to_left_expansion_keys(page: &Page<'_>) -> Result<(), Report> {
    const TREE: &str = "RTL tree";
    page.click_element_with_id("test-tc-before-rtl").await?;
    page.press_tab().await?;
    expect_tree_focus(page, TREE, "Photos").await?;
    page.send_keys_to_active(Key::Down).await?;
    expect_tree_focus(page, TREE, "Projects").await?;
    page.send_keys_to_active(Key::Left).await?;
    page.wait_for_text("test-tc-rtl-expanded", "projects")
        .await?;
    page.send_keys_to_active(Key::Down).await?;
    expect_tree_focus(page, TREE, "Projects-1").await?;
    page.send_keys_to_active(Key::Right).await?;
    expect_tree_focus(page, TREE, "Projects").await?;
    page.send_keys_to_active(Key::Right).await?;
    page.wait_for_text("test-tc-rtl-expanded", "").await?;
    expect_tree_rows(page, TREE, &["Photos", "Projects", "School", "Notes"]).await
}

/// A focused row hidden by collapsing its parent (here: the app clears the bound expanded keys)
/// is no longer the focused key: tabbing back in focuses a visible row (the last, coming from
/// after the tree).
async fn collapsing_the_parent_of_the_focused_row(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/tree-cases").await?;
    const TREE: &str = "Selection tree";
    page.click_element_with_id("test-tc-before-selection")
        .await?;
    page.press_tab().await?;
    expect_tree_focus(page, TREE, "Photos").await?;
    page.send_keys_to_active(Key::Down).await?;
    page.send_keys_to_active(Key::Right).await?;
    page.wait_for_text("test-tc-selection-expanded", "projects")
        .await?;
    page.send_keys_to_active(Key::Down).await?;
    expect_tree_focus(page, TREE, "Projects-1").await?;

    page.click_element_with_id("test-tc-selection-collapse-all")
        .await?;
    expect_tree_rows(page, TREE, &["Photos", "Projects", "School", "Notes"]).await?;
    page.press_shift_tab().await?;
    expect_tree_focus(page, TREE, "Notes").await
}

/// An item that gets children becomes expandable: `aria-expanded` and an expand button.
async fn an_item_getting_children(page: &Page<'_>) -> Result<(), Report> {
    const TREE: &str = "Selection tree";
    let notes = tree_row(page, TREE, "Notes").await?;
    assert_that!(notes.attr("aria-expanded").await?).is_none();
    assert_that!(notes.find_all(By::Css("button")).await?.len()).is_equal_to(0);
    page.click_element_with_id("test-tc-selection-add-child")
        .await?;
    let notes = tree_row(page, TREE, "Notes").await?;
    page.wait_for_attr(&notes, "aria-expanded", Some("false"))
        .await?;
    let button = notes.find(By::Css("button")).await?;
    assert_that!(button.attr("aria-label").await?).is_equal_to(Some("Expand".to_owned()));
    button.click().await?;
    expect_tree_rows(
        page,
        TREE,
        &["Photos", "Projects", "School", "Notes", "Draft"],
    )
    .await
}

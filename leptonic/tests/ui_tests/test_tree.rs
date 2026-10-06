// Upstream: react-aria-components/test/Tree.test.tsx @ 99e6102368
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

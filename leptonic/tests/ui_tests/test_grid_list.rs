// Upstream: react-aria-components/test/GridList.test.js @ 99e6102368
use std::{borrow::Cow, time::Duration};

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, TypingData, WebDriver, WebElement},
};
use leptos_browser_test::bail;
use rootcause::Report;

use crate::pages::{BaseActions, Page};

const FOLDERS: [&str; 5] = ["Inbox", "Drafts", "Spam", "Sent", "Trash"];

/// Behavior of the `GridList` atoms, asserted on the DOM/ARIA level so that these tests keep
/// passing while the collection hooks underneath are rewritten. Elements are found by role and
/// text, as users perceive them. Spec: react-aria-components `GridList.test.js`.
pub struct GridListTests {}

#[async_trait]
impl BrowserTest<str> for GridListTests {
    fn name(&self) -> Cow<'_, str> {
        "grid_list_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/grid-list").await?;

        aria_structure(&page).await?;
        tab_into_the_list_focuses_the_first_row(&page).await?;
        keyboard_navigation_skips_disabled_rows(&page).await?;
        tab_out_and_back_restores_the_focused_row(&page).await?;
        select_all_and_clear(&page).await?;
        space_toggles_selection(&page).await?;
        click_selection(&page).await?;
        disabled_row_is_marked(&page).await?;
        select_all_skips_disabled_row(&page).await?;
        shift_arrow_extends_selection(&page).await?;
        selection_announcements(&page).await?;

        Ok(())
    }
}

async fn row(page: &Page<'_>, folder: &str) -> Result<WebElement, Report> {
    page.by_role_and_text("row", folder).await
}

async fn attr(element: &WebElement, name: &str) -> Result<Option<String>, Report> {
    Ok(element.attr(name).await?)
}

async fn expect_focus(page: &Page<'_>, folder: &str) -> Result<(), Report> {
    page.wait_for_focus("row", Some(folder)).await
}

async fn expect_selection(page: &Page<'_>, expected: &str) -> Result<(), Report> {
    page.wait_for_text("test-gl-selection", expected).await
}

async fn aria_structure(page: &Page<'_>) -> Result<(), Report> {
    let grid = page.css("[role=grid]").await?;
    assert_that!(attr(&grid, "aria-label").await?).is_equal_to(Some("Folders".to_owned()));
    assert_that!(attr(&grid, "aria-multiselectable").await?).is_equal_to(Some("true".to_owned()));

    let rows = grid.find_all(By::Css("[role=row]")).await?;
    assert_that!(rows.len()).is_equal_to(FOLDERS.len());
    for folder in FOLDERS {
        let row = row(page, folder).await?;
        // Rows that can't be selected (the disabled one) have no `aria-selected` at all.
        let expected = (folder != "Spam").then(|| "false".to_owned());
        assert_that!(attr(&row, "aria-selected").await?)
            .with_detail_message(format!("row {folder:?}"))
            .is_equal_to(expected);
        let cells = row.find_all(By::Css("[role=gridcell]")).await?;
        assert_that!(cells.len())
            .with_detail_message(format!("gridcells in row {folder:?}"))
            .is_equal_to(1);
        if folder != "Spam" {
            assert_that!(attr(&row, "aria-disabled").await?)
                .with_detail_message(format!("row {folder:?}"))
                .is_none();
        }
    }
    Ok(())
}

/// The grid list is a single tab stop. Without a previously focused row, Tab lands on the first.
async fn tab_into_the_list_focuses_the_first_row(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-gl-before").await?;
    page.press_tab().await?;
    expect_focus(page, "Inbox").await
}

async fn keyboard_navigation_skips_disabled_rows(page: &Page<'_>) -> Result<(), Report> {
    for (key, expected) in [
        (Key::Down, "Drafts"),
        (Key::Down, "Sent"),
        (Key::Up, "Drafts"),
        (Key::End, "Trash"),
        // No wrapping by default.
        (Key::Down, "Trash"),
        (Key::Home, "Inbox"),
        (Key::Up, "Inbox"),
        (Key::Down, "Drafts"),
    ] {
        page.send_keys_to_active(key.clone()).await?;
        expect_focus(page, expected)
            .await
            .map_err(|e| e.context(format!("after pressing {key:?}")).into_dynamic())?;
    }
    // Moving focus does not select (selection behavior "toggle").
    stays!(
        "the text of #test-gl-selection",
        String::new(),
        page.read_text_of("test-gl-selection").await?
    );
    Ok(())
}

/// Focus leaves the list with Tab and returns to the last focused row with Shift+Tab.
async fn tab_out_and_back_restores_the_focused_row(page: &Page<'_>) -> Result<(), Report> {
    expect_focus(page, "Drafts").await?;
    page.press_tab().await?;
    assert_that!(page.active_element_id().await?).is_equal_to(Some("test-gl-after".to_owned()));
    page.press_shift_tab().await?;
    expect_focus(page, "Drafts").await.map_err(|e| {
        e.context("after Shift+Tab back into the list")
            .into_dynamic()
    })
}

/// Expect `key` to move focus to `folder` and the selection to become `selection`.
async fn press(
    page: &Page<'_>,
    key: impl Into<TypingData> + Send,
    focus: &str,
    selection: &str,
) -> Result<(), Report> {
    let key: TypingData = key.into();
    let description = format!("after pressing {key:?}");
    let context = || description.clone();
    page.send_keys_to_active(key).await?;
    expect_focus(page, focus)
        .await
        .map_err(|e| e.context(context()).into_dynamic())?;
    expect_selection(page, selection)
        .await
        .map_err(|e| e.context(context()).into_dynamic())
}

/// Returns an error instead of panicking, so that it can be used in known-issue checks.
async fn expect_attr(
    page: &Page<'_>,
    folder: &str,
    name: &str,
    expected: Option<&str>,
) -> Result<(), Report> {
    let actual = attr(&row(page, folder).await?, name).await?;
    if actual.as_deref() != expected {
        bail!("expected {name}={expected:?} on row {folder:?}, got {actual:?}");
    }
    Ok(())
}

async fn expect_selected(page: &Page<'_>, folder: &str, selected: bool) -> Result<(), Report> {
    expect_attr(page, folder, "aria-selected", Some(&selected.to_string())).await
}

/// Ctrl+A selects all rows; Escape clears the selection (default escape key behavior).
async fn select_all_and_clear(page: &Page<'_>) -> Result<(), Report> {
    expect_focus(page, "Drafts").await?;
    page.send_keys_to_active(Key::Control + "a").await?;
    expect_selection(page, "all")
        .await
        .map_err(|e| e.context("after pressing Ctrl+A").into_dynamic())?;
    for folder in ["Inbox", "Drafts", "Sent", "Trash"] {
        expect_selected(page, folder, true).await?;
    }
    press(page, Key::Escape, "Drafts", "").await?;
    for folder in ["Inbox", "Drafts", "Sent", "Trash"] {
        expect_selected(page, folder, false).await?;
    }
    Ok(())
}

/// Space toggles the focused row (multiple selection, toggle behavior).
async fn space_toggles_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/grid-list").await?;
    page.click_element_with_id("test-gl-before").await?;
    page.press_tab().await?;
    expect_focus(page, "Inbox").await?;
    press(page, Key::Space, "Inbox", "Inbox").await?;
    expect_selected(page, "Inbox", true).await?;
    press(page, Key::Down, "Drafts", "Inbox").await?;
    press(page, Key::Space, "Drafts", "Drafts,Inbox").await?;
    press(page, Key::Space, "Drafts", "Inbox").await?;
    expect_selected(page, "Drafts", false).await
}

/// Clicking toggles rows (multiple selection, toggle behavior) and focuses them. Clicking a
/// disabled row does nothing.
async fn click_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/grid-list").await?;
    row(page, "Inbox").await?.click().await?;
    expect_selection(page, "Inbox").await?;
    expect_focus(page, "Inbox").await?;
    expect_selected(page, "Inbox", true).await?;
    row(page, "Drafts").await?.click().await?;
    expect_selection(page, "Drafts,Inbox").await?;
    row(page, "Inbox").await?.click().await?;
    expect_selection(page, "Drafts").await?;

    row(page, "Spam").await?.click().await?;
    tokio::time::sleep(Duration::from_millis(200)).await;
    let selection = page.read_text_of("test-gl-selection").await?;
    if selection != "Drafts" {
        bail!("clicking the disabled row changed the selection to {selection:?}");
    }
    Ok(())
}

/// A row disabled through the list's `disabled_keys` is marked `aria-disabled`.
async fn disabled_row_is_marked(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/grid-list").await?;
    expect_attr(page, "Spam", "aria-disabled", Some("true")).await
}

/// Ctrl+A selects all rows, but a disabled row is not reported as selected: it has no
/// `aria-selected` (react-aria `useGridListItem` sets it only if `canSelectItem`).
async fn select_all_skips_disabled_row(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/grid-list").await?;
    page.click_element_with_id("test-gl-before").await?;
    page.press_tab().await?;
    expect_focus(page, "Inbox").await?;
    page.send_keys_to_active(Key::Control + "a").await?;
    expect_selection(page, "all").await?;
    expect_attr(page, "Spam", "aria-selected", None).await
}

/// Shift+Arrow extends the selection, skipping disabled rows. Without a previous selection,
/// the anchor is the newly focused row (react-aria `SelectionManager.extendSelection`).
async fn shift_arrow_extends_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/grid-list").await?;
    page.click_element_with_id("test-gl-before").await?;
    page.press_tab().await?;
    expect_focus(page, "Inbox").await?;
    press(page, Key::Shift + Key::Down, "Drafts", "Drafts").await?;
    press(page, Key::Shift + Key::Down, "Sent", "Drafts,Sent").await?;
    press(page, Key::Shift + Key::Up, "Drafts", "Drafts").await
}

/// The newest polite announcement of the live announcer.
async fn last_announcement(page: &Page<'_>) -> Result<String, Report> {
    let entries = page
        .driver
        .find_all(By::Css("[data-live-announcer] [aria-live=polite] div"))
        .await?;
    match entries.last() {
        Some(entry) => Ok(entry.prop("textContent").await?.unwrap_or_default()),
        None => Ok(String::new()),
    }
}

/// Selection changes are announced ("should allow multiple items to be selected in multiple
/// selection" and "should support select all and clear all via keyboard" in `ListView.test.js`).
async fn selection_announcements(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path("/atoms/grid-list").await?;
    row(page, "Inbox").await?.click().await?;
    wait_for!(
        "the announcement",
        "Inbox selected.",
        last_announcement(page).await?
    );
    row(page, "Drafts").await?.click().await?;
    wait_for!(
        "the announcement",
        "Drafts selected. 2 items selected.",
        last_announcement(page).await?
    );
    row(page, "Drafts").await?.click().await?;
    wait_for!(
        "the announcement",
        "Drafts not selected. 1 item selected.",
        last_announcement(page).await?
    );
    page.send_keys_to_active(Key::Control + "a").await?;
    wait_for!(
        "the announcement",
        "All items selected.",
        last_announcement(page).await?
    );
    page.send_keys_to_active(Key::Escape).await?;
    wait_for!(
        "the announcement",
        "No items selected.",
        last_announcement(page).await?
    );
    Ok(())
}

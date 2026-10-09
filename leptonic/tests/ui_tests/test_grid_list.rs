// Upstream: react-aria-components/test/GridList.test.js @ 99e6102368
//! Behavior of the `GridList` atoms, asserted on the DOM/ARIA level so that these tests keep
//! passing while the collection hooks underneath are rewritten. Elements are found by role and
//! text, as users perceive them. Spec: react-aria-components `GridList.test.js`.
use assertr::{matchers::eq, prelude::*};
use browser_test::thirtyfour::prelude::*;
use rootcause::{Report, prelude::ResultExt};

use crate::pages::{ElementActions, Page, PageActions, role};

const PATH: &str = "/atoms/grid-list";

const FOLDERS: [&str; 5] = ["Inbox", "Drafts", "Spam", "Sent", "Trash"];

async fn row(page: &Page<'_>, folder: &str) -> Result<WebElement, Report> {
    page.element(role("row").text(folder)).await
}

async fn expect_focus(page: &Page<'_>, folder: &str) -> Result<(), Report> {
    page.wait_for_focus(&row(page, folder).await?).await?;
    Ok(())
}

async fn expect_selection(page: &Page<'_>, expected: &str) -> Result<(), Report> {
    page.element("#test-gl-selection")
        .await?
        .wait_for_inner_text(expected)
        .await?;
    Ok(())
}

pub async fn aria_structure(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let grid = page.element("[role=grid]").await?;
    assert_that!(grid.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Folders");
    assert_that!(grid.attr("aria-multiselectable").await?)
        .get_some()
        .is_equal_to("true");

    let rows = grid.elements("[role=row]").await?;
    assert_that!(rows).has_length(FOLDERS.len());
    for folder in FOLDERS {
        let row = row(page, folder).await?;
        // Rows that can't be selected (the disabled one) have no `aria-selected` at all.
        let expected = (folder != "Spam").then_some("false");
        assert_that!(row.attr("aria-selected").await?.as_deref())
            .with_detail_message(format!("row {folder:?}"))
            .is_equal_to(expected);
        let cells = row.elements("[role=gridcell]").await?;
        assert_that!(cells)
            .with_detail_message(format!("gridcells in row {folder:?}"))
            .has_length(1);
        if folder != "Spam" {
            assert_that!(row.attr("aria-disabled").await?)
                .with_detail_message(format!("row {folder:?}"))
                .is_none();
        }
    }
    Ok(())
}

/// Tab from the button before the list into it: focus lands on the first row, as nothing in the
/// list was focused before.
async fn tab_into_list(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-gl-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    expect_focus(page, "Inbox").await?;
    Ok(())
}

/// Tab into the list, then Down onto "Drafts".
async fn focus_drafts(page: &Page<'_>) -> Result<(), Report> {
    tab_into_list(page).await?;
    page.send_keys(Key::Down).await?;
    expect_focus(page, "Drafts").await?;
    Ok(())
}

/// The grid list is a single tab stop. Without a previously focused row, Tab lands on the first.
pub async fn tab_into_the_list_focuses_the_first_row(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    tab_into_list(page).await?;
    Ok(())
}

pub async fn keyboard_navigation_skips_disabled_rows(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    tab_into_list(page).await?;
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
        page.send_keys(key.clone()).await?;
        expect_focus(page, expected)
            .await
            .context_with(|| format!("after pressing {key:?}"))?;
    }
    // Moving focus does not select (selection behavior "toggle").
    page.element("#test-gl-selection")
        .await?
        .inner_text_stays("")
        .await?;
    Ok(())
}

/// Focus leaves the list with Tab and returns to the last focused row with Shift+Tab.
pub async fn tab_out_and_back_restores_the_focused_row(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_drafts(page).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-gl-after").await?)
        .await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    expect_focus(page, "Drafts")
        .await
        .context("after Shift+Tab back into the list")?;
    Ok(())
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
    page.send_keys(key).await?;
    expect_focus(page, focus)
        .await
        .context_with(|| description.clone())?;
    expect_selection(page, selection)
        .await
        .context_with(|| description.clone())?;
    Ok(())
}

/// Wait until the row `folder` is (or isn't) selected.
async fn expect_selected(page: &Page<'_>, folder: &str, selected: bool) -> Result<(), Report> {
    row(page, folder)
        .await?
        .wait_for_attr("aria-selected", Some(&selected.to_string()))
        .await?;
    Ok(())
}

/// Ctrl+A selects all rows; Escape clears the selection (default escape key behavior).
pub async fn select_all_and_clear(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_drafts(page).await?;
    page.send_keys(Key::Control + "a").await?;
    expect_selection(page, "all")
        .await
        .context("after pressing Ctrl+A")?;
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
pub async fn space_toggles_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    tab_into_list(page).await?;
    press(page, Key::Space, "Inbox", "Inbox").await?;
    expect_selected(page, "Inbox", true).await?;
    press(page, Key::Down, "Drafts", "Inbox").await?;
    press(page, Key::Space, "Drafts", "Drafts,Inbox").await?;
    press(page, Key::Space, "Drafts", "Inbox").await?;
    expect_selected(page, "Drafts", false).await?;
    Ok(())
}

/// Clicking toggles rows (multiple selection, toggle behavior) and focuses them. Clicking a
/// disabled row does nothing.
pub async fn click_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    row(page, "Inbox").await?.click().await?;
    expect_selection(page, "Inbox").await?;
    expect_focus(page, "Inbox").await?;
    expect_selected(page, "Inbox", true).await?;
    row(page, "Drafts").await?.click().await?;
    expect_selection(page, "Drafts,Inbox").await?;
    row(page, "Inbox").await?.click().await?;
    expect_selection(page, "Drafts").await?;

    row(page, "Spam").await?.click().await?;
    page.element("#test-gl-selection")
        .await?
        .inner_text_stays("Drafts")
        .await?;
    Ok(())
}

/// A row disabled through the list's `disabled_keys` is marked `aria-disabled`.
pub async fn disabled_row_is_marked(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    assert_that!(row(page, "Spam").await?.attr("aria-disabled").await?)
        .get_some()
        .is_equal_to("true");
    Ok(())
}

/// Ctrl+A selects all rows, but a disabled row is not reported as selected: it has no
/// `aria-selected` (react-aria `useGridListItem` sets it only if `canSelectItem`).
pub async fn select_all_skips_disabled_row(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    tab_into_list(page).await?;
    page.send_keys(Key::Control + "a").await?;
    expect_selection(page, "all").await?;
    row(page, "Spam")
        .await?
        .attr_stays("aria-selected", None)
        .await?;
    Ok(())
}

/// Shift+Arrow extends the selection, skipping disabled rows. Without a previous selection,
/// the anchor is the newly focused row (react-aria `SelectionManager.extendSelection`).
pub async fn shift_arrow_extends_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    tab_into_list(page).await?;
    press(page, Key::Shift + Key::Down, "Drafts", "Drafts").await?;
    press(page, Key::Shift + Key::Down, "Sent", "Drafts,Sent").await?;
    press(page, Key::Shift + Key::Up, "Drafts", "Drafts").await?;
    Ok(())
}

/// The newest polite announcement of the live announcer.
async fn last_announcement(page: &Page<'_>) -> Result<String, Report> {
    let entries = page
        .elements("[data-live-announcer] [aria-live=polite] div")
        .await?;
    match entries.last() {
        Some(entry) => Ok(entry.prop("textContent").await?.unwrap_or_default()),
        None => Ok(String::new()),
    }
}

/// Selection changes are announced ("should allow multiple items to be selected in multiple
/// selection" and "should support select all and clear all via keyboard" in `ListView.test.js`).
pub async fn selection_announcements(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    row(page, "Inbox").await?.click().await?;
    assert_that!(|| last_announcement(page))
        .eventually_ok()
        .matches(eq("Inbox selected."))
        .await;
    row(page, "Drafts").await?.click().await?;
    assert_that!(|| last_announcement(page))
        .eventually_ok()
        .matches(eq("Drafts selected. 2 items selected."))
        .await;
    row(page, "Drafts").await?.click().await?;
    assert_that!(|| last_announcement(page))
        .eventually_ok()
        .matches(eq("Drafts not selected. 1 item selected."))
        .await;
    page.send_keys(Key::Control + "a").await?;
    assert_that!(|| last_announcement(page))
        .eventually_ok()
        .matches(eq("All items selected."))
        .await;
    page.send_keys(Key::Escape).await?;
    assert_that!(|| last_announcement(page))
        .eventually_ok()
        .matches(eq("No items selected."))
        .await;
    Ok(())
}

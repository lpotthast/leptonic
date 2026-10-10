// Upstream: react-aria-components/test/GridList.test.js @ 99e6102368
//! Behavior of the `GridList` atoms, asserted on the DOM/ARIA level so that these tests keep
//! passing while the collection hooks underneath are rewritten. Elements are found by role and
//! text, as users perceive them. Spec: react-aria-components `GridList.test.js`.
use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::{Report, prelude::ResultExt};

use crate::pages::{ElementActions, Page, role};

const PATH: &str = "/atoms/grid-list";

const FOLDERS: [&str; 5] = ["Inbox", "Drafts", "Spam", "Sent", "Trash"];

async fn row(page: &Page<'_>, folder: &str) -> Result<WebElement, Report> {
    page.element(role(AriaRole::Row).text(folder)).await
}

async fn expect_selection(page: &Page<'_>, expected: &str) -> Result<(), Report> {
    page.element("#test-gl-selection")
        .await?
        .wait_for_inner_text(expected)
        .await?;
    Ok(())
}

/// The grid list is a labelled, multiselectable grid whose rows have one gridcell each and
/// `aria-selected` (none on the disabled row, which can't be selected), and enabled rows have no
/// `aria-disabled`.
#[browser_test]
pub async fn aria_structure(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let grid = page.element("[role=grid]").await?;
    assert_that!(grid)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Folders");
    assert_that!(grid)
        .has_attribute("aria-multiselectable")
        .await
        .is_equal_to("true");

    let rows = grid.elements("[role=row]").await?;
    assert_that!(rows).has_length(FOLDERS.len());
    for folder in FOLDERS {
        let row = row(page, folder).await?;
        // Rows that can't be selected (the disabled one) have no `aria-selected` at all.
        let expected = (folder != "Spam").then_some("false");
        assert_that!(row)
            .attribute("aria-selected")
            .await
            .derive_owned(|value| value.as_deref())
            .with_detail_message(format!("row {folder:?}"))
            .is_equal_to(expected);
        let cells = row.elements("[role=gridcell]").await?;
        assert_that!(cells)
            .with_detail_message(format!("gridcells in row {folder:?}"))
            .has_length(1);
        if folder != "Spam" {
            assert_that!(row)
                .attribute("aria-disabled")
                .await
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
    page.wait_for_focus(&row(page, "Inbox").await?).await?;
    Ok(())
}

/// Tab into the list, then Down onto "Drafts".
async fn focus_drafts(page: &Page<'_>) -> Result<(), Report> {
    tab_into_list(page).await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&row(page, "Drafts").await?).await?;
    Ok(())
}

/// Tabbing into the list, with no row focused before, focuses the first row.
#[browser_test]
pub async fn tab_into_the_list_focuses_the_first_row(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    tab_into_list(page).await?;
    Ok(())
}

/// Up, Down, Home and End move focus between the rows, skipping the disabled one (Drafts to Sent
/// passes Spam) and not wrapping at either end; moving focus doesn't select ("should support
/// isDisabled prop on items").
#[browser_test]
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
        page.wait_for_focus(&row(page, expected).await?)
            .await
            .context_with(|| format!("after pressing {key:?}"))?;
    }
    // Moving focus does not select (selection behavior "toggle").
    page.element("#test-gl-selection")
        .await?
        .inner_text_stays("", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Focus leaves the list with Tab and returns to the last focused row with Shift+Tab.
#[browser_test]
pub async fn tab_out_and_back_restores_the_focused_row(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_drafts(page).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-gl-after").await?)
        .await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&row(page, "Drafts").await?)
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
    page.wait_for_focus(&row(page, focus).await?)
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
#[browser_test]
pub async fn select_all_and_clear(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_drafts(page).await?;
    page.send_keys(page.primary_modifier().await? + "a").await?;
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

/// Space toggles the selection of the focused row and keeps the other selected rows.
#[browser_test]
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

/// Clicking a row focuses it and toggles its selection, keeping the other selected rows, while
/// clicking the disabled row does nothing.
#[browser_test]
pub async fn click_selection(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    row(page, "Inbox").await?.click().await?;
    expect_selection(page, "Inbox").await?;
    page.wait_for_focus(&row(page, "Inbox").await?).await?;
    expect_selected(page, "Inbox", true).await?;
    row(page, "Drafts").await?.click().await?;
    expect_selection(page, "Drafts,Inbox").await?;
    row(page, "Inbox").await?.click().await?;
    expect_selection(page, "Drafts").await?;

    row(page, "Spam").await?.click().await?;
    page.element("#test-gl-selection")
        .await?
        .inner_text_stays("Drafts", std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A row disabled through the list's `disabled_keys` is marked `aria-disabled`.
#[browser_test]
pub async fn disabled_row_is_marked(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    assert_that!(row(page, "Spam").await?)
        .has_attribute("aria-disabled")
        .await
        .is_equal_to("true");
    Ok(())
}

/// Ctrl+A selects all rows, but the disabled row, which can't be selected, gets no
/// `aria-selected`.
#[browser_test]
pub async fn select_all_skips_disabled_row(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    tab_into_list(page).await?;
    page.send_keys(page.primary_modifier().await? + "a").await?;
    expect_selection(page, "all").await?;
    row(page, "Spam")
        .await?
        .attr_stays("aria-selected", None, std::time::Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Shift+Down and Shift+Up extend the selection, skipping the disabled row, and without a previous
/// selection the newly focused row is the anchor.
#[browser_test]
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

/// Selecting and deselecting a row, selecting all and clearing are announced with the selected
/// count ("should allow multiple items to be selected in multiple selection", "should support
/// select all and clear all via keyboard" of `ListView.test.js`).
#[browser_test]
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
    page.send_keys(page.primary_modifier().await? + "a").await?;
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

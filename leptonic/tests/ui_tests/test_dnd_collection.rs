// Upstream: react-aria/test/dnd/useDroppableCollection.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, dnd::DndPage};

const LOG: &str = "test-dnd-collection-log";

/// Dropping into a collection (`use_droppable_collection`, `use_drop_indicator`): keyboard drags
/// between, on and around its rows (arrow keys, Home/End, PageUp/PageDown, the initial target
/// next to the focused or selected rows), native drags (synthesized `DragEvent`s), and focus and
/// selection after a drop. Fixture: `/hooks/dnd-collection` (react-aria's `DroppableGridExample`).
pub struct DndCollectionTests {}

#[async_trait]
impl BrowserTest<str> for DndCollectionTests {
    fn name(&self) -> Cow<'_, str> {
        "dnd_collection_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = DndPage { driver, base_url };
        basic_drag_and_drop(&page).await?;
        arrow_key_navigation(&page).await?;
        home_and_end(&page).await?;
        page_up_and_page_down(&page).await?;
        page_up_and_page_down_skip_invalid_targets(&page).await?;
        page.expect_no_page_errors().await
    }
}

/// Where a keyboard drag starts in the collection (next to the focused or selected rows), and
/// native drags.
pub struct DndCollectionTargetTests {}

#[async_trait]
impl BrowserTest<str> for DndCollectionTargetTests {
    fn name(&self) -> Cow<'_, str> {
        "dnd_collection_target_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = DndPage { driver, base_url };
        after_the_last_focused_item(&page).await?;
        after_the_selected_items(&page).await?;
        before_the_selected_items(&page).await?;
        on_the_first_selected_item(&page).await?;
        on_the_last_selected_item(&page).await?;
        native_basic_drag_and_drop(&page).await?;
        native_drop_on_an_item(&page).await?;
        page.expect_no_page_errors().await
    }
}

async fn row(page: &DndPage<'_>, text: &str) -> Result<WebElement, Report> {
    page.css("[role=grid][aria-label='List']")
        .await?
        .find(By::XPath(format!(
            ".//*[@role='row'][@aria-selected][.//*[@role='gridcell'][normalize-space(.)='{text}']]"
        )))
        .await
        .map_err(Into::into)
}

/// The texts of the list's rows, in order.
async fn row_texts(page: &DndPage<'_>) -> Result<Vec<String>, Report> {
    let texts = page
        .driver
        .execute(
            "return [...document.querySelectorAll('[role=grid][aria-label=List] [role=row][aria-selected]')].map(r => r.textContent.trim());",
            vec![],
        )
        .await?;
    Ok(texts
        .json()
        .as_array()
        .map(|texts| {
            texts
                .iter()
                .map(|t| t.as_str().unwrap_or_default().to_owned())
                .collect()
        })
        .unwrap_or_default())
}

/// Waits until the focused element is the drop indicator labelled `label`.
async fn expect_focused_indicator(page: &DndPage<'_>, label: &str) -> Result<(), Report> {
    wait_for!("the focused drop indicator", Some(label.to_owned()), {
        let active = page.driver.active_element().await?;
        if active.attr("aria-roledescription").await?.as_deref() == Some("drop indicator") {
            active.attr("aria-label").await?
        } else {
            None
        }
    });
    Ok(())
}

/// Opens the fixture with `query`, focuses the draggable and starts a keyboard drag.
async fn start_drag(page: &DndPage<'_>, query: &str) -> Result<(), Report> {
    page.goto_path(&format!("/hooks/dnd-collection{query}"))
        .await?;
    focus_draggable(page).await?;
    page.send_keys_to_active(Key::Enter).await?;
    Ok(())
}

async fn focus_draggable(page: &DndPage<'_>) -> Result<(), Report> {
    page.by_role_and_text("button", "Before")
        .await?
        .click()
        .await?;
    page.press_tab().await?;
    page.wait_for_focus_on(
        &page.by_role_and_text("button", "Drag me").await?,
        "the draggable",
    )
    .await
}

/// "keyboard: should perform basic drag and drop": the drop inserts the item, which is then
/// focused and selected.
async fn basic_drag_and_drop(page: &DndPage<'_>) -> Result<(), Report> {
    start_drag(page, "").await?;
    expect_focused_indicator(page, "Drop on").await?;
    page.send_keys_to_active(Key::Down).await?;
    expect_focused_indicator(page, "Insert before One").await?;
    page.send_keys_to_active(Key::Down).await?;
    expect_focused_indicator(page, "Drop on One").await?;
    page.send_keys_to_active(Key::Down).await?;
    expect_focused_indicator(page, "Insert between One and Two").await?;
    page.send_keys_to_active(Key::Enter).await?;
    wait_for!(
        "the rows",
        ["One", "hello world", "Two", "Three"],
        row_texts(page).await?
    );
    let inserted = row(page, "hello world").await?;
    page.wait_for_focus_on(&inserted, "the inserted row")
        .await?;
    page.wait_for_attr(&inserted, "aria-selected", Some("true"))
        .await?;
    page.expect_log_settled(
        LOG,
        &[
            "enter root",
            "exit root",
            "enter 1 before",
            "exit 1 before",
            "enter 1 on",
            "exit 1 on",
            "enter 2 before",
            "insert 2 before hello world Move",
            "dragend Move",
            "exit 2 before",
        ],
    )
    .await
}

const TARGETS: [&str; 7] = [
    "Drop on",
    "Insert before One",
    "Drop on One",
    "Insert between One and Two",
    "Insert between Two and Three",
    "Drop on Three",
    "Insert after Three",
];

/// "should support arrow key navigation": down through every valid target ("on Two" isn't one),
/// wrapping, and back up.
async fn arrow_key_navigation(page: &DndPage<'_>) -> Result<(), Report> {
    start_drag(page, "").await?;
    for (i, target) in TARGETS.iter().enumerate() {
        if i > 0 {
            page.send_keys_to_active(Key::Down).await?;
        }
        expect_focused_indicator(page, target).await?;
    }
    page.send_keys_to_active(Key::Down).await?;
    expect_focused_indicator(page, TARGETS[0]).await?;
    for target in TARGETS.iter().rev() {
        page.send_keys_to_active(Key::Up).await?;
        expect_focused_indicator(page, target).await?;
    }
    page.send_keys_to_active(Key::Escape).await
}

/// "supports Home and End".
async fn home_and_end(page: &DndPage<'_>) -> Result<(), Report> {
    start_drag(page, "").await?;
    expect_focused_indicator(page, "Drop on").await?;
    page.send_keys_to_active(Key::End).await?;
    expect_focused_indicator(page, "Insert after Three").await?;
    page.send_keys_to_active(Key::Home).await?;
    expect_focused_indicator(page, "Drop on").await?;
    page.expect_log(
        LOG,
        &[
            "enter root",
            "exit root",
            "enter 3 after",
            "exit 3 after",
            "enter root",
        ],
    )
    .await?;
    page.send_keys_to_active(Key::Escape).await?;
    Ok(())
}

/// "supports PageUp and PageDown" (rows 50px, the list 150px high).
async fn page_up_and_page_down(page: &DndPage<'_>) -> Result<(), Report> {
    start_drag(page, "?items=6").await?;
    expect_focused_indicator(page, "Drop on").await?;
    for target in [
        "Insert between Item 2 and Item 3",
        "Insert between Item 4 and Item 5",
        "Insert after Item 5",
    ] {
        page.send_keys_to_active(Key::PageDown).await?;
        expect_focused_indicator(page, target).await?;
    }
    for target in [
        "Insert between Item 3 and Item 4",
        "Insert between Item 1 and Item 2",
        "Insert between Item 0 and Item 1",
        "Drop on",
    ] {
        page.send_keys_to_active(Key::PageUp).await?;
        expect_focused_indicator(page, target).await?;
    }
    page.send_keys_to_active(Key::Escape).await?;
    Ok(())
}

/// "should skip invalid targets with PageUp and PageDown" (only drops on items 0, 2, 3, 5).
async fn page_up_and_page_down_skip_invalid_targets(page: &DndPage<'_>) -> Result<(), Report> {
    start_drag(page, "?items=6&only-on&cancel=1,4").await?;
    expect_focused_indicator(page, "Drop on Item 0").await?;
    for target in ["Drop on Item 2", "Drop on Item 5"] {
        page.send_keys_to_active(Key::PageDown).await?;
        expect_focused_indicator(page, target).await?;
    }
    for target in ["Drop on Item 3", "Drop on Item 0"] {
        page.send_keys_to_active(Key::PageUp).await?;
        expect_focused_indicator(page, target).await?;
    }
    page.send_keys_to_active(Key::Escape).await?;
    Ok(())
}

/// Focuses the first row (Tab from the draggable).
async fn focus_first_row(page: &DndPage<'_>, query: &str) -> Result<(), Report> {
    page.goto_path(&format!("/hooks/dnd-collection{query}"))
        .await?;
    focus_draggable(page).await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&row(page, "One").await?, "row One")
        .await
}

/// Shift + Tab back to the draggable and starts a keyboard drag.
async fn drag_from_the_draggable(page: &DndPage<'_>) -> Result<(), Report> {
    page.press_shift_tab().await?;
    page.wait_for_focus_on(
        &page.by_role_and_text("button", "Drag me").await?,
        "the draggable",
    )
    .await?;
    page.send_keys_to_active(Key::Enter).await
}

/// Selects One, Three and Two (in that order), ending focused on Two.
async fn select_one_three_two(page: &DndPage<'_>) -> Result<(), Report> {
    page.send_keys_to_active(Key::Space).await?;
    page.wait_for_attr(&row(page, "One").await?, "aria-selected", Some("true"))
        .await?;
    page.send_keys_to_active(Key::Down).await?;
    page.send_keys_to_active(Key::Down).await?;
    page.send_keys_to_active(Key::Space).await?;
    page.wait_for_attr(&row(page, "Three").await?, "aria-selected", Some("true"))
        .await?;
    page.send_keys_to_active(Key::Up).await?;
    page.send_keys_to_active(Key::Space).await?;
    page.wait_for_attr(&row(page, "Two").await?, "aria-selected", Some("true"))
        .await
}

/// Selects Three and Two (in that order), ending focused on Two.
async fn select_three_two(page: &DndPage<'_>) -> Result<(), Report> {
    for _ in 0..3 {
        page.send_keys_to_active(Key::Down).await?;
    }
    page.send_keys_to_active(Key::Space).await?;
    page.wait_for_attr(&row(page, "Three").await?, "aria-selected", Some("true"))
        .await?;
    page.send_keys_to_active(Key::Up).await?;
    page.send_keys_to_active(Key::Space).await?;
    page.wait_for_attr(&row(page, "Two").await?, "aria-selected", Some("true"))
        .await
}

/// "should default to dropping after the last focused item if any".
async fn after_the_last_focused_item(page: &DndPage<'_>) -> Result<(), Report> {
    focus_first_row(page, "").await?;
    page.send_keys_to_active(Key::Down).await?;
    page.wait_for_focus_on(&row(page, "Two").await?, "row Two")
        .await?;
    drag_from_the_draggable(page).await?;
    expect_focused_indicator(page, "Insert between Two and Three").await?;
    page.send_keys_to_active(Key::Escape).await
}

/// "should default to dropping after the selected items if any".
async fn after_the_selected_items(page: &DndPage<'_>) -> Result<(), Report> {
    focus_first_row(page, "").await?;
    select_one_three_two(page).await?;
    drag_from_the_draggable(page).await?;
    expect_focused_indicator(page, "Insert after Three").await?;
    page.send_keys_to_active(Key::Escape).await
}

/// "should default to before the selected items if the last focused item is the first selected
/// item".
async fn before_the_selected_items(page: &DndPage<'_>) -> Result<(), Report> {
    focus_first_row(page, "").await?;
    select_three_two(page).await?;
    drag_from_the_draggable(page).await?;
    expect_focused_indicator(page, "Insert between One and Two").await?;
    page.send_keys_to_active(Key::Escape).await
}

/// "should default to on the first selected item if the last focused item is the first selected
/// item and only dropping on items is allowed".
async fn on_the_first_selected_item(page: &DndPage<'_>) -> Result<(), Report> {
    focus_first_row(page, "?only-on&cancel=").await?;
    select_three_two(page).await?;
    drag_from_the_draggable(page).await?;
    expect_focused_indicator(page, "Drop on Two").await?;
    page.send_keys_to_active(Key::Escape).await
}

/// "should default to on the last selected item when only dropping on items is allowed".
async fn on_the_last_selected_item(page: &DndPage<'_>) -> Result<(), Report> {
    focus_first_row(page, "?only-on&cancel=").await?;
    select_one_three_two(page).await?;
    drag_from_the_draggable(page).await?;
    expect_focused_indicator(page, "Drop on Three").await?;
    page.send_keys_to_active(Key::Escape).await
}

/// The cell of the row `text`.
async fn cell(page: &DndPage<'_>, text: &str) -> Result<WebElement, Report> {
    Ok(row(page, text)
        .await?
        .find(By::Css("[role=gridcell]"))
        .await?)
}

/// "native drag and drop: should perform basic drag and drop": the drop target follows the
/// pointer (before, on and after rows; "on Two" isn't valid), the drop inserts the item, which is
/// then focused and selected.
async fn native_basic_drag_and_drop(page: &DndPage<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/dnd-collection").await?;
    let draggable = page.by_role_and_text("button", "Drag me").await?;
    page.fire_drag_event(&draggable, "dragstart", &[]).await?;
    let one = cell(page, "One").await?;
    let two = cell(page, "Two").await?;
    let three = cell(page, "Three").await?;
    page.fire_drag_event_at(&one, "dragenter", &[], Some((1.0, 1.0)))
        .await?;
    page.expect_log(LOG, &["enter 1 before"]).await?;
    page.fire_drag_event_at(&one, "dragover", &[], Some((1.0, 20.0)))
        .await?;
    page.expect_log(LOG, &["enter 1 before", "exit 1 before", "enter 1 on"])
        .await?;
    assert_that!(page.transfer("dropEffect").await?).is_equal_to(serde_json::Value::from("copy"));
    page.fire_drag_event_at(&two, "dragover", &[], Some((1.0, 3.0)))
        .await?;
    // "On Two" isn't valid: its upper half is before it, its lower half after it.
    page.fire_drag_event_at(&two, "dragover", &[], Some((1.0, 30.0)))
        .await?;
    page.expect_log(
        LOG,
        &[
            "enter 1 before",
            "exit 1 before",
            "enter 1 on",
            "exit 1 on",
            "enter 2 before",
            "exit 2 before",
            "enter 2 after",
        ],
    )
    .await?;
    page.fire_drag_event_at(&three, "drop", &[], Some((2.0, 2.0)))
        .await?;
    page.fire_drag_event(&draggable, "dragend", &[]).await?;
    wait_for!(
        "the rows",
        ["One", "Two", "hello world", "Three"],
        row_texts(page).await?
    );
    let inserted = row(page, "hello world").await?;
    page.wait_for_focus_on(&inserted, "the inserted row")
        .await?;
    page.wait_for_attr(&inserted, "aria-selected", Some("true"))
        .await?;
    page.expect_log_settled(
        LOG,
        &[
            "enter 1 before",
            "exit 1 before",
            "enter 1 on",
            "exit 1 on",
            "enter 2 before",
            "exit 2 before",
            "enter 2 after",
            "insert 2 after hello world Move",
            "exit 2 after",
            "dragend Move",
        ],
    )
    .await
}

/// "supports dropping on an item": the item gets focus, not selection.
async fn native_drop_on_an_item(page: &DndPage<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/dnd-collection").await?;
    let draggable = page.by_role_and_text("button", "Drag me").await?;
    page.fire_drag_event(&draggable, "dragstart", &[]).await?;
    let one = cell(page, "One").await?;
    page.fire_drag_event_at(&one, "dragenter", &[], Some((1.0, 20.0)))
        .await?;
    page.expect_log(LOG, &["enter 1 on"]).await?;
    assert_that!(page.transfer("dropEffect").await?).is_equal_to(serde_json::Value::from("copy"));
    page.fire_drag_event_at(&one, "drop", &[], Some((2.0, 2.0)))
        .await?;
    page.fire_drag_event(&draggable, "dragend", &[]).await?;
    let one_row = row(page, "One").await?;
    page.wait_for_focus_on(&one_row, "row One").await?;
    page.expect_log_settled(
        LOG,
        &[
            "enter 1 on",
            "on 1 hello world Copy",
            "exit 1 on",
            "dragend Copy",
        ],
    )
    .await?;
    assert_that!(row_texts(page).await?).is_equal_to(vec![
        "One".to_owned(),
        "Two".to_owned(),
        "Three".to_owned(),
    ]);
    assert_that!(one_row.attr("aria-selected").await?).is_not_equal_to(Some("true".to_owned()));
    Ok(())
}

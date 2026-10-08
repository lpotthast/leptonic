// Upstream: react-aria/test/dnd/useDroppableCollection.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    pages::{ElementActions, PageActions, dnd::DndPage, role, xpath},
    polling::wait_for,
};

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
        cases!(
            basic_drag_and_drop(&page),
            arrow_key_navigation(&page),
            home_and_end(&page),
            page_up_and_page_down(&page),
            page_up_and_page_down_skip_invalid_targets(&page),
        );
        Ok(())
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
        cases!(
            after_the_last_focused_item(&page),
            after_the_selected_items(&page),
            before_the_selected_items(&page),
            on_the_first_selected_item(&page),
            on_the_last_selected_item(&page),
            native_basic_drag_and_drop(&page),
            native_drop_on_an_item(&page),
        );
        Ok(())
    }
}

/// The row `text` of the list.
async fn row(page: &DndPage<'_>, text: &str) -> Result<WebElement, Report> {
    page.element("[role=grid][aria-label='List']")
        .await?
        .element(xpath(format!(
            ".//*[@role='row'][@aria-selected][.//*[@role='gridcell'][normalize-space(.)='{text}']]"
        )))
        .await
}

/// Waits until the drop indicator labelled `label` has focus.
async fn expect_focused_indicator(page: &DndPage<'_>, label: &str) -> Result<(), Report> {
    let indicator = page
        .element(format!(
            "[aria-roledescription='drop indicator'][aria-label='{label}']"
        ))
        .await?;
    page.wait_for_focus(&indicator).await?;
    Ok(())
}

/// Opens the fixture with `query`, focuses the draggable and starts a keyboard drag.
async fn start_drag(page: &DndPage<'_>, query: &str) -> Result<(), Report> {
    page.goto_path(&format!("/hooks/dnd-collection{query}"))
        .await?;
    focus_draggable(page).await?;
    page.send_keys(Key::Enter).await?;
    Ok(())
}

/// Focuses the drag source ("Drag me") with the keyboard.
async fn focus_draggable(page: &DndPage<'_>) -> Result<(), Report> {
    page.element(role("button").text("Before"))
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(role("button").text("Drag me")).await?)
        .await?;
    Ok(())
}

/// "keyboard: should perform basic drag and drop": the drop inserts the item, which is then
/// focused and selected.
async fn basic_drag_and_drop(page: &DndPage<'_>) -> Result<(), Report> {
    start_drag(page, "").await?;
    expect_focused_indicator(page, "Drop on").await?;
    page.send_keys(Key::Down).await?;
    expect_focused_indicator(page, "Insert before One").await?;
    page.send_keys(Key::Down).await?;
    expect_focused_indicator(page, "Drop on One").await?;
    page.send_keys(Key::Down).await?;
    expect_focused_indicator(page, "Insert between One and Two").await?;
    page.send_keys(Key::Enter).await?;
    wait_for("the rows")
        .observing(|| page.inner_texts("[role=grid][aria-label=List] [role=row][aria-selected]"))
        .to_be_equal_to(["One", "hello world", "Two", "Three"])
        .await?;
    let inserted = row(page, "hello world").await?;
    page.wait_for_focus(&inserted).await?;
    inserted
        .wait_for_attr("aria-selected", Some("true"))
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
    .await?;
    Ok(())
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
            page.send_keys(Key::Down).await?;
        }
        expect_focused_indicator(page, target).await?;
    }
    page.send_keys(Key::Down).await?;
    expect_focused_indicator(page, TARGETS[0]).await?;
    for target in TARGETS.iter().rev() {
        page.send_keys(Key::Up).await?;
        expect_focused_indicator(page, target).await?;
    }
    page.send_keys(Key::Escape).await?;
    Ok(())
}

/// "supports Home and End".
async fn home_and_end(page: &DndPage<'_>) -> Result<(), Report> {
    start_drag(page, "").await?;
    expect_focused_indicator(page, "Drop on").await?;
    page.send_keys(Key::End).await?;
    expect_focused_indicator(page, "Insert after Three").await?;
    page.send_keys(Key::Home).await?;
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
    page.send_keys(Key::Escape).await?;
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
        page.send_keys(Key::PageDown).await?;
        expect_focused_indicator(page, target).await?;
    }
    for target in [
        "Insert between Item 3 and Item 4",
        "Insert between Item 1 and Item 2",
        "Insert between Item 0 and Item 1",
        "Drop on",
    ] {
        page.send_keys(Key::PageUp).await?;
        expect_focused_indicator(page, target).await?;
    }
    page.send_keys(Key::Escape).await?;
    Ok(())
}

/// "should skip invalid targets with PageUp and PageDown" (only drops on items 0, 2, 3, 5).
async fn page_up_and_page_down_skip_invalid_targets(page: &DndPage<'_>) -> Result<(), Report> {
    start_drag(page, "?items=6&only-on&cancel=1,4").await?;
    expect_focused_indicator(page, "Drop on Item 0").await?;
    for target in ["Drop on Item 2", "Drop on Item 5"] {
        page.send_keys(Key::PageDown).await?;
        expect_focused_indicator(page, target).await?;
    }
    for target in ["Drop on Item 3", "Drop on Item 0"] {
        page.send_keys(Key::PageUp).await?;
        expect_focused_indicator(page, target).await?;
    }
    page.send_keys(Key::Escape).await?;
    Ok(())
}

/// Focuses the first row (Tab from the draggable).
async fn focus_first_row(page: &DndPage<'_>, query: &str) -> Result<(), Report> {
    page.goto_path(&format!("/hooks/dnd-collection{query}"))
        .await?;
    focus_draggable(page).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&row(page, "One").await?).await?;
    Ok(())
}

/// Shift + Tab back to the draggable and starts a keyboard drag.
async fn drag_from_the_draggable(page: &DndPage<'_>) -> Result<(), Report> {
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&page.element(role("button").text("Drag me")).await?)
        .await?;
    page.send_keys(Key::Enter).await?;
    Ok(())
}

/// Selects One, Three and Two (in that order), ending focused on Two.
async fn select_one_three_two(page: &DndPage<'_>) -> Result<(), Report> {
    page.send_keys(Key::Space).await?;
    row(page, "One")
        .await?
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    page.send_keys(Key::Down).await?;
    page.send_keys(Key::Down).await?;
    page.send_keys(Key::Space).await?;
    row(page, "Three")
        .await?
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    page.send_keys(Key::Up).await?;
    page.send_keys(Key::Space).await?;
    row(page, "Two")
        .await?
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    Ok(())
}

/// Selects Three and Two (in that order), ending focused on Two.
async fn select_three_two(page: &DndPage<'_>) -> Result<(), Report> {
    for _ in 0..3 {
        page.send_keys(Key::Down).await?;
    }
    page.send_keys(Key::Space).await?;
    row(page, "Three")
        .await?
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    page.send_keys(Key::Up).await?;
    page.send_keys(Key::Space).await?;
    row(page, "Two")
        .await?
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    Ok(())
}

/// "should default to dropping after the last focused item if any".
async fn after_the_last_focused_item(page: &DndPage<'_>) -> Result<(), Report> {
    focus_first_row(page, "").await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&row(page, "Two").await?).await?;
    drag_from_the_draggable(page).await?;
    expect_focused_indicator(page, "Insert between Two and Three").await?;
    page.send_keys(Key::Escape).await?;
    Ok(())
}

/// "should default to dropping after the selected items if any".
async fn after_the_selected_items(page: &DndPage<'_>) -> Result<(), Report> {
    focus_first_row(page, "").await?;
    select_one_three_two(page).await?;
    drag_from_the_draggable(page).await?;
    expect_focused_indicator(page, "Insert after Three").await?;
    page.send_keys(Key::Escape).await?;
    Ok(())
}

/// "should default to before the selected items if the last focused item is the first selected
/// item".
async fn before_the_selected_items(page: &DndPage<'_>) -> Result<(), Report> {
    focus_first_row(page, "").await?;
    select_three_two(page).await?;
    drag_from_the_draggable(page).await?;
    expect_focused_indicator(page, "Insert between One and Two").await?;
    page.send_keys(Key::Escape).await?;
    Ok(())
}

/// "should default to on the first selected item if the last focused item is the first selected
/// item and only dropping on items is allowed".
async fn on_the_first_selected_item(page: &DndPage<'_>) -> Result<(), Report> {
    focus_first_row(page, "?only-on&cancel=").await?;
    select_three_two(page).await?;
    drag_from_the_draggable(page).await?;
    expect_focused_indicator(page, "Drop on Two").await?;
    page.send_keys(Key::Escape).await?;
    Ok(())
}

/// "should default to on the last selected item when only dropping on items is allowed".
async fn on_the_last_selected_item(page: &DndPage<'_>) -> Result<(), Report> {
    focus_first_row(page, "?only-on&cancel=").await?;
    select_one_three_two(page).await?;
    drag_from_the_draggable(page).await?;
    expect_focused_indicator(page, "Drop on Three").await?;
    page.send_keys(Key::Escape).await?;
    Ok(())
}

/// The cell of the row `text`.
async fn cell(page: &DndPage<'_>, text: &str) -> Result<WebElement, Report> {
    row(page, text).await?.element("[role=gridcell]").await
}

/// "native drag and drop: should perform basic drag and drop": the drop target follows the
/// pointer (before, on and after rows; "on Two" isn't valid), the drop inserts the item, which is
/// then focused and selected.
async fn native_basic_drag_and_drop(page: &DndPage<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/dnd-collection").await?;
    let draggable = page.element(role("button").text("Drag me")).await?;
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
    assert_that!(page.transfer::<String>("dropEffect").await?).is_equal_to("copy");
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
    wait_for("the rows")
        .observing(|| page.inner_texts("[role=grid][aria-label=List] [role=row][aria-selected]"))
        .to_be_equal_to(["One", "Two", "hello world", "Three"])
        .await?;
    let inserted = row(page, "hello world").await?;
    page.wait_for_focus(&inserted).await?;
    inserted
        .wait_for_attr("aria-selected", Some("true"))
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
    .await?;
    Ok(())
}

/// "supports dropping on an item": the item gets focus, not selection.
async fn native_drop_on_an_item(page: &DndPage<'_>) -> Result<(), Report> {
    page.goto_path("/hooks/dnd-collection").await?;
    let draggable = page.element(role("button").text("Drag me")).await?;
    page.fire_drag_event(&draggable, "dragstart", &[]).await?;
    let one = cell(page, "One").await?;
    page.fire_drag_event_at(&one, "dragenter", &[], Some((1.0, 20.0)))
        .await?;
    page.expect_log(LOG, &["enter 1 on"]).await?;
    assert_that!(page.transfer::<String>("dropEffect").await?).is_equal_to("copy");
    page.fire_drag_event_at(&one, "drop", &[], Some((2.0, 2.0)))
        .await?;
    page.fire_drag_event(&draggable, "dragend", &[]).await?;
    let one_row = row(page, "One").await?;
    page.wait_for_focus(&one_row).await?;
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
    assert_that!(
        page.inner_texts("[role=grid][aria-label=List] [role=row][aria-selected]")
            .await?
    )
    .contains_exactly(["One", "Two", "Three"]);
    assert_that!(one_row.attr("aria-selected").await?)
        .get_some()
        .is_equal_to("false");
    Ok(())
}

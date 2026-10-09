// Upstream: react-aria/test/dnd/useDroppableCollection.test.js @ 99e6102368
//! Dropping into a collection (`use_droppable_collection`, `use_drop_indicator`): keyboard drags
//! between, on and around its rows (arrow keys, Home/End, PageUp/PageDown, the initial target
//! next to the focused or selected rows), native drags (synthesized `DragEvent`s), and focus and
//! selection after a drop. Fixture: `/hooks/dnd-collection` (react-aria's `DroppableGridExample`).
//!
//! Where a keyboard drag starts in the collection (next to the focused or selected rows), native
//! drags, and screen reader drags (descriptions, labels and hidden drop positions).
//!
//! Not mirrored: "should auto scroll when near the bottom" (`use_auto_scroll` scrolls only in
//! WebKit on macOS, which doesn't scroll during native drags itself; the suite runs Chrome).
use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::Report;

use crate::{
    fixtures::dnd::DndActions,
    pages::{DragKind, ElementActions, Page, css, role},
};

const LOG: &str = "test-dnd-collection-log";
const PATH: &str = "/hooks/dnd-collection";

/// The row `text` of the list.
async fn row(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.element("[role=grid][aria-label='List']")
        .await?
        .element(css("[role=row][aria-selected]").has(role(AriaRole::Gridcell).text(text)))
        .await
}

/// Waits until the drop indicator labelled `label` has focus.
async fn expect_focused_indicator(page: &Page<'_>, label: &str) -> Result<(), Report> {
    let indicator = page
        .element(format!(
            "[aria-roledescription='drop indicator'][aria-label='{label}']"
        ))
        .await?;
    page.wait_for_focus(&indicator).await?;
    Ok(())
}

/// Opens the fixture with `query`, focuses the draggable and starts a keyboard drag.
async fn start_drag(page: &Page<'_>, query: &str) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}{query}")).await?;
    focus_draggable(page).await?;
    page.send_keys(Key::Enter).await?;
    Ok(())
}

/// Focuses the drag source ("Drag me") with the keyboard.
async fn focus_draggable(page: &Page<'_>) -> Result<(), Report> {
    page.element(role(AriaRole::Button).text("Before"))
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(role(AriaRole::Button).text("Drag me")).await?)
        .await?;
    Ok(())
}

/// In a keyboard drag, ArrowDown moves through the drop targets and Enter inserts the item between
/// One and Two, which is then focused and selected ("should perform basic drag and drop").
#[browser_test]
pub async fn basic_drag_and_drop(page: &Page<'_>) -> Result<(), Report> {
    start_drag(page, "").await?;
    expect_focused_indicator(page, "Drop on").await?;
    page.send_keys(Key::Down).await?;
    expect_focused_indicator(page, "Insert before One").await?;
    page.send_keys(Key::Down).await?;
    expect_focused_indicator(page, "Drop on One").await?;
    page.send_keys(Key::Down).await?;
    expect_focused_indicator(page, "Insert between One and Two").await?;
    page.send_keys(Key::Enter).await?;
    assert_that!(|| page.inner_texts("[role=grid][aria-label=List] [role=row][aria-selected]"))
        .eventually_ok()
        .matches(eq(["One", "hello world", "Two", "Three"]))
        .await;
    let inserted = row(page, "hello world").await?;
    page.wait_for_focus(&inserted).await?;
    inserted
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    DndActions::new(page)
        .wait_for_log(
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
    DndActions::new(page)
        .log_stays(
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
            std::time::Duration::from_millis(100),
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

/// In a keyboard drag, ArrowDown visits every valid drop target in order (not "on Two") and wraps
/// around, and ArrowUp visits them in reverse ("should support arrow key navigation").
#[browser_test]
pub async fn arrow_key_navigation(page: &Page<'_>) -> Result<(), Report> {
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

/// In a keyboard drag, End moves to the last drop target (after Three) and Home back to the
/// collection itself ("supports Home and End").
#[browser_test]
pub async fn home_and_end(page: &Page<'_>) -> Result<(), Report> {
    start_drag(page, "").await?;
    expect_focused_indicator(page, "Drop on").await?;
    page.send_keys(Key::End).await?;
    expect_focused_indicator(page, "Insert after Three").await?;
    page.send_keys(Key::Home).await?;
    expect_focused_indicator(page, "Drop on").await?;
    DndActions::new(page)
        .wait_for_log(
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

/// In a keyboard drag, PageDown and PageUp move through the drop targets a page at a time (rows
/// 50px high in a 150px list) ("supports PageUp and PageDown").
#[browser_test]
pub async fn page_up_and_page_down(page: &Page<'_>) -> Result<(), Report> {
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

/// PageDown and PageUp skip invalid drop targets when only drops on items 0, 2, 3 and 5 are
/// allowed ("should skip invalid targets with PageUp and PageDown").
#[browser_test]
pub async fn page_up_and_page_down_skip_invalid_targets(page: &Page<'_>) -> Result<(), Report> {
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
async fn focus_first_row(page: &Page<'_>, query: &str) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}{query}")).await?;
    focus_draggable(page).await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&row(page, "One").await?).await?;
    Ok(())
}

/// Shift + Tab back to the draggable and starts a keyboard drag.
async fn drag_from_the_draggable(page: &Page<'_>) -> Result<(), Report> {
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&page.element(role(AriaRole::Button).text("Drag me")).await?)
        .await?;
    page.send_keys(Key::Enter).await?;
    Ok(())
}

/// Selects One, Three and Two (in that order), ending focused on Two.
async fn select_one_three_two(page: &Page<'_>) -> Result<(), Report> {
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
async fn select_three_two(page: &Page<'_>) -> Result<(), Report> {
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

/// A keyboard drag starts at the drop target after the last focused row ("should default to
/// dropping after the last focused item if any").
#[browser_test]
pub async fn after_the_last_focused_item(page: &Page<'_>) -> Result<(), Report> {
    focus_first_row(page, "").await?;
    page.send_keys(Key::Down).await?;
    page.wait_for_focus(&row(page, "Two").await?).await?;
    drag_from_the_draggable(page).await?;
    expect_focused_indicator(page, "Insert between Two and Three").await?;
    page.send_keys(Key::Escape).await?;
    Ok(())
}

/// With rows selected, a keyboard drag starts at the drop target after the last selected row
/// ("should default to dropping after the selected items if any").
#[browser_test]
pub async fn after_the_selected_items(page: &Page<'_>) -> Result<(), Report> {
    focus_first_row(page, "").await?;
    select_one_three_two(page).await?;
    drag_from_the_draggable(page).await?;
    expect_focused_indicator(page, "Insert after Three").await?;
    page.send_keys(Key::Escape).await?;
    Ok(())
}

/// When the last focused row is the first selected one, a keyboard drag starts before the
/// selected rows ("should default to before the selected items if the last focused item is the
/// first selected item").
#[browser_test]
pub async fn before_the_selected_items(page: &Page<'_>) -> Result<(), Report> {
    focus_first_row(page, "").await?;
    select_three_two(page).await?;
    drag_from_the_draggable(page).await?;
    expect_focused_indicator(page, "Insert between One and Two").await?;
    page.send_keys(Key::Escape).await?;
    Ok(())
}

/// When only drops on rows are allowed and the last focused row is the first selected one, a
/// keyboard drag starts on that row ("should default to on the first selected item if the last
/// focused item is the first selected item and only dropping on items is allowed").
#[browser_test]
pub async fn on_the_first_selected_item(page: &Page<'_>) -> Result<(), Report> {
    focus_first_row(page, "?only-on&cancel=").await?;
    select_three_two(page).await?;
    drag_from_the_draggable(page).await?;
    expect_focused_indicator(page, "Drop on Two").await?;
    page.send_keys(Key::Escape).await?;
    Ok(())
}

/// When only drops on rows are allowed, a keyboard drag starts on the last selected row ("should
/// default to on the last selected item when only dropping on items is allowed").
#[browser_test]
pub async fn on_the_last_selected_item(page: &Page<'_>) -> Result<(), Report> {
    focus_first_row(page, "?only-on&cancel=").await?;
    select_one_three_two(page).await?;
    drag_from_the_draggable(page).await?;
    expect_focused_indicator(page, "Drop on Three").await?;
    page.send_keys(Key::Escape).await?;
    Ok(())
}

/// The cell of the row `text`.
async fn cell(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    row(page, text).await?.element("[role=gridcell]").await
}

/// In a native drag the drop target follows the pointer before, on and after rows (skipping the
/// invalid "on Two"), and the drop inserts the item, which is then focused and selected ("should
/// perform basic drag and drop").
#[browser_test]
pub async fn native_basic_drag_and_drop(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let draggable = page.element(role(AriaRole::Button).text("Drag me")).await?;
    DndActions::new(page)
        .fire_drag_event(&draggable, DragKind::Start, &[])
        .await?;
    let one = cell(page, "One").await?;
    let two = cell(page, "Two").await?;
    let three = cell(page, "Three").await?;
    DndActions::new(page)
        .fire_drag_event_at(&one, DragKind::Enter, &[], Some((1.0, 1.0)))
        .await?;
    DndActions::new(page)
        .wait_for_log(LOG, &["enter 1 before"])
        .await?;
    DndActions::new(page)
        .fire_drag_event_at(&one, DragKind::Over, &[], Some((1.0, 20.0)))
        .await?;
    DndActions::new(page)
        .wait_for_log(LOG, &["enter 1 before", "exit 1 before", "enter 1 on"])
        .await?;
    let transfer = DndActions::new(page).transfer().await?;
    assert_that!(transfer.drop_effect).is_equal_to("copy");
    DndActions::new(page)
        .fire_drag_event_at(&two, DragKind::Over, &[], Some((1.0, 3.0)))
        .await?;
    // "On Two" isn't valid: its upper half is before it, its lower half after it.
    DndActions::new(page)
        .fire_drag_event_at(&two, DragKind::Over, &[], Some((1.0, 30.0)))
        .await?;
    DndActions::new(page)
        .wait_for_log(
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
    DndActions::new(page)
        .fire_drag_event_at(&three, DragKind::Drop, &[], Some((2.0, 2.0)))
        .await?;
    DndActions::new(page)
        .fire_drag_event(&draggable, DragKind::End, &[])
        .await?;
    assert_that!(|| page.inner_texts("[role=grid][aria-label=List] [role=row][aria-selected]"))
        .eventually_ok()
        .matches(eq(["One", "Two", "hello world", "Three"]))
        .await;
    let inserted = row(page, "hello world").await?;
    page.wait_for_focus(&inserted).await?;
    inserted
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    DndActions::new(page)
        .wait_for_log(
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
    DndActions::new(page)
        .log_stays(
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
            std::time::Duration::from_millis(100),
        )
        .await?;
    Ok(())
}

/// A native drop on a row copies onto it and focuses it without selecting it or inserting a row
/// ("supports dropping on an item").
#[browser_test]
pub async fn native_drop_on_an_item(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let draggable = page.element(role(AriaRole::Button).text("Drag me")).await?;
    DndActions::new(page)
        .fire_drag_event(&draggable, DragKind::Start, &[])
        .await?;
    let one = cell(page, "One").await?;
    DndActions::new(page)
        .fire_drag_event_at(&one, DragKind::Enter, &[], Some((1.0, 20.0)))
        .await?;
    DndActions::new(page)
        .wait_for_log(LOG, &["enter 1 on"])
        .await?;
    let transfer = DndActions::new(page).transfer().await?;
    assert_that!(transfer.drop_effect).is_equal_to("copy");
    DndActions::new(page)
        .fire_drag_event_at(&one, DragKind::Drop, &[], Some((2.0, 2.0)))
        .await?;
    DndActions::new(page)
        .fire_drag_event(&draggable, DragKind::End, &[])
        .await?;
    let one_row = row(page, "One").await?;
    page.wait_for_focus(&one_row).await?;
    DndActions::new(page)
        .wait_for_log(
            LOG,
            &[
                "enter 1 on",
                "on 1 hello world Copy",
                "exit 1 on",
                "dragend Copy",
            ],
        )
        .await?;
    DndActions::new(page)
        .log_stays(
            LOG,
            &[
                "enter 1 on",
                "on 1 hello world Copy",
                "exit 1 on",
                "dragend Copy",
            ],
            std::time::Duration::from_millis(100),
        )
        .await?;
    assert_that!(
        page.inner_texts("[role=grid][aria-label=List] [role=row][aria-selected]")
            .await?
    )
    .contains_exactly(["One", "Two", "Three"]);
    assert_that!(one_row)
        .has_attribute("aria-selected")
        .await
        .is_equal_to("false");
    Ok(())
}

/// The drop indicator labelled `label`.
async fn indicator(page: &Page<'_>, label: &str) -> Result<WebElement, Report> {
    page.element(format!(
        "[aria-roledescription='drop indicator'][aria-label='{label}']"
    ))
    .await
}

/// The drop indicators a screen reader drag offers (in order): label, `aria-labelledby` and
/// `tabindex`, and the description.
async fn offered_indicators(
    page: &Page<'_>,
) -> Result<Vec<(String, Option<String>, Option<String>, String)>, Report> {
    let mut offered = Vec::new();
    for indicator in page
        .elements("[aria-roledescription='drop indicator']:not([aria-hidden])")
        .await?
    {
        offered.push((
            indicator.attr("aria-label").await?.unwrap_or_default(),
            indicator.attr("aria-labelledby").await?,
            indicator.attr("tabindex").await?,
            indicator.accessible_description().await?,
        ));
    }
    Ok(offered)
}

/// Opens the fixture and starts a screen reader drag of "Drag me" (focus and a virtual click);
/// waits until the drop indicators are offered.
async fn start_virtual_drag(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let draggable = page.element(role(AriaRole::Button).text("Drag me")).await?;
    draggable.focus().await?;
    draggable.virtual_click().await?;
    indicator(page, "Insert before One")
        .await?
        .wait_for_attr("aria-hidden", None)
        .await?;
    Ok(())
}

/// In a screen reader drag, clicking a drop indicator inserts the item there, which is then
/// focused and selected ("should perform basic drag and drop").
#[browser_test]
pub async fn screen_reader_basic_drag_and_drop(page: &Page<'_>) -> Result<(), Report> {
    start_virtual_drag(page).await?;
    let between = indicator(page, "Insert between One and Two").await?;
    between.focus().await?;
    between.virtual_click().await?;
    assert_that!(|| page.inner_texts("[role=grid][aria-label=List] [role=row][aria-selected]"))
        .eventually_ok()
        .matches(eq(["One", "hello world", "Two", "Three"]))
        .await;
    let inserted = row(page, "hello world").await?;
    page.wait_for_focus(&inserted).await?;
    inserted
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    DndActions::new(page)
        .wait_for_log(
            LOG,
            &[
                "enter root",
                "exit root",
                "enter 2 before",
                "insert 2 before hello world Move",
                "dragend Move",
                "exit 2 before",
            ],
        )
        .await?;
    Ok(())
}

/// In a screen reader drag, every drop indicator offered is described as a drop target ("Click to
/// drop."); the collection itself isn't ("should add descriptions to each item").
#[browser_test]
pub async fn screen_reader_descriptions(page: &Page<'_>) -> Result<(), Report> {
    start_virtual_drag(page).await?;
    let grid = page.element("[role=grid][aria-label=List]").await?;
    assert_that!(grid)
        .attribute("aria-describedby")
        .await
        .is_none();
    let descriptions: Vec<String> = offered_indicators(page)
        .await?
        .into_iter()
        .map(|(_, _, _, description)| description)
        .collect();
    assert_that!(descriptions).is_equal_to(vec!["Click to drop.".to_owned(); 7]);
    Ok(())
}

/// In a screen reader drag, the drop indicators offered are labelled by their position (the root
/// indicator also by the collection), out of the tab order ("should show insertion
/// indicators").
#[browser_test]
pub async fn screen_reader_insertion_indicators(page: &Page<'_>) -> Result<(), Report> {
    start_virtual_drag(page).await?;
    let root = indicator(page, "Drop on").await?;
    let root_id = root.attr("id").await?.unwrap_or_default();
    let grid_id = page
        .element("[role=grid][aria-label=List]")
        .await?
        .attr("id")
        .await?
        .unwrap_or_default();
    let offered: Vec<(String, Option<String>, Option<String>)> = offered_indicators(page)
        .await?
        .into_iter()
        .map(|(label, labelledby, tabindex, _)| (label, labelledby, tabindex))
        .collect();
    let positioned = |label: &str| (label.to_owned(), None, Some("-1".to_owned()));
    assert_that!(offered).is_equal_to(vec![
        (
            "Drop on".to_owned(),
            Some(format!("{root_id} {grid_id}")),
            Some("-1".to_owned()),
        ),
        positioned("Insert before One"),
        positioned("Drop on One"),
        positioned("Insert between One and Two"),
        positioned("Insert between Two and Three"),
        positioned("Drop on Three"),
        positioned("Insert after Three"),
    ]);
    Ok(())
}

/// In a screen reader drag, a row that doesn't take the drop ("Two") offers no drop on it
/// ("should hide items that do not accept the drop").
#[browser_test]
pub async fn screen_reader_hides_rows_not_taking_the_drop(page: &Page<'_>) -> Result<(), Report> {
    start_virtual_drag(page).await?;
    let on_two = indicator(page, "Drop on Two").await?;
    assert_that!(on_two)
        .has_attribute("aria-hidden")
        .await
        .is_equal_to("true");
    let on_one = indicator(page, "Drop on One").await?;
    assert_that!(on_one)
        .attribute("aria-hidden")
        .await
        .is_none();
    Ok(())
}

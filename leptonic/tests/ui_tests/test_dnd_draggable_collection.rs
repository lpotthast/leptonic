// Upstream: react-aria/test/dnd/useDraggableCollection.test.js @ 99e6102368
//! Dragging items of a collection (`use_draggable_collection_state`, `use_draggable_item`,
//! `use_draggable_collection`): a grid with drag buttons and a list box without, dragged
//! natively, with the keyboard and with a screen reader; a selected item drags the whole
//! selection, an item outside the selection only itself. Spec: react-aria's
//! `useDraggableCollection.test.js`. Fixture: `/hooks/dnd-draggable-collection`.
//!
//! The screen reader list box cases run on an emulated iPhone (VoiceOver's drag) but with a fine
//! pointer: the touch descriptions they check upstream ("Long press to start dragging") need a
//! coarse one, covered by `messages`' native tests.
use std::time::Duration;

use assertr::{matchers::eq, prelude::*};
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::Report;

use crate::{
    fixtures::dnd::{DndActions, DragImage},
    pages::{
        DragKind, ElementActions, Page, Platform, PointerKind, PointerType, SyntheticEvent, css,
        role,
    },
};

const PATH: &str = "/hooks/dnd-draggable-collection";
const LOG: &str = "test-dnd-draggable-log";
const ITEMS_JSON: &str = "application/vnd.react-aria.items+json";

/// The row `text` of the grid.
async fn row(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.element("[role=grid][aria-label='Draggable list']")
        .await?
        .element(role(AriaRole::Row).has(css("span").text(text)))
        .await
}

/// The cell of the row `text`.
async fn cell(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    row(page, text)
        .await?
        .element(role(AriaRole::Gridcell))
        .await
}

/// The option `text` of the list box.
async fn option(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.element("[role=listbox]")
        .await?
        .element(role(AriaRole::Option).text(text))
        .await
}

/// The drop target "Drop here".
async fn target(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element(role(AriaRole::Button).text("Drop here")).await
}

/// The texts of the grid's rows.
async fn rows(page: &Page<'_>) -> Result<Vec<String>, Report> {
    page.inner_texts("[role=grid] [role=row] span").await
}

/// Fires a native drag of the transfer the drag started with onto "Drop here" and drops it.
async fn native_drop(page: &Page<'_>) -> Result<(), Report> {
    let target = target(page).await?;
    let dnd = DndActions::new(page);
    dnd.fire_drag_event(&target, DragKind::Enter, &[]).await?;
    dnd.fire_drag_event(&target, DragKind::Over, &[]).await?;
    dnd.fire_drag_event(&target, DragKind::Drop, &[]).await?;
    Ok(())
}

/// Focuses the first row ("Foo") with the keyboard (Tab into the grid), then the row `text`
/// below it (ArrowDown).
async fn focus_row(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.element(role(AriaRole::Button).text("Before"))
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&row(page, "Foo").await?).await?;
    let row = row(page, text).await?;
    if text != "Foo" {
        page.send_keys(Key::Down).await?;
        page.wait_for_focus(&row).await?;
    }
    Ok(row)
}

// ---- Native ----

/// A native drag of an item that isn't selected drags it with its data and preview, the drop
/// gets it, and a move removes it ("should drag a single item").
#[browser_test]
pub async fn native_a_single_item(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=grid")).await?;
    let bar = cell(page, "Bar").await?;
    let dnd = DndActions::new(page);
    dnd.fire_drag_event_at(&bar, DragKind::Start, &[], Some((0.0, 0.0)))
        .await?;
    row(page, "Bar")
        .await?
        .wait_for_attr("data-dragging", Some("true"))
        .await?;
    assert_that!(row(page, "Foo").await?)
        .attribute("data-dragging")
        .await
        .is_none();
    assert_that!(row(page, "Baz").await?)
        .attribute("data-dragging")
        .await
        .is_none();
    let transfer = dnd.transfer().await?;
    assert_that!(transfer.data).contains_exactly([
        ("folder".to_owned(), "Bar".to_owned()),
        ("text/plain".to_owned(), "Bar".to_owned()),
        (
            ITEMS_JSON.to_owned(),
            r#"[{"folder":"Bar","text/plain":"Bar"}]"#.to_owned(),
        ),
    ]);
    assert_that!(transfer.drag_image.map(|image| image.text)).is_equal_to(Some("Bar".to_owned()));
    dnd.fire_drag_event_at(&bar, DragKind::Drag, &[], Some((1.0, 1.0)))
        .await?;
    native_drop(page).await?;
    dnd.fire_drag_event(&bar, DragKind::End, &[]).await?;
    dnd.wait_for_log(
        LOG,
        &[
            "start bar",
            "move bar",
            "drop Move",
            "item folder=Bar, text/plain=Bar",
            "end Move bar internal=false",
        ],
    )
    .await?;
    assert_that!(|| rows(page))
        .eventually_ok()
        .matches(eq(["Foo", "Baz"]))
        .await;
    Ok(())
}

/// A native drag of a selected item drags the whole selection: native types joined, all items
/// as JSON, a preview with the count ("should drag multiple selected items").
#[browser_test]
pub async fn native_several_selected_items(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=grid")).await?;
    cell(page, "Foo").await?.click().await?;
    row(page, "Foo")
        .await?
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    cell(page, "Bar").await?.click().await?;
    row(page, "Bar")
        .await?
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    let bar = cell(page, "Bar").await?;
    let dnd = DndActions::new(page);
    dnd.fire_drag_event_at(&bar, DragKind::Start, &[], Some((0.0, 0.0)))
        .await?;
    row(page, "Foo")
        .await?
        .wait_for_attr("data-dragging", Some("true"))
        .await?;
    row(page, "Bar")
        .await?
        .wait_for_attr("data-dragging", Some("true"))
        .await?;
    assert_that!(row(page, "Baz").await?)
        .attribute("data-dragging")
        .await
        .is_none();
    let transfer = dnd.transfer().await?;
    assert_that!(transfer.data).contains_exactly([
        ("folder".to_owned(), "Foo".to_owned()),
        ("text/plain".to_owned(), "Foo\nBar".to_owned()),
        (
            ITEMS_JSON.to_owned(),
            r#"[{"folder":"Foo","text/plain":"Foo"},{"folder":"Bar","text/plain":"Bar"}]"#
                .to_owned(),
        ),
    ]);
    assert_that!(transfer.drag_image.map(|image| image.text)).is_equal_to(Some("Bar 2".to_owned()));
    native_drop(page).await?;
    dnd.fire_drag_event(&bar, DragKind::End, &[]).await?;
    dnd.wait_for_log(
        LOG,
        &[
            "start foo,bar",
            "drop Move",
            "item folder=Foo, text/plain=Foo",
            "item folder=Bar, text/plain=Bar",
            "end Move foo,bar internal=false",
        ],
    )
    .await?;
    assert_that!(|| rows(page))
        .eventually_ok()
        .matches(eq(["Baz"]))
        .await;
    Ok(())
}

/// A native drag of an item outside the selection drags only that item ("should only drag
/// dragged item when not selected").
#[browser_test]
pub async fn native_only_the_dragged_item(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=grid")).await?;
    cell(page, "Foo").await?.click().await?;
    row(page, "Foo")
        .await?
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    let bar = cell(page, "Bar").await?;
    let dnd = DndActions::new(page);
    dnd.fire_drag_event_at(&bar, DragKind::Start, &[], Some((0.0, 0.0)))
        .await?;
    row(page, "Bar")
        .await?
        .wait_for_attr("data-dragging", Some("true"))
        .await?;
    assert_that!(row(page, "Foo").await?)
        .attribute("data-dragging")
        .await
        .is_none();
    let transfer = dnd.transfer().await?;
    assert_that!(transfer.drag_image).is_equal_to(Some(DragImage {
        text: "Bar".to_owned(),
        x: 0.0,
        y: 0.0,
    }));
    native_drop(page).await?;
    dnd.fire_drag_event(&bar, DragKind::End, &[]).await?;
    dnd.wait_for_log(
        LOG,
        &[
            "start bar",
            "drop Move",
            "item folder=Bar, text/plain=Bar",
            "end Move bar internal=false",
        ],
    )
    .await?;
    assert_that!(|| rows(page))
        .eventually_ok()
        .matches(eq(["Foo", "Baz"]))
        .await;
    Ok(())
}

// ---- Keyboard ----

/// Drops the keyboard drag on "Drop here" (which the drag focused) with Enter.
async fn keyboard_drop(page: &Page<'_>) -> Result<(), Report> {
    page.wait_for_focus(&target(page).await?).await?;
    page.send_keys(Key::Enter).await?;
    Ok(())
}

/// Enter on a row's drag button (reached with ArrowRight) drags that row with the keyboard; Enter
/// on the drop target drops it ("should drag a single item").
#[browser_test]
pub async fn keyboard_a_single_item(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=grid")).await?;
    // Non-dragged rows are hidden from accessibility while the keyboard drag is active.
    let source_item = row(page, "Foo").await?;
    focus_row(page, "Bar").await?;
    page.send_keys(Key::Right).await?;
    let button = row(page, "Bar").await?.element("button").await?;
    page.wait_for_focus(&button).await?;
    assert_that!(button)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Drag Bar");
    page.send_keys(Key::Enter).await?;
    row(page, "Bar")
        .await?
        .wait_for_attr("data-dragging", Some("true"))
        .await?;
    assert_that!(source_item)
        .attribute("data-dragging")
        .await
        .is_none();
    keyboard_drop(page).await?;
    DndActions::new(page)
        .wait_for_log(
            LOG,
            &[
                "start bar",
                "drop Move",
                "item folder=Bar, text/plain=Bar",
                "end Move bar internal=false",
            ],
        )
        .await?;
    assert_that!(|| rows(page))
        .eventually_ok()
        .matches(eq(["Foo", "Baz"]))
        .await;
    Ok(())
}

/// The drag button of a selected row drags the whole selection, and says so ("Drag 2 selected
/// items") ("should drag multiple selected items").
#[browser_test]
pub async fn keyboard_several_selected_items(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=grid")).await?;
    let source_item = focus_row(page, "Foo").await?;
    page.send_keys(Key::Space).await?;
    source_item
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    page.send_keys(Key::Down).await?;
    let bar = row(page, "Bar").await?;
    page.wait_for_focus(&bar).await?;
    page.send_keys(Key::Space).await?;
    bar.wait_for_attr("aria-selected", Some("true")).await?;
    page.send_keys(Key::Right).await?;
    let button = bar.element("button").await?;
    page.wait_for_focus(&button).await?;
    button
        .wait_for_attr("aria-label", Some("Drag 2 selected items"))
        .await?;
    page.send_keys(Key::Enter).await?;
    source_item
        .wait_for_attr("data-dragging", Some("true"))
        .await?;
    bar.wait_for_attr("data-dragging", Some("true")).await?;
    keyboard_drop(page).await?;
    DndActions::new(page)
        .wait_for_log(
            LOG,
            &[
                "start foo,bar",
                "drop Move",
                "item folder=Foo, text/plain=Foo",
                "item folder=Bar, text/plain=Bar",
                "end Move foo,bar internal=false",
            ],
        )
        .await?;
    assert_that!(|| rows(page))
        .eventually_ok()
        .matches(eq(["Baz"]))
        .await;
    Ok(())
}

/// The drag button of a row outside the selection drags only that row ("should only drag current
/// item when not selected").
#[browser_test]
pub async fn keyboard_only_the_current_item(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=grid")).await?;
    let source_item = focus_row(page, "Foo").await?;
    page.send_keys(Key::Space).await?;
    source_item
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    page.send_keys(Key::Down).await?;
    let bar = row(page, "Bar").await?;
    page.wait_for_focus(&bar).await?;
    page.send_keys(Key::Right).await?;
    page.wait_for_focus(&bar.element("button").await?).await?;
    page.send_keys(Key::Enter).await?;
    bar.wait_for_attr("data-dragging", Some("true")).await?;
    assert_that!(source_item)
        .attribute("data-dragging")
        .await
        .is_none();
    keyboard_drop(page).await?;
    DndActions::new(page)
        .wait_for_log(
            LOG,
            &[
                "start bar",
                "drop Move",
                "item folder=Bar, text/plain=Bar",
                "end Move bar internal=false",
            ],
        )
        .await?;
    Ok(())
}

/// In a list box without drag buttons, the options describe how to drag them (and how many) and
/// Enter drags the selection ("should work with a listbox without a drag button").
#[browser_test]
pub async fn keyboard_a_list_box_without_drag_buttons(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=listbox")).await?;
    page.element(role(AriaRole::Button).text("Before"))
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    let source_item = option(page, "Foo").await?;
    page.wait_for_focus(&source_item).await?;
    page.send_keys(Key::Space).await?;
    source_item
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    assert_that!(|| source_item.accessible_description())
        .eventually_ok()
        .matches(eq("Press Enter to start dragging."))
        .await;
    page.send_keys(Key::Down).await?;
    let bar = option(page, "Bar").await?;
    page.wait_for_focus(&bar).await?;
    page.send_keys(Key::Space).await?;
    bar.wait_for_attr("aria-selected", Some("true")).await?;
    assert_that!(|| bar.accessible_description())
        .eventually_ok()
        .matches(eq("Press Enter to drag 2 selected items."))
        .await;
    page.send_keys(Key::Enter).await?;
    keyboard_drop(page).await?;
    DndActions::new(page)
        .wait_for_log(
            LOG,
            &[
                "start foo,bar",
                "drop Move",
                "item text/plain=Foo",
                "item text/plain=Bar",
                "end Move foo,bar internal=false",
            ],
        )
        .await?;
    Ok(())
}

/// In a list box whose options have an action, Enter performs the action and Alt + Enter drags
/// (both described) ("should support row actions").
#[browser_test]
pub async fn keyboard_row_actions(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=listbox&action"))
        .await?;
    page.element(role(AriaRole::Button).text("Before"))
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    let source_item = option(page, "Foo").await?;
    page.wait_for_focus(&source_item).await?;
    page.send_keys(Key::Space).await?;
    source_item
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    assert_that!(|| source_item.accessible_description())
        .eventually_ok()
        .matches(eq("Press Alt + Enter to start dragging."))
        .await;
    page.send_keys(Key::Shift + Key::Down).await?;
    let bar = option(page, "Bar").await?;
    page.wait_for_focus(&bar).await?;
    bar.wait_for_attr("aria-selected", Some("true")).await?;
    assert_that!(|| bar.accessible_description())
        .eventually_ok()
        .matches(eq("Press Alt + Enter to drag 2 selected items."))
        .await;
    page.send_keys(Key::Enter).await?;
    let dnd = DndActions::new(page);
    dnd.wait_for_log(LOG, &["action bar"]).await?;
    dnd.log_stays(LOG, &["action bar"], Duration::from_millis(100))
        .await?;
    page.send_keys(Key::Alt + Key::Enter).await?;
    keyboard_drop(page).await?;
    dnd.wait_for_log(
        LOG,
        &[
            "action bar",
            "start foo,bar",
            "drop Move",
            "item text/plain=Foo",
            "item text/plain=Bar",
            "end Move foo,bar internal=false",
        ],
    )
    .await?;
    Ok(())
}

// ---- Screen reader ----

/// Drops the screen reader drag on "Drop here": focus and a virtual click.
async fn virtual_drop(page: &Page<'_>) -> Result<(), Report> {
    let target = target(page).await?;
    target.focus().await?;
    target.virtual_click().await?;
    Ok(())
}

/// A virtual click on a row's drag button drags that row; one on the drop target drops it
/// ("should drag a single item").
#[browser_test]
pub async fn screen_reader_a_single_item(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=grid")).await?;
    let bar = row(page, "Bar").await?;
    let button = bar.element("button").await?;
    assert_that!(button)
        .has_attribute("aria-label")
        .await
        .is_equal_to("Drag Bar");
    button.focus().await?;
    button.virtual_click().await?;
    bar.wait_for_attr("data-dragging", Some("true")).await?;
    page.wait_for_focus(&button).await?;
    virtual_drop(page).await?;
    DndActions::new(page)
        .wait_for_log(
            LOG,
            &[
                "start bar",
                "drop Move",
                "item folder=Bar, text/plain=Bar",
                "end Move bar internal=false",
            ],
        )
        .await?;
    Ok(())
}

/// A virtual click on the drag button of a selected row drags the whole selection ("should drag
/// multiple selected items").
#[browser_test]
pub async fn screen_reader_several_selected_items(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=grid")).await?;
    let source_item = row(page, "Foo").await?;
    let bar = row(page, "Bar").await?;
    cell(page, "Foo").await?.virtual_click().await?;
    source_item
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    cell(page, "Bar").await?.virtual_click().await?;
    bar.wait_for_attr("aria-selected", Some("true")).await?;
    let button = bar.element("button").await?;
    button
        .wait_for_attr("aria-label", Some("Drag 2 selected items"))
        .await?;
    button.focus().await?;
    button.virtual_click().await?;
    source_item
        .wait_for_attr("data-dragging", Some("true"))
        .await?;
    bar.wait_for_attr("data-dragging", Some("true")).await?;
    virtual_drop(page).await?;
    DndActions::new(page)
        .wait_for_log(
            LOG,
            &[
                "start foo,bar",
                "drop Move",
                "item folder=Foo, text/plain=Foo",
                "item folder=Bar, text/plain=Bar",
                "end Move foo,bar internal=false",
            ],
        )
        .await?;
    Ok(())
}

/// A virtual click on the drag button of a row outside the selection drags only that row
/// ("should drag only clicked item when not selected").
#[browser_test]
pub async fn screen_reader_only_the_clicked_item(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=grid")).await?;
    let source_item = row(page, "Foo").await?;
    cell(page, "Foo").await?.virtual_click().await?;
    source_item
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    let bar = row(page, "Bar").await?;
    let button = bar.element("button").await?;
    button.focus().await?;
    button.virtual_click().await?;
    bar.wait_for_attr("data-dragging", Some("true")).await?;
    assert_that!(source_item)
        .attribute("data-dragging")
        .await
        .is_none();
    virtual_drop(page).await?;
    DndActions::new(page)
        .wait_for_log(
            LOG,
            &[
                "start bar",
                "drop Move",
                "item folder=Bar, text/plain=Bar",
                "end Move bar internal=false",
            ],
        )
        .await?;
    Ok(())
}

/// Starts a VoiceOver drag of `option` on iOS: VoiceOver's pointer (a touch smaller than 1×1),
/// then the `dragstart` it causes.
async fn voiceover_drag(page: &Page<'_>, option: &WebElement) -> Result<(), Report> {
    option
        .dispatch(
            SyntheticEvent::pointer(PointerKind::Down)
                .pointer_type(PointerType::Touch)
                .size(0.33, 0.33),
        )
        .await?;
    DndActions::new(page)
        .fire_drag_event(option, DragKind::Start, &[])
        .await?;
    Ok(())
}

/// With VoiceOver on iOS, dragging a selected option of a list box without drag buttons drags
/// the selection as a screen reader drag ("should work with a listbox without a drag button").
#[browser_test]
pub async fn screen_reader_a_list_box_without_drag_buttons(page: &Page<'_>) -> Result<(), Report> {
    page.emulate_platform(Platform::IPhone).await?;
    page.goto_path(&format!("{PATH}?only=listbox")).await?;
    let source_item = option(page, "Foo").await?;
    let bar = option(page, "Bar").await?;
    source_item.click().await?;
    source_item
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    bar.click().await?;
    bar.wait_for_attr("aria-selected", Some("true")).await?;
    voiceover_drag(page, &bar).await?;
    bar.wait_for_attr("data-dragging", Some("true")).await?;
    source_item
        .wait_for_attr("data-dragging", Some("true"))
        .await?;
    virtual_drop(page).await?;
    DndActions::new(page)
        .wait_for_log(
            LOG,
            &[
                "start foo,bar",
                "drop Move",
                "item text/plain=Foo",
                "item text/plain=Bar",
                "end Move foo,bar internal=false",
            ],
        )
        .await?;
    Ok(())
}

/// With VoiceOver on iOS, options with an action are dragged the same way ("should support row
/// actions").
#[browser_test]
pub async fn screen_reader_row_actions(page: &Page<'_>) -> Result<(), Report> {
    page.emulate_platform(Platform::IPhone).await?;
    page.goto_path(&format!("{PATH}?only=listbox&action"))
        .await?;
    let source_item = option(page, "Foo").await?;
    let bar = option(page, "Bar").await?;
    source_item.click().await?;
    source_item
        .wait_for_attr("aria-selected", Some("true"))
        .await?;
    page.send_keys(Key::Shift + Key::Down).await?;
    bar.wait_for_attr("aria-selected", Some("true")).await?;
    voiceover_drag(page, &bar).await?;
    bar.wait_for_attr("data-dragging", Some("true")).await?;
    virtual_drop(page).await?;
    DndActions::new(page)
        .wait_for_log(
            LOG,
            &[
                "start foo,bar",
                "drop Move",
                "item text/plain=Foo",
                "item text/plain=Bar",
                "end Move foo,bar internal=false",
            ],
        )
        .await?;
    Ok(())
}

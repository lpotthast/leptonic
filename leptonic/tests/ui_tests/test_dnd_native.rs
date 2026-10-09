// Upstream: react-aria/test/dnd/dnd.test.js @ 99e6102368
//! Native drags between single elements (`use_drag`, `use_drop`) with synthesized `DragEvent`s
//! carrying a stand-in transfer: the events a drop target and a drag source report, nested drag
//! sources and drop targets, the dragged data (custom types, several items, files and
//! directories), drop operations (modifier keys, WebKit's wrong `effectAllowed`, file types) and
//! drag previews; keyboard drags of custom data and between nested drop targets. Spec:
//! react-aria's `dnd.test.js` ("native drag and drop", "keyboard"). Fixture: `/hooks/dnd-native`.
//!
//! Not mirrored: "does not fire onDropEnter and onDropExit repeatedly for portal children" (React
//! portals bubble events through the component tree, Leptos' `Portal` doesn't).
use std::time::Duration;

use assertr::prelude::*;
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::Report;

use crate::{
    fixtures::dnd::{DndActions, DragImage, ExternalData},
    pages::{DragKind, ElementActions, KeyKind, Modifier, Page, SyntheticEvent, role},
};

const PATH: &str = "/hooks/dnd-native";
const LOG: &str = "test-dnd-native-log";
/// How long a drag hovers a drop target before it activates (`use_drop`'s timer).
const ACTIVATE: Duration = Duration::from_millis(800);

/// The fixture's element named `name` (`data-name`).
async fn named(page: &Page<'_>, name: &str) -> Result<WebElement, Report> {
    page.element(format!("[data-name='{name}']")).await
}

/// "hello world" as plain text, dragged from outside the page.
const HELLO: ExternalData<'static> = ExternalData::Text {
    kind: "text/plain",
    data: "hello world",
};

/// Starts a keyboard drag from "Drag me": the nearest drop target takes the focus.
async fn start_keyboard_drag(page: &Page<'_>) -> Result<(), Report> {
    let source = named(page, "source").await?;
    source.focus().await?;
    page.wait_for_focus(&source).await?;
    page.send_keys(Key::Enter).await?;
    Ok(())
}

// ---- Events ----

/// A drag source reports a move only when the drag's position changes ("fires onDragMove only
/// when the drag actually moves").
#[browser_test]
pub async fn drag_moves_only_when_moving(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic")).await?;
    let source = named(page, "source").await?;
    let dnd = DndActions::new(page);
    dnd.fire_drag_event_at(&source, DragKind::Start, &[], Some((0.0, 0.0)))
        .await?;
    dnd.fire_drag_event_at(&source, DragKind::Drag, &[], Some((0.0, 0.0)))
        .await?;
    dnd.log_stays(LOG, &["source start"], Duration::from_millis(100))
        .await?;
    dnd.fire_drag_event_at(&source, DragKind::Drag, &[], Some((1.0, 1.0)))
        .await?;
    dnd.fire_drag_event_at(&source, DragKind::Drag, &[], Some((1.0, 1.0)))
        .await?;
    dnd.log_stays(
        LOG,
        &["source start", "source move"],
        Duration::from_millis(100),
    )
    .await?;
    Ok(())
}

/// A drop target reports a move only when the drag's position changes, relative to the target
/// ("fires onDropMove only when the drag actually moves").
#[browser_test]
pub async fn drop_moves_only_when_moving(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic")).await?;
    let target = named(page, "target").await?;
    let dnd = DndActions::new(page);
    dnd.begin_external_drag(&[HELLO]).await?;
    dnd.fire_drag_event_at(&target, DragKind::Enter, &[], Some((1.0, 1.0)))
        .await?;
    dnd.fire_drag_event_at(&target, DragKind::Over, &[], Some((1.0, 1.0)))
        .await?;
    dnd.log_stays(LOG, &["target enter"], Duration::from_millis(100))
        .await?;
    dnd.fire_drag_event_at(&target, DragKind::Over, &[], Some((2.0, 2.0)))
        .await?;
    dnd.fire_drag_event_at(&target, DragKind::Over, &[], Some((2.0, 2.0)))
        .await?;
    dnd.log_stays(
        LOG,
        &["target enter", "target move 2 2"],
        Duration::from_millis(100),
    )
    .await?;
    Ok(())
}

/// Leaving a drop target exits it at the drag's position ("fires onDropExit when moving off a
/// drop target").
#[browser_test]
pub async fn drop_exit_when_leaving(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic&coords")).await?;
    let target = named(page, "target").await?;
    let dnd = DndActions::new(page);
    dnd.begin_external_drag(&[HELLO]).await?;
    dnd.fire_drag_event_at(&target, DragKind::Enter, &[], Some((1.0, 1.0)))
        .await?;
    dnd.fire_drag_event_at(&target, DragKind::Over, &[], Some((2.0, 2.0)))
        .await?;
    dnd.wait_for_log(LOG, &["target enter 1 1", "target move 2 2"])
        .await?;
    dnd.fire_drag_event_at(&target, DragKind::Leave, &[], Some((2.0, 2.0)))
        .await?;
    dnd.wait_for_log(
        LOG,
        &["target enter 1 1", "target move 2 2", "target exit 2 2"],
    )
    .await?;
    Ok(())
}

/// A drop also exits the drop target ("fires onDropExit on drop").
#[browser_test]
pub async fn drop_exit_on_drop(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic&coords")).await?;
    let target = named(page, "target").await?;
    let dnd = DndActions::new(page);
    dnd.begin_external_drag(&[HELLO]).await?;
    dnd.fire_drag_event_at(&target, DragKind::Enter, &[], Some((1.0, 1.0)))
        .await?;
    dnd.fire_drag_event_at(&target, DragKind::Over, &[], Some((2.0, 2.0)))
        .await?;
    dnd.fire_drag_event_at(&target, DragKind::Drop, &[], Some((2.0, 2.0)))
        .await?;
    dnd.wait_for_log(
        LOG,
        &[
            "target enter 1 1",
            "target move 2 2",
            "target drop 2 2 Move",
            "target exit 2 2",
            "target item text text/plain=hello world",
        ],
    )
    .await?;
    Ok(())
}

/// A drag held over a drop target activates it after 800 ms, at its last position ("fires
/// onDropActivate when a drag is held over the target").
#[browser_test]
pub async fn drop_activate_when_held(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic&coords")).await?;
    let target = named(page, "target").await?;
    let dnd = DndActions::new(page);
    dnd.begin_external_drag(&[HELLO]).await?;
    dnd.fire_drag_event_at(&target, DragKind::Enter, &[], Some((1.0, 1.0)))
        .await?;
    dnd.fire_drag_event_at(&target, DragKind::Over, &[], Some((2.0, 2.0)))
        .await?;
    // Not before the activation timer (800 ms).
    dnd.log_stays(
        LOG,
        &["target enter 1 1", "target move 2 2"],
        Duration::from_millis(500),
    )
    .await?;
    dnd.wait_for_log(
        LOG,
        &["target enter 1 1", "target move 2 2", "target activate 2 2"],
    )
    .await?;
    Ok(())
}

/// A drag that leaves the drop target before the activation timer fires doesn't activate it
/// ("does not fire onDropActivate if the drag leaves the target before the timer fires").
#[browser_test]
pub async fn no_drop_activate_after_leaving(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic")).await?;
    let target = named(page, "target").await?;
    let dnd = DndActions::new(page);
    dnd.begin_external_drag(&[HELLO]).await?;
    dnd.fire_drag_event_at(&target, DragKind::Enter, &[], Some((1.0, 1.0)))
        .await?;
    dnd.fire_drag_event_at(&target, DragKind::Over, &[], Some((2.0, 2.0)))
        .await?;
    dnd.log_stays(
        LOG,
        &["target enter", "target move 2 2"],
        Duration::from_millis(500),
    )
    .await?;
    dnd.fire_drag_event_at(&target, DragKind::Leave, &[], Some((2.0, 2.0)))
        .await?;
    // Past the activation timer (800 ms).
    dnd.log_stays(
        LOG,
        &["target enter", "target move 2 2", "target exit"],
        ACTIVATE + Duration::from_millis(200),
    )
    .await?;
    Ok(())
}

/// Entering and leaving an element inside the drop target neither enters nor exits the target
/// again; leaving the target itself exits it ("does not fire onDropEnter and onDropExit for
/// nested elements").
#[browser_test]
pub async fn nested_elements_neither_enter_nor_exit(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic")).await?;
    let target = named(page, "target").await?;
    let child = named(page, "child").await?;
    let dnd = DndActions::new(page);
    dnd.begin_external_drag(&[HELLO]).await?;
    dnd.fire_drag_event_at(&target, DragKind::Enter, &[], Some((1.0, 1.0)))
        .await?;
    dnd.fire_drag_event(&child, DragKind::Enter, &[]).await?;
    dnd.fire_drag_event(&child, DragKind::Leave, &[]).await?;
    dnd.log_stays(LOG, &["target enter"], Duration::from_millis(100))
        .await?;
    dnd.fire_drag_event_at(&target, DragKind::Leave, &[], Some((1.0, 1.0)))
        .await?;
    dnd.wait_for_log(LOG, &["target enter", "target exit"])
        .await?;
    Ok(())
}

/// Dragging a drag source inside another one moves only the inner one ("does not trigger parent
/// drag when dragging child").
#[browser_test]
pub async fn nested_drag_source(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=nested-drag")).await?;
    let child = named(page, "child").await?;
    let dnd = DndActions::new(page);
    dnd.fire_drag_event_at(&child, DragKind::Drag, &[], Some((1.0, 1.0)))
        .await?;
    dnd.log_stays(LOG, &["child move"], Duration::from_millis(100))
        .await?;
    Ok(())
}

/// A drop on a drop target inside another one drops on and exits only the inner one ("does not
/// trigger parent onDrop and onDropExit when dropping on child").
#[browser_test]
pub async fn nested_drop_target_drop(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=nested-drop")).await?;
    let inner = named(page, "inner").await?;
    let dnd = DndActions::new(page);
    dnd.begin_external_drag(&[HELLO]).await?;
    dnd.fire_drag_event_at(&inner, DragKind::Drop, &[], Some((1.0, 1.0)))
        .await?;
    dnd.wait_for_log(
        LOG,
        &[
            "inner drop Cancel",
            "inner exit",
            "inner item text text/plain=hello world",
        ],
    )
    .await?;
    dnd.log_stays(
        LOG,
        &[
            "inner drop Cancel",
            "inner exit",
            "inner item text text/plain=hello world",
        ],
        Duration::from_millis(100),
    )
    .await?;
    Ok(())
}

/// Entering a drop target inside another one enters only the inner one ("does not trigger parent
/// onDropEnter when entering drop child").
#[browser_test]
pub async fn nested_drop_target_enter(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=nested-drop")).await?;
    let inner = named(page, "inner").await?;
    let dnd = DndActions::new(page);
    dnd.begin_external_drag(&[HELLO]).await?;
    dnd.fire_drag_event_at(&inner, DragKind::Enter, &[], Some((1.0, 1.0)))
        .await?;
    dnd.log_stays(LOG, &["inner enter"], Duration::from_millis(100))
        .await?;
    Ok(())
}

/// Leaving a drop target inside another one exits only the inner one ("does not trigger parent
/// onDropExit when exiting child").
#[browser_test]
pub async fn nested_drop_target_exit(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=nested-drop")).await?;
    let inner = named(page, "inner").await?;
    let dnd = DndActions::new(page);
    dnd.begin_external_drag(&[HELLO]).await?;
    dnd.fire_drag_event_at(&inner, DragKind::Enter, &[], Some((1.0, 1.0)))
        .await?;
    dnd.fire_drag_event_at(&inner, DragKind::Leave, &[], Some((1.0, 1.0)))
        .await?;
    dnd.log_stays(
        LOG,
        &["inner enter", "inner exit"],
        Duration::from_millis(100),
    )
    .await?;
    Ok(())
}

/// A drag held over a drop target inside another one activates only the inner one ("does not
/// trigger parent onDropActivate when hovering on drop child").
#[browser_test]
pub async fn nested_drop_target_activate(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=nested-drop&coords"))
        .await?;
    let inner = named(page, "inner").await?;
    let dnd = DndActions::new(page);
    dnd.begin_external_drag(&[HELLO]).await?;
    dnd.fire_drag_event_at(&inner, DragKind::Enter, &[], Some((1.0, 1.0)))
        .await?;
    dnd.fire_drag_event_at(&inner, DragKind::Over, &[], Some((2.0, 2.0)))
        .await?;
    // Past the activation timer (800 ms).
    dnd.wait_for_log(
        LOG,
        &["inner enter 1 1", "inner move 2 2", "inner activate 2 2"],
    )
    .await?;
    dnd.log_stays(
        LOG,
        &["inner enter 1 1", "inner move 2 2", "inner activate 2 2"],
        ACTIVATE,
    )
    .await?;
    Ok(())
}

/// A drag moving over a drop target inside another one moves only over the inner one ("does not
/// trigger parent onDropMove when moving on drop child").
#[browser_test]
pub async fn nested_drop_target_move(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=nested-drop")).await?;
    let inner = named(page, "inner").await?;
    let dnd = DndActions::new(page);
    dnd.begin_external_drag(&[HELLO]).await?;
    dnd.fire_drag_event_at(&inner, DragKind::Enter, &[], Some((1.0, 1.0)))
        .await?;
    dnd.fire_drag_event_at(&inner, DragKind::Over, &[], Some((2.0, 2.0)))
        .await?;
    dnd.log_stays(
        LOG,
        &["inner enter", "inner move 2 2"],
        Duration::from_millis(100),
    )
    .await?;
    Ok(())
}

// ---- Drag data ----

/// A drag of an item of a custom type carries exactly that type, and the drop gets it back
/// ("should work with custom data types").
#[browser_test]
pub async fn custom_data_types(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic&items=custom"))
        .await?;
    let source = named(page, "source").await?;
    let dnd = DndActions::new(page);
    dnd.fire_drag_event(&source, DragKind::Start, &[]).await?;
    let transfer = dnd.transfer().await?;
    assert_that!(transfer.data).contains_exactly([("test".to_owned(), "test data".to_owned())]);
    let target = named(page, "target").await?;
    dnd.fire_drag_event(&target, DragKind::Enter, &[]).await?;
    dnd.fire_drag_event(&target, DragKind::Drop, &[]).await?;
    dnd.wait_for_log(
        LOG,
        &[
            "source start",
            "target enter",
            "target drop Move",
            "target exit",
            "target item text test=test data",
        ],
    )
    .await?;
    Ok(())
}

/// Several items of one custom type travel as the first item's data plus all items as JSON, and
/// the drop gets every item back ("should work with multiple items of the same custom type").
#[browser_test]
pub async fn several_items_of_a_custom_type(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic&items=multiple"))
        .await?;
    let source = named(page, "source").await?;
    let dnd = DndActions::new(page);
    dnd.fire_drag_event(&source, DragKind::Start, &[]).await?;
    let transfer = dnd.transfer().await?;
    assert_that!(transfer.data).contains_exactly([
        ("test".to_owned(), "item 1".to_owned()),
        (
            "application/vnd.react-aria.items+json".to_owned(),
            r#"[{"test":"item 1"},{"test":"item 2"}]"#.to_owned(),
        ),
    ]);
    drag_and_drop_from_started(page).await?;
    dnd.wait_for_log(
        LOG,
        &[
            "source start",
            "target enter",
            "target drop Move",
            "target exit",
            "target item text test=item 1",
            "target item text test=item 2",
        ],
    )
    .await?;
    Ok(())
}

/// Enters and drops on the target "Drop here" (the drag started already).
async fn drag_and_drop_from_started(page: &Page<'_>) -> Result<(), Report> {
    let target = named(page, "target").await?;
    let dnd = DndActions::new(page);
    dnd.fire_drag_event(&target, DragKind::Enter, &[]).await?;
    dnd.fire_drag_event(&target, DragKind::Drop, &[]).await?;
    Ok(())
}

/// An item of several types carries each type plus the item as JSON, and the drop gets one item
/// with all types back ("should work with items of multiple types").
#[browser_test]
pub async fn an_item_of_several_types(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic&items=types"))
        .await?;
    let source = named(page, "source").await?;
    let dnd = DndActions::new(page);
    dnd.fire_drag_event(&source, DragKind::Start, &[]).await?;
    let transfer = dnd.transfer().await?;
    assert_that!(transfer.data).contains_exactly([
        ("test".to_owned(), "test data".to_owned()),
        ("text/plain".to_owned(), "test data".to_owned()),
        (
            "application/vnd.react-aria.items+json".to_owned(),
            r#"[{"test":"test data","text/plain":"test data"}]"#.to_owned(),
        ),
    ]);
    drag_and_drop_from_started(page).await?;
    dnd.wait_for_log(
        LOG,
        &[
            "source start",
            "target enter",
            "target drop Move",
            "target exit",
            "target item text test=test data, text/plain=test data",
        ],
    )
    .await?;
    Ok(())
}

/// Several items of several types: native types are joined across items (`text/plain` by
/// newlines), custom ones carry the first item's data, and the drop gets every item back ("should
/// work with multiple items of multiple types").
#[browser_test]
pub async fn several_items_of_several_types(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic&items=multiple-types"))
        .await?;
    let source = named(page, "source").await?;
    let dnd = DndActions::new(page);
    dnd.fire_drag_event(&source, DragKind::Start, &[]).await?;
    let transfer = dnd.transfer().await?;
    assert_that!(transfer.data).contains_exactly([
        ("test".to_owned(), "item 1".to_owned()),
        ("text/plain".to_owned(), "item 1\nitem 2".to_owned()),
        (
            "application/vnd.react-aria.items+json".to_owned(),
            r#"[{"test":"item 1","text/plain":"item 1"},{"test":"item 2","text/plain":"item 2"}]"#
                .to_owned(),
        ),
    ]);
    drag_and_drop_from_started(page).await?;
    dnd.wait_for_log(
        LOG,
        &[
            "source start",
            "target enter",
            "target drop Move",
            "target exit",
            "target item text test=item 1, text/plain=item 1",
            "target item text test=item 2, text/plain=item 2",
        ],
    )
    .await?;
    Ok(())
}

/// Text of several native types dropped from outside the page arrives as one item with every
/// type ("should support dropping multiple native types").
#[browser_test]
pub async fn several_native_types(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic")).await?;
    let dnd = DndActions::new(page);
    dnd.begin_external_drag(&[
        HELLO,
        ExternalData::Text {
            kind: "text/html",
            data: "<p>hello world</p>",
        },
    ])
    .await?;
    drag_and_drop_from_started(page).await?;
    dnd.wait_for_log(
        LOG,
        &[
            "target enter",
            "target drop Move",
            "target exit",
            "target item text text/plain=hello world, text/html=<p>hello world</p>",
        ],
    )
    .await?;
    Ok(())
}

/// A dropped file arrives with its type, name and content ("should support dropping files").
#[browser_test]
pub async fn a_file(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic")).await?;
    let dnd = DndActions::new(page);
    dnd.begin_external_drag(&[ExternalData::File {
        name: "test.txt",
        kind: "text/plain",
        content: "hello world",
    }])
    .await?;
    drag_and_drop_from_started(page).await?;
    dnd.wait_for_log(
        LOG,
        &[
            "target enter",
            "target drop Move",
            "target exit",
            "target item file text/plain test.txt: hello world",
        ],
    )
    .await?;
    Ok(())
}

/// Several dropped files arrive in order ("should support dropping multiple files").
#[browser_test]
pub async fn several_files(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic")).await?;
    let dnd = DndActions::new(page);
    dnd.begin_external_drag(&[
        ExternalData::File {
            name: "test.txt",
            kind: "text/plain",
            content: "hello world",
        },
        ExternalData::File {
            name: "test.html",
            kind: "text/html",
            content: "<p>hello world</p>",
        },
    ])
    .await?;
    drag_and_drop_from_started(page).await?;
    dnd.wait_for_log(
        LOG,
        &[
            "target enter",
            "target drop Move",
            "target exit",
            "target item file text/plain test.txt: hello world",
            "target item file text/html test.html: <p>hello world</p>",
        ],
    )
    .await?;
    Ok(())
}

/// Files and text dropped together arrive as a file item and a text item ("should support
/// dropping both text and files").
#[browser_test]
pub async fn text_and_files(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic")).await?;
    let dnd = DndActions::new(page);
    dnd.begin_external_drag(&[
        ExternalData::File {
            name: "test.txt",
            kind: "text/plain",
            content: "hello world",
        },
        ExternalData::Text {
            kind: "text/html",
            data: "<p>hello world</p>",
        },
    ])
    .await?;
    drag_and_drop_from_started(page).await?;
    dnd.wait_for_log(
        LOG,
        &[
            "target enter",
            "target drop Move",
            "target exit",
            "target item file text/plain test.txt: hello world",
            "target item text text/html=<p>hello world</p>",
        ],
    )
    .await?;
    Ok(())
}

/// A dropped directory lists its files and nested directories ("should support dropping
/// directories").
#[browser_test]
pub async fn a_directory(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic")).await?;
    let dnd = DndActions::new(page);
    dnd.begin_external_drag(&[ExternalData::Directory {
        name: "test",
        entries: vec![
            ExternalData::File {
                name: "test.txt",
                kind: "text/plain",
                content: "hello world",
            },
            ExternalData::Directory {
                name: "nested",
                entries: vec![ExternalData::File {
                    name: "foo.html",
                    kind: "text/html",
                    content: "<p>foo</p>",
                }],
            },
        ],
    }])
    .await?;
    drag_and_drop_from_started(page).await?;
    dnd.wait_for_log(
        LOG,
        &[
            "target enter",
            "target drop Move",
            "target exit",
            "target item directory test: [file text/plain test.txt: hello world, directory nested: [file text/html foo.html: <p>foo</p>]]",
        ],
    )
    .await?;
    Ok(())
}

/// A file of an unknown type arrives as `application/octet-stream` ("should handle unknown file
/// types using a generic mime type").
#[browser_test]
pub async fn a_file_of_an_unknown_type(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic")).await?;
    let dnd = DndActions::new(page);
    dnd.begin_external_drag(&[ExternalData::File {
        name: "test.abc",
        kind: "",
        content: "hello world",
    }])
    .await?;
    drag_and_drop_from_started(page).await?;
    dnd.wait_for_log(
        LOG,
        &[
            "target enter",
            "target drop Move",
            "target exit",
            "target item file application/octet-stream test.abc: hello world",
        ],
    )
    .await?;
    Ok(())
}

// ---- Drop operations ----

/// A drop target's `get_drop_operation` (asked with the drag's types and every operation) decides
/// the drop effect, the drop's operation and the drag's end ("should support getDropOperation to
/// override the default operation").
#[browser_test]
pub async fn get_drop_operation_overrides_the_default(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic&op=copy"))
        .await?;
    let source = named(page, "source").await?;
    let target = named(page, "target").await?;
    let dnd = DndActions::new(page);
    dnd.fire_drag_event(&source, DragKind::Start, &[]).await?;
    let transfer = dnd.transfer().await?;
    assert_that!(transfer.drop_effect).is_equal_to("none");
    dnd.fire_drag_event(&target, DragKind::Enter, &[]).await?;
    let transfer = dnd.transfer().await?;
    assert_that!(transfer.drop_effect).is_equal_to("copy");
    target
        .wait_for_attr("data-drop-target", Some("true"))
        .await?;
    dnd.fire_drag_event(&target, DragKind::Drop, &[]).await?;
    dnd.fire_drag_event(&source, DragKind::End, &[]).await?;
    dnd.wait_for_log(
        LOG,
        &[
            "source start",
            "target operation move,copy,link: text/plain",
            "target enter",
            "target drop Copy",
            "target exit",
            "target item text text/plain=hello world",
            "source end Copy",
        ],
    )
    .await?;
    Ok(())
}

/// A drag source allowing only copy limits the operations the drop target is asked with ("should
/// support getAllowedDropOperations to limit allowed operations").
#[browser_test]
pub async fn allowed_operations_limit_the_drop(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic&op=copy&allowed=copy"))
        .await?;
    let source = named(page, "source").await?;
    let target = named(page, "target").await?;
    let dnd = DndActions::new(page);
    dnd.fire_drag_event(&source, DragKind::Start, &[]).await?;
    dnd.fire_drag_event(&target, DragKind::Enter, &[]).await?;
    let transfer = dnd.transfer().await?;
    assert_that!(transfer.effect_allowed).is_equal_to("copy");
    assert_that!(transfer.drop_effect).is_equal_to("copy");
    dnd.fire_drag_event(&target, DragKind::Drop, &[]).await?;
    dnd.fire_drag_event(&source, DragKind::End, &[]).await?;
    dnd.wait_for_log(
        LOG,
        &[
            "source start",
            "target operation copy: text/plain",
            "target enter",
            "target drop Copy",
            "target exit",
            "target item text text/plain=hello world",
            "source end Copy",
        ],
    )
    .await?;
    Ok(())
}

/// A drop target whose `get_drop_operation` cancels isn't entered and takes no drop effect
/// ("should support canceling drops with getDropOperation").
#[browser_test]
pub async fn get_drop_operation_cancels(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic&op=cancel"))
        .await?;
    let source = named(page, "source").await?;
    let target = named(page, "target").await?;
    let dnd = DndActions::new(page);
    dnd.fire_drag_event(&source, DragKind::Start, &[]).await?;
    dnd.fire_drag_event(&target, DragKind::Enter, &[]).await?;
    let transfer = dnd.transfer().await?;
    assert_that!(transfer.drop_effect).is_equal_to("none");
    target
        .attr_stays("data-drop-target", None, Duration::from_millis(100))
        .await?;
    dnd.log_stays(
        LOG,
        &[
            "source start",
            "target operation move,copy,link: text/plain",
        ],
        Duration::from_millis(100),
    )
    .await?;
    Ok(())
}

/// When the browser narrows `effectAllowed` (a modifier key was pressed), the drop effect follows
/// without exiting the drop target ("should update drop operation if modifier key is pressed").
#[browser_test]
pub async fn effect_allowed_narrowed_by_the_browser(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic")).await?;
    let source = named(page, "source").await?;
    let target = named(page, "target").await?;
    let dnd = DndActions::new(page);
    dnd.fire_drag_event(&source, DragKind::Start, &[]).await?;
    dnd.fire_drag_event_at(&target, DragKind::Enter, &[], Some((1.0, 1.0)))
        .await?;
    let transfer = dnd.transfer().await?;
    assert_that!(transfer.drop_effect).is_equal_to("move");
    target
        .wait_for_attr("data-drop-target", Some("true"))
        .await?;
    dnd.set_effect_allowed("copy").await?;
    dnd.fire_drag_event_at(&target, DragKind::Over, &[], Some((1.0, 1.0)))
        .await?;
    let transfer = dnd.transfer().await?;
    assert_that!(transfer.drop_effect).is_equal_to("copy");
    target
        .attr_stays("data-drop-target", Some("true"), Duration::from_millis(100))
        .await?;
    dnd.log_stays(
        LOG,
        &["source start", "target enter", "target move 1 1"],
        Duration::from_millis(100),
    )
    .await?;
    Ok(())
}

/// When the browser narrows `effectAllowed` to an operation the drop target refuses, the drop
/// effect becomes none and the drag exits the target ("should update drop operation to cancel if
/// modifier key is pressed that is not allowed").
#[browser_test]
pub async fn effect_allowed_narrowed_to_a_refused_operation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic&op=move"))
        .await?;
    let source = named(page, "source").await?;
    let target = named(page, "target").await?;
    let dnd = DndActions::new(page);
    dnd.fire_drag_event(&source, DragKind::Start, &[]).await?;
    dnd.fire_drag_event_at(&target, DragKind::Enter, &[], Some((1.0, 1.0)))
        .await?;
    let transfer = dnd.transfer().await?;
    assert_that!(transfer.drop_effect).is_equal_to("move");
    target
        .wait_for_attr("data-drop-target", Some("true"))
        .await?;
    dnd.set_effect_allowed("copy").await?;
    dnd.fire_drag_event_at(&target, DragKind::Over, &[], Some((1.0, 1.0)))
        .await?;
    let transfer = dnd.transfer().await?;
    assert_that!(transfer.drop_effect).is_equal_to("none");
    target.wait_for_attr("data-drop-target", None).await?;
    dnd.wait_for_log(
        LOG,
        &[
            "source start",
            "target operation move,copy,link: text/plain",
            "target enter",
            "target operation copy: text/plain",
            "target exit",
        ],
    )
    .await?;
    Ok(())
}

/// Where the browser leaves `effectAllowed` as it is, a modifier key (Alt on Windows and Linux)
/// picks the operation itself ("should update drop operation if modifier key is pressed and
/// browser does not update effectAllowed").
#[browser_test]
pub async fn modifier_keys_pick_the_operation(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic")).await?;
    let source = named(page, "source").await?;
    let target = named(page, "target").await?;
    let dnd = DndActions::new(page);
    dnd.fire_drag_event(&source, DragKind::Start, &[]).await?;
    dnd.fire_drag_event_at(&target, DragKind::Enter, &[], Some((1.0, 1.0)))
        .await?;
    let transfer = dnd.transfer().await?;
    assert_that!(transfer.drop_effect).is_equal_to("move");
    dnd.fire_drag_event_at(&target, DragKind::Over, &[Modifier::Alt], Some((1.0, 1.0)))
        .await?;
    let transfer = dnd.transfer().await?;
    assert_that!(transfer.drop_effect).is_equal_to("link");
    target
        .attr_stays("data-drop-target", Some("true"), Duration::from_millis(100))
        .await?;
    dnd.log_stays(
        LOG,
        &["source start", "target enter", "target move 1 1"],
        Duration::from_millis(100),
    )
    .await?;
    Ok(())
}

/// When WebKit reports `effectAllowed` wrongly (always `copyMove`), the drag source's own allowed
/// operations still limit the drop ("should handle when browser does not set effectAllowed
/// properly").
#[browser_test]
pub async fn wrong_effect_allowed_of_the_browser(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic&allowed=copy"))
        .await?;
    let source = named(page, "source").await?;
    let target = named(page, "target").await?;
    let dnd = DndActions::new(page);
    dnd.fire_drag_event(&source, DragKind::Start, &[]).await?;
    let transfer = dnd.transfer().await?;
    assert_that!(transfer.effect_allowed).is_equal_to("copy");
    // WebKit's bug.
    dnd.set_effect_allowed("copyMove").await?;
    dnd.fire_drag_event_at(&target, DragKind::Enter, &[], Some((1.0, 1.0)))
        .await?;
    let transfer = dnd.transfer().await?;
    assert_that!(transfer.drop_effect).is_equal_to("copy");
    target
        .wait_for_attr("data-drop-target", Some("true"))
        .await?;
    dnd.fire_drag_event_at(&target, DragKind::Over, &[Modifier::Alt], Some((1.0, 1.0)))
        .await?;
    let transfer = dnd.transfer().await?;
    assert_that!(transfer.drop_effect).is_equal_to("none");
    target.wait_for_attr("data-drop-target", None).await?;
    dnd.wait_for_log(LOG, &["source start", "target enter", "target exit"])
        .await?;
    Ok(())
}

/// `get_drop_operation` sees the types of dragged files before the drop ("should pass file types
/// to getDropOperation").
#[browser_test]
pub async fn file_types_before_the_drop(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic&op=move"))
        .await?;
    let target = named(page, "target").await?;
    let dnd = DndActions::new(page);
    dnd.begin_external_drag(&[ExternalData::File {
        name: "test.txt",
        kind: "text/plain",
        content: "hello world",
    }])
    .await?;
    dnd.fire_drag_event(&target, DragKind::Enter, &[]).await?;
    dnd.wait_for_log(
        LOG,
        &[
            "target operation move,copy,link: text/plain",
            "target enter",
        ],
    )
    .await?;
    Ok(())
}

/// `get_drop_operation` sees a file of an unknown type as `application/octet-stream` ("should
/// handle unknown file types using a generic mime type").
#[browser_test]
pub async fn unknown_file_types_before_the_drop(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic&op=move"))
        .await?;
    let target = named(page, "target").await?;
    let dnd = DndActions::new(page);
    dnd.begin_external_drag(&[ExternalData::File {
        name: "test.abc",
        kind: "",
        content: "hello world",
    }])
    .await?;
    dnd.fire_drag_event(&target, DragKind::Enter, &[]).await?;
    dnd.wait_for_log(
        LOG,
        &[
            "target operation move,copy,link: application/octet-stream",
            "target enter",
        ],
    )
    .await?;
    Ok(())
}

/// Where the browser doesn't reveal dragged files' types before the drop (Safari), every type
/// counts as available ("should handle when no file types are available").
#[browser_test]
pub async fn no_file_types_before_the_drop(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic&op=move"))
        .await?;
    let target = named(page, "target").await?;
    let dnd = DndActions::new(page);
    dnd.begin_external_drag(&[ExternalData::Text {
        kind: "Files",
        data: "",
    }])
    .await?;
    dnd.fire_drag_event(&target, DragKind::Enter, &[]).await?;
    dnd.wait_for_log(
        LOG,
        &[
            "target operation move,copy,link: test text/plain text/html image/jpeg application/octet-stream application/vnd.react-aria.items+json",
            "target enter",
        ],
    )
    .await?;
    Ok(())
}

/// The JSON of all dragged items isn't one of the types `get_drop_operation` sees ("should not
/// include react-aria custom type in list of types passed to getDropOperation").
#[browser_test]
pub async fn the_items_json_is_no_drag_type(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic&op=move&items=multiple"))
        .await?;
    let source = named(page, "source").await?;
    let target = named(page, "target").await?;
    let dnd = DndActions::new(page);
    dnd.fire_drag_event(&source, DragKind::Start, &[]).await?;
    let transfer = dnd.transfer().await?;
    assert_that!(transfer.types).contains_exactly([
        "test".to_owned(),
        "application/vnd.react-aria.items+json".to_owned(),
    ]);
    dnd.fire_drag_event(&target, DragKind::Enter, &[]).await?;
    dnd.wait_for_log(
        LOG,
        &[
            "source start",
            "target operation move,copy,link: test",
            "target enter",
        ],
    )
    .await?;
    Ok(())
}

// ---- Drag preview ----

/// A drag shows the preview element under the pointer at the pointer's offset in the drag source
/// ("should support rendering a custom drag preview").
#[browser_test]
pub async fn a_drag_preview(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic&preview=100x30"))
        .await?;
    let source = named(page, "source").await?;
    let dnd = DndActions::new(page);
    dnd.fire_drag_event_at(&source, DragKind::Start, &[], Some((5.0, 5.0)))
        .await?;
    let transfer = dnd.transfer().await?;
    assert_that!(transfer.drag_image).is_equal_to(Some(DragImage {
        text: "Drag preview".to_owned(),
        x: 5.0,
        y: 5.0,
    }));
    Ok(())
}

/// A preview smaller than the pointer's offset in the drag source is centered under the pointer
/// ("should center the drag image under the mouse if the size is smaller than the original
/// target").
#[browser_test]
pub async fn a_small_drag_preview_is_centered(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic&preview=20x20"))
        .await?;
    let source = named(page, "source").await?;
    let dnd = DndActions::new(page);
    dnd.fire_drag_event_at(&source, DragKind::Start, &[], Some((30.0, 15.0)))
        .await?;
    let transfer = dnd.transfer().await?;
    assert_that!(transfer.drag_image).is_equal_to(Some(DragImage {
        text: "Drag preview".to_owned(),
        x: 10.0,
        y: 10.0,
    }));
    Ok(())
}

/// The preview's own offset replaces the pointer's ("should use the offset returned from
/// renderPreview").
#[browser_test]
pub async fn the_drag_preview_offset(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!(
        "{PATH}?only=basic&preview=20x20&preview-offset=12,15"
    ))
    .await?;
    let source = named(page, "source").await?;
    let dnd = DndActions::new(page);
    dnd.fire_drag_event_at(&source, DragKind::Start, &[], Some((10.0, 10.0)))
        .await?;
    let transfer = dnd.transfer().await?;
    assert_that!(transfer.drag_image).is_equal_to(Some(DragImage {
        text: "Drag preview".to_owned(),
        x: 12.0,
        y: 15.0,
    }));
    Ok(())
}

/// The preview's offset is clamped to the preview's bounds ("should clamp the offset returned
/// from renderPreview to the preview bounds").
#[browser_test]
pub async fn the_drag_preview_offset_is_clamped(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!(
        "{PATH}?only=basic&preview=20x20&preview-offset=50,-10"
    ))
    .await?;
    let source = named(page, "source").await?;
    let dnd = DndActions::new(page);
    dnd.fire_drag_event_at(&source, DragKind::Start, &[], Some((0.0, 0.0)))
        .await?;
    let transfer = dnd.transfer().await?;
    assert_that!(transfer.drag_image).is_equal_to(Some(DragImage {
        text: "Drag preview".to_owned(),
        x: 20.0,
        y: 0.0,
    }));
    Ok(())
}

// ---- The drag source's lifetime ----

/// A drag source removed during its drag ends the drag once: when it unmounts, not again for the
/// `drag` and `dragend` the browser still sends to the removed element.
#[browser_test]
pub async fn a_removed_drag_source_ends_once(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=removable")).await?;
    let source = named(page, "source").await?;
    let dnd = DndActions::new(page);
    dnd.fire_drag_event(&source, DragKind::Start, &[]).await?;
    source.wait_for_attr("data-dragging", Some("true")).await?;
    page.element(role(AriaRole::Button).text("Remove source"))
        .await?
        .click()
        .await?;
    page.wait_for_count("[data-name='source']", 0).await?;
    dnd.fire_at_drag_source(DragKind::Drag).await?;
    dnd.fire_at_drag_source(DragKind::End).await?;
    dnd.wait_for_log(LOG, &["source start", "source end Cancel"])
        .await?;
    dnd.log_stays(
        LOG,
        &["source start", "source end Cancel"],
        Duration::from_millis(100),
    )
    .await?;
    Ok(())
}

/// A drag that ends before the frame after its start never marks the drag source as dragging.
#[browser_test]
pub async fn a_drag_ending_in_its_first_frame(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic")).await?;
    let source = named(page, "source").await?;
    let dnd = DndActions::new(page);
    dnd.fire_drag_events(&source, &[DragKind::Start, DragKind::End])
        .await?;
    dnd.wait_for_log(LOG, &["source start", "source end Cancel"])
        .await?;
    source
        .attr_stays("data-dragging", None, Duration::from_millis(100))
        .await?;
    Ok(())
}

// ---- Keyboard ----

/// A keyboard drag of an item of a custom type drops exactly that item ("should work with custom
/// data types").
#[browser_test]
pub async fn keyboard_custom_data_types(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic&items=custom"))
        .await?;
    start_keyboard_drag(page).await?;
    page.wait_for_focus(&named(page, "target").await?).await?;
    page.send_keys(Key::Enter).await?;
    DndActions::new(page)
        .wait_for_log(
            LOG,
            &[
                "source start",
                "target enter",
                "target drop Move",
                "source end Move",
                "target exit",
                "target item text test=test data",
            ],
        )
        .await?;
    Ok(())
}

/// A keyboard drag of several items of a custom type drops every item ("should work with
/// multiple items of the same custom type").
#[browser_test]
pub async fn keyboard_several_items_of_a_custom_type(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic&items=multiple"))
        .await?;
    start_keyboard_drag(page).await?;
    page.wait_for_focus(&named(page, "target").await?).await?;
    page.send_keys(Key::Enter).await?;
    DndActions::new(page)
        .wait_for_log(
            LOG,
            &[
                "source start",
                "target enter",
                "target drop Move",
                "source end Move",
                "target exit",
                "target item text test=item 1",
                "target item text test=item 2",
            ],
        )
        .await?;
    Ok(())
}

/// A keyboard drag of an item of several types drops it with every type ("should work with items
/// of multiple types").
#[browser_test]
pub async fn keyboard_an_item_of_several_types(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic&items=types"))
        .await?;
    start_keyboard_drag(page).await?;
    page.wait_for_focus(&named(page, "target").await?).await?;
    page.send_keys(Key::Enter).await?;
    DndActions::new(page)
        .wait_for_log(
            LOG,
            &[
                "source start",
                "target enter",
                "target drop Move",
                "source end Move",
                "target exit",
                "target item text test=test data, text/plain=test data",
            ],
        )
        .await?;
    Ok(())
}

/// A keyboard drag of several items of several types drops every item with every type ("should
/// work with multiple items of multiple types").
#[browser_test]
pub async fn keyboard_several_items_of_several_types(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic&items=multiple-types"))
        .await?;
    start_keyboard_drag(page).await?;
    page.wait_for_focus(&named(page, "target").await?).await?;
    page.send_keys(Key::Enter).await?;
    DndActions::new(page)
        .wait_for_log(
            LOG,
            &[
                "source start",
                "target enter",
                "target drop Move",
                "source end Move",
                "target exit",
                "target item text test=item 1, text/plain=item 1",
                "target item text test=item 2, text/plain=item 2",
            ],
        )
        .await?;
    Ok(())
}

/// A keyboard drag starts on the drop target nearest to the drag source, also inside another
/// drop target; Tab moves to the outer one and Enter drops only there ("should support nested
/// drop targets").
#[browser_test]
pub async fn keyboard_nested_drop_targets(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=nested-drop")).await?;
    let source = named(page, "source").await?;
    let outer = named(page, "outer").await?;
    let inner = named(page, "inner").await?;
    start_keyboard_drag(page).await?;
    page.wait_for_focus(&inner).await?;
    source.wait_for_attr("data-dragging", Some("true")).await?;
    inner
        .wait_for_attr("data-drop-target", Some("true"))
        .await?;
    assert_that!(outer)
        .attribute("data-drop-target")
        .await
        .is_none();
    assert_that!(inner)
        .accessible_description()
        .await
        .is_equal_to("Press Enter to drop. Press Escape to cancel drag.");
    let dnd = DndActions::new(page);
    dnd.wait_for_log(LOG, &["source start", "inner enter"])
        .await?;

    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&outer).await?;
    outer
        .wait_for_attr("data-drop-target", Some("true"))
        .await?;
    inner.wait_for_attr("data-drop-target", None).await?;
    dnd.wait_for_log(
        LOG,
        &["source start", "inner enter", "inner exit", "outer enter"],
    )
    .await?;

    page.send_keys(Key::Enter).await?;
    dnd.wait_for_log(
        LOG,
        &[
            "source start",
            "inner enter",
            "inner exit",
            "outer enter",
            "outer drop Move",
            "source end Move",
            "outer exit",
            "outer item text text/plain=hello world",
        ],
    )
    .await?;
    page.wait_for_focus(&outer).await?;
    source.wait_for_attr("data-dragging", None).await?;
    outer.wait_for_attr("data-drop-target", None).await?;
    assert_that!(outer)
        .attribute("aria-describedby")
        .await
        .is_none();
    assert_that!(inner)
        .attribute("aria-describedby")
        .await
        .is_none();
    Ok(())
}

/// Two Enter presses on the drag source before the drag's first frame start one keyboard drag.
#[browser_test]
pub async fn a_second_enter_starts_no_second_drag(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(&format!("{PATH}?only=basic")).await?;
    let source = named(page, "source").await?;
    page.element(role(AriaRole::Button).text("Before"))
        .await?
        .click()
        .await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&source).await?;
    source
        .dispatch_both(
            SyntheticEvent::keyboard(KeyKind::Up, "Enter"),
            SyntheticEvent::keyboard(KeyKind::Up, "Enter"),
        )
        .await?;
    page.wait_for_focus(&named(page, "target").await?).await?;
    DndActions::new(page)
        .log_stays(
            LOG,
            &["source start", "target enter"],
            Duration::from_millis(100),
        )
        .await?;
    page.send_keys(Key::Escape).await?;
    DndActions::new(page)
        .wait_for_log(
            LOG,
            &[
                "source start",
                "target enter",
                "target exit",
                "source end Cancel",
            ],
        )
        .await?;
    Ok(())
}

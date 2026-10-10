use std::collections::HashSet;

use leptonic::{
    CapturedElement, IntoAttrs, Orientation, flag,
    hooks::{
        button::{UseButtonInput, use_button},
        collections::{
            CollectionOptions, Key, ListLayout, ListState, SelectionBehavior, SelectionMode,
            SelectionOptions, UseListCollectionInput, UseListStateInput, use_list_collection,
            use_list_state,
        },
        dnd::{
            DragItem, DragPreview, DraggableCollectionEndEvent, DraggableCollectionMoveEvent,
            DraggableCollectionStartEvent, DraggableCollectionState, DropEvent, DropItem,
            UseDraggableCollectionInput, UseDraggableCollectionStateInput, UseDraggableItemInput,
            UseDraggableItemReturn, UseDropInput, UseDropReturn, use_draggable_collection,
            use_draggable_collection_state, use_draggable_item, use_drop,
        },
        gridlist::{
            FocusMode, GridListData, KeyboardNavigationBehavior, UseGridListInput,
            UseGridListItemInput, UseGridListItemReturn, UseGridListReturn, use_grid_list,
            use_grid_list_item,
        },
        listbox::{
            ListBoxData, UseListBoxInput, UseListBoxReturn, UseOptionInput, UseOptionReturn,
            use_listbox, use_option,
        },
    },
};
use leptos::{context::Provider, html, prelude::*};
use leptos_router::hooks::use_query_map;
use send_wrapper::SendWrapper;

use crate::pages::Section;

/// An item of the draggable collections: Foo and Bar are folders, Baz an item.
#[derive(Debug, Clone, PartialEq)]
struct Entry {
    id: &'static str,
    kind: &'static str,
    text: &'static str,
}

const ENTRIES: [Entry; 3] = [
    Entry {
        id: "foo",
        kind: "folder",
        text: "Foo",
    },
    Entry {
        id: "bar",
        kind: "folder",
        text: "Bar",
    },
    Entry {
        id: "baz",
        kind: "item",
        text: "Baz",
    },
];

/// Collections whose items are dragged (react-aria's `useDraggableCollection.test.js` setups) and
/// a drop target "Drop here". `#test-dnd-draggable-log` logs `start <keys>`, `move <keys>`,
/// `end <operation> <keys> internal=<bool>` (keys in collection order), the drop target's
/// `drop <operation>` and one `item <type>=<data>, ...` per dropped item, and the list box's
/// `action <key>`. A drag that ends as a move removes the dragged items.
///
/// Sections (`?only=<name>`):
/// - `grid`: the grid "Draggable list" (multiple selection), each row's data `<kind>` and
///   `text/plain` = its text, with a drag button ("Drag Bar", ...); drags show the preview
///   "<text> <count>" (the count only with several items).
/// - `listbox`: the list box "Test" (multiple selection) without drag buttons, data `text/plain`
///   = the text. Query `action`: its options have an action (replace selection behavior; keyboard
///   drags start with Alt + Enter).
#[component]
pub fn PageHookDndDraggableCollection() -> impl IntoView {
    let action = use_query_map().with_untracked(|q| q.get("action").is_some());
    let log = RwSignal::new(Vec::<String>::new());
    view! {
        <div id="test-page-hook-dnd-draggable-collection">
            <h1>"Draggable collections"</h1>
            <button>"Before"</button>
            <Section name="grid">
                <DraggableGrid log />
            </Section>
            <Section name="listbox">
                <DraggableListBox log action />
            </Section>
            <Target log />
            <ol id="test-dnd-draggable-log">
                {move || log.get().into_iter().map(|e| view! { <li>{e}</li> }).collect_view()}
            </ol>
        </div>
    }
}

/// The entries' keys in collection order.
fn ordered(keys: &HashSet<Key>) -> String {
    ENTRIES
        .iter()
        .filter(|entry| keys.contains(&Key::from(entry.id)))
        .map(|entry| entry.id)
        .collect::<Vec<_>>()
        .join(",")
}

/// The list state and drag state of a draggable collection of [`ENTRIES`], logging to `log`;
/// a move removes the dragged entries.
fn draggable_state(
    log: RwSignal<Vec<String>>,
    data: fn(&Entry) -> DragItem,
    selection_behavior: SelectionBehavior,
    preview: Option<Callback<Vec<DragItem>, Option<DragPreview>>>,
) -> (RwSignal<Vec<Entry>>, ListState, DraggableCollectionState) {
    let push = move |entry: String| log.update(|l| l.push(entry));
    let entries = RwSignal::new(ENTRIES.to_vec());
    let collection = use_list_collection(UseListCollectionInput {
        items: entries.into(),
        key: |entry: &Entry| Key::from(entry.id),
        text_value: |entry: &Entry| entry.text.to_owned(),
    });
    let list = use_list_state(UseListStateInput {
        collection,
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            selection_behavior: Signal::stored(selection_behavior),
            ..SelectionOptions::default()
        },
    });
    let state = use_draggable_collection_state(UseDraggableCollectionStateInput {
        list,
        get_items: Callback::new(move |keys: HashSet<Key>| {
            ENTRIES
                .iter()
                .filter(|entry| keys.contains(&Key::from(entry.id)))
                .map(data)
                .collect()
        }),
        preview,
        allowed_drop_operations: None,
        on_drag_start: Some(Callback::new(move |e: DraggableCollectionStartEvent| {
            push(format!("start {}", ordered(&e.keys)));
        })),
        on_drag_move: Some(Callback::new(move |e: DraggableCollectionMoveEvent| {
            push(format!("move {}", ordered(&e.keys)));
        })),
        on_drag_end: Some(Callback::new(move |e: DraggableCollectionEndEvent| {
            push(format!(
                "end {:?} {} internal={}",
                e.drop_operation,
                ordered(&e.keys),
                e.is_internal
            ));
            if e.drop_operation == leptonic::hooks::dnd::DropOperation::Move {
                entries.update(|entries| {
                    entries.retain(|entry| !e.keys.contains(&Key::from(entry.id)));
                });
            }
        })),
        is_disabled: Signal::stored(false),
    });
    (entries, list, state)
}

/// What the grid's rows need.
#[derive(Clone)]
struct GridContext {
    list: GridListData,
    state: DraggableCollectionState,
}

#[component]
fn DraggableGrid(log: RwSignal<Vec<String>>) -> impl IntoView {
    let preview_ref = NodeRef::<html::Div>::new();
    let state_slot = StoredValue::new(None::<DraggableCollectionState>);
    let preview = Callback::new(move |items: Vec<DragItem>| {
        let element = preview_ref.get_untracked()?;
        let text = state_slot
            .get_value()
            .and_then(|state| state.dragged_key.get_untracked())
            .and_then(|key| ENTRIES.iter().find(|entry| Key::from(entry.id) == key))
            .map_or("", |entry| entry.text);
        let count = if items.len() > 1 {
            format!(" {}", items.len())
        } else {
            String::new()
        };
        element.set_text_content(Some(&format!("{text}{count}")));
        Some(DragPreview {
            element: SendWrapper::new(element.into()),
            offset: None,
        })
    });
    let (entries, list, state) = draggable_state(
        log,
        |entry| {
            DragItem::new()
                .with(entry.kind, entry.text)
                .with("text/plain", entry.text)
        },
        SelectionBehavior::Toggle,
        Some(preview),
    );
    state_slot.set_value(Some(state));
    let element = CapturedElement::new();
    use_draggable_collection(UseDraggableCollectionInput { state, element });
    let UseGridListReturn { props, data } = use_grid_list(UseGridListInput {
        aria_label: "Draggable list".into(),
        state: list,
        element,
        id: None,
        aria_labelledby: Signal::stored(None),
        layout: ListLayout::Stack,
        orientation: Orientation::Vertical.into(),
        keyboard_delegate: None,
        options: CollectionOptions::default(),
        keyboard_navigation_behavior: KeyboardNavigationBehavior::default(),
        should_select_on_press_up: false,
        on_action: None,
        tree: None,
    });
    view! {
        <Provider value=GridContext { list: data, state }>
            <div {..props.into_attrs()}>
                <For
                    each=move || entries.get()
                    key=|entry| entry.id
                    children=move |entry: Entry| view! { <GridRow entry /> }
                />
            </div>
        </Provider>
        <div
            node_ref=preview_ref
            class="test-dnd-draggable-preview"
            style="position: fixed; left: -1000px; top: 0; width: 100px; height: 20px"
        ></div>
    }
}

#[component]
fn GridRow(entry: Entry) -> impl IntoView {
    let GridContext { list, state } = expect_context::<GridContext>();
    let key = Key::from(entry.id);
    let UseGridListItemReturn {
        row_props,
        grid_cell_props,
        ..
    } = use_grid_list_item(UseGridListItemInput {
        list,
        key: key.clone(),
        focus_mode: FocusMode::Row,
        allows_arrow_navigation: false,
        on_context_menu: None,
    });
    let UseDraggableItemReturn {
        drag_props,
        drag_button,
        drag_button_label,
        is_dragging,
    } = use_draggable_item(UseDraggableItemInput {
        state,
        key,
        has_drag_button: true,
        has_action: false,
    });
    let button = use_button(UseButtonInput {
        aria_label: MaybeProp::derive(move || Some(drag_button_label.get())),
        ..drag_button
    });
    let (button_attrs, button_styles) = button.props.into_parts();
    let (row_attrs, row_styles) = row_props.into_parts();
    view! {
        <div
            {..row_attrs}
            {..drag_props.into_attrs()}
            style=row_styles
            data-dragging=flag(is_dragging)
        >
            <div {..grid_cell_props.into_attrs()}>
                <span>{entry.text}</span>
                <button {..button_attrs} style=button_styles>
                    "≡"
                </button>
            </div>
        </div>
    }
}

/// What the list box's options need.
#[derive(Clone)]
struct ListBoxContext {
    list: ListBoxData,
    state: DraggableCollectionState,
    action: bool,
}

#[component]
fn DraggableListBox(log: RwSignal<Vec<String>>, action: bool) -> impl IntoView {
    let push = move |entry: String| log.update(|l| l.push(entry));
    let (entries, list, state) = draggable_state(
        log,
        |entry| DragItem::text(entry.text),
        if action {
            SelectionBehavior::Replace
        } else {
            SelectionBehavior::Toggle
        },
        None,
    );
    let element = CapturedElement::new();
    use_draggable_collection(UseDraggableCollectionInput { state, element });
    let UseListBoxReturn { props, data } = use_listbox(UseListBoxInput {
        state: list,
        element,
        id: None,
        aria_label: "Test".into(),
        aria_labelledby: Signal::stored(None),
        orientation: Signal::stored(Orientation::Vertical),
        layout: ListLayout::Stack,
        keyboard_delegate: None,
        layout_delegate: None,
        is_virtualized: false,
        options: CollectionOptions::default(),
        should_select_on_press_up: false,
        should_focus_on_hover: false,
        on_action: action.then(|| Callback::new(move |key: Key| push(format!("action {key}")))),
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });
    view! {
        <Provider value=ListBoxContext {
            list: data,
            state,
            action,
        }>
            <div {..props.into_attrs()}>
                <For
                    each=move || entries.get()
                    key=|entry| entry.id
                    children=move |entry: Entry| view! { <ListBoxOption entry /> }
                />
            </div>
        </Provider>
    }
}

#[component]
fn ListBoxOption(entry: Entry) -> impl IntoView {
    let ListBoxContext {
        list,
        state,
        action,
    } = expect_context::<ListBoxContext>();
    let key = Key::from(entry.id);
    let UseOptionReturn { props, .. } = use_option(UseOptionInput {
        list,
        key: key.clone(),
        on_context_menu: None,
    });
    let UseDraggableItemReturn {
        mut drag_props,
        is_dragging,
        ..
    } = use_draggable_item(UseDraggableItemInput {
        state,
        key,
        has_drag_button: false,
        has_action: action,
    });
    // Both describe the option: join the ids (react-aria merges them).
    let (mut option_props, option_styles) = props.into_inner();
    let option_description = option_props.aria_describedby;
    let drag_description = drag_props.aria_describedby;
    let description = Signal::derive(move || {
        let ids: Vec<String> = [option_description.get(), drag_description.get()]
            .into_iter()
            .flatten()
            .collect();
        (!ids.is_empty()).then(|| ids.join(" "))
    });
    option_props.aria_describedby = description;
    drag_props.aria_describedby = description;
    view! {
        <div
            {..option_props.into_attrs()}
            {..drag_props.into_attrs()}
            style=option_styles
            data-dragging=flag(is_dragging)
        >
            {entry.text}
        </div>
    }
}

/// The drop target "Drop here", logging drops.
#[component]
fn Target(log: RwSignal<Vec<String>>) -> impl IntoView {
    let push = move |entry: String| log.update(|l| l.push(entry));
    let element = CapturedElement::new();
    let UseDropReturn { drop_props, .. } = use_drop(UseDropInput {
        element,
        get_drop_operation: None,
        get_drop_operation_for_point: None,
        on_drop_enter: None,
        on_drop_move: None,
        on_drop_activate: None,
        on_drop_exit: None,
        on_drop: Some(Callback::new(move |e: DropEvent| {
            push(format!("drop {:?}", e.drop_operation));
            for item in e.items {
                if let DropItem::Text(text) = item {
                    let entries: Vec<String> = text
                        .types()
                        .map(|kind| format!("{kind}={}", text.get_text(kind).unwrap_or_default()))
                        .collect();
                    push(format!("item {}", entries.join(", ")));
                }
            }
        })),
        has_drop_button: false,
        is_disabled: Signal::stored(false),
    });
    view! {
        <div role="button" tabindex="0" {..drop_props.into_attrs()}>
            "Drop here"
        </div>
    }
}

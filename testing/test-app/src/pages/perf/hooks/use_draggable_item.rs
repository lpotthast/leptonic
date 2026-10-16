//! The hook `use_draggable_item` in its collection, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_draggable_item.rs`): [`DraggableListPage`], a grid list
//! (`#perf-list`, "Items", multiple selection) of [`SMALL`] or [`LARGE`] rows ("Item 0", ...; keys
//! `item-0`, ...; `data-row` holds a row's number), each draggable (`use_draggable_item`, dragging the text of every dragged
//! row), with the collection's drag state (`use_draggable_collection_state`) and
//! `use_draggable_collection`. The fixtures of those two hooks are the same page.
//!
//! The list is mounted at first (with [`SMALL`] rows), between a plain button (`#perf-before`)
//! and a `use_drop` target (`#perf-drop`, "Drop here") followed by another plain button
//! (`#perf-after`). The page announces an empty message once mounted, so that the live
//! announcer (which keyboard drags announce through) exists before the test measures.
//!
//! Its controls ([`Control`]) mount and unmount the list, switch its size (`small`, `large`; while
//! unmounted), select all rows and none (`select-all`, `clear-selection`), disable and enable
//! dragging (`disable`, `enable`: the drag state's `is_disabled`), and read the drag state
//! (`drag-count`: how many rows a drag of the first one drags; `items`: how many drag items it
//! carries; `dragging`: how many rows are being dragged) and what the callbacks saw (`starts`,
//! `ends`, `drops`, `last-internal`: whether the last drag ended inside the list, 1 or 0).

use std::collections::HashSet;

use leptonic::{
    CapturedElement, IntoAttrs, Orientation, announce_polite, flag,
    hooks::{
        collections::{
            CollectionOptions, Key, ListLayout, ListState, SelectionMode, SelectionOptions,
            UseListCollectionInput, UseListStateInput, use_list_collection, use_list_state,
        },
        dnd::{
            DragItem, DraggableCollectionEndEvent, DraggableCollectionState,
            UseDraggableCollectionInput, UseDraggableCollectionStateInput, UseDraggableItemInput,
            UseDraggableItemReturn, UseDropInput, UseDropReturn, use_draggable_collection,
            use_draggable_collection_state, use_draggable_item, use_drop,
        },
        gridlist::{
            FocusMode, GridListData, KeyboardNavigationBehavior, UseGridListInput,
            UseGridListItemInput, UseGridListItemReturn, UseGridListReturn, use_grid_list,
            use_grid_list_item,
        },
    },
};
use leptos::{context::Provider, prelude::*};

use crate::pages::perf::{Control, PerfControls};

/// The rows of the small list.
pub const SMALL: u32 = 10;
/// The rows of the large list.
pub const LARGE: u32 = 1_000;

#[component]
pub fn PagePerf() -> impl IntoView {
    view! { <DraggableListPage title="use_draggable_item performance" /> }
}

/// What the list's states and callbacks share with the page.
#[derive(Clone, Copy)]
struct Shared {
    disabled: RwSignal<bool>,
    /// The mounted list's states, for the controls.
    states: StoredValue<Option<(ListState, DraggableCollectionState)>>,
    starts: StoredValue<u32>,
    ends: StoredValue<u32>,
    last_internal: StoredValue<bool>,
}

/// The page of the draggable list (see the module documentation), titled `title`.
#[component]
pub fn DraggableListPage(title: &'static str) -> impl IntoView {
    let mounted = RwSignal::new(true);
    let count = RwSignal::new(SMALL);
    let shared = Shared {
        disabled: RwSignal::new(false),
        states: StoredValue::new(None),
        starts: StoredValue::new(0),
        ends: StoredValue::new(0),
        last_internal: StoredValue::new(false),
    };
    let drops = StoredValue::new(0_u32);
    let with_list = move |f: fn(ListState)| {
        if let Some((list, _)) = shared.states.get_value() {
            f(list);
        }
    };
    let read_state = move |f: fn(DraggableCollectionState) -> usize| {
        shared.states.get_value().map_or(-1.0, |(_, state)| {
            u32::try_from(untrack(|| f(state))).map_or(-1.0, f64::from)
        })
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || count.set(SMALL)),
        Control::action("large", move || count.set(LARGE)),
        Control::action("select-all", move || {
            with_list(|list| list.selection.select_all());
        }),
        Control::action("clear-selection", move || {
            with_list(|list| list.selection.clear_selection());
        }),
        Control::action("disable", move || shared.disabled.set(true)),
        Control::action("enable", move || shared.disabled.set(false)),
        Control::read("drag-count", move || {
            read_state(|state| state.drag_count(&Key::from("item-0")))
        }),
        Control::read("items", move || {
            read_state(|state| state.items(&Key::from("item-0")).len())
        }),
        Control::read("dragging", move || {
            read_state(|state| state.dragging_keys.with_untracked(HashSet::len))
        }),
        Control::read("starts", move || f64::from(shared.starts.get_value())),
        Control::read("ends", move || f64::from(shared.ends.get_value())),
        Control::read("drops", move || f64::from(drops.get_value())),
        Control::read("last-internal", move || {
            if shared.last_internal.get_value() {
                1.0
            } else {
                0.0
            }
        }),
    ];
    Effect::new(|| announce_polite(""));

    let element = CapturedElement::new();
    let UseDropReturn {
        drop_props,
        is_drop_target,
        ..
    } = use_drop(UseDropInput {
        element,
        get_drop_operation: None,
        get_drop_operation_for_point: None,
        on_drop_enter: None,
        on_drop_move: None,
        on_drop_activate: None,
        on_drop_exit: None,
        on_drop: Some(Callback::new(move |_| {
            drops.update_value(|drops| *drops += 1);
        })),
        has_drop_button: false,
        is_disabled: Signal::stored(false),
    });
    view! {
        <h1>{title}</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <DraggableList count=count.get_untracked() shared />
        </Show>
        <div
            id="perf-drop"
            role="button"
            tabindex="0"
            {..drop_props.into_attrs()}
            data-drop-target=flag(is_drop_target)
        >
            "Drop here"
        </div>
        <button id="perf-after">"After"</button>
    }
}

/// What the rows need.
#[derive(Clone)]
struct RowContext {
    list: GridListData,
    state: DraggableCollectionState,
}

/// The grid list of `count` draggable rows.
#[component]
fn DraggableList(count: u32, shared: Shared) -> impl IntoView {
    let collection = use_list_collection(UseListCollectionInput {
        items: Signal::stored((0..count).collect::<Vec<u32>>()),
        key: |i| Key::from(format!("item-{i}")),
        text_value: |i| format!("Item {i}"),
    });
    let list = use_list_state(UseListStateInput {
        collection,
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            ..SelectionOptions::default()
        },
    });
    let state = use_draggable_collection_state(UseDraggableCollectionStateInput {
        list,
        get_items: Callback::new(|keys: HashSet<Key>| {
            keys.iter()
                .map(|key| DragItem::text(key.to_string()))
                .collect()
        }),
        preview: None,
        allowed_drop_operations: None,
        on_drag_start: Some(Callback::new(move |_| {
            shared.starts.update_value(|starts| *starts += 1);
        })),
        on_drag_move: None,
        on_drag_end: Some(Callback::new(move |e: DraggableCollectionEndEvent| {
            shared.ends.update_value(|ends| *ends += 1);
            shared.last_internal.set_value(e.is_internal);
        })),
        is_disabled: shared.disabled.into(),
    });
    shared.states.set_value(Some((list, state)));
    on_cleanup(move || shared.states.set_value(None));
    let element = CapturedElement::new();
    use_draggable_collection(UseDraggableCollectionInput { state, element });
    let UseGridListReturn { props, data } = use_grid_list(UseGridListInput {
        aria_label: "Items".into(),
        state: list,
        element,
        id: Some("perf-list".to_owned()),
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
        <Provider value=RowContext { list: data, state }>
            <div {..props.into_attrs()}>
                {(0..count).map(|index| view! { <Row index /> }).collect_view()}
            </div>
        </Provider>
    }
}

/// A draggable row.
#[component]
fn Row(index: u32) -> impl IntoView {
    let RowContext { list, state } = expect_context::<RowContext>();
    let key = Key::from(format!("item-{index}"));
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
        is_dragging,
        ..
    } = use_draggable_item(UseDraggableItemInput {
        state,
        key,
        has_drag_button: false,
        has_action: false,
    });
    let (row_attrs, row_styles) = row_props.into_parts();
    view! {
        <div
            {..row_attrs}
            {..drag_props.into_attrs()}
            style=row_styles
            data-row=index
            data-dragging=flag(is_dragging)
        >
            <div {..grid_cell_props.into_attrs()}>{format!("Item {index}")}</div>
        </div>
    }
}

//! The hook `use_droppable_collection_state` with its collection, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_droppable_collection_state.rs`): the drop state of a grid list (`#perf-list`, "Items", 150px high, scrolling) of [`SMALL`] or [`LARGE`] rows, 30px
//! high ("Item 0", ...; keys `item-0`, ...) taking drops on the list itself (a drop indicator for
//! the root first), between its rows (a drop indicator before each row and one after the last) and
//! on its rows (`use_droppable_item`). Drops between rows insert the dropped text as a row.
//!
//! Each drop indicator (`role=button`) has `data-target` (`root`, `before-item-<i>`,
//! `after-item-<i>`) and `data-drop-target` while it is the drop target; each row has `data-row`
//! (its number) and `data-drop-target` while it is. The page
//! announces an empty message once mounted, so that the live announcer (which keyboard drags
//! announce through) exists before the test measures. Mounted at first, with [`SMALL`] rows.
//!
//! Its controls ([`Control`]) mount and unmount the list, switch its size (`small`, `large`; while
//! unmounted), set the drop target through the state (`target-root`, `target-first`: on the first
//! row, `target-between`: between the first two rows, `target-none`), and disable and enable
//! dropping (`disable`, `enable`: the state's `is_disabled`).

use std::sync::Arc;

use leptonic::{
    CapturedElement, Orientation, announce_polite, flag,
    hooks::{
        collections::{
            CollectionOptions, Key, ListLayout, SelectionMode, SelectionOptions,
            UseListCollectionInput, UseListKeyboardDelegateInput, UseListStateInput,
            use_list_collection, use_list_keyboard_delegate, use_list_state,
        },
        dnd::{
            DropItem, DropPosition, DropTarget, DroppableCollectionData,
            DroppableCollectionInsertDropEvent, DroppableCollectionOptions,
            DroppableCollectionState, ListDropTargetDelegate, UseDropIndicatorInput,
            UseDropIndicatorReturn, UseDroppableCollectionInput, UseDroppableCollectionReturn,
            UseDroppableCollectionStateInput, UseDroppableItemInput, UseDroppableItemReturn,
            use_drop_indicator, use_droppable_collection, use_droppable_collection_state,
            use_droppable_item,
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
const SMALL: u32 = 10;
/// The rows of the large list.
const LARGE: u32 = 1_000;

/// A row: its key's number and its text.
#[derive(Debug, Clone, PartialEq)]
struct Item {
    id: u32,
    text: String,
}

/// What the list's state and callbacks share with the page.
#[derive(Clone, Copy)]
struct Shared {
    disabled: RwSignal<bool>,
    /// The mounted list's drop state, for the controls.
    state: StoredValue<Option<DroppableCollectionState>>,
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let count = RwSignal::new(SMALL);
    let shared = Shared {
        disabled: RwSignal::new(false),
        state: StoredValue::new(None),
    };
    let set_target = move |target: Option<DropTarget>| {
        if let Some(state) = shared.state.get_value() {
            state.set_target(target);
        }
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || count.set(SMALL)),
        Control::action("large", move || count.set(LARGE)),
        Control::action("target-root", move || set_target(Some(DropTarget::Root))),
        Control::action("target-first", move || {
            set_target(Some(DropTarget::item("item-0", DropPosition::On)));
        }),
        Control::action("target-between", move || {
            set_target(Some(DropTarget::item("item-1", DropPosition::Before)));
        }),
        Control::action("target-none", move || set_target(None)),
        Control::action("disable", move || shared.disabled.set(true)),
        Control::action("enable", move || shared.disabled.set(false)),
    ];
    Effect::new(|| announce_polite(""));
    view! {
        <h1>"use_droppable_collection_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <DroppableList count=count.get_untracked() shared />
        </Show>
    }
}

/// What the rows and drop indicators need.
#[derive(Clone)]
struct ListContext {
    list: GridListData,
    drop: DroppableCollectionData,
}

/// The text of the first dropped text item.
fn dropped_text(items: &[DropItem]) -> String {
    items
        .iter()
        .find_map(|item| match item {
            DropItem::Text(text) => text.get_text("text/plain").map(ToOwned::to_owned),
            _ => None,
        })
        .unwrap_or_default()
}

/// The grid list of `count` rows taking drops.
#[component]
fn DroppableList(count: u32, shared: Shared) -> impl IntoView {
    let items = RwSignal::new(
        (0..count)
            .map(|id| Item {
                id,
                text: format!("Item {id}"),
            })
            .collect::<Vec<_>>(),
    );
    // Inserted rows get ids after the initial ones.
    let next_id = StoredValue::new(count);
    let collection = use_list_collection(UseListCollectionInput {
        items: items.into(),
        key: |item: &Item| Key::from(format!("item-{}", item.id)),
        text_value: |item: &Item| item.text.clone(),
    });
    let list = use_list_state(UseListStateInput {
        collection,
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            ..SelectionOptions::default()
        },
    });
    let element = CapturedElement::new();
    let UseGridListReturn {
        grid_list: props,
        data,
    } = use_grid_list(UseGridListInput {
        aria_label: "Items".into(),
        state: list,
        element,
        id: Some("perf-list".into()),
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
    let on_insert = Callback::new(move |e: DroppableCollectionInsertDropEvent| {
        let target = e
            .target
            .key
            .to_string()
            .trim_start_matches("item-")
            .parse::<u32>()
            .ok();
        let id = next_id.get_value();
        next_id.set_value(id + 1);
        let text = dropped_text(&e.items);
        items.update(|items| {
            let index = items
                .iter()
                .position(|item| Some(item.id) == target)
                .unwrap_or(0);
            let index = if e.target.drop_position == DropPosition::After {
                index + 1
            } else {
                index
            };
            items.insert(index, Item { id, text });
        });
    });
    let state = use_droppable_collection_state(UseDroppableCollectionStateInput {
        list,
        options: DroppableCollectionOptions {
            on_insert: Some(on_insert),
            // Accepting drops on the list itself.
            on_root_drop: Some(Callback::new(|_| {})),
            // Accepting drops on the rows.
            on_item_drop: Some(Callback::new(|_| {})),
            ..DroppableCollectionOptions::default()
        },
        is_disabled: shared.disabled.into(),
    });
    shared.state.set_value(Some(state));
    on_cleanup(move || shared.state.set_value(None));
    let UseDroppableCollectionReturn {
        collection: collection_props,
        data: drop,
    } = use_droppable_collection(UseDroppableCollectionInput {
        state,
        element,
        collection_id: data.id.clone(),
        keyboard_delegate: use_list_keyboard_delegate(UseListKeyboardDelegateInput {
            state: list,
            element,
            orientation: Orientation::Vertical.into(),
            layout: ListLayout::Stack,
            layout_delegate: None,
        }),
        drop_target_delegate: Arc::new(ListDropTargetDelegate::new(
            list.collection,
            list.item_elements,
            element,
        )),
        on_key_down: None,
    });
    let last = move || items.with(|items| items.last().map(|item| item.id));
    view! {
        <Provider value=ListContext { list: data, drop }>
            <div
                {..props.attrs()}
                {..collection_props.attrs()}
                style="height: 150px; overflow: auto; position: relative"
            >
                <DropIndicator target=DropTarget::Root name="root".to_owned() />
                <For
                    each=move || items.get()
                    key=|item| item.id
                    children=move |item: Item| view! { <Row item /> }
                />
                {move || {
                    last()
                        .map(|id| {
                            view! {
                                <DropIndicator
                                    target=DropTarget::item(format!("item-{id}"), DropPosition::After)
                                    name=format!("after-item-{id}")
                                />
                            }
                        })
                }}
            </div>
        </Provider>
    }
}

/// A row taking drops on it, after the drop indicator before it.
#[component]
fn Row(item: Item) -> impl IntoView {
    let ListContext { list, drop } = expect_context::<ListContext>();
    let key = Key::from(format!("item-{}", item.id));
    let UseGridListItemReturn {
        row: row_props,
        cell: grid_cell_props,
        ..
    } = use_grid_list_item(UseGridListItemInput {
        list,
        key: key.clone(),
        focus_mode: FocusMode::Row,
        allows_arrow_navigation: false,
        on_context_menu: None,
    });
    let element = CapturedElement::new();
    let UseDroppableItemReturn {
        item: drop_props,
        is_drop_target,
    } = use_droppable_item(UseDroppableItemInput {
        collection: drop,
        target: DropTarget::item(key.clone(), DropPosition::On).into(),
        element,
        activate_button: None,
    });
    let (row_attrs, row_styles) = row_props.attrs();
    view! {
        <DropIndicator
            target=DropTarget::item(key, DropPosition::Before)
            name=format!("before-item-{}", item.id)
        />
        <div
            {..row_attrs}
            {..drop_props.attrs()}
            {..element.attr()}
            style=row_styles
            data-row=item.id
            data-drop-target=flag(is_drop_target)
        >
            <div {..grid_cell_props.attrs()} style="height: 30px">
                {item.text}
            </div>
        </div>
    }
}

/// A drop indicator: a row of its own with a button, the drop target `target`.
#[component]
fn DropIndicator(target: DropTarget, name: String) -> impl IntoView {
    let ListContext { drop, .. } = expect_context::<ListContext>();
    let UseDropIndicatorReturn {
        indicator: drop_indicator_props,
        is_drop_target,
        is_hidden,
    } = use_drop_indicator(UseDropIndicatorInput {
        collection: drop,
        target: target.into(),
        activate_button: None,
    });
    view! {
        <div role="row">
            <div role="gridcell">
                <div
                    role="button"
                    {..drop_indicator_props.attrs()}
                    data-target=name
                    data-drop-target=flag(is_drop_target)
                    data-hidden=flag(is_hidden)
                ></div>
            </div>
        </div>
    }
}

//! Hook items (`use_grid_list_item` on plain elements) in a hook grid list, for the performance
//! test of `use_grid_list_item` (`leptonic/tests/perf_tests/hooks/use_grid_list_item.rs`): a list
//! (`use_grid_list` on a `div`) of [`SMALL`] items "Item <i>", each with a button "Edit <i>", with
//! multiple selection, between two plain buttons (`#perf-before`, `#perf-after`) to tab from and
//! to, mounted at first. Its controls ([`Control`]) mount and unmount the list (`mount`,
//! `mount-with-actions`: with an action; `unmount`), set its size while unmounted (`small`,
//! `large`: [`LARGE`] items), add and remove an item (mounting and unmounting a
//! `use_grid_list_item`), disable the second item, prepare the live announcer (`prepare-announcer`:
//! its first announcement waits for its live regions) and read how many items are selected
//! (`selected`) and how often an action ran (`actions`).

use std::{collections::HashSet, sync::Arc};

use leptonic::{
    Assertiveness, CapturedElement, Orientation, announce, flag,
    hooks::{
        collections::{
            Collection, CollectionMemo, CollectionOptions, Key, ListLayout, ListState, Selection,
            SelectionMode, SelectionOptions, UseListStateInput, use_list_state,
        },
        gridlist::{
            FocusMode, GridListData, KeyboardNavigationBehavior, UseGridListInput,
            UseGridListItemInput, UseGridListItemReturn, UseGridListReturn, use_grid_list,
            use_grid_list_item,
        },
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The items of the small list.
const SMALL: usize = 10;
/// The items of the large list.
const LARGE: usize = 1_000;

/// How many items of `selection` are selected (-1: all).
fn selected_count(selection: &Selection) -> f64 {
    match selection {
        Selection::All => -1.0,
        Selection::Keys(keys) => f64::from(u32::try_from(keys.len()).unwrap_or(u32::MAX)),
    }
}

/// `items` items ("Item <i>", key `i`).
fn list_collection(items: RwSignal<usize>) -> CollectionMemo {
    Memo::new(move |_| {
        Arc::new(Collection::build(|b| {
            for i in 0..items.get() {
                b.item(i, format!("Item {i}"));
            }
        }))
    })
}

/// An item: `use_grid_list_item` on a `div` (its cell a `div` inside), its pressed state in
/// `data-pressed`.
#[component]
fn HookGridListItem(list: GridListData, key: Key, children: Children) -> impl IntoView {
    let UseGridListItemReturn {
        row: row_props,
        cell: grid_cell_props,
        is_pressed,
        ..
    } = use_grid_list_item(UseGridListItemInput {
        list,
        key,
        focus_mode: FocusMode::Row,
        allows_arrow_navigation: false,
        on_context_menu: None,
    });
    let (attrs, styles) = row_props.attrs();
    view! {
        <div {..attrs} style=styles data-pressed=flag(is_pressed)>
            <div {..grid_cell_props.attrs()}>{children()}</div>
        </div>
    }
}

/// The list: `use_grid_list` on a `div`, with its items; `on_action` counts actions.
#[component]
fn HookGridList(
    state: ListState,
    items: RwSignal<usize>,
    on_action: Option<Callback<Key>>,
) -> impl IntoView {
    let UseGridListReturn {
        grid_list: props,
        data,
    } = use_grid_list(UseGridListInput {
        state,
        element: CapturedElement::new(),
        id: None,
        aria_label: "Perf list".into(),
        aria_labelledby: Signal::stored(None),
        layout: ListLayout::Stack,
        orientation: Signal::stored(Orientation::Vertical),
        keyboard_delegate: None,
        options: CollectionOptions::default(),
        keyboard_navigation_behavior: KeyboardNavigationBehavior::Arrow,
        should_select_on_press_up: false,
        on_action,
        tree: None,
    });
    view! {
        <div {..props.attrs()}>
            <For each=move || 0..items.get() key=|i| *i let:i>
                <HookGridListItem list=data.clone() key=Key::from(i)>
                    {format!("Item {i}")}
                    <button>{format!("Edit {i}")}</button>
                </HookGridListItem>
            </For>
        </div>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let with_actions = RwSignal::new(false);
    let items = RwSignal::new(SMALL);
    let disabled = RwSignal::new(HashSet::<Key>::new());
    let actions = StoredValue::new(0_u32);
    let state = use_list_state(UseListStateInput {
        collection: list_collection(items),
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            disabled_keys: disabled.into(),
            ..SelectionOptions::default()
        },
    });
    let controls = vec![
        Control::action("mount", move || {
            with_actions.set(false);
            mounted.set(true);
        }),
        Control::action("mount-with-actions", move || {
            with_actions.set(true);
            mounted.set(true);
        }),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || items.set(SMALL)),
        Control::action("large", move || items.set(LARGE)),
        Control::action("add-item", move || items.update(|items| *items += 1)),
        Control::action("remove-item", move || items.update(|items| *items -= 1)),
        Control::action("disable-second", move || {
            disabled.set(HashSet::from([Key::from(1_usize)]));
        }),
        Control::action("prepare-announcer", || announce("", Assertiveness::Polite)),
        Control::read("selected", move || {
            selected_count(&untrack(|| state.selection.raw_selection()))
        }),
        Control::read("actions", move || f64::from(actions.get_value())),
    ];
    let on_action = Callback::new(move |_: Key| actions.update_value(|count| *count += 1));
    view! {
        <h1>"use_grid_list_item performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <HookGridList
                state
                items
                on_action=with_actions.get_untracked().then_some(on_action)
            />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

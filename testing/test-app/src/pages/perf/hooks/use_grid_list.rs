//! A grid list built from the grid list hooks on plain elements, for the performance test of
//! `use_grid_list` (`leptonic/tests/perf_tests/hooks/use_grid_list.rs`): `use_grid_list` on a
//! `div` (`[role=grid]`), its [`SMALL`] items ("Item <i>", key `i`) `use_grid_list_item`s
//! ([`HookGridListItem`]), with multiple selection, between two plain buttons (`#perf-before`,
//! `#perf-after`) to tab from and to, mounted at first. The state (`use_list_state`) lives
//! outside the mounted part, so mounting measures `use_grid_list` and its items.
//!
//! Its controls ([`Control`]) mount and unmount the list, set its size (`small`, `large`:
//! [`LARGE`] items; `empty`: none), change the selection mode, prepare the live announcer
//! (`prepare-announcer`: its first announcement waits for its live regions) and read how many
//! items are selected (`selected`, -1 for all).
//!
//! [`HookGridListItem`] also builds the lists of the fixtures of the other grid list hooks.

use leptonic::{
    Assertiveness, CapturedElement, IntoAttrs, Orientation, announce, flag,
    hooks::{
        collections::{
            CollectionOptions, Key, ListLayout, ListState, SelectionMode, SelectionOptions,
            UseListStateInput, use_list_state,
        },
        gridlist::{
            FocusMode, GridListData, KeyboardNavigationBehavior, UseGridListInput,
            UseGridListItemInput, UseGridListItemReturn, UseGridListReturn, use_grid_list,
            use_grid_list_item,
        },
    },
};
use leptos::prelude::*;

pub use crate::pages::perf::atoms::grid_list::{LARGE, SMALL, list_collection, selected_count};
use crate::pages::perf::{Control, PerfControls};

/// An item: `use_grid_list_item` on a `div` (its cell a `div` inside), its pressed state in
/// `data-pressed`.
#[component]
pub fn HookGridListItem(list: GridListData, key: Key, children: Children) -> impl IntoView {
    let UseGridListItemReturn {
        row_props,
        grid_cell_props,
        is_pressed,
        ..
    } = use_grid_list_item(UseGridListItemInput {
        list,
        key,
        focus_mode: FocusMode::Row,
        allows_arrow_navigation: false,
        on_context_menu: None,
    });
    let (attrs, styles) = row_props.into_parts();
    view! {
        <div {..attrs} style=styles data-pressed=flag(is_pressed)>
            <div {..grid_cell_props.into_attrs()}>{children()}</div>
        </div>
    }
}

/// The list: `use_grid_list` on a `div`, with its items; `on_action` counts actions.
#[component]
pub fn HookGridList(
    state: ListState,
    items: RwSignal<usize>,
    on_action: Option<Callback<Key>>,
) -> impl IntoView {
    let UseGridListReturn { props, data } = use_grid_list(UseGridListInput {
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
        <div {..props.into_attrs()}>
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
    let items = RwSignal::new(SMALL);
    let selection_mode = RwSignal::new(SelectionMode::Multiple);
    let state = use_list_state(UseListStateInput {
        collection: list_collection(items),
        selection: SelectionOptions {
            selection_mode: selection_mode.into(),
            ..SelectionOptions::default()
        },
    });
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || items.set(SMALL)),
        Control::action("large", move || items.set(LARGE)),
        Control::action("empty", move || items.set(0)),
        Control::action("selection-none", move || {
            selection_mode.set(SelectionMode::None);
        }),
        Control::action("selection-multiple", move || {
            selection_mode.set(SelectionMode::Multiple);
        }),
        Control::action("prepare-announcer", || announce("", Assertiveness::Polite)),
        Control::read("selected", move || {
            selected_count(&untrack(|| state.selection.raw_selection()))
        }),
    ];
    view! {
        <h1>"use_grid_list performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <HookGridList state items on_action=None />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

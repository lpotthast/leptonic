//! `use_list_state` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_list_state.rs`): the state of a multiple selection of
//! the items "Item 0", "Item 1", ... ([`SMALL`] at first, [`LARGE`] after the control `large`),
//! shown in a plain list (`#perf-list`, one `<li>` per item with `data-perf-item` holding its
//! number, `aria-selected` and `data-focused` from the state). Mounted at first (mounting creates
//! the state). The controls ([`Control`]) call the state's methods (toggling, replacing and
//! extending the selection, selecting and toggling all, clearing, focusing an item) and remove
//! the focused item from the collection (the state moves the focus to the next one).

use leptonic::hooks::collections::{
    Key, ListState, SelectionMode, SelectionOptions, UseListCollectionInput, UseListStateInput,
    use_list_collection, use_list_state,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The items at first.
pub const SMALL: u32 = 10;
/// The items after the control `large`.
pub const LARGE: u32 = 1_000;

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let items = RwSignal::new((0..SMALL).collect::<Vec<u32>>());
    // The state while mounted, for the controls.
    let state = StoredValue::new(None::<ListState>);
    let with_state = move |run: fn(&ListState, u32)| {
        move || {
            let last = items.with_untracked(|items| items.last().copied().unwrap_or_default());
            state.with_value(|state| {
                if let Some(state) = state {
                    run(state, last);
                }
            });
        }
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("large", move || items.set((0..LARGE).collect())),
        Control::action(
            "toggle-first",
            with_state(|state, _| state.selection.toggle_selection(&Key::from(0_u32))),
        ),
        Control::action(
            "replace-with-last",
            with_state(|state, last| state.selection.replace_selection(&Key::from(last))),
        ),
        Control::action(
            "extend-to-last",
            with_state(|state, last| state.selection.extend_selection(&Key::from(last))),
        ),
        Control::action(
            "select-all",
            with_state(|state, _| state.selection.select_all()),
        ),
        Control::action(
            "toggle-select-all",
            with_state(|state, _| state.selection.toggle_select_all()),
        ),
        Control::action(
            "clear-selection",
            with_state(|state, _| state.selection.clear_selection()),
        ),
        Control::action(
            "focus-first",
            with_state(|state, _| {
                state.selection.set_focused(true);
                state
                    .selection
                    .set_focused_key(Some(Key::from(0_u32)), None);
            }),
        ),
        Control::action("remove-first", move || {
            items.update(|items| {
                items.remove(0);
            });
        }),
    ];
    view! {
        <h1>"use_list_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <Items items state />
        </Show>
    }
}

/// The state (created while mounted), shown as a list.
#[component]
fn Items(items: RwSignal<Vec<u32>>, state: StoredValue<Option<ListState>>) -> impl IntoView {
    let list = use_list_state(UseListStateInput {
        collection: use_list_collection(UseListCollectionInput {
            items: items.into(),
            key: |i| Key::from(*i),
            text_value: |i| format!("Item {i}"),
        }),
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            ..SelectionOptions::default()
        },
    });
    state.set_value(Some(list));
    on_cleanup(move || state.set_value(None));
    view! {
        <ul id="perf-list">
            <For each=move || items.get() key=|i| *i let:i>
                <li
                    data-perf-item=i
                    aria-selected=move || list.selection.is_selected(&Key::from(i)).to_string()
                    data-focused=move || {
                        list.selection.is_focused_key(&Key::from(i)).then_some("true")
                    }
                >
                    {format!("Item {i}")}
                </li>
            </For>
        </ul>
    }
}

//! `use_list_state_view` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_list_state_view.rs`): a view of the state of a multiple
//! selection of the items "Item 0", "Item 1", ... ([`SMALL`] at first, [`LARGE`] after the
//! control `large`), showing all items or only the even ones (the control `even`), in a plain
//! list (`#perf-list`, one `<li>` per item of the view with `data-perf-item` holding its number,
//! `aria-selected` and `data-focused` from the view). The state stays; the view is mounted at
//! first (mounting creates it). The controls ([`Control`]) change the filter, focus the second
//! item and select all items through the view.

use leptonic::hooks::collections::{
    Key, ListState, SelectionMode, SelectionOptions, UseListCollectionInput, UseListStateInput,
    UseListStateViewInput, use_collection, use_list_collection, use_list_state,
    use_list_state_view,
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
    let even = RwSignal::new(false);
    let state = use_list_state(UseListStateInput {
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
    // The view while mounted, for the controls.
    let view_state = StoredValue::new(None::<ListState>);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("large", move || items.set((0..LARGE).collect())),
        Control::action("even", move || even.set(true)),
        Control::action("all", move || even.set(false)),
        Control::action("focus-second", move || {
            view_state.with_value(|view| {
                if let Some(view) = view {
                    view.selection.set_focused(true);
                    view.selection.set_focused_key(Some(Key::from(1_u32)), None);
                }
            });
        }),
        Control::action("select-all", move || {
            view_state.with_value(|view| {
                if let Some(view) = view {
                    view.selection.select_all();
                }
            });
        }),
    ];
    view! {
        <h1>"use_list_state_view performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <View state items even view_state />
        </Show>
    }
}

/// The view (created while mounted), shown as a list.
#[component]
fn View(
    state: ListState,
    items: RwSignal<Vec<u32>>,
    even: RwSignal<bool>,
    view_state: StoredValue<Option<ListState>>,
) -> impl IntoView {
    let shown = move || {
        let even = even.get();
        items.with(|items| {
            items
                .iter()
                .copied()
                .filter(|i| !even || i % 2 == 0)
                .collect::<Vec<u32>>()
        })
    };
    let collection = use_collection(move |b| {
        for i in shown() {
            b.item(i, format!("Item {i}"));
        }
    });
    let view = use_list_state_view(UseListStateViewInput { state, collection });
    view_state.set_value(Some(view));
    on_cleanup(move || view_state.set_value(None));
    view! {
        <ul id="perf-list">
            <For each=shown key=|i| *i let:i>
                <li
                    data-perf-item=i
                    aria-selected=move || view.selection.is_selected(&Key::from(i)).to_string()
                    data-focused=move || {
                        view.selection.is_focused_key(&Key::from(i)).then_some("true")
                    }
                >
                    {format!("Item {i}")}
                </li>
            </For>
        </ul>
    }
}

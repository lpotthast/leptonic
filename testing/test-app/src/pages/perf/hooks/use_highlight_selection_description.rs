//! `use_highlight_selection_description` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_highlight_selection_description.rs`): the hook on the
//! selection of a grid state with row actions ([`SMALL`] rows "Row <i>" of three cells), with
//! multiple selection, called by a mounted component (mounted at first) that describes
//! `#perf-described` with it, after a plain button (`#perf-before`). Its controls ([`Control`])
//! mount and unmount it and change the selection behavior (`replace`, `toggle`). The description
//! only exists on touch devices.

use std::sync::Arc;

use leptonic::hooks::{
    collections::{Collection, CollectionMemo, SelectionBehavior, SelectionMode, SelectionOptions},
    grid::{
        GridFocusMode, GridState, UseGridStateInput, UseHighlightSelectionDescriptionInput,
        use_grid_state, use_highlight_selection_description,
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The rows of the small grid.
const SMALL: usize = 10;


/// `rows` rows ("Row <i>", key `i`), each with the cells "Row <i>", "Value <i>" and
/// "Group <i % 3>".
fn grid_collection(rows: RwSignal<usize>) -> CollectionMemo {
    Memo::new(move |_| {
        Arc::new(Collection::build(|b| {
            for i in 0..rows.get() {
                b.row(i, format!("Row {i}"), |r| {
                    r.cell(format!("Row {i}"));
                    r.cell(format!("Value {i}"));
                    r.cell(format!("Group {}", i % 3));
                });
            }
        }))
    })
}

/// Calls the hook, describing `#perf-described`.
#[component]
fn Described(state: GridState) -> impl IntoView {
    let description = use_highlight_selection_description(UseHighlightSelectionDescriptionInput {
        selection: state.list.selection,
        has_item_actions: true,
    });
    view! { <div id="perf-described" aria-describedby=description>"Rows with actions"</div> }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let behavior = RwSignal::new(SelectionBehavior::Toggle);
    let state = use_grid_state(UseGridStateInput {
        collection: grid_collection(RwSignal::new(SMALL)),
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            selection_behavior: behavior.into(),
            ..SelectionOptions::default()
        },
        focus_mode: GridFocusMode::Row,
    });
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("replace", move || behavior.set(SelectionBehavior::Replace)),
        Control::action("toggle", move || behavior.set(SelectionBehavior::Toggle)),
    ];
    view! {
        <h1>"use_highlight_selection_description performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Described state />
        </Show>
    }
}

//! `use_grid_selection_announcement` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_grid_selection_announcement.rs`): the hook on the state of
//! [`SMALL`] rows "Row <i>" of three cells, with multiple selection, called by a mounted component
//! (`#perf-announcer`, mounted at first). Its controls ([`Control`]) mount and unmount it, set the
//! size while unmounted (`small`, `large`: [`LARGE`] rows), mark the grid focused or not (the hook
//! announces changes only while it is), change the selection (one row, every row by key, all, none)
//! and prepare the live announcer (`prepare-announcer`: its first announcement waits for its live
//! regions).

use std::sync::Arc;

use leptonic::{
    Assertiveness, announce,
    hooks::{
        collections::{Collection, CollectionMemo, Key, SelectionMode, SelectionOptions},
        grid::{
            GridFocusMode, GridState, UseGridSelectionAnnouncementInput, UseGridStateInput,
            use_grid_selection_announcement, use_grid_state,
        },
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The rows of the small grid.
const SMALL: usize = 10;

/// The rows of the large grid.
const LARGE: usize = 1_000;

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

/// Calls the hook.
#[component]
fn Announcer(state: GridState) -> impl IntoView {
    use_grid_selection_announcement(UseGridSelectionAnnouncementInput {
        selection: state.list.selection,
        collection: state.list.collection,
        get_row_text: None,
    });
    view! { <p id="perf-announcer">"Announcing selection changes"</p> }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let rows = RwSignal::new(SMALL);
    let state = use_grid_state(UseGridStateInput {
        collection: grid_collection(rows),
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            ..SelectionOptions::default()
        },
        focus_mode: GridFocusMode::Row,
    });
    let selection = state.list.selection;
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || rows.set(SMALL)),
        Control::action("large", move || rows.set(LARGE)),
        Control::action("focus", move || selection.set_focused(true)),
        Control::action("blur", move || selection.set_focused(false)),
        Control::action("toggle-first", move || {
            selection.toggle_selection(&Key::from(0_usize));
        }),
        Control::action("select-every-key", move || {
            selection.set_selected_keys((0..rows.get_untracked()).map(Key::from));
        }),
        Control::action("select-all", move || selection.select_all()),
        Control::action("clear-selection", move || selection.clear_selection()),
        Control::action("prepare-announcer", || announce("", Assertiveness::Polite)),
    ];
    view! {
        <h1>"use_grid_selection_announcement performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <Announcer state />
        </Show>
    }
}

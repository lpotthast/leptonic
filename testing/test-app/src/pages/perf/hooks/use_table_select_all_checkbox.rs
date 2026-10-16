//! A select all checkbox (`use_table_select_all_checkbox`, rendered with `use_checkbox`,
//! `#perf-select-all input`) for a hook table, for the performance test of
//! `use_table_select_all_checkbox`
//! (`leptonic/tests/perf_tests/hooks/use_table_select_all_checkbox.rs`): the table of the
//! `use_table` fixture ([`super::use_table`]: [`SMALL`] rows "Row <i>" in the columns Name, Value
//! and Group) with multiple selection and selection checkboxes, the checkbox above it, mounted at
//! first. Its controls ([`Control`]) mount and unmount the table, set its size (`small`,
//! `large`: [`LARGE`] rows, while unmounted; `empty`: none), select the first row and none,
//! change the selection mode (`single`, `multiple`), prepare the live announcer
//! (`prepare-announcer`: its first announcement waits for its live regions) and read how many
//! rows are selected (`selected`, -1 for all).

use leptonic::{
    Assertiveness, CapturedElement, IntoAttrs, announce,
    hooks::{
        collections::{CollectionOptions, Key, SelectionMode, SelectionOptions},
        form::use_checkbox,
        grid::GridFocusMode,
        gridlist::KeyboardNavigationBehavior,
        table::{
            TableState, UseTableInput, UseTableReturn, UseTableSelectAllCheckboxInput,
            UseTableStateInput, use_table, use_table_select_all_checkbox, use_table_state,
        },
    },
};
use leptos::prelude::*;

use super::use_table::{
    HookTableBody, HookTableHeader, LARGE, SMALL, selected_count, table_collection,
};
use crate::pages::perf::{Control, PerfControls};

/// The checkbox and the table: `use_table` on a `<table>`.
#[component]
fn PerfTable(state: TableState, rows: RwSignal<usize>) -> impl IntoView {
    let UseTableReturn { props, data } = use_table(UseTableInput {
        state,
        element: CapturedElement::new(),
        id: None,
        aria_label: "Perf table".into(),
        aria_labelledby: Signal::stored(None),
        keyboard_delegate: None,
        options: CollectionOptions::default(),
        keyboard_navigation_behavior: KeyboardNavigationBehavior::Arrow,
        should_select_on_press_up: false,
        on_row_action: None,
        on_cell_action: None,
    });
    let checkbox = use_checkbox(use_table_select_all_checkbox(
        UseTableSelectAllCheckboxInput {
            table: data.clone(),
        },
    ));
    let (attrs, styles) = checkbox.input_props.into_parts();
    view! {
        <div id="perf-select-all">
            <input {..attrs} style=styles />
        </div>
        <table {..props.into_attrs()}>
            <HookTableHeader table=data.clone() />
            <HookTableBody table=data rows />
        </table>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let rows = RwSignal::new(SMALL);
    let selection_mode = RwSignal::new(SelectionMode::Multiple);
    let state = use_table_state(UseTableStateInput {
        table: table_collection(rows, false),
        show_selection_checkboxes: Signal::stored(true),
        selection: SelectionOptions {
            selection_mode: selection_mode.into(),
            ..SelectionOptions::default()
        },
        focus_mode: GridFocusMode::Row,
        default_sort_descriptor: None,
        sort_descriptor: None,
        on_sort_change: None,
        tree: None,
    });
    let selection = state.grid.list.selection;
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || rows.set(SMALL)),
        Control::action("large", move || rows.set(LARGE)),
        Control::action("empty", move || rows.set(0)),
        Control::action("select-first", move || {
            selection.set_selected_keys([Key::from(0_usize)]);
        }),
        Control::action("clear-selection", move || selection.clear_selection()),
        Control::action("single", move || selection_mode.set(SelectionMode::Single)),
        Control::action("multiple", move || {
            selection_mode.set(SelectionMode::Multiple);
        }),
        Control::action("prepare-announcer", || announce("", Assertiveness::Polite)),
        Control::read("selected", move || {
            selected_count(&untrack(|| selection.raw_selection()))
        }),
    ];
    view! {
        <h1>"use_table_select_all_checkbox performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <PerfTable state rows />
        </Show>
    }
}

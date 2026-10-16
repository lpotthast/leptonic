//! Hook header placeholders (`use_table_header_placeholder` on `<th>`s) of a hook table with a
//! column group, for the performance test of `use_table_header_placeholder`
//! (`leptonic/tests/perf_tests/hooks/use_table_header_placeholder.rs`): the columns Name, Details
//! (a group of Value and Group) and Notes, so that the group's header row has a placeholder above
//! Name and one above Notes, and [`SMALL`] rows ("Row <i>", key `i`), with multiple selection,
//! mounted at first. Its controls ([`Control`]) mount and unmount the header alone
//! (`mount-header`, `unmount-header`) or the table (`mount`, `unmount`), set the size while
//! unmounted (`small`, `large`: [`LARGE`] rows) and add and remove a row (`add-row`,
//! `remove-row`).

use std::sync::Arc;

use leptonic::{
    CapturedElement, IntoAttrs,
    hooks::{
        collections::{CollectionOptions, SelectionMode, SelectionOptions},
        grid::GridFocusMode,
        gridlist::KeyboardNavigationBehavior,
        table::{
            TableCollection, TableState, UseTableInput, UseTableReturn, UseTableStateInput,
            use_table, use_table_state,
        },
    },
};
use leptos::prelude::*;

use super::use_table::{HookTableBody, HookTableHeader, LARGE, SMALL};
use crate::pages::perf::{Control, PerfControls};

/// The table: `use_table` on a `<table>`, its header mounted while `header`.
#[component]
fn PerfTable(state: TableState, rows: RwSignal<usize>, header: RwSignal<bool>) -> impl IntoView {
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
    let header_table = data.clone();
    view! {
        <table {..props.into_attrs()}>
            <Show when=move || header.get()>
                <HookTableHeader table=header_table.clone() />
            </Show>
            <HookTableBody table=data rows />
        </table>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let header = RwSignal::new(true);
    let rows = RwSignal::new(SMALL);
    let table = Memo::new(move |_| {
        Arc::new(TableCollection::build(|t| {
            let _ = t.column("name", "Name").row_header();
            t.column_group("details", "Details", |g| {
                g.column("value", "Value");
                g.column("group", "Group");
            });
            t.column("notes", "Notes");
            for i in 0..rows.get() {
                t.row(i, format!("Row {i}"), |r| {
                    r.cell(format!("Row {i}"));
                    r.cell(format!("Value {i}"));
                    r.cell(format!("Group {}", i % 3));
                    r.cell(format!("Notes {i}"));
                });
            }
        }))
    });
    let state = use_table_state(UseTableStateInput {
        table,
        show_selection_checkboxes: Signal::stored(false),
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            ..SelectionOptions::default()
        },
        focus_mode: GridFocusMode::Row,
        default_sort_descriptor: None,
        sort_descriptor: None,
        on_sort_change: None,
        tree: None,
    });
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("mount-header", move || header.set(true)),
        Control::action("unmount-header", move || header.set(false)),
        Control::action("small", move || rows.set(SMALL)),
        Control::action("large", move || rows.set(LARGE)),
        Control::action("add-row", move || rows.update(|rows| *rows += 1)),
        Control::action("remove-row", move || rows.update(|rows| *rows -= 1)),
    ];
    view! {
        <h1>"use_table_header_placeholder performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <PerfTable state rows header />
        </Show>
    }
}

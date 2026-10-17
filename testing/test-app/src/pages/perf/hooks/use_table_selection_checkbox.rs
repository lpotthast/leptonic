//! Row selection checkboxes (`use_table_selection_checkbox`, rendered with `use_checkbox`) in the
//! selection column of a hook table, for the performance test of `use_table_selection_checkbox`
//! (`leptonic/tests/perf_tests/hooks/use_table_selection_checkbox.rs`): a table (`use_table` on a
//! `<table>`) of [`SMALL`] rows "Row <i>" in the columns Name, Value and Group, with multiple
//! selection and selection checkboxes, between two plain buttons (`#perf-before`, `#perf-after`) to
//! tab from and to, mounted at first. Its controls ([`Control`]) mount and unmount the table, set
//! its size while unmounted (`small`, `large`: [`LARGE`] rows), select all rows and none, disable
//! the first row and enable it, prepare the live announcer (`prepare-announcer`: its first
//! announcement waits for its live regions) and read how many rows are selected (`selected`, -1 for
//! all).

use std::{collections::HashSet, sync::Arc};

use leptonic::{
    Assertiveness, CapturedElement, announce, flag,
    hooks::{
        collections::{
            CollectionOptions, Key, NodeKind, Selection, SelectionMode, SelectionOptions,
        },
        form::use_checkbox,
        grid::GridFocusMode,
        gridlist::KeyboardNavigationBehavior,
        table::{
            ColumnKind, TableCollection, TableData, TableState, UseTableCellInput,
            UseTableCellReturn, UseTableColumnHeaderInput, UseTableColumnHeaderReturn,
            UseTableHeaderPlaceholderInput, UseTableInput, UseTableReturn, UseTableRowInput,
            UseTableRowReturn, UseTableSelectionCheckboxInput, UseTableStateInput, use_table,
            use_table_cell, use_table_column_header, use_table_header_placeholder,
            use_table_header_row, use_table_row, use_table_selection_checkbox, use_table_state,
        },
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The rows of the small table.
const SMALL: usize = 10;
/// The rows of the large table.
const LARGE: usize = 1_000;

/// How many rows of `selection` are selected (-1: all).
fn selected_count(selection: &Selection) -> f64 {
    match selection {
        Selection::All => -1.0,
        Selection::Keys(keys) => f64::from(u32::try_from(keys.len()).unwrap_or(u32::MAX)),
    }
}

/// `rows` rows ("Row <i>", key `i`) in the sortable columns `name` (the row header), `value` and
/// `group`.
fn table_collection(rows: RwSignal<usize>) -> Memo<Arc<TableCollection>> {
    Memo::new(move |_| {
        Arc::new(TableCollection::build(|t| {
            for (key, text) in [("name", "Name"), ("value", "Value"), ("group", "Group")] {
                let column = t.column(key, text).allows_sorting();
                if key == "name" {
                    let _ = column.row_header();
                }
            }
            for i in 0..rows.get() {
                t.row(i, format!("Row {i}"), |r| {
                    r.cell(format!("Row {i}"));
                    r.cell(format!("Value {i}"));
                    r.cell(format!("Group {}", i % 3));
                });
            }
        }))
    })
}

/// The state of a table of `rows` rows with multiple selection and selection checkboxes, the rows
/// `disabled` disabled.
fn hook_table_state(rows: RwSignal<usize>, disabled: Signal<HashSet<Key>>) -> TableState {
    use_table_state(UseTableStateInput {
        table: table_collection(rows),
        show_selection_checkboxes: Signal::stored(true),
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            disabled_keys: disabled,
            ..SelectionOptions::default()
        },
        focus_mode: GridFocusMode::Row,
        default_sort_descriptor: None,
        sort_descriptor: None,
        on_sort_change: None,
        tree: None,
    })
}

/// The table: `use_table` on a `<table>`, with its header and body.
#[component]
fn HookTable(state: TableState, rows: RwSignal<usize>) -> impl IntoView {
    let UseTableReturn { table: props, data } = use_table(UseTableInput {
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
    view! {
        <table {..props.attrs()}>
            <HookTableHeader table=data.clone() />
            <HookTableBody table=data rows />
        </table>
    }
}

/// A column header: `use_table_column_header` on a `<th>` with the column's name, its pressed
/// state in `data-pressed`.
#[component]
fn HookColumnHeader(table: TableData, key: Key) -> impl IntoView {
    let columns = table.state.columns;
    let name_key = key.clone();
    let name = move || {
        columns.with(|columns| {
            columns
                .iter()
                .find(|c| c.key == name_key)
                .map(|c| c.text_value.to_string())
        })
    };
    let UseTableColumnHeaderReturn {
        column_header: column_header_props,
        is_pressed,
    } = use_table_column_header(UseTableColumnHeaderInput {
        table,
        key,
        allows_arrow_navigation: false,
    });
    let (attrs, styles) = column_header_props.attrs();
    view! {
        <th {..attrs} style=styles data-pressed=flag(is_pressed)>
            {name}
        </th>
    }
}

/// The header: a `<thead>` of `use_table_header_row`s, their column headers and placeholders.
#[component]
fn HookTableHeader(table: TableData) -> impl IntoView {
    let rows = table.state.table;
    view! {
        <thead>
            <For
                each=move || rows.with(|t| t.header_rows().to_vec())
                key=Clone::clone
                children=move |row: Key| {
                    let table = table.clone();
                    let cells = move || {
                        rows.with(|t| {
                            t.collection()
                                .children(&row)
                                .map(|n| (n.key.clone(), n.kind))
                                .collect::<Vec<_>>()
                        })
                    };
                    view! {
                        <tr {..use_table_header_row().attrs()}>
                            <For
                                each=cells
                                key=|(key, _)| key.clone()
                                children=move |(key, kind): (Key, NodeKind)| {
                                    if kind == NodeKind::Placeholder {
                                        let placeholder = use_table_header_placeholder(UseTableHeaderPlaceholderInput {
                                            table: table.clone(),
                                            key,
                                        });
                                        view! { <th {..placeholder.attrs()}></th> }.into_any()
                                    } else {
                                        view! { <HookColumnHeader table=table.clone() key /> }
                                            .into_any()
                                    }
                                }
                            />
                        </tr>
                    }
                }
            />
        </thead>
    }
}

/// A cell: `use_table_cell` on a `<td>` (its pressed state in `data-pressed`) for the column at
/// `index` of row `row`, with the cell's text, or the row's selection checkbox
/// (`use_table_selection_checkbox`) in a selection column.
#[component]
fn HookTableCell(table: TableData, row: Key, index: usize) -> impl IntoView {
    let (key, kind, text) = table.state.table.with_untracked(|t| {
        let key = t
            .cell_key(&row, index)
            .unwrap_or_else(|| Key::cell(&row, index));
        let kind = t.column_at(index).map_or(ColumnKind::Data, |c| c.kind);
        let text = t
            .collection()
            .get(&key)
            .map(|n| n.text_value.to_string())
            .unwrap_or_default();
        (key, kind, text)
    });
    let content = if kind == ColumnKind::SelectionCheckbox {
        let checkbox = use_checkbox(use_table_selection_checkbox(
            UseTableSelectionCheckboxInput {
                table: table.clone(),
                key: row,
            },
        ));
        let (attrs, styles) = checkbox
            .input
            .props()
            .expect("the first input takes its props")
            .into_parts();
        view! { <input {..attrs} style=styles /> }.into_any()
    } else {
        text.into_any()
    };
    let UseTableCellReturn {
        cell: grid_cell_props,
        is_pressed,
    } = use_table_cell(UseTableCellInput {
        should_select_on_press_up: table.grid.should_select_on_press_up,
        table,
        key,
        focus_mode: None,
        allows_arrow_navigation: false,
    });
    let (attrs, styles) = grid_cell_props.attrs();
    view! {
        <td {..attrs} style=styles data-pressed=flag(is_pressed)>
            {content}
        </td>
    }
}

/// A row: `use_table_row` on a `<tr>` (its pressed state in `data-pressed`) with a cell per
/// column.
#[component]
fn HookTableRow(table: TableData, key: Key) -> impl IntoView {
    let columns = table.state.table.with_untracked(|t| t.column_count());
    let UseTableRowReturn {
        row: row_props,
        is_pressed,
        ..
    } = use_table_row(UseTableRowInput {
        table: table.clone(),
        key: key.clone(),
        on_context_menu: None,
    });
    let (attrs, styles) = row_props.attrs();
    view! {
        <tr {..attrs} style=styles data-pressed=flag(is_pressed)>
            {(0..columns)
                .map(move |index| {
                    view! { <HookTableCell table=table.clone() row=key.clone() index /> }
                })
                .collect_view()}
        </tr>
    }
}

/// The body: a `<tbody>` with `rows` rows.
#[component]
fn HookTableBody(table: TableData, rows: RwSignal<usize>) -> impl IntoView {
    view! {
        <tbody>
            <For each=move || 0..rows.get() key=|i| *i let:i>
                <HookTableRow table=table.clone() key=Key::from(i) />
            </For>
        </tbody>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let rows = RwSignal::new(SMALL);
    let disabled = RwSignal::new(HashSet::<Key>::new());
    let state = hook_table_state(rows, disabled.into());
    let selection = state.grid.list.selection;
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || rows.set(SMALL)),
        Control::action("large", move || rows.set(LARGE)),
        Control::action("select-all", move || selection.select_all()),
        Control::action("clear-selection", move || selection.clear_selection()),
        Control::action("disable-first", move || {
            disabled.set(HashSet::from([Key::from(0_usize)]));
        }),
        Control::action("enable-all", move || disabled.set(HashSet::new())),
        Control::action("prepare-announcer", || announce("", Assertiveness::Polite)),
        Control::read("selected", move || {
            selected_count(&untrack(|| selection.raw_selection()))
        }),
    ];
    view! {
        <h1>"use_table_selection_checkbox performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <HookTable state rows />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

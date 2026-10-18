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
    CapturedElement, flag,
    hooks::{
        collections::{CollectionOptions, Key, NodeKind, SelectionMode, SelectionOptions},
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
fn HookTableCell(
    table: TableData,
    row: Key,
    index: usize,
    is_selected: Signal<bool>,
) -> impl IntoView {
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
                is_selected,
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
        is_selected,
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
                    view! { <HookTableCell table=table.clone() row=key.clone() index is_selected /> }
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

/// The table: `use_table` on a `<table>`, its header mounted while `header`.
#[component]
fn PerfTable(state: TableState, rows: RwSignal<usize>, header: RwSignal<bool>) -> impl IntoView {
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
    let header_table = data.clone();
    view! {
        <table {..props.attrs()}>
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

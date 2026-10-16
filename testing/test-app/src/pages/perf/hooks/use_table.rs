//! A table built from the table hooks on plain elements, for the performance test of
//! `use_table` (`leptonic/tests/perf_tests/hooks/use_table.rs`): `use_table` on a `<table>`
//! (`[role=grid]`) with a header ([`HookTableHeader`]: `use_table_header_row`s with
//! `use_table_column_header`s) and a body of [`SMALL`] rows ([`HookTableRow`]: `use_table_row`s
//! with `use_table_cell`s; "Row <i>", key `i`) in the sortable columns Name (the row header),
//! Value and Group, with multiple selection, between two plain buttons (`#perf-before`,
//! `#perf-after`) to tab from and to, mounted at first. The state (`use_table_state`) lives
//! outside the mounted part, so mounting measures `use_table` and its rows.
//!
//! Its controls ([`Control`]) mount and unmount the table, set its size (`small`, `large`:
//! [`LARGE`] rows, while unmounted; `empty`: none), sort it (`sort-ascending`,
//! `sort-descending`: by Value), prepare the live announcer (`prepare-announcer`: its first
//! announcement waits for its live regions) and read how many rows are selected (`selected`, -1
//! for all).
//!
//! [`HookTable`], [`HookTableHeader`], [`HookTableBody`] and [`HookTableRow`] also build the
//! tables of the fixtures of the other table hooks.

use std::{collections::HashSet, sync::Arc};

use leptonic::{
    Assertiveness, CapturedElement, IntoAttrs, announce, flag,
    hooks::{
        collections::{CollectionOptions, Key, NodeKind, SelectionMode, SelectionOptions},
        form::use_checkbox,
        grid::GridFocusMode,
        gridlist::KeyboardNavigationBehavior,
        table::{
            ColumnKind, SortDirection, TableData, TableState, UseTableCellInput,
            UseTableCellReturn, UseTableColumnHeaderInput, UseTableColumnHeaderReturn,
            UseTableHeaderPlaceholderInput, UseTableInput, UseTableReturn, UseTableRowInput,
            UseTableRowReturn, UseTableSelectionCheckboxInput, UseTableStateInput, use_table,
            use_table_cell, use_table_column_header, use_table_header_placeholder,
            use_table_header_row, use_table_row, use_table_selection_checkbox, use_table_state,
        },
    },
};
use leptos::prelude::*;

pub use crate::pages::perf::atoms::table::{LARGE, SMALL, selected_count, table_collection};
use crate::pages::perf::{Control, PerfControls};

/// A column header: `use_table_column_header` on a `<th>` with the column's name, its pressed
/// state in `data-pressed`.
#[component]
pub fn HookColumnHeader(table: TableData, key: Key) -> impl IntoView {
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
        column_header_props,
        is_pressed,
    } = use_table_column_header(UseTableColumnHeaderInput {
        table,
        key,
        allows_arrow_navigation: false,
    });
    let (attrs, styles) = column_header_props.into_parts();
    view! {
        <th {..attrs} style=styles data-pressed=flag(is_pressed)>
            {name}
        </th>
    }
}

/// The header: a `<thead>` of `use_table_header_row`s, their column headers and placeholders.
#[component]
pub fn HookTableHeader(table: TableData) -> impl IntoView {
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
                        <tr {..use_table_header_row().into_attrs()}>
                            <For
                                each=cells
                                key=|(key, _)| key.clone()
                                children=move |(key, kind): (Key, NodeKind)| {
                                    if kind == NodeKind::Placeholder {
                                        let placeholder = use_table_header_placeholder(UseTableHeaderPlaceholderInput {
                                            table: table.clone(),
                                            key,
                                        });
                                        view! { <th {..placeholder.into_attrs()}></th> }.into_any()
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
pub fn HookTableCell(table: TableData, row: Key, index: usize) -> impl IntoView {
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
        let (attrs, styles) = checkbox.input_props.into_parts();
        view! { <input {..attrs} style=styles /> }.into_any()
    } else {
        text.into_any()
    };
    let UseTableCellReturn {
        grid_cell_props,
        is_pressed,
    } = use_table_cell(UseTableCellInput {
        should_select_on_press_up: table.grid.should_select_on_press_up,
        table,
        key,
        focus_mode: None,
        allows_arrow_navigation: false,
    });
    let (attrs, styles) = grid_cell_props.into_parts();
    view! {
        <td {..attrs} style=styles data-pressed=flag(is_pressed)>
            {content}
        </td>
    }
}

/// A row: `use_table_row` on a `<tr>` (its pressed state in `data-pressed`) with a cell per
/// column.
#[component]
pub fn HookTableRow(table: TableData, key: Key) -> impl IntoView {
    let columns = table.state.table.with_untracked(|t| t.column_count());
    let UseTableRowReturn {
        row_props,
        is_pressed,
        ..
    } = use_table_row(UseTableRowInput {
        table: table.clone(),
        key: key.clone(),
        on_context_menu: None,
    });
    let (attrs, styles) = row_props.into_parts();
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
pub fn HookTableBody(table: TableData, rows: RwSignal<usize>) -> impl IntoView {
    view! {
        <tbody>
            <For each=move || 0..rows.get() key=|i| *i let:i>
                <HookTableRow table=table.clone() key=Key::from(i) />
            </For>
        </tbody>
    }
}

/// The table: `use_table` on a `<table>`, with its header and body; the actions count row and
/// cell actions.
#[component]
pub fn HookTable(
    state: TableState,
    rows: RwSignal<usize>,
    on_row_action: Option<Callback<Key>>,
    on_cell_action: Option<Callback<Key>>,
) -> impl IntoView {
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
        on_row_action,
        on_cell_action,
    });
    view! {
        <table {..props.into_attrs()}>
            <HookTableHeader table=data.clone() />
            <HookTableBody table=data rows />
        </table>
    }
}

/// The state of a table of `rows` rows with multiple selection (`checkboxes`: with selection
/// checkboxes), the rows `disabled` disabled (the key set of the selection options).
#[allow(clippy::implicit_hasher)]
pub fn hook_table_state(
    rows: RwSignal<usize>,
    checkboxes: bool,
    disabled: Signal<HashSet<Key>>,
) -> TableState {
    use_table_state(UseTableStateInput {
        table: table_collection(rows, false),
        show_selection_checkboxes: Signal::stored(checkboxes),
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

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let rows = RwSignal::new(SMALL);
    let state = hook_table_state(rows, false, Signal::stored(HashSet::new()));
    let value = Arc::new(Key::from("value"));
    let ascending = Arc::clone(&value);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || rows.set(SMALL)),
        Control::action("large", move || rows.set(LARGE)),
        Control::action("empty", move || rows.set(0)),
        Control::action("sort-ascending", move || {
            state.sort(&ascending, Some(SortDirection::Ascending));
        }),
        Control::action("sort-descending", move || {
            state.sort(&value, Some(SortDirection::Descending));
        }),
        Control::action("prepare-announcer", || announce("", Assertiveness::Polite)),
        Control::read("selected", move || {
            selected_count(&untrack(|| state.grid.list.selection.raw_selection()))
        }),
    ];
    view! {
        <h1>"use_table performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <HookTable state rows on_row_action=None on_cell_action=None />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

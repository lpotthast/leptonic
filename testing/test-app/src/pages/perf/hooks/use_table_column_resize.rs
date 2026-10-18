//! Hook column resizers (`use_table_column_resize`: a resizer `div` with a range input, in each
//! column header of a hook table), for the performance test of `use_table_column_resize`
//! (`leptonic/tests/perf_tests/hooks/use_table_column_resize.rs`): a table (`use_table` on a
//! `<table>`) of [`SMALL`] rows "Row <i>" in the resizable columns Name, Value and Group, with a
//! column resize state (`use_table_column_resize_state`) for a table width of 900 pixels, the
//! column headers' widths from it, styled by the test-app's `test-resizable-container` class (the
//! resizers 8 pixels wide at the right edge of their column headers), between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls ([`Control`])
//! mount and unmount the table, set its size while unmounted (`small`, `large`: [`LARGE`] rows) and
//! read how often a resize ended (`resize-ends`).

use std::sync::Arc;

use leptonic::{
    CapturedElement, flag,
    hooks::{
        collections::{CollectionOptions, Key, SelectionMode, SelectionOptions},
        form::use_checkbox,
        grid::GridFocusMode,
        gridlist::KeyboardNavigationBehavior,
        table::{
            ColumnKind, TableCollection, TableColumnResizeState, TableData, TableState,
            UseTableCellInput, UseTableCellReturn, UseTableColumnHeaderInput,
            UseTableColumnHeaderReturn, UseTableColumnResizeInput, UseTableColumnResizeReturn,
            UseTableColumnResizeStateInput, UseTableInput, UseTableReturn, UseTableRowInput,
            UseTableRowReturn, UseTableSelectionCheckboxInput, UseTableStateInput, use_table,
            use_table_cell, use_table_column_header, use_table_column_resize,
            use_table_column_resize_state, use_table_header_row, use_table_row,
            use_table_selection_checkbox, use_table_state,
        },
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The rows of the small table.
const SMALL: usize = 10;
/// The rows of the large table.
const LARGE: usize = 1_000;

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

/// The state of a table of `rows` rows with multiple selection.
fn hook_table_state(rows: RwSignal<usize>) -> TableState {
    use_table_state(UseTableStateInput {
        table: table_collection(rows),
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
    })
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

/// A column header (`use_table_column_header` on a `<th>`, its width from `resize`) with its
/// resizer (`use_table_column_resize`).
#[component]
fn ResizableHeader(
    table: TableData,
    resize: TableColumnResizeState,
    key: Key,
    name: String,
    resize_ends: StoredValue<u32>,
) -> impl IntoView {
    let header = CapturedElement::new();
    let UseTableColumnResizeReturn {
        resizer: resizer_props,
        input: input_props,
        is_resizing,
        ..
    } = use_table_column_resize(UseTableColumnResizeInput {
        state: resize,
        table: table.clone(),
        column: key.clone(),
        aria_label: "Resizer".into(),
        element: CapturedElement::new(),
        trigger: Some(header),
        is_disabled: Signal::stored(false),
        on_resize_start: None,
        on_resize: None,
        on_resize_end: Some(Callback::new(move |_| {
            resize_ends.update_value(|count| *count += 1);
        })),
    });
    let width_key = key.clone();
    let UseTableColumnHeaderReturn {
        column_header: column_header_props,
        ..
    } = use_table_column_header(UseTableColumnHeaderInput {
        table,
        key,
        allows_arrow_navigation: false,
    });
    let (attrs, styles) = column_header_props.attrs();
    let styles = styles.add_optional_unchecked("width", move || {
        Some(format!("{}px", resize.column_width(&width_key)))
    });
    let (resizer_attrs, resizer_styles) = resizer_props.attrs();
    let (input_attrs, input_styles) = input_props.attrs();
    view! {
        <th
            {..attrs}
            {..header.attr()}
            style=styles
            data-resizing=move || is_resizing.get().then_some("true")
        >
            {name}
            <div {..resizer_attrs} style=resizer_styles data-column-resizer="true">
                <input {..input_attrs} style=input_styles />
            </div>
        </th>
    }
}

/// The table: `use_table` on a `<table>` with resizable column headers.
#[component]
fn PerfTable(
    state: TableState,
    rows: RwSignal<usize>,
    resize_ends: StoredValue<u32>,
) -> impl IntoView {
    let resize = use_table_column_resize_state(UseTableColumnResizeStateInput {
        table_state: state,
        table_width: Signal::stored(900.0),
        default_width: None,
        default_min_width: None,
    });
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
    let columns = state.columns.get_untracked();
    let header_table = data.clone();
    view! {
        <div class="test-resizable-container">
            <table {..props.attrs()} style="table-layout: fixed; width: min-content">
                <thead>
                    <tr {..use_table_header_row().attrs()}>
                        {columns
                            .iter()
                            .map(|column| {
                                view! {
                                    <ResizableHeader
                                        table=header_table.clone()
                                        resize
                                        key=column.key.clone()
                                        name=column.text_value.to_string()
                                        resize_ends
                                    />
                                }
                            })
                            .collect_view()}
                    </tr>
                </thead>
                <HookTableBody table=data rows />
            </table>
        </div>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let rows = RwSignal::new(SMALL);
    let resize_ends = StoredValue::new(0_u32);
    let state = hook_table_state(rows);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || rows.set(SMALL)),
        Control::action("large", move || rows.set(LARGE)),
        Control::read("resize-ends", move || f64::from(resize_ends.get_value())),
    ];
    view! {
        <h1>"use_table_column_resize performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <PerfTable state rows resize_ends />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

//! Hook column resizers (`use_table_column_resize`: a resizer `div` with a range input, in each
//! column header of a hook table), for the performance test of `use_table_column_resize`
//! (`leptonic/tests/perf_tests/hooks/use_table_column_resize.rs`): the table of the `use_table`
//! fixture ([`super::use_table`]: [`SMALL`] rows "Row <i>" in the columns Name, Value and Group,
//! here resizable) with a column resize state (`use_table_column_resize_state`) for a table width
//! of 900 pixels, the column headers' widths from it, styled by the test-app's
//! `test-resizable-container` class (the resizers 8 pixels wide at the right edge of their column
//! headers), between two plain buttons (`#perf-before`, `#perf-after`) to tab from and to,
//! mounted at first. Its controls ([`Control`]) mount and unmount the table, set its size while
//! unmounted (`small`, `large`: [`LARGE`] rows) and read how often a resize ended
//! (`resize-ends`).

use std::collections::HashSet;

use leptonic::{
    CapturedElement, IntoAttrs,
    hooks::{
        collections::{CollectionOptions, Key},
        gridlist::KeyboardNavigationBehavior,
        table::{
            TableColumnResizeState, TableData, TableState, UseTableColumnHeaderInput,
            UseTableColumnHeaderReturn, UseTableColumnResizeInput, UseTableColumnResizeReturn,
            UseTableColumnResizeStateInput, UseTableInput, UseTableReturn, use_table,
            use_table_column_header, use_table_column_resize, use_table_column_resize_state,
            use_table_header_row,
        },
    },
};
use leptos::prelude::*;

use super::use_table::{HookTableBody, LARGE, SMALL, hook_table_state};
use crate::pages::perf::{Control, PerfControls};

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
        resizer_props,
        input_props,
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
        column_header_props,
        ..
    } = use_table_column_header(UseTableColumnHeaderInput {
        table,
        key,
        allows_arrow_navigation: false,
    });
    let (attrs, styles) = column_header_props.into_parts();
    let styles = styles.add_optional_unchecked("width", move || {
        Some(format!("{}px", resize.column_width(&width_key)))
    });
    let (resizer_attrs, resizer_styles) = resizer_props.into_parts();
    let (input_attrs, input_styles) = input_props.into_parts();
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
    let columns = state.columns.get_untracked();
    let header_table = data.clone();
    view! {
        <div class="test-resizable-container">
            <table {..props.into_attrs()} style="table-layout: fixed; width: min-content">
                <thead>
                    <tr {..use_table_header_row().into_attrs()}>
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
    let state = hook_table_state(rows, false, Signal::stored(HashSet::new()));
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

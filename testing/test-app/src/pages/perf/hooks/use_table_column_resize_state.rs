//! The column resize state of a table (`use_table_column_resize_state`) alone, for its
//! performance test (`leptonic/tests/perf_tests/hooks/use_table_column_resize_state.rs`): on the
//! state of the `use_table` fixture's table ([`super::use_table`]: [`SMALL`] rows "Row <i>" in the
//! columns Name, Value and Group, here resizable) with a table width of [`WIDE`] pixels, created
//! by a mounted component (mounted at first) that shows the column widths (`#perf-widths`, in
//! pixels, in column order) and the column being resized (`#perf-resizing`). Its controls
//! ([`Control`]) mount and unmount it, set the size while unmounted (`small`, `large`: [`LARGE`]
//! rows), change the table width (`narrow`: [`NARROW`] pixels, `wide`), resize the column Name to
//! 350 pixels (`resize-name`) and start and end a resize of it (`start-resize`, `end-resize`).

use leptonic::hooks::{
    collections::{Key, SelectionMode, SelectionOptions},
    grid::GridFocusMode,
    table::{
        TableColumnResizeState, TableState, UseTableColumnResizeStateInput, UseTableStateInput,
        use_table_column_resize_state, use_table_state,
    },
};
use leptos::prelude::*;

use super::use_table::{LARGE, SMALL, table_collection};
use crate::pages::perf::{Control, PerfControls};

/// The table width, in pixels.
pub const WIDE: f64 = 900.0;
/// The narrower table width, in pixels.
pub const NARROW: f64 = 600.0;

/// Creates the resize state (into `state`, while mounted) and shows it.
#[component]
fn PerfState(
    table: TableState,
    width: RwSignal<f64>,
    state: StoredValue<Option<TableColumnResizeState>>,
) -> impl IntoView {
    let resize = use_table_column_resize_state(UseTableColumnResizeStateInput {
        table_state: table,
        table_width: width.into(),
        default_width: None,
        default_min_width: None,
    });
    state.set_value(Some(resize));
    on_cleanup(move || state.set_value(None));
    let columns = table.columns;
    let widths = move || {
        columns.with(|columns| {
            columns
                .iter()
                .map(|column| resize.column_width(&column.key).to_string())
                .collect::<Vec<_>>()
                .join(" ")
        })
    };
    let resizing = move || {
        resize
            .resizing_column
            .with(|column| column.as_ref().map(ToString::to_string))
    };
    view! {
        <output id="perf-widths">{widths}</output>
        <output id="perf-resizing">{resizing}</output>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let rows = RwSignal::new(SMALL);
    let width = RwSignal::new(WIDE);
    let table = use_table_state(UseTableStateInput {
        table: table_collection(rows, true),
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
    let state = StoredValue::new(None::<TableColumnResizeState>);
    let name = Key::from("name");
    let start_name = name.clone();
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || rows.set(SMALL)),
        Control::action("large", move || rows.set(LARGE)),
        Control::action("narrow", move || width.set(NARROW)),
        Control::action("wide", move || width.set(WIDE)),
        Control::action("resize-name", move || {
            if let Some(resize) = state.get_value() {
                let _ = resize.update_resized_columns(&name, 350.0);
            }
        }),
        Control::action("start-resize", move || {
            if let Some(resize) = state.get_value() {
                resize.start_resize(start_name.clone());
            }
        }),
        Control::action("end-resize", move || {
            if let Some(resize) = state.get_value() {
                resize.end_resize();
            }
        }),
    ];
    view! {
        <h1>"use_table_column_resize_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <PerfState table width state />
        </Show>
    }
}

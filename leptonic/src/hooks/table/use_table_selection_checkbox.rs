// Upstream: react-aria/src/table/useTableSelectionCheckbox.ts @ 99e6102368
use leptos::prelude::*;

use super::TableData;
use crate::hooks::{
    UseGridSelectionCheckboxInput,
    collections::{Key, SelectionMode},
    form::{ToggleOptions, ToggleState, use_checkbox::UseCheckboxInput},
    use_grid_selection_checkbox,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Return the `UseCheckboxInput` for `use_checkbox`, with English labels ("Select",
//   "Select All").
//
// =============================================================================

/// A checkbox selecting a row of a table, labelled "Select" plus the row's row header cells.
pub fn use_table_selection_checkbox(table: &TableData, key: Key) -> UseCheckboxInput {
    let row_labelledby = table.row_labelledby(&key);
    let checkbox = use_grid_selection_checkbox(UseGridSelectionCheckboxInput {
        selection: table.state.grid.list.selection,
        key,
    });
    let mut checkbox = checkbox;
    let id = checkbox.options.id.clone().unwrap_or_default();
    checkbox.options.aria_labelledby = Some(format!("{id} {row_labelledby}"));
    checkbox
}

/// A checkbox selecting all rows of a table (in multiple selection mode), indeterminate when
/// some rows are selected.
pub fn use_table_select_all_checkbox(table: &TableData) -> UseCheckboxInput {
    let selection = table.state.grid.list.selection;
    let rows = table.state.table;
    let state = ToggleState::new(
        Signal::derive(move || selection.is_select_all()),
        false,
        Callback::new(move |_| selection.toggle_select_all()),
    );
    UseCheckboxInput {
        is_indeterminate: Signal::derive(move || {
            !selection.is_empty() && !selection.is_select_all()
        }),
        options: ToggleOptions {
            is_disabled: Signal::derive(move || {
                selection.selection_mode() != SelectionMode::Multiple
                    || rows.with(|t| t.size() == 0)
            }),
            aria_label: "Select All".into(),
            ..ToggleOptions::default()
        },
        state,
    }
}

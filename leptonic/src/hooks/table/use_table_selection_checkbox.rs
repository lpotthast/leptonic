// Upstream: react-aria/src/table/useTableSelectionCheckbox.ts @ 99e6102368
// Upstream: react-aria-components/test/Table.test.js @ 99e6102368
use leptos::prelude::*;

use super::TableData;
use crate::{
    ValueBinding,
    hooks::{
        collections::{Key, SelectionMode},
        form::{
            ToggleOptions, UseToggleStateInput, use_checkbox::UseCheckboxInput, use_toggle_state,
        },
        grid::{UseGridSelectionCheckboxInput, use_grid_selection_checkbox},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Returns the `UseCheckboxInput` for `use_checkbox`, to render the checkbox with it (input
//   composition: one checkbox hook per element instead of merged DOM props).
//
// =============================================================================

/// Input of [`use_table_selection_checkbox`].
#[derive(Debug, Clone)]
pub struct UseTableSelectionCheckboxInput {
    /// The table (from `use_table`).
    pub table: TableData,
    /// The row the checkbox selects.
    pub key: Key,
    /// Whether the row is selected: its `use_table_row`'s `is_selected`.
    pub is_selected: Signal<bool>,
}

/// A checkbox selecting a row of a table, labelled "Select" plus the row's row header cells.
pub fn use_table_selection_checkbox(input: UseTableSelectionCheckboxInput) -> UseCheckboxInput {
    crate::metrics::subject!();
    let UseTableSelectionCheckboxInput {
        table,
        key,
        is_selected,
    } = input;
    let mut checkbox = use_grid_selection_checkbox(UseGridSelectionCheckboxInput {
        selection: table.state.grid.list.selection,
        key: key.clone(),
        is_selected,
        strings: table.grid.strings,
    });
    // Its own label, then the row's (following the row header columns and cells).
    let own_id = checkbox.options.id.clone();
    checkbox.options.aria_labelledby = Memo::new(move |_| {
        crate::utils::id::join_ids([own_id.clone(), table.row_labelledby(&key)])
    })
    .into();
    checkbox
}

/// Input of [`use_table_select_all_checkbox`].
#[derive(Debug, Clone)]
pub struct UseTableSelectAllCheckboxInput {
    /// The table (from `use_table`).
    pub table: TableData,
}

/// A checkbox selecting all rows of a table (in multiple selection mode), indeterminate when
/// some rows are selected.
pub fn use_table_select_all_checkbox(input: UseTableSelectAllCheckboxInput) -> UseCheckboxInput {
    crate::metrics::subject!();
    let UseTableSelectAllCheckboxInput { table } = input;
    let selection = table.state.grid.list.selection;
    let rows = table.state.table;
    let strings = table.strings;
    // Compares the selection with every selectable key: once per selection change.
    let is_select_all = Memo::new(move |_| selection.is_select_all());
    let state = use_toggle_state(UseToggleStateInput {
        value: Some(ValueBinding::new(
            is_select_all.into(),
            Callback::new(move |_| selection.toggle_select_all()),
        )),
        ..UseToggleStateInput::default()
    });
    UseCheckboxInput {
        is_indeterminate: Signal::derive(move || !selection.is_empty() && !is_select_all.get()),
        options: ToggleOptions {
            is_disabled: Signal::derive(move || {
                selection.selection_mode() != SelectionMode::Multiple
                    || rows.with(|t| t.size() == 0)
            }),
            // "Select" in single selection mode, where it is disabled (as react-aria).
            aria_label: Signal::derive(move || {
                let strings = strings.read();
                Some(if selection.selection_mode() == SelectionMode::Single {
                    strings.select()
                } else {
                    strings.select_all()
                })
            })
            .into(),
            ..ToggleOptions::default()
        },
        state,
        validation: None,
    }
}

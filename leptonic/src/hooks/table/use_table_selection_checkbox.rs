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
    utils::intl_strings::{TableStrings, use_localized_strings},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Return the `UseCheckboxInput` for `use_checkbox`, to render the checkbox with it.
//
// =============================================================================

/// Input of [`use_table_selection_checkbox`].
#[derive(Debug, Clone)]
pub struct UseTableSelectionCheckboxInput {
    /// The table (from `use_table`).
    pub table: TableData,
    /// The row the checkbox selects.
    pub key: Key,
}

/// A checkbox selecting a row of a table, labelled "Select" plus the row's row header cells.
pub fn use_table_selection_checkbox(input: UseTableSelectionCheckboxInput) -> UseCheckboxInput {
    let UseTableSelectionCheckboxInput { table, key } = input;
    let row_labelledby = untrack(|| table.row_labelledby(&key));
    let checkbox = use_grid_selection_checkbox(UseGridSelectionCheckboxInput {
        selection: table.state.grid.list.selection,
        key,
    });
    let mut checkbox = checkbox;
    let id = checkbox.options.id.clone().unwrap_or_default();
    checkbox.options.aria_labelledby = Some(format!("{id} {row_labelledby}"));
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
    let UseTableSelectAllCheckboxInput { table } = input;
    let selection = table.state.grid.list.selection;
    let rows = table.state.table;
    let strings = use_localized_strings::<TableStrings>();
    let state = use_toggle_state(UseToggleStateInput {
        value: Some(ValueBinding::new(
            Signal::derive(move || selection.is_select_all()),
            Callback::new(move |_| selection.toggle_select_all()),
        )),
        ..UseToggleStateInput::default()
    });
    UseCheckboxInput {
        is_indeterminate: Signal::derive(move || {
            !selection.is_empty() && !selection.is_select_all()
        }),
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
    }
}

// Upstream: react-aria/src/grid/useGridSelectionCheckbox.ts @ 99e6102368
use leptos::prelude::*;

use crate::{
    ValueBinding,
    hooks::{
        collections::{Key, SelectionManager},
        form::{
            ToggleOptions, UseToggleStateInput, use_checkbox::UseCheckboxInput, use_toggle_state,
        },
    },
    utils::{id::use_id, intl_strings::GridStrings},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Returns the `UseCheckboxInput` for `use_checkbox`, to render the checkbox with it (input
//   composition: one checkbox hook per element instead of merged DOM props).
// - Takes the grid's localized messages (`strings`) instead of loading them per checkbox: one
//   memo per grid, not per row (`performance.md` R5). React-aria: `useLocalizedStringFormatter`
//   in every checkbox.
// - Takes the row's selection state (`is_selected`) instead of reading the selection per checkbox
//   (`performance.md` R5).
//
// =============================================================================

/// Input of [`use_grid_selection_checkbox`].
#[derive(Debug, Clone)]
pub struct UseGridSelectionCheckboxInput {
    pub selection: SelectionManager,
    /// The row the checkbox selects.
    pub key: Key,
    /// Whether the row is selected: its row hook's `is_selected` (`use_grid_row`,
    /// `use_table_row`, `use_grid_list_item`), which the checkbox shows instead of reading the
    /// selection again.
    pub is_selected: Signal<bool>,
    /// The grid's messages, shared by its checkboxes: `GridData::strings`, or
    /// `use_localized_strings::<GridStrings>()` once for all of them.
    pub strings: Memo<GridStrings>,
}

/// A checkbox selecting a row of a grid (or grid list, or table).
pub fn use_grid_selection_checkbox(input: UseGridSelectionCheckboxInput) -> UseCheckboxInput {
    crate::metrics::subject!();
    let UseGridSelectionCheckboxInput {
        selection,
        key,
        is_selected,
        strings,
    } = input;
    let key = StoredValue::new(key);
    // The selection lives in the collection: the checkbox shows the row's and toggles it.
    let state = use_toggle_state(UseToggleStateInput {
        value: Some(ValueBinding::new(
            is_selected,
            Callback::new(move |_| key.with_value(|k| selection.toggle_selection(k))),
        )),
        ..UseToggleStateInput::default()
    });
    UseCheckboxInput {
        options: ToggleOptions {
            id: Some(use_id("checkbox")),
            is_disabled: Memo::new(move |_| !key.with_value(|k| selection.can_select_item(k)))
                .into(),
            aria_label: Signal::derive(move || Some(strings.read().select())).into(),
            ..ToggleOptions::default()
        },
        state,
        is_indeterminate: Signal::stored(false),
        validation: None,
    }
}

// Upstream: react-aria/src/grid/useGridSelectionCheckbox.ts @ 99e6102368
use leptos::prelude::*;

use crate::{
    hooks::{
        collections::{Key, SelectionManager},
        form::{ToggleOptions, ToggleState, use_checkbox::UseCheckboxInput},
    },
    utils::id::use_id,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Returns the `UseCheckboxInput` for `use_checkbox`. The label ("Select") is English only.
//
// =============================================================================

/// Input of [`use_grid_selection_checkbox`].
#[derive(Debug, Clone)]
pub struct UseGridSelectionCheckboxInput {
    pub selection: SelectionManager,
    /// The row the checkbox selects.
    pub key: Key,
}

/// A checkbox selecting a row of a grid (or grid list, or table).
pub fn use_grid_selection_checkbox(input: UseGridSelectionCheckboxInput) -> UseCheckboxInput {
    let UseGridSelectionCheckboxInput { selection, key } = input;
    let key = StoredValue::new(key);
    // The selection lives in the collection: the checkbox reads it and toggles it.
    let state = ToggleState::new(
        Signal::derive(move || key.with_value(|k| selection.is_selected(k))),
        false,
        Callback::new(move |_| key.with_value(|k| selection.toggle_selection(k))),
    );
    UseCheckboxInput {
        options: ToggleOptions {
            id: Some(use_id("checkbox")),
            is_disabled: Signal::derive(move || !key.with_value(|k| selection.can_select_item(k))),
            aria_label: "Select".into(),
            ..ToggleOptions::default()
        },
        state,
        is_indeterminate: Signal::stored(false),
    }
}

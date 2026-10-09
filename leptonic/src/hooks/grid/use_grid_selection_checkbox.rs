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
    utils::{
        id::use_id,
        intl_strings::{GridStrings, use_localized_strings},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Returns the `UseCheckboxInput` for `use_checkbox`, to render the checkbox with it.
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
    let strings = use_localized_strings::<GridStrings>();
    // The selection lives in the collection: the checkbox reads it and toggles it.
    let state = use_toggle_state(UseToggleStateInput {
        value: Some(ValueBinding::new(
            Signal::derive(move || key.with_value(|k| selection.is_selected(k))),
            Callback::new(move |_| key.with_value(|k| selection.toggle_selection(k))),
        )),
        ..UseToggleStateInput::default()
    });
    UseCheckboxInput {
        options: ToggleOptions {
            id: Some(use_id("checkbox")),
            is_disabled: Signal::derive(move || !key.with_value(|k| selection.can_select_item(k))),
            aria_label: Signal::derive(move || Some(strings.read().select())).into(),
            ..ToggleOptions::default()
        },
        state,
        is_indeterminate: Signal::stored(false),
    }
}

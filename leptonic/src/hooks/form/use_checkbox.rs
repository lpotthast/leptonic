// Upstream: react-aria/src/checkbox/useCheckbox.ts @ 99e6102368
use leptos::prelude::*;
use web_sys::MouseEvent;

use super::{
    use_form_validation_state::UseFormValidationStateReturn,
    use_toggle::{ToggleOptions, UseToggleInput, UseToggleReturn, use_toggle_with},
    use_toggle_state::ToggleState,
};
use crate::{hooks::PropsWithStyles, utils::EventHandler};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `indeterminate` is part of the input's props (a DOM property) instead of being set in an
//   effect.
//
// =============================================================================

/// Input of [`use_checkbox`].
#[derive(Debug, Clone)]
pub struct UseCheckboxInput {
    pub state: ToggleState,
    /// Shows the checkbox as neither checked nor unchecked (e.g. "select all" with some rows
    /// selected). Purely visual: the state stays as it is.
    pub is_indeterminate: Signal<bool>,
    pub options: ToggleOptions,
}

/// Output of [`use_checkbox`].
pub type UseCheckboxReturn = UseToggleReturn;

/// Provides the behavior and accessibility of a checkbox: an `<input type="checkbox">` inside a
/// `<label>`.
///
/// ```ignore
/// let state = use_toggle_state(UseToggleStateInput::default());
/// let checkbox = use_checkbox(UseCheckboxInput {
///     state,
///     is_indeterminate: Signal::stored(false),
///     options: ToggleOptions::default(),
/// });
/// let (label_attrs, label_styles) = checkbox.label_props.into_parts();
/// let (input_attrs, input_styles) = checkbox.input_props.into_parts();
/// view! {
///     <label {..label_attrs} style=label_styles>
///         <input {..input_attrs} style=input_styles />
///         "Subscribe"
///     </label>
/// }
/// ```
pub fn use_checkbox(input: UseCheckboxInput) -> UseCheckboxReturn {
    use_checkbox_with(input, None)
}

/// [`use_checkbox`] with a checkbox group's validation (see `use_toggle_with`).
pub(crate) fn use_checkbox_with(
    input: UseCheckboxInput,
    group_validation: Option<UseFormValidationStateReturn>,
) -> UseCheckboxReturn {
    let UseCheckboxInput {
        state,
        is_indeterminate,
        options,
    } = input;
    let mut toggle = use_toggle_with(UseToggleInput { state, options }, None, group_validation);

    let (mut input_props, input_styles) = toggle.input_props.into_inner();
    input_props.indeterminate = is_indeterminate;
    toggle.input_props = PropsWithStyles::new(input_props, input_styles);

    // Pressing the label must not move focus away from the input before it is toggled.
    let (mut label_props, label_styles) = toggle.label_props.into_inner();
    label_props.on_mousedown = label_props
        .on_mousedown
        .chain(EventHandler::new(|e: MouseEvent| e.prevent_default()));
    toggle.label_props = PropsWithStyles::new(label_props, label_styles);
    toggle
}

// Upstream: react-aria/src/checkbox/useCheckbox.ts @ 99e6102368
// Upstream: react-aria-components/test/Checkbox.test.js @ 99e6102368
use leptos::prelude::*;
use web_sys::MouseEvent;

use super::{
    use_form_validation_state::FormValidationState,
    use_toggle::{ToggleOptions, ToggleRole, UseToggleInput, UseToggleReturn, use_toggle},
    use_toggle_state::ToggleState,
};
use crate::{EventHandler, PropsWithStyles};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `validation: Option<FormValidationState>`, the validation of a checkbox group's item, is an
//   input field (react-aria: the private `privateValidationStateProp` prop).
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
    /// A validation managed by someone else, used instead of the checkbox's own: a checkbox
    /// group item's (see [`use_checkbox_group_item`](fn@super::use_checkbox_group_item)).
    /// `None`: the checkbox validates itself.
    pub validation: Option<FormValidationState>,
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
///     validation: None,
///     options: ToggleOptions::default(),
/// });
/// let (label_attrs, label_styles) = checkbox.label.attrs();
/// let (input_attrs, input_styles) = checkbox.input.attrs();
/// view! {
///     <label {..label_attrs} style=label_styles>
///         <input {..input_attrs} style=input_styles />
///         "Subscribe"
///     </label>
/// }
/// ```
pub fn use_checkbox(input: UseCheckboxInput) -> UseCheckboxReturn {
    crate::metrics::subject!();
    let UseCheckboxInput {
        state,
        is_indeterminate,
        validation,
        options,
    } = input;
    let mut toggle = use_toggle(UseToggleInput {
        state,
        role: ToggleRole::Checkbox,
        validation,
        options,
    });

    toggle.input = toggle.input.map(move |props| {
        let (mut input_props, input_styles) = props.into_inner();
        input_props.indeterminate = is_indeterminate;
        PropsWithStyles::new(input_props, input_styles)
    });

    // Pressing the label must not move focus away from the input before it is toggled.
    let prevent_default = EventHandler::new(|e: MouseEvent| e.prevent_default());
    toggle.label = toggle.label.map(move |props| {
        let (mut label_props, label_styles) = props.into_inner();
        label_props.on_mousedown = label_props.on_mousedown.chain(prevent_default.clone());
        PropsWithStyles::new(label_props, label_styles)
    });
    toggle
}

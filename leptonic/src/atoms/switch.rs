// Upstream: react-aria-components/src/Switch.tsx @ 99e6102368
use leptos::prelude::*;

use super::form::use_validation_behavior;
use crate::{
    Out,
    hooks::{
        IntoAttrs, ToggleOptions, UseHoverInput, UseSwitchInput, UseToggleStateInput, ValidateFn,
        ValidationBehavior, use_hover, use_switch, use_toggle_state,
    },
    utils::{
        ValueBinding, classes::Classes, data_attributes::flag, default_class::with_default_class,
        styles::Styles, visually_hidden::visually_hidden_styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Selection (C4): `default_selected` + `on_change`, or `is_selected` + `set_selected`
//   (react-aria: `isSelected` + `onChange`).
// - Render props become `data-*` attributes plus plain children.
//
// =============================================================================

/// A headless switch: a `<label>` around a visually hidden `<input type="checkbox"
/// role="switch">` and the children (draw the track with them, styled through the label's
/// data attributes).
///
/// Data attributes: `data-selected`, `data-pressed`, `data-hovered`, `data-focused`,
/// `data-focus-visible`, `data-disabled`, `data-readonly`, `data-invalid`, `data-required`.
///
/// Default class: `leptonic-Switch`.
#[allow(clippy::too_many_arguments)]
#[component]
pub fn Switch(
    #[prop(optional)] default_selected: bool,
    /// Called when the switch is turned on or off.
    #[prop(into, optional)]
    on_change: Option<Callback<bool>>,
    /// Whether the toggle is selected (controlled): a value or any signal.
    #[prop(into, optional)]
    is_selected: Option<Signal<bool>>,
    /// Receives the selection: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_selected: Option<Out<bool>>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] is_read_only: Signal<bool>,
    #[prop(into, optional)] is_required: Signal<bool>,
    #[prop(into, optional)] is_invalid: Signal<bool>,
    #[prop(optional)] validate: Option<ValidateFn<bool>>,
    /// Default: the surrounding [`Form`](super::form::Form)'s, else `Native`.
    #[prop(optional)]
    validation_behavior: Option<ValidationBehavior>,
    #[prop(into, optional)] name: Option<String>,
    /// The input's `value` (submitted while on).
    #[prop(into, optional)]
    form_value: Option<String>,
    #[prop(into, optional)] form: Option<String>,
    /// The input's id.
    #[prop(into, optional)]
    id: Option<String>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] aria_describedby: Option<String>,
    #[prop(optional)] auto_focus: bool,
    #[prop(into, optional)] on_focus_change: Option<Callback<bool>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let classes = with_default_class("leptonic-Switch", classes);
    let validation_behavior = use_validation_behavior(validation_behavior);
    let (value, on_change) = ValueBinding::from_state_props(is_selected, set_selected, on_change);
    let state = use_toggle_state(UseToggleStateInput {
        default_selected,
        value,
        on_change,
        is_read_only,
    });
    let switch = use_switch(UseSwitchInput {
        options: ToggleOptions {
            id,
            is_disabled,
            is_read_only,
            is_required,
            is_invalid,
            validate,
            validation_behavior,
            name,
            form,
            value: form_value,
            aria_label,
            aria_labelledby,
            aria_describedby,
            auto_focus,
            on_focus_change,
            ..ToggleOptions::default()
        },
        state,
    });
    let hover = use_hover(UseHoverInput {
        is_disabled: switch.is_disabled,
        ..UseHoverInput::default()
    });

    let (label_attrs, label_styles) = switch.label_props.into_parts();
    let (input_attrs, input_styles) = switch.input_props.into_parts();

    view! {
        <label
            {..label_attrs}
            {..hover.props.into_attrs()}
            class=classes
            style=label_styles.merge(styles)
            data-selected=flag(switch.is_selected)
            data-pressed=flag(switch.is_pressed)
            data-hovered=flag(hover.is_hovered)
            data-focused=flag(switch.is_focused)
            data-focus-visible=flag(switch.is_focus_visible)
            data-disabled=flag(switch.is_disabled)
            data-readonly=flag(switch.is_read_only)
            data-invalid=flag(switch.is_invalid)
            data-required=flag(is_required)
        >
            <input {..input_attrs} style=input_styles.merge(visually_hidden_styles()) />
            {children.map(|children| children())}
        </label>
    }
}

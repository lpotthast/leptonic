// Upstream: react-aria-components/src/RadioGroup.tsx @ 99e6102368
use leptos::{context::Provider, prelude::*};

use super::{
    field::{FieldContext, LabelContext},
    form::use_validation_behavior,
};
use crate::{
    Out,
    atoms::field::LabelPresence,
    hooks::{
        IntoAttrs, Orientation, RadioGroupData, UseHoverInput, UseRadioGroupInput,
        UseRadioGroupReturn, UseRadioGroupStateInput, UseRadioInput, ValidateFn,
        ValidationBehavior, collections::Key, use_hover, use_radio, use_radio_group,
        use_radio_group_state,
    },
    utils::{
        classes::Classes, data_attributes::flag, styles::Styles,
        visually_hidden::visually_hidden_styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The selected value is split into `value` (a value or any signal) and `set_value` (an `Out`),
//   plus `default_value` and `on_change` (C4; react-aria: controlled/uncontrolled `value`).
// - Values are collection `Key`s (react-aria: strings).
// - Render props become `data-*` attributes plus plain children.
//
// =============================================================================

/// Context from [`RadioGroup`] to its radios.
#[derive(Clone)]
pub struct RadioGroupCtx {
    pub data: RadioGroupData,
    pub is_invalid: Signal<bool>,
}

/// A headless radio group (`role="radiogroup"`): one of its [`Radio`]s is selected, the arrow
/// keys move the selection. Label it with a [`Label`](super::field::Label) (or `aria_label`), and
/// add a [`Description`](super::field::Description) and [`FieldError`](super::field::FieldError)
/// as needed.
///
/// Data attributes: `data-orientation`, `data-disabled`, `data-readonly`, `data-required`,
/// `data-invalid`.
#[allow(clippy::too_many_arguments)]
#[component]
pub fn RadioGroup(
    #[prop(into, optional)] default_value: Option<Key>,
    /// The selected value (controlled): a value or any signal.
    #[prop(into, optional)]
    value: Option<Signal<Option<Key>>>,
    /// Receives the new state: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_value: Option<Out<Option<Key>>>,
    #[prop(into, optional)] on_change: Option<Callback<Option<Key>>>,
    /// The group's layout, announced as `aria-orientation` (default vertical). All arrow keys
    /// move the selection; in a horizontal group, Left/Right follow the writing direction.
    #[prop(default = Orientation::Vertical)]
    orientation: Orientation,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] is_read_only: Signal<bool>,
    #[prop(into, optional)] is_required: Signal<bool>,
    #[prop(into, optional)] is_invalid: Signal<bool>,
    #[prop(optional)] validate: Option<ValidateFn<Option<Key>>>,
    /// Default: the surrounding [`Form`](super::form::Form)'s, else `Native`.
    #[prop(optional)]
    validation_behavior: Option<ValidationBehavior>,
    /// The radios' `name`. Generated when `None`.
    #[prop(into, optional)]
    name: Option<String>,
    #[prop(into, optional)] form: Option<String>,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] aria_describedby: Option<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let validation_behavior = use_validation_behavior(validation_behavior);
    let (value, on_change) =
        crate::utils::ValueBinding::from_state_props(value, set_value, on_change);
    let state = use_radio_group_state(UseRadioGroupStateInput {
        default_value,
        value,
        on_change,
        name,
        is_disabled,
        is_read_only,
        is_required,
        is_invalid,
        validate,
        validation_behavior,
    });
    // As in react-aria-components: a visible label is expected unless an ARIA label is given.
    let label_presence = LabelPresence::new(aria_label, aria_labelledby.as_ref());
    let has_label = label_presence.has_label;
    let UseRadioGroupReturn {
        props,
        label_props,
        description_props,
        error_message_props,
        data,
        is_invalid,
        validation_errors,
        validation_details,
        ..
    } = use_radio_group(UseRadioGroupInput {
        id,
        has_label,
        aria_label,
        aria_labelledby,
        aria_describedby,
        orientation,
        form,
        ..UseRadioGroupInput::new(state)
    });
    let ctx = RadioGroupCtx { data, is_invalid };
    let label = LabelContext::span(label_props).with_presence(label_presence);
    let field = FieldContext {
        description: description_props,
        error_message: error_message_props,
        is_invalid,
        validation_errors,
        validation_details,
    };

    view! {
        <Provider value=ctx>
            <Provider value=label><Provider value=field>
                <div
                    {..props.into_attrs()}
                    class=classes
                    style=styles
                    data-orientation=orientation.as_str()
                    data-disabled=flag(state.is_disabled)
                    data-readonly=flag(state.is_read_only)
                    data-required=flag(state.is_required)
                    data-invalid=flag(is_invalid)
                >
                    {children()}
                </div>
            </Provider></Provider>
        </Provider>
    }
}

/// A headless radio in a [`RadioGroup`]: a `<label>` around a visually hidden
/// `<input type="radio">` and the children.
///
/// Data attributes: `data-selected`, `data-pressed`, `data-hovered`, `data-focused`,
/// `data-focus-visible`, `data-disabled`, `data-readonly`, `data-invalid`, `data-required`.
#[component]
pub fn Radio(
    /// The value the radio selects.
    #[prop(into)]
    value: Key,
    #[prop(into, optional)] is_disabled: Signal<bool>,
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
    let group = expect_context::<RadioGroupCtx>();
    let state = group.data.state;
    let radio = use_radio(UseRadioInput {
        is_disabled,
        id,
        aria_label,
        aria_labelledby,
        aria_describedby,
        auto_focus,
        on_focus_change,
        ..UseRadioInput::new(group.data, value)
    });
    let hover = use_hover(UseHoverInput {
        is_disabled: radio.is_disabled,
        ..UseHoverInput::default()
    });

    let (label_attrs, label_styles) = radio.label_props.into_parts();
    let (input_attrs, input_styles) = radio.input_props.into_parts();

    view! {
        <label
            {..label_attrs}
            {..hover.props.into_attrs()}
            class=classes
            style=label_styles.merge(styles)
            data-selected=flag(radio.is_selected)
            data-pressed=flag(radio.is_pressed)
            data-hovered=flag(hover.is_hovered)
            data-focused=flag(radio.is_focused)
            data-focus-visible=flag(radio.is_focus_visible)
            data-disabled=flag(radio.is_disabled)
            data-readonly=flag(state.is_read_only)
            data-invalid=flag(group.is_invalid)
            data-required=flag(state.is_required)
        >
            <input {..input_attrs} style=input_styles.merge(visually_hidden_styles()) />
            {children.map(|children| children())}
        </label>
    }
}

// Upstream: react-aria-components/src/RadioGroup.tsx @ 99e6102368
// Upstream: react-aria-components/test/RadioGroup.test.js @ 99e6102368
use leptos::{context::Provider, prelude::*};
use leptos_classes::Classes;

use super::{
    field::{FieldContext, LabelContext},
    form::use_validation_behavior,
    typed_values::{KeyedStateProps, keyed_state_props},
};
use crate::{
    IntoAttrs, Orientation, Out,
    atoms::field::LabelPresence,
    hooks::{
        collections::{Key, SelectionValue},
        form::{
            RadioGroupData, UseRadioGroupInput, UseRadioGroupReturn, UseRadioGroupStateInput,
            UseRadioInput, UseRadioReturn, ValidateFn, ValidationBehavior, use_radio,
            use_radio_group, use_radio_group_state,
        },
        interactions::{UseHoverInput, use_hover},
    },
    utils::{
        data_attributes::flag, default_class::with_default_class, dev_warn, styles::Styles,
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
// - The value is typed (`V: SelectionValue`: an enum with `selection_value!`, `String`, an
//   integer, a `Key`); radios take theirs as a `Key` (`From<V> for Key`). React-aria: strings.
// - Render props become `data-*` attributes plus plain children.
// - No single `Radio` (react-aria-components deprecates it): a radio is a `RadioField` with its
//   `RadioButton`.
//
// =============================================================================

/// Context from [`RadioGroup`] to its radios.
#[derive(Clone)]
pub struct RadioGroupContext {
    pub data: RadioGroupData,
    pub is_invalid: Signal<bool>,
}

/// A headless radio group (`role="radiogroup"`): one of its [`RadioField`]s is selected, the arrow
/// keys move the selection. Label it with a [`Label`](super::field::Label) (or `aria_label`), and
/// add a [`Description`](super::field::Description) and [`FieldError`](super::field::FieldError)
/// as needed.
///
/// Data attributes: `data-orientation`, `data-disabled`, `data-readonly`, `data-required`,
/// `data-invalid`.
///
/// Default class: `leptonic-RadioGroup`.
#[allow(clippy::too_many_arguments)]
#[component]
pub fn RadioGroup<V: SelectionValue>(
    /// The initially selected value. Ignored with `value`.
    #[prop(optional)]
    default_value: Option<V>,
    /// The selected value (controlled): a value or any signal.
    #[prop(into, optional)]
    value: Option<Signal<Option<V>>>,
    /// Receives the new state: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_value: Option<Out<Option<V>>>,
    #[prop(into, optional)] on_change: Option<Callback<Option<V>>>,
    /// The group's layout, announced as `aria-orientation` (default vertical). All arrow keys
    /// move the selection; in a horizontal group, Left/Right follow the writing direction.
    #[prop(into, default = Orientation::Vertical.into())]
    orientation: Signal<Orientation>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] is_read_only: Signal<bool>,
    #[prop(into, optional)] is_required: Signal<bool>,
    #[prop(into, optional)] is_invalid: Signal<bool>,
    #[prop(optional)] validate: Option<ValidateFn<Option<V>>>,
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
    let classes = with_default_class("leptonic-RadioGroup", classes);
    let validation_behavior = use_validation_behavior(validation_behavior);
    let KeyedStateProps {
        default_value,
        value,
        set_value,
        on_change,
        validate,
    } = keyed_state_props(Some(default_value), value, set_value, on_change, validate);
    let (value, on_change) = crate::ValueBinding::from_state_props(value, set_value, on_change);
    let state = use_radio_group_state(UseRadioGroupStateInput {
        default_value: default_value.flatten(),
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
        state,
        aria_errormessage: None,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });
    let ctx = RadioGroupContext { data, is_invalid };
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
                    data-orientation=move || orientation.get().as_str()
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

/// A headless radio in a [`RadioGroup`] with a description of its own: a `<div>` around a
/// [`RadioButton`] (the clickable `<label>`) and a [`Description`](super::field::Description).
/// A [`FieldError`](super::field::FieldError) in it shows the group's errors.
///
/// Data attributes: `data-selected`, `data-disabled`, `data-readonly`, `data-invalid`,
/// `data-required`.
///
/// Default class: `leptonic-RadioField`.
#[component]
pub fn RadioField(
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
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-RadioField", classes);
    let group = expect_context::<RadioGroupContext>();
    let state = group.data.state;
    let radio = use_radio_atom(
        &group,
        RadioSetup {
            value,
            is_disabled,
            id,
            aria_label,
            aria_labelledby,
            aria_describedby,
            auto_focus,
            on_focus_change,
        },
    );
    let (is_selected, is_disabled) = (radio.is_selected, radio.is_disabled);
    // The radio's description; errors are the group's (react-aria-components keeps the group's
    // `FieldErrorContext`).
    let field = FieldContext {
        description: radio.description_props.clone(),
        ..expect_context::<FieldContext>()
    };
    let button = RadioButtonContext {
        radio: StoredValue::new(Some(radio)),
    };

    view! {
        <Provider value=button>
            <Provider value=field>
                <div
                    class=classes
                    style=styles
                    data-selected=flag(is_selected)
                    data-disabled=flag(is_disabled)
                    data-readonly=flag(state.is_read_only)
                    data-invalid=flag(group.is_invalid)
                    data-required=flag(state.is_required)
                >
                    {children()}
                </div>
            </Provider>
        </Provider>
    }
}

/// The clickable part of a [`RadioField`]: a `<label>` around a visually hidden
/// `<input type="radio">` and the children.
///
/// Data attributes: `data-selected`, `data-pressed`, `data-hovered`, `data-focused`,
/// `data-focus-visible`, `data-disabled`, `data-readonly`, `data-invalid`, `data-required`.
///
/// Default class: `leptonic-RadioButton`.
#[component]
pub fn RadioButton(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let classes = with_default_class("leptonic-RadioButton", classes);
    let (Some(group), Some(ctx)) = (
        use_context::<RadioGroupContext>(),
        use_context::<RadioButtonContext>(),
    ) else {
        dev_warn!("a <RadioButton> belongs in a <RadioField> in a <RadioGroup>");
        return ().into_any();
    };
    let Some(radio) = ctx.radio.try_update_value(Option::take).flatten() else {
        dev_warn!("a <RadioField> has one <RadioButton>");
        return ().into_any();
    };
    radio_button(group, radio, classes, styles, children).into_any()
}

/// What a [`RadioField`] hands its [`RadioButton`].
#[derive(Clone)]
struct RadioButtonContext {
    /// The radio, taken by the button.
    radio: StoredValue<Option<UseRadioReturn>>,
}

/// The settings of a [`RadioField`].
struct RadioSetup {
    value: Key,
    is_disabled: Signal<bool>,
    id: Option<String>,
    aria_label: MaybeProp<String>,
    aria_labelledby: Option<String>,
    aria_describedby: Option<String>,
    auto_focus: bool,
    on_focus_change: Option<Callback<bool>>,
}

/// The radio of a [`RadioField`] in `group`.
fn use_radio_atom(group: &RadioGroupContext, setup: RadioSetup) -> UseRadioReturn {
    let RadioSetup {
        value,
        is_disabled,
        id,
        aria_label,
        aria_labelledby,
        aria_describedby,
        auto_focus,
        on_focus_change,
    } = setup;
    use_radio(UseRadioInput {
        is_disabled,
        id,
        aria_label,
        aria_labelledby,
        aria_describedby,
        auto_focus,
        on_focus_change,
        group: group.data,
        value,
        on_focus: None,
        on_blur: None,
        on_press_start: None,
        on_press_end: None,
        on_press_up: None,
        on_press: None,
        on_press_change: None,
    })
}

/// The `<label>` of a [`RadioButton`].
fn radio_button(
    group: RadioGroupContext,
    radio: UseRadioReturn,
    classes: Classes,
    styles: Styles,
    children: Option<Children>,
) -> impl IntoView {
    let state = group.data.state;
    let is_disabled = radio.is_disabled;
    let hover = use_hover(UseHoverInput {
        is_disabled: Signal::derive(move || is_disabled.get() || state.is_read_only.get()),
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

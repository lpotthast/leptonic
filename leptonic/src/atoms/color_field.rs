//! Headless color field atoms: a color as hex text, or one channel of a color as a number.
// Upstream: react-aria-components/src/ColorField.tsx @ 99e6102368

use leptos::prelude::*;

use super::{
    color_picker::ColorPickerContext,
    field::{FieldContext, LabelContext, LabelPresence},
    form::use_validation_behavior,
    input::{InputContext, InputState},
};
use crate::{
    Out,
    hooks::{
        IntoAttrs, UseColorChannelFieldInput, UseColorChannelFieldStateInput, UseColorFieldInput,
        UseColorFieldReturn, UseColorFieldStateInput, UseNumberFieldInput, UseNumberFieldReturn,
        ValidateFn, ValidationBehavior, use_color_channel_field, use_color_channel_field_state,
        use_color_field, use_color_field_state,
    },
    utils::{
        ValueBinding,
        classes::Classes,
        color::{ColorChannel, RGB8},
        data_attributes::flag,
        default_class::with_default_class,
        scoped_context::scoped_view,
        styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Two atoms: `ColorField` for hex text (an `RGB8`) and `ColorChannelField` for one channel
//   (react-aria-components: one `ColorField` with an optional `channel` and `colorSpace`).
// - The value is split into `value` (a value or any signal) and `set_value` (an `Out`), plus
//   `default_value` and `on_change` (C4).
//
// =============================================================================

/// A field for a color as hex text: a `Label`, an `Input` and optionally a `Description` and a
/// `FieldError`.
///
/// Data attributes: `data-disabled`, `data-readonly`, `data-required`, `data-invalid`.
///
/// Default class: `leptonic-ColorField`.
#[component]
#[allow(clippy::too_many_arguments)]
pub fn ColorField(
    /// The initial color (`None`: empty).
    #[prop(optional)]
    default_value: Option<RGB8>,
    /// The color (controlled): a value or any signal. Default: the `ColorPicker`'s around it.
    #[prop(into, optional)]
    value: Option<Signal<Option<RGB8>>>,
    /// Receives the committed color: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_value: Option<Out<Option<RGB8>>>,
    /// Called with the committed color.
    #[prop(into, optional)]
    on_change: Option<Callback<Option<RGB8>>>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] is_read_only: Signal<bool>,
    #[prop(into, optional)] is_required: Signal<bool>,
    #[prop(into, optional)] is_invalid: Signal<bool>,
    #[prop(optional)] validate: Option<ValidateFn<Option<RGB8>>>,
    #[prop(optional)] validation_behavior: Option<ValidationBehavior>,
    /// The name of a hidden input with the color's hex text, for forms.
    #[prop(into, optional)]
    name: Option<String>,
    #[prop(into, optional)] form: Option<String>,
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(optional)] auto_focus: bool,
    /// Whether the scroll wheel leaves the color alone (it steps while the field has focus).
    #[prop(optional)]
    is_wheel_disabled: bool,
    /// The input's id.
    #[prop(into, optional)]
    id: Option<String>,
    /// Labels the field when there is no `Label`.
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] aria_describedby: Option<String>,
    #[prop(into, optional)] on_focus_change: Option<Callback<bool>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-ColorField", classes);
    let (value, on_change) = ValueBinding::from_state_props(value, set_value, on_change);
    let value = value.or_else(ColorPickerContext::optional_binding::<RGB8>);
    let state = use_color_field_state(UseColorFieldStateInput {
        default_value,
        value,
        is_invalid,
        validate,
        validation_behavior: use_validation_behavior(validation_behavior),
        name: name.clone(),
        on_change,
    });
    let label_presence = LabelPresence::new(aria_label, aria_labelledby.as_ref());
    let UseColorFieldReturn {
        label_props,
        input_props,
        description_props,
        error_message_props,
        is_focused,
        is_focus_visible,
        is_invalid,
        validation_errors,
        validation_details,
        ..
    } = use_color_field(UseColorFieldInput {
        id,
        has_label: label_presence.has_label,
        aria_label,
        aria_labelledby,
        aria_describedby,
        is_disabled,
        is_read_only,
        is_required,
        placeholder,
        auto_focus,
        is_wheel_disabled,
        on_focus_change,
        state,
        on_focus: None,
        on_blur: None,
        on_key_down: None,
        on_key_up: None,
    });
    let provide = move || {
        provide_context(LabelContext::label(label_props).with_presence(label_presence));
        provide_context(FieldContext {
            description: description_props,
            error_message: error_message_props,
            is_invalid,
            validation_errors,
            validation_details,
        });
        provide_context(InputContext::new(
            move || input_props.clone().into_attrs(),
            InputState {
                is_disabled,
                is_invalid,
                is_focused,
                is_focus_visible,
            },
        ));
    };
    let hidden_value = move || {
        state
            .color_value
            .get()
            .map_or_else(String::new, |color| format!("#{color:X}"))
    };
    scoped_view(provide, move || {
        view! {
            <div
                class=classes
                style=styles
                data-disabled=flag(is_disabled)
                data-readonly=flag(is_read_only)
                data-required=flag(is_required)
                data-invalid=flag(is_invalid)
            >
                {children()}
                {name.map(|name| view! {
                    <input
                        type="hidden"
                        name=name
                        form=form
                        disabled=is_disabled
                        prop:value=hidden_value
                        value=hidden_value
                    />
                })}
            </div>
        }
    })
}

/// A field for one channel of a color, as a number: a `Label`, an `Input` and optionally a
/// `Description` and a `FieldError`. Without a label, the channel's name labels it.
///
/// Data attributes: `data-disabled`, `data-readonly`, `data-required`, `data-invalid`.
///
/// Default class: `leptonic-ColorChannelField`.
#[component]
#[allow(clippy::too_many_arguments)]
pub fn ColorChannelField<Ch: ColorChannel<Color: Default>>(
    /// The channel the field edits.
    channel: Ch,
    /// The initial color (`None`: empty).
    #[prop(optional)]
    default_value: Option<Ch::Color>,
    /// The color (controlled): a value or any signal. Default: the `ColorPicker`'s around it.
    #[prop(into, optional)]
    value: Option<Signal<Option<Ch::Color>>>,
    /// Receives the color: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_value: Option<Out<Option<Ch::Color>>>,
    /// Called with the color when the channel's value is committed.
    #[prop(into, optional)]
    on_change: Option<Callback<Option<Ch::Color>>>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] is_read_only: Signal<bool>,
    #[prop(into, optional)] is_required: Signal<bool>,
    #[prop(into, optional)] is_invalid: Signal<bool>,
    #[prop(optional)] validate: Option<ValidateFn<Option<f64>>>,
    #[prop(optional)] validation_behavior: Option<ValidationBehavior>,
    /// The name of a hidden input with the channel's value, for forms.
    #[prop(into, optional)]
    name: Option<String>,
    #[prop(into, optional)] form: Option<String>,
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    /// The input's id.
    #[prop(into, optional)]
    id: Option<String>,
    /// Labels the field when there is no `Label`.
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] aria_describedby: Option<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-ColorChannelField", classes);
    let (value, on_change) = ValueBinding::from_state_props(value, set_value, on_change);
    let value = value.or_else(ColorPickerContext::optional_binding::<Ch::Color>);
    let state = use_color_channel_field_state(UseColorChannelFieldStateInput {
        default_value,
        value,
        is_disabled,
        is_read_only,
        is_invalid,
        validate,
        validation_behavior: use_validation_behavior(validation_behavior),
        name: name.clone(),
        on_change,
        channel,
    });
    let label_presence = LabelPresence::new(aria_label, aria_labelledby.as_ref());
    let mut input = UseColorChannelFieldInput {
        state,
        field: UseNumberFieldInput {
            state: state.number,
            id: None,
            has_label: Signal::stored(false),
            aria_label: MaybeProp::default(),
            aria_labelledby: None,
            aria_describedby: None,
            is_required: Signal::stored(false),
            placeholder: MaybeProp::default(),
            auto_focus: false,
            is_wheel_disabled: false,
            increment_aria_label: MaybeProp::default(),
            decrement_aria_label: MaybeProp::default(),
            on_focus: None,
            on_blur: None,
            on_focus_change: None,
            on_key_down: None,
            on_key_up: None,
        },
    };
    input.field.id = id;
    input.field.has_label = label_presence.has_label;
    input.field.aria_label = aria_label;
    input.field.aria_labelledby = aria_labelledby;
    input.field.aria_describedby = aria_describedby;
    input.field.is_required = is_required;
    input.field.placeholder = placeholder;
    let UseNumberFieldReturn {
        label_props,
        input_props,
        description_props,
        error_message_props,
        is_focused,
        is_focus_visible,
        is_invalid,
        validation_errors,
        validation_details,
        ..
    } = use_color_channel_field(input);
    let provide = move || {
        provide_context(LabelContext::label(label_props).with_presence(label_presence));
        provide_context(FieldContext {
            description: description_props,
            error_message: error_message_props,
            is_invalid,
            validation_errors,
            validation_details,
        });
        provide_context(InputContext::new(
            move || input_props.clone().into_attrs(),
            InputState {
                is_disabled,
                is_invalid,
                is_focused,
                is_focus_visible,
            },
        ));
    };
    let number = state.number.number_value;
    let hidden_value = move || number.get().map_or_else(String::new, |v| v.to_string());
    scoped_view(provide, move || {
        view! {
            <div
                class=classes
                style=styles
                data-disabled=flag(is_disabled)
                data-readonly=flag(is_read_only)
                data-required=flag(is_required)
                data-invalid=flag(is_invalid)
            >
                {children()}
                {name.map(|name| view! {
                    <input
                        type="hidden"
                        name=name
                        form=form
                        disabled=is_disabled
                        prop:value=hidden_value
                        value=hidden_value
                    />
                })}
            </div>
        }
    })
}

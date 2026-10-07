// Upstream: react-aria-components/src/NumberField.tsx @ 99e6102368
use leptos::prelude::*;

use super::{
    field::{FieldContext, LabelContext},
    form::use_validation_behavior,
    input::{InputContext, InputState},
};
use crate::{
    Out,
    atoms::field::LabelPresence,
    hooks::{
        CommitBehavior, IntoAttrs, UseButtonInput, UseFocusRingInput, UseHoverInput,
        UseNumberFieldGroupProps, UseNumberFieldInput, UseNumberFieldReturn,
        UseNumberFieldStateInput, ValidateFn, ValidationBehavior, use_button, use_focus_ring,
        use_hover, use_number_field, use_number_field_state,
    },
    utils::{
        NumberValue, ValueBinding, classes::Classes, data_attributes::flag,
        default_class::with_default_class, number_formatter::NumberFormatOptions,
        number_value::OptionalNumberSignal, scoped_context::scoped_view, styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Generic over the value type (C15): `NumberField<u8>`, `NumberField<f64>`, ..., usually
//   inferred from `default_value` or `value` (an `OptionalNumberSignal`).
// - Value (C4): `default_value` + `on_change`, or `value` + `set_value` (react-aria: `value` +
//   `onChange`).
// - The group and the stepper buttons are the `NumberFieldGroup`, `NumberFieldIncrementButton`
//   and `NumberFieldDecrementButton` parts (react-aria-components: `Group` and `Button`s with
//   `slot="increment"`/`"decrement"`). Reason: Leptos contexts are typed; dedicated parts need
//   no generic group/button contexts with slot strings.
// - Render props become `data-*` attributes plus plain children.
//
// =============================================================================

/// Context from [`NumberField`] to its group and buttons.
#[derive(Clone)]
struct NumberFieldCtx {
    group: StoredValue<UseNumberFieldGroupProps>,
    increment: StoredValue<UseButtonInput>,
    decrement: StoredValue<UseButtonInput>,
    is_disabled: Signal<bool>,
    is_invalid: Signal<bool>,
}

/// A headless number field for values of type `T` (any primitive integer or float). Compose it
/// from a [`Label`](super::field::Label), a [`NumberFieldGroup`] holding an
/// [`Input`](super::input::Input) and the [`NumberFieldDecrementButton`] and
/// [`NumberFieldIncrementButton`], a [`Description`](super::field::Description) and a
/// [`FieldError`](super::field::FieldError).
///
/// With a `name`, a hidden input submits the value with a form.
///
/// Data attributes: `data-disabled`, `data-readonly`, `data-required`, `data-invalid`.
///
/// Default class: `leptonic-NumberField`.
#[allow(clippy::too_many_arguments, clippy::needless_pass_by_value)]
#[component]
pub fn NumberField<T: NumberValue>(
    /// The initial value (`None`: empty).
    #[prop(optional)]
    default_value: Option<T>,
    /// The value (controlled), replacing `default_value`: a number, an `Option` (`None`: empty),
    /// or any signal of them.
    #[prop(into, optional)]
    value: Option<OptionalNumberSignal<T>>,
    /// Receives the new state: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_value: Option<Out<Option<T>>>,
    /// Called when a committed value changes.
    #[prop(into, optional)]
    on_change: Option<Callback<Option<T>>>,
    #[prop(into, optional)] min_value: MaybeProp<T>,
    #[prop(into, optional)] max_value: MaybeProp<T>,
    /// The step of increments. Default: 1 (0.01 for percentages). Typed values snap to it only
    /// when it is set.
    #[prop(into, optional)]
    step: MaybeProp<T>,
    #[prop(into, optional)] format_options: Signal<NumberFormatOptions>,
    #[prop(optional)] commit_behavior: CommitBehavior,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] is_read_only: Signal<bool>,
    #[prop(into, optional)] is_required: Signal<bool>,
    #[prop(into, optional)] is_invalid: Signal<bool>,
    #[prop(optional)] validate: Option<ValidateFn<Option<T>>>,
    /// Default: the surrounding [`Form`](super::form::Form)'s, else `Native`.
    #[prop(optional)]
    validation_behavior: Option<ValidationBehavior>,
    /// The hidden input's `name`, submitting the value with a form.
    #[prop(into, optional)]
    name: Option<String>,
    #[prop(into, optional)] form: Option<String>,
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(optional)] auto_focus: bool,
    /// Whether the scroll wheel leaves the value alone.
    #[prop(optional)]
    is_wheel_disabled: bool,
    #[prop(into, optional)] increment_aria_label: MaybeProp<String>,
    #[prop(into, optional)] decrement_aria_label: MaybeProp<String>,
    /// The input's id.
    #[prop(into, optional)]
    id: Option<String>,
    /// Labels the field when there is no [`Label`](super::field::Label).
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] aria_describedby: Option<String>,
    #[prop(into, optional)] on_focus_change: Option<Callback<bool>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-NumberField", classes);
    let (value, on_change) = ValueBinding::from_state_props(
        value.map(OptionalNumberSignal::into_signal),
        set_value,
        on_change,
    );
    let state = use_number_field_state(UseNumberFieldStateInput {
        default_value,
        value,
        on_change,
        min_value: Signal::derive(move || min_value.get()),
        max_value: Signal::derive(move || max_value.get()),
        step: Signal::derive(move || step.get()),
        format_options,
        commit_behavior,
        is_disabled,
        is_read_only,
        is_invalid,
        validate,
        validation_behavior: use_validation_behavior(validation_behavior),
        name: name.clone(),
    });
    // As in react-aria-components: a visible label is expected unless an ARIA label is given.
    let label_presence = LabelPresence::new(aria_label, aria_labelledby.as_ref());
    let has_label = label_presence.has_label;
    let UseNumberFieldReturn {
        group_props,
        label_props,
        input_props,
        increment_button,
        decrement_button,
        description_props,
        error_message_props,
        is_focused,
        is_focus_visible,
        is_invalid,
        validation_errors,
        validation_details,
        ..
    } = use_number_field(UseNumberFieldInput {
        id,
        has_label,
        aria_label,
        aria_labelledby,
        aria_describedby,
        is_required,
        placeholder,
        auto_focus,
        is_wheel_disabled,
        increment_aria_label,
        decrement_aria_label,
        on_focus_change,
        state,
        on_focus: None,
        on_blur: None,
        on_key_down: None,
        on_key_up: None,
    });

    // Contexts for the children only, with the `<div>` as the root (getting attributes set on the
    // component).
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
        provide_context(NumberFieldCtx {
            group: StoredValue::new(group_props),
            increment: StoredValue::new(increment_button),
            decrement: StoredValue::new(decrement_button),
            is_disabled,
            is_invalid,
        });
    };
    let hidden_value = move || {
        state
            .number_value
            .get()
            .map_or_else(String::new, |v| v.to_string())
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

/// The group of the [`NumberField`]'s input and stepper buttons (`role="group"`).
///
/// Data attributes: `data-hovered`, `data-focus-within`, `data-focus-visible`, `data-disabled`,
/// `data-invalid`.
///
/// Default class: `leptonic-NumberFieldGroup`.
#[component]
pub fn NumberFieldGroup(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-NumberFieldGroup", classes);
    let ctx = expect_context::<NumberFieldCtx>();
    let ring = use_focus_ring(UseFocusRingInput {
        within: true,
        ..UseFocusRingInput::default()
    });
    let hover = use_hover(UseHoverInput {
        is_disabled: ctx.is_disabled,
        ..UseHoverInput::default()
    });

    view! {
        <div
            {..ctx.group.get_value().into_attrs()}
            {..ring.props.into_attrs()}
            {..hover.props.into_attrs()}
            class=classes
            style=styles
            data-hovered=flag(hover.is_hovered)
            data-focus-within=flag(ring.is_focused)
            data-focus-visible=flag(ring.is_focus_visible)
            data-disabled=flag(ctx.is_disabled)
            data-invalid=flag(ctx.is_invalid)
        >
            {children()}
        </div>
    }
}

/// The button incrementing the [`NumberField`] around it ("Increase <label>"; not in the tab
/// order, the arrow keys step from the input).
///
/// Data attributes: `data-pressed`, `data-hovered`, `data-focused`, `data-focus-visible`,
/// `data-disabled`.
///
/// Default class: `leptonic-NumberFieldIncrementButton`.
#[component]
pub fn NumberFieldIncrementButton(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-NumberFieldIncrementButton", classes);
    let ctx = expect_context::<NumberFieldCtx>();
    stepper_button(ctx.increment.get_value(), classes, styles, children)
}

/// The button decrementing the [`NumberField`] around it ("Decrease <label>").
///
/// Data attributes: `data-pressed`, `data-hovered`, `data-focused`, `data-focus-visible`,
/// `data-disabled`.
///
/// Default class: `leptonic-NumberFieldDecrementButton`.
#[component]
pub fn NumberFieldDecrementButton(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-NumberFieldDecrementButton", classes);
    let ctx = expect_context::<NumberFieldCtx>();
    stepper_button(ctx.decrement.get_value(), classes, styles, children)
}

fn stepper_button(
    input: UseButtonInput,
    classes: Classes,
    styles: Styles,
    children: Children,
) -> impl IntoView {
    let button = use_button(input);
    let (attrs, button_styles) = button.props.into_parts();

    view! {
        <button
            {..attrs}
            class=classes
            style=button_styles.merge(styles)
            data-pressed=flag(button.is_pressed)
            data-hovered=flag(button.is_hovered)
            data-focused=flag(button.is_focused)
            data-disabled=flag(button.is_disabled)
        >
            {children()}
        </button>
    }
}

use leptos::prelude::*;

use crate::{
    Out,
    atoms::{
        field::{Description, FieldError, Label, TextElement},
        input::Input,
        number_field::{
            NumberField as NumberFieldAtom, NumberFieldDecrementButton, NumberFieldGroup,
            NumberFieldIncrementButton, NumberFieldProps as NumberFieldAtomProps,
        },
    },
    components::icon::Icon,
    hooks::{CommitBehavior, ValidateFn, ValidationBehavior},
    utils::number_value::OptionalNumberSignal,
    utils::{NumberValue, classes::Classes, number_formatter::NumberFormatOptions, styles::Styles},
};

/// A number field for values of type `T` (any primitive integer or float) with its label,
/// description, stepper buttons and validation errors.
///
/// Its value starts at `default_value` and is reported through `on_change`; or it is `value`, and
/// changes go to `set_value` (e.g. both an `RwSignal<Option<T>>`).
#[allow(clippy::too_many_arguments, clippy::needless_pass_by_value)]
#[component]
pub fn NumberField<T: NumberValue>(
    /// The visible label. Without it, set `aria_label`.
    #[prop(into, optional)]
    label: MaybeProp<String>,
    #[prop(into, optional)] description: MaybeProp<String>,
    #[prop(optional)] default_value: Option<T>,
    /// The value (controlled), replacing `default_value`: a number, an `Option` (`None`: empty),
    /// or any signal of them.
    #[prop(into, optional)]
    value: Option<OptionalNumberSignal<T>>,
    /// Receives the new value: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_value: Option<Out<Option<T>>>,
    #[prop(into, optional)] on_change: Option<Callback<Option<T>>>,
    #[prop(into, optional)] min_value: MaybeProp<T>,
    #[prop(into, optional)] max_value: MaybeProp<T>,
    #[prop(into, optional)] step: MaybeProp<T>,
    #[prop(into, optional)] format_options: Signal<NumberFormatOptions>,
    #[prop(optional)] commit_behavior: CommitBehavior,
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] is_read_only: Signal<bool>,
    #[prop(into, optional)] is_required: Signal<bool>,
    #[prop(into, optional)] is_invalid: Signal<bool>,
    #[prop(optional)] validate: Option<ValidateFn<Option<T>>>,
    #[prop(optional)] validation_behavior: Option<ValidationBehavior>,
    #[prop(into, optional)] name: Option<String>,
    #[prop(optional)] auto_focus: bool,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] on_focus_change: Option<Callback<bool>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let children = move || {
        view! {
            {move || label.get().map(|label| view! { <Label classes="leptonic-field-label">{label}</Label> })}
            <NumberFieldGroup classes="leptonic-number-field-group">
                <Input classes="leptonic-text-field-input" />
                <NumberFieldDecrementButton classes="leptonic-number-field-step">
                    <Icon icon=icondata::BsDash />
                </NumberFieldDecrementButton>
                <NumberFieldIncrementButton classes="leptonic-number-field-step">
                    <Icon icon=icondata::BsPlus />
                </NumberFieldIncrementButton>
            </NumberFieldGroup>
            {move || description.get().map(|description| view! {
                <Description element=TextElement::Div classes="leptonic-field-description">
                    {description}
                </Description>
            })}
            <FieldError element=TextElement::Div classes="leptonic-field-error" />
        }
        .into_any()
    };
    let mut props = NumberFieldAtomProps::builder()
        .min_value(min_value)
        .max_value(max_value)
        .step(step)
        .format_options(format_options)
        .commit_behavior(commit_behavior)
        .placeholder(placeholder)
        .is_disabled(is_disabled)
        .is_read_only(is_read_only)
        .is_required(is_required)
        .is_invalid(is_invalid)
        .auto_focus(auto_focus)
        .aria_label(aria_label)
        .classes(
            classes
                .add("leptonic-text-field")
                .add("leptonic-number-field"),
        )
        .styles(styles)
        .children(Box::new(children))
        .build();
    props.default_value = default_value;
    props.value = value;
    props.set_value = set_value;
    props.on_change = on_change;
    props.validate = validate;
    props.validation_behavior = validation_behavior;
    props.name = name;
    props.on_focus_change = on_focus_change;
    NumberFieldAtom(props)
}

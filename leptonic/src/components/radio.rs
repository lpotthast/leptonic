use leptos::prelude::*;

use crate::{
    atoms::field::{Description, FieldError, Label, TextElement},
    atoms::radio::{
        Radio as RadioAtom, RadioGroup as RadioGroupAtom, RadioGroupProps as RadioGroupAtomProps,
    },
    hooks::{Orientation, collections::Key},
    utils::{classes::Classes, styles::Styles},
};

/// A labelled group of [`Radio`]s, one of which is selected. The arrow keys move the
/// selection.
#[allow(clippy::too_many_arguments)]
#[component]
pub fn RadioGroup(
    /// The visible label. Without it, set `aria_label`.
    #[prop(into, optional)]
    label: Option<String>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] description: Option<String>,
    #[prop(into, optional)] default_value: Option<Key>,
    #[prop(into, optional)] on_change: Option<Callback<Option<Key>>>,
    #[prop(default = Orientation::Vertical)] orientation: Orientation,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] is_read_only: Signal<bool>,
    #[prop(into, optional)] is_required: Signal<bool>,
    #[prop(into, optional)] is_invalid: Signal<bool>,
    #[prop(into, optional)] name: Option<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let content = move || {
        view! {
            {label.map(|label| view! {
                <Label classes="leptonic-radio-group-label">{label}</Label>
            })}
            <div class="leptonic-radio-group-items" data-orientation=orientation.as_str()>
                {children()}
            </div>
            {description.map(|description| view! {
                <Description element=TextElement::Div classes="leptonic-radio-group-description">
                    {description}
                </Description>
            })}
            <FieldError element=TextElement::Div classes="leptonic-radio-group-error" />
        }
        .into_any()
    };
    // Built from the props struct: the view macro can't forward `Option` props.
    RadioGroupAtom(RadioGroupAtomProps {
        default_value,
        on_change,
        orientation,
        is_disabled,
        is_read_only,
        is_required,
        is_invalid,
        validate: None,
        validation_behavior: None,
        name,
        form: None,
        id: None,
        aria_label,
        aria_labelledby: None,
        aria_describedby: None,
        classes: classes.add("leptonic-radio-group"),
        styles,
        children: Box::new(content),
    })
}

/// A radio in a [`RadioGroup`], with its label (the children).
#[component]
pub fn Radio(
    /// The value the radio selects.
    #[prop(into)]
    value: Key,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    /// The accessible name, when there is no visible label.
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <RadioAtom value is_disabled aria_label classes=classes.add("leptonic-radio") styles>
            <span class="leptonic-radio-circle" aria-hidden="true">
                <span class="leptonic-radio-fill" />
            </span>
            {children.map(|children| view! { <span class="leptonic-radio-label">{children()}</span> })}
        </RadioAtom>
    }
}

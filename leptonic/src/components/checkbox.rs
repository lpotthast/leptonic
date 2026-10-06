use leptos::prelude::*;

use crate::{
    Out,
    atoms::{
        checkbox::{
            Checkbox as CheckboxAtom, CheckboxGroup as CheckboxGroupAtom,
            CheckboxGroupProps as CheckboxGroupAtomProps, CheckboxProps as CheckboxAtomProps,
        },
        field::{Description, FieldError, Label, TextElement},
    },
    components::icon::Icon,
    hooks::{Orientation, collections::Key},
    utils::{classes::Classes, styles::Styles},
};

/// A checkbox with its label (the children).
///
/// Its selection starts at `default_selected` and is reported through `on_change`; or it is
/// `is_selected`, and changes go to `set_selected` (e.g. both an `RwSignal<bool>`).
/// Inside a [`CheckboxGroup`], give it a `value` instead.
#[allow(clippy::too_many_arguments, clippy::needless_pass_by_value)]
#[component]
pub fn Checkbox(
    #[prop(optional)] default_selected: bool,
    #[prop(into, optional)] on_change: Option<Callback<bool>>,
    /// The selection (controlled): a value or any signal.
    #[prop(into, optional)]
    is_selected: Option<Signal<bool>>,
    /// Receives the new state: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_selected: Option<Out<bool>>,
    /// The checkbox's value in its [`CheckboxGroup`].
    #[prop(into, optional)]
    value: Option<Key>,
    #[prop(into, optional)] is_indeterminate: Signal<bool>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] is_read_only: Signal<bool>,
    #[prop(into, optional)] is_required: Signal<bool>,
    #[prop(into, optional)] is_invalid: Signal<bool>,
    #[prop(into, optional)] name: Option<String>,
    /// The value submitted with the form while checked.
    #[prop(into, optional)]
    form_value: Option<String>,
    /// The accessible name, when there is no visible label.
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(default = icondata::BsCheck2)] checked_icon: icondata::Icon,
    #[prop(default = icondata::BsDash)] indeterminate_icon: icondata::Icon,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let content = move || {
        view! {
            <span class="leptonic-checkbox-box" aria-hidden="true">
                <Icon icon=checked_icon classes="leptonic-checkbox-checked-icon" />
                <Icon icon=indeterminate_icon classes="leptonic-checkbox-indeterminate-icon" />
            </span>
            {children.map(|children| view! { <span class="leptonic-checkbox-label">{children()}</span> })}
        }
        .into_any()
    };
    // Built from the props struct: the view macro can't forward `Option` props.
    CheckboxAtom(CheckboxAtomProps {
        value,
        default_selected,
        on_change,
        is_selected,
        set_selected,
        is_indeterminate,
        is_disabled,
        is_read_only,
        is_required,
        is_invalid,
        validate: None,
        validation_behavior: None,
        name,
        form_value,
        form: None,
        id: None,
        aria_label,
        aria_labelledby: None,
        aria_describedby: None,
        auto_focus: false,
        on_focus_change: None,
        classes: classes.add("leptonic-checkbox"),
        styles,
        children: Some(Box::new(content)),
    })
}

/// A labelled group of [`Checkbox`]es (each with a `value`) selecting a set of values.
#[allow(clippy::too_many_arguments)]
#[component]
pub fn CheckboxGroup(
    /// The visible label. Without it, set `aria_label`.
    #[prop(into, optional)]
    label: Option<String>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] description: Option<String>,
    #[prop(into, optional)] default_value: Vec<Key>,
    /// The checked values (controlled): a value or any signal.
    #[prop(into, optional)]
    value: Option<Signal<Vec<Key>>>,
    /// Receives the new state: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_value: Option<Out<Vec<Key>>>,
    #[prop(into, optional)] on_change: Option<Callback<Vec<Key>>>,
    #[prop(default = Orientation::Vertical)] orientation: Orientation,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] is_read_only: Signal<bool>,
    /// Whether at least one checkbox must be checked.
    #[prop(into, optional)]
    is_required: Signal<bool>,
    #[prop(into, optional)] is_invalid: Signal<bool>,
    #[prop(into, optional)] name: Option<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let content = move || {
        view! {
            {label.map(|label| view! {
                <Label classes="leptonic-checkbox-group-label">{label}</Label>
            })}
            <div class="leptonic-checkbox-group-items" data-orientation=orientation.as_str()>
                {children()}
            </div>
            {description.map(|description| view! {
                <Description element=TextElement::Div classes="leptonic-checkbox-group-description">
                    {description}
                </Description>
            })}
            <FieldError element=TextElement::Div classes="leptonic-checkbox-group-error" />
        }
        .into_any()
    };
    CheckboxGroupAtom(CheckboxGroupAtomProps {
        default_value,
        value,
        set_value,
        on_change,
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
        classes: classes.add("leptonic-checkbox-group"),
        styles,
        children: Box::new(content),
    })
}

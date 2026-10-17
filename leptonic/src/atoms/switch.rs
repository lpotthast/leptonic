// Upstream: react-aria-components/src/Switch.tsx @ 99e6102368
// Upstream: react-aria-components/test/Switch.test.js @ 99e6102368
use leptos::{context::Provider, prelude::*};
use leptos_classes::Classes;

use super::{
    checkbox::{ToggleButton, ToggleButtonParts},
    field::FieldContext,
    form::use_validation_behavior,
};
use crate::{
    Out, ValueBinding,
    hooks::{
        form::{
            ToggleOptions, UseSwitchInput, UseSwitchReturn, UseToggleStateInput, ValidateFn,
            ValidationBehavior, use_switch, use_toggle_state,
        },
        interactions::{UseHoverInput, use_hover},
    },
    utils::{
        data_attributes::data_flags, default_class::with_default_class, dev_warn, styles::Styles,
        visually_hidden::visually_hidden_styles,
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
// - No single `Switch` (react-aria-components deprecates it): a switch is a `SwitchField` with
//   its `SwitchButton`.
//
// =============================================================================

/// A headless switch with a description and an error message of its own: a `<div>` around a
/// [`SwitchButton`] (the clickable `<label>` with the track), a
/// [`Description`](super::field::Description) and a [`FieldError`](super::field::FieldError).
///
/// Data attributes: `data-selected`, `data-disabled`, `data-readonly`, `data-invalid`,
/// `data-required`.
///
/// Default class: `leptonic-SwitchField`.
#[component]
pub fn SwitchField(
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
    id: Option<crate::Id>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: MaybeProp<String>,
    #[prop(into, optional)] aria_describedby: MaybeProp<String>,
    #[prop(optional)] auto_focus: bool,
    #[prop(into, optional)] on_focus_change: Option<Callback<bool>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    crate::metrics::subject!();
    let classes = with_default_class("leptonic-SwitchField", classes);
    let switch = use_switch_atom(SwitchSetup {
        default_selected,
        on_change,
        is_selected,
        set_selected,
        is_disabled,
        is_read_only,
        is_required,
        is_invalid,
        validate,
        validation_behavior,
        name,
        form_value,
        form,
        id,
        aria_label,
        aria_labelledby,
        aria_describedby,
        auto_focus,
        on_focus_change,
    });
    let (is_selected, is_disabled, is_read_only, is_invalid) = (
        switch.is_selected,
        switch.is_disabled,
        switch.is_read_only,
        switch.is_invalid,
    );
    let field = FieldContext {
        description: switch.description.clone(),
        error_message: switch.error_message.clone(),
        is_invalid,
        validation_errors: switch.validation_errors,
        validation_details: switch.validation_details,
    };
    let button = SwitchButtonContext {
        button: ToggleButtonParts::of(switch),
        is_required,
    };

    view! {
        <Provider value=button>
            <Provider value=field>
                <div
                    class=classes
                    style={styles}
                    {..data_flags([
                        ("data-selected", is_selected),
                        ("data-disabled", is_disabled),
                        ("data-readonly", is_read_only),
                    ])}
                    {..data_flags([
                        ("data-invalid", is_invalid),
                        ("data-required", is_required),
                    ])}
                >
                    {children()}
                </div>
            </Provider>
        </Provider>
    }
}

/// The clickable part of a [`SwitchField`]: a `<label>` around a visually hidden
/// `<input type="checkbox" role="switch">` and the children (the track and the label text).
///
/// Data attributes: `data-selected`, `data-pressed`, `data-hovered`, `data-focused`,
/// `data-focus-visible`, `data-disabled`, `data-readonly`, `data-invalid`, `data-required`.
///
/// Default class: `leptonic-SwitchButton`.
#[component]
pub fn SwitchButton(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    crate::metrics::subject!();
    let classes = with_default_class("leptonic-SwitchButton", classes);
    let Some(ctx) = use_context::<SwitchButtonContext>() else {
        dev_warn!("a <SwitchButton> belongs in a <SwitchField>");
        return None;
    };
    // One button per field (`Part`: a second one renders nothing).
    let switch = ctx.button.props()?;
    Some(switch_button(
        switch,
        ctx.is_required,
        classes,
        styles,
        children,
    ))
}

/// What a [`SwitchField`] hands its [`SwitchButton`].
#[derive(Clone)]
struct SwitchButtonContext {
    /// The switch, for the button rendered.
    button: ToggleButtonParts,
    is_required: Signal<bool>,
}

/// The settings of a [`SwitchField`].
struct SwitchSetup {
    default_selected: bool,
    on_change: Option<Callback<bool>>,
    is_selected: Option<Signal<bool>>,
    set_selected: Option<Out<bool>>,
    is_disabled: Signal<bool>,
    is_read_only: Signal<bool>,
    is_required: Signal<bool>,
    is_invalid: Signal<bool>,
    validate: Option<ValidateFn<bool>>,
    validation_behavior: Option<ValidationBehavior>,
    name: Option<String>,
    form_value: Option<String>,
    form: Option<String>,
    id: Option<crate::Id>,
    aria_label: MaybeProp<String>,
    aria_labelledby: MaybeProp<String>,
    aria_describedby: MaybeProp<String>,
    auto_focus: bool,
    on_focus_change: Option<Callback<bool>>,
}

/// The switch of a [`SwitchField`].
fn use_switch_atom(setup: SwitchSetup) -> UseSwitchReturn {
    let SwitchSetup {
        default_selected,
        on_change,
        is_selected,
        set_selected,
        is_disabled,
        is_read_only,
        is_required,
        is_invalid,
        validate,
        validation_behavior,
        name,
        form_value,
        form,
        id,
        aria_label,
        aria_labelledby,
        aria_describedby,
        auto_focus,
        on_focus_change,
    } = setup;
    let (value, on_change) = ValueBinding::from_state_props(is_selected, set_selected, on_change);
    let state = use_toggle_state(UseToggleStateInput {
        default_selected,
        value,
        on_change,
        is_read_only,
    });
    use_switch(UseSwitchInput {
        options: ToggleOptions {
            id,
            is_disabled,
            is_read_only,
            is_required,
            is_invalid,
            validate,
            validation_behavior: Some(use_validation_behavior(validation_behavior)),
            name,
            form,
            value: form_value,
            aria_label,
            aria_labelledby: crate::utils::id::id_ref_prop(aria_labelledby),
            aria_describedby: crate::utils::id::id_ref_prop(aria_describedby),
            auto_focus,
            on_focus_change,
            ..ToggleOptions::default()
        },
        state,
    })
}

/// The `<label>` of a [`SwitchButton`].
fn switch_button(
    switch: ToggleButton,
    is_required: Signal<bool>,
    classes: Classes,
    styles: Styles,
    children: Option<Children>,
) -> impl IntoView {
    let hover = use_hover(UseHoverInput {
        is_disabled: Signal::derive(move || switch.is_disabled.get() || switch.is_read_only.get()),
        on_hover_start: None,
        on_hover_end: None,
        on_hover_change: None,
    });
    let (label_attrs, label_styles) = switch.label;
    let (input_attrs, input_styles) = switch.input;

    view! {
        <label
            {..label_attrs}
            {..hover.target.attrs()}
            class=classes
            style=label_styles.merge(styles)
            {..data_flags([
                ("data-pressed", switch.is_pressed),
                ("data-hovered", hover.is_hovered),
                ("data-focused", switch.is_focused),
            ])}
            {..data_flags([("data-focus-visible", switch.is_focus_visible)])}
            {..data_flags([
                ("data-selected", switch.is_selected),
                ("data-disabled", switch.is_disabled),
                ("data-readonly", switch.is_read_only),
            ])}
            {..data_flags([
                ("data-invalid", switch.is_invalid),
                ("data-required", is_required),
            ])}
        >
            <input {..input_attrs} style=input_styles.merge(visually_hidden_styles()) />
            {children.map(|children| children())}
        </label>
    }
}

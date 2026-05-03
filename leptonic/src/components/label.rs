use leptos::prelude::*;

use crate::{
    components::form_control::FormControlContext,
    hooks::{UsePressInput, UsePressReturn, use_press},
    utils::{classes::Classes, styles::Styles},
};

/// Interactive label usable in forms. Automatically registers with the parent `FormControl` to control a sibling input.
#[component]
pub fn Label(
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let fc_ctx = use_context::<FormControlContext>();

    let UsePressReturn {
        props: press_props,
        is_pressed: _,
    } = use_press(UsePressInput {
        disabled,
        force_prevent_default: false,
        force_propagation: false,
        allow_text_selection_on_press: false,
        should_cancel_on_pointer_exit: false,
        prevent_focus_on_press: false,
        force_is_pressed: None,
        on_press: Callback::new(move |_| {
            if let Some(fc_ctx) = &fc_ctx {
                fc_ctx.input.with_untracked(move |input| {
                    if let Some(input) = input {
                        input.on_label_press();
                    }
                });
            }
        }),
        on_press_up: None,
        on_press_start: None,
        on_press_end: None,
        on_press_change: None,
        on_double_press: None,
        on_long_press_start: None,
        on_long_press: None,
        on_long_press_end: None,
        long_press_threshold: None,
        long_press_accessibility_description: None,
    });

    let (press_attrs, press_styles) = press_props.into_parts();
    let styles = press_styles.merge(styles);

    view! {
        <label
            {..press_attrs}
            class=classes.add("leptonic-label")
            style=styles
        >
            {children()}
        </label>
    }
}

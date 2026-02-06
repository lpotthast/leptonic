use leptos::prelude::*;

use crate::{
    components::form_control::FormControlContext,
    hooks::{use_press, UsePressInput, UsePressReturn},
};

/// Interactive label usable in forms. Automatically registers with the parent `FormControl` to control a sibling input.
#[component]
pub fn Label(children: Children, #[prop(into, optional)] disabled: Signal<bool>) -> impl IntoView {
    let fc_ctx = use_context::<FormControlContext>();

    let UsePressReturn {
        props,
        is_pressed: _,
    } = use_press(UsePressInput {
        disabled,
        force_prevent_default: false,
        allow_propagation: false,
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
    });

    view! {
        <leptonic-label {..props.into_attrs()}>
            { children() }
        </leptonic-label>
    }
}

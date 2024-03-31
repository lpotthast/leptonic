use leptos::*;

use crate::{components::form_control::FormControlContext, hooks::*, OptMaybeSignal, Transparent};

/// Interactive label usable in forms. Automatically registers with the parent `FormControl` to control a sibling input.
#[component]
pub fn Label(
    children: Children,
    #[prop(into, optional)] disabled: OptMaybeSignal<bool>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView {
    let fc_ctx = use_context::<FormControlContext>();

    let UsePressReturn {
        is_pressed: _,
        props,
    } = use_press(UsePressInput {
        disabled: disabled.0.unwrap_or(false.into()),
        force_prevent_default: false,
        on_press: Callback::new(move |_| {
            if let Some(fc_ctx) = &fc_ctx {
                fc_ctx.input.with_untracked(move |input| match input {
                    Some(input) => {
                        input.on_label_press();
                    }
                    None => {}
                });
            }
        }),
        on_press_up: None,
        on_press_start: None,
        on_press_end: None,
    });

    view! {
        <Transparent>
            <leptonic-label
                {..props.attrs}
                {..props.handlers}
                id=id
                class=class
                style=style
            >
                { children() }
            </leptonic-label>
        </Transparent>
    }
}

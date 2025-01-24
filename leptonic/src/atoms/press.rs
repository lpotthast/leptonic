use leptos::prelude::*;

use crate::hooks::interactions::use_press::{use_press, PressEvent, UsePressInput, UsePressReturn};

#[component]
pub fn Pressable(
    #[prop(into)] disabled: Signal<bool>,
    on_press: Callback<(PressEvent,)>,
    children: Children,
) -> impl IntoView {
    let UsePressReturn {
        attrs,
        is_pressed: _,
    } = use_press(UsePressInput {
        disabled,
        force_prevent_default: false,
        allow_propagation: false,
        on_press,
        on_press_up: None,
        on_press_start: None,
        on_press_end: None,
    });

    view! {
        <div
            style="display: contents"
            {..attrs}
        >
            { children() }
        </div>
    }
}

// TODO: PressResponder

use leptos::*;

use crate::hooks::interactions::use_press::{use_press, PressEvent, UsePressInput, UsePressReturn};

#[component]
pub fn Pressable(
    #[prop(into)] disabled: MaybeSignal<bool>,
    on_press: Callback<PressEvent>,
    children: Children,
) -> impl IntoView {
    let UsePressReturn {
        props,
        is_pressed: _,
    } = use_press(UsePressInput {
        disabled,
        force_prevent_default: false,
        on_press,
        on_press_up: None,
        on_press_start: None,
        on_press_end: None,
    });

    view! {
        <leptonic-pressable
            {..props.attrs}
            on:keydown=props.on_key_down
            on:click=props.on_click
            on:pointerdown=props.on_pointer_down
            style="display: contents"
        >
            { children() }
        </leptonic-pressable>
    }
}

// TODO: PressResponder

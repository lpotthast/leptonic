use leptos::*;

use crate::hooks::*;

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
            //use:attrs=props.attrs
            //use:handlers=props.handlers
            style="display: contents"
        >
            { children() }
        </leptonic-pressable>
    }
}

// TODO: PressResponder

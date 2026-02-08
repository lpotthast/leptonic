use leptos::prelude::*;

use crate::hooks::{use_press, PressEvent, UsePressInput, UsePressReturn};

#[component]
pub fn Pressable(
    #[prop(into)] disabled: Signal<bool>,
    on_press: Callback<PressEvent>,
    children: Children,
) -> impl IntoView {
    let UsePressReturn {
        props,
        is_pressed: _,
    } = use_press(UsePressInput {
        disabled,
        force_prevent_default: false,
        allow_propagation: false,
        allow_text_selection_on_press: false,
        should_cancel_on_pointer_exit: false,
        on_press,
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

    view! {
        <div style="display: contents" {..props.into_attrs()}>
            {children()}
        </div>
    }
}

// TODO: PressResponder

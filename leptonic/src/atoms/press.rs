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
        force_propagation: false,
        allow_text_selection_on_press: false,
        should_cancel_on_pointer_exit: false,
        prevent_focus_on_press: false,
        force_is_pressed: None,
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
        <div {..props.into_attrs()} attr:data-pressable="true" style="display: contents">
            {children()}
        </div>
    }
}

// TODO: PressResponder — implement PressResponderContext to allow parent components
// to intercept and augment press behavior of child pressable elements (similar to
// react-aria's PressResponderContext). This enables patterns like menu items that
// close their parent menu on press without the item needing to know about the menu.

use leptonic::{
    hooks::*,
    utils::{
        aria::{AriaExpanded, AriaHasPopup},
        classes::Classes,
    },
};
use leptos::{html, prelude::*};
use leptos_use::use_window;

#[component]
pub fn BasicButtonDemo() -> impl IntoView {
    let el: NodeRef<html::Div> = NodeRef::new();

    let UseButtonReturn {
        props,
        is_hovered: _,
        is_pressed: _,
        is_focus_visible: _,
    } = use_button(UseButtonInput {
        disabled: false.into(),
        aria_haspopup: AriaHasPopup::default().into(),
        aria_expanded: AriaExpanded::default().into(),
        use_press_input: UsePressInput {
            disabled: false.into(),
            force_prevent_default: false,
            force_propagation: false,
            allow_text_selection_on_press: false,
            should_cancel_on_pointer_exit: false,
            prevent_focus_on_press: false,
            force_is_pressed: None,
            on_press: Callback::new(move |_e| {
                if let Some(window) = use_window().as_ref() {
                    let _ = window.alert_with_message("Pressed!");
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
        },
        use_hover_input: UseHoverInput {
            disabled: false.into(),
            on_hover_start: None,
            on_hover_end: None,
            on_hover_change: None,
        },
        use_focus_ring_input: UseFocusRingInput {
            disabled: false.into(),
            within: false,
            auto_focus: false,
            is_text_input: false,
            on_focus: None,
            on_blur: None,
            on_focus_change: None,
        },
    });

    let (button_props, button_styles) = props.into_inner();

    view! {
        <div
            {..button_props.into_attrs()}
            style=button_styles
            node_ref=el
            class=Classes::from("demo-btn")
        >
            "Press me"
        </div>
    }
}

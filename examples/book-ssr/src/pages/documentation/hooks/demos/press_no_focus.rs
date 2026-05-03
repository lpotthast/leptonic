use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn PressNoFocusDemo() -> impl IntoView {
    let (focus_demo_focused, set_focus_demo_focused) = signal(false);
    let UsePressReturn {
        props: no_focus_props,
        is_pressed: no_focus_is_pressed,
    } = use_press(UsePressInput {
        disabled: Signal::derive(|| false),
        force_prevent_default: false,
        force_propagation: false,
        allow_text_selection_on_press: false,
        should_cancel_on_pointer_exit: false,
        prevent_focus_on_press: true,
        force_is_pressed: None,
        on_press: Callback::new(|_| {}),
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

    let (no_focus_press_props, no_focus_press_styles) = no_focus_props.into_inner();

    view! {
        <p>
            <strong>"prevent_focus_on_press"</strong>
            " \u{2014} Click the button below: it will not receive focus (no outline appears). "
            "Compare with the demo button above which receives focus on click."
        </p>

        <button
            {..no_focus_press_props.into_attrs()}
            style=no_focus_press_styles
            on:focus=move |_| set_focus_demo_focused.set(true)
            on:blur=move |_| set_focus_demo_focused.set(false)
            style:background=move || if no_focus_is_pressed.get() { "var(--brand-color)" } else { "" }
            style:color=move || if no_focus_is_pressed.get() { "white" } else { "" }
            style:transform=move || if no_focus_is_pressed.get() { "scale(0.97)" } else { "" }
            style:transition="background 0.1s, color 0.1s, transform 0.1s"
            style:outline=move || if focus_demo_focused.get() { "2px solid var(--brand-color)" } else { "none" }
            style:outline-offset="2px"
        >
            "Click me (no focus)"
        </button>

        <p>"Focused: " { move || focus_demo_focused.get() }</p>
    }
}

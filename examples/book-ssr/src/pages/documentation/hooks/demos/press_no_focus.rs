use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn PressNoFocusDemo() -> impl IntoView {
    let (focused, set_focused) = signal(false);

    let UsePressReturn { props, is_pressed } = use_press(UsePressInput {
        prevent_focus_on_press: true,
        ..Default::default()
    });
    let (attrs, styles) = props.into_parts();

    view! {
        <p>
            <strong>"prevent_focus_on_press"</strong>
            " \u{2014} Click the button: it does not receive focus, so no focus outline appears. "
            "The button above does receive focus when clicked."
        </p>

        <button
            {..attrs}
            style=styles
            class="demo-press-button demo-interactions-show-focus"
            class:pressed=move || is_pressed.get()
            on:focus=move |_| set_focused.set(true)
            on:blur=move |_| set_focused.set(false)
        >
            "Click me (no focus)"
        </button>

        <p>"Focused: "{move || focused.get()}</p>
    }
}

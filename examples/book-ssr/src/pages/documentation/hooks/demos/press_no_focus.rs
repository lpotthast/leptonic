use leptonic::{
    flag,
    hooks::{
        focus::{UseFocusInput, UseFocusReturn, use_focus},
        interactions::{UsePressInput, UsePressReturn, use_press},
    },
};
use leptos::prelude::*;

#[component]
pub fn PressNoFocusDemo() -> impl IntoView {
    let focused = RwSignal::new(false);

    let UsePressReturn { target, is_pressed } = use_press(UsePressInput {
        prevent_focus_on_press: true.into(),
        ..Default::default()
    });
    let (attrs, styles) = target.attrs();

    // Only to show whether the button has focus.
    let UseFocusReturn {
        target: focus_target,
    } = use_focus(UseFocusInput {
        on_focus_change: Some(Callback::new(move |is_focused| focused.set(is_focused))),
        ..Default::default()
    });

    view! {
        <button
            {..attrs}
            {..focus_target.attrs()}
            style=styles
            class="demo-press-button"
            data-pressed=flag(is_pressed)
        >
            "Press me (no focus)"
        </button>

        <p class="demo-status">
            {move || if focused.get() { "The button has focus." } else { "The button doesn\u{2019}t have focus." }}
        </p>
    }
}

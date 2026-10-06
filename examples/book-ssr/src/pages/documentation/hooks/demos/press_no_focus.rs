use leptonic::{hooks::*, utils::data_attributes::flag};
use leptos::prelude::*;

#[component]
pub fn PressNoFocusDemo() -> impl IntoView {
    let focused = RwSignal::new(false);

    let UsePressReturn { props, is_pressed } = use_press(UsePressInput {
        prevent_focus_on_press: true.into(),
        ..Default::default()
    });
    let (attrs, styles) = props.into_parts();

    // Only to show whether the button has focus.
    let UseFocusReturn { props: focus_props } = use_focus(UseFocusInput {
        on_focus_change: Some(Callback::new(move |is_focused| focused.set(is_focused))),
        ..Default::default()
    });

    view! {
        <button
            {..attrs}
            {..focus_props.into_attrs()}
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

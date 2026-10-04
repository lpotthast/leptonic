use leptonic::{hooks::*, utils::classes::Classes};
use leptos::prelude::*;

#[component]
pub fn BasicButtonDemo() -> impl IntoView {
    let presses = RwSignal::new(0u32);

    // A `<div>` that looks, behaves and is announced like a button.
    let UseButtonReturn {
        props,
        is_pressed,
        is_hovered,
        is_focus_visible,
        ..
    } = use_button(UseButtonInput {
        element_type: ButtonElementType::Other,
        on_press: Some(Callback::new(move |_| presses.update(|p| *p += 1))),
        ..Default::default()
    });

    let (attrs, styles) = props.into_parts();

    view! {
        <div {..attrs} style=styles class=Classes::from("demo-btn")>
            "Press me"
        </div>
        <p>
            {move || format!("Pressed {} times. ", presses.get())}
            {move || is_pressed.get().then_some("Pressing. ")}
            {move || is_hovered.get().then_some("Hovered. ")}
            {move || is_focus_visible.get().then_some("Focus visible.")}
        </p>
    }
}

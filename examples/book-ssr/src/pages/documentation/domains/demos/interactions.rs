use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn InteractionsDomainDemo() -> impl IntoView {
    let (count, set_count) = signal(0);

    let UsePressReturn { props, is_pressed } = use_press(UsePressInput {
        on_press: Callback::new(move |_| set_count.update(|c| *c += 1)),
        ..Default::default()
    });
    let (attrs, styles) = props.into_parts();

    view! {
        <button {..attrs} style=styles class="demo-press-button" class:pressed=move || is_pressed.get()>
            {move || if is_pressed.get() { "Pressing\u{2026}" } else { "Press me" }}
        </button>
        <p class="demo-mt-1">
            "Pressed "<strong>{move || count.get()}</strong>
            {move || if count.get() == 1 { " time" } else { " times" }}
        </p>
    }
}

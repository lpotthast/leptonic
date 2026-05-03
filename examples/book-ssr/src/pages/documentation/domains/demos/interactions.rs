use leptonic::hooks::*;
use leptos::prelude::*;
use leptos_classes::Classes;

#[component]
pub fn InteractionsDomainDemo() -> impl IntoView {
    let (count, set_count) = signal(0);

    let UsePressReturn { props, is_pressed } = use_press(UsePressInput {
        disabled: Signal::derive(|| false),
        force_prevent_default: false,
        force_propagation: false,
        allow_text_selection_on_press: false,
        should_cancel_on_pointer_exit: false,
        prevent_focus_on_press: false,
        force_is_pressed: None,
        on_press: Callback::new(move |_: PressEvent| {
            set_count.update(|c| *c += 1);
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
    });

    let (press_props, press_styles) = props.into_inner();

    view! {
        <div
            tabindex=0
            {..press_props.into_attrs()}
            style=press_styles
            class=Classes::builder().with_toggle(is_pressed.get(), "demo-container-active", "demo-btn").build()
        >
            { move || if is_pressed.get() { "Pressing\u{2026}" } else { "Press me" } }
        </div>
        <p class=Classes::from("demo-mt-1")>
            "Pressed: "
            <strong>{ move || count.get() }</strong>
            { move || match count.get() { 1 => " time", _ => " times" } }
        </p>
    }
}

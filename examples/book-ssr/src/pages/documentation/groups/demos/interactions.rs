use leptonic::{atoms::checkbox::{CheckboxButton, CheckboxField}, hooks::*, utils::data_attributes::flag};
use leptos::prelude::*;

#[component]
pub fn InteractionsQuickStartDemo() -> impl IntoView {
    let count = RwSignal::new(0);
    let disabled = RwSignal::new(false);

    let UsePressReturn { props, is_pressed } = use_press(UsePressInput {
        is_disabled: disabled.into(),
        on_press: Some(Callback::new(move |_| count.update(|c| *c += 1))),
        ..Default::default()
    });
    let (attrs, styles) = props.into_parts();

    view! {
        // `data-pressed` shows the hook's `is_pressed` state to CSS.
        <button {..attrs} style=styles class="demo-press-button" data-pressed=flag(is_pressed) disabled=move || disabled.get()>
            "Press me"
        </button>

        <div class="demo-controls">
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>

        <p class="demo-status">
            {move || match count.get() {
                1 => "Pressed 1 time.".to_owned(),
                n => format!("Pressed {n} times."),
            }}
        </p>
    }
}

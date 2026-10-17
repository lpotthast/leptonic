use leptonic::{
    atoms::checkbox::{CheckboxButton, CheckboxField},
    hooks::interactions::{UseWheelSteppingInput, UseWheelSteppingReturn, use_wheel_stepping},
};
use leptos::prelude::*;

#[component]
pub fn WheelSteppingDemo() -> impl IntoView {
    let minutes = RwSignal::new(15_u32);
    let disabled = RwSignal::new(false);
    let steps = RwSignal::new(0_u32);

    let UseWheelSteppingReturn { group, input } = use_wheel_stepping(UseWheelSteppingInput {
        is_disabled: disabled.into(),
        on_increment: Some(Callback::new(move |()| {
            minutes.update(|m| *m = (*m + 5).min(120));
            steps.update(|s| *s += 1);
        })),
        on_decrement: Some(Callback::new(move |()| {
            minutes.update(|m| *m = m.saturating_sub(5));
            steps.update(|s| *s += 1);
        })),
        ..UseWheelSteppingInput::default()
    });

    view! {
        <div class="demo-field">
            <label class="demo-field-label" for="demo-wheel-stepping-minutes">"Break (minutes, 0\u{2013}120)"</label>
            // Focus anywhere in the group lets the wheel step; the wheel acts over the input.
            <div {..group.attrs()}>
                <input
                    id="demo-wheel-stepping-minutes"
                    class="demo-input demo-text-input"
                    inputmode="numeric"
                    disabled=move || disabled.get()
                    prop:value=move || minutes.get().to_string()
                    on:change=move |e| {
                        if let Ok(value) = event_target_value(&e).trim().parse::<u32>() {
                            minutes.set(value.min(120));
                        }
                    }
                    {..input.attrs()}
                />
            </div>
        </div>

        <div class="demo-controls">
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>

        <p class="demo-status">
            {move || format!("Focus the field and scroll over it. Stepped {} times.", steps.get())}
        </p>
    }
}

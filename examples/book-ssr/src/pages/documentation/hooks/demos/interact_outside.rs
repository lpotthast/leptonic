use leptonic::{
    atoms::{button::Button, checkbox::{CheckboxButton, CheckboxField}},
    hooks::*,
};
use leptos::prelude::*;

#[component]
pub fn InteractOutsideDemo() -> impl IntoView {
    let outside_clicks = RwSignal::new(0);
    let is_open = RwSignal::new(true);
    let disabled = RwSignal::new(false);

    let UseInteractOutsideReturn { props } = use_interact_outside(UseInteractOutsideInput {
        is_disabled: disabled.into(),
        on_interact_outside: Some(Callback::new(move |_| {
            outside_clicks.update(|count| *count += 1);
            is_open.set(false);
        })),
        ..Default::default()
    });
    let attrs = props.into_attrs();

    view! {
        <Show
            when=move || is_open.get()
            fallback=move || view! { <Button on_press=move |_| is_open.set(true) classes="demo-btn">"Reopen"</Button> }
        >
            <div {..attrs.clone()} class="demo-interactions-panel">
                <p class="demo-container-title">"Click outside to close"</p>
                <p class="demo-muted-text">"This panel closes when you click anywhere outside of it."</p>
            </div>
        </Show>

        <div class="demo-controls">
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>

        <p class="demo-status">
            {move || match outside_clicks.get() {
                1 => "1 outside click.".to_owned(),
                n => format!("{n} outside clicks."),
            }}
        </p>
    }
}

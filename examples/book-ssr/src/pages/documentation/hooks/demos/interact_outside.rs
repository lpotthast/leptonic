use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;
use leptos_classes::Classes;

#[component]
pub fn InteractOutsideDemo() -> impl IntoView {
    let (outside_click_count, set_outside_click_count) = signal(0);
    let (is_open, set_is_open) = signal(true);
    let (disabled, set_disabled) = signal(false);

    let interact_outside = use_interact_outside(UseInteractOutsideInput {
        disabled: disabled.into(),
        on_interact_outside_start: None,
        on_interact_outside: Some(Callback::new(move |_| {
            set_outside_click_count.update(|count| *count += 1);
            set_is_open.set(false);
        })),
    });

    let interact_outside_attrs = interact_outside.props.into_attrs();

    view! {
        <FormControl classes="demo-form-row">
            <Checkbox checked=disabled set_checked=set_disabled />
            <Label>"Disable outside interaction detection"</Label>
        </FormControl>

        <Show when=move || is_open.get()>
            <div
                {..interact_outside_attrs.clone()}
                class=Classes::from("demo-btn")
            >
                <p style="margin: 0 0 0.5em 0; font-weight: bold;">"Click outside to close"</p>
                <p style="margin: 0; font-size: 0.9em; opacity: 0.8;">
                    "This element will close when you click anywhere outside of it."
                </p>
            </div>
        </Show>

        <Show when=move || !is_open.get()>
            <button
                on:click=move |_| set_is_open.set(true)
                class=Classes::from("demo-btn-primary")
            >
                "Reopen"
            </button>
        </Show>

        <p>"Outside clicks detected: " <strong>{ move || outside_click_count.get() }</strong></p>
    }
}

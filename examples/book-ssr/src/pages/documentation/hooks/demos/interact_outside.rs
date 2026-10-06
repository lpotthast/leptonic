use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;

#[component]
pub fn InteractOutsideDemo() -> impl IntoView {
    let (outside_click_count, set_outside_click_count) = signal(0);
    let (is_open, set_is_open) = signal(true);
    let (disabled, set_disabled) = signal(false);

    let interact_outside = use_interact_outside(UseInteractOutsideInput {
        is_disabled: disabled.into(),
        on_interact_outside_start: None,
        on_interact_outside: Some(Callback::new(move |_| {
            set_outside_click_count.update(|count| *count += 1);
            set_is_open.set(false);
        })),
    });
    let attrs = interact_outside.props.into_attrs();

    view! {
        <Checkbox state=(disabled, set_disabled) classes="demo-form-row">"Disable outside interaction detection"</Checkbox>

        <Show
            when=move || is_open.get()
            fallback=move || view! {
                <Button on_press=move |_| set_is_open.set(true) classes="demo-mt-half">"Reopen"</Button>
            }
        >
            <div {..attrs.clone()} class="demo-interactions-panel">
                <p class="demo-container-title">"Click outside to close"</p>
                <p class="demo-muted-text">"This panel closes when you click anywhere outside of it."</p>
            </div>
        </Show>

        <p>"Outside clicks detected: "<strong>{move || outside_click_count.get()}</strong></p>
    }
}

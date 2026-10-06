use leptonic::{
    components::prelude::*,
    hooks::*,
    utils::{classes::Classes, css::em},
};
use leptos::prelude::*;

#[component]
pub fn FocusWithinDemo() -> impl IntoView {
    let (disabled, set_disabled) = signal(false);
    let (focus_count, set_focus_count) = signal(0);
    let (blur_count, set_blur_count) = signal(0);
    let (change_count, set_change_count) = signal(0);

    let UseFocusWithinReturn {
        props,
        is_focus_within,
    } = use_focus_within(UseFocusWithinInput {
        is_disabled: disabled.into(),
        on_focus_within: Some(Callback::new(move |_| {
            set_focus_count.update(|c| *c += 1);
        })),
        on_blur_within: Some(Callback::new(move |_| {
            set_blur_count.update(|c| *c += 1);
        })),
        on_focus_within_change: Some(Callback::new(move |_focused: bool| {
            set_change_count.update(|c| *c += 1);
        })),
    });

    view! {
        <div
            {..props.into_attrs()}
            class=Classes::builder().with_toggle(is_focus_within, "demo-container-active", "demo-container-inactive").build()
        >
            <Stack orientation=StackOrientation::Vertical spacing=em(1.0)>
                <p class="demo-focus-flush">
                    "Focus within: "
                    <strong class=Classes::builder().with_toggle(is_focus_within, "demo-state-active", "demo-state-inactive").build()>
                        { move || if is_focus_within.get() { "Yes" } else { "No" } }
                    </strong>
                </p>

                <Stack orientation=StackOrientation::Horizontal spacing=em(1.0)>
                    <input
                        type="text"
                        placeholder="Click me..."
                        class=Classes::from("demo-input")
                    />
                    <button class=Classes::from("demo-btn")>
                        "Button 1"
                    </button>
                    <button class=Classes::from("demo-btn")>
                        "Button 2"
                    </button>
                </Stack>

                <p class="demo-focus-hint">
                    "Tab between the elements: focus stays within the container."
                </p>
            </Stack>
        </div>

        <div class=Classes::from("demo-flex-gap")>
            <p>"Focus within: " <strong>{ move || focus_count.get() }</strong></p>
            <p>"Blur within: " <strong>{ move || blur_count.get() }</strong></p>
            <p>"Change: " <strong>{ move || change_count.get() }</strong></p>
        </div>

        <Checkbox state=(disabled, set_disabled) classes="demo-form-row">"Disabled"</Checkbox>
    }
}

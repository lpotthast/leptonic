use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;
use leptos_classes::Classes;

#[component]
pub fn FocusRingKeyboardDemo() -> impl IntoView {
    let (disabled, set_disabled) = signal(false);
    let (focus_count, set_focus_count) = signal(0);
    let (blur_count, set_blur_count) = signal(0);

    let focus_ring = use_focus_ring(UseFocusRingInput::default());

    let focus_ring_custom = use_focus_ring(UseFocusRingInput {
        disabled: disabled.into(),
        on_focus: Some(Callback::new(move |_| {
            set_focus_count.update(|c| *c += 1);
        })),
        on_blur: Some(Callback::new(move |_| {
            set_blur_count.update(|c| *c += 1);
        })),
        ..Default::default()
    });

    view! {
        <button
            {..focus_ring.props.into_attrs()}
            tabindex="0"
            class=Classes::from("demo-btn")
        >
            "Tab to me (keyboard) or click me (mouse)"
        </button>

        <p class=Classes::from("demo-mt-1")>
            "Is focused: " <strong>{ move || focus_ring.is_focused.get().to_string() }</strong>
            " | Focus ring visible: " <strong>{ move || focus_ring.is_focus_visible.get().to_string() }</strong>
        </p>

        <button
            {..focus_ring_custom.props.into_attrs()}
            tabindex="0"
            class=Classes::from("demo-btn")
        >
            "Focus ring with callbacks"
        </button>

        <div class=Classes::from("demo-flex-gap")>
            <p>"Focus count: " <strong>{ move || focus_count.get() }</strong></p>
            <p>"Blur count: " <strong>{ move || blur_count.get() }</strong></p>
        </div>

        <FormControl classes="demo-form-row">
            <Checkbox checked=disabled set_checked=set_disabled />
            <Label>"Disabled"</Label>
        </FormControl>
    }
}

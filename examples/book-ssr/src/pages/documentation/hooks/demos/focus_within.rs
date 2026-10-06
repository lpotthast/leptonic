use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;

fn times(count: u32) -> String {
    if count == 1 { "1 time".to_string() } else { format!("{count} times") }
}

#[component]
pub fn FocusWithinDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);
    let (entered, set_entered) = signal(0_u32);
    let (left, set_left) = signal(0_u32);

    let UseFocusWithinReturn {
        props,
        is_focus_within,
    } = use_focus_within(UseFocusWithinInput {
        is_disabled: disabled.into(),
        on_focus_within: Some(Callback::new(move |_| set_entered.update(|c| *c += 1))),
        on_blur_within: Some(Callback::new(move |_| set_left.update(|c| *c += 1))),
        ..Default::default()
    });

    view! {
        // The hook sets no attribute: the demo exposes its state as `data-focus-within` for the CSS.
        <div
            class="demo-focus-within-box"
            data-focus-within=move || is_focus_within.get().then_some("true")
            {..props.into_attrs()}
        >
            <label class="demo-focus-label">
                "Search "
                <input type="search" class="demo-focus-item"/>
            </label>
            <Button>"Search"</Button>
            <Button variant=ButtonVariant::Outlined>"Clear"</Button>
        </div>

        <p class="demo-status">
            {move || if is_focus_within.get() { "Focus is inside the group." } else { "Focus is outside the group." }}
            " Focus entered " {move || times(entered.get())} " and left " {move || times(left.get())} "."
        </p>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}

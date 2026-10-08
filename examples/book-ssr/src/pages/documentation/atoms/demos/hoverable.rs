use leptonic::{
    atoms::prelude::*,
    hooks::{HoverEndEvent, HoverStartEvent},
};
use leptos::prelude::*;

#[component]
pub fn HoverableDemo() -> impl IntoView {
    let hovered = RwSignal::new(false);
    let last_pointer = RwSignal::new(None::<String>);
    let disabled = RwSignal::new(false);

    view! {
        // The box is styled through the `data-hovered` attribute the atom adds.
        <Hoverable
            is_disabled=disabled
            on_hover_start=move |e: HoverStartEvent| {
                hovered.set(true);
                last_pointer.set(Some(e.pointer_type.to_string()));
            }
            on_hover_end=move |_: HoverEndEvent| hovered.set(false)
        >
            <div class="demo-hover-target">"Hover me"</div>
        </Hoverable>

        <div class="demo-controls">
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>

        <p class="demo-status">
            {move || {
                let state = if hovered.get() { "Hovered" } else { "Not hovered" };
                match last_pointer.get() {
                    Some(pointer) => format!("{state}, last pointer type: {pointer}."),
                    None => format!("{state}."),
                }
            }}
        </p>
    }
}

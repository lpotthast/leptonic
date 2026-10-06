use leptonic::{
    atoms::prelude::*,
    components::prelude::Checkbox,
    hooks::{HoverEndEvent, HoverStartEvent},
    utils::classes::Classes,
};
use leptos::prelude::*;

#[component]
pub fn HoverableDemo() -> impl IntoView {
    let (hovered, set_hovered) = signal(false);
    let (last_pointer, set_last_pointer) = signal(String::from("none"));
    let (disabled, set_disabled) = signal(false);

    view! {
        <Hoverable
            is_disabled=disabled
            on_hover_start=move |e: HoverStartEvent| {
                set_hovered.set(true);
                set_last_pointer.set(e.pointer_type.to_string());
            }
            on_hover_end=move |_: HoverEndEvent| set_hovered.set(false)
        >
            <div class=Classes::from("demo-hover-target").add_reactive("hovered", hovered)>"Hover me"</div>
        </Hoverable>

        <Checkbox state=(disabled, set_disabled)>"Disabled"</Checkbox>

        <div class="demo-state-display">
            <div><strong>"Hovered: "</strong>{move || hovered.get().to_string()}</div>
            <div><strong>"Last pointer: "</strong>{last_pointer}</div>
        </div>
    }
}

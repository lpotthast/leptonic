use leptonic::{atoms::checkbox::Checkbox, hooks::*, utils::data_attributes::flag};
use leptos::prelude::*;

#[component]
pub fn BasicButtonDemo() -> impl IntoView {
    let presses = RwSignal::new(0u32);
    let disabled = RwSignal::new(false);

    // A `<div>` that looks, behaves and is announced like a button.
    let UseButtonReturn {
        props,
        is_pressed,
        is_hovered,
        ..
    } = use_button(UseButtonInput {
        element_type: ButtonElementType::Other,
        is_disabled: disabled.into(),
        on_press: Some(Callback::new(move |_| presses.update(|n| *n += 1))),
        ..Default::default()
    });
    let (attrs, styles) = props.into_parts();

    // The hook sets `role`, `tabindex`, `aria-disabled` and `data-focus-visible`; the pressed and hovered states
    // are its return values, rendered as data attributes for the stylesheet.
    view! {
        <div {..attrs} style=styles class="demo-hook-button" data-pressed=flag(is_pressed) data-hovered=flag(is_hovered)>
            "Press me"
        </div>
        <p class="demo-status">
            {move || match presses.get() {
                1 => "Pressed 1 time.".to_owned(),
                n => format!("Pressed {n} times."),
            }}
        </p>
        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Disabled"
            </Checkbox>
        </div>
    }
}

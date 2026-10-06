use leptonic::{
    atoms::prelude::*,
    hooks::{FocusManager as Manager, FocusManagerOptions},
};
use leptos::{ev::KeyboardEvent, prelude::*};

#[component]
pub fn FocusManagerDemo() -> impl IntoView {
    view! {
        <FocusManager classes="demo-focus-group" let:manager>
            <ArrowKeyButtons manager/>
        </FocusManager>
    }
}

/// Buttons between which the arrow keys move focus, wrapping around at the ends.
#[component]
fn ArrowKeyButtons(manager: Manager) -> impl IntoView {
    let on_keydown = move |e: KeyboardEvent| {
        let options = FocusManagerOptions {
            wrap: true,
            ..FocusManagerOptions::default()
        };
        match e.key().as_str() {
            "ArrowRight" => manager.focus_next(options),
            "ArrowLeft" => manager.focus_previous(options),
            "Home" => manager.focus_first(options),
            "End" => manager.focus_last(options),
            _ => return,
        };
        e.prevent_default();
    };

    view! {
        <div class="demo-focus-manager-buttons" on:keydown=on_keydown>
            <button class="demo-focus-item">"Cut"</button>
            <button class="demo-focus-item">"Copy"</button>
            <button class="demo-focus-item">"Paste"</button>
        </div>
    }
}

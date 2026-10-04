use leptonic::hooks::{Modality, UseFocusVisibleInput, get_modality, use_focus_visible};
use leptos::prelude::*;

#[component]
pub fn PageHookFocusVisible() -> impl IntoView {
    let fv = use_focus_visible(UseFocusVisibleInput::default());
    let focus_should_be_visible = fv.focus_should_be_visible;
    let modality = fv.modality;

    // Stored (global) modality — read on keyup to capture silent updates.
    let (stored_modality, set_stored_modality) = signal(get_modality());

    view! {
        <div
            id="test-page-hook-focus-visible"
            on:keyup=move |_| {
                set_stored_modality.set(get_modality());
            }
            on:pointerdown=move |_| {
                set_stored_modality.set(get_modality());
            }
        >
            <h1>"Focus Visible Hook Test Page"</h1>

            <button id="test-fv-before">"Before"</button>

            <div id="test-fv-target" tabindex="0">
                "Focus Target"
            </div>

            <input type="text" id="test-fv-text-input" placeholder="Text input" />

            <button id="test-fv-after">"After"</button>

            <div>
                "Focus visible: "
                <span id="test-fv-visible">
                    {move || if focus_should_be_visible.get() { "true" } else { "false" }}
                </span>
            </div>
            <div>
                "Modality: "
                <span id="test-fv-modality">
                    {move || match modality.get() {
                        Modality::Unknown => "Unknown",
                        Modality::Pointer => "Pointer",
                        Modality::Keyboard => "Keyboard",
                        Modality::Virtual => "Virtual",
                    }}
                </span>
            </div>
            <div>
                "Stored modality: "
                <span id="test-fv-stored-modality">
                    {move || match stored_modality.get() {
                        Modality::Unknown => "Unknown",
                        Modality::Pointer => "Pointer",
                        Modality::Keyboard => "Keyboard",
                        Modality::Virtual => "Virtual",
                    }}
                </span>
            </div>
        </div>
    }
}

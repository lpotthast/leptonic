use leptonic::hooks::{
    Modality, UseFocusVisibleInput, get_modality, get_pointer_type, use_focus_visible,
};
use leptos::prelude::*;

#[component]
pub fn PageHookFocusVisible() -> impl IntoView {
    let fv = use_focus_visible(UseFocusVisibleInput::default());
    let focus_should_be_visible = fv.focus_should_be_visible;
    let modality = fv.modality;

    // Stored (global) modality — read on keyup to capture silent updates.
    let (stored_modality, set_stored_modality) = signal(get_modality());
    // The pointer type of the last interaction, read at the same moments.
    let (pointer_type, set_pointer_type) = signal(String::new());

    view! {
        <div
            id="test-page-hook-focus-visible"
            on:keyup=move |_| {
                set_stored_modality.set(get_modality());
                set_pointer_type.set(get_pointer_type().to_string());
            }
            on:pointerdown=move |_| {
                set_stored_modality.set(get_modality());
                set_pointer_type.set(get_pointer_type().to_string());
            }
        >
            <h1>"Focus Visible Hook Test Page"</h1>

            <button id="test-fv-before">"Before"</button>

            <div id="test-fv-target" tabindex="0">
                "Focus Target"
            </div>

            <input type="text" id="test-fv-text-input" placeholder="Text input" />

            <button id="test-fv-after">"After"</button>

            // A form library focusing the first invalid field (react-aria-components' Form test).
            <form id="test-fv-form" on:submit=|e: leptos::ev::SubmitEvent| e.prevent_default()>
                <input type="text" id="test-fv-required" aria-label="Name" required />
            </form>

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
                "Pointer type: " <span id="test-fv-pointer-type">{pointer_type}</span>
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

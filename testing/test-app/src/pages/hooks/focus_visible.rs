use leptonic::{
    IntoAttrs,
    hooks::focus::{
        Modality, UseFocusRingInput, UseFocusVisibleInput, WindowFocusTracking,
        add_window_focus_tracking, get_modality, get_pointer_type, use_focus_ring,
        use_focus_visible, use_interaction_modality,
    },
};
use leptos::{html, prelude::*, web_sys};

use crate::pages::prevent_focus_steal;

#[component]
pub fn PageHookFocusVisible() -> impl IntoView {
    let focus_should_be_visible =
        use_focus_visible(UseFocusVisibleInput::default()).focus_should_be_visible;
    let modality = use_interaction_modality();

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
                    {move || modality_name(modality.get())}
                </span>
            </div>
            <div>
                "Pointer type: " <span id="test-fv-pointer-type">{pointer_type}</span>
            </div>
            <OtherWindow />
            <div>
                "Stored modality: "
                <span id="test-fv-stored-modality">
                    {move || modality_name(stored_modality.get())}
                </span>
            </div>
        </div>
    }
}

fn modality_name(modality: Option<Modality>) -> &'static str {
    match modality {
        None => "None",
        Some(Modality::Pointer) => "Pointer",
        Some(Modality::Keyboard) => "Keyboard",
        Some(Modality::Virtual) => "Virtual",
    }
}

/// An iframe whose button has a focus ring (useFocusVisible.test.js, "Setups global event
/// listeners in a different window"): its window's interactions count once
/// `add_window_focus_tracking` tracks it. The ring's state is shown in this document.
#[component]
fn OtherWindow() -> impl IntoView {
    // The iframe renders on the client only, so its `load` listener is there before it loads.
    let client = RwSignal::new(false);
    Effect::new(move || client.set(true));
    let frame = NodeRef::<html::Iframe>::new();
    let frame_button = StoredValue::new_local(None::<web_sys::HtmlElement>);
    let tracking = StoredValue::new_local(None::<WindowFocusTracking>);
    let ready = RwSignal::new(false);
    let visible = RwSignal::new(false);

    let frame_body = move || {
        frame
            .get_untracked()
            .and_then(|frame| frame.content_document())
            .and_then(|document| document.body())
    };
    let on_load = move |_| {
        let Some(body) = frame_body() else {
            return;
        };
        leptos::mount::mount_to(body.clone(), move || view! { <FrameButton visible /> }).forget();
        frame_button.set_value(
            body.query_selector("button")
                .ok()
                .flatten()
                .and_then(|button| wasm_bindgen::JsCast::dyn_into(button).ok()),
        );
        ready.set(true);
    };
    let control = |id: &'static str, label: &'static str, action: fn(&OtherWindowControls)| {
        let controls = OtherWindowControls {
            frame,
            frame_button,
            tracking,
        };
        view! {
            <button
                id=id
                tabindex="-1"
                on:mousedown=prevent_focus_steal
                on:click=move |_| action(&controls)
            >
                {label}
            </button>
        }
    };

    view! {
        <div>
            <Show when=move || client.get()>
                <iframe
                    node_ref=frame
                    title="Other window"
                    srcdoc="<!doctype html><html><body style=\"margin: 0\"></body></html>"
                    on:load=on_load
                ></iframe>
            </Show>
            {control("test-fv-frame-focus", "Focus the frame's button", |c| c.focus_button())}
            {control("test-fv-frame-track", "Track the frame", |c| c.track())}
            {control("test-fv-frame-stop", "Stop tracking", |c| c.stop())}
            {control("test-fv-frame-unload", "beforeunload in the frame", |c| c.before_unload())}
            "Frame: " <span id="test-fv-frame-ready">{move || if ready.get() { "ready" } else { "loading" }}</span>
            ", focus visible: " <span id="test-fv-frame-visible">{move || visible.get().to_string()}</span>
        </div>
    }
}

/// What the controls of [`OtherWindow`] act on.
#[derive(Clone, Copy)]
struct OtherWindowControls {
    frame: NodeRef<html::Iframe>,
    frame_button: StoredValue<Option<web_sys::HtmlElement>, LocalStorage>,
    tracking: StoredValue<Option<WindowFocusTracking>, LocalStorage>,
}

impl OtherWindowControls {
    fn focus_button(&self) {
        if let Some(button) = self.frame_button.get_value() {
            let _ = button.focus();
        }
    }

    fn track(&self) {
        if let Some(button) = self.frame_button.get_value() {
            self.tracking
                .set_value(Some(add_window_focus_tracking(&button)));
        }
    }

    fn stop(&self) {
        self.tracking.set_value(None);
    }

    fn before_unload(&self) {
        if let Some(window) = self.frame.get_untracked().and_then(|f| f.content_window())
            && let Ok(event) = web_sys::Event::new("beforeunload")
        {
            let _ = window.dispatch_event(&event);
        }
    }
}

/// The button inside the iframe, filling it.
#[component]
fn FrameButton(visible: RwSignal<bool>) -> impl IntoView {
    let ring = use_focus_ring(UseFocusRingInput::default());
    let is_focus_visible = ring.is_focus_visible;
    Effect::new(move || visible.set(is_focus_visible.get()));
    view! {
        <button style="width: 100%; height: 100px" {..ring.props.into_attrs()}>
            "In the frame"
        </button>
    }
}

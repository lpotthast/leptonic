use leptonic::{
    atoms::{button::Button, focus_scope::FocusScope},
    hooks::*,
};
use leptos::prelude::*;

/// `use_overlay_trigger_state` alone: a button opens a panel, the panel closes itself, and the state counts openings.
#[component]
pub fn OverlayTriggerStateDemo() -> impl IntoView {
    let times_opened = RwSignal::new(0_u32);

    let state = use_overlay_trigger_state(UseOverlayTriggerStateInput {
        on_open_change: Some(Callback::new(move |is_open: bool| {
            if is_open {
                times_opened.update(|count| *count += 1);
            }
        })),
        ..UseOverlayTriggerStateInput::default()
    });

    view! {
        <Button on_press=move |_| state.toggle() classes="demo-btn">
            {move || if state.is_open.get() { "Hide tips" } else { "Show tips" }}
        </Button>

        <Show when=move || state.is_open.get()>
            <div role="region" aria-label="Tips" class="demo-overlay-inline-panel">
                // Returns focus to the toggle button when the tips close.
                <FocusScope restore_focus=true>
                    <p class="demo-overlay-text">"Press the button again, or close the tips here."</p>
                    <Button on_press=move |_| state.close() classes="demo-btn">"Close"</Button>
                </FocusScope>
            </div>
        </Show>

        <p class="demo-status">
            {move || {
                let open = if state.is_open.get() { "Open" } else { "Closed" };
                match times_opened.get() {
                    1 => format!("{open}. Opened 1 time."),
                    count => format!("{open}. Opened {count} times."),
                }
            }}
        </p>
    }
}

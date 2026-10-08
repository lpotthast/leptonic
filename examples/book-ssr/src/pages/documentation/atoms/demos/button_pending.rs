use std::time::Duration;

use leptonic::atoms::{button::Button, progress_bar::ProgressBar};
use leptos::prelude::*;

#[component]
pub fn ButtonPendingDemo() -> impl IntoView {
    let pending = RwSignal::new(false);
    let saved = RwSignal::new(0_u32);

    // Saving takes two seconds. Meanwhile the button stays focusable but ignores presses.
    let save = move |_| {
        pending.set(true);
        set_timeout(
            move || {
                pending.set(false);
                saved.update(|saved| *saved += 1);
            },
            Duration::from_secs(2),
        );
    };

    view! {
        // With an `aria_label`, the progress bar's label joins the button's name while pending: "Save Saving".
        <Button on_press=save is_pending=pending aria_label="Save" classes="demo-pending-btn">
            "Save"
            <Show when=move || pending.get()>
                <ProgressBar value={None::<u8>} aria_label="Saving" classes="demo-pending-progress">
                    <span class="demo-spinner" aria-hidden="true"></span>
                </ProgressBar>
            </Show>
        </Button>
        <p class="demo-status">
            {move || {
                if pending.get() {
                    "Saving\u{2026}".to_owned()
                } else {
                    match saved.get() {
                        0 => "Not saved yet.".to_owned(),
                        1 => "Saved 1 time.".to_owned(),
                        n => format!("Saved {n} times."),
                    }
                }
            }}
        </p>
    }
}

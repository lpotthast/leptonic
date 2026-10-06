use leptonic::{atoms::prelude as atoms, components::prelude::*};
use leptos::prelude::*;

const TOTAL_BYTES: u32 = 4_000;

#[component]
pub fn ProgressBarAtomDemo() -> impl IntoView {
    let uploaded = RwSignal::new(1_200_u32);
    let size_unknown = RwSignal::new(false);
    // `None`: the progress isn't known, the bar is indeterminate.
    let progress = Signal::derive(move || (!size_unknown.get()).then(|| uploaded.get()));

    view! {
        <atoms::ProgressBar value=progress max_value=TOTAL_BYTES classes="demo-value-bar-container">
            <div class="demo-value-bar-header">
                <atoms::Label>"Uploading"</atoms::Label>
                <atoms::ProgressBarValueText />
            </div>
            <div class="demo-value-bar">
                <atoms::ProgressBarFill classes="demo-value-bar-fill" />
            </div>
        </atoms::ProgressBar>
        <div class="demo-inline-controls">
            <Button on_press=move |_| uploaded.update(|bytes| *bytes = (*bytes + 400).min(TOTAL_BYTES))>
                "Upload 400 bytes"
            </Button>
            <Button on_press=move |_| uploaded.set(0)>"Restart"</Button>
        </div>
        <div class="demo-controls">
            <Checkbox is_selected=size_unknown set_selected=size_unknown>"Size unknown"</Checkbox>
        </div>
    }
}

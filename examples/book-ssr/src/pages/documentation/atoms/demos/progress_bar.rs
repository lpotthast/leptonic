use leptonic::atoms;
use leptos::prelude::*;

const TOTAL_BYTES: u32 = 4_000;

#[component]
pub fn ProgressBarAtomDemo() -> impl IntoView {
    let uploaded = RwSignal::new(1_200_u32);
    let size_unknown = RwSignal::new(false);
    // `None`: the progress isn't known, the bar is indeterminate.
    let progress = Signal::derive(move || (!size_unknown.get()).then(|| uploaded.get()));

    view! {
        <atoms::progress_bar::ProgressBar value=progress max_value=TOTAL_BYTES classes="demo-value-bar-container">
            <div class="demo-value-bar-header">
                <atoms::field::Label>"Uploading"</atoms::field::Label>
                <atoms::progress_bar::ProgressBarValueText />
            </div>
            <div class="demo-value-bar">
                <atoms::progress_bar::ProgressBarFill classes="demo-value-bar-fill" />
            </div>
        </atoms::progress_bar::ProgressBar>
        <div class="demo-inline-controls">
            <atoms::button::Button on_press=move |_| uploaded.update(|bytes| *bytes = (*bytes + 400).min(TOTAL_BYTES)) classes="demo-btn">
                "Upload 400 bytes"
            </atoms::button::Button>
            <atoms::button::Button on_press=move |_| uploaded.set(0) classes="demo-btn">"Restart"</atoms::button::Button>
        </div>
        <div class="demo-controls">
            <atoms::checkbox::CheckboxField is_selected=size_unknown set_selected=size_unknown>
                <atoms::checkbox::CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Size unknown"
                </atoms::checkbox::CheckboxButton>
            </atoms::checkbox::CheckboxField>
        </div>
    }
}

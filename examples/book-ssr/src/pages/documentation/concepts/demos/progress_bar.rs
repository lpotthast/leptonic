use leptonic::atoms::prelude::{Label, ProgressBar, ProgressBarFill, ProgressBarValueText};
use leptos::prelude::*;

#[component]
pub fn ProgressBarConceptDemo() -> impl IntoView {
    view! {
        // 75 of 100: the value text reads "75%".
        <ProgressBar value=75.0 classes="demo-value-bar-container">
            <div class="demo-value-bar-header">
                <Label>"Uploading"</Label>
                <ProgressBarValueText/>
            </div>
            // The track is your own markup; the fill's width follows the value.
            <div class="demo-value-bar">
                <ProgressBarFill classes="demo-value-bar-fill"/>
            </div>
        </ProgressBar>
    }
}

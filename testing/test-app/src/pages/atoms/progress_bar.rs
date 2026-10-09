use leptonic::{
    atoms::{
        field::Label,
        meter::{Meter, MeterFill, MeterValueText},
        progress_bar::{ProgressBar, ProgressBarFill, ProgressBarValueText},
    },
    leptos_styles::Styles,
};
use leptos::prelude::*;

/// Progress bars and meters (react-aria-components' `ProgressBar.test.js` and `Meter.test.js`
/// setups): a label, the value text and a fill in a 200px track; custom and empty ranges.
#[component]
pub fn PageAtomProgressBar() -> impl IntoView {
    let loaded = RwSignal::new(25_u32);
    // Progress that isn't known.
    let unknown: Option<f64> = None;
    let track = "width: 200px; height: 8px; background: #ddd;";
    let fill = || {
        Styles::new()
            .add_unchecked("height", "8px")
            .add_unchecked("background", "blue")
    };

    view! {
        <h1>"Progress bar"</h1>
        <ProgressBar value=loaded id="test-pb-basic">
            <Label>"Loading\u{2026}"</Label>
            <ProgressBarValueText classes="value" />
            <div style=track>
                <ProgressBarFill classes="fill" styles=fill() />
            </div>
        </ProgressBar>
        <button id="test-pb-more" on:click=move |_| loaded.update(|l| *l += 25)>"More"</button>

        <ProgressBar value=Some(3.0) max_value=6.0 aria_label="Custom range">
            <ProgressBarValueText classes="value" />
        </ProgressBar>

        <ProgressBar value=Some(5_i32) min_value=5 max_value=5 aria_label="Empty range">
            <ProgressBarValueText classes="value" />
        </ProgressBar>

        <ProgressBar value=Some(0_i32) min_value=0 max_value=0 aria_label="Zero range">
            <ProgressBarValueText classes="value" />
            <div style=track><ProgressBarFill classes="fill" styles=fill() /></div>
        </ProgressBar>

        <ProgressBar value=unknown aria_label="Indeterminate">
            <ProgressBarValueText classes="value" />
            <div style=track><ProgressBarFill classes="fill" styles=fill() /></div>
        </ProgressBar>

        <ProgressBar value=Some(1_u32) max_value=4 value_label="1 of 4" aria_label="Files" />

        // A `Label` and an `aria_label`: both name it.
        <ProgressBar value=20 id="test-pb-both" aria_label="Named">
            <Label>"Visible"</Label>
        </ProgressBar>

        <h1>"Meter"</h1>
        <Meter value=75_u8>
            <Label>"Storage"</Label>
            <MeterValueText classes="value" />
            <div style=track><MeterFill classes="fill" styles=fill() /></div>
        </Meter>
        <Meter value=3.0 max_value=6.0 aria_label="Meter custom range">
            <MeterValueText classes="value" />
            <div style=track><MeterFill classes="fill" styles=fill() /></div>
        </Meter>
        <Meter value=0_i32 min_value=0 max_value=0 aria_label="Meter empty range">
            <MeterValueText classes="value" />
            <div style=track><MeterFill classes="fill" styles=fill() /></div>
        </Meter>
    }
}

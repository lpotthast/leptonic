use leptonic::atoms::prelude as atoms;
use leptos::prelude::*;

#[component]
pub fn MeterAtomDemo() -> impl IntoView {
    // Gigabytes used of 64.
    let used = RwSignal::new(41.5_f64);

    view! {
        <atoms::Meter
            value=used
            max_value=64.0
            // Replaces the percentage as value text (and `aria-valuetext`).
            value_label=MaybeProp::derive(move || Some(format!("{} of 64 GB", used.get())))
            classes="demo-value-bar-container"
        >
            <div class="demo-value-bar-header">
                <atoms::Label>"Storage"</atoms::Label>
                <atoms::MeterValueText />
            </div>
            <div class="demo-value-bar demo-value-bar-thick">
                <atoms::MeterFill classes="demo-value-bar-fill" />
            </div>
        </atoms::Meter>
        <div class="demo-inline-controls">
            <atoms::Button on_press=move |_| used.update(|gb| *gb = (*gb - 8.0).max(0.0)) classes="demo-btn">"Free 8 GB"</atoms::Button>
            <atoms::Button on_press=move |_| used.update(|gb| *gb = (*gb + 8.0).min(64.0)) classes="demo-btn">"Use 8 GB"</atoms::Button>
        </div>
    }
}

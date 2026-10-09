use leptonic::atoms::{
    button::Button,
    field::Label,
    meter::{Meter, MeterFill, MeterValueText},
};
use leptos::prelude::*;

#[component]
pub fn MeterStorageDemo() -> impl IntoView {
    // Gigabytes used of 64, shown as a custom value text.
    let used = RwSignal::new(41.5_f64);

    view! {
        <Meter
            value=used
            max_value=64.0
            value_label=MaybeProp::derive(move || Some(format!("{} of 64 GB", used.get())))
            classes="demo-value-bar-container"
        >
            <div class="demo-value-bar-header">
                <Label>"Storage"</Label>
                <MeterValueText/>
            </div>
            // The track is your own markup; the fill's width follows the value.
            <div class="demo-value-bar">
                <MeterFill classes="demo-value-bar-fill"/>
            </div>
        </Meter>
        <div class="demo-inline-controls">
            <Button on_press=move |_| used.update(|gb| *gb = (*gb - 8.0).max(0.0)) classes="demo-btn">"Free 8 GB"</Button>
            <Button on_press=move |_| used.update(|gb| *gb = (*gb + 8.0).min(64.0)) classes="demo-btn">"Use 8 GB"</Button>
        </div>
    }
}

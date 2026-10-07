use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn MeterStorageDemo() -> impl IntoView {
    // Gigabytes used of 64, shown as a custom value text.
    let used = RwSignal::new(41.5_f64);

    view! {
        <Meter
            value=used
            max_value=64.0
            label="Storage"
            value_label=MaybeProp::derive(move || Some(format!("{} of 64 GB", used.get())))
        />
        <div class="demo-inline-controls">
            <Button on_press=move |_| used.update(|gb| *gb = (*gb - 8.0).max(0.0))>"Free 8 GB"</Button>
            <Button on_press=move |_| used.update(|gb| *gb = (*gb + 8.0).min(64.0))>"Use 8 GB"</Button>
        </div>
    }
}

use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn MeterDiskDemo() -> impl IntoView {
    let (disk_usage, _set_disk_usage) = signal(72.5);

    let disk_meter = use_meter(UseMeterInput {
        value: disk_usage.into(),
        label: Some("Disk Usage".to_string()),
        min_value: 0.0,
        max_value: 100.0,
        ..Default::default()
    });

    view! {
        <div style="max-width: 300px;">
            <div style="display: flex; justify-content: space-between; margin-bottom: 0.5em;">
                <label id={disk_meter.label_props.id.clone()}>"Disk Usage"</label>
                <span>{ move || disk_meter.value_label.get() }</span>
            </div>
            <div
                {..disk_meter.meter_props.into_attrs()}
                style="height: 20px; background: #e0e0e0; border-radius: 4px; overflow: hidden;"
            >
                <div style=move || format!(
                    "height: 100%; width: {}%; background: {}; transition: width 0.3s;",
                    disk_meter.percentage.get(),
                    if disk_meter.percentage.get() > 80.0 { "#e53935" }
                    else if disk_meter.percentage.get() > 60.0 { "#fb8c00" }
                    else { "#43a047" }
                )></div>
            </div>
        </div>
    }
}

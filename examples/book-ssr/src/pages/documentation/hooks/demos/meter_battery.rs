use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn MeterBatteryDemo() -> impl IntoView {
    let (battery, _set_battery) = signal(45.0);

    let battery_meter = use_meter(UseMeterInput {
        value: battery.into(),
        label: Some("Battery".to_string()),
        min_value: 0.0,
        max_value: 100.0,
        format_options: Some(MeterFormatOptions {
            style: MeterFormatStyle::Percent,
            decimals: 0,
        }),
        ..Default::default()
    });

    view! {
        <div style="max-width: 300px;">
            <div style="display: flex; justify-content: space-between; margin-bottom: 0.5em;">
                <label id={battery_meter.label_props.id.clone()}>"Battery Level"</label>
                <span>{ move || battery_meter.value_label.get() }</span>
            </div>
            <div
                {..battery_meter.meter_props.into_attrs()}
                style="height: 20px; background: #e0e0e0; border-radius: 4px; overflow: hidden;"
            >
                <div style=move || format!(
                    "height: 100%; width: {}%; background: {}; transition: width 0.3s;",
                    battery_meter.percentage.get(),
                    if battery_meter.percentage.get() < 20.0 { "#e53935" }
                    else if battery_meter.percentage.get() < 50.0 { "#fb8c00" }
                    else { "#43a047" }
                )></div>
            </div>
        </div>
    }
}

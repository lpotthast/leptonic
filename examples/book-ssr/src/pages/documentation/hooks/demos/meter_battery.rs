use leptonic::{
    components::prelude::*,
    hooks::*,
    utils::{
        css::{CssDimension, NonNegativeLengthPercentage, Size, try_pct},
        style::WidthProperty,
        styles::Styles,
    },
};
use leptos::prelude::*;

#[component]
pub fn MeterBatteryDemo() -> impl IntoView {
    let (battery, set_battery) = signal(45.0);

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
    let percentage = battery_meter.percentage;

    // The fill width is the only dynamic style; the color comes from a class.
    let fill_styles =
        Styles::new().add_reactive(move || WidthProperty.declare(fill_width(percentage.get())));
    let fill_class = move || {
        let level = match percentage.get() {
            p if p < 20.0 => "danger",
            p if p < 50.0 => "warning",
            _ => "ok",
        };
        format!("demo-value-bar-fill {level}")
    };

    view! {
        <div class="demo-value-bar-container">
            <div class="demo-value-bar-header">
                <label id=battery_meter.label_props.id>"Battery Level"</label>
                <span>{move || battery_meter.value_label.get()}</span>
            </div>
            <div {..battery_meter.meter_props.into_attrs()} class="demo-value-bar demo-value-bar-thick">
                <div class=fill_class style=fill_styles></div>
            </div>
        </div>

        <div class="demo-inline-controls">
            <Button on_press=move |_| set_battery.update(|v| *v = f64::max(*v - 10.0, 0.0))>"Discharge"</Button>
            <Button on_press=move |_| set_battery.update(|v| *v = f64::min(*v + 10.0, 100.0))>"Charge"</Button>
        </div>
    }
}

/// The width of a fill covering `percent` of its track. Non-finite or negative input renders as zero width.
fn fill_width(percent: f64) -> Size {
    try_pct(percent)
        .ok()
        .and_then(|width| NonNegativeLengthPercentage::try_from(width).ok())
        .unwrap_or_else(|| NonNegativeLengthPercentage::new(CssDimension::Zero))
        .into()
}

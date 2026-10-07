use leptonic::{
    atoms::button::Button,
    hooks::*,
    utils::{
        css::{computed_pct, computed_size},
        style::WidthProperty,
        styles::Styles,
    },
};
use leptos::prelude::*;

#[component]
pub fn MeterDiskDemo() -> impl IntoView {
    let disk_usage = RwSignal::new(72.5_f64);

    let UseProgressBarReturn {
        props,
        label_props,
        percentage,
        value_text,
    } = use_meter(UseMeterInput::<f64> {
        value: disk_usage.into(),
        has_label: true.into(),
        ..UseMeterInput::default()
    });
    // A meter always has a value, so `percentage` is always `Some`.
    let percentage = Signal::derive(move || percentage.get().unwrap_or_default().as_percent());

    let fill_styles = Styles::new()
        .add_reactive(move || WidthProperty.declare(computed_size(computed_pct(percentage.get()))));
    // How full the disk is, for the fill color.
    let level = move || match percentage.get() {
        p if p > 80.0 => "critical",
        p if p > 60.0 => "warning",
        _ => "good",
    };

    view! {
        <div class="demo-value-bar-container">
            <div class="demo-value-bar-header">
                <span {..label_props.into_attrs()}>"Disk Usage"</span>
                <span>{value_text}</span>
            </div>
            <div {..props.into_attrs()} class="demo-value-bar demo-value-bar-thick">
                <div class="demo-value-bar-fill" data-level=level style=fill_styles></div>
            </div>
        </div>

        <div class="demo-inline-controls">
            <Button on_press=move |_| disk_usage.update(|v| *v = (*v - 10.0).max(0.0)) classes="demo-btn">"Free 10%"</Button>
            <Button on_press=move |_| disk_usage.update(|v| *v = (*v + 10.0).min(100.0)) classes="demo-btn">"Use 10%"</Button>
        </div>
    }
}

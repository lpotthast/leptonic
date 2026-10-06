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
pub fn ProgressDeterminateDemo() -> impl IntoView {
    let (value, set_value) = signal(65.0);

    let UseProgressBarReturn {
        progress_props,
        label_props,
        percentage,
        value_label,
        ..
    } = use_progress_bar(UseProgressBarInput {
        label: Some("Loading progress".into()),
        value: Signal::derive(move || Some(value.get())),
        min_value: 0.0,
        max_value: 100.0,
        show_value_label: true,
        is_indeterminate: false,
    });

    // The fill width is the only dynamic style; everything else is in CSS classes.
    let fill_styles = Styles::new()
        .add_reactive(move || WidthProperty.declare(fill_width(percentage.get().unwrap_or(0.0))));

    view! {
        <div class="demo-value-bar-container">
            <div class="demo-value-bar-header">
                <label id=label_props.id>"Loading progress"</label>
                <span>{move || value_label.get()}</span>
            </div>
            <div {..progress_props.into_attrs()} class="demo-value-bar">
                <div class="demo-value-bar-fill" style=fill_styles></div>
            </div>
        </div>

        <div class="demo-inline-controls">
            <Button on_press=move |_| set_value.update(|v| *v = f64::max(*v - 10.0, 0.0))>"-10%"</Button>
            <Button on_press=move |_| set_value.update(|v| *v = f64::min(*v + 10.0, 100.0))>"+10%"</Button>
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

use leptonic::{
    components::prelude::*,
    hooks::*,
    utils::{
        css::{computed_pct, computed_size},
        style::WidthProperty,
        styles::Styles,
    },
};
use leptos::prelude::*;

#[component]
pub fn ProgressBarHookDemo() -> impl IntoView {
    let loaded = RwSignal::new(60.0_f64);
    let duration_unknown = RwSignal::new(false);

    let UseProgressBarReturn {
        props,
        label_props,
        percentage,
        value_text,
    } = use_progress_bar(UseProgressBarInput {
        // `None`: the progress isn't known, the bar is indeterminate.
        value: Signal::derive(move || (!duration_unknown.get()).then(|| loaded.get())),
        has_label: true.into(),
        ..UseProgressBarInput::default()
    });

    // Sized while the progress is known; while indeterminate, the CSS animates the fill.
    let fill_styles = Styles::new().add_optional(move || {
        percentage
            .get()
            .map(|percentage| WidthProperty.declare(computed_size(computed_pct(percentage))))
    });

    view! {
        <div class="demo-value-bar-container">
            <div class="demo-value-bar-header">
                <span {..label_props.into_attrs()}>"Loading"</span>
                <span>{value_text}</span>
            </div>
            <div {..props.into_attrs()} class="demo-value-bar">
                <div class="demo-value-bar-fill" style=fill_styles></div>
            </div>
        </div>

        <div class="demo-inline-controls">
            <Button on_press=move |_| loaded.update(|v| *v = (*v - 10.0).max(0.0))>"Back 10%"</Button>
            <Button on_press=move |_| loaded.update(|v| *v = (*v + 10.0).min(100.0))>"Ahead 10%"</Button>
        </div>
        <div class="demo-controls">
            <Checkbox is_selected=duration_unknown set_selected=duration_unknown>"Duration unknown"</Checkbox>
        </div>
    }
}

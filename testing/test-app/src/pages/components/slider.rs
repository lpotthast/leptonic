use leptonic::{components::slider::Slider, hooks::SliderMarks, utils::orientation::Orientation};
use leptos::prelude::*;

/// The styled `Slider`'s theme states: a disabled slider, and a vertical one with marks.
#[component]
pub fn PageComponentSlider() -> impl IntoView {
    let disabled = RwSignal::new(40.0);
    let vertical = RwSignal::new(100.0);
    view! {
        <div id="test-page-component-slider">
            <h1>"Slider components"</h1>
            <button id="test-cslider-before">"Before"</button>
            <div id="test-cslider-horizontal">
                <Slider value=disabled set_value=disabled min_value=0.0 max_value=100.0 aria_label="Horizontal" />
            </div>
            <div id="test-cslider-disabled">
                <Slider
                    value=disabled
                    set_value=disabled
                    min_value=0.0
                    max_value=100.0
                    is_disabled=true
                    aria_label="Disabled"
                />
            </div>
            <div id="test-cslider-vertical">
                <Slider
                    value=vertical
                    set_value=vertical
                    min_value=0.0
                    max_value=100.0
                    step=25.0
                    orientation=Orientation::Vertical
                    marks=SliderMarks::Automatic { create_names: false }
                    aria_label="Vertical"
                />
            </div>
        </div>
    }
}

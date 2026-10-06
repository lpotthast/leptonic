use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn SliderBasicDemo() -> impl IntoView {
    // The value type is the slider's: integers step by whole numbers, floats by any step.
    let percent = RwSignal::new(50_u8);
    let fraction = RwSignal::new(0.5_f64);

    view! {
        <Slider value=percent set_value=percent min_value=0 max_value=100 aria_label="Percent" />
        <p class="demo-status">{move || format!("u8, step 1: {}.", percent.get())}</p>

        <Slider value=fraction set_value=fraction min_value=0.0 max_value=1.0 step=0.0001 aria_label="Fraction" />
        <p class="demo-status">{move || format!("f64, step 0.0001: {:.4}.", fraction.get())}</p>
    }
}

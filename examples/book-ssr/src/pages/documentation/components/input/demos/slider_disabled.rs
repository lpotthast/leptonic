use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn SliderDisabledDemo() -> impl IntoView {
    let volume = RwSignal::new(40_u8);
    let disabled = RwSignal::new(true);

    view! {
        <Slider value=volume set_value=volume min_value=0 max_value=100 is_disabled=disabled aria_label="Volume"/>
        <p class="demo-status">{move || format!("Volume: {}.", volume.get())}</p>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}

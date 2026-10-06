use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn SliderVariantsDemo() -> impl IntoView {
    let round = RwSignal::new(30_u8);
    let block = RwSignal::new(70_u8);

    view! {
        <div class="demo-control-stack">
            <Slider value=round set_value=round min_value=0 max_value=100 aria_label="Round thumb"/>
            <Slider value=block set_value=block min_value=0 max_value=100 variant=SliderVariant::Block aria_label="Block thumb"/>
        </div>
        <p class="demo-status">{move || format!("Round: {}. Block: {}.", round.get(), block.get())}</p>
    }
}

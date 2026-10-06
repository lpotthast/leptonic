use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn SliderPopoverDemo() -> impl IntoView {
    let on_interaction = RwSignal::new(50_u8);
    let always = RwSignal::new(30_u8);
    let range = RwSignal::new(20_u8..=80);

    view! {
        <div class="demo-control-stack">
            <Slider
                value=on_interaction
                set_value=on_interaction
                min_value=0
                max_value=100
                popover=SliderPopover::When { hovered: true, dragged: true }
                aria_label="Shown on interaction"
            />
            <Slider value=always set_value=always min_value=0 max_value=100 popover=SliderPopover::Always aria_label="Always shown"/>
            <RangeSlider
                value=range
                set_value=range
                min_value=0
                max_value=100
                popover=SliderPopover::When { hovered: true, dragged: true }
                aria_label="Range"
            />
        </div>
        <p class="demo-status">
            {move || format!(
                "Values: {}, {} and {} to {}.",
                on_interaction.get(),
                always.get(),
                range.with(|range| *range.start()),
                range.with(|range| *range.end()),
            )}
        </p>
    }
}

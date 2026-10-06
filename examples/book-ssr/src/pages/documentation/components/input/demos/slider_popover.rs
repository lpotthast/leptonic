use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn SliderPopoverDemo() -> impl IntoView {
    let (on_interaction, set_on_interaction) = signal(50.0);
    let (always, set_always) = signal(30.0);
    let (range_from, set_range_from) = signal(20.0);
    let (range_to, set_range_to) = signal(80.0);

    view! {
        <div class="demo-control-stack">
            <Slider min=0.0 max=100.0 step=1.0
                value=on_interaction set_value=set_on_interaction
                popover=SliderPopover::When { hovered: true, dragged: true }
                value_display=move |v| format!("{v:.0}")/>

            <Slider min=0.0 max=100.0 step=1.0
                value=always set_value=set_always
                popover=SliderPopover::Always
                value_display=move |v| format!("{v:.0}")/>

            <RangeSlider
                value_a=range_from set_value_a=set_range_from
                value_b=range_to set_value_b=set_range_to
                min=0.0 max=100.0 step=1.0
                popover=SliderPopover::When { hovered: true, dragged: true }
                value_display=move |v| format!("{v:.0}")/>
        </div>
    }
}

use leptonic::atoms::{
    field::Label,
    slider::{
        Slider, SliderFill, SliderOutput, SliderPopover, SliderThumb, SliderThumbTooltip,
        SliderTrack,
    },
};
use leptos::prelude::*;

#[component]
pub fn SliderRangeDemo() -> impl IntoView {
    let price = RwSignal::new(vec![20_u16, 80]);
    // Show a thumb's value while it is hovered or dragged.
    let popover = SliderPopover::OnHoverOrDrag;

    view! {
        <Slider min_value=0 max_value=100 values=price set_values=price classes=["demo-slider", "demo-slider-blue"]>
            <Label classes="demo-slider-label">"Price"</Label>
            <SliderTrack classes="demo-slider-track">
                <SliderFill classes="demo-slider-fill"/>
                <SliderThumb index=0 aria_label="Minimum" classes="demo-slider-thumb">
                    <SliderThumbTooltip popover classes="demo-slider-tooltip"/>
                </SliderThumb>
                <SliderThumb index=1 aria_label="Maximum" classes="demo-slider-thumb">
                    <SliderThumbTooltip popover classes="demo-slider-tooltip"/>
                </SliderThumb>
            </SliderTrack>
            <SliderOutput classes=["demo-slider-output", "demo-slider-output-wide"]/>
        </Slider>

        <p class="demo-status">
            {move || price.with(|price| match price.as_slice() {
                [min, max] => format!("Selected: {min} to {max} euros."),
                _ => String::new(),
            })}
        </p>
    }
}

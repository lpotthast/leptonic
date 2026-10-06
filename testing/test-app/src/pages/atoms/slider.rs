use leptonic::{
    atoms::slider::{Slider, SliderPopover, SliderThumb, SliderThumbTooltip, SliderTrack},
    hooks::SliderValues,
};
use leptos::prelude::*;

/// Sliders with value tooltips: always visible, and visible while hovered or dragged.
#[component]
pub fn PageAtomSlider() -> impl IntoView {
    view! {
        <div id="test-page-atom-slider">
            <h1>"Slider"</h1>
            <Slider values=SliderValues::Uncontrolled(vec![30.0]) aria_label="Always">
                <SliderTrack>
                    <SliderThumb aria_label="Always">
                        <SliderThumbTooltip popover=SliderPopover::Always classes="tooltip" />
                    </SliderThumb>
                </SliderTrack>
            </Slider>
            <Slider values=SliderValues::Uncontrolled(vec![60.0]) aria_label="On hover">
                <SliderTrack>
                    <SliderThumb aria_label="On hover">
                        <SliderThumbTooltip
                            popover=SliderPopover::When { hovered: true, dragged: true }
                            classes="tooltip"
                        />
                    </SliderThumb>
                </SliderTrack>
            </Slider>
        </div>
    }
}

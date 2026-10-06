use leptonic::atoms::{
    field::Label,
    slider::{
        Slider, SliderFill, SliderOutput, SliderPopover, SliderThumb, SliderThumbTooltip,
        SliderTrack,
    },
};
use leptos::prelude::*;

/// Slider atoms (react-aria-components' `Slider.test.js` setups):
/// - "Volume": labelled, one `f64` thumb at 30 (0..=100, step 5) with an output and a fill from
///   `offset` 50; `#test-slider-volume-ends` counts `on_change_end`.
/// - "Price": two `i32` thumbs bound to app state (`#test-slider-price` shows it).
/// - "Vertical" (one thumb at 50) and "Disabled".
/// - Tooltips: "Always" and "On hover".
#[component]
pub fn PageAtomSlider() -> impl IntoView {
    let ends = RwSignal::new(0u32);
    let price = RwSignal::new(vec![20_i32, 80]);

    view! {
        <div id="test-page-atom-slider">
            <style>
                ".test-slider-track { width: 200px; height: 20px; background: #ddd; margin: 20px; }
                .test-slider-track-vertical { width: 20px; height: 200px; background: #ddd; margin: 20px; }
                #test-slider-volume-fill { height: 100%; }
                .test-slider-thumb { width: 16px; height: 16px; background: #333; border-radius: 50%; top: 50%; }
                .test-slider-track-vertical .test-slider-thumb { left: 50%; }
                .tooltip:not([data-visible]) { display: none; }"
            </style>
            <Slider
                min_value=0.0
                max_value=100.0
                step=5.0
                default_values=vec![30.0_f64]
                on_change_end=move |_| ends.update(|n| *n += 1)
                id="test-slider-volume"
            >
                <Label>"Volume"</Label>
                <SliderOutput attr:id="test-slider-volume-output" />
                <SliderTrack classes="test-slider-track" attr:id="test-slider-volume-track">
                    <SliderFill offset=50.0 attr:id="test-slider-volume-fill" />
                    <SliderThumb classes="test-slider-thumb" />
                </SliderTrack>
            </Slider>
            <div>"Ends: " <span id="test-slider-volume-ends">{ends}</span></div>

            <Slider min_value=0 max_value=100 values=price set_values=price aria_label="Price">
                <SliderOutput attr:id="test-slider-price-output" />
                <SliderTrack classes="test-slider-track">
                    <SliderThumb index=0 aria_label="Minimum" classes="test-slider-thumb" />
                    <SliderThumb index=1 aria_label="Maximum" classes="test-slider-thumb" />
                </SliderTrack>
            </Slider>
            <div>"Price: " <span id="test-slider-price">{move || format!("{:?}", price.get())}</span></div>

            <Slider
                min_value=0
                max_value=100
                default_values=vec![50_i32]
                orientation=leptonic::utils::orientation::Orientation::Vertical
                aria_label="Vertical"
            >
                <SliderTrack classes="test-slider-track-vertical">
                    <SliderThumb classes="test-slider-thumb" />
                </SliderTrack>
            </Slider>

            <Slider min_value=0 max_value=10 default_values=vec![0_i32] is_disabled=true aria_label="Disabled">
                <SliderTrack classes="test-slider-track">
                    <SliderThumb classes="test-slider-thumb" />
                </SliderTrack>
            </Slider>

            <Slider min_value=0 max_value=100 default_values=vec![30_i32] aria_label="Always">
                <SliderTrack classes="test-slider-track">
                    <SliderThumb classes="test-slider-thumb">
                        <SliderThumbTooltip popover=SliderPopover::Always classes="tooltip" />
                    </SliderThumb>
                </SliderTrack>
            </Slider>
            <Slider min_value=0 max_value=100 default_values=vec![60_i32] aria_label="On hover">
                <SliderTrack classes="test-slider-track">
                    <SliderThumb classes="test-slider-thumb">
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

use leptonic::{
    NumberFormatOptions, NumberStyle,
    atoms::{
        field::Label,
        slider::{
            Slider, SliderFill, SliderOutput, SliderPopover, SliderThumb, SliderThumbTooltip,
            SliderTrack,
        },
    },
};
use leptos::prelude::*;

/// Slider atoms (react-aria-components' `Slider.test.js` setups):
/// - "Volume": labelled, one `f64` thumb at 30 (0..=100, step 5) with an output and a fill from
///   `offset` 50; `#test-slider-volume-ends` counts `on_change_end`.
/// - "Price": two `i32` thumbs bound to app state (`#test-slider-price` shows it).
/// - "Vertical" (one thumb at 50, with a fill) and "Disabled"; "Vertical offset" (at 30, a fill
///   from 50).
/// - Tooltips: "Always" (with a fill from the start) and "On hover".
/// - "Percent": an `f64` slider from 0 to 1 (step 0.01) formatted as percentages, at 0.2;
///   "Percent range": two thumbs at 0.2 and 0.6.
/// - `#test-slider-form`: "Reset bound" (one thumb bound to app state at 10), "Reset range" (two
///   bound thumbs at 10 and 40) and "Reset own" (at 10), with a reset button.
#[component]
pub fn PageAtomSlider() -> impl IntoView {
    let ends = RwSignal::new(0u32);
    let price = RwSignal::new(vec![20_i32, 80]);
    let reset_bound = RwSignal::new(vec![10_i32]);
    let reset_range = RwSignal::new(vec![10_i32, 40]);
    let percent = || NumberFormatOptions {
        style: NumberStyle::Percent,
        ..NumberFormatOptions::default()
    };

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
                orientation=leptonic::Orientation::Vertical
                aria_label="Vertical"
            >
                <SliderTrack classes="test-slider-track-vertical">
                    <SliderFill attr:id="test-slider-vertical-fill" />
                    <SliderThumb classes="test-slider-thumb" />
                </SliderTrack>
            </Slider>
            <Slider
                min_value=0
                max_value=100
                default_values=vec![30_i32]
                orientation=leptonic::Orientation::Vertical
                aria_label="Vertical offset"
            >
                <SliderTrack classes="test-slider-track-vertical">
                    <SliderFill offset=50.0 attr:id="test-slider-vertical-offset-fill" />
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
                    <SliderFill attr:id="test-slider-always-fill" />
                    <SliderThumb classes="test-slider-thumb">
                        <SliderThumbTooltip popover=SliderPopover::Always classes="tooltip" />
                    </SliderThumb>
                </SliderTrack>
            </Slider>
            <Slider min_value=0 max_value=100 default_values=vec![60_i32] aria_label="On hover">
                <SliderTrack classes="test-slider-track">
                    <SliderThumb classes="test-slider-thumb">
                        <SliderThumbTooltip popover=SliderPopover::OnHoverOrDrag classes="tooltip" />
                    </SliderThumb>
                </SliderTrack>
            </Slider>
            <Slider
                min_value=0.0
                max_value=1.0
                step=0.01
                default_values=vec![0.2_f64]
                format_options=percent()
                aria_label="Percent"
            >
                <SliderOutput />
                <SliderTrack classes="test-slider-track">
                    <SliderThumb classes="test-slider-thumb" />
                </SliderTrack>
            </Slider>
            <Slider
                min_value=0.0
                max_value=1.0
                step=0.01
                default_values=vec![0.2_f64, 0.6]
                format_options=percent()
                aria_label="Percent range"
            >
                <SliderOutput />
                <SliderTrack classes="test-slider-track">
                    <SliderThumb index=0 classes="test-slider-thumb" />
                    <SliderThumb index=1 classes="test-slider-thumb" />
                </SliderTrack>
            </Slider>

            <form id="test-slider-form">
                <Slider min_value=0 max_value=100 values=reset_bound set_values=reset_bound aria_label="Reset bound">
                    <SliderTrack classes="test-slider-track">
                        <SliderThumb classes="test-slider-thumb" />
                    </SliderTrack>
                </Slider>
                <Slider min_value=0 max_value=100 values=reset_range set_values=reset_range aria_label="Reset range">
                    <SliderTrack classes="test-slider-track">
                        <SliderThumb index=0 classes="test-slider-thumb" />
                        <SliderThumb index=1 classes="test-slider-thumb" />
                    </SliderTrack>
                </Slider>
                <Slider min_value=0 max_value=100 default_values=vec![10_i32] aria_label="Reset own">
                    <SliderTrack classes="test-slider-track">
                        <SliderThumb classes="test-slider-thumb" />
                    </SliderTrack>
                </Slider>
                <button type="reset" id="test-slider-reset">"Reset"</button>
            </form>
        </div>
    }
}

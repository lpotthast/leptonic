use leptonic::{
    atoms::{
        field::Label,
        slider::{Slider, SliderOutput, SliderThumb, SliderTrack},
    },
    utils::{
        i18n::{I18nProvider, Locale},
        orientation::Orientation,
    },
};
use leptos::prelude::*;

/// Slider interactions (react-aria's `useSlider.test.js`/`useSliderThumb.test.js` and
/// react-aria-components' `Slider.test.js`). Every slider is named by its `aria-label`, ranges
/// from 0 to 100 in steps of 1 and has a track of 200 pixels (a value is half a pixel offset) with
/// 8-pixel thumbs, so that presses on the track don't hit a thumb. A logged slider writes
/// `change:[..]` and `end:[..]` to `#<name>-log`, separated by `;`.
///
/// - Track presses and drags: "track", "drag", "stacked-before", "stacked-after", "many-before",
///   "many-after", "disabled" (`is_disabled`), "vertical" (a 200-pixel tall track).
/// - "rtl": in Hebrew (right to left).
/// - Keys: "keys" (one thumb at 10), "keys-vertical" (vertical, at 10), "page" (at 20).
/// - "thumb-disabled": the second of two thumbs is disabled.
/// - "three": three thumbs with an output (`#three-output`).
/// - "controlled": two thumbs bound to app state; `#controlled-reset` sets it to `[0, 100]`.
/// - "form": a thumb of the form `test-form`; "labels": a labelled slider whose thumbs have their
///   own `Label`s ("Min", "Max") or an `aria-label`; "attributes": an `f32` slider (step 0.1) whose
///   thumb is required, invalid and has an error message and details.
/// - "restricted": app values `[-20, 150]` out of the range; "missing": one value, two thumbs.
#[component]
pub fn PageAtomSliderInteractions() -> impl IntoView {
    let controlled = RwSignal::new(vec![30_i32, 60]);
    let restricted = RwSignal::new(vec![-20_i32, 150]);
    let hebrew: Locale = "he".parse().expect("a valid locale");

    view! {
        <div id="test-page-atom-slider-interactions">
            <style>
                ".track { position: relative; width: 200px; height: 20px; background: #ddd; margin: 16px; }
                .track-vertical { position: relative; width: 20px; height: 200px; background: #ddd; margin: 16px; }
                .thumb { width: 8px; height: 8px; background: #333; top: 50%; }
                .track-vertical .thumb { left: 50%; }"
            </style>
            <button id="before">"Before"</button>
            {logged("track", vec![10, 80], Orientation::Horizontal, false)}
            {logged("drag", vec![10, 80], Orientation::Horizontal, false)}
            {logged("stacked-before", vec![40, 40], Orientation::Horizontal, false)}
            {logged("stacked-after", vec![40, 40], Orientation::Horizontal, false)}
            {logged("many-before", vec![25, 25, 50, 75, 75], Orientation::Horizontal, false)}
            {logged("many-after", vec![25, 25, 50, 75, 75], Orientation::Horizontal, false)}
            {logged("disabled", vec![10, 80], Orientation::Horizontal, true)}
            {logged("vertical", vec![10, 80], Orientation::Vertical, false)}
            <I18nProvider locale=hebrew>{logged("rtl", vec![10, 80], Orientation::Horizontal, false)}</I18nProvider>
            {logged("keys", vec![10], Orientation::Horizontal, false)}
            {logged("keys-vertical", vec![10], Orientation::Vertical, false)}
            {logged("page", vec![20], Orientation::Horizontal, false)}

            <Slider min_value=0 max_value=100 default_values=vec![10_i32, 80] aria_label="thumb-disabled">
                <SliderTrack classes="track" attr:id="thumb-disabled-track">
                    <SliderThumb index=0 classes="thumb" />
                    <SliderThumb index=1 is_disabled=true classes="thumb" />
                </SliderTrack>
            </Slider>

            <Slider min_value=0 max_value=100 default_values=vec![30_i32, 60, 80] aria_label="three">
                <SliderOutput attr:id="three-output" />
                <SliderTrack classes="track">
                    <SliderThumb index=0 classes="thumb" />
                    <SliderThumb index=1 classes="thumb" />
                    <SliderThumb index=2 classes="thumb" />
                </SliderTrack>
            </Slider>

            <button id="controlled-before">"Before controlled"</button>
            <Slider min_value=0 max_value=100 values=controlled set_values=controlled aria_label="controlled">
                <SliderTrack classes="track">
                    <SliderThumb index=0 classes="thumb" />
                    <SliderThumb index=1 classes="thumb" />
                </SliderTrack>
            </Slider>
            <button id="controlled-reset" on:click=move |_| controlled.set(vec![0, 100])>"Reset"</button>

            <form id="test-form"></form>
            <Slider min_value=0 max_value=100 default_values=vec![50_i32] aria_label="form">
                <SliderTrack classes="track">
                    <SliderThumb form="test-form" name="volume" classes="thumb" />
                </SliderTrack>
            </Slider>

            <Slider min_value=0 max_value=100 default_values=vec![20_i32, 70] id="labels">
                <Label>"Price"</Label>
                <SliderTrack classes="track">
                    <SliderThumb index=0 classes="thumb"><Label>"Min"</Label></SliderThumb>
                    <SliderThumb index=1 aria_label="Max" classes="thumb" />
                </SliderTrack>
            </Slider>

            <span id="attributes-error">"Too loud"</span>
            <span id="attributes-details">"Details"</span>
            <Slider
                min_value=0.0
                max_value=1.0
                step=0.1
                default_values=vec![0.1_f32]
                is_required=true
                aria_label="attributes"
                aria_details="attributes-details"
            >
                <SliderTrack classes="track">
                    <SliderThumb
                        is_invalid=true
                        aria_errormessage="attributes-error"
                        classes="thumb"
                    />
                </SliderTrack>
            </Slider>

            <Slider min_value=0 max_value=100 values=restricted set_values=restricted aria_label="restricted">
                <SliderOutput attr:id="restricted-output" />
                <SliderTrack classes="track">
                    <SliderThumb index=0 classes="thumb" />
                    <SliderThumb index=1 classes="thumb" />
                </SliderTrack>
            </Slider>

            <Slider min_value=0 max_value=100 default_values=vec![30_i32] aria_label="missing">
                <SliderTrack classes="track">
                    <SliderThumb index=0 classes="thumb" />
                    <SliderThumb index=1 classes="thumb" />
                </SliderTrack>
            </Slider>
        </div>
    }
}

/// A slider named `name` whose changes are logged to `#<name>-log`, one thumb per value.
fn logged(
    name: &'static str,
    values: Vec<i32>,
    orientation: Orientation,
    is_disabled: bool,
) -> impl IntoView {
    let log = RwSignal::new(Vec::<String>::new());
    let thumbs = values.len();
    let track = match orientation {
        Orientation::Horizontal => "track",
        Orientation::Vertical => "track-vertical",
    };
    view! {
        <div id=name>
            <Slider
                min_value=0
                max_value=100
                default_values=values
                orientation=orientation
                is_disabled=is_disabled
                aria_label=name
                on_change={move |v: Vec<i32>| log.update(|l| l.push(format!("change:{v:?}")))}
                on_change_end={move |v: Vec<i32>| log.update(|l| l.push(format!("end:{v:?}")))}
            >
                <SliderTrack classes=track attr:id=format!("{name}-track")>
                    {(0..thumbs)
                        .map(|index| view! { <SliderThumb index=index classes="thumb" /> })
                        .collect_view()}
                </SliderTrack>
            </Slider>
            <div>"Log: " <span id=format!("{name}-log")>{move || log.get().join(";")}</span></div>
        </div>
    }
}

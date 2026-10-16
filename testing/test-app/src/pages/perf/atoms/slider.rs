//! The `Slider` (`#perf-slider`, 0 to 100 in steps of 1) with a `Label`, a `SliderOutput` and a
//! 200px `SliderTrack` (`#perf-slider-track`) with a `SliderFill` and one thumb (at 30) or two
//! (at 20 and 80), for its performance test (`leptonic/tests/perf_tests/atoms/slider.rs`): its
//! values controlled by app state (`values` + `set_values`), between two plain buttons
//! (`#perf-before`, `#perf-after`), mounted with one thumb at first. Its controls ([`Control`])
//! mount it (with one or two thumbs) and unmount it, set the app's values, and change the
//! slider's reactive props.

use leptonic::{
    NumberFormatOptions, Orientation,
    atoms::{
        field::Label,
        slider::{Slider, SliderFill, SliderOutput, SliderThumb, SliderTrack},
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The track (200 × 20 pixels) and its thumbs.
const STYLE: &str = "#perf-slider-track { position: relative; width: 200px; height: 20px; \
                     background: #ddd; margin: 20px; } \
                     .perf-thumb { width: 16px; height: 16px; background: #333; top: 50%; }";

#[component]
pub fn PagePerf() -> impl IntoView {
    // The number of thumbs of the mounted slider.
    let thumbs = RwSignal::new(Some(1_usize));
    let values = RwSignal::new(vec![30.0_f64]);
    let max = RwSignal::new(100.0_f64);
    let step = RwSignal::new(1.0_f64);
    let orientation = RwSignal::new(Orientation::Horizontal);
    let format = RwSignal::new(NumberFormatOptions::default());
    let disabled = RwSignal::new(false);
    let required = RwSignal::new(false);
    let invalid = RwSignal::new(false);
    let controls = vec![
        Control::action("mount", move || thumbs.set(Some(1))),
        Control::action("mount-range", move || thumbs.set(Some(2))),
        Control::action("unmount", move || thumbs.set(None)),
        Control::action("set-range-values", move || values.set(vec![20.0, 80.0])),
        Control::action("set-value", move || values.set(vec![60.0])),
        Control::action("reset-value", move || values.set(vec![30.0])),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("make-vertical", move || {
            orientation.set(Orientation::Vertical);
        }),
        Control::action("make-horizontal", move || {
            orientation.set(Orientation::Horizontal);
        }),
        Control::action("widen", move || max.set(200.0)),
        Control::action("narrow", move || max.set(100.0)),
        Control::action("coarsen", move || step.set(5.0)),
        Control::action("refine", move || step.set(1.0)),
        Control::action("format-fraction", move || {
            format.set(NumberFormatOptions {
                minimum_fraction_digits: Some(1),
                ..NumberFormatOptions::default()
            });
        }),
        Control::action("format-integer", move || {
            format.set(NumberFormatOptions::default());
        }),
        Control::action("require", move || required.set(true)),
        Control::action("make-optional", move || required.set(false)),
        Control::action("invalidate", move || invalid.set(true)),
        Control::action("validate", move || invalid.set(false)),
    ];
    view! {
        <h1>"Slider performance"</h1>
        <style>{STYLE}</style>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        {move || {
            thumbs
                .get()
                .map(|count| {
                    view! {
                        <Slider
                            id="perf-slider"
                            min_value=0.0
                            max_value=max
                            step=step
                            values=values
                            set_values=values
                            orientation=orientation
                            format_options=format
                            is_disabled=disabled
                            is_required=required
                            is_invalid=invalid
                        >
                            <Label>"Volume"</Label>
                            <SliderOutput />
                            <SliderTrack attr:id="perf-slider-track">
                                <SliderFill />
                                {(0..count)
                                    .map(|index| {
                                        view! { <SliderThumb index classes="perf-thumb" /> }
                                    })
                                    .collect_view()}
                            </SliderTrack>
                        </Slider>
                    }
                })
        }}
        <button id="perf-after">"After"</button>
    }
}

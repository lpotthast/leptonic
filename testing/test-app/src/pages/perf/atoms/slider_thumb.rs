//! The `SliderThumb` (`#perf-slider-thumb`, 16 × 16 pixels, with its hidden input) on the 200px
//! `SliderTrack` of a `Slider` (0 to 100 in steps of 1, at 30), for its performance test
//! (`leptonic/tests/perf_tests/atoms/slider_thumb.rs`): the slider's values controlled by app
//! state (`values` + `set_values`), between two plain buttons (`#perf-before`, `#perf-after`), the
//! thumb mounted at first. Its controls ([`Control`]) mount and unmount the thumb (the slider and
//! its track stay), change the thumb's reactive props, and read the value (`value`) and how often
//! a change ended (`change-ends`).

use leptonic::atoms::slider::{Slider, SliderThumb, SliderTrack};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The track (200 × 20 pixels) and its thumb.
const STYLE: &str = "#perf-slider-track { position: relative; width: 200px; height: 20px; \
                     background: #ddd; margin: 20px; } \
                     .perf-thumb { width: 16px; height: 16px; background: #333; top: 50%; }";

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let values = RwSignal::new(vec![30.0_f64]);
    let disabled = RwSignal::new(false);
    let required = RwSignal::new(false);
    let invalid = RwSignal::new(false);
    let change_ends = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("require", move || required.set(true)),
        Control::action("make-optional", move || required.set(false)),
        Control::action("invalidate", move || invalid.set(true)),
        Control::action("validate", move || invalid.set(false)),
        Control::read("value", move || {
            values.with_untracked(|values| values.first().copied().unwrap_or(f64::NAN))
        }),
        Control::read("change-ends", move || f64::from(change_ends.get_value())),
    ];
    view! {
        <h1>"SliderThumb performance"</h1>
        <style>{STYLE}</style>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Slider
            min_value=0.0
            max_value=100.0
            values=values
            set_values=values
            on_change_end=move |_| change_ends.update_value(|count| *count += 1)
            aria_label="Volume"
        >
            <SliderTrack attr:id="perf-slider-track">
                <Show when=move || mounted.get()>
                    <SliderThumb
                        attr:id="perf-slider-thumb"
                        classes="perf-thumb"
                        is_disabled=disabled
                        is_required=required
                        is_invalid=invalid
                    />
                </Show>
            </SliderTrack>
        </Slider>
        <button id="perf-after">"After"</button>
    }
}

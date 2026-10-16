//! The `SliderThumbTooltip` (`#perf-slider-thumb-tooltip`, shown on hover or drag) in the thumb
//! (`#perf-slider-thumb`) on the 200px `SliderTrack` of a `Slider` (0 to 100 in steps of 1, at
//! 30), for its performance test (`leptonic/tests/perf_tests/atoms/slider_thumb_tooltip.rs`): the
//! slider's values controlled by app state (`values` + `set_values`), between two plain buttons
//! (`#perf-before`, `#perf-after`), the tooltip mounted at first. Its controls ([`Control`]) mount
//! and unmount the tooltip (the thumb stays) and set the value (60, or 30 again).

use leptonic::atoms::slider::{
    Slider, SliderPopover, SliderThumb, SliderThumbTooltip, SliderTrack,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The track (200 × 20 pixels), its thumb and the tooltip above it.
const STYLE: &str = "#perf-slider-track { position: relative; width: 200px; height: 20px; \
                     background: #ddd; margin: 40px 20px 20px; } \
                     .perf-thumb { width: 16px; height: 16px; background: #333; top: 50%; } \
                     #perf-slider-thumb-tooltip { position: absolute; bottom: 20px; } \
                     #perf-slider-thumb-tooltip:not([data-visible]) { display: none; }";

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let values = RwSignal::new(vec![30.0_f64]);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("set-value", move || values.set(vec![60.0])),
        Control::action("reset-value", move || values.set(vec![30.0])),
        Control::read("value", move || {
            values.with_untracked(|values| values.first().copied().unwrap_or(f64::NAN))
        }),
    ];
    view! {
        <h1>"SliderThumbTooltip performance"</h1>
        <style>{STYLE}</style>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Slider min_value=0.0 max_value=100.0 values=values set_values=values aria_label="Volume">
            <SliderTrack attr:id="perf-slider-track">
                <SliderThumb attr:id="perf-slider-thumb" classes="perf-thumb">
                    <Show when=move || mounted.get()>
                        <SliderThumbTooltip
                            attr:id="perf-slider-thumb-tooltip"
                            popover=SliderPopover::OnHoverOrDrag
                        />
                    </Show>
                </SliderThumb>
            </SliderTrack>
        </Slider>
        <button id="perf-after">"After"</button>
    }
}

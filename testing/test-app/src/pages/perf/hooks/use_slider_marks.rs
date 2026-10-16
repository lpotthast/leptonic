//! `use_slider_marks` alone, its marks rendered as plain `<div>`s (`.perf-mark`, with
//! `data-in-range`) in `#perf-slider-marks`, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_slider_marks.rs`): a range of 0 to 100 in steps of 1, a
//! value of 30, automatic named marks (21: every fifth step) or three custom ones (Low, Middle,
//! High), the automatic marks mounted at first. Its controls ([`Control`]) mount the marks
//! (automatic or custom) and unmount them, and change the hook's inputs: the value (60, or 30
//! again), the maximum (200, or 100 again) and the step (10, or 1 again).

use leptonic::hooks::slider::{
    ComputedSliderMark, CustomSliderMark, SliderMarkPlacement, SliderMarkValue,
    UseSliderMarksInput, use_slider_marks,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The placement of the mounted marks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Marks {
    /// Named marks along the steps.
    Automatic,
    /// Low (0), Middle (50) and High (100).
    Custom,
}

/// The placement of `marks`.
fn placement(marks: Marks) -> SliderMarkPlacement {
    match marks {
        Marks::Automatic => SliderMarkPlacement::Automatic { create_names: true },
        Marks::Custom => SliderMarkPlacement::Custom {
            marks: vec![
                CustomSliderMark {
                    value: SliderMarkValue::Value(0.0),
                    name: "Low".into(),
                },
                CustomSliderMark {
                    value: SliderMarkValue::Value(50.0),
                    name: "Middle".into(),
                },
                CustomSliderMark {
                    value: SliderMarkValue::Value(100.0),
                    name: "High".into(),
                },
            ],
        },
    }
}

/// The marks: `use_slider_marks`, each mark a `<div>` with its name.
#[component]
fn Subject(
    marks: Marks,
    #[prop(into)] values: Signal<Vec<f64>>,
    #[prop(into)] max: Signal<f64>,
    #[prop(into)] step: Signal<f64>,
) -> impl IntoView {
    let computed = use_slider_marks(UseSliderMarksInput {
        min_value: Signal::stored(0.0),
        max_value: max,
        step,
        values,
        marks: placement(marks),
        format: Callback::new(|value: f64| value.to_string()),
    });
    view! {
        <div id="perf-slider-marks">
            <For
                each=move || computed.get()
                key=|mark| mark.value.to_bits()
                children=|mark: ComputedSliderMark| {
                    let name = mark.name.clone().unwrap_or_default();
                    view! {
                        <div
                            class="perf-mark"
                            data-in-range=move || mark.is_in_range().then_some("true")
                        >
                            {name}
                        </div>
                    }
                }
            />
        </div>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let marks = RwSignal::new(Some(Marks::Automatic));
    let values = RwSignal::new(vec![30.0_f64]);
    let max = RwSignal::new(100.0_f64);
    let step = RwSignal::new(1.0_f64);
    let controls = vec![
        Control::action("mount", move || marks.set(Some(Marks::Automatic))),
        Control::action("mount-custom", move || marks.set(Some(Marks::Custom))),
        Control::action("unmount", move || marks.set(None)),
        Control::action("set-value", move || values.set(vec![60.0])),
        Control::action("reset-value", move || values.set(vec![30.0])),
        Control::action("widen", move || max.set(200.0)),
        Control::action("narrow", move || max.set(100.0)),
        Control::action("coarsen", move || step.set(10.0)),
        Control::action("refine", move || step.set(1.0)),
    ];
    view! {
        <h1>"use_slider_marks performance"</h1>
        <PerfControls controls />
        {move || { marks.get().map(|marks| view! { <Subject marks values max step /> }) }}
    }
}

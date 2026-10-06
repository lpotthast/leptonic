use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn SliderMarksDemo() -> impl IntoView {
    let rating = RwSignal::new(6_u8);
    let custom = RwSignal::new(6.0_f64);
    let weight = RwSignal::new(4.4_f64);

    view! {
        <div class="demo-control-stack">
            // One mark per step.
            <Slider
                value=rating
                set_value=rating
                min_value=1
                max_value=10
                marks=SliderMarks::Automatic { create_names: false }
                aria_label="Rating"
            />

            // Marks at chosen positions, by value or by fraction of the track.
            <Slider
                value=custom
                set_value=custom
                min_value=1.0
                max_value=10.0
                step=0.5
                marks=SliderMarks::Custom {
                    marks: vec![
                        SliderMark { value: SliderMarkValue::Value(5.5), name: Some("5.5".into()) },
                        SliderMark { value: SliderMarkValue::Value(7.0), name: Some("7".into()) },
                        SliderMark { value: SliderMarkValue::Percentage(0.9), name: Some("90%".into()) },
                    ],
                }
                aria_label="Score"
            />

            // A fractional step.
            <Slider
                value=weight
                set_value=weight
                min_value=2.0
                max_value=8.0
                step=0.4
                marks=SliderMarks::Automatic { create_names: false }
                aria_label="Weight"
            />
        </div>
        <p class="demo-status">
            {move || format!("Rating: {}. Score: {}. Weight: {} kg.", rating.get(), custom.get(), weight.get())}
        </p>
    }
}

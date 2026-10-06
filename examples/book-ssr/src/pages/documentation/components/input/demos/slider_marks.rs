use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn SliderMarksDemo() -> impl IntoView {
    let (rating, set_rating) = signal(6.0);
    let (weight, set_weight) = signal(4.2);
    let (offset, set_offset) = signal(-3.0);

    view! {
        <div class="demo-control-stack">
            // One mark per step.
            <Slider min=1.0 max=10.0 step=1.0
                value=rating set_value=set_rating
                marks=SliderMarks::Automatic { create_names: false }
                value_display=move |v| format!("{v:.0}")/>

            // Marks at chosen positions, by value or by fraction of the track.
            <Slider min=1.0 max=10.0 step=1.0
                value=rating set_value=set_rating
                marks=SliderMarks::Custom {
                    marks: vec![
                        SliderMark { value: SliderMarkValue::Value(5.5), name: Some("5.5".into()) },
                        SliderMark { value: SliderMarkValue::Value(7.0), name: Some("7".into()) },
                        SliderMark { value: SliderMarkValue::Percentage(0.888), name: Some("88%".into()) },
                    ]
                }
                value_display=move |v| format!("{v:.0}")/>

            // A fractional step.
            <Slider min=2.0 max=8.0 step=0.4
                value=weight set_value=set_weight
                marks=SliderMarks::Automatic { create_names: false }
                value_display=move |v| format!("{v:.1}")/>

            // `min` greater than `max` reverses the slider.
            <Slider min=9.0 max=-9.0 step=1.0
                value=offset set_value=set_offset
                marks=SliderMarks::Automatic { create_names: false }
                value_display=move |v| format!("{v:.0}")/>
        </div>
    }
}

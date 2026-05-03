use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn SliderMarksDemo() -> impl IntoView {
    let (value, set_value) = signal(6.0);
    let (value2, set_value2) = signal(4.2);
    let (value3, set_value3) = signal(-3.0);
    let (range_a, set_range_a) = signal(0.5);
    let (range_b, set_range_b) = signal(0.75);
    let (range_a_step, set_range_a_step) = signal(2.0);
    let (range_b_step, set_range_b_step) = signal(4.0);
    let (popover_value, set_popover_value) = signal(50.0);
    let (popover_always_value, set_popover_always_value) = signal(30.0);
    let (popover_range_a, set_popover_range_a) = signal(20.0);
    let (popover_range_b, set_popover_range_b) = signal(80.0);

    view! {
        <Slider min=1.0 max=10.0 step=1.0
            value=value set_value=set_value
            marks=SliderMarks::Automatic { create_names: false }
            value_display=move |v| format!("{v:.0}")/>

        <Slider min=1.0 max=10.0 step=1.0
            value=value set_value=set_value
            marks=SliderMarks::Custom {
                marks: vec![
                    SliderMark {
                        value: SliderMarkValue::Value(5.5),
                        name: Some("5.5".into())
                    },
                    SliderMark {
                        value: SliderMarkValue::Value(7.0),
                        name: Some("7".into())
                    },
                    SliderMark {
                        value: SliderMarkValue::Percentage(0.888),
                        name: Some("88%".into())
                    }
                ]
            }
            value_display=move |v| format!("{v:.0}")/>

        <Slider value=value2 set_value=set_value2 min=2.0 max=8.0 step=0.4
            marks=SliderMarks::Automatic { create_names: false }
            value_display=move |v| format!("{v:.1}")/>

        <Slider value=value3 set_value=set_value3 min=9.0 max=-9.0 step=1.0
            marks=SliderMarks::Automatic { create_names: false }
            value_display=move |v| format!("{v:.0}")/>

        <RangeSlider
            value_a=range_a
            value_b=range_b
            set_value_a=set_range_a
            set_value_b=set_range_b
            min=0.0
            max=1.0
            value_display=move |v| format!("{v:.4}")
        />

        <RangeSlider
            value_a=range_a_step
            value_b=range_b_step
            set_value_a=set_range_a_step
            set_value_b=set_range_b_step
            min=1.0
            max=5.0
            step=1.0
            marks=SliderMarks::Automatic { create_names: true }
            value_display=move |v| format!("{v:.0}")
        />

        <Slider min=0.0 max=100.0 step=1.0
            value=popover_value set_value=set_popover_value
            popover=SliderPopover::When { hovered: true, dragged: true }
            value_display=move |v| format!("{v:.0}")/>

        <Slider min=0.0 max=100.0 step=1.0
            value=popover_always_value set_value=set_popover_always_value
            popover=SliderPopover::Always
            value_display=move |v| format!("{v:.0}")/>

        <RangeSlider
            value_a=popover_range_a
            value_b=popover_range_b
            set_value_a=set_popover_range_a
            set_value_b=set_popover_range_b
            min=0.0
            max=100.0
            step=1.0
            popover=SliderPopover::When { hovered: true, dragged: true }
            value_display=move |v| format!("{v:.0}")
        />
    }
}

use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;
use leptos_icons::BsIcon;

#[component]
pub fn PageSlider(cx: Scope) -> impl IntoView {
    let (value1, set_value1) = create_signal(cx, 6.0);
    let (value2, set_value2) = create_signal(cx, 4.2);
    let (value3, set_value3) = create_signal(cx, -3.0);
    let (value4, set_value4) = create_signal(cx, 0.5);
    let (value5, set_value5) = create_signal(cx, 0.5);
    let (range_a, set_range_a) = create_signal(cx, 0.5);
    let (range_b, set_range_b) = create_signal(cx, 0.75);
    let (range_a_step, set_range_a_step) = create_signal(cx, 2.0);
    let (range_b_step, set_range_b_step) = create_signal(cx, 4.0);

    view! { cx,
        <H1>"Slider"</H1>

        <P>"Allow users to adjust a value within a specified range by sliding a handle."</P>

        <P>
            "All sliders require the "<Code inline=true>"min"</Code>", "<Code inline=true>"max"</Code>" and "<Code inline=true>"step"</Code>" properties, specifying the range of values the slider provides. "
            "Using a small step value results in a smooth or \"continuous\" slider."
        </P>

        <Code>
            {indoc!(r#"
                let (value, set_value) = create_signal(cx, 6.0);
                view! {cx,
                    <Slider min=0.0 max=1.0 step=0.0001
                        value=value set_value=move |v| set_value.set(v)
                        value_display=create_callback(cx, move |v| format!("{v:.4}")) />
                }
            "#)}
        </Code>

        <P>
            "The slider always operates with "<Code inline=true>"f64"</Code>" values and may suffer from typical IEEE-math rounding problems. "
            "We use the "<Code inline=true>"value_display"</Code>" property to specify how a selected value should be rendered."
        </P>

        <Slider min=0.0 max=1.0 step=0.0001
            value=value4 set_value=move |v| set_value4.set(v)
            value_display=create_callback(cx, move |v| format!("{v:.4}")) />

        <H2>"Volume slider"</H2>

        <P>"Continuous sliders are perfect when the exact value selected is of no interest to your user. For example, when operating a volume slider."</P>

        <Code>
            {indoc!(r#"
                let (value, set_value) = create_signal(cx, 0.5);
                view! {cx,
                    <Stack orientation=StackOrientation::Horizontal spacing=Size::Zero>
                        <Icon icon=BsIcon::BsVolumeDownFill style="font-size: 2.5em;"/>
                        <Slider min=0.0 max=1.0 step=0.0001
                            value=value set_value=move |v| set_value.set(v)
                            value_display=create_callback(cx, move |v| format!("{:.0}%", v * 100.0))
                            style="width: 10em"/>
                        <Icon icon=BsIcon::BsVolumeUpFill style="font-size: 2.5em; margin-left: 0.25em;"/>
                    </Stack>
                }
            "#)}
        </Code>

        <Stack orientation=StackOrientation::Horizontal spacing=Size::Zero>
            <Icon icon=BsIcon::BsVolumeDownFill style="font-size: 2.5em;"/>
            <Slider min=0.0 max=1.0 step=0.0001 value=value5 set_value=move |v| set_value5.set(v) style="width: 10em"
                value_display=create_callback(cx, move |v| format!("{:.0}%", v * 100.0))/>
            <Icon icon=BsIcon::BsVolumeUpFill style="font-size: 2.5em; margin-left: 0.25em;"/>
        </Stack>

        <H2>"Marks"</H2>

        <P>
            "Small step values result in lesser selectable values, as only values starting from min and increased by multiples of step are selectable. "
            "To help visualize the selectable values of the slider, marks can be automatically generated."
        </P>

        <Code>
            {indoc!(r#"
                let (value, set_value) = create_signal(cx, 6.0);
                view! {cx,
                    <Slider min=1.0 max=10.0 step=1.0
                        value=value set_value=move |v| set_value.set(v)
                        marks=SliderMarks::Automatic { create_names: false }
                        value_display=create_callback(cx, move |v| format!("{v:.0}"))/>
                }
            "#)}
        </Code>

        <Slider min=1.0 max=10.0 step=1.0
            value=value1 set_value=move |v| set_value1.set(v)
            marks=SliderMarks::Automatic { create_names: false }
            value_display=create_callback(cx, move |v| format!("{v:.0}"))/>

        <P>
            "Note that marks are only helpful when dealing with sliders having a limited number of selectable values, meaning ones with small ranges and a big stepping values. "
            "Automatic mark generation is currently limited to creating 20 evenly spaced marks. Continuous sliders will not create thousands of them."
        </P>

        <H2>"Arbitrary ranges"</H2>

        <P>"Sliders can use any combination of min, max and step values."</P>

        <Slider value=value2 set_value=move |v| set_value2.set(v) min=2.0 max=8.0 step=0.4
            marks=SliderMarks::Automatic { create_names: false }
            value_display=create_callback(cx, move |v| format!("{v:.1}"))/>

        <P>"You can also use a positive value for the "<Code inline=true>"min"</Code>" prop, and a negative value for the "<Code inline=true>"max"</Code>" prop, resulting in an reversed axis."</P>

        <Slider value=value3 set_value=move |v| set_value3.set(v) min=9.0 max=-9.0 step=1.0
            marks=SliderMarks::Automatic { create_names: false }
            value_display=create_callback(cx, move |v| format!("{v:.0}")) />

        <H2>"Range sliders"</H2>

        <P>
            "A range of values can be selected using the "<Code inline=true>"RangeSlider"</Code>" component. "
            "The component requires two values and in return provides a slider with two control knobs, allowing you to select a range of values. "
            "One knob can be dragged over the other, letting them switch places."
        </P>

        <Code>
            {indoc!(r#"
                let (value_a, set_value_a) = create_signal(cx, 0.5);
                let (value_b, set_value_b) = create_signal(cx, 0.75);
                view! {cx,
                    <RangeSlider
                        value_a=range_a
                        value_b=range_b
                        set_value_a=move |v| set_range_a.set(v)
                        set_value_b=move |v| set_range_b.set(v)
                        min=0.0
                        max=1.0
                        step=0.0001
                        popover=SliderPopover::Always
                        value_display=create_callback(cx, move |v| format!("{v:.4}"))
                    />
                }
            "#)}
        </Code>

        <RangeSlider
            value_a=range_a
            value_b=range_b
            set_value_a=move |v| set_range_a.set(v)
            set_value_b=move |v| set_range_b.set(v)
            min=0.0
            max=1.0
            step=0.0001
            popover=SliderPopover::Always
            value_display=create_callback(cx, move |v| format!("{v:.4}"))
        />

        <P>"Range sliders can also use marks, just like the normal slider."</P>

        <RangeSlider
            value_a=range_a_step
            value_b=range_b_step
            set_value_a=move |v| set_range_a_step.set(v)
            set_value_b=move |v| set_range_b_step.set(v)
            min=1.0
            max=5.0
            step=1.0
            marks=SliderMarks::Automatic { create_names: true }
            value_display=create_callback(cx, move |v| format!("{v:.0}"))
        />

        <H2>"Keyboard input"</H2>

        <P>
            "Slider knobs are keyboard-interactable and can be cycled through using the "<Code inline=true>"Tab"</Code>" key. "
            "Manipulation of slider knobs using the error keys will come in a future update."
        </P>

        <H2>"Styling"</H2>

        <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

        <Code>
            {indoc!(r#"
                --slider-margin
                --slider-bar-height
                --slider-bar-background-color
                --slider-range-height
                --slider-knob-size
                --slider-knob-halo-size
                --slider-knob-halo-size-while-dragged
                --slider-knob-halo-opacity
                --slider-knob-transition-speed
                --slider-knob-box-shadow
                --slider-mark-size
                --slider-mark-color
                --slider-mark-color-in-range
                --slider-mark-title-color
                --slider-mark-title-color-in-range
            "#)}
        </Code>
    }
}

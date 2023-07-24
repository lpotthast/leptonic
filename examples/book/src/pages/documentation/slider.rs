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

    view! { cx,
        <H1>"Slider"</H1>

        <Code>
            {indoc!(r#"
                let (value, set_value) = create_signal(cx, 6.0);
                view! {cx,
                    <Slider min=1.0 max=10.0 step=1.0 marks=SliderMarks::Automatic { create_names: false } value=value set_value=move |v| set_value.set(v) />
                }
            "#)}
        </Code>

        <P>"Value: "{move || value1.get()}</P>

        <Slider min=1.0 max=10.0 step=1.0 marks=SliderMarks::Automatic { create_names: false } value=value1 set_value=move |v| set_value1.set(v) />

        <H2>"With icons"</H2>

        <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(0.5)>
            <Icon icon=BsIcon::BsVolumeDownFill style="font-size: 2.5em;"/>
            <Slider min=0.0 max=1.0 step=0.0001 value=value5 set_value=move |v| set_value5.set(v) style="width: 10em"/>
            <Icon icon=BsIcon::BsVolumeUpFill style="font-size: 2.5em; margin-left: 0.25em;"/>
        </Stack>

        <P>"Value: "{move || format!("{:.1}", value2.get())}</P>

        <Slider value=value2 set_value=move |v| set_value2.set(v) min=2.0 max=8.0 step=0.4 />

        <P>"You can also use a positive value for the "<Code inline=true>"min"</Code>" prop, and a negative value for the "<Code inline=true>"max"</Code>" prop."</P>

        <P>"Value: "{move || value3.get()}</P>

        <Slider value=value3 set_value=move |v| set_value3.set(v) min=9.0 max=-9.0 step=1.0 />

        <P>"Use a small stepping value to achieve a smooth slider."</P>

        <P>"Value: "{move || format!("{:.4}", value4.get())}</P>

        <Slider value=value4 set_value=move |v| set_value4.set(v) min=0.0 max=1.0 step=0.0001 />

        <H2>"Range sliders"</H2>

        <P>"Value A: "{move || format!("{:.4}", range_a.get())}</P>
        <P>"Value B: "{move || format!("{:.4}", range_b.get())}</P>

        <RangeSlider
            value_a=range_a
            value_b=range_b
            set_value_a=move |v| set_range_a.set(v)
            set_value_b=move |v| set_range_b.set(v)
            min=0.0
            max=1.0
            step=0.0001
        />
    }
}

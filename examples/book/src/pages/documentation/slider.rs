use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageSlider(cx: Scope) -> impl IntoView {
    let (value1, set_value1) = create_signal(cx, 6.0);
    let (value2, set_value2) = create_signal(cx, 4.2);
    let (value3, set_value3) = create_signal(cx, -3.0);
    let (value4, set_value4) = create_signal(cx, 0.5);

    view! { cx,
        <H1>"Slider"</H1>

        <P>"Value: "{move || value1.get()}</P>

        <Slider value=value1 set_value=move |v| set_value1.set(v) min=1.0 max=10.0 step=1.0 />

        <P>"Value: "{move || format!("{:.1}", value2.get())}</P>

        <Slider value=value2 set_value=move |v| set_value2.set(v) min=2.0 max=8.0 step=0.4 />

        <P>"You can also use a positive value for the "<Code inline=true>"min"</Code>" prop, and a negative value for the "<Code inline=true>"max"</Code>" prop."</P>

        <P>"Value: "{move || value3.get()}</P>

        <Slider value=value3 set_value=move |v| set_value3.set(v) min=9.0 max=-9.0 step=1.0 />

        <P>"Use a small stepping value to achieve a smooth slider."</P>

        <P>"Value: "{move || format!("{:.4}", value4.get())}</P>

        <Slider value=value4 set_value=move |v| set_value4.set(v) min=0.0 max=1.0 step=0.0001 />
    }
}

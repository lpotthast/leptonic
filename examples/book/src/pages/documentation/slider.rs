use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageSlider(cx: Scope) -> impl IntoView {
    let (value1, set_value1) = create_signal(cx, 9.2);
    let (value2, set_value2) = create_signal(cx, 6.0);

    view! { cx,
        <H1>"Slider"</H1>

        <P>"Value2: "{move || value1.get()}</P>

        <Slider value=value1 set_value=move |v| set_value1.set(v) min=2.0 max=8.0 step=0.4 />

        <P>"Value1: "{move || value2.get()}</P>

        <Slider value=value2 set_value=move |v| set_value2.set(v) min=1.0 max=10.0 step=1.0 />
    }
}

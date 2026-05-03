use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn SliderBasicDemo() -> impl IntoView {
    let (value, set_value) = signal(0.5);

    view! {
        <Slider min=0.0 max=100.0 step=1.0
            value=value set_value=set_value
            value_display=move |v| format!("{v:.4}") />

        <Slider min=0.0 max=1.0 step=0.0001
            value=value set_value=set_value
            value_display=move |v| format!("{v:.4}") />
    }
}

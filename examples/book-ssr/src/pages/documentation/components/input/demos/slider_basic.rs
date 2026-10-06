use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn SliderBasicDemo() -> impl IntoView {
    let (percent, set_percent) = signal(50.0);
    let (fraction, set_fraction) = signal(0.5);

    view! {
        <Slider min=0.0 max=100.0 step=1.0 value=percent set_value=set_percent/>
        <p class="demo-status">{move || format!("Step 1: {:.0}", percent.get())}</p>

        <Slider min=0.0 max=1.0 step=0.0001 value=fraction set_value=set_fraction/>
        <p class="demo-status">{move || format!("Step 0.0001: {:.4}", fraction.get())}</p>
    }
}

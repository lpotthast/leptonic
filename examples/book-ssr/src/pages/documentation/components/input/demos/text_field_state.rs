use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn TextFieldStateDemo() -> impl IntoView {
    // The field shows the signal and writes every change to it.
    let city = RwSignal::new("Berlin".to_owned());

    view! {
        <div class="demo-form">
            <TextField label="City" state=city/>
        </div>
        <div class="demo-control-row demo-mt-1">
            <Button on_press=move |_| city.update(|city| *city = city.to_uppercase())>"Uppercase"</Button>
            <Button on_press=move |_| city.set(String::new()) color=ButtonColor::Secondary>"Clear"</Button>
        </div>
        <p class="demo-status">{move || format!("City: {}", city.get())}</p>
    }
}

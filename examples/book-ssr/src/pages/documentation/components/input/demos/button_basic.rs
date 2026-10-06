use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn ButtonBasicDemo() -> impl IntoView {
    let presses = RwSignal::new(0u32);

    view! {
        <Button on_press=move |_| presses.update(|n| *n += 1)>"Press me"</Button>
        <p class="demo-status">
            {move || match presses.get() {
                1 => "Pressed 1 time.".to_owned(),
                n => format!("Pressed {n} times."),
            }}
        </p>
    }
}

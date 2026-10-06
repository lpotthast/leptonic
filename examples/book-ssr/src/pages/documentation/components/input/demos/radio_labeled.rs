use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn RadioLabeledDemo() -> impl IntoView {
    view! {
        <RadioGroup label="Shipping" description="Express arrives the next day.">
            <Radio value="standard">"Standard shipping"</Radio>
            <Radio value="express">"Express shipping"</Radio>
        </RadioGroup>
    }
}

use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn RadioGroupDemo() -> impl IntoView {
    let (checked2, set_checked2) = signal(false);
    let (checked3, set_checked3) = signal(false);

    view! {
        <RadioGroup>
            <Radio checked=checked2 set_checked=set_checked2 />
            <Radio checked=checked3 set_checked=set_checked3 />
        </RadioGroup>
    }
}

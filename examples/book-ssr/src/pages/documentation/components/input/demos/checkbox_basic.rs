use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn CheckboxBasicDemo() -> impl IntoView {
    let (checked, set_checked) = signal(false);

    view! {
        <Checkbox checked=checked set_checked=set_checked />
        <span>"checked: " {move || checked.get()}</span>
    }
}

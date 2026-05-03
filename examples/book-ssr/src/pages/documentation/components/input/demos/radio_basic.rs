use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn RadioBasicDemo() -> impl IntoView {
    let (checked, set_checked) = signal(false);

    view! {
        <Radio checked=checked set_checked=set_checked />
        <span>"checked: " {move || checked.get()}</span>
    }
}

use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn TextFieldConceptDemo() -> impl IntoView {
    let (value, set_value) = signal(String::new());

    view! {
        <TextInput get=value set=set_value />
    }
}

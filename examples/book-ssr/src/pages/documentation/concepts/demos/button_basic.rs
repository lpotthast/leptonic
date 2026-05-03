use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn ButtonBasicConceptDemo() -> impl IntoView {
    view! {
        <Button on_press=move |_| {}>"My Button"</Button>
    }
}

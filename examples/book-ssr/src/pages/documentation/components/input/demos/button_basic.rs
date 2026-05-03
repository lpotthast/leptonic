use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn ButtonBasicDemo() -> impl IntoView {
    view! {
        <div>
            <Button on_press=move |_| {}>"My Button"</Button>
        </div>
    }
}

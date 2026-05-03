use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn ButtonGroupDemo() -> impl IntoView {
    view! {
        <ButtonGroup>
            <Button on_press=move |_| {}>"Button 1"</Button>
            <Button on_press=move |_| {}>"Button 2"</Button>
            <Button on_press=move |_| {}>"Button 3"</Button>
        </ButtonGroup>
    }
}

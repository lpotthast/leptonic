use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn ButtonVariantsDemo() -> impl IntoView {
    view! {
        <ButtonWrapper>
            <Button variant=ButtonVariant::Flat>"Flat"</Button>
            <Button variant=ButtonVariant::Outlined>"Outlined"</Button>
            <Button>"Filled"</Button>
        </ButtonWrapper>
    }
}

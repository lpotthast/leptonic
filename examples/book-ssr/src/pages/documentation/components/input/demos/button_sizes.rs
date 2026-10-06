use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn ButtonSizesDemo() -> impl IntoView {
    view! {
        <ButtonWrapper>
            <Button size=ButtonSize::Small>"Small"</Button>
            <Button>"Normal"</Button>
            <Button size=ButtonSize::Big>"Big"</Button>
        </ButtonWrapper>
    }
}

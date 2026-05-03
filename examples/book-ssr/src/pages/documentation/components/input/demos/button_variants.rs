use leptonic::{components::prelude::*, utils::css::em};
use leptos::prelude::*;

#[component]
pub fn ButtonVariantsDemo() -> impl IntoView {
    view! {
        <Stack orientation=StackOrientation::Horizontal spacing=em(0.6) attr:style="justify-content: flex-start;">
            <Button on_press=move |_| {} variant=ButtonVariant::Flat>"Flat"</Button>
            <Button on_press=move |_| {} variant=ButtonVariant::Outlined>"Outlined"</Button>
            <Button on_press=move |_| {} variant=ButtonVariant::Filled>"Filled"</Button>
        </Stack>
    }
}

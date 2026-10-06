use leptonic::{components::prelude::*, utils::css::em};
use leptos::prelude::*;

#[component]
pub fn StackDemo() -> impl IntoView {
    view! {
        <Stack spacing=em(0.5)>
            <div class="demo-stack-item">"First"</div>
            <div class="demo-stack-item">"Second"</div>
            <div class="demo-stack-item">"Third"</div>
        </Stack>

        <Stack orientation=StackOrientation::Horizontal spacing=em(0.5) classes="demo-mt-1">
            <div class="demo-stack-item">"First"</div>
            <div class="demo-stack-item">"Second"</div>
            <div class="demo-stack-item">"Third"</div>
        </Stack>
    }
}

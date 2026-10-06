use leptonic::{components::prelude::*, prelude::*, utils::css::em};
use leptos::prelude::*;

#[component]
pub fn AppBarDemo() -> impl IntoView {
    view! {
        <div class="demo-clf-frame demo-clf-frame-scroll">
            <AppBar height=em(3.0) classes="demo-clf-app-bar">
                <span class="demo-clf-app-bar-title">"Leptonic"</span>
                <Stack orientation=StackOrientation::Horizontal spacing=em(1.0) classes="demo-clf-app-bar-actions">
                    <Icon icon=icondata::BsBell/>
                    <Icon icon=icondata::BsPower/>
                </Stack>
            </AppBar>

            <div class="demo-clf-frame-content">
                <p>"Scroll \u{2193}"</p>
                <Stack spacing=em(0.5)>
                    {(0..10).map(|_| view! { <Skeleton height=em(3.0)/> }).collect_view()}
                </Stack>
            </div>
        </div>
    }
}

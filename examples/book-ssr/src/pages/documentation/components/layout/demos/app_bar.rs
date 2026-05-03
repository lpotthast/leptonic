use leptonic::{components::prelude::*, prelude::*, utils::css::em};
use leptos::prelude::*;

#[component]
pub fn AppBarDemo() -> impl IntoView {
    let app_bar_height = em(3.0);

    view! {
        <div style="position: relative; border: 4px solid gray; width: 100%; height: 20em; overflow: auto;">
            <AppBar height=app_bar_height attr:style="z-index: 1; background: var(--brand-color); color: white;">
                <h3 style="margin-left: 1em; color: white;">"Leptonic"</h3>
                <Stack orientation=StackOrientation::Horizontal spacing=em(1.0) attr:style="margin-right: 1em">
                    <Icon icon=icondata::BsBell></Icon>
                    <Icon icon=icondata::BsPower></Icon>
                </Stack>
            </AppBar>

            <div style="padding: 0.5em;">
                <p>"Scroll ↓"</p>
                <Stack spacing=em(0.5)>
                    {(0..10).map(|_| view! { <Skeleton height=em(3.0)/> }).collect_view()}
                </Stack>
            </div>
        </div>
    }
}

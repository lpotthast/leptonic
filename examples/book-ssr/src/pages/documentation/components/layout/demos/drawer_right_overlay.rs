use leptonic::{components::prelude::*, utils::css::em};
use leptos::prelude::*;

#[component]
pub fn DrawerRightOverlayDemo() -> impl IntoView {
    let (shown, set_shown) = signal(true);

    view! {
        <div class="demo-inline-controls demo-mb-1">
            <Switch state=(shown, set_shown)>"Show drawer"</Switch>
        </div>

        <div class="demo-clf-frame demo-clf-frame-row">
            <div class="demo-clf-frame-content">
                <p>"Scroll \u{2193}"</p>
                <Stack spacing=em(0.5)>
                    {(0..8).map(|_| view! { <Skeleton height=em(3.0)/> }).collect_view()}
                </Stack>
            </div>
            <Drawer side=DrawerSide::Right shown=shown classes=["demo-clf-drawer", "demo-clf-drawer-overlay"]>
                <Stack spacing=em(0.5)>
                    {(0..8).map(|_| view! { <Skeleton height=em(3.0)/> }).collect_view()}
                </Stack>
            </Drawer>
        </div>
    }
}

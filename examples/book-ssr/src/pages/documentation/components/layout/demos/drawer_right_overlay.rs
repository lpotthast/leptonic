use leptonic::{components::prelude::*, utils::css::em};
use leptos::prelude::*;

#[component]
pub fn DrawerRightOverlayDemo() -> impl IntoView {
    let (shown, set_shown) = signal(true);

    view! {
        <div class="demo-drawer-frame">
            <div class="demo-drawer-content">
                <p>"Scroll " <span aria-hidden="true">"\u{2193}"</span></p>
                <Stack spacing=em(0.5)>
                    {(0..8).map(|_| view! { <Skeleton height=em(3.0)/> }).collect_view()}
                </Stack>
            </div>
            <Drawer side=DrawerSide::Right shown=shown classes=["demo-drawer", "demo-drawer-overlay"]>
                <Stack spacing=em(0.5)>
                    {(0..8).map(|_| view! { <Skeleton height=em(3.0)/> }).collect_view()}
                </Stack>
            </Drawer>
        </div>

        <div class="demo-controls">
            <Switch is_selected=shown set_selected=set_shown>"Show drawer"</Switch>
        </div>
    }
}

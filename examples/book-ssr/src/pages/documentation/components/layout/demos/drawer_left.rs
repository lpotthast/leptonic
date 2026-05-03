use leptonic::{components::prelude::*, utils::css::em};
use leptos::prelude::*;

#[component]
pub fn DrawerLeftDemo() -> impl IntoView {
    let (shown, set_shown) = signal(true);

    view! {
        <Toggle state=shown set_state=set_shown/>

        <div style="display: flex; flex-direction: row; justify-content: flex-start; align-items: flex-start; border: 4px solid gray; width: 100%; height: 20em; overflow: hidden;">
            <Drawer side=DrawerSide::Left shown=shown attr:style="overflow-y: scroll; padding: 0.5em; background-color: var(--brand-color); border-right: 1px solid gray; z-index: 1;">
                <Stack spacing=em(0.5)>
                    {(0..8).map(|_| view! { <Skeleton height=em(3.0)/> }).collect_view()}
                </Stack>
            </Drawer>
            <div style="padding: 0.5em; display: flex; flex-direction: column; overflow-y: scroll; width: 100%; height: 100%;">
                <p>"Scroll ↓"</p>
                <Stack spacing=em(0.5)>
                    {(0..8).map(|_| view! { <Skeleton height=em(3.0)/> }).collect_view()}
                </Stack>
            </div>
        </div>
    }
}

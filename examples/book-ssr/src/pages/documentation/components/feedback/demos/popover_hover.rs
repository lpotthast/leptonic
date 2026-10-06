use leptonic::{
    atoms::prelude::Hoverable, components::prelude::*, hooks::PlacementY, utils::css::em,
};
use leptos::prelude::*;

#[component]
pub fn PopoverHoverDemo() -> impl IntoView {
    let (show, set_show) = signal(false);

    view! {
        <div class="demo-popover-stage">
            <Popover state=(show, set_show) placement_y=PlacementY::Above>
                <PopoverTrigger slot>
                    <Hoverable
                        on_hover_start=move |_| set_show.set(true)
                        on_hover_end=move |_| set_show.set(false)
                    >
                        <Skeleton animated=false width=em(10.0)>
                            "Hover me!"
                        </Skeleton>
                    </Hoverable>
                </PopoverTrigger>
                "Shown while you hover the trigger."
            </Popover>
        </div>
    }
}

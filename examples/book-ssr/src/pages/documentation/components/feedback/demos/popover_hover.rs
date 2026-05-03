use leptonic::hooks::PlacementY;
use leptonic::{atoms::prelude::Hoverable, components::prelude::*, utils::css::em};
use leptos::prelude::*;

#[component]
pub fn PopoverHoverDemo() -> impl IntoView {
    let (show, set_show) = signal(false);

    view! {
        <div style="margin-top: 3em; margin-bottom: 1em;">
            <Popover
                show_when=show
                on_close=move |()| set_show.set(false)
                placement_y=PlacementY::Above
            >
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
                "1"
            </Popover>
        </div>
    }
}

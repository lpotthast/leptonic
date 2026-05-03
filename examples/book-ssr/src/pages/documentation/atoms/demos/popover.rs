use leptonic::{
    atoms::{
        button::Button,
        hoverable::Hoverable,
        popover::{Popover, PopoverContent, PopoverContext, PopoverTrigger},
    },
    hooks::{PlacementX, PlacementY},
    utils::locale::WritingDirection,
};
use leptos::prelude::*;

#[component]
pub fn PopoverDemo() -> impl IntoView {
    view! {
        <Popover>
            <PopoverTrigger>
                {
                    let ctx = expect_context::<PopoverContext>();
                    view! {
                        <Button on_press=move |_| ctx.set_state.set(!ctx.state.get_untracked())>
                            "Press me"
                        </Button>

                        <Hoverable on_hover_start=move |_| ctx.set_state.set(true) on_hover_end=move |_| ctx.set_state.set(false)>
                            <div>
                                "Hover me"
                            </div>
                        </Hoverable>
                    }
                }
            </PopoverTrigger>

            <PopoverContent placement_x=PlacementX::Center placement_y=PlacementY::Above writing_direction=WritingDirection::Ltr>
                "Overlay"
            </PopoverContent>
        </Popover>
    }
}

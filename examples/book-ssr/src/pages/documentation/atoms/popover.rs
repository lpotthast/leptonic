use indoc::indoc;
use leptonic::atoms::popover::PopoverContext;
use leptonic::atoms::prelude as atoms;
use leptonic::components::prelude::*;
use leptonic::hooks::overlay::{PlacementX, PlacementY};
use leptonic::utils::locale::WritingDirection;
use leptos::*;

use crate::pages::documentation::article::Article;

#[component]
pub fn PageAtomPopover() -> impl IntoView {
    view! {
        <Article>
            <H1>"Popover"</H1>

            <P>"..."</P>

            <Code>
                {indoc!(r#"
                    use leptonic::atoms::prelude as atoms;

                    view! {
                        <atoms::Popover disabled=false>
                            <atoms::PopoverTrigger>
                                { 
                                    let ctx = expect_context::<PopoverContext>();
                                    view! {
                                        <atoms::Button on_hover_start=move |_| ctx.set_state.set(true) on_hover_end=move |_| ctx.set_state.set(false)>
                                            "Press me"
                                        </atoms::Button>
                                    }
                                }
                            </atoms::PopoverTrigger>
                
                            <atoms::PopoverContent placement_x=PlacementX::Center placement_y=PlacementY::Above writing_direction=WritingDirection::Ltr>
                                "Overlay"
                            </atoms::PopoverContent>
                        </atoms::Popover>
                    }
                "#)}
            </Code>

            <atoms::Popover disabled=false>
                <atoms::PopoverTrigger>
                    {
                        let ctx = expect_context::<PopoverContext>();
                        view! {
                            <atoms::Button on_press=move |_| ctx.set_state.set(!ctx.state.get_untracked())>
                                "Press me"
                            </atoms::Button>

                            <atoms::Hoverable on_hover_start=move |_| ctx.set_state.set(true) on_hover_end=move |_| ctx.set_state.set(false)>
                                <div>
                                    "Hover me"
                                </div>
                            </atoms::Hoverable>
                        }
                    }
                </atoms::PopoverTrigger>

                <atoms::PopoverContent placement_x=PlacementX::Center placement_y=PlacementY::Above writing_direction=WritingDirection::Ltr>
                    "Overlay"
                </atoms::PopoverContent>
            </atoms::Popover>
        </Article>
    }
}

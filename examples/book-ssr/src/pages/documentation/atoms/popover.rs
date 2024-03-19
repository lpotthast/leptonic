use indoc::indoc;
use leptonic::atoms::button::Button;
use leptonic::atoms::hoverable::Hoverable;
use leptonic::atoms::link::AnchorLink;
use leptonic::atoms::popover::{Popover, PopoverContent, PopoverContext, PopoverTrigger};
use leptonic::components::typography::{Code, H1, P};
use leptonic::hooks::{PlacementX, PlacementY};
use leptonic::utils::locale::WritingDirection;
use leptos::*;

use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;

#[component]
pub fn PageAtomPopover() -> impl IntoView {
    view! {
        <Article>
            <H1 id="popover" class="anchor">
                "Popover"
                <AnchorLink href="#popover" description="Direct link to article header"/>
            </H1>

            <P>"..."</P>

            <Code>
                {indoc!(r#"
                    use leptonic::atoms::button::Button;
                    use leptonic::atoms::hoverable::Hoverable;
                    use leptonic::atoms::link::AnchorLink;
                    use leptonic::atoms::popover::{Popover, PopoverContent, PopoverContext, PopoverTrigger};

                    view! {
                        <Popover disabled=false>
                            <PopoverTrigger>
                                {
                                    let ctx = expect_context::<PopoverContext>();
                                    view! {
                                        <Button on_hover_start=move |_| ctx.set_state.set(true) on_hover_end=move |_| ctx.set_state.set(false)>
                                            "Press me"
                                        </Button>
                                    }
                                }
                            </PopoverTrigger>

                            <PopoverContent placement_x=PlacementX::Center placement_y=PlacementY::Above writing_direction=WritingDirection::Ltr>
                                "Overlay"
                            </PopoverContent>
                        </Popover>
                    }
                "#)}
            </Code>

            <Popover disabled=false>
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
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Popover", link: "#popover" },
            ]
        }/>
    }
}

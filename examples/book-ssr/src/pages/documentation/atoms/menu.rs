use indoc::{formatdoc, indoc};
use leptonic::atoms::button::Button;
use leptonic::atoms::hoverable::Hoverable;
use leptonic::atoms::link::AnchorLink;
use leptonic::atoms::menu::{Menu, MenuItem, MenuTrigger};
use leptonic::atoms::popover::{Popover, PopoverContent, PopoverContext, PopoverTrigger};
use leptonic::components::typography::{Code, H1, P};
use leptonic::hooks::{PlacementX, PlacementY};
use leptonic::utils::aria::AriaAccessibleName;
use leptonic::utils::locale::WritingDirection;
use leptos_meta::Style;
use leptos::*;

use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;

#[component]
pub fn PageAtomMenu() -> impl IntoView {
    view! {
        <Article>
            <H1 id="menu" class="anchor">
                "Menu"
                <AnchorLink href="#menu" description="Direct link to article header"/>
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

            <Style>
                {indoc! {r##"
                    leptonic-menu {
                        display: flex;
                        flex-direction: column;
                        border: 1px solid grey;
                        background-color: darkgrey;
                    }
                    leptonic-menu-item {
                        display: flex;
                    }
                "##}}
            </Style>

            <Menu accessible_name=AriaAccessibleName::label("menu") on_action=move |a| { tracing::info!("Clicked: {a:?}") }>
                <MenuItem key="open">"Open"</MenuItem>
                <MenuItem key="rename">"Rename…"</MenuItem>
                <MenuItem key="duplicate">"Duplicate"</MenuItem>
                <MenuItem key="share">"Share…"</MenuItem>
                <MenuItem key="delete">"Delete…"</MenuItem>
            </Menu>

            /*
            <MenuTrigger disabled=false>
                <Button aria_label=Oco::Borrowed("Menu")>"☰"</Button>
                <Popover disabled=false>
                    <Menu on_action=move |a| {/*alert*/}>
                        <MenuItem id="open">"Open"</MenuItem>
                        <MenuItem id="rename">"Rename…"</MenuItem>
                        <MenuItem id="duplicate">"Duplicate"</MenuItem>
                        <MenuItem id="share">"Share…"</MenuItem>
                        <MenuItem id="delete">"Delete…"</MenuItem>
                    </Menu>
                </Popover>
            </MenuTrigger>


            <MenuContext>
                <MenuTrigger disabled=false>
                    <Button aria_label=Oco::Borrowed("Menu")>"☰"</Button>
                </MenuTrigger>
                <MenuContent>
                    <Popover disabled=false>
                        <Menu on_action=move |a| {/*alert*/}>
                            <MenuItem id="open">"Open"</MenuItem>
                            <MenuItem id="rename">"Rename…"</MenuItem>
                            <MenuItem id="duplicate">"Duplicate"</MenuItem>
                            <MenuItem id="share">"Share…"</MenuItem>
                            <MenuItem id="delete">"Delete…"</MenuItem>
                        </Menu>
                    </Popover>
                </MenuContent>
            </MenuContext>
            */
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Menu", link: "#menu" },
            ]
        }/>
    }
}

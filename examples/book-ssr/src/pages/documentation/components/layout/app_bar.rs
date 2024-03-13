use indoc::indoc;
use leptonic::{atoms::link::AnchorLink, components::prelude::*, prelude::*};
use leptos::*;

use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageAppBar() -> impl IntoView {
    let app_bar_height = Height::Em(3.0);

    view! {
        <Article>
            <H1 id="overview">
                "App Bar"
                <AnchorLink href="#overview" description="Direct link to main article header"/>
            </H1>

            <P>"The "<Code inline=true>"<AppBar>"</Code>" component sticks to the top of its parent and provides a convenient entrypoint for many app layouts."</P>

            <Box style="position: relative; border: 4px solid gray; width: 100%; height: 20em; overflow: auto;">
                <AppBar height=app_bar_height style="z-index: 1; background: var(--brand-color); color: white;">
                    <H3 style="margin-left: 1em; color: white;">"Leptonic"</H3>
                    <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(1.0) style="margin-right: 1em">
                        <Icon icon=icondata::BsBell></Icon>
                        <Icon icon=icondata::BsPower></Icon>
                    </Stack>
                </AppBar>

                <Box style="padding: 0.5em;">
                    <P>"Scroll ↓"</P>
                    <Stack spacing=Size::Em(0.5)>
                        {(0..10).map(|_| view! { <Skeleton height=Size::Em(3.0)/> }).collect_view()}
                    </Stack>
                </Box>
            </Box>

            <Code>
                {indoc!(r#"
                    <Box style="position: relative; border: 4px solid gray; width: 100%; height: 20em; overflow: auto;">
                        <AppBar height=app_bar_height style="z-index: 1; background: var(--brand-color); color: white;">
                            <H3 style="margin-left: 1em; color: white;">"Leptonic"</H3>
                            <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(1.0) style="margin-right: 1em">
                                <Icon icon=icondata::BsGithub></Icon>
                                <Icon icon=icondata::BsPower></Icon>
                            </Stack>
                        </AppBar>

                        <Box style="padding: 0.5em;">
                            <P>"Scroll ↓"</P>
                            <Stack spacing=Size::Em(0.5)>
                                {(0..10).map(|_| view! { <Skeleton height=Size::Em(3.0)/> }).collect_view()}
                            </Stack>
                        </Box>
                    </Box>
                "#)}
            </Code>

            <H2 id="styling">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </H2>

            <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

            <Code>
                {indoc!(r"
                    --app-bar-height
                    --app-bar-background-color
                    --app-bar-border-bottom
                    --app-bar-box-shadow
                ")}
            </Code>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "AppBar", link: "#overview" },
                Toc::Leaf { title: "Styling", link: "#styling" },
            ]
        }/>
    }
}

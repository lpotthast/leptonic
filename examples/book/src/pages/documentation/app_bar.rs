use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;
use leptos_icons::BsIcon;

#[component]
pub fn PageAppBar(cx: Scope) -> impl IntoView {
    let app_bar_height = Height::Em(3.0);

    view! { cx,
        <H1>"App Bar"</H1>

        <P>"The "<Code inline=true>"<AppBar>"</Code>" component sticks to the top of its parent and provides a convenient entrypoint for many app layouts."</P>

        <Box style="position: relative; border: 4px solid gray; width: 100%; height: 20em; overflow: auto;">
            <AppBar height=app_bar_height style="z-index: 1; background: var(--brand-color); color: white;">
                <H3 style="margin-left: 1em; color: white;">"Leptonic"</H3>
                <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(1.0) style="margin-right: 1em">
                    <Icon icon=BsIcon::BsBell></Icon>
                    <Icon icon=BsIcon::BsPower></Icon>
                </Stack>
            </AppBar>

            <Box style="padding: 0.5em;">
                <P>"Scroll ↓"</P>
                <Stack spacing=Size::Em(0.5)>
                    {(0..10).map(|_| view! { cx, <Skeleton height=Size::Em(3.0)/> }).collect_view(cx)}
                </Stack>
            </Box>
        </Box>

        <Code>
            {indoc!(r#"
                <Box style="position: relative; border: 4px solid gray; width: 100%; height: 20em; overflow: auto;">
                    <AppBar height=app_bar_height style="z-index: 1; background: var(--brand-color); color: white;">
                        <H3 style="margin-left: 1em; color: white;">"Leptonic"</H3>
                        <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(1.0) style="margin-right: 1em">
                            <Icon icon=BsIcon::BsGithub></Icon>
                            <Icon icon=BsIcon::BsPower></Icon>
                        </Stack>
                    </AppBar>

                    <Box style="padding: 0.5em;">
                        <P>"Scroll ↓"</P>
                        <Stack spacing=Size::Em(0.5)>
                            {(0..10).map(|_| view! { cx, <Skeleton height=Size::Em(3.0)/> }).collect_view(cx)}
                        </Stack>
                    </Box>
                </Box>
            "#)}
        </Code>

        <H2>"Styling"</H2>

        <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

        <Code>
            {indoc!(r#"
                --app-bar-height
                --app-bar-background-color
                --app-bar-border-bottom
                --app-bar-box-shadow
            "#)}
        </Code>
    }
}

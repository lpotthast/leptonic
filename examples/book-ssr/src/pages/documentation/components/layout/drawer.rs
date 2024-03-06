use indoc::indoc;
use leptonic::{components::prelude::*, prelude::*};
use leptos::*;

use crate::pages::documentation::article::Article;

#[component]
pub fn PageDrawer() -> impl IntoView {
    let (shown, set_shown) = create_signal(true);
    let (shown2, set_shown2) = create_signal(true);

    view! {
        <Article>
            <H1>"Drawer"</H1>

            <P>
                "The "<Code inline=true>"<Drawer>"</Code>" component is intended to be used as a side menu. It is animated to conditionally move in and out of visibility."
                "The required "<Code inline=true>"side"</Code>" prop controls to which side the drawer should move when hiding."
            </P>

            <Toggle state=shown set_state=set_shown/>

            <Box style="display: flex; flex-direction: row; justify-content: flex-start; align-items: flex-start; border: 4px solid gray; width: 100%; height: 20em; overflow: hidden;">
                <Drawer side=DrawerSide::Left shown=shown style="overflow-y: scroll; padding: 0.5em; background-color: var(--brand-color); border-right: 1px solid gray; z-index: 1;">
                    <Stack spacing=Size::Em(0.5)>
                        {(0..8).map(|_| view! { <Skeleton height=Size::Em(3.0)/> }).collect_view()}
                    </Stack>
                </Drawer>
                <Box style="padding: 0.5em; display: flex; flex-direction: column; overflow-y: scroll; width: 100%; height: 100%;">
                    <P>"Scroll ↓"</P>
                    <Stack spacing=Size::Em(0.5)>
                        {(0..8).map(|_| view! { <Skeleton height=Size::Em(3.0)/> }).collect_view()}
                    </Stack>
                </Box>
            </Box>

            <Code>
                {indoc!(r#"
                    <Box style="display: flex; flex-direction: row; justify-content: flex-start; align-items: flex-start; border: 4px solid gray; width: 100%; height: 20em; overflow: hidden;">
                        <Drawer side=DrawerSide::Left shown=shown style="overflow-y: scroll; padding: 0.5em; background-color: var(--brand-color); border-right: 1px solid gray;">
                            <Stack spacing=Size::Em(0.5)>
                                {(0..8).map(|_| view! { <Skeleton height=Size::Em(3.0)/> }).collect_view()}
                            </Stack>
                        </Drawer>
                        <Box style="padding: 0.5em; display: flex; flex-direction: column; overflow-y: scroll; width: 100%; height: 100%;">
                            <P>"Scroll ↓"</P>
                            <Stack spacing=Size::Em(0.5)>
                                {(0..8).map(|_| view! { <Skeleton height=Size::Em(3.0)/> }).collect_view()}
                            </Stack>
                        </Box>
                    </Box>
                "#)}
            </Code>

            <H2>"Layout shifts"</H2>

            <P>
                "To avoid layout shifts, you may declare the drawer as absolutely positioned to let it overlay your content when shown. "
                "This is especially useful when the menu is only animated on user action on small / mobile screens and fills the whole width of the viewport when shown. "
                "When viewing this documentation on a small device, the open- and closeable main and documentation menus are created this way."
            </P>

            <Toggle state=shown2 set_state=set_shown2/>

            <Box style="position: relative; display: flex; flex-direction: row; justify-content: flex-start; align-items: flex-start; border: 4px solid gray; width: 100%; height: 20em; overflow: hidden;">
                <Box style="padding: 0.5em; display: flex; flex-direction: column; overflow-y: scroll; width: 100%; height: 100%;">
                    <P>"Scroll ↓"</P>
                    <Stack spacing=Size::Em(0.5)>
                        {(0..8).map(|_| view! { <Skeleton height=Size::Em(3.0)/> }).collect_view()}
                    </Stack>
                </Box>
                <Drawer side=DrawerSide::Right shown=shown2 style="padding: 0.5em; height: 19.5em; overflow: scroll; position: absolute; top: 0; right: 0; background-color: var(--brand-color); border-left: 1px solid gray; z-index: 1;">
                    <Stack spacing=Size::Em(0.5)>
                        {(0..8).map(|_| view! { <Skeleton height=Size::Em(3.0)/> }).collect_view()}
                    </Stack>
                </Drawer>
            </Box>

            <Code>
                {indoc!(r#"
                    <Box style="position: relative; display: flex; flex-direction: row; justify-content: flex-start; align-items: flex-start; border: 4px solid gray; width: 100%; height: 20em; overflow: hidden;">
                        <Box style="padding: 0.5em; display: flex; flex-direction: column; overflow-y: scroll; width: 100%; height: 100%;">
                            <P>"Scroll ↓"</P>
                            <Stack spacing=Size::Em(0.5)>
                                {(0..8).map(|_| view! { <Skeleton height=Size::Em(3.0)/> }).collect_view()}
                            </Stack>
                        </Box>
                        <Drawer side=DrawerSide::Right shown=shown2 style="padding: 0.5em; height: 19.5em; overflow: scroll; position: absolute; top: 0; right: 0; background-color: var(--brand-color); border-left: 1px solid gray;">
                            <Stack spacing=Size::Em(0.5)>
                                {(0..8).map(|_| view! { <Skeleton height=Size::Em(3.0)/> }).collect_view()}
                            </Stack>
                        </Drawer>
                    </Box>
                "#)}
            </Code>

            <H2>"Styling"</H2>

            <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

            <Code>
                {indoc!(r"
                    --drawer-background-color
                    --drawer-box-shadow
                ")}
            </Code>
        </Article>
    }
}

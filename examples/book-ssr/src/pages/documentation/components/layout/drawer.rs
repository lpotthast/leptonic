use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::drawer_left::DrawerLeftDemo;
use super::demos::drawer_right_overlay::DrawerRightOverlayDemo;
use crate::pages::documentation::demo_shell::DemoShell;
use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageDrawer() -> impl IntoView {
    view! {
        <Article>
            <h1 id="drawer" class="anchor">
                "Drawer"
                <AnchorLink href="#drawer" description="Direct link to article header"/>
            </h1>

            <p>
                "The "<Code inline=true>"<Drawer>"</Code>" component is intended to be used as a side menu. It is animated to conditionally move in and out of visibility."
                "The required "<Code inline=true>"side"</Code>" prop controls to which side the drawer should move when hiding."
            </p>

            <DemoShell source=include_str!("demos/drawer_left.rs")>
                <DrawerLeftDemo />
            </DemoShell>

            <h2 id="layout-shifts" class="anchor">
                "Layout shifts"
                <AnchorLink href="#layout-shifts" description="Direct link to section: Layout shifts"/>
            </h2>

            <p>
                "To avoid layout shifts, you may declare the drawer as absolutely positioned to let it overlay your content when shown. "
                "This is especially useful when the menu is only animated on user action on small / mobile screens and fills the whole width of the viewport when shown. "
                "When viewing this documentation on a small device, the open- and closeable main and documentation menus are created this way."
            </p>

            <DemoShell source=include_str!("demos/drawer_right_overlay.rs")>
                <DrawerRightOverlayDemo />
            </DemoShell>

            <h2 id="styling">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </h2>

            <p>"You may overwrite any of the following CSS variables to meet your styling needs."</p>

            <Code language=Language::Rust>
                {indoc!(r"
                    --drawer-background-color
                    --drawer-box-shadow
                ")}
            </Code>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Drawer", link: "#drawer" },
                Toc::Leaf { title: "Layout shifts", link: "#layout-shifts" },
                Toc::Leaf { title: "Styling", link: "#styling" },
            ]
        }/>
    }
}

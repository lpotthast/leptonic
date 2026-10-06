use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::app_bar::AppBarDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageAppBar() -> impl IntoView {
    view! {
        <DocPage title="App Bar">
            <p>
                "The "<Code inline=true>"AppBar"</Code>" component is a horizontal bar that sticks to the top of its "
                "scrolling parent. Many app layouts use one as their entry point, holding the app name, navigation and "
                "global actions. It lays its children out in a row, with space between them."
            </p>

            <Demo description="App bar sticking to the top of a scrolling container" source=include_str!("demos/app_bar.rs")>
                <AppBarDemo/>
            </Demo>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="AppBar">
                    <ApiRow name="height" ty="Option<Height>" default="None">
                        "The bar height. Sets "<Code inline=true>"--app-bar-height"</Code>
                        " on the element; without it, the theme\u{2019}s value applies."
                    </ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                        "Additional classes and styles."
                    </ApiRow>
                    <ApiRow name="children" ty="Children">"The bar content."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>
                    "The theme gives the bar a high "<Code inline=true>"z-index"</Code>" so that it stays above scrolling "
                    "content. Override any of these CSS variables to adapt it to your design:"
                </p>
                <CssVariables prefix="--app-bar-" scss=theme_scss!("app-bar")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::components::Drawer.materialize()>"Drawer"</Link></li>
                <li><Link href=routes::doc::components::Stack.materialize()>"Stack"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

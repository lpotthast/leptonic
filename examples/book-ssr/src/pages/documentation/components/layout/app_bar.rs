use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::app_bar::AppBarDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageAppBar() -> impl IntoView {
    view! {
        <DocPage title="App Bar Component">
            <p>
                "An app bar is the horizontal bar at the top of an app, holding its name, the main navigation and global "
                "actions such as notifications or the user menu. The "<Code inline=true>"AppBar"</Code>" component renders "
                "it as a "<Code inline=true>"<header>"</Code>" that sticks to the top of its scrolling parent and lays its "
                "children out in a row, with the space between them."
            </p>

            <Demo
                description="App bar with a title and two icon buttons, sticking to the top of a scrolling frame"
                source=include_str!("demos/app_bar.rs")
            >
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

            <Section title="Accessibility">
                <p>
                    "Rendered outside of "<Code inline=true>"<main>"</Code>", "<Code inline=true>"<article>"</Code>", "
                    <Code inline=true>"<nav>"</Code>" and similar elements, a "<Code inline=true>"<header>"</Code>
                    " is the page\u{2019}s "<Code inline=true>"banner"</Code>" landmark, which screen reader users jump to "
                    "directly. Put the app bar next to your "<Code inline=true>"<main>"</Code>", not inside it, and wrap its "
                    "links in a "<Code inline=true>"<nav>"</Code>". Icon-only buttons need an "
                    <Code inline=true>"aria-label"</Code>", as in the demo: the "<Link href=routes::doc::Icon.materialize()>"Icon"</Link>
                    " inside is hidden from screen readers."
                </p>
            </Section>

            <Section title="Styling">
                <p>
                    "The theme gives the bar a high "<Code inline=true>"z-index"</Code>" (1000) so that it stays above "
                    "scrolling content. Override any of these CSS variables to adapt it to your design:"
                </p>
                <CssVariables prefix="--app-bar-" scss=theme_scss!("app-bar")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Layout.materialize()>"Content & Layout"</Link></li>
                <li><Link href=routes::doc::Stack.materialize()>"Stack Component"</Link></li>
                <li><Link href=routes::doc::Icon.materialize()>"Icon Component"</Link></li>
                <li><Link href=routes::doc::Button.materialize()>"Button"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

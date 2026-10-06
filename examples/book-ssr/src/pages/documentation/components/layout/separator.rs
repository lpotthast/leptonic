use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::separator::SeparatorDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageSeparator() -> impl IntoView {
    view! {
        <DocPage title="Separator Component">
            <p>
                "The themed "<Code inline=true>"Separator"</Code>" component draws a line between content: a horizontal "
                <Code inline=true>"<hr>"</Code>", or a vertical separator between side-by-side content. See the "
                <Link href=routes::doc::Separator.materialize()>"Separator overview"</Link>
                " for concept guidance."
            </p>

            <Demo description="Horizontal separator between paragraphs and a vertical one in a row" source=include_str!("demos/separator.rs")>
                <SeparatorDemo/>
            </Demo>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="components::separator::Separator">
                    <ApiRow name="orientation" ty="Orientation" default="Horizontal">
                        "A vertical separator stretches to the height of its row; put it in a flex container."
                    </ApiRow>
                    <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the separator."</ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                        "Additional classes and styles."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt separators to your design:"</p>
                <CssVariables prefix="--separator-" scss=theme_scss!("separator")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Separator.materialize()>"Separator overview"</Link></li>
                <li><Link href=routes::doc::separator::Hook.materialize()>"use_separator"</Link></li>
                <li><Link href=routes::doc::separator::Atom.materialize()>"Separator Atom"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

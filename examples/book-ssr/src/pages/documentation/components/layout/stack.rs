use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::stack::StackDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageStack() -> impl IntoView {
    view! {
        <DocPage title="Stack Component">
            <p>
                "A stack displays its children one after the other, spaced out by a fixed distance. The "
                <Code inline=true>"Stack"</Code>" component is a flex container that arranges them vertically or "
                "horizontally and centers them on both axes."
            </p>

            <Demo description="A vertical and a horizontal stack of three items each" source=include_str!("demos/stack.rs")>
                <StackDemo/>
            </Demo>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="Stack">
                    <ApiRow name="spacing" ty="CssDimension">"The space between two children. Required."</ApiRow>
                    <ApiRow name="orientation" ty="StackOrientation" default="Vertical">
                        <Code inline=true>"Vertical"</Code>" or "<Code inline=true>"Horizontal"</Code>
                        ". Rendered as the "<Code inline=true>"data-orientation"</Code>" attribute."
                    </ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                        "Additional classes and styles."
                    </ApiRow>
                    <ApiRow name="children" ty="Children">"The stacked elements."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>
                    "The stack sets this variable from its "<Code inline=true>"spacing"</Code>" prop. "
                    "Its children are centered along and across its direction ("<Code inline=true>"justify-content"</Code>
                    " and "<Code inline=true>"align-items"</Code>" are "<Code inline=true>"center"</Code>"); override "
                    "them through "<Code inline=true>"classes"</Code>", e.g. "<Code inline=true>"align-items: stretch"</Code>
                    " for children as wide as a vertical stack."
                </p>
                <CssVariables prefix="--gap" scss=theme_scss!("stack")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Layout.materialize()>"Content & Layout"</Link></li>
                <li><Link href=routes::doc::GridLayout.materialize()>"Grid Layout Components"</Link></li>
                <li><Link href=routes::doc::Toolbar.materialize()>"Toolbar"</Link>" (controls in a row, one tab stop)"</li>
            </SeeAlso>
        </DocPage>
    }
}

use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::stack::StackDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageStack() -> impl IntoView {
    view! {
        <DocPage title="Stack">
            <p>
                "A stack displays its children one after the other, spaced out by a fixed distance. The "
                <Code inline=true>"Stack"</Code>" component is a flex container that arranges them vertically or "
                "horizontally and centers them on the cross axis."
            </p>

            <Demo description="Vertical and horizontal stack of three items" source=include_str!("demos/stack.rs")>
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
                    "Its children are centered; override "<Code inline=true>"align-items"</Code>" on "
                    <Code inline=true>".leptonic-stack"</Code>" to change that."
                </p>
                <CssVariables prefix="--gap" scss=theme_scss!("stack")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::grid::Component.materialize()>"Grid component"</Link></li>
                <li><Link href=routes::doc::components::Skeleton.materialize()>"Skeleton"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::collapsible::CollapsibleDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageCollapsible() -> impl IntoView {
    view! {
        <DocPage title="Collapsible component">
            <p>
                "The themed "<Code inline=true>"Collapsible"</Code>" shows a header that toggles the visibility of a body "
                "when clicked. "<Code inline=true>"Collapsibles"</Code>" groups several of them, for example to build an "
                "accordion in which only one body is open at a time. See the "
                <Link href=routes::doc::Collapsible.materialize()>"Collapsible overview"</Link>" for concept guidance."
            </p>

            <Demo description="Accordion of three collapsibles" source=include_str!("demos/collapsible.rs")>
                <CollapsibleDemo/>
            </Demo>

            <Section title="Props">
                <Section title="Collapsible">
                    <ApiTable kind=ApiKind::Props of="Collapsible">
                        <ApiRow name="open" ty="bool" default="false">"Whether the body is initially shown."</ApiRow>
                        <ApiRow name="on_open" ty="Option<OnOpen>" default="None">
                            "What happens to the other collapsibles of the enclosing "<Code inline=true>"Collapsibles"</Code>
                            " when this one opens. Overrides the group\u{2019}s "<Code inline=true>"default_on_open"</Code>
                            ". Only meaningful inside a "<Code inline=true>"Collapsibles"</Code>"."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles."
                        </ApiRow>
                        <ApiRow name="collapsible_header" ty="CollapsibleHeader">
                            "The always visible header slot. Clicking it toggles the body."
                        </ApiRow>
                        <ApiRow name="collapsible_body" ty="CollapsibleBody">"The body slot."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="CollapsibleHeader and CollapsibleBody">
                    <p>
                        "Both slots take their content as children. "<Code inline=true>"CollapsibleBody"</Code>
                        " additionally accepts a static "<Code inline=true>"class"</Code>" ("<Code inline=true>"String"</Code>
                        "), added to the body element."
                    </p>
                </Section>

                <Section title="Collapsibles">
                    <ApiTable kind=ApiKind::Props of="Collapsibles">
                        <ApiRow name="default_on_open" ty="OnOpen">
                            "What happens to the other collapsibles when one of them opens. Required."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">
                            "Content containing the "<Code inline=true>"Collapsible"</Code>"s, at any depth."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Grouping">
                <p>
                    "A "<Code inline=true>"Collapsible"</Code>" works on its own. Wrapped in "
                    <Code inline=true>"Collapsibles"</Code>
                    ", it registers with the group, which reacts when a member opens:"
                </p>
                <DocTable headers=&["OnOpen", "Behavior"]>
                    <TableRow>
                        <TableCell><Code inline=true>"DoNothing"</Code></TableCell>
                        <TableCell>"The other collapsibles keep their state. This is the default of the enum."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"CloseOthers"</Code></TableCell>
                        <TableCell>"All other collapsibles of the group close, so at most one stays open."</TableCell>
                    </TableRow>
                </DocTable>
                <p>
                    "In the demo above, the group closes the others whenever a collapsible opens. The third collapsible "
                    "overrides this with "<Code inline=true>"on_open=OnOpen::DoNothing"</Code>
                    ": opening it leaves the others open, but opening one of the others still closes it."
                </p>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt collapsibles to your design:"</p>
                <CssVariables prefix="--collapsible-" scss=theme_scss!("collapsible")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Collapsible.materialize()>"Collapsible overview"</Link></li>
                <li><Link href=routes::doc::collapsible::Hook.materialize()>"use_disclosure"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

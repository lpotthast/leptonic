use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::separator::SeparatorAtomDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageAtomSeparator() -> impl IntoView {
    view! {
        <DocPage title="Separator Atom">
            <p>
                "The unstyled "<Code inline=true>"Separator"</Code>" atom renders a separator in the right element for its "
                "orientation. See the "<Link href=routes::doc::Separator.materialize()>"Separator overview"</Link>
                " for concept guidance."
            </p>

            <Section title="Hooks Used">
                <p>
                    "It calls "<Link href=routes::doc::separator::Hook.materialize()>"use_separator"</Link>
                    ": a horizontal separator is an "<Code inline=true>"<hr>"</Code>"; a vertical one, or any separator "
                    "inside a "<Link href=routes::doc::menu::Atom.materialize()>"Menu"</Link>" atom, is a "<Code inline=true>"<div>"</Code>
                    " with "<Code inline=true>"role=\"separator\""</Code>"."
                </p>
            </Section>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="atoms::separator::Separator">
                    <ApiRow name="orientation" ty="Orientation" default="Horizontal">"Whether the separator divides stacked or side-by-side content."</ApiRow>
                    <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the separator."</ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the separator element."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::prelude as atoms;

                        view! {
                            <atoms::Menu collection=actions aria_label="Actions">
                                <atoms::MenuItem key="copy">"Copy"</atoms::MenuItem>
                                // A <div role="separator"> between the items.
                                <atoms::Separator classes="my-menu-separator"/>
                                <atoms::MenuItem key="delete">"Delete"</atoms::MenuItem>
                            </atoms::Menu>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <Demo description="Horizontal and vertical separators" source=include_str!("demos/separator.rs")>
                    <SeparatorAtomDemo/>
                </Demo>
            </Section>

            <Section title="Styling">
                <p>
                    "The atom adds no classes. Vertical separators carry "<Code inline=true>"aria-orientation=\"vertical\""</Code>
                    ", which you can select on:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r#"
                        .my-separator { border: none; border-top: 1px solid var(--border); }
                        .my-separator[aria-orientation="vertical"] { border-top: none; border-left: 1px solid var(--border); align-self: stretch; }
                    "#)}
                </Code>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Separator.materialize()>"Separator overview"</Link></li>
                <li><Link href=routes::doc::separator::Hook.materialize()>"use_separator"</Link></li>
                <li><Link href=routes::doc::separator::Component.materialize()>"Separator Component"</Link></li>
                <li><Link href=routes::doc::toolbar::Atom.materialize()>"Toolbar Atom"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

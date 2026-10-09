use indoc::indoc;
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
                    <ApiRow name="orientation" ty="Signal<Orientation>" default="Horizontal">
                        "Whether the separator divides stacked or side-by-side content: a value or any signal."
                    </ApiRow>
                    <ApiRow name="id" ty="Option<String>" default="None">"The element id."</ApiRow>
                    <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the separator."</ApiRow>
                    <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"The ids of the elements naming the separator."</ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the separator element."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms;

                        view! {
                            <atoms::menu::Menu collection=actions aria_label="Actions">
                                <atoms::menu::MenuItem key="copy">"Copy"</atoms::menu::MenuItem>
                                // A <div role="separator"> between the items.
                                <atoms::separator::Separator classes="my-menu-separator"/>
                                <atoms::menu::MenuItem key="delete">"Delete"</atoms::menu::MenuItem>
                            </atoms::menu::Menu>
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
                    "The atom brings no styles. It renders the class "<Code inline=true>"leptonic-Separator"</Code>
                    " followed by the "<Code inline=true>"classes"</Code>" you pass. A horizontal separator is an "
                    <Code inline=true>"<hr>"</Code>" (reset the browser\u{2019}s border); a vertical one carries "
                    <Code inline=true>"aria-orientation=\"vertical\""</Code>", which you can select on. The book\u{2019}s "
                    "demos use these rules:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r#"
                        .my-separator { margin: 1em 0; border: none; border-top: 1px solid var(--border); }
                        .my-separator[aria-orientation="vertical"] { align-self: stretch; margin: 0; border-top: none; border-left: 1px solid var(--border); }
                    "#)}
                </Code>
                <p>
                    "Leptonic also ships an optional atom theme that styles the default classes, for apps that don\u{2019}t "
                    "want to start from scratch: "<Code inline=true>"@use \"leptonic/leptonic-atoms\";"</Code>"."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Separator.materialize()>"Separator overview"</Link></li>
                <li><Link href=routes::doc::separator::Hook.materialize()>"use_separator"</Link></li>
                <li><Link href=routes::doc::toolbar::Atom.materialize()>"Toolbar Atom"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

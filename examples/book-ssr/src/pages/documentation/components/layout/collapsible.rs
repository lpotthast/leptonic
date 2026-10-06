use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{collapsible::CollapsibleDemo, collapsible_basic::CollapsibleBasicDemo};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageCollapsible() -> impl IntoView {
    view! {
        <DocPage title="Disclosure Components">
            <p>
                "The themed "<AnchorLink href="#collapsible">"Collapsible"</AnchorLink>" shows a header button that opens and "
                "closes a body; "<AnchorLink href="#collapsibles">"Collapsibles"</AnchorLink>" groups several of them into an "
                "accordion. Both are built on the "<Link href=routes::doc::disclosure::Atom.materialize()>"Disclosure Atoms"</Link>
                ". See the "<Link href=routes::doc::Disclosure.materialize()>"Disclosure overview"</Link>" for concept guidance."
            </p>

            <Demo description="A collapsible whose open state lives in the app, with a Disabled checkbox" source=include_str!("demos/collapsible_basic.rs")>
                <CollapsibleBasicDemo/>
            </Demo>

            <Section title="Collapsible">
                <p>
                    "Fill the "<Code inline=true>"CollapsibleHeader"</Code>" and "<Code inline=true>"CollapsibleBody"</Code>
                    " slots. Keep the open state in your app with "<Code inline=true>"is_expanded"</Code>" and "
                    <Code inline=true>"set_expanded"</Code>", as the demo above does, or let the collapsible hold it, starting "
                    "at "<Code inline=true>"default_expanded"</Code>"."
                </p>
                <Section title="Props" id="collapsible-props">
                    <ApiTable kind=ApiKind::Props of="Collapsible">
                        <ApiRow name="id" ty="Option<Key>" default="generated">"Its key in a "<Code inline=true>"Collapsibles"</Code>" group."</ApiRow>
                        <ApiRow name="default_expanded" ty="bool" default="false">
                            "Whether the body starts open. Ignored with "<Code inline=true>"is_expanded"</Code>" or in a group."
                        </ApiRow>
                        <ApiRow name="is_expanded" ty="Option<Signal<bool>>" default="None">
                            "Whether the body is open (controlled): a value or any signal."
                        </ApiRow>
                        <ApiRow name="set_expanded" ty="Option<Out<bool>>" default="None">
                            "Receives the open state: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>", closure, \u{2026}"
                        </ApiRow>
                        <ApiRow name="on_expanded_change" ty="Option<Callback<bool>>" default="None">"Called when the body opens or closes."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the header can\u{2019}t toggle the body."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Added to the collapsible element."</ApiRow>
                        <ApiRow name="collapsible_header" ty="CollapsibleHeader">
                            "The header slot: the content of the header button. Required."
                        </ApiRow>
                        <ApiRow name="collapsible_body" ty="CollapsibleBody">
                            "The body slot, with optional "<Code inline=true>"classes"</Code>". Required."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Collapsibles">
                <p>
                    "An accordion: by default, opening one collapsible closes the others; "
                    <Code inline=true>"DisclosureGroupExpansion::Multiple"</Code>" lets any number stay open. Give each "
                    "collapsible an "<Code inline=true>"id"</Code>" to address it in the open keys. The collapsibles may sit "
                    "at any depth inside the group, e.g. in a "<Code inline=true>"Stack"</Code>"."
                </p>

                <Demo description="An accordion of three collapsibles whose open key lives in the app" source=include_str!("demos/collapsible.rs")>
                    <CollapsibleDemo/>
                </Demo>

                <Section title="Props" id="collapsibles-props">
                    <ApiTable kind=ApiKind::Props of="Collapsibles">
                        <ApiRow name="expansion" ty="DisclosureGroupExpansion" default="Single">
                            <Code inline=true>"Single"</Code>": opening one closes the others; "
                            <Code inline=true>"Multiple"</Code>": any number."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether all collapsibles are disabled."</ApiRow>
                        <ApiRow name="default_expanded_keys" ty="Vec<Key>" default="empty">
                            "The initially open collapsibles (their "<Code inline=true>"id"</Code>"s). Ignored with "
                            <Code inline=true>"expanded_keys"</Code>"."
                        </ApiRow>
                        <ApiRow name="expanded_keys" ty="Option<Signal<HashSet<Key>>>" default="None">"The open collapsibles (controlled): a value or any signal."</ApiRow>
                        <ApiRow name="set_expanded_keys" ty="Option<Out<HashSet<Key>>>" default="None">
                            "Receives the open collapsibles: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>", closure, \u{2026}"
                        </ApiRow>
                        <ApiRow name="on_expanded_change" ty="Option<Callback<HashSet<Key>>>" default="None">"Called with the open collapsibles."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Added to the group element."</ApiRow>
                        <ApiRow name="children" ty="Children">"Content containing the "<Code inline=true>"Collapsible"</Code>"s, at any depth. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Accessibility">
                <p>
                    "The header button isn\u{2019}t inside a heading, so heading navigation doesn\u{2019}t find the sections of "
                    "an accordion. Where that matters, build the disclosure from the "
                    <Link href=routes::doc::disclosure::Atom.materialize()>"Disclosure Atoms"</Link>" and put the trigger in a "
                    "heading."
                </p>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt collapsibles to your design:"</p>
                <CssVariables prefix="--collapsible-" scss=theme_scss!("collapsible")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Disclosure.materialize()>"Disclosure overview"</Link></li>
                <li><Link href=routes::doc::disclosure::Hook.materialize()>"Disclosure Hooks"</Link></li>
                <li><Link href=routes::doc::disclosure::Atom.materialize()>"Disclosure Atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

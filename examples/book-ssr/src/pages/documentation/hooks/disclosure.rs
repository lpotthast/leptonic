use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::disclosure::DisclosureDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseDisclosure() -> impl IntoView {
    view! {
        <DocPage title="use_disclosure">
            <p>
                "The "<Code inline=true>"use_disclosure"</Code>" hook connects a trigger button with a content panel it "
                "shows and hides, and "<Code inline=true>"use_disclosure_state"</Code>" holds the expanded state. "
                "See the "<Link href=routes::doc::Collapsible.materialize()>"Collapsible overview"</Link>" for concept guidance."
            </p>

            <ReactAria hook="useDisclosure"/>

            <Section title="use_disclosure">
                <Section title="Input" id="use-disclosure-input">
                    <p>
                        "The hook doesn\u{2019}t own the expanded state: it reads "<Code inline=true>"is_expanded"</Code>
                        " and reports requested changes through "<Code inline=true>"on_expanded_change"</Code>"."
                    </p>

                    <ApiTable kind=ApiKind::Input of="UseDisclosureInput">
                        <ApiRow name="is_expanded" ty="Signal<bool>" default="false">"Whether the panel is expanded."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">
                            "Whether the disclosure is disabled. A disabled trigger ignores clicks and keys and gets "
                            <Code inline=true>"aria-disabled=\"true\""</Code>"."
                        </ApiRow>
                        <ApiRow name="on_expanded_change" ty="Option<Callback<bool>>" default="None">
                            "Called with the new expanded state when the trigger is activated. Without it, the trigger does nothing."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-disclosure-return">
                    <ApiTable kind=ApiKind::Return of="UseDisclosureReturn">
                        <ApiRow name="trigger_props" ty="UseDisclosureTriggerProps">
                            "Attributes and event handlers for the trigger button: "<Code inline=true>"id"</Code>", "
                            <Code inline=true>"aria-expanded"</Code>", "<Code inline=true>"aria-controls"</Code>", "
                            <Code inline=true>"aria-disabled"</Code>", "<Code inline=true>"data-focus-visible"</Code>
                            " and the click, key and focus handlers. Spread with "<Code inline=true>"{..trigger_props.into_attrs()}"</Code>"."
                        </ApiRow>
                        <ApiRow name="content_props" ty="UseDisclosureContentProps">
                            "Attributes for the panel: "<Code inline=true>"id"</Code>", "<Code inline=true>"role=\"region\""</Code>", "
                            <Code inline=true>"aria-labelledby"</Code>" (the trigger) and "<Code inline=true>"aria-hidden"</Code>
                            " while collapsed."
                        </ApiRow>
                        <ApiRow name="trigger_id, content_id" ty="String">"The generated ids of trigger and panel."</ApiRow>
                        <ApiRow name="is_expanded" ty="Signal<bool>">"The "<Code inline=true>"is_expanded"</Code>" input, passed through."</ApiRow>
                        <ApiRow name="toggle" ty="Callback<()>">
                            "Requests the opposite state through "<Code inline=true>"on_expanded_change"</Code>
                            ", unless the disclosure is disabled."
                        </ApiRow>
                        <ApiRow name="is_focus_visible" ty="Signal<bool>">
                            "Whether the trigger has keyboard focus and should show a focus ring."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-disclosure-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            let UseDisclosureStateReturn { is_expanded, expand, collapse, .. } = use_disclosure_state(false);

                            let disclosure = use_disclosure(UseDisclosureInput {
                                is_expanded,
                                on_expanded_change: Some(Callback::new(move |expanded: bool| {
                                    if expanded { expand.run(()) } else { collapse.run(()) }
                                })),
                                ..Default::default()
                            });

                            view! {
                                <button {..disclosure.trigger_props.into_attrs()}>"Details"</button>
                                <div {..disclosure.content_props.into_attrs()} hidden=move || !is_expanded.get()>
                                    "Hidden content"
                                </div>
                            }
                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="use_disclosure_state">
                <p>
                    <Code inline=true>"use_disclosure_state(default_expanded: bool)"</Code>
                    " creates the expanded state and the callbacks that change it."
                </p>

                <ApiTable kind=ApiKind::Return of="UseDisclosureStateReturn">
                    <ApiRow name="is_expanded" ty="Signal<bool>">"Whether the disclosure is expanded."</ApiRow>
                    <ApiRow name="expand, collapse" ty="Callback<()>">"Expand or collapse the disclosure."</ApiRow>
                    <ApiRow name="toggle" ty="Callback<()>">"Flip the expanded state."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Demo">
                <p>"The panel is hidden with CSS while the hook marks it "<Code inline=true>"aria-hidden"</Code>"."</p>

                <Demo description="Disclosure with a toggle button, a collapsible panel and a disabled switch" source=include_str!("demos/disclosure.rs")>
                    <DisclosureDemo/>
                </Demo>
            </Section>

            <Section title="Hiding the panel">
                <p>
                    "The hook marks a collapsed panel with "<Code inline=true>"aria-hidden=\"true\""</Code>
                    " but leaves it visible. Hide it yourself, so that it disappears for sighted users and its content can\u{2019}t "
                    "be reached with "<Code inline=true>"Tab"</Code>": set the "<Code inline=true>"hidden"</Code>
                    " attribute, select on "<Code inline=true>"[aria-hidden=\"true\"]"</Code>" in CSS, or don\u{2019}t render the content."
                </p>
            </Section>

            <Section title="Use Cases">
                <ul>
                    <li>"FAQ accordions"</li>
                    <li>"Collapsible sections"</li>
                    <li>"Expandable cards"</li>
                    <li>"Show more/less content"</li>
                </ul>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="Tab">"Focus the trigger."</KeyRow>
                    <KeyRow keys="Enter / Space">"Expand or collapse the panel."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Collapsible.materialize()>"Collapsible overview"</Link></li>
                <li><Link href=routes::doc::collapsible::Component.materialize()>"Collapsible component"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

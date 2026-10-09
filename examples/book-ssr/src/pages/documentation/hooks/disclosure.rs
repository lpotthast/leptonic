use indoc::indoc;
use leptos::prelude::*;

use super::demos::disclosure::DisclosureDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageUseDisclosure() -> impl IntoView {
    view! {
        <DocPage title="Disclosure Hooks">
            <p>
                <AnchorLink href="#use-disclosure">"use_disclosure"</AnchorLink>" connects a trigger button with the panel it "
                "shows and hides, "<AnchorLink href="#use-disclosure-state">"use_disclosure_state"</AnchorLink>" holds whether "
                "it is expanded, and "<AnchorLink href="#use-disclosure-group-state">"use_disclosure_group_state"</AnchorLink>
                " which disclosures of an accordion are. See the "<Link href=routes::doc::Disclosure.materialize()>"Disclosure overview"</Link>
                " for concept guidance."
            </p>

            <ReactAria hook="useDisclosure"/>

            <Section title="Demo">
                <p>
                    "A disclosure built from the hooks, styled through the "<Code inline=true>"aria-expanded"</Code>" and "
                    <Code inline=true>"disabled"</Code>" attributes they set."
                </p>
                <Demo description="Disclosure with a toggle button, a collapsible panel and a Disabled checkbox" source=include_str!("demos/disclosure.rs")>
                    <DisclosureDemo/>
                </Demo>
            </Section>

            <Section title="use_disclosure">
                <Section title="Input" id="use-disclosure-input">
                    <p>
                        "Pass a "<Code inline=true>"UseDisclosureInput"</Code>" with both fields named; "<Code inline=true>"state"</Code>
                        " is the result of "<AnchorLink href="#use-disclosure-state">"use_disclosure_state"</AnchorLink>"."
                    </p>

                    <ApiTable kind=ApiKind::Input of="UseDisclosureInput">
                        <ApiRow name="state" ty="DisclosureState">"Whether the panel is expanded. Required."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the trigger can\u{2019}t toggle the panel."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-disclosure-return">
                    <ApiTable kind=ApiKind::Return of="UseDisclosureReturn">
                        <ApiRow name="button" ty="UseButtonInput">
                            "The trigger\u{2019}s configuration: its id, "<Code inline=true>"aria-expanded"</Code>", "
                            <Code inline=true>"aria-controls"</Code>" and the toggling. Pass it to "
                            <Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>
                            " (add your own settings with struct update syntax)."
                        </ApiRow>
                        <ApiRow name="panel_props" ty="UseDisclosurePanelProps">
                            "For the panel: its id, "<Code inline=true>"role=\"group\""</Code>", "<Code inline=true>"aria-labelledby"</Code>
                            " (the trigger), "<Code inline=true>"aria-hidden"</Code>" and "<Code inline=true>"hidden=\"until-found\""</Code>
                            " while collapsed, and its capture. Spread with "<Code inline=true>"{..panel_props.into_attrs()}"</Code>"."
                        </ApiRow>
                        <ApiRow name="trigger_id" ty="String">"The trigger\u{2019}s id (also in "<Code inline=true>"button"</Code>")."</ApiRow>
                        <ApiRow name="panel_element" ty="CapturedElement">"The panel element, once rendered."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-disclosure-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use leptonic::{
                                IntoAttrs,
                                hooks::{
                                    button::use_button,
                                    disclosure::{
                                        UseDisclosureInput,
                                        UseDisclosureStateInput,
                                        use_disclosure,
                                        use_disclosure_state,
                                    },
                                },
                            };

                            let state = use_disclosure_state(UseDisclosureStateInput::default());
                            let disclosure = use_disclosure(UseDisclosureInput { state, is_disabled: false.into() });

                            let (button_attrs, button_styles) = use_button(disclosure.button).props.into_parts();

                            view! {
                                <h3><button {..button_attrs} style=button_styles>"Details"</button></h3>
                                <div {..disclosure.panel_props.into_attrs()}>"Hidden content"</div>
                            }
                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="use_disclosure_state">
                <p>"Holds whether a disclosure is expanded."</p>

                <Section title="Input" id="use-disclosure-state-input">
                    <ApiTable kind=ApiKind::Input of="UseDisclosureStateInput">
                        <ApiRow name="default_expanded" ty="bool" default="false">"Whether it starts expanded."</ApiRow>
                        <ApiRow name="value" ty="Option<ValueBinding<bool>>" default="None">
                            "The expanded state as app state (e.g. an "<Code inline=true>"RwSignal<bool>"</Code>"), replacing "
                            <Code inline=true>"default_expanded"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_expanded_change" ty="Option<Callback<bool>>" default="None">"Called when it expands or collapses."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-disclosure-state-return">
                    <ApiTable kind=ApiKind::Fields of="DisclosureState">
                        <ApiRow name="is_expanded" ty="Signal<bool>">
                            "Whether the disclosure is expanded. Change it with "<Code inline=true>"expand()"</Code>", "
                            <Code inline=true>"collapse()"</Code>", "<Code inline=true>"toggle()"</Code>" or "
                            <Code inline=true>"set_expanded(bool)"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-disclosure-state-example">
                    <Code language=Language::Rust>
                        {indoc!(r"
                            use leptonic::{
                                ValueBinding,
                                hooks::disclosure::{UseDisclosureStateInput, use_disclosure_state},
                            };

                            // The expanded state lives in the app, e.g. to expand the disclosure from elsewhere.
                            let expanded = RwSignal::new(true);
                            let state = use_disclosure_state(UseDisclosureStateInput {
                                value: Some(ValueBinding::from(expanded)),
                                ..UseDisclosureStateInput::default()
                            });
                        ")}
                    </Code>
                </Section>
            </Section>

            <Section title="use_disclosure_group_state">
                <p>
                    "Holds which disclosures of a group (an accordion) are expanded, by key. The "
                    <Link href=format!("{}#disclosuregroup", routes::doc::disclosure::Atom.materialize())>"DisclosureGroup"</Link>
                    " atom uses it and gives each of its disclosures the expanded state of its key."
                </p>

                <Section title="Input" id="use-disclosure-group-state-input">
                    <ApiTable kind=ApiKind::Input of="UseDisclosureGroupStateInput">
                        <ApiRow name="expansion" ty="Signal<DisclosureGroupExpansion>" default="Single">
                            <Code inline=true>"Single"</Code>": expanding one collapses the others; "<Code inline=true>"Multiple"</Code>": any number."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether all disclosures are disabled."</ApiRow>
                        <ApiRow name="default_expanded_keys" ty="HashSet<Key>" default="empty">
                            "The initially expanded disclosures, in order (a single-expansion group keeps the first)."
                        </ApiRow>
                        <ApiRow name="value" ty="Option<ValueBinding<HashSet<Key>>>" default="None">"The expanded keys as app state."</ApiRow>
                        <ApiRow name="on_expanded_change" ty="Option<Callback<HashSet<Key>>>" default="None">"Called with the expanded keys."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-disclosure-group-state-return">
                    <ApiTable kind=ApiKind::Fields of="DisclosureGroupState">
                        <ApiRow name="expansion" ty="Signal<DisclosureGroupExpansion>">"From the input."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>">"From the input."</ApiRow>
                        <ApiRow name="expanded_keys" ty="Signal<HashSet<Key>>">
                            "The expanded disclosures. "<Code inline=true>"is_expanded(&key)"</Code>", "<Code inline=true>"toggle_key(&key)"</Code>
                            " and "<Code inline=true>"set_expanded_keys(keys)"</Code>" read and change them."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-disclosure-group-state-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use leptonic::{
                                hooks::{
                                    collections::Key,
                                    disclosure::{
                                        UseDisclosureGroupStateInput,
                                        use_disclosure_group_state,
                                    },
                                },
                            };

                            let group = use_disclosure_group_state(UseDisclosureGroupStateInput {
                                default_expanded_keys: HashSet::from([Key::from("shipping")]),
                                ..UseDisclosureGroupStateInput::default()
                            });
                            // Expanding "returns" collapses "shipping".
                            group.toggle_key(&Key::from("returns"));
                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="Hiding the Panel">
                <p>
                    "A collapsed panel is "<Code inline=true>"hidden=\"until-found\""</Code>": hidden, but the browser\u{2019}s find "
                    "in page still finds its content, and expands the disclosure when it does. Put padding on the panel\u{2019}s "
                    "content rather than the panel: a panel hidden this way keeps its own box."
                </p>
                <p>
                    "To animate the panel, transition its size: while it expands or collapses, "
                    <Code inline=true>"--disclosure-panel-width"</Code>" and "<Code inline=true>"--disclosure-panel-height"</Code>
                    " hold its size in pixels (afterwards "<Code inline=true>"auto"</Code>" when expanded)."
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .my-panel { height: var(--disclosure-panel-height); overflow: clip; transition: height 200ms; }
                        @media (prefers-reduced-motion: reduce) { .my-panel { transition: none; } }
                    ")}
                </Code>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Disclosure.materialize()>"Disclosure overview"</Link></li>
                <li><Link href=routes::doc::disclosure::Atom.materialize()>"Disclosure Atoms"</Link></li>
                <li><Link href=routes::doc::button::Hook.materialize()>"use_button"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

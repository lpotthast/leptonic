use indoc::indoc;
use leptos::prelude::*;

use super::demos::disclosure::DisclosureAtomDemo;
use crate::{kit::*, routes};

/// A link to a section of the disclosure hooks page.
fn hook_section(id: &str) -> String {
    format!("{}#{id}", routes::doc::disclosure::Hook.materialize())
}

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomDisclosure() -> impl IntoView {
    view! {
        <DocPage title="Disclosure Atoms">
            <p>
                "The unstyled "<AnchorLink href="#disclosure">"Disclosure"</AnchorLink>", "
                <AnchorLink href="#disclosuretrigger">"DisclosureTrigger"</AnchorLink>", "
                <AnchorLink href="#disclosurepanel">"DisclosurePanel"</AnchorLink>" and "
                <AnchorLink href="#disclosuregroup">"DisclosureGroup"</AnchorLink>" render a button showing and hiding a "
                "panel, and accordions of them. See the "<Link href=routes::doc::Disclosure.materialize()>"Disclosure overview"</Link>
                " for concept guidance."
            </p>

            <Section title="Hooks Used">
                <DocTable headers=&["Atom", "Hook"]>
                    <TableRow>
                        <TableCell><Code inline=true>"Disclosure"</Code></TableCell>
                        <TableCell>
                            <Link href=hook_section("use-disclosure")>"use_disclosure"</Link>" and "
                            <Link href=hook_section("use-disclosure-state")>"use_disclosure_state"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"DisclosureTrigger"</Code></TableCell>
                        <TableCell>
                            "None of its own: it hands the "<Link href=routes::doc::button::Atom.materialize()>"Button"</Link>
                            " atom inside it the toggling, "<Code inline=true>"aria-expanded"</Code>" and "
                            <Code inline=true>"aria-controls"</Code>" through a "
                            <Link href=routes::doc::interactions::PressResponder.materialize()>"PressResponder"</Link>"."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"DisclosureGroup"</Code></TableCell>
                        <TableCell><Link href=hook_section("use-disclosure-group-state")>"use_disclosure_group_state"</Link></TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::prelude::{Button, Disclosure, DisclosurePanel, DisclosureTrigger};

                        view! {
                            <Disclosure>
                                <h3><DisclosureTrigger><Button>"Details"</Button></DisclosureTrigger></h3>
                                <DisclosurePanel>"Content"</DisclosurePanel>
                            </Disclosure>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "An accordion of two disclosures, styled through the "<AnchorLink href="#data-attributes">"data attributes"</AnchorLink>
                    ". The expanded disclosures live in an "<Code inline=true>"RwSignal"</Code>" of the demo, passed as "
                    <Code inline=true>"expanded_keys"</Code>" and "<Code inline=true>"set_expanded_keys"</Code>"."
                </p>
                <Demo description="An accordion of two disclosures, with a Disabled checkbox" source=include_str!("demos/disclosure.rs")>
                    <DisclosureAtomDemo/>
                </Demo>
            </Section>

            <Section title="Disclosure">
                <p>
                    "A "<Code inline=true>"<div>"</Code>" around the trigger and the panel. Put the trigger in a heading when the "
                    "disclosure is a section of the page."
                </p>
                <Section title="Props" id="disclosure-props">
                    <ApiTable kind=ApiKind::Props of="Disclosure">
                        <ApiRow name="id" ty="Option<Key>" default="generated">"Its key in a "<Code inline=true>"DisclosureGroup"</Code>"."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the button can\u{2019}t toggle the panel."</ApiRow>
                        <ApiRow name="default_expanded" ty="bool" default="false">
                            "Whether the panel starts expanded. Ignored with "<Code inline=true>"is_expanded"</Code>" or in a group."
                        </ApiRow>
                        <ApiRow name="is_expanded" ty="Option<Signal<bool>>" default="None">
                            "Whether the panel is expanded (controlled): a value or any signal."
                        </ApiRow>
                        <ApiRow name="set_expanded" ty="Option<Out<bool>>" default="None">
                            "Receives the expanded state: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>", closure, \u{2026}"
                        </ApiRow>
                        <ApiRow name="on_expanded_change" ty="Option<Callback<bool>>" default="None">"Called when the panel expands or collapses."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the disclosure element."</ApiRow>
                        <ApiRow name="children" ty="Children">"The trigger and the panel. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="DisclosureTrigger">
                <p>
                    "Wraps the disclosure\u{2019}s button (a "<Link href=routes::doc::button::Atom.materialize()>"Button"</Link>
                    " atom) and hands it the toggling. It renders no element; other buttons in the disclosure (e.g. a menu next to "
                    "the heading) stay unaffected."
                </p>
                <Section title="Props" id="disclosure-trigger-props">
                    <ApiTable kind=ApiKind::Props of="DisclosureTrigger">
                        <ApiRow name="children" ty="Children">"The trigger button. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="DisclosurePanel">
                <p>
                    "Shown while expanded and named by the trigger. Collapsed, it is "<Code inline=true>"hidden=\"until-found\""</Code>
                    ": find in page still finds (and expands) its content."
                </p>
                <Section title="Props" id="disclosure-panel-props">
                    <ApiTable kind=ApiKind::Props of="DisclosurePanel">
                        <ApiRow name="role" ty="DisclosurePanelRole" default="Group">
                            <Code inline=true>"Region"</Code>" makes the panel a landmark; use it sparingly."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the panel element."</ApiRow>
                        <ApiRow name="children" ty="Children">"The content. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="DisclosureGroup">
                <p>
                    "A "<Code inline=true>"<div>"</Code>" around "<Code inline=true>"Disclosure"</Code>"s: an accordion. Give "
                    "each disclosure an "<Code inline=true>"id"</Code>" to address it in the expanded keys."
                </p>
                <Section title="Props" id="disclosure-group-props">
                    <ApiTable kind=ApiKind::Props of="DisclosureGroup">
                        <ApiRow name="expansion" ty="DisclosureGroupExpansion" default="Single">
                            <Code inline=true>"Single"</Code>": expanding one collapses the others; "<Code inline=true>"Multiple"</Code>": any number."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether all disclosures are disabled."</ApiRow>
                        <ApiRow name="default_expanded_keys" ty="Vec<Key>" default="empty">
                            "The initially expanded disclosures (their "<Code inline=true>"id"</Code>"s). Ignored with "
                            <Code inline=true>"expanded_keys"</Code>"."
                        </ApiRow>
                        <ApiRow name="expanded_keys" ty="Option<Signal<HashSet<Key>>>" default="None">"The expanded disclosures (controlled): a value or any signal."</ApiRow>
                        <ApiRow name="set_expanded_keys" ty="Option<Out<HashSet<Key>>>" default="None">
                            "Receives the expanded disclosures: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>", closure, \u{2026}"
                        </ApiRow>
                        <ApiRow name="on_expanded_change" ty="Option<Callback<HashSet<Key>>>" default="None">"Called with the expanded disclosures."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the group element."</ApiRow>
                        <ApiRow name="children" ty="Children">"The disclosures. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-expanded" ty="true">"On a "<Code inline=true>"Disclosure"</Code>" while expanded."</ApiRow>
                    <ApiRow name="data-disabled" ty="true">"On a disabled "<Code inline=true>"Disclosure"</Code>" or "<Code inline=true>"DisclosureGroup"</Code>"."</ApiRow>
                    <ApiRow name="data-focus-visible-within" ty="true">"On a "<Code inline=true>"Disclosure"</Code>" or panel with keyboard focus inside."</ApiRow>
                </ApiTable>
                <p>
                    "The trigger has the "<Link href=format!("{}#data-attributes", routes::doc::button::Atom.materialize())>"Button atom\u{2019}s"</Link>
                    " data attributes ("<Code inline=true>"data-hovered"</Code>", "<Code inline=true>"data-pressed"</Code>", "
                    <Code inline=true>"data-focus-visible"</Code>", "<Code inline=true>"data-disabled"</Code>", \u{2026})."
                </p>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms bring no styles. They render the classes "<Code inline=true>"leptonic-Disclosure"</Code>", "
                    <Code inline=true>"leptonic-DisclosurePanel"</Code>" and "<Code inline=true>"leptonic-DisclosureGroup"</Code>
                    ", each followed by the "<Code inline=true>"classes"</Code>" you pass; the trigger is a "
                    <Code inline=true>"Button"</Code>" atom with its own class. The chevron is your own markup, hidden from "
                    "assistive technology: turn it through the disclosure\u{2019}s "<Code inline=true>"data-expanded"</Code>
                    ". Put the panel\u{2019}s padding on its content, as a panel hidden with "<Code inline=true>"until-found"</Code>
                    " keeps its own box. To animate, transition the panel\u{2019}s size with "
                    <Code inline=true>"--disclosure-panel-height"</Code>" (see "
                    <Link href=hook_section("hiding-the-panel")>"Hiding the Panel"</Link>"). The book\u{2019}s demos use "
                    "these rules:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .my-disclosure { border: 1px solid var(--border); border-radius: 8px; overflow: hidden; }
                        .my-trigger { display: flex; justify-content: space-between; width: 100%; padding: 1em; border: none; background: var(--surface); }
                        .my-trigger[data-hovered] { background: var(--border); }
                        .my-trigger[data-focus-visible] { outline: 2px solid var(--focus); outline-offset: -2px; }
                        .my-trigger[data-disabled] { color: var(--muted); cursor: not-allowed; }
                        .my-chevron { display: flex; transition: transform 150ms; }
                        .my-disclosure[data-expanded] .my-chevron { transform: rotate(180deg); }
                        .my-panel p { margin: 0; padding: 1em; }
                        @media (prefers-reduced-motion: reduce) { .my-chevron { transition: none; } }
                    ")}
                </Code>
                <p>
                    "Leptonic also ships an optional atom theme that styles the default classes, for apps that don\u{2019}t "
                    "want to start from scratch: "<Code inline=true>"@use \"leptonic/leptonic-atoms\";"</Code>"."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Disclosure.materialize()>"Disclosure overview"</Link></li>
                <li><Link href=routes::doc::disclosure::Hook.materialize()>"Disclosure Hooks"</Link></li>
                <li><Link href=routes::doc::button::Atom.materialize()>"Button Atom"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

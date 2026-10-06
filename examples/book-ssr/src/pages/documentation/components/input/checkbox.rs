use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    checkbox_basic::CheckboxBasicDemo, checkbox_disabled::CheckboxDisabledDemo,
    checkbox_group::CheckboxGroupDemo, checkbox_indeterminate::CheckboxIndeterminateDemo,
};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageCheckbox() -> impl IntoView {
    view! {
        <DocPage title="Checkbox Components">
            <p>
                "The themed "<Code inline=true>"Checkbox"</Code>" component, with its label as children, and the "
                <Code inline=true>"CheckboxGroup"</Code>" for a set of related checkboxes. See the "
                <Link href=routes::doc::Checkbox.materialize()>"Checkbox overview"</Link>" for concept guidance."
            </p>

            <Demo description="Newsletter checkbox controlled by a signal, showing its state" source=include_str!("demos/checkbox_basic.rs")>
                <CheckboxBasicDemo/>
            </Demo>

            <Section title="Checkbox">
                <p>"A checkbox with its label as children."</p>
                <Section title="Props" id="checkbox-props">
                    <ApiTable kind=ApiKind::Props of="components::checkbox::Checkbox">
                        <ApiRow name="default_selected" ty="bool" default="false">"Whether the checkbox starts checked."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<bool>>" default="None">
                            "Called when the checkbox is checked or unchecked, also while "<Code inline=true>"is_selected"</Code>
                            " controls the selection."
                        </ApiRow>
                        <ApiRow name="is_selected" ty="Option<Signal<bool>>" default="None">
                            "The selection (controlled): a value or any signal."
                        </ApiRow>
                        <ApiRow name="set_selected" ty="Option<Out<bool>>" default="None">
                            "Receives the new state: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="value" ty="Option<Key>" default="None">
                            "The checkbox\u{2019}s value in its "<Code inline=true>"CheckboxGroup"</Code>" (required there)."
                        </ApiRow>
                        <ApiRow name="is_indeterminate" ty="Signal<bool>" default="false">"Shows a dash instead of the check mark."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables the checkbox."</ApiRow>
                        <ApiRow name="is_read_only" ty="Signal<bool>" default="false">"Shows the state without allowing changes."</ApiRow>
                        <ApiRow name="is_required" ty="Signal<bool>" default="false">"Marks the checkbox as required."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">"Marks the checkbox invalid (a red border)."</ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">"The input\u{2019}s "<Code inline=true>"name"</Code>", for form submission."</ApiRow>
                        <ApiRow name="form_value" ty="Option<String>" default="None">
                            "The value submitted while checked. Without it, the browser submits "<Code inline=true>"on"</Code>"."
                        </ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"The accessible name, for a checkbox without children."</ApiRow>
                        <ApiRow name="checked_icon" ty="icondata::Icon" default="icondata::BsCheck2">"The check mark."</ApiRow>
                        <ApiRow name="indeterminate_icon" ty="icondata::Icon" default="icondata::BsDash">"The mark while indeterminate."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<label>"</Code>"."</ApiRow>
                        <ApiRow name="children" ty="Option<Children>" default="None">"The label text."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="CheckboxGroup">
                <p>
                    "A labelled group of checkboxes, each with a "<Code inline=true>"value"</Code>
                    ", selecting a set of values. See "<AnchorLink href="#groups">"Groups"</AnchorLink>"."
                </p>
                <Section title="Props" id="checkbox-group-props">
                    <ApiTable kind=ApiKind::Props of="components::checkbox::CheckboxGroup">
                        <ApiRow name="label" ty="Option<String>" default="None">"The visible label. Without it, set "<Code inline=true>"aria_label"</Code>"."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"The accessible name, for a group without visible label."</ApiRow>
                        <ApiRow name="description" ty="Option<String>" default="None">"Help text below the checkboxes, describing the group and each checkbox."</ApiRow>
                        <ApiRow name="default_value" ty="Vec<Key>" default="vec![]">"The initially checked values."</ApiRow>
                        <ApiRow name="value" ty="Option<Signal<Vec<Key>>>" default="None">
                            "The checked values (controlled): a value or any signal."
                        </ApiRow>
                        <ApiRow name="set_value" ty="Option<Out<Vec<Key>>>" default="None">
                            "Receives the new state: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Vec<Key>>>" default="None">"Called with the checked values when they change."</ApiRow>
                        <ApiRow name="orientation" ty="Orientation" default="Vertical">"Lays the checkboxes out in a column or a row."</ApiRow>
                        <ApiRow name="is_disabled, is_read_only" ty="Signal<bool>" default="false">"Disables the checkboxes, or prevents changes."</ApiRow>
                        <ApiRow name="is_required" ty="Signal<bool>" default="false">"At least one checkbox must be checked."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">"Marks the group invalid."</ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">"The checkboxes\u{2019} "<Code inline=true>"name"</Code>", for form submission."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the group element."</ApiRow>
                        <ApiRow name="children" ty="Children">"The checkboxes, each with a "<Code inline=true>"value"</Code>". Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="State">
                <p>
                    "A checkbox either keeps its own state, starting at "<Code inline=true>"default_selected"</Code>
                    " and reporting changes through "<Code inline=true>"on_change"</Code>", or shows your state, given as "
                    <Code inline=true>"is_selected"</Code>", and hands changes to "<Code inline=true>"set_selected"</Code>
                    ". The demo above passes one "<Code inline=true>"RwSignal"</Code>" as both: "
                    <Code inline=true>"is_selected=subscribed set_selected=subscribed"</Code>". "<Code inline=true>"is_selected"</Code>
                    " takes any signal, "<Code inline=true>"set_selected"</Code>" any setter: an "<Code inline=true>"RwSignal"</Code>
                    ", a "<Code inline=true>"WriteSignal"</Code>", a closure or a "<Code inline=true>"Callback"</Code>
                    ". The children are the label; pressing it toggles the checkbox too."
                </p>
            </Section>

            <Section title="Indeterminate">
                <p>
                    "A \u{201c}select all\u{201d} checkbox is checked when all items are, and indeterminate when some are. Its "
                    "state is derived from the items: "<Code inline=true>"is_selected"</Code>" is a derived signal, and "
                    <Code inline=true>"set_selected"</Code>" a closure updating every item:"
                </p>
                <Demo
                    description="Select-all checkbox that is indeterminate while some files are selected"
                    source=include_str!("demos/checkbox_indeterminate.rs")
                    source_open=true
                >
                    <CheckboxIndeterminateDemo/>
                </Demo>
            </Section>

            <Section title="Groups">
                <p>
                    "A "<Code inline=true>"CheckboxGroup"</Code>" holds the checked values of its checkboxes, as "
                    <Code inline=true>"Key"</Code>"s (strings or integers). Give each checkbox a "<Code inline=true>"value"</Code>
                    " instead of a selection:"
                </p>
                <Demo
                    description="Horizontal group of notification channel checkboxes with a label and a description"
                    source=include_str!("demos/checkbox_group.rs")
                >
                    <CheckboxGroupDemo/>
                </Demo>
            </Section>

            <Section title="Disabled and Read-Only">
                <p>
                    "A disabled checkbox can\u{2019}t be focused or changed. A read-only checkbox can be focused, and screen "
                    "readers announce it as read-only, but it can\u{2019}t be changed either. Both accept signals."
                </p>
                <Demo description="Checkbox with disabled and read-only toggles" source=include_str!("demos/checkbox_disabled.rs")>
                    <CheckboxDisabledDemo/>
                </Demo>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt the checkbox to your design:"</p>
                <CssVariables prefix="--checkbox-" scss=theme_scss!("checkbox")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Checkbox.materialize()>"Checkbox overview"</Link></li>
                <li><Link href=routes::doc::checkbox::Hook.materialize()>"Checkbox Hooks"</Link></li>
                <li><Link href=routes::doc::checkbox::Atom.materialize()>"Checkbox Atoms"</Link></li>
                <li><Link href=routes::doc::switch::Component.materialize()>"Switch Component"</Link></li>
                <li><Link href=routes::doc::radio::Component.materialize()>"Radio Components"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

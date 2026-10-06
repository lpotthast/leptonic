use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    radio_basic::RadioBasicDemo, radio_disabled::RadioDisabledDemo, radio_group::RadioGroupDemo,
    radio_labeled::RadioLabeledDemo,
};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageRadio() -> impl IntoView {
    view! {
        <DocPage title="Radio Components">
            <p>
                "The themed "<Code inline=true>"RadioGroup"</Code>" and "<Code inline=true>"Radio"</Code>
                " components: a group of radio buttons, of which one is selected. See the "<Link href=routes::doc::Radio.materialize()>"Radio overview"</Link>
                " for concept guidance."
            </p>

            <Demo description="Yes/no radio group reporting the selection" source=include_str!("demos/radio_basic.rs")>
                <RadioBasicDemo/>
            </Demo>

            <Section title="RadioGroup">
                <p>
                    "A group of radios with an optional label and description. It holds the selected value; see "
                    <AnchorLink href="#values">"Values"</AnchorLink>"."
                </p>
                <Section title="Props" id="radio-group-props">
                    <ApiTable kind=ApiKind::Props of="components::radio::RadioGroup">
                        <ApiRow name="label" ty="Option<String>" default="None">"The visible label. Without it, set "<Code inline=true>"aria_label"</Code>"."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"The accessible name, for a group without visible label."</ApiRow>
                        <ApiRow name="description" ty="Option<String>" default="None">"Help text below the radios, describing the group and each radio."</ApiRow>
                        <ApiRow name="default_value" ty="Option<Key>" default="None">
                            "The initially selected value, e.g. "<Code inline=true>"default_value=\"standard\""</Code>"."
                        </ApiRow>
                        <ApiRow name="value" ty="Option<Signal<Option<Key>>>" default="None">
                            "The selected value (controlled): a value or any signal."
                        </ApiRow>
                        <ApiRow name="set_value" ty="Option<Out<Option<Key>>>" default="None">
                            "Receives the new state: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Option<Key>>>" default="None">"Called with the selected value when it changes."</ApiRow>
                        <ApiRow name="orientation" ty="Orientation" default="Vertical">
                            "Lays the radios out in a column or a row, and sets "<Code inline=true>"aria-orientation"</Code>"."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables all radios."</ApiRow>
                        <ApiRow name="is_read_only" ty="Signal<bool>" default="false">"Shows the selection without allowing changes."</ApiRow>
                        <ApiRow name="is_required" ty="Signal<bool>" default="false">"Marks the group as required."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">"Marks the group invalid (red circles)."</ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">"The radios\u{2019} "<Code inline=true>"name"</Code>". Generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the group element."</ApiRow>
                        <ApiRow name="children" ty="Children">"The radios. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Radio">
                <p>"A radio of the enclosing "<Code inline=true>"RadioGroup"</Code>", with its label as children."</p>
                <Section title="Props" id="radio-props">
                    <ApiTable kind=ApiKind::Props of="components::radio::Radio">
                        <ApiRow name="value" ty="Key">"The value the radio selects, e.g. "<Code inline=true>"value=\"express\""</Code>". Required."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables this radio."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"The accessible name, for a radio without children."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<label>"</Code>"."</ApiRow>
                        <ApiRow name="children" ty="Option<Children>" default="None">"The label text."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Values">
                <p>
                    "The group keeps the selected value, a "<Code inline=true>"Key"</Code>" (a string or an integer). It starts at "
                    <Code inline=true>"default_value"</Code>" and reports changes through "<Code inline=true>"on_change"</Code>
                    ", or shows your state, given as "<Code inline=true>"value"</Code>", and hands changes to "
                    <Code inline=true>"set_value"</Code>"; the demos pass one "<Code inline=true>"RwSignal"</Code>
                    " as both. Each radio\u{2019}s children are its label; pressing the label selects it. The arrow keys move "
                    "the selection between the radios."
                </p>
            </Section>

            <Section title="Orientation">
                <p>"Lay the radios out in a row with "<Code inline=true>"Orientation::Horizontal"</Code>":"</p>
                <Demo description="Horizontal size radio group with a label" source=include_str!("demos/radio_group.rs")>
                    <RadioGroupDemo/>
                </Demo>
            </Section>

            <Section title="Description">
                <p>"A "<Code inline=true>"description"</Code>" explains the choice. Screen readers read it with each radio:"</p>
                <Demo description="Shipping radio group with a label, a description and the selected option" source=include_str!("demos/radio_labeled.rs")>
                    <RadioLabeledDemo/>
                </Demo>
            </Section>

            <Section title="Disabled and Read-Only">
                <p>
                    "Disable the whole group with "<Code inline=true>"is_disabled"</Code>" on the "<Code inline=true>"RadioGroup"</Code>
                    ", or single options with "<Code inline=true>"is_disabled"</Code>" on a "<Code inline=true>"Radio"</Code>
                    ". The arrow keys skip disabled radios. A read-only group can be focused, and screen readers announce it as "
                    "read-only, but its selection can\u{2019}t change."
                </p>
                <Demo
                    description="Plan radio group with a disabled option and checkboxes disabling the group or making it read-only"
                    source=include_str!("demos/radio_disabled.rs")
                >
                    <RadioDisabledDemo/>
                </Demo>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt the radio buttons to your design:"</p>
                <CssVariables prefix="--radio-" scss=theme_scss!("radio")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Radio.materialize()>"Radio overview"</Link></li>
                <li><Link href=routes::doc::radio::Hook.materialize()>"Radio Hooks"</Link></li>
                <li><Link href=routes::doc::radio::Atom.materialize()>"Radio Atoms"</Link></li>
                <li><Link href=routes::doc::checkbox::Component.materialize()>"Checkbox Components"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

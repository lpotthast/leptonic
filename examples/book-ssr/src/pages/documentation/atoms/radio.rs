use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::radio::RadioAtomDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomRadio() -> impl IntoView {
    view! {
        <DocPage title="Radio Atoms">
            <p>
                "The radio atoms render an unstyled radio group: "<Code inline=true>"RadioGroup"</Code>" holds the selected value, "
                "each "<Code inline=true>"Radio"</Code>" is a "<Code inline=true>"<label>"</Code>" around a visually hidden "
                <Code inline=true>"<input type=\"radio\">"</Code>" and your children. The "
                <Link href=routes::doc::atoms::Field.materialize()>"field atoms"</Link>" "<Code inline=true>"Label"</Code>", "
                <Code inline=true>"Description"</Code>" and "<Code inline=true>"FieldError"</Code>" complete the group. You "
                "style them through the data attributes they render. See the "
                <Link href=routes::doc::Radio.materialize()>"Radio overview"</Link>" for concept guidance."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Code inline=true>"RadioGroup"</Code>" calls "<Link href=routes::doc::radio::Hook.materialize()>
                    <Code inline=true>"use_radio_group_state"</Code>" and "<Code inline=true>"use_radio_group"</Code></Link>
                    ", "<Code inline=true>"Radio"</Code>" calls "<Code inline=true>"use_radio"</Code>" and "
                    <Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link>"."
                </p>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::{
                            field::Label,
                            radio::{Radio, RadioGroup},
                        };

                        view! {
                            <RadioGroup default_value="free" on_change=move |plan| set_plan.set(plan)>
                                <Label>"Plan"</Label>
                                <Radio value="free" classes="my-radio">
                                    <span class="my-radio-circle" aria-hidden="true"></span>
                                    "Free"
                                </Radio>
                                <Radio value="pro" classes="my-radio">
                                    <span class="my-radio-circle" aria-hidden="true"></span>
                                    "Pro"
                                </Radio>
                            </RadioGroup>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "A horizontal group: the radios sit in a "<Code inline=true>"<div>"</Code>" laid out as a row. Tab into the "
                    "group and use the arrow keys; the disabled Team plan is skipped."
                </p>
                <Demo
                    description="Horizontal plan radio group drawn with CSS, with a disabled radio and disabled and read-only toggles"
                    source=include_str!("demos/radio.rs")
                >
                    <RadioAtomDemo/>
                </Demo>
            </Section>

            <Section title="RadioGroup">
                <p>
                    "A "<Code inline=true>"<div role=\"radiogroup\">"</Code>" around its children, which may contain any markup "
                    "besides the radios and parts. Label it with a "<Code inline=true>"Label"</Code>" or "
                    <Code inline=true>"aria_label"</Code>", and add a "<Code inline=true>"Description"</Code>" and a "
                    <Code inline=true>"FieldError"</Code>" as needed (see "
                    <Link href=routes::doc::atoms::Field.materialize()>"Field Atoms"</Link>")."
                </p>
                <Section title="Props" id="radio-group-props">
                    <ApiTable kind=ApiKind::Props of="atoms::radio::RadioGroup">
                        <ApiRow name="default_value" ty="Option<Key>" default="None">"The initially selected value."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Option<Key>>>" default="None">"Called with the selected value when it changes."</ApiRow>
                        <ApiRow name="orientation" ty="Orientation" default="Vertical">
                            "Sets "<Code inline=true>"aria-orientation"</Code>" and "<Code inline=true>"data-orientation"</Code>
                            ". Lay the radios out to match."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables all radios."</ApiRow>
                        <ApiRow name="is_read_only" ty="Signal<bool>" default="false">"The radios can be focused, but the selection can\u{2019}t change."</ApiRow>
                        <ApiRow name="is_required" ty="Signal<bool>" default="false">"Marks the group as required."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">"Marks the group invalid."</ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<Option<Key>>>" default="None">"Validates the selected value."</ApiRow>
                        <ApiRow name="validation_behavior" ty="Option<ValidationBehavior>" default="None">
                            "When errors are shown. "<Code inline=true>"None"</Code>": the behavior of the surrounding "
                            <Link href=routes::doc::atoms::Form.materialize()>"Form"</Link>", else "<Code inline=true>"Native"</Code>"."
                        </ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">"The radios\u{2019} "<Code inline=true>"name"</Code>". Generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="form" ty="Option<String>" default="None">"The id of the form the radios belong to, when they aren\u{2019}t inside it."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The group element\u{2019}s id."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                            "Labels the group without a "<Code inline=true>"Label"</Code>"."
                        </ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"The id(s) of other elements labelling the group."</ApiRow>
                        <ApiRow name="aria_describedby" ty="Option<String>" default="None">"Further describing elements."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the group element."</ApiRow>
                        <ApiRow name="children" ty="Children">"The radios, the field atoms and any other content."</ApiRow>
                    </ApiTable>
                    <p>
                        "See "<Link href=format!("{}#use-radio-group-state", routes::doc::radio::Hook.materialize())>
                        "use_radio_group_state"</Link>" for how these settings behave."
                    </p>
                </Section>

                <Section title="Data Attributes" id="radio-group-data-attributes">
                    <ApiTable kind=ApiKind::DataAttributes>
                        <ApiRow name="data-orientation" ty="horizontal | vertical">"The group\u{2019}s orientation."</ApiRow>
                        <ApiRow name="data-disabled" ty="true">"The group is disabled."</ApiRow>
                        <ApiRow name="data-readonly" ty="true">"The group is read-only."</ApiRow>
                        <ApiRow name="data-required" ty="true">"The group is required."</ApiRow>
                        <ApiRow name="data-invalid" ty="true">"The group is invalid."</ApiRow>
                    </ApiTable>
                    <p>"Each attribute is "<Code inline=true>"true"</Code>" while its state applies, and absent otherwise."</p>
                </Section>
            </Section>

            <Section title="Radio">
                <p>
                    "A radio of the enclosing "<Code inline=true>"RadioGroup"</Code>": a "<Code inline=true>"<label>"</Code>
                    " around the visually hidden input and the children."
                </p>
                <Section title="Props" id="radio-props">
                    <ApiTable kind=ApiKind::Props of="atoms::radio::Radio">
                        <ApiRow name="value" ty="Key">"The value the radio selects, e.g. "<Code inline=true>"value=\"pro\""</Code>"."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables this radio. It is also disabled while the group is."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The input\u{2019}s id."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"The accessible name, for a radio without label text."</ApiRow>
                        <ApiRow name="aria_labelledby, aria_describedby" ty="Option<String>" default="None">"Further labelling or describing elements."</ApiRow>
                        <ApiRow name="auto_focus" ty="bool" default="false">"Focuses the radio when it mounts."</ApiRow>
                        <ApiRow name="on_focus_change" ty="Option<Callback<bool>>" default="None">"Called when the radio gains or loses focus."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<label>"</Code>"."</ApiRow>
                        <ApiRow name="children" ty="Option<Children>" default="None">"The circle and the label text."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Data Attributes" id="radio-data-attributes">
                    <p>"Set to "<Code inline=true>"true"</Code>" on the "<Code inline=true>"<label>"</Code>" while the state applies:"</p>
                    <ApiTable kind=ApiKind::DataAttributes>
                        <ApiRow name="data-selected" ty="true">"The radio is selected."</ApiRow>
                        <ApiRow name="data-pressed" ty="true">"The radio or its label is pressed."</ApiRow>
                        <ApiRow name="data-hovered" ty="true">"A mouse or pen is over the label."</ApiRow>
                        <ApiRow name="data-focused" ty="true">"The input has focus."</ApiRow>
                        <ApiRow name="data-focus-visible" ty="true">"The input has keyboard focus: show a focus ring."</ApiRow>
                        <ApiRow name="data-disabled" ty="true">"The radio or its group is disabled."</ApiRow>
                        <ApiRow name="data-readonly" ty="true">"The group is read-only."</ApiRow>
                        <ApiRow name="data-invalid" ty="true">"The group is invalid."</ApiRow>
                        <ApiRow name="data-required" ty="true">"The group is required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Styling">
                <p>
                    "Select the states with attribute selectors on your class; draw the focus ring with "
                    <Code inline=true>"data-focus-visible"</Code>", as the input itself is visually hidden:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .my-radio { display: inline-flex; align-items: center; gap: 0.5em; cursor: pointer; }
                        .my-radio-circle { width: 1em; height: 1em; border: 2px solid gray; border-radius: 50%; }
                        .my-radio[data-selected] .my-radio-circle { border: 0.3em solid royalblue; }
                        .my-radio[data-focus-visible] .my-radio-circle { outline: 2px solid royalblue; outline-offset: 2px; }
                        .my-radio[data-disabled] { opacity: 0.5; cursor: not-allowed; }
                    ")}
                </Code>
                <p>"The demo shows its complete styles under \u{201c}View styles\u{201d}."</p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Radio.materialize()>"Radio overview"</Link></li>
                <li><Link href=routes::doc::radio::Hook.materialize()>"Radio hooks"</Link></li>
                <li><Link href=routes::doc::radio::Component.materialize()>"Radio component"</Link></li>
                <li><Link href=routes::doc::checkbox::Atom.materialize()>"Checkbox atoms"</Link></li>
                <li><Link href=routes::doc::atoms::Field.materialize()>"Field atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

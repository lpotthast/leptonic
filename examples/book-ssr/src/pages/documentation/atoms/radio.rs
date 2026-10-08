use indoc::indoc;
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
                "each "<Code inline=true>"RadioField"</Code>" holds a radio, and its "<Code inline=true>"RadioButton"</Code>" is the "
                "clickable "<Code inline=true>"<label>"</Code>" around a visually hidden "
                <Code inline=true>"<input type=\"radio\">"</Code>" and your children. The "
                <Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link>" "<Code inline=true>"Label"</Code>", "
                <Code inline=true>"Description"</Code>" and "<Code inline=true>"FieldError"</Code>" complete the group. You "
                "style them through the data attributes they render. See the "
                <Link href=routes::doc::Radio.materialize()>"Radio overview"</Link>" for concept guidance."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Code inline=true>"RadioGroup"</Code>" calls "
                    <Link href=format!("{}#use-radio-group-state", routes::doc::radio::Hook.materialize())>"use_radio_group_state"</Link>
                    " and "<Link href=format!("{}#use-radio-group", routes::doc::radio::Hook.materialize())>"use_radio_group"</Link>
                    ", "<Code inline=true>"RadioField"</Code>" calls "
                    <Link href=format!("{}#use-radio", routes::doc::radio::Hook.materialize())>"use_radio"</Link>" and "
                    <Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link>"."
                </p>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            atoms::{
                                field::Label,
                                radio::{RadioButton, RadioField, RadioGroup},
                            },
                            hooks::Key,
                        };
                        use leptos::prelude::*;

                        let plan = RwSignal::new(Some(Key::from("free")));

                        view! {
                            <RadioGroup value=plan set_value=plan>
                                <Label>"Plan"</Label>
                                <RadioField value="free">
                                    <RadioButton classes="my-radio">
                                        <span class="my-radio-circle" aria-hidden="true"></span>
                                        "Free"
                                    </RadioButton>
                                </RadioField>
                                <RadioField value="pro">
                                    <RadioButton classes="my-radio">
                                        <span class="my-radio-circle" aria-hidden="true"></span>
                                        "Pro"
                                    </RadioButton>
                                </RadioField>
                            </RadioGroup>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "A horizontal group: the radios sit in a "<Code inline=true>"<div>"</Code>" laid out as a row. Tab into the "
                    "group and use the arrow keys; they skip the Team plan while it is disabled. The group\u{2019}s value is "
                    "an enum, "<Code inline=true>"Option<Plan>"</Code>", made a selection value with "
                    <Code inline=true>"selection_value!"</Code>"."
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
                    <Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link>")."
                </p>
                <p>
                    "The group is generic over its value type "<Code inline=true>"V"</Code>": a "<Code inline=true>"Key"</Code>
                    ", a "<Code inline=true>"String"</Code>", an integer or your enum (see "<Link href=format!("{}#selectionvalue", routes::doc::CollectionState.materialize())>"SelectionValue"</Link>"). The radios\u{2019} "
                    <Code inline=true>"value"</Code>"s convert into keys, so an enum value works there too."
                </p>
                <Section title="Props" id="radio-group-props">
                    <ApiTable kind=ApiKind::Props of="atoms::radio::RadioGroup">
                        <ApiRow name="default_value" ty="Option<V>" default="None">"The initially selected value."</ApiRow>
                        <ApiRow name="value" ty="Option<Signal<Option<V>>>" default="None">
                            "The selected value (controlled): a value or any signal."
                        </ApiRow>
                        <ApiRow name="set_value" ty="Option<Out<Option<V>>>" default="None">
                            "Receives the new state: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Option<V>>>" default="None">"Called with the selected value when it changes."</ApiRow>
                        <ApiRow name="orientation" ty="Orientation" default="Vertical">
                            "Sets "<Code inline=true>"aria-orientation"</Code>" and "<Code inline=true>"data-orientation"</Code>
                            ". Lay the radios out to match."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables all radios."</ApiRow>
                        <ApiRow name="is_read_only" ty="Signal<bool>" default="false">"The radios can be focused, but the selection can\u{2019}t change."</ApiRow>
                        <ApiRow name="is_required" ty="Signal<bool>" default="false">"Marks the group as required."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">"Marks the group invalid."</ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<Option<V>>>" default="None">"Validates the selected value."</ApiRow>
                        <ApiRow name="validation_behavior" ty="Option<ValidationBehavior>" default="None">
                            "When errors are shown. "<Code inline=true>"None"</Code>": the behavior of the surrounding "
                            <Link href=routes::doc::Form.materialize()>"Form"</Link>", else "<Code inline=true>"Native"</Code>"."
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
                        <ApiRow name="children" ty="Children">"The radios, the field atoms and any other content. Required."</ApiRow>
                    </ApiTable>
                    <p>
                        "See "<Link href=format!("{}#use-radio-group-state", routes::doc::radio::Hook.materialize())>
                        "use_radio_group_state"</Link>" for how these settings behave."
                    </p>
                </Section>
            </Section>

            <Section title="RadioField">
                <p>
                    "A radio of the enclosing "<Code inline=true>"RadioGroup"</Code>": a "<Code inline=true>"<div>"</Code>" around a "
                    <AnchorLink href="#radiobutton">"RadioButton"</AnchorLink>" and, as needed, a "<Code inline=true>"Description"</Code>
                    " of its own; a "<Code inline=true>"FieldError"</Code>" in it shows the group\u{2019}s errors. There is no single "
                    <Code inline=true>"Radio"</Code>" atom: a radio is always a field and its button."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        <RadioGroup value=plan set_value=plan>
                            <Label>"Plan"</Label>
                            <RadioField value="pro" classes="my-radio-field">
                                <RadioButton classes="my-radio">
                                    <span class="my-radio-circle" aria-hidden="true"></span>
                                    "Pro"
                                </RadioButton>
                                <Description>"Unlimited projects, billed monthly."</Description>
                            </RadioField>
                        </RadioGroup>
                    "#)}
                </Code>
                <Section title="Props" id="radiofield-props">
                    <ApiTable kind=ApiKind::Props of="RadioField">
                        <ApiRow name="value" ty="Key">"The value the radio selects, e.g. "<Code inline=true>"value=\"pro\""</Code>". Required."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables this radio. It is also disabled while the group is."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The input\u{2019}s id."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"The accessible name, for a radio without label text."</ApiRow>
                        <ApiRow name="aria_labelledby, aria_describedby" ty="Option<String>" default="None">"Further labelling or describing elements."</ApiRow>
                        <ApiRow name="auto_focus" ty="bool" default="false">"Focuses the radio when it mounts."</ApiRow>
                        <ApiRow name="on_focus_change" ty="Option<Callback<bool>>" default="None">"Called when the radio gains or loses focus."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<div>"</Code>"."</ApiRow>
                        <ApiRow name="children" ty="Children">"The "<Code inline=true>"RadioButton"</Code>", and a "<Code inline=true>"Description"</Code>" and a "<Code inline=true>"FieldError"</Code>" as needed. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="RadioButton">
                <p>
                    "The clickable "<Code inline=true>"<label>"</Code>" of a "<AnchorLink href="#radiofield">"RadioField"</AnchorLink>
                    ", around a visually hidden "<Code inline=true>"<input type=\"radio\">"</Code>" and your children (the circle and the label text). "
                    "It takes the field\u{2019}s state and renders the data attributes below."
                </p>
                <Section title="Props" id="radiobutton-props">
                    <ApiTable kind=ApiKind::Props of="RadioButton">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<label>"</Code>"."</ApiRow>
                        <ApiRow name="children" ty="Option<Children>" default="None">"The circle and the label text."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <p>
                    "Each attribute is "<Code inline=true>"true"</Code>" while its state applies, and absent otherwise. "
                    <Code inline=true>"RadioGroup"</Code>" sets them on its group element:"
                </p>
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-orientation" ty="horizontal | vertical">"The group\u{2019}s orientation (always set)."</ApiRow>
                    <ApiRow name="data-disabled" ty="true">"The group is disabled."</ApiRow>
                    <ApiRow name="data-readonly" ty="true">"The group is read-only."</ApiRow>
                    <ApiRow name="data-required" ty="true">"The group is required."</ApiRow>
                    <ApiRow name="data-invalid" ty="true">"The group is invalid."</ApiRow>
                </ApiTable>
                <p>
                    <Code inline=true>"RadioButton"</Code>" sets them on its "<Code inline=true>"<label>"</Code>"; "
                    <Code inline=true>"RadioField"</Code>" sets those of the radio\u{2019}s state (all but "
                    <Code inline=true>"data-pressed"</Code>", "<Code inline=true>"data-hovered"</Code>" and the focus attributes) "
                    "on its "<Code inline=true>"<div>"</Code>":"
                </p>
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

            <Section title="Styling">
                <p>
                    "The atoms bring no styles. "<Code inline=true>"RadioGroup"</Code>" renders a "<Code inline=true>"<div>"</Code>
                    " (default class "<Code inline=true>"leptonic-RadioGroup"</Code>") around its children: a "
                    <Code inline=true>"Label"</Code>", the radios, a "<Code inline=true>"Description"</Code>" and a "
                    <Code inline=true>"FieldError"</Code>" ("<Link href=format!("{}#styling", routes::doc::field::Atom.materialize())>"Field Atoms"</Link>
                    "). Each "<Code inline=true>"RadioField"</Code>" renders a "<Code inline=true>"<div>"</Code>" (default class "
                    <Code inline=true>"leptonic-RadioField"</Code>"), its "<Code inline=true>"RadioButton"</Code>" a "
                    <Code inline=true>"<label>"</Code>" (default class "<Code inline=true>"leptonic-RadioButton"</Code>") around a "
                    "visually hidden "<Code inline=true>"<input>"</Code>", followed by your children: draw the circle as the "
                    "button\u{2019}s first child, hidden from assistive technology, and put the label text after it:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        <RadioField value="standard">
                            <RadioButton classes="demo-radio">
                                <span class="demo-radio-circle" aria-hidden="true"></span>
                                "Standard shipping"
                            </RadioButton>
                        </RadioField>
                    "#)}
                </Code>
                <p>
                    "Style the circle through the data attributes of the button, and draw the focus ring around it with "
                    <Code inline=true>"data-focus-visible"</Code>", as the input itself is invisible. The demos above use this CSS:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r#"
                        .demo-radio { display: inline-flex; align-items: center; gap: 0.5rem; cursor: pointer; }
                        .demo-radio[data-disabled] { opacity: 0.5; cursor: not-allowed; }

                        .demo-radio-circle {
                            display: inline-flex;
                            align-items: center;
                            justify-content: center;
                            width: 1.1em;
                            height: 1.1em;
                            border: 2px solid var(--muted);
                            border-radius: 50%;
                        }
                        .demo-radio-circle::after { content: ""; width: 0.55em; height: 0.55em; border-radius: 50%; }
                        .demo-radio[data-hovered] .demo-radio-circle,
                        .demo-radio[data-selected] .demo-radio-circle { border-color: var(--accent); }
                        .demo-radio[data-selected] .demo-radio-circle::after { background: var(--accent); }
                        .demo-radio[data-focus-visible] .demo-radio-circle { outline: 2px solid var(--focus); outline-offset: 2px; }

                        .demo-choice-group-items { display: flex; flex-direction: column; gap: 0.5rem; }
                        [data-orientation="horizontal"] > .demo-choice-group-items { flex-direction: row; gap: 1rem; }
                    "#)}
                </Code>
                <p>
                    "Apps that don\u{2019}t want to style from scratch can load leptonic\u{2019}s optional atom theme, "
                    <Code inline=true>"@use \"leptonic/leptonic-atoms\";"</Code>", which styles the default classes."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Radio.materialize()>"Radio overview"</Link></li>
                <li><Link href=routes::doc::radio::Hook.materialize()>"Radio Hooks"</Link></li>
                <li><Link href=routes::doc::checkbox::Atom.materialize()>"Checkbox Atoms"</Link></li>
                <li><Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

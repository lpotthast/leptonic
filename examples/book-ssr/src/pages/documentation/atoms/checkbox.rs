use indoc::indoc;
use leptos::prelude::*;

use super::demos::{checkbox::CheckboxAtomDemo, checkbox_group::CheckboxGroupAtomDemo};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomCheckbox() -> impl IntoView {
    view! {
        <DocPage title="Checkbox Atoms">
            <p>
                "The checkbox atoms render unstyled checkboxes: a "<Code inline=true>"CheckboxField"</Code>" holds a "
                "checkbox\u{2019}s state, and its "<Code inline=true>"CheckboxButton"</Code>" is the clickable "
                <Code inline=true>"<label>"</Code>" around a visually hidden "<Code inline=true>"<input type=\"checkbox\">"</Code>
                " and your children, which draw the box and the label text. "<Code inline=true>"CheckboxGroup"</Code>
                " groups checkboxes selecting a set of values; the "
                <Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link>" label and describe it. You style "
                "them through the data attributes they render. See the "<Link href=routes::doc::Checkbox.materialize()>"Checkbox overview"</Link>" for concept guidance."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Code inline=true>"CheckboxField"</Code>" calls "
                    <Link href=format!("{}#use-toggle-state", routes::doc::checkbox::Hook.materialize())>"use_toggle_state"</Link>" and "
                    <Link href=format!("{}#use-checkbox", routes::doc::checkbox::Hook.materialize())>"use_checkbox"</Link>
                    " (inside a group: "
                    <Link href=format!("{}#use-checkbox-group-item", routes::doc::checkbox::Hook.materialize())>"use_checkbox_group_item"</Link>
                    ") and "<Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link>". "
                    <Code inline=true>"CheckboxGroup"</Code>" calls "
                    <Link href=format!("{}#use-checkbox-group-state", routes::doc::checkbox::Hook.materialize())>"use_checkbox_group_state"</Link>
                    " and "<Link href=format!("{}#use-checkbox-group", routes::doc::checkbox::Hook.materialize())>"use_checkbox_group"</Link>"."
                </p>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::checkbox::{CheckboxButton, CheckboxField};
                        use leptos::prelude::*;

                        let subscribed = RwSignal::new(false);

                        view! {
                            <CheckboxField is_selected=subscribed set_selected=subscribed>
                                <CheckboxButton classes="my-checkbox">
                                    <span class="my-checkbox-box" aria-hidden="true"></span>
                                    "Subscribe to the newsletter"
                                </CheckboxButton>
                            </CheckboxField>
                        }
                    "#)}
                </Code>
                <p>
                    "Without "<Code inline=true>"is_selected"</Code>", the checkbox keeps its own selection: it starts at "
                    <Code inline=true>"default_selected"</Code>" and reports changes through "<Code inline=true>"on_change"</Code>"."
                </p>
            </Section>

            <Section title="Demo">
                <p>"The box is a "<Code inline=true>"<span>"</Code>" styled through the data attributes of the "<Code inline=true>"CheckboxButton"</Code>":"</p>
                <Demo
                    description="Newsletter checkbox drawn with CSS, with indeterminate, disabled and read-only toggles"
                    source=include_str!("demos/checkbox.rs")
                >
                    <CheckboxAtomDemo/>
                </Demo>
                <p>
                    "A "<Code inline=true>"CheckboxGroup"</Code>" with a "<Code inline=true>"Label"</Code>", a "
                    <Code inline=true>"Description"</Code>" and a "<Code inline=true>"FieldError"</Code>
                    ". Uncheck all days to see the error message."
                </p>
                <Demo
                    description="Required group of weekday checkboxes with a label, a description, an error message and a disabled toggle"
                    source=include_str!("demos/checkbox_group.rs")
                >
                    <CheckboxGroupAtomDemo/>
                </Demo>
            </Section>

            <Section title="CheckboxField">
                <p>
                    "A checkbox: a "<Code inline=true>"<div>"</Code>" holding its state, around a "
                    <AnchorLink href="#checkboxbutton">"CheckboxButton"</AnchorLink>" and, as needed, a "
                    <Code inline=true>"Description"</Code>" and a "<Code inline=true>"FieldError"</Code>" of its own. In a "
                    <Code inline=true>"CheckboxGroup"</Code>" the group validates, so a "<Code inline=true>"FieldError"</Code>
                    " in it shows nothing. There is no single "<Code inline=true>"Checkbox"</Code>" atom: a checkbox is always "
                    "a field and its button."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        <CheckboxField is_selected=accepted set_selected=accepted is_required=true classes="my-checkbox-field">
                            <CheckboxButton classes="my-checkbox">
                                <span class="my-checkbox-box" aria-hidden="true"></span>
                                "I accept the terms"
                            </CheckboxButton>
                            <Description>"You can withdraw your consent at any time."</Description>
                            <FieldError/>
                        </CheckboxField>
                    "#)}
                </Code>
                <Section title="Props" id="checkboxfield-props">
                    <ApiTable kind=ApiKind::Props of="CheckboxField">
                        <ApiRow name="value" ty="Option<Key>" default="None">
                            "The checkbox\u{2019}s value in its "<Code inline=true>"CheckboxGroup"</Code>
                            " (required there). The group then holds the selection: "<Code inline=true>"default_selected"</Code>
                            ", "<Code inline=true>"is_selected"</Code>" and "<Code inline=true>"set_selected"</Code>" don\u{2019}t apply."
                        </ApiRow>
                        <ApiRow name="default_selected" ty="bool" default="false">"Whether the checkbox starts checked."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<bool>>" default="None">
                            "Called when the checkbox is checked or unchecked, also while "<Code inline=true>"is_selected"</Code>
                            " controls the selection."
                        </ApiRow>
                        <ApiRow name="is_selected" ty="Option<Signal<bool>>" default="None">
                            "Whether the toggle is selected (controlled): a value or any signal."
                        </ApiRow>
                        <ApiRow name="set_selected" ty="Option<Out<bool>>" default="None">
                            "Receives the selection: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="is_indeterminate" ty="Signal<bool>" default="false">
                            "Shows the checkbox as partially checked, whatever its selection."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables the checkbox."</ApiRow>
                        <ApiRow name="is_read_only" ty="Signal<bool>" default="false">"The checkbox can be focused but not changed."</ApiRow>
                        <ApiRow name="is_required" ty="Signal<bool>" default="false">"Marks the checkbox as required."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">"Marks the checkbox invalid."</ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<bool>>" default="None">"Validates the selection."</ApiRow>
                        <ApiRow name="validation_behavior" ty="Option<ValidationBehavior>" default="None">
                            "When errors are shown. "<Code inline=true>"None"</Code>": the behavior of the surrounding "
                            <Link href=routes::doc::Form.materialize()>"Form"</Link>", else "<Code inline=true>"Native"</Code>"."
                        </ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">"The input\u{2019}s "<Code inline=true>"name"</Code>" (in a group: the group\u{2019}s)."</ApiRow>
                        <ApiRow name="form_value" ty="Option<String>" default="None">
                            "The value submitted while checked (in a group: "<Code inline=true>"value"</Code>")."
                        </ApiRow>
                        <ApiRow name="form" ty="Option<String>" default="None">"The id of the form the input belongs to, when it isn\u{2019}t inside it."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The input\u{2019}s id."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"The accessible name, for a checkbox without label text."</ApiRow>
                        <ApiRow name="aria_labelledby, aria_describedby" ty="Option<String>" default="None">"Further labelling or describing elements."</ApiRow>
                        <ApiRow name="auto_focus" ty="bool" default="false">"Focuses the checkbox when it mounts."</ApiRow>
                        <ApiRow name="on_focus_change" ty="Option<Callback<bool>>" default="None">"Called when the checkbox gains or loses focus."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<div>"</Code>"."</ApiRow>
                        <ApiRow name="children" ty="Children">"The "<Code inline=true>"CheckboxButton"</Code>", and a "<Code inline=true>"Description"</Code>" and a "<Code inline=true>"FieldError"</Code>" as needed. Required."</ApiRow>
                    </ApiTable>
                    <p>
                        "See "<Link href=format!("{}#toggleoptions", routes::doc::checkbox::Hook.materialize())>"ToggleOptions"</Link>
                        " for how these settings behave."
                    </p>
                </Section>
            </Section>

            <Section title="CheckboxButton">
                <p>
                    "The clickable "<Code inline=true>"<label>"</Code>" of a "<AnchorLink href="#checkboxfield">"CheckboxField"</AnchorLink>
                    ", around a visually hidden "<Code inline=true>"<input type=\"checkbox\">"</Code>" and your children (the box and the label text). "
                    "It takes the field\u{2019}s state and renders the data attributes below."
                </p>
                <Section title="Props" id="checkboxbutton-props">
                    <ApiTable kind=ApiKind::Props of="CheckboxButton">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<label>"</Code>"."</ApiRow>
                        <ApiRow name="children" ty="Option<Children>" default="None">"The box and the label text."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="CheckboxGroup">
                <p>
                    "A "<Code inline=true>"<div role=\"group\">"</Code>" selecting a set of values. Its "<Code inline=true>"CheckboxField"</Code>
                    "s need a "<Code inline=true>"value"</Code>". Label it with a "<Code inline=true>"Label"</Code>
                    " (or "<Code inline=true>"aria_label"</Code>"), and add a "<Code inline=true>"Description"</Code>
                    " and a "<Code inline=true>"FieldError"</Code>" as needed; they describe the group and each checkbox. See "
                    <Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link>"."
                </p>
                <p>
                    "It is generic over the type "<Code inline=true>"V"</Code>" of its values: a "<Code inline=true>"Key"</Code>
                    ", a "<Code inline=true>"String"</Code>", an integer or your enum (see "<Link href=format!("{}#selectionvalue", routes::doc::CollectionState.materialize())>"SelectionValue"</Link>")."
                </p>
                <Section title="Props" id="checkbox-group-props">
                    <ApiTable kind=ApiKind::Props of="atoms::checkbox::CheckboxGroup">
                        <ApiRow name="default_value" ty="Vec<V>" default="vec![]">"The initially checked values."</ApiRow>
                        <ApiRow name="value" ty="Option<Signal<Vec<V>>>" default="None">
                            "The checked values (controlled): a value or any signal."
                        </ApiRow>
                        <ApiRow name="set_value" ty="Option<Out<Vec<V>>>" default="None">
                            "Receives the new state: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Vec<V>>>" default="None">"Called with the checked values when they change."</ApiRow>
                        <ApiRow name="is_disabled, is_read_only" ty="Signal<bool>" default="false">"Disables the checkboxes, or prevents changes."</ApiRow>
                        <ApiRow name="is_required" ty="Signal<bool>" default="false">"At least one checkbox must be checked."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">"Marks the group invalid."</ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<Vec<V>>>" default="None">"Validates the checked values."</ApiRow>
                        <ApiRow name="validation_behavior" ty="Option<ValidationBehavior>" default="None">
                            "When errors are shown. "<Code inline=true>"None"</Code>": the behavior of the surrounding "
                            <Link href=routes::doc::Form.materialize()>"Form"</Link>", else "<Code inline=true>"Native"</Code>"."
                        </ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">"The checkboxes\u{2019} "<Code inline=true>"name"</Code>"."</ApiRow>
                        <ApiRow name="form" ty="Option<String>" default="None">"The id of the form the checkboxes belong to, when they aren\u{2019}t inside it."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The group element\u{2019}s id."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                            "Labels the group without a "<Code inline=true>"Label"</Code>"."
                        </ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"The id(s) of other elements labelling the group."</ApiRow>
                        <ApiRow name="aria_describedby" ty="Option<String>" default="None">"Further describing elements."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the group element."</ApiRow>
                        <ApiRow name="children" ty="Children">"The checkboxes, the field atoms and any other content. Required."</ApiRow>
                    </ApiTable>
                    <p>
                        "See "<Link href=format!("{}#use-checkbox-group-state", routes::doc::checkbox::Hook.materialize())>
                        "use_checkbox_group_state"</Link>" for how these settings behave."
                    </p>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <p>
                    "Each attribute is "<Code inline=true>"true"</Code>" while its state applies, and absent otherwise. "
                    <Code inline=true>"CheckboxButton"</Code>" sets them on its "<Code inline=true>"<label>"</Code>"; "
                    <Code inline=true>"CheckboxField"</Code>" sets those of the checkbox\u{2019}s state (all but "
                    <Code inline=true>"data-pressed"</Code>", "<Code inline=true>"data-hovered"</Code>" and the focus attributes) "
                    "on its "<Code inline=true>"<div>"</Code>":"
                </p>
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-selected" ty="true">"The checkbox is checked."</ApiRow>
                    <ApiRow name="data-indeterminate" ty="true">"The checkbox is indeterminate."</ApiRow>
                    <ApiRow name="data-pressed" ty="true">"The checkbox or its label is pressed."</ApiRow>
                    <ApiRow name="data-hovered" ty="true">"A mouse or pen is over the label."</ApiRow>
                    <ApiRow name="data-focused" ty="true">"The input has focus."</ApiRow>
                    <ApiRow name="data-focus-visible" ty="true">"The input has keyboard focus: show a focus ring."</ApiRow>
                    <ApiRow name="data-disabled" ty="true">"The checkbox (or its group) is disabled."</ApiRow>
                    <ApiRow name="data-readonly" ty="true">"The checkbox (or its group) is read-only."</ApiRow>
                    <ApiRow name="data-invalid" ty="true">"The checkbox (or its group) is invalid."</ApiRow>
                    <ApiRow name="data-required" ty="true">"The checkbox\u{2019}s own "<Code inline=true>"is_required"</Code>" is set."</ApiRow>
                </ApiTable>
                <p><Code inline=true>"CheckboxGroup"</Code>" sets them on its group element:"</p>
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-disabled" ty="true">"The group is disabled."</ApiRow>
                    <ApiRow name="data-readonly" ty="true">"The group is read-only."</ApiRow>
                    <ApiRow name="data-required" ty="true">"The group is required."</ApiRow>
                    <ApiRow name="data-invalid" ty="true">"The group is invalid."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms bring no styles. "<Code inline=true>"CheckboxField"</Code>" renders a "<Code inline=true>"<div>"</Code>
                    " (default class "<Code inline=true>"leptonic-CheckboxField"</Code>"), "<Code inline=true>"CheckboxButton"</Code>" a "
                    <Code inline=true>"<label>"</Code>" (default class "<Code inline=true>"leptonic-CheckboxButton"</Code>") around a "
                    "visually hidden "<Code inline=true>"<input>"</Code>", followed by your children. Draw the box yourself as the "
                    "button\u{2019}s first child, hidden from assistive technology, and put the label text after it:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        <CheckboxField is_selected=subscribed set_selected=subscribed>
                            <CheckboxButton classes="demo-check">
                                <span class="demo-check-box" aria-hidden="true"></span>
                                "Subscribe to the newsletter"
                            </CheckboxButton>
                        </CheckboxField>
                    "#)}
                </Code>
                <p>
                    "Style the box through the data attributes of the button. The input has the focus, but is invisible, so "
                    "draw the focus ring around the box with "<Code inline=true>"data-focus-visible"</Code>". The demos above "
                    "use this CSS:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r#"
                        .demo-check { display: inline-flex; align-items: center; gap: 0.5rem; cursor: pointer; }
                        .demo-check[data-disabled] { opacity: 0.5; cursor: not-allowed; }

                        .demo-check-box {
                            display: inline-flex;
                            align-items: center;
                            justify-content: center;
                            width: 1.1em;
                            height: 1.1em;
                            border: 2px solid var(--muted);
                            border-radius: 4px;
                            color: var(--surface);
                        }
                        .demo-check[data-hovered] .demo-check-box { border-color: var(--accent); }
                        .demo-check[data-selected] .demo-check-box,
                        .demo-check[data-indeterminate] .demo-check-box { background: var(--accent); border-color: var(--accent); }
                        .demo-check[data-selected] .demo-check-box::after { content: "\2713"; }
                        .demo-check[data-indeterminate] .demo-check-box::after { content: "\2013"; }
                        .demo-check[data-focus-visible] .demo-check-box { outline: 2px solid var(--focus); outline-offset: 2px; }
                    "#)}
                </Code>
                <p>
                    <Code inline=true>"CheckboxGroup"</Code>" renders a "<Code inline=true>"<div>"</Code>" (default class "
                    <Code inline=true>"leptonic-CheckboxGroup"</Code>") around its children: a "<Code inline=true>"Label"</Code>
                    ", the checkboxes, a "<Code inline=true>"Description"</Code>" and a "<Code inline=true>"FieldError"</Code>
                    ", styled like any "<Link href=format!("{}#styling", routes::doc::field::Atom.materialize())>"field part"</Link>"."
                </p>
                <p>
                    "Apps that don\u{2019}t want to style from scratch can load leptonic\u{2019}s optional atom theme, "
                    <Code inline=true>"@use \"leptonic/leptonic-atoms\";"</Code>", which styles the default classes."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Checkbox.materialize()>"Checkbox overview"</Link></li>
                <li><Link href=routes::doc::checkbox::Hook.materialize()>"Checkbox Hooks"</Link></li>
                <li><Link href=routes::doc::switch::Atom.materialize()>"Switch Atoms"</Link></li>
                <li><Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{checkbox::CheckboxAtomDemo, checkbox_group::CheckboxGroupAtomDemo};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomCheckbox() -> impl IntoView {
    view! {
        <DocPage title="Checkbox Atoms">
            <p>
                "The checkbox atoms render unstyled checkboxes: "<Code inline=true>"Checkbox"</Code>" is a "
                <Code inline=true>"<label>"</Code>" around a visually hidden "<Code inline=true>"<input type=\"checkbox\">"</Code>
                " and your children, which draw the box and the label text. "<Code inline=true>"CheckboxGroup"</Code>
                " groups checkboxes selecting a set of values; the "
                <Link href=routes::doc::atoms::Field.materialize()>"field atoms"</Link>" label and describe it. You style "
                "them through the data attributes they render. See the "<Link href=routes::doc::Checkbox.materialize()>"Checkbox overview"</Link>" for concept guidance."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Code inline=true>"Checkbox"</Code>" calls "<Link href=routes::doc::checkbox::Hook.materialize()>
                    <Code inline=true>"use_toggle_state"</Code>" and "<Code inline=true>"use_checkbox"</Code></Link>
                    " (inside a group: "<Code inline=true>"use_checkbox_group_item"</Code>") and "
                    <Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link>". "
                    <Code inline=true>"CheckboxGroup"</Code>" calls "<Code inline=true>"use_checkbox_group_state"</Code>" and "
                    <Code inline=true>"use_checkbox_group"</Code>"."
                </p>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::checkbox::Checkbox;

                        let subscribed = RwSignal::new(false);

                        view! {
                            <Checkbox state=subscribed classes="my-checkbox">
                                <span class="my-checkbox-box" aria-hidden="true"></span>
                                "Subscribe to the newsletter"
                            </Checkbox>
                        }
                    "#)}
                </Code>
                <p>
                    "Without "<Code inline=true>"state"</Code>", the checkbox keeps its own selection: it starts at "
                    <Code inline=true>"default_selected"</Code>" and reports changes through "<Code inline=true>"on_change"</Code>"."
                </p>
            </Section>

            <Section title="Demo">
                <p>"The box is a "<Code inline=true>"<span>"</Code>" styled through the label\u{2019}s data attributes:"</p>
                <Demo
                    description="Newsletter checkbox drawn with CSS, with indeterminate, disabled and read-only toggles"
                    source=include_str!("demos/checkbox.rs")
                >
                    <CheckboxAtomDemo/>
                </Demo>
            </Section>

            <Section title="Checkbox">
                <Section title="Props" id="checkbox-props">
                    <ApiTable kind=ApiKind::Props of="atoms::checkbox::Checkbox">
                        <ApiRow name="value" ty="Option<Key>" default="None">
                            "The checkbox\u{2019}s value in its "<Code inline=true>"CheckboxGroup"</Code>
                            " (required there). The group then holds the selection: "<Code inline=true>"default_selected"</Code>
                            " and "<Code inline=true>"state"</Code>" don\u{2019}t apply."
                        </ApiRow>
                        <ApiRow name="default_selected" ty="bool" default="false">"Whether the checkbox starts checked."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<bool>>" default="None">
                            "Called when the checkbox is checked or unchecked. Also called when "<Code inline=true>"state"</Code>
                            " is given."
                        </ApiRow>
                        <ApiRow name="state" ty="Option<ToggleState>" default="None">
                            "Binds the selection to your state, e.g. "<Code inline=true>"state=rw_signal"</Code>" or "
                            <Code inline=true>"state=(read, write)"</Code>". Replaces "<Code inline=true>"default_selected"</Code>"."
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
                            <Link href=routes::doc::atoms::Form.materialize()>"Form"</Link>", else "<Code inline=true>"Native"</Code>"."
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
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<label>"</Code>"."</ApiRow>
                        <ApiRow name="children" ty="Option<Children>" default="None">"The box and the label text."</ApiRow>
                    </ApiTable>
                    <p>
                        "See "<Link href=format!("{}#toggleoptions", routes::doc::checkbox::Hook.materialize())>"ToggleOptions"</Link>
                        " for how these settings behave."
                    </p>
                </Section>

                <Section title="Data Attributes" id="checkbox-data-attributes">
                    <p>"Set to "<Code inline=true>"true"</Code>" on the "<Code inline=true>"<label>"</Code>" while the state applies:"</p>
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
                        <ApiRow name="data-required" ty="true">"The checkbox\u{2019}s "<Code inline=true>"is_required"</Code>" is set."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="CheckboxGroup">
                <p>
                    "A "<Code inline=true>"<div role=\"group\">"</Code>" selecting a set of values. Its "<Code inline=true>"Checkbox"</Code>
                    "es need a "<Code inline=true>"value"</Code>". Label it with a "<Code inline=true>"Label"</Code>
                    " (or "<Code inline=true>"aria_label"</Code>"), and add a "<Code inline=true>"Description"</Code>
                    " and a "<Code inline=true>"FieldError"</Code>" as needed; they describe the group and each checkbox. See "
                    <Link href=routes::doc::atoms::Field.materialize()>"Field Atoms"</Link>"."
                </p>
                <Demo
                    description="Required group of weekday checkboxes with a label, a description and an error message"
                    source=include_str!("demos/checkbox_group.rs")
                >
                    <CheckboxGroupAtomDemo/>
                </Demo>
                <p>"Uncheck all days to see the error message."</p>

                <Section title="Props" id="checkbox-group-props">
                    <ApiTable kind=ApiKind::Props of="atoms::checkbox::CheckboxGroup">
                        <ApiRow name="default_value" ty="Vec<Key>" default="vec![]">"The initially checked values."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Vec<Key>>>" default="None">"Called with the checked values when they change."</ApiRow>
                        <ApiRow name="is_disabled, is_read_only" ty="Signal<bool>" default="false">"Disables the checkboxes, or prevents changes."</ApiRow>
                        <ApiRow name="is_required" ty="Signal<bool>" default="false">"At least one checkbox must be checked."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">"Marks the group invalid."</ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<Vec<Key>>>" default="None">"Validates the checked values."</ApiRow>
                        <ApiRow name="validation_behavior" ty="Option<ValidationBehavior>" default="None">
                            "When errors are shown. "<Code inline=true>"None"</Code>": the behavior of the surrounding "
                            <Link href=routes::doc::atoms::Form.materialize()>"Form"</Link>", else "<Code inline=true>"Native"</Code>"."
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
                        <ApiRow name="children" ty="Children">"The checkboxes, the field atoms and any other content."</ApiRow>
                    </ApiTable>
                    <p>
                        "See "<Link href=format!("{}#use-checkbox-group-state", routes::doc::checkbox::Hook.materialize())>
                        "use_checkbox_group_state"</Link>" for how these settings behave."
                    </p>
                </Section>

                <Section title="Data Attributes" id="checkbox-group-data-attributes">
                    <p>"Set to "<Code inline=true>"true"</Code>" on the group element while the state applies:"</p>
                    <ApiTable kind=ApiKind::DataAttributes>
                        <ApiRow name="data-disabled" ty="true">"The group is disabled."</ApiRow>
                        <ApiRow name="data-readonly" ty="true">"The group is read-only."</ApiRow>
                        <ApiRow name="data-required" ty="true">"The group is required."</ApiRow>
                        <ApiRow name="data-invalid" ty="true">"The group is invalid."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Styling">
                <p>
                    "Select the states with attribute selectors on your class. The input is visually hidden but focusable, so draw "
                    "the focus ring with "<Code inline=true>"data-focus-visible"</Code>":"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .my-checkbox { display: inline-flex; align-items: center; gap: 0.5em; cursor: pointer; }
                        .my-checkbox-box { width: 1em; height: 1em; border: 2px solid gray; border-radius: 4px; }
                        .my-checkbox[data-selected] .my-checkbox-box { background: royalblue; border-color: royalblue; }
                        .my-checkbox[data-focus-visible] .my-checkbox-box { outline: 2px solid royalblue; outline-offset: 2px; }
                        .my-checkbox[data-disabled] { opacity: 0.5; cursor: not-allowed; }
                    ")}
                </Code>
                <p>"The demos above show their complete styles under \u{201c}View styles\u{201d}."</p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Checkbox.materialize()>"Checkbox overview"</Link></li>
                <li><Link href=routes::doc::checkbox::Hook.materialize()>"Checkbox hooks"</Link></li>
                <li><Link href=routes::doc::checkbox::Component.materialize()>"Checkbox component"</Link></li>
                <li><Link href=routes::doc::switch::Atom.materialize()>"Switch atom"</Link></li>
                <li><Link href=routes::doc::atoms::Field.materialize()>"Field atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

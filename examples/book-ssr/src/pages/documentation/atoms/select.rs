use indoc::indoc;
use leptos::prelude::*;

use super::demos::{select::SelectAtomDemo, select_form::SelectFormDemo};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomSelect() -> impl IntoView {
    view! {
        <DocPage title="Select Atoms">
            <p>
                "The select atoms render an unstyled select whose parts you place where your design needs them. See "
                "the "<Link href=routes::doc::Select.materialize()>"Select overview"</Link>" for concept guidance."
            </p>

            <Section title="Hooks Used">
                <DocTable headers=&["Atom", "Hooks"]>
                    <TableRow>
                        <TableCell><Code inline=true>"Select"</Code></TableCell>
                        <TableCell>
                            <Link href=hook_section("use-select-state")>"use_select_state"</Link>", "
                            <Link href=hook_section("use-select")>"use_select"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"SelectTrigger"</Code></TableCell>
                        <TableCell>
                            <Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>", configured by "
                            <Code inline=true>"use_select"</Code>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"SelectPopover"</Code></TableCell>
                        <TableCell>
                            <Link href=routes::doc::popover::Hook.materialize()>"use_popover"</Link>
                            ", rendered like a modal "<Link href=routes::doc::popover::Atom.materialize()>"Popover"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"HiddenSelect"</Code></TableCell>
                        <TableCell><Link href=hook_section("use-hidden-select")>"use_hidden_select"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>
                            <Code inline=true>"SelectValue"</Code>", and the "
                            <Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link>" "
                            <Code inline=true>"Label"</Code>", "<Code inline=true>"Description"</Code>", "
                            <Code inline=true>"FieldError"</Code>
                        </TableCell>
                        <TableCell>"Props returned by "<Link href=hook_section("use-select")>"use_select"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"ListBox"</Code>" in the popover"</TableCell>
                        <TableCell>
                            <Link href=format!("{}#use-listbox", routes::doc::listbox::Hook.materialize())>"use_listbox"</Link>
                            ", configured by "<Code inline=true>"use_select"</Code>"; its items use "
                            <Link href=format!("{}#use-option", routes::doc::listbox::Hook.materialize())>"use_option"</Link>
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Example">
                <p>
                    "A "<Code inline=true>"Select"</Code>" holds the state and contains its parts: a "
                    <Link href=routes::doc::field::Atom.materialize()>"Label"</Link>", a "
                    <Code inline=true>"SelectTrigger"</Code>" with a "<Code inline=true>"SelectValue"</Code>", and a "
                    <Code inline=true>"SelectPopover"</Code>" with a "
                    <Link href=routes::doc::listbox::Atom.materialize()>"ListBox"</Link>" of one "
                    <Code inline=true>"ListBoxItem"</Code>" per option:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            atoms::{field::Label, listbox::{ListBox, ListBoxItem}, select::*},
                            hooks::collections::{Key, use_list_collection},
                        };
                        use leptos::{logging::log, prelude::*};

                        let fruits = use_list_collection(
                            Signal::stored(vec!["Apple", "Banana", "Cherry"]),
                            |fruit| Key::from(*fruit),
                            |fruit| (*fruit).to_owned(),
                        );

                        view! {
                            <Select collection=fruits on_change=Callback::new(|keys: Vec<Key>| log!("{keys:?}"))>
                                <Label>"Fruit"</Label>
                                <SelectTrigger>
                                    <SelectValue placeholder="Pick a fruit"/>
                                </SelectTrigger>
                                <SelectPopover>
                                    <ListBox>
                                        <ListBoxItem key="Apple">"Apple"</ListBoxItem>
                                        <ListBoxItem key="Banana">"Banana"</ListBoxItem>
                                        <ListBoxItem key="Cherry">"Cherry"</ListBoxItem>
                                    </ListBox>
                                </SelectPopover>
                            </Select>
                        }
                    "#)}
                </Code>
                <p>
                    "The listbox in the popover takes its options and settings from the select, so it needs no props. All "
                    <Link href=routes::doc::listbox::Atom.materialize()>"Listbox Atoms"</Link>" work in it: "
                    <Code inline=true>"ListBoxItems"</Code>", sections, item labels and descriptions."
                </p>
            </Section>

            <Section title="Demo">
                <p>
                    "Offices in three sections. Open the select with a click, or focus the trigger and press "
                    <Keys keys="Enter"/>", "<Keys keys="Space"/>" or "<Keys keys="ArrowDown"/>". On the closed trigger, "
                    <Keys keys="ArrowLeft"/>" and "<Keys keys="ArrowRight"/>" change the value, and typing a letter selects the "
                    "first matching office. Lisbon is fully booked: disabled in the collection, so it is skipped. The "
                    "demo is styled through the data attributes listed below."
                </p>
                <p>
                    "The selected office lives in app state, passed as "<Code inline=true>"value"</Code>" and "
                    <Code inline=true>"set_value"</Code>": \u{201c}Clear\u{201d} empties it, so the placeholder shows again."
                </p>
                <Demo
                    description="Office select with a label, placeholder, description, sections, a disabled option, its value controlled by app state and a disabled toggle"
                    source=include_str!("demos/select.rs")
                >
                    <SelectAtomDemo/>
                </Demo>
            </Section>

            <Section title="Forms">
                <p>
                    "Give the select a "<Code inline=true>"name"</Code>" and render a "<Code inline=true>"HiddenSelect"</Code>
                    ": it mirrors the value in a visually hidden native "<Code inline=true>"<select>"</Code>", so the value "
                    "is submitted with the form (as the option\u{2019}s key), resetting the form restores "
                    <Code inline=true>"default_value"</Code>", and browsers can autofill it. Pick a size, then submit or "
                    "reset the form:"
                </p>
                <Demo
                    description="T-shirt size select inside a form, showing the submitted value and form reset"
                    source=include_str!("demos/select_form.rs")
                >
                    <SelectFormDemo/>
                </Demo>
                <p>
                    "For validation, pass "<Code inline=true>"validate"</Code>" (or "<Code inline=true>"is_invalid"</Code>
                    ") and render a "<Link href=routes::doc::field::Atom.materialize()>"FieldError"</Link>". "<Code inline=true>"name"</Code>
                    " also matches errors reported by the server; see "
                    <Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link>"."
                </p>
            </Section>

            <Section title="Select">
                <p>
                    "Creates the select state and renders a "<Code inline=true>"<div>"</Code>" around its children, which may "
                    "contain any markup besides the parts."
                </p>
                <p>
                    "Label it with a "<Link href=routes::doc::field::Atom.materialize()>"Label"</Link>": a "
                    <Code inline=true>"<span>"</Code>" labelling the trigger and the listbox, which focuses the trigger when "
                    "clicked. A "<Code inline=true>"Description"</Code>" describes the trigger while it is rendered, and a "
                    <Code inline=true>"FieldError"</Code>" shows the validation errors while the select is invalid."
                </p>
                <Section title="Props" id="select-props">
                    <ApiTable kind=ApiKind::Props of="atoms::select::Select">
                        <ApiRow name="collection" ty="CollectionMemo">
                            "The options, from "<Code inline=true>"use_collection"</Code>" or "
                            <Code inline=true>"use_list_collection"</Code>". Required."
                        </ApiRow>
                        <ApiRow name="selection_mode" ty="SelectMode" default="Single">
                            <Code inline=true>"Single"</Code>" or "<Code inline=true>"Multiple"</Code>"."
                        </ApiRow>
                        <ApiRow name="default_value" ty="Vec<Key>" default="vec![]">
                            "The initially selected keys (at most one in "<Code inline=true>"Single"</Code>" mode). Resetting "
                            "the form restores it."
                        </ApiRow>
                        <ApiRow name="value" ty="Option<Signal<Vec<Key>>>" default="None">
                            "The selected keys (controlled), replacing "<Code inline=true>"default_value"</Code>": a value or any signal."
                        </ApiRow>
                        <ApiRow name="set_value" ty="Option<Out<Vec<Key>>>" default="None">
                            "Receives the selected keys: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Vec<Key>>>" default="None">
                            "Called with the selected keys, in collection order, when they change."
                        </ApiRow>
                        <ApiRow name="disabled_keys" ty="Option<Signal<HashSet<Key>>>" default="None">
                            "Options that can\u{2019}t be focused or selected, besides those disabled in the collection."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">
                            "Disables the trigger and the hidden form element."
                        </ApiRow>
                        <ApiRow name="is_required" ty="bool" default="false">
                            "Sets "<Code inline=true>"required"</Code>" on the hidden form element, with "
                            <Code inline=true>"ValidationBehavior::Native"</Code>" only."
                        </ApiRow>
                        <ApiRow name="default_open" ty="bool" default="false">"Start with the popover open."</ApiRow>
                        <ApiRow name="on_open_change" ty="Option<Callback<bool>>" default="None">
                            "Called when the popover opens or closes."
                        </ApiRow>
                        <ApiRow name="allows_empty_collection" ty="bool" default="false">
                            "Allow opening the popover without options, e.g. to show an empty state."
                        </ApiRow>
                        <ApiRow name="should_close_on_select" ty="CloseOnSelect" default="Auto">
                            "Close the popover when an option is selected: "<Code inline=true>"Always"</Code>", "
                            <Code inline=true>"Never"</Code>", or "<Code inline=true>"Auto"</Code>" (in "<Code inline=true>"Single"</Code>" mode). "
                            "A "<Code inline=true>"bool"</Code>" converts into it."
                        </ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                            "Labels the select when it has no "<Code inline=true>"Label"</Code>
                            ". Without it or "<Code inline=true>"aria_labelledby"</Code>", a "<Code inline=true>"Label"</Code>" is expected."
                        </ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"The id(s) of other elements labelling the select."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">
                            "Marks the select invalid while "<Code inline=true>"true"</Code>", taking precedence over all other validation; "
                            <Code inline=true>"false"</Code>" leaves validation to the other sources."
                        </ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<Vec<Key>>>" default="None">
                            "Validates the selected keys, returning error messages."
                        </ApiRow>
                        <ApiRow name="validation_behavior" ty="Option<ValidationBehavior>" default="None">
                            "When errors are shown. "<Code inline=true>"None"</Code>": the behavior of the surrounding "
                            <Link href=routes::doc::Form.materialize()>"Form"</Link>", else "<Code inline=true>"Native"</Code>"."
                        </ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">
                            "The form field name of the "<Code inline=true>"HiddenSelect"</Code>", also used to match server "
                            "validation errors."
                        </ApiRow>
                        <ApiRow name="form" ty="Option<String>" default="None">
                            "The id of the form the select belongs to, if it isn\u{2019}t inside of it."
                        </ApiRow>
                        <ApiRow name="on_focus_change" ty="Option<Callback<bool>>" default="None">
                            "Called when focus enters or leaves the select (trigger and popover)."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Classes and styles of the wrapping "<Code inline=true>"<div>"</Code>"."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">"The parts, the field atoms and any other content."</ApiRow>
                    </ApiTable>
                    <p>
                        "See "<Link href=hook_section("use-select-state")>"use_select_state"</Link>" for how these settings behave."
                    </p>
                </Section>
            </Section>

            <Section title="SelectTrigger">
                <p>
                    "The "<Code inline=true>"<button>"</Code>" that opens the popover, with "
                    <Code inline=true>"aria-haspopup=\"listbox\""</Code>" and "<Code inline=true>"aria-expanded"</Code>
                    ". It is labelled by the selected value and the label. The popover is positioned at it."
                </p>
                <Section title="Props" id="select-trigger-props">
                    <ApiTable kind=ApiKind::Props of="SelectTrigger">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Classes and styles of the "<Code inline=true>"<button>"</Code>"."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">
                            "The button content: a "<Code inline=true>"SelectValue"</Code>", and typically an icon."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="SelectValue">
                <p>
                    "The text of the selected option (in "<Code inline=true>"Multiple"</Code>" mode: the options\u{2019} texts, "
                    "separated by commas), or the placeholder, as a "<Code inline=true>"<span>"</Code>". The text comes from "
                    "the collection\u{2019}s text values. To show something richer, read the selection from the "
                    <AnchorLink href="#composition">"context"</AnchorLink>" instead."
                </p>
                <Section title="Props" id="select-value-props">
                    <ApiTable kind=ApiKind::Props of="SelectValue">
                        <ApiRow name="placeholder" ty="Option<String>" default="None">"Shown while nothing is selected."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Classes and styles of the "<Code inline=true>"<span>"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="SelectPopover">
                <p>
                    "The popover with the options. It is rendered into the document body while open and positioned at the "
                    "trigger, flipped when there is no room. Focus moves into its listbox and returns to the trigger when "
                    "it closes. "<Keys keys="Escape"/>", a click outside and focus leaving the popover close it."
                </p>
                <Section title="Props" id="select-popover-props">
                    <ApiTable kind=ApiKind::Props of="SelectPopover">
                        <ApiRow name="placement" ty="Signal<Placement>" default="BottomStart">"Where the popover goes relative to the trigger, see "<Link href=format!("{}#placements", routes::doc::overlay_behavior::UseOverlayPosition.materialize())>"Placements"</Link>"."</ApiRow>
                        <ApiRow name="max_height" ty="Signal<Option<f64>>" default="None">"A maximum height; the room available limits it further."</ApiRow>
                        <ApiRow name="offset, cross_offset" ty="Signal<f64>" default="0.0">
                            "Distance from the trigger and shift along its edge, in pixels."
                        </ApiRow>
                        <ApiRow name="container_padding" ty="Signal<f64>" default="12.0">"Minimum distance from the viewport edges."</ApiRow>
                        <ApiRow name="should_flip" ty="Signal<bool>" default="true">"Flip above the trigger when there is no room below."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Classes and styles of the popover "<Code inline=true>"<div>"</Code>". The position is set as inline style."
                        </ApiRow>
                        <ApiRow name="children" ty="ChildrenFn">"A "<Code inline=true>"ListBox"</Code>" with the options."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="HiddenSelect">
                <p>
                    "A visually hidden, "<Code inline=true>"aria-hidden"</Code>" native form element mirroring the value: a "
                    <Code inline=true>"<select>"</Code>" for up to 300 options, hidden inputs for more (rendered only when the "
                    "select has a "<Code inline=true>"name"</Code>"). The options\u{2019} keys are the submitted values. Its "
                    "name, form, disabled and required state come from "<Code inline=true>"Select"</Code>"."
                </p>
                <Section title="Props" id="hidden-select-props">
                    <ApiTable kind=ApiKind::Props of="HiddenSelect">
                        <ApiRow name="auto_complete" ty="Option<String>" default="None">
                            "The "<Code inline=true>"autocomplete"</Code>" attribute, a hint for browser autofill (e.g. "
                            <Code inline=true>"\"country\""</Code>")."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <p>
                    "Flags are rendered as "<Code inline=true>"data-open=\"true\""</Code>" while the "
                    "state applies."
                </p>
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-open" ty="true">
                        "On "<Code inline=true>"Select"</Code>" and "<Code inline=true>"SelectTrigger"</Code>": the popover is open."
                    </ApiRow>
                    <ApiRow name="data-disabled" ty="true">
                        "On "<Code inline=true>"Select"</Code>" and "<Code inline=true>"SelectTrigger"</Code>": the select is "
                        "disabled. On an item: the option is disabled."
                    </ApiRow>
                    <ApiRow name="data-invalid" ty="true">
                        "On "<Code inline=true>"Select"</Code>" and "<Code inline=true>"SelectTrigger"</Code>": the value is invalid."
                    </ApiRow>
                    <ApiRow name="data-pressed" ty="true">
                        "On "<Code inline=true>"SelectTrigger"</Code>" and items: being pressed."
                    </ApiRow>
                    <ApiRow name="data-focus-visible" ty="true">
                        "On "<Code inline=true>"SelectTrigger"</Code>" and items: has focus, and focus should be shown "
                        "(keyboard navigation)."
                    </ApiRow>
                    <ApiRow name="data-placeholder" ty="true">
                        "On "<Code inline=true>"SelectValue"</Code>": nothing is selected; the placeholder is shown."
                    </ApiRow>
                    <ApiRow name="data-focused" ty="true">
                        "On an item: the option has focus, by keyboard or pointer hover."
                    </ApiRow>
                    <ApiRow name="data-selected" ty="true">"On an item: the option is selected."</ApiRow>
                </ApiTable>
                <p>
                    "The items are "<Code inline=true>"ListBoxItem"</Code>"s, see the "
                    <Link href=format!("{}#data-attributes", routes::doc::listbox::Atom.materialize())>"Listbox Atoms"</Link>"."
                </p>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms bring no styles. Their default classes are "<Code inline=true>"leptonic-Select"</Code>
                    " (the "<Code inline=true>"<div>"</Code>" around the parts), "<Code inline=true>"leptonic-SelectTrigger"</Code>
                    " (a "<Code inline=true>"<button>"</Code>"), "<Code inline=true>"leptonic-SelectValue"</Code>" and "
                    <Code inline=true>"leptonic-SelectPopover"</Code>"; the options are "<Code inline=true>"leptonic-ListBoxItem"</Code>
                    "s in a "<Code inline=true>"leptonic-ListBox"</Code>", and section headings "
                    <Code inline=true>"leptonic-ListBoxSectionHeading"</Code>". Put the "<Code inline=true>"SelectValue"</Code>
                    " and your own caret, hidden from assistive technology, into the trigger:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        <SelectTrigger classes="demo-sel-trigger">
                            <SelectValue placeholder="Choose an office" classes="demo-sel-value"/>
                            <span class="demo-sel-caret" aria-hidden="true">"\u{25bc}"</span>
                        </SelectTrigger>
                    "#)}
                </Code>
                <p>
                    "Target the state with the data attributes. Hovering an option focuses it, so "
                    <Code inline=true>"[data-focused]"</Code>" highlights the option under the pointer and the arrow keys "
                    "alike; "<Code inline=true>"[data-focus-visible]"</Code>" is added while the keyboard is in use. Draw the "
                    "options\u{2019} focus ring inside them, so that the popover doesn\u{2019}t clip it. The trigger renders "
                    "no hover attribute yet, so "<Code inline=true>":hover"</Code>" stands in. The popover is rendered into "
                    "the document body, so style it through its own classes, not as a descendant of the select; it sets no "
                    "width, so give it a "<Code inline=true>"min-width"</Code>". The demos above use this CSS:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r#"
                        .demo-sel { display: inline-flex; flex-direction: column; gap: 0.25rem; width: 16em; }

                        .demo-sel-trigger {
                            display: flex;
                            align-items: center;
                            justify-content: space-between;
                            padding: 0.5rem 1rem;
                            border: 1px solid var(--border);
                            border-radius: 6px;
                            background: var(--surface);
                            font: inherit;
                        }
                        .demo-sel-trigger:hover:not([data-disabled]),
                        .demo-sel-trigger[data-open] { border-color: var(--accent); }
                        .demo-sel-trigger[data-focus-visible] { outline: 2px solid var(--focus); outline-offset: 2px; }
                        .demo-sel-trigger[data-invalid] { border-color: var(--danger); }
                        .demo-sel-trigger[data-disabled] { opacity: 0.5; cursor: not-allowed; }

                        .demo-sel-value[data-placeholder] { color: var(--muted); }
                        [data-open] > .demo-sel-caret { transform: rotate(180deg); }

                        .demo-sel-popover { min-width: 16em; max-height: 18em; overflow-y: auto; padding: 0.25rem; border: 1px solid var(--border); border-radius: 6px; background: var(--surface); }

                        .demo-sel-item { display: flex; justify-content: space-between; padding: 0.5rem; border-radius: 6px; cursor: pointer; }
                        .demo-sel-item[data-focused] { background: var(--border); }
                        .demo-sel-item[data-focus-visible] { outline: 2px solid var(--focus); outline-offset: -2px; }
                        .demo-sel-item[data-selected] { color: var(--accent); font-weight: 600; }
                        .demo-sel-item[data-selected]::after { content: "\2713"; }
                        .demo-sel-item[data-disabled] { opacity: 0.5; cursor: not-allowed; }
                    "#)}
                </Code>
                <p>
                    "Apps that don\u{2019}t want to style from scratch can load leptonic\u{2019}s optional atom theme, "
                    <Code inline=true>"@use \"leptonic/leptonic-atoms\";"</Code>", which styles the default classes."
                </p>
            </Section>

            <Section title="Composition">
                <p>
                    "The parts find the select through the "<Code inline=true>"SelectCtx"</Code>" context. Your own "
                    "Leptos components inside "<Code inline=true>"Select"</Code>" can read it too, e.g. to show the number of "
                    "selected options in a multiple select\u{2019}s trigger:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        #[component]
                        fn SelectedCount() -> impl IntoView {
                            let state = expect_context::<SelectCtx>().state;
                            move || match state.value().len() {
                                0 => "None selected".to_owned(),
                                n => format!("{n} selected"),
                            }
                        }
                    "#)}
                </Code>
                <p>
                    <Code inline=true>"state"</Code>" is the "<Code inline=true>"SelectState"</Code>" documented on the "
                    <Link href=hook_section("use-select-state")>"hook page"</Link>". "<Code inline=true>"SelectCtx"</Code>
                    " also has "<Code inline=true>"is_invalid"</Code>" and "<Code inline=true>"is_disabled"</Code>"."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Select.materialize()>"Select overview"</Link></li>
                <li><Link href=routes::doc::select::Hook.materialize()>"Select Hooks"</Link></li>
                <li><Link href=routes::doc::listbox::Atom.materialize()>"Listbox Atoms"</Link></li>
                <li><Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link></li>
                <li><Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

/// A link target on the select hook page.
fn hook_section(id: &str) -> String {
    format!("{}#{id}", routes::doc::select::Hook.materialize())
}

use indoc::indoc;
use leptos::prelude::*;

use super::demos::combobox::ComboBoxAtomDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomComboBox() -> impl IntoView {
    view! {
        <DocPage title="Combobox Atoms">
            <p>
                "The combobox atoms render an unstyled combobox whose parts you place where your design needs them. "
                "See the "<Link href=routes::doc::Combobox.materialize()>"Combobox overview"</Link>" for concept guidance."
            </p>

            <Section title="Hooks Used">
                <DocTable headers=&["Atom", "Hooks"]>
                    <TableRow>
                        <TableCell><Code inline=true>"ComboBox"</Code></TableCell>
                        <TableCell>
                            <Link href=format!("{}#use-combobox-state", routes::doc::combobox::Hook.materialize())>"use_combobox_state"</Link>", "
                            <Link href=format!("{}#use-combobox", routes::doc::combobox::Hook.materialize())>"use_combobox"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Input"</Code></TableCell>
                        <TableCell>
                            <Link href=routes::doc::text_field::Hook.materialize()>"use_text_field"</Link>", configured by "
                            <Code inline=true>"use_combobox"</Code>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"ComboBoxButton"</Code></TableCell>
                        <TableCell>
                            <Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>", configured by "
                            <Code inline=true>"use_combobox"</Code>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"ComboBoxPopover"</Code></TableCell>
                        <TableCell>
                            <Link href=routes::doc::popover::Hook.materialize()>"use_popover"</Link>", non-modal"
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"ListBox"</Code>" in the popover"</TableCell>
                        <TableCell>
                            <Link href=format!("{}#use-listbox", routes::doc::listbox::Hook.materialize())>"use_listbox"</Link>
                            ", configured by "<Code inline=true>"use_combobox"</Code>"; its items use "
                            <Link href=format!("{}#use-option", routes::doc::listbox::Hook.materialize())>"use_option"</Link>
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Example">
                <p>
                    "A combobox is a "<Code inline=true>"ComboBox"</Code>" containing its parts. The popover contains a "
                    <Code inline=true>"ListBox"</Code>" whose "<Code inline=true>"ListBoxItems"</Code>" renders the options "
                    "matching the input:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            atoms::{combobox::*, field::Label, input::Input, listbox::{ListBox, ListBoxItems}},
                            hooks::{Key, use_contains_filter, use_list_collection},
                        };
                        use leptos::prelude::*;

                        let fruits = use_list_collection(
                            Signal::stored(vec!["Apple", "Banana", "Cherry"]),
                            |fruit| Key::from(*fruit),
                            |fruit| (*fruit).to_owned(),
                        );

                        view! {
                            <ComboBox collection=fruits filter=use_contains_filter()>
                                <Label>"Fruit"</Label>
                                <Input/>
                                <ComboBoxButton><span aria-hidden="true">"▼"</span></ComboBoxButton>
                                <ComboBoxPopover>
                                    <ListBox>
                                        <ListBoxItems let:node>{node.text_value.to_string()}</ListBoxItems>
                                    </ListBox>
                                </ComboBoxPopover>
                            </ComboBox>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "Type to filter the countries, or open the popover with the button or "<Keys keys="ArrowDown"/>
                    ". Finland is disabled. The demo is styled through the data attributes listed below."
                </p>
                <p>
                    "The selected country and the input text live in app state, passed as "<Code inline=true>"value"</Code>
                    " / "<Code inline=true>"set_value"</Code>" and "<Code inline=true>"input_value"</Code>" / "
                    <Code inline=true>"set_input_value"</Code>". \u{201c}Ship to Sweden\u{201d} sets the value from the app, "
                    "and the input follows with the country\u{2019}s name."
                </p>
                <Demo
                    description="Country combobox built from the atoms, styled through data attributes, with its value and input text controlled by app state and a disabled toggle"
                    source=include_str!("demos/combobox.rs")
                >
                    <ComboBoxAtomDemo/>
                </Demo>
            </Section>

            <Section title="ComboBox">
                <p>
                    "Creates the combobox state and renders a "<Code inline=true>"<div>"</Code>" around its children, "
                    "which may contain any markup besides the parts."
                </p>
                <p>
                    "Label it with a "<Link href=routes::doc::field::Atom.materialize()>"Label"</Link>": a "
                    <Code inline=true>"<label>"</Code>" for the input, which also labels the button and the listbox. A "
                    <Code inline=true>"Description"</Code>" describes the input while it is rendered, and a "
                    <Code inline=true>"FieldError"</Code>" shows the validation errors while the combobox is invalid."
                </p>
                <Section title="Props" id="combobox-props">
                    <ApiTable kind=ApiKind::Props of="ComboBox">
                        <ApiRow name="collection" ty="CollectionMemo">
                            "All options, from "<Code inline=true>"use_collection"</Code>" or "<Code inline=true>"use_list_collection"</Code>"."
                        </ApiRow>
                        <ApiRow name="filter" ty="Option<ComboBoxFilter>" default="None">
                            "Shows the options matching the input, e.g. "<Code inline=true>"use_contains_filter()"</Code>
                            ". Without a filter, all options are shown."
                        </ApiRow>
                        <ApiRow name="selection_mode" ty="SelectMode" default="Single">"One option or several."</ApiRow>
                        <ApiRow name="default_value" ty="Vec<Key>" default="vec![]">"The initially selected keys."</ApiRow>
                        <ApiRow name="value" ty="Option<Signal<Vec<Key>>>" default="None">
                            "The selected keys (controlled), replacing "<Code inline=true>"default_value"</Code>": a value or any signal."
                        </ApiRow>
                        <ApiRow name="set_value" ty="Option<Out<Vec<Key>>>" default="None">
                            "Receives the new state: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Vec<Key>>>" default="None">
                            "Called with the selected keys when they change."
                        </ApiRow>
                        <ApiRow name="default_input_value" ty="Option<String>" default="None">
                            "The initial input text. By default, the selected option\u{2019}s text."
                        </ApiRow>
                        <ApiRow name="input_value" ty="Option<Signal<String>>" default="None">
                            "The input text (controlled), replacing "<Code inline=true>"default_input_value"</Code>": a value or any signal."
                        </ApiRow>
                        <ApiRow name="set_input_value" ty="Option<Out<String>>" default="None">
                            "Receives the new state: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="on_input_change" ty="Option<Callback<String>>" default="None">
                            "Called when the input text changes."
                        </ApiRow>
                        <ApiRow name="disabled_keys" ty="Option<Signal<HashSet<Key>>>" default="None">
                            "Options that can\u{2019}t be focused or selected, besides those disabled in the collection."
                        </ApiRow>
                        <ApiRow name="menu_trigger" ty="ComboBoxMenuTrigger" default="Input">
                            "When the popover opens: on typing, also on focus, or only with the button and arrow keys."
                        </ApiRow>
                        <ApiRow name="allows_empty_collection" ty="bool" default="false">
                            "Keep the popover open when no option matches."
                        </ApiRow>
                        <ApiRow name="allows_custom_value" ty="bool" default="false">
                            "Keep typed text that matches no option (the selection is cleared)."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables the input and the button."</ApiRow>
                        <ApiRow name="is_read_only" ty="Signal<bool>" default="false">
                            "Shows the value without allowing changes."
                        </ApiRow>
                        <ApiRow name="is_required" ty="bool" default="false">"Marks the input as required."</ApiRow>
                        <ApiRow name="aria_label, aria_labelledby" ty="MaybeProp<String>, Option<String>" default="None">
                            "Labels the combobox when it has no "<Code inline=true>"Label"</Code>
                            ". Without them, a "<Code inline=true>"Label"</Code>" is expected."
                        </ApiRow>
                        <ApiRow name="placeholder" ty="Option<String>" default="None">"The input\u{2019}s placeholder."</ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">
                            "The form field name of the input, also used to match server validation errors."
                        </ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">
                            "Marks the combobox invalid while "<Code inline=true>"true"</Code>", taking precedence over all other validation; "
                            <Code inline=true>"false"</Code>" leaves validation to the other sources."
                        </ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<ComboBoxValue>>" default="None">
                            "Validates the input text and the selected keys."
                        </ApiRow>
                        <ApiRow name="validation_behavior" ty="Option<ValidationBehavior>" default="None">
                            "When errors are shown. "<Code inline=true>"None"</Code>": the behavior of the surrounding "
                            <Link href=routes::doc::Form.materialize()>"Form"</Link>", else "<Code inline=true>"Native"</Code>"."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Classes and styles of the wrapping "<Code inline=true>"<div>"</Code>"."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">"The parts, the field atoms and any other content."</ApiRow>
                    </ApiTable>
                    <p>
                        "See "<Link href=format!("{}#use-combobox-state", routes::doc::combobox::Hook.materialize())>
                        "use_combobox_state"</Link>" for how these settings behave."
                    </p>
                </Section>
            </Section>

            <Section title="Input">
                <p>
                    "The text input is the "<Link href=routes::doc::text_field::Atom.materialize()>"Input"</Link>" atom, "
                    "with "<Code inline=true>"role=\"combobox\""</Code>". It has focus while the user works with the "
                    "combobox, and the popover is positioned at it. It renders the "
                    <Link href=format!("{}#input-data-attributes", routes::doc::text_field::Atom.materialize())>
                        "data attributes of the Input atom"
                    </Link>"."
                </p>
            </Section>

            <Section title="ComboBoxButton">
                <p>
                    "The button that opens and closes the popover. It is not in the tab order and doesn\u{2019}t take focus: "
                    "keyboard users open the popover with "<Keys keys="ArrowDown"/>" in the input."
                </p>
                <Section title="Props" id="combobox-button-props">
                    <ApiTable kind=ApiKind::Props of="ComboBoxButton">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Classes and styles of the "<Code inline=true>"<button>"</Code>"."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">
                            "The button content, typically an icon. The button is labelled \u{201c}Show suggestions\u{201d}."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ComboBoxPopover">
                <p>
                    "The popover with the options. It is rendered into the document body while open, positioned at the input "
                    "and flipped when there is no room. It is non-modal: focus stays in the input, and the page stays "
                    "usable. Focus leaving the combobox and scrolling the page close it."
                </p>
                <Section title="Props" id="combobox-popover-props">
                    <ApiTable kind=ApiKind::Props of="ComboBoxPopover">
                        <ApiRow name="placement" ty="Signal<Placement>" default="BottomStart">"Where the popover goes relative to the input, see "<Link href=format!("{}#placements", routes::doc::overlay_behavior::UseOverlayPosition.materialize())>"Placements"</Link>"."</ApiRow>
                        <ApiRow name="max_height" ty="Signal<Option<f64>>" default="None">"A maximum height; the room available limits it further."</ApiRow>
                        <ApiRow name="offset, cross_offset" ty="Signal<f64>" default="0.0">
                            "Distance from the input and shift along its edge, in pixels."
                        </ApiRow>
                        <ApiRow name="container_padding" ty="Signal<f64>" default="12.0">"Minimum distance from the viewport edges."</ApiRow>
                        <ApiRow name="should_flip" ty="Signal<bool>" default="true">"Flip above the input when there is no room below."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Classes and styles of the popover "<Code inline=true>"<div>"</Code>". The position is set as inline style."
                        </ApiRow>
                        <ApiRow name="children" ty="ChildrenFn">"A "<Code inline=true>"ListBox"</Code>" with the options."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Options">
                <p>
                    "Inside a "<Code inline=true>"ComboBoxPopover"</Code>", the "
                    <Link href=routes::doc::listbox::Atom.materialize()>"ListBox"</Link>" atom takes its options and settings "
                    "from the combobox; its own props are ignored apart from "<Code inline=true>"classes"</Code>" and "
                    <Code inline=true>"styles"</Code>". Render the options with "
                    <Link href=format!("{}#listboxitems", routes::doc::listbox::Atom.materialize())>"ListBoxItems"</Link>
                    ": it renders one "<Code inline=true>"ListBoxItem"</Code>" per item of the listbox\u{2019}s collection as "
                    "it currently is \u{2014} for a combobox, the options matching the input."
                </p>
            </Section>

            <Section title="Data Attributes">
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-open" ty="true">
                        "On "<Code inline=true>"ComboBox"</Code>" and "<Code inline=true>"ComboBoxButton"</Code>": the popover is open."
                    </ApiRow>
                    <ApiRow name="data-disabled" ty="true">
                        "On "<Code inline=true>"ComboBox"</Code>" and "<Code inline=true>"ComboBoxButton"</Code>": the combobox "
                        "is disabled. On an item: the option is disabled."
                    </ApiRow>
                    <ApiRow name="data-hovered" ty="true">
                        "On "<Code inline=true>"ComboBoxButton"</Code>": a mouse or pen is over the button."
                    </ApiRow>
                    <ApiRow name="data-invalid" ty="true">
                        "On "<Code inline=true>"ComboBox"</Code>" and the "<Code inline=true>"Input"</Code>": the value is invalid."
                    </ApiRow>
                    <ApiRow name="data-focus-visible" ty="true">
                        "On the "<Code inline=true>"Input"</Code>": it has keyboard focus. On an item: the option is "
                        "focused by keyboard."
                    </ApiRow>
                    <ApiRow name="data-pressed" ty="true">
                        "On "<Code inline=true>"ComboBoxButton"</Code>" and items: being pressed."
                    </ApiRow>
                    <ApiRow name="data-focused" ty="true">
                        "On an item: the option has the (virtual) focus, by keyboard or pointer hover."
                    </ApiRow>
                    <ApiRow name="data-selected" ty="true">"On an item: the option is selected."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms bring no styles. Their default classes are "<Code inline=true>"leptonic-ComboBox"</Code>
                    " (the "<Code inline=true>"<div>"</Code>" around the parts), "<Code inline=true>"leptonic-Input"</Code>", "
                    <Code inline=true>"leptonic-ComboBoxButton"</Code>" and "<Code inline=true>"leptonic-ComboBoxPopover"</Code>
                    "; the options are "<Code inline=true>"leptonic-ListBoxItem"</Code>"s in a "<Code inline=true>"leptonic-ListBox"</Code>
                    ". The button is named for assistive technology; its content is yours, e.g. an "
                    <Code inline=true>"aria-hidden"</Code>" caret. Wrap the input and the button in your own element to "
                    "draw them as one control."
                </p>
                <p>
                    "Target the state with the data attributes. The options are focused virtually (the focus stays in the "
                    "input), so style the focused option with "<Code inline=true>"[data-focused]"</Code>", not "
                    <Code inline=true>":focus"</Code>". The popover is rendered into the document body, so style it "
                    "through its own classes, not as a descendant of the combobox. The demos above use this CSS:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .demo-combo { display: inline-flex; flex-direction: column; gap: 0.25rem; width: 16em; }
                        .demo-combo-field { display: flex; }

                        .demo-combo-atom-input { flex: 1; min-width: 0; padding: 0.5rem 1rem; border: 1px solid var(--border); border-right: none; border-radius: 6px 0 0 6px; }
                        .demo-combo-atom-input[data-hovered],
                        .demo-combo-atom-input[data-focused] { border-color: var(--accent); }
                        .demo-combo-atom-input[data-focus-visible] { outline: 2px solid var(--focus); outline-offset: 1px; }
                        .demo-combo-atom-input[data-disabled] { opacity: 0.5; }

                        .demo-combo-atom-button { padding: 0 1rem; border: 1px solid var(--border); border-radius: 0 6px 6px 0; background: var(--surface); color: var(--muted); }
                        .demo-combo-atom-button[data-hovered] { color: var(--accent); }
                        .demo-combo-atom-button[data-open] span { display: inline-block; transform: rotate(180deg); }
                        .demo-combo-atom-button[data-disabled] { opacity: 0.5; cursor: not-allowed; }

                        .demo-combo-popover { min-width: 16em; max-height: 15em; overflow-y: auto; padding: 0.25rem; border: 1px solid var(--border); border-radius: 6px; background: var(--surface); }

                        .demo-combo-item { padding: 0.5rem; border-radius: 6px; cursor: pointer; }
                        .demo-combo-item[data-focused] { background: var(--border); }
                        .demo-combo-item[data-selected] { color: var(--accent); font-weight: 600; }
                        .demo-combo-item[data-disabled] { opacity: 0.5; cursor: not-allowed; }
                    ")}
                </Code>
                <p>
                    "Apps that don\u{2019}t want to style from scratch can load leptonic\u{2019}s optional atom theme, "
                    <Code inline=true>"@use \"leptonic/leptonic-atoms\";"</Code>", which styles the default classes."
                </p>
            </Section>

            <Section title="Composition">
                <p>
                    "The parts find the combobox through the "<Code inline=true>"ComboBoxCtx"</Code>" context. Your own "
                    "Leptos components inside "<Code inline=true>"ComboBox"</Code>" can read it too, e.g. to show a hint while "
                    "the text matches no selection:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        #[component]
                        fn SearchHint() -> impl IntoView {
                            let state = expect_context::<ComboBoxCtx>().state;
                            let searching = move || !state.input_value().is_empty() && state.selected_key().is_none();
                            view! {
                                <Show when=searching>
                                    <p>"Searching for: " {move || state.input_value()}</p>
                                </Show>
                            }
                        }
                    "#)}
                </Code>
                <p>
                    <Code inline=true>"state"</Code>" is the "<Code inline=true>"ComboBoxState"</Code>" documented on the "
                    <Link href=format!("{}#use-combobox-state", routes::doc::combobox::Hook.materialize())>"hook page"</Link>"."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Combobox.materialize()>"Combobox overview"</Link></li>
                <li><Link href=routes::doc::combobox::Hook.materialize()>"Combobox Hooks"</Link></li>
                <li><Link href=routes::doc::listbox::Atom.materialize()>"Listbox Atoms"</Link></li>
                <li><Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

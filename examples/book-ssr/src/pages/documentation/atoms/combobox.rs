use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::combobox::ComboBoxAtomDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomComboBox() -> impl IntoView {
    view! {
        <DocPage title="Combobox atoms">
            <p>
                "The combobox atoms render an unstyled combobox with the behavior of the "
                <Link href=routes::doc::combobox::Hook.materialize()>"combobox hooks"</Link>": "
                <Code inline=true>"ComboBox"</Code>" holds the state, and you place its parts \u{2014} label, input, "
                "button and popover \u{2014} where your design needs them. See the "
                <Link href=routes::doc::Combobox.materialize()>"Combobox overview"</Link>" for concept guidance."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Code inline=true>"ComboBox"</Code>" calls "
                    <Link href=routes::doc::combobox::Hook.materialize()><Code inline=true>"use_combobox_state"</Code>" and "
                    <Code inline=true>"use_combobox"</Code></Link>". The parts use "
                    <Link href=routes::doc::text_field::Hook.materialize()>"use_text_field"</Link>" (the "<Code inline=true>"Input"</Code>"), "
                    <Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>" (button), "
                    <Link href=routes::doc::overlays::UseOverlay.materialize()>"use_overlay"</Link>" and "
                    <Code inline=true>"use_overlay_position"</Code>" (popover), and the "
                    <Link href=routes::doc::listbox::Hook.materialize()>"listbox hooks"</Link>" (options)."
                </p>
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
                            hooks::{collections::use_list_collection, use_contains_filter},
                        };

                        let fruits = use_list_collection(
                            Signal::stored(vec!["Apple", "Banana", "Cherry"]),
                            |fruit| Key::from(*fruit),
                            |fruit| (*fruit).to_owned(),
                        );

                        view! {
                            <ComboBox collection=fruits filter=use_contains_filter()>
                                <Label>"Fruit"</Label>
                                <Input/>
                                <ComboBoxButton>"\u{25bc}"</ComboBoxButton>
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
                    "The selected country and the input text live in signals of the demo, bound with "
                    <Code inline=true>"value"</Code>" ("<Code inline=true>"RwSignal<Vec<Key>>"</Code>") and "
                    <Code inline=true>"input_value"</Code>" ("<Code inline=true>"RwSignal<String>"</Code>"): the combo box "
                    "shows them, and typing and choosing write them. \u{201c}Ship to Sweden\u{201d} sets the value from the "
                    "app, and the input follows with the country\u{2019}s name. Bind only "<Code inline=true>"value"</Code>
                    " if you don\u{2019}t need the text. ""A signal pair "<Code inline=true>"(read, write)"</Code>" works as well, and "
                    <Code inline=true>"ValueBinding::new(signal, callback)"</Code>" binds any other storage."
                </p>
                <Demo
                    description="Country combobox built from the atoms, styled through data attributes, with its value and input text bound to app state and a disabled toggle"
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
                    "Label it with a "<Link href=routes::doc::atoms::Field.materialize()>"Label"</Link>": a "
                    <Code inline=true>"<label>"</Code>" for the input, which also labels the button and the listbox. A "
                    <Code inline=true>"Description"</Code>" describes the input while it is rendered, and a "
                    <Code inline=true>"FieldError"</Code>" shows the validation errors while the combobox is invalid."
                </p>
                <Section title="Props">
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
                        <ApiRow name="value" ty="Option<ValueBinding<Vec<Key>>>" default="None">
                            "The selected keys as app state (e.g. an "<Code inline=true>"RwSignal<Vec<Key>>"</Code>"), replacing "<Code inline=true>"default_value"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Vec<Key>>>" default="None">
                            "Called with the selected keys when they change."
                        </ApiRow>
                        <ApiRow name="default_input_value" ty="Option<String>" default="None">
                            "The initial input text. By default, the selected option\u{2019}s text."
                        </ApiRow>
                        <ApiRow name="input_value" ty="Option<ValueBinding<String>>" default="None">
                            "The input text as app state (e.g. an "<Code inline=true>"RwSignal<String>"</Code>"), replacing "<Code inline=true>"default_input_value"</Code>"."
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
                            <Link href=routes::doc::atoms::Form.materialize()>"Form"</Link>", else "<Code inline=true>"Native"</Code>"."
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
                <Section title="Props">
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
                    "and flipped when there is no room. It is non-modal: focus stays in the input. A click outside closes it."
                </p>
                <Section title="Props">
                    <ApiTable kind=ApiKind::Props of="ComboBoxPopover">
                        <ApiRow name="placement_x" ty="Signal<PlacementX>" default="Left">"Horizontal placement relative to the input."</ApiRow>
                        <ApiRow name="placement_y" ty="Signal<PlacementY>" default="Below">"Vertical placement relative to the input."</ApiRow>
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

            <Section title="ListBoxItems">
                <p>
                    "Inside a "<Code inline=true>"ComboBoxPopover"</Code>", the "<Code inline=true>"ListBox"</Code>
                    " atom takes its options and settings from the combobox; its own props are ignored apart from "
                    <Code inline=true>"classes"</Code>" and "<Code inline=true>"styles"</Code>". "
                    <Code inline=true>"ListBoxItems"</Code>" renders one "<Code inline=true>"ListBoxItem"</Code>" per item of "
                    "the listbox\u{2019}s collection as it currently is \u{2014} for a combobox, the options matching the input. "
                    "Its children render the content of an item from its collection "<Code inline=true>"Node"</Code>" ("
                    <Code inline=true>"let:node"</Code>")."
                </p>
                <Section title="Props">
                    <ApiTable kind=ApiKind::Props of="ListBoxItems">
                        <ApiRow name="children" ty="Fn(Node) -> impl IntoView">
                            "Renders an item\u{2019}s content. The node has the item\u{2019}s "<Code inline=true>"key"</Code>" and "
                            <Code inline=true>"text_value"</Code>"; look up richer data by the key."
                        </ApiRow>
                        <ApiRow name="classes" ty="Classes" default="empty">"Classes of each item."</ApiRow>
                    </ApiTable>
                </Section>
                <p>
                    "The content can use "<Code inline=true>"ListBoxItemLabel"</Code>" and "
                    <Code inline=true>"ListBoxItemDescription"</Code>" for a label and a description that label and describe "
                    "the option."
                </p>
            </Section>

            <Section title="Data Attributes">
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-open" ty="true">
                        "On "<Code inline=true>"ComboBox"</Code>" and "<Code inline=true>"ComboBoxButton"</Code>": the popover is open."
                    </ApiRow>
                    <ApiRow name="data-disabled" ty="true">
                        "On "<Code inline=true>"ComboBox"</Code>": the combobox is disabled. On an item: the option is disabled."
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
                    "The atoms bring no styles. Pass "<Code inline=true>"classes"</Code>" and target the state with attribute "
                    "selectors. The options are focused virtually, so style the focused option with "
                    <Code inline=true>"[data-focused]"</Code>", not "<Code inline=true>":focus"</Code>":"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .my-input[data-focus-visible] { outline: 2px solid royalblue; }
                        .my-option[data-focused] { background: #eef; }
                        .my-option[data-selected] { font-weight: bold; }
                        .my-option[data-disabled] { opacity: 0.5; }
                    ")}
                </Code>
                <p>
                    "The popover is rendered into the document body, so style it through its own classes, not as a "
                    "descendant of the combobox."
                </p>
            </Section>

            <Section title="Composition">
                <p>
                    "The parts find the combobox through the "<Code inline=true>"ComboBoxCtx"</Code>" context. Your own "
                    "components inside "<Code inline=true>"ComboBox"</Code>" can read it too, e.g. to show a hint while "
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
                <li><Link href=routes::doc::combobox::Hook.materialize()>"Combobox hooks"</Link></li>
                <li><Link href=routes::doc::Listbox.materialize()>"Listbox concept"</Link></li>
                <li><Link href=routes::doc::atoms::Field.materialize()>"Field atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

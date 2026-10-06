use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::listbox::ListBoxAtomDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomListBox() -> impl IntoView {
    view! {
        <DocPage title="ListBox Atoms">
            <p>
                "The listbox atoms render an unstyled, accessible listbox with the behavior of the "
                <Link href=routes::doc::listbox::Hook.materialize()>"listbox hooks"</Link>": "
                <Code inline=true>"ListBox"</Code>" holds the list state, and you render a "
                <Code inline=true>"ListBoxItem"</Code>" per option, optionally grouped in "
                <Code inline=true>"ListBoxSection"</Code>"s. See the "
                <Link href=routes::doc::Listbox.materialize()>"Listbox overview"</Link>" for concept guidance."
            </p>

            <Section title="Hooks Used">
                <DocTable headers=&["Atom", "Hooks"]>
                    <TableRow>
                        <TableCell><Code inline=true>"ListBox"</Code></TableCell>
                        <TableCell>
                            <Link href=format!("{}#use-list-state", routes::doc::Collections.materialize())>"use_list_state"</Link>
                            " (unless you pass a "<Code inline=true>"state"</Code>"), "
                            <Link href=hook_section("use-listbox")>"use_listbox"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"ListBoxItem"</Code>", "<Code inline=true>"ListBoxItems"</Code></TableCell>
                        <TableCell><Link href=hook_section("use-option")>"use_option"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"ListBoxItemLabel"</Code>", "<Code inline=true>"ListBoxItemDescription"</Code></TableCell>
                        <TableCell>
                            "The "<Code inline=true>"label_props"</Code>" and "<Code inline=true>"description_props"</Code>
                            " slots of "<Link href=hook_section("use-option")>"use_option"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"ListBoxSection"</Code></TableCell>
                        <TableCell><Link href=hook_section("use-listbox-section")>"use_listbox_section"</Link></TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Example">
                <p>
                    "Describe the options as a collection, then render a "<Code inline=true>"ListBoxItem"</Code>" per option, "
                    "in collection order. Items name their option by key:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            atoms::listbox::{ListBox, ListBoxItem},
                            hooks::{SelectionMode, collections::{Key, Selection, use_list_collection}},
                        };

                        let fruits = use_list_collection(
                            Signal::stored(vec!["Apple", "Banana", "Cherry"]),
                            |fruit| Key::from(*fruit),
                            |fruit| (*fruit).to_owned(),
                        );

                        view! {
                            <ListBox
                                collection=fruits
                                selection_mode=SelectionMode::Multiple
                                aria_label="Fruits"
                                on_selection_change=Callback::new(|selection: Selection| log!("{selection:?}"))
                            >
                                <ListBoxItem key="Apple">"Apple"</ListBoxItem>
                                <ListBoxItem key="Banana">"Banana"</ListBoxItem>
                                <ListBoxItem key="Cherry">"Cherry"</ListBoxItem>
                            </ListBox>
                        }
                    "#)}
                </Code>
                <p>
                    "For a flat collection, "<Code inline=true>"ListBoxItems"</Code>" renders the items for you:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        <ListBox collection=fruits aria_label="Fruits">
                            <ListBoxItems let:node>{node.text_value.to_string()}</ListBoxItems>
                        </ListBox>
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "Pizza toppings in three sections, with multiple selection. Each option has a label and a description "
                    "(its price). Tab into the list and use the arrow keys, "<Keys keys="Space"/>" to toggle an option, "
                    <Keys keys="Shift"/>" + arrow keys to extend the selection, or type a letter to jump to a topping. "
                    "The anchovies are sold out: disabled in the collection, so they can\u{2019}t be focused or selected. "
                    "The demo is styled through the data attributes listed below. The selection lives in an "
                    <Code inline=true>"RwSignal<Selection>"</Code>" of the demo, bound with "<Code inline=true>"selection"</Code>
                    ": the list shows it, selecting writes it, and \u{201c}Clear\u{201d} empties it. "
                    <Code inline=true>"Selection::default()"</Code>" selects nothing, "<Code inline=true>"Selection::All"</Code>
                    " every selectable option. ""A signal pair "<Code inline=true>"(read, write)"</Code>" works as well, and "
                    <Code inline=true>"ValueBinding::new(signal, callback)"</Code>" binds any other storage."
                </p>
                <Demo
                    description="Pizza topping listbox with sections, multiple selection, option descriptions and a disabled option"
                    source=include_str!("demos/listbox.rs")
                >
                    <ListBoxAtomDemo/>
                </Demo>
            </Section>

            <Section title="ListBox">
                <p>
                    "Creates the list state from "<Code inline=true>"collection"</Code>" and the selection props, and renders "
                    "the listbox "<Code inline=true>"<div>"</Code>". Its children are the listbox\u{2019}s items and sections."
                </p>
                <p>
                    "Inside a "<Link href=routes::doc::select::Atom.materialize()>"Select"</Link>" or "
                    <Link href=routes::doc::combobox::Atom.materialize()>"ComboBox"</Link>" popover, the listbox shows the "
                    "options of the select or combobox with its settings; all props apart from "<Code inline=true>"classes"</Code>
                    ", "<Code inline=true>"styles"</Code>" and "<Code inline=true>"children"</Code>" are then ignored."
                </p>
                <Section title="Props" id="listbox-props">
                    <ApiTable kind=ApiKind::Props of="ListBox">
                        <ApiRow name="collection" ty="Option<CollectionMemo>" default="None">
                            "The options, from "<Code inline=true>"use_collection"</Code>" or "
                            <Code inline=true>"use_list_collection"</Code>". Required unless "<Code inline=true>"state"</Code>
                            " is given or the listbox is inside a select or combobox."
                        </ApiRow>
                        <ApiRow name="state" ty="Option<ListState>" default="None">
                            "Use an existing list state, e.g. to change the selection from outside. "
                            <Code inline=true>"collection"</Code>" and the selection props are then ignored."
                        </ApiRow>
                        <ApiRow name="selection_mode" ty="Signal<SelectionMode>" default="None">
                            "Whether nothing, one or many options can be selected. "<Code inline=true>"None"</Code>
                            " is a list without selection."
                        </ApiRow>
                        <ApiRow name="selection_behavior" ty="SelectionBehavior" default="Toggle">
                            "How pointer presses change the selection: "<Code inline=true>"Toggle"</Code>" the option, or "
                            <Code inline=true>"Replace"</Code>" the selection with it."
                        </ApiRow>
                        <ApiRow name="default_selected_keys" ty="Vec<Key>" default="vec![]">"The initially selected options."</ApiRow>
                        <ApiRow name="selection" ty="Option<ValueBinding<Selection>>" default="None">
                            "The selection as app state (e.g. an "<Code inline=true>"RwSignal<Selection>"</Code>"), replacing "<Code inline=true>"default_selected_keys"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_selection_change" ty="Option<Callback<Selection>>" default="None">
                            "Called with the new selection. Selecting all ("<Keys keys="Control + A"/>") reports "
                            <Code inline=true>"Selection::All"</Code>"."
                        </ApiRow>
                        <ApiRow name="disabled_keys" ty="Option<Signal<HashSet<Key>>>" default="None">
                            "Options that are disabled, besides those disabled in the collection."
                        </ApiRow>
                        <ApiRow name="disabled_behavior" ty="DisabledBehavior" default="All">
                            <Code inline=true>"All"</Code>": disabled options can\u{2019}t be focused or selected. "
                            <Code inline=true>"Selection"</Code>": they can be focused and trigger "<Code inline=true>"on_action"</Code>
                            ", but not be selected."
                        </ApiRow>
                        <ApiRow name="disallow_empty_selection" ty="bool" default="false">
                            "Prevent deselecting the last selected option."
                        </ApiRow>
                        <ApiRow name="aria_label, aria_labelledby" ty="MaybeProp<String>, Option<String>" default="None">
                            "The accessible name. The atom renders no label: give either a label text or the id of a visible label."
                        </ApiRow>
                        <ApiRow name="orientation" ty="Orientation" default="Vertical">
                            "Which arrow keys move between options."
                        </ApiRow>
                        <ApiRow name="layout" ty="ListLayout" default="Stack">
                            <Code inline=true>"Stack"</Code>": one option per row. "<Code inline=true>"Grid"</Code>
                            ": options wrap into rows, and up and down find the option in the same column."
                        </ApiRow>
                        <ApiRow name="should_focus_wrap" ty="bool" default="false">"Arrow keys wrap around at the ends."</ApiRow>
                        <ApiRow name="auto_focus" ty="Option<AutoFocus>" default="None">
                            "Focus an option when the listbox mounts: the "<Code inline=true>"Selected"</Code>", "
                            <Code inline=true>"First"</Code>" or "<Code inline=true>"Last"</Code>" one."
                        </ApiRow>
                        <ApiRow name="escape_key_behavior" ty="EscapeKeyBehavior" default="ClearSelection">
                            <Code inline=true>"None"</Code>" leaves "<Keys keys="Escape"/>" alone, e.g. to close a surrounding dialog."
                        </ApiRow>
                        <ApiRow name="on_action" ty="Option<Callback<Key>>" default="None">
                            "Called with the key of an activated option: pressed without a selection mode, double-clicked, or "
                            <Keys keys="Enter"/>" in "<Code inline=true>"Replace"</Code>" selection behavior."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Classes and styles of the listbox "<Code inline=true>"<div>"</Code>"."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">
                            "The options and sections, one per collection entry, in collection order."
                        </ApiRow>
                    </ApiTable>
                    <p>
                        "See the "<Link href=hook_section("collections")>"Collections"</Link>" section of the hook page for "
                        "how the selection settings behave."
                    </p>
                </Section>
            </Section>

            <Section title="ListBoxItem">
                <p>
                    "An option, rendered as a "<Code inline=true>"<div>"</Code>" with "<Code inline=true>"role=\"option\""</Code>
                    ". Its text, disabled state and link come from the collection item with its key; its content is up to you."
                </p>
                <Section title="Props" id="listbox-item-props">
                    <ApiTable kind=ApiKind::Props of="ListBoxItem">
                        <ApiRow name="key" ty="Key">"The option\u{2019}s key in the listbox\u{2019}s collection. Required."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Classes and styles of the option "<Code inline=true>"<div>"</Code>"."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">"The option\u{2019}s content."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ListBoxItems">
                <p>
                    "Renders one "<Code inline=true>"ListBoxItem"</Code>" per item of the listbox\u{2019}s collection, as it "
                    "currently is (inside a combobox: the options matching the input). Its children render an item\u{2019}s "
                    "content from its collection "<Code inline=true>"Node"</Code>" ("<Code inline=true>"let:node"</Code>"). "
                    "It renders no sections: for a collection with sections, render "<Code inline=true>"ListBoxSection"</Code>
                    "s yourself."
                </p>
                <Section title="Props" id="listbox-items-props">
                    <ApiTable kind=ApiKind::Props of="ListBoxItems">
                        <ApiRow name="children" ty="Fn(Node) -> impl IntoView">
                            "Renders an item\u{2019}s content. The node has the item\u{2019}s "<Code inline=true>"key"</Code>" and "
                            <Code inline=true>"text_value"</Code>"; look up richer data by the key."
                        </ApiRow>
                        <ApiRow name="classes" ty="Classes" default="empty">"Classes of each item."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ListBoxItemLabel">
                <p>
                    "The main text of an option, as a "<Code inline=true>"<span>"</Code>". While it is rendered, it labels the "
                    "option ("<Code inline=true>"aria-labelledby"</Code>"), so that additional content like a description "
                    "isn\u{2019}t part of the option\u{2019}s name. Use at most one per option."
                </p>
                <Section title="Props" id="listbox-item-label-props">
                    <ApiTable kind=ApiKind::Props of="ListBoxItemLabel">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Classes and styles of the "<Code inline=true>"<span>"</Code>"."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">"The label text."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ListBoxItemDescription">
                <p>
                    "Secondary text of an option, as a "<Code inline=true>"<span>"</Code>". While it is rendered, it describes "
                    "the option ("<Code inline=true>"aria-describedby"</Code>"). Use at most one per option."
                </p>
                <Section title="Props" id="listbox-item-description-props">
                    <ApiTable kind=ApiKind::Props of="ListBoxItemDescription">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Classes and styles of the "<Code inline=true>"<span>"</Code>"."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">"The description text."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ListBoxSection">
                <p>
                    "A group of options, for a section of the collection. It renders a "<Code inline=true>"<div>"</Code>
                    " with "<Code inline=true>"role=\"presentation\""</Code>" containing the section\u{2019}s heading (if the "
                    "collection section has a header) and a "<Code inline=true>"role=\"group\""</Code>" "
                    <Code inline=true>"<div>"</Code>" with the children, labelled by the heading:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        let produce = use_collection(|b| {
                            b.section("fruit", |s| {
                                s.header("fruit-heading", "Fruit");
                                s.item("apple", "Apple");
                                s.item("banana", "Banana");
                            });
                            b.section("vegetables", |s| {
                                s.header("vegetables-heading", "Vegetables");
                                s.item("carrot", "Carrot");
                            });
                        });

                        view! {
                            <ListBox collection=produce aria_label="Produce">
                                <ListBoxSection key="fruit">
                                    <ListBoxItem key="apple">"Apple"</ListBoxItem>
                                    <ListBoxItem key="banana">"Banana"</ListBoxItem>
                                </ListBoxSection>
                                <ListBoxSection key="vegetables">
                                    <ListBoxItem key="carrot">"Carrot"</ListBoxItem>
                                </ListBoxSection>
                            </ListBox>
                        }
                    "#)}
                </Code>
                <p>
                    "A section without a header needs an accessible name: "
                    <Code inline=true>"b.section(..).aria_label(\"Fruit\")"</Code>"."
                </p>
                <Section title="Props" id="listbox-section-props">
                    <ApiTable kind=ApiKind::Props of="ListBoxSection">
                        <ApiRow name="key" ty="Key">"The section\u{2019}s key in the listbox\u{2019}s collection. Required."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Classes and styles of the group "<Code inline=true>"<div>"</Code>" holding the options."
                        </ApiRow>
                        <ApiRow name="heading_classes" ty="Classes" default="empty">
                            "Classes of the heading "<Code inline=true>"<div>"</Code>"."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">"The section\u{2019}s options, in collection order."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <p>
                    "Rendered on "<Code inline=true>"ListBoxItem"</Code>" as "<Code inline=true>"data-selected=\"true\""</Code>
                    " while the state applies, and absent otherwise."
                </p>
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-selected" ty="true">"The option is selected."</ApiRow>
                    <ApiRow name="data-focused" ty="true">"The option has focus."</ApiRow>
                    <ApiRow name="data-focus-visible" ty="true">
                        "The option has focus and focus should be shown (keyboard navigation)."
                    </ApiRow>
                    <ApiRow name="data-disabled" ty="true">"The option is disabled."</ApiRow>
                    <ApiRow name="data-pressed" ty="true">"The option is being pressed."</ApiRow>
                </ApiTable>
                <p>
                    "The ARIA attributes are there to style as well: "<Code inline=true>"aria-selected"</Code>" and "
                    <Code inline=true>"aria-disabled"</Code>" on options, "<Code inline=true>"aria-multiselectable"</Code>
                    " on the listbox, "<Code inline=true>"role=\"group\""</Code>" on section groups."
                </p>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms bring no styles. Pass "<Code inline=true>"classes"</Code>" and target the state with attribute "
                    "selectors. Options receive DOM focus; style keyboard focus through "
                    <Code inline=true>"[data-focus-visible]"</Code>", which follows leptonic\u{2019}s input modality "
                    "tracking. The leptonic theme draws a generic outline around every "
                    <Code inline=true>"[data-focus-visible]"</Code>" element; give your options their own outline, so that "
                    "a scrolling listbox doesn\u{2019}t clip it:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .my-option[data-focus-visible] { outline: 2px solid royalblue; outline-offset: -2px; }
                        .my-option[data-selected] { background: #eef; font-weight: bold; }
                        .my-option[data-disabled] { opacity: 0.5; }
                    ")}
                </Code>
            </Section>

            <Section title="Composition">
                <p>
                    <Code inline=true>"ListBoxItem"</Code>" provides its state as "<Code inline=true>"ListBoxItemCtx"</Code>
                    " context ("<Code inline=true>"is_selected"</Code>", "<Code inline=true>"is_focused"</Code>", "
                    <Code inline=true>"is_focus_visible"</Code>", "<Code inline=true>"is_disabled"</Code>", "
                    <Code inline=true>"is_pressed"</Code>"). Components inside an option can read it, e.g. a check mark:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        #[component]
                        fn CheckMark() -> impl IntoView {
                            let item = expect_context::<ListBoxItemCtx>();
                            move || item.is_selected.get().then_some("\u{2713}")
                        }
                    "#)}
                </Code>
                <p>
                    <Code inline=true>"ListBox"</Code>" provides "<Code inline=true>"ListBoxData"</Code>", whose "
                    <Code inline=true>"state"</Code>" is the "<Code inline=true>"ListState"</Code>" documented under "
                    <Link href=format!("{}#liststate", routes::doc::Collections.materialize())>"Collections"</Link>
                    ". Your own options can call "<Link href=hook_section("use-option")>"use_option"</Link>" with it."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Listbox.materialize()>"Listbox overview"</Link></li>
                <li><Link href=routes::doc::listbox::Hook.materialize()>"Listbox hooks"</Link></li>
                <li><Link href=routes::doc::select::Atom.materialize()>"Select atoms"</Link></li>
                <li><Link href=routes::doc::combobox::Atom.materialize()>"Combobox atoms"</Link></li>
                <li><Link href=routes::doc::Collections.materialize()>"Collections"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

/// A link target on the listbox hook page.
fn hook_section(id: &str) -> String {
    format!("{}#{id}", routes::doc::listbox::Hook.materialize())
}

use indoc::indoc;
use leptos::prelude::*;

use super::demos::listbox::ListBoxAtomDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomListBox() -> impl IntoView {
    view! {
        <DocPage title="Listbox Atoms">
            <p>
                "The listbox atoms render an unstyled listbox with its options and sections. See the "
                <Link href=routes::doc::Listbox.materialize()>"Listbox overview"</Link>" for concept guidance."
            </p>

            <Section title="Hooks Used">
                <DocTable headers=&["Atom", "Hooks"]>
                    <TableRow>
                        <TableCell><Code inline=true>"ListBox"</Code></TableCell>
                        <TableCell>
                            <Link href=format!("{}#use-list-state", routes::doc::CollectionState.materialize())>"use_list_state"</Link>
                            ", "
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
                        <TableCell><Code inline=true>"ListBoxSection"</Code>", "<Code inline=true>"ListBoxSectionHeading"</Code></TableCell>
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
                            hooks::{Key, SelectionMode, collections::Selection, use_list_collection},
                        };
                        use leptos::{logging::log, prelude::*};

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
                    "The selection lives in app state, passed as "<Code inline=true>"selection"</Code>" and "
                    <Code inline=true>"set_selection"</Code>": \u{201c}Clear\u{201d} empties it."
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
                            <Code inline=true>"use_list_collection"</Code>". Required unless the listbox is inside a select or "
                            "combobox, which provides it: without either, the listbox panics."
                        </ApiRow>
                        <ApiRow name="selection_mode" ty="Signal<SelectionMode>" default="None">
                            "Whether nothing, one or many options can be selected. "<Code inline=true>"None"</Code>
                            " is a list without selection."
                        </ApiRow>
                        <ApiRow name="selection_behavior" ty="Signal<SelectionBehavior>" default="Toggle">
                            "How pointer presses change the selection: "<Code inline=true>"Toggle"</Code>" the option, or "
                            <Code inline=true>"Replace"</Code>" the selection with it."
                        </ApiRow>
                        <ApiRow name="default_selected_keys" ty="Vec<Key>" default="vec![]">"The initially selected options."</ApiRow>
                        <ApiRow name="selection" ty="Option<Signal<Selection>>" default="None">
                            "The selection (controlled), replacing "<Code inline=true>"default_selected_keys"</Code>": a value or any signal."
                        </ApiRow>
                        <ApiRow name="set_selection" ty="Option<Out<Selection>>" default="None">
                            "Receives the new state: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="on_selection_change" ty="Option<Callback<Selection>>" default="None">
                            "Called with the new selection. Selecting all ("<Keys keys="Control + A"/>", "<Keys keys="Meta + A"/>" on macOS) reports "
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
                        <ApiRow name="empty_state" ty="Option<ViewFn>" default="None">
                            "Shown while there are no options (also inside a "<Code inline=true>"Select"</Code>" or "
                            <Code inline=true>"ComboBox"</Code>"), in a "<Code inline=true>"role=\"option\""</Code>" element with "
                            <Code inline=true>"display: contents"</Code>"."
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
                <p>
                    "Inside a "<Link href=routes::doc::collection_state::Virtualizer.materialize()>"Virtualizer"</Link>
                    ", it renders only the options in view, for listboxes of thousands of options."
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
                    "A group of options, for a section of the collection: one "<Code inline=true>"<section>"</Code>" with "
                    <Code inline=true>"role=\"group\""</Code>", holding its children. Render a "<Code inline=true>"ListBoxSectionHeading"</Code>
                    " in it when the collection section has a header; it labels the group:"
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
                                    <ListBoxSectionHeading/>
                                    <ListBoxItem key="apple">"Apple"</ListBoxItem>
                                    <ListBoxItem key="banana">"Banana"</ListBoxItem>
                                </ListBoxSection>
                                <ListBoxSection key="vegetables">
                                    <ListBoxSectionHeading/>
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
                            "Classes and styles of the "<Code inline=true>"<section>"</Code>"."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">
                            "The section\u{2019}s "<Code inline=true>"ListBoxSectionHeading"</Code>" and options, in collection order."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ListBoxSectionHeading">
                <p>
                    "The heading of a "<Code inline=true>"ListBoxSection"</Code>": a "<Code inline=true>"<header>"</Code>" naming "
                    "the group. It shows its children, or else the section\u{2019}s header text from the collection. Assistive "
                    "technology doesn\u{2019}t see it as a heading (a listbox can\u{2019}t contain headings), only as the "
                    "group\u{2019}s name. Render one per section, only for sections with a header."
                </p>
                <Section title="Props" id="listbox-section-heading-props">
                    <ApiTable kind=ApiKind::Props of="ListBoxSectionHeading">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Classes and styles of the "<Code inline=true>"<header>"</Code>"."
                        </ApiRow>
                        <ApiRow name="children" ty="Option<Children>" default="None">
                            "The heading\u{2019}s content. "<Code inline=true>"None"</Code>": the collection\u{2019}s header text."
                        </ApiRow>
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
                    "On "<Code inline=true>"ListBox"</Code>": "<Code inline=true>"data-empty"</Code>" (no options), "<Code inline=true>"data-focused"</Code>" and "<Code inline=true>"data-focus-visible"</Code>
                    " (the listbox itself has the focus, which it only takes while empty), "<Code inline=true>"data-layout"</Code>" ("<Code inline=true>"stack"</Code>" or "
                    <Code inline=true>"grid"</Code>") and "<Code inline=true>"data-orientation"</Code>" ("<Code inline=true>"vertical"</Code>" or "<Code inline=true>"horizontal"</Code>")."
                </p>
                <p>
                    "The ARIA attributes are there to style as well: "<Code inline=true>"aria-selected"</Code>" and "
                    <Code inline=true>"aria-disabled"</Code>" on options, "<Code inline=true>"aria-multiselectable"</Code>
                    " on the listbox, "<Code inline=true>"role=\"group\""</Code>" on section groups."
                </p>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms bring no styles. "<Code inline=true>"ListBox"</Code>" renders a "<Code inline=true>"<div>"</Code>" with the class "<Code inline=true>"leptonic-ListBox"</Code>", "
                    <Code inline=true>"ListBoxItem"</Code>" one with "<Code inline=true>"leptonic-ListBoxItem"</Code>" (its "<Code inline=true>"ListBoxItemLabel"</Code>" and "
                    <Code inline=true>"ListBoxItemDescription"</Code>" are "<Code inline=true>"<span>"</Code>"s with "<Code inline=true>"leptonic-ListBoxItemLabel"</Code>" and "
                    <Code inline=true>"leptonic-ListBoxItemDescription"</Code>"), and "<Code inline=true>"ListBoxSection"</Code>" a group with "
                    <Code inline=true>"leptonic-ListBoxSection"</Code>" around its "<Code inline=true>"ListBoxSectionHeading"</Code>" ("
                    <Code inline=true>"leptonic-ListBoxSectionHeading"</Code>"). Your "<Code inline=true>"classes"</Code>" follow the default class. Anything else in an option, like the "
                    "demo\u{2019}s check box, is your own markup: mark it "<Code inline=true>"aria-hidden"</Code>" and style it through the option\u{2019}s "
                    "data attributes. The demo above uses this CSS:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .demo-lb-listbox { padding: 0.25rem; border: 1px solid var(--border); border-radius: 8px; background: var(--surface); }
                        .demo-lb-listbox[data-focus-visible] { outline: 2px solid var(--focus); outline-offset: 2px; }
                        .demo-lb-heading { padding: 0.5rem 0.5rem 0.25rem; color: var(--muted); font-size: 0.75rem; text-transform: uppercase; }
                        .demo-lb-item { display: flex; align-items: center; gap: 0.5rem; padding: 0.5rem; border-radius: 8px; cursor: pointer; }
                        .demo-lb-item:hover:not([data-disabled]), .demo-lb-item[data-pressed] { background: var(--border); }
                        .demo-lb-item[data-focus-visible] { outline: 2px solid var(--focus); outline-offset: -2px; }
                        .demo-lb-item[data-disabled] { opacity: 0.5; cursor: not-allowed; }
                        .demo-lb-check { width: 1em; height: 1em; border: 1px solid var(--muted); border-radius: 4px; }
                        [data-selected] > .demo-lb-check { border-color: var(--accent); background: var(--accent); }
                        .demo-lb-price { margin-left: auto; color: var(--muted); }
                    ")}
                </Code>
                <p>
                    "Apps that don\u{2019}t want to style from scratch can load leptonic\u{2019}s optional atom theme, "
                    <Code inline=true>"@use \"leptonic/leptonic-atoms\";"</Code>", which styles the default classes."
                </p>
            </Section>

            <Section title="Composition">
                <p>
                    "A picker of your own that shows its options in a "<Code inline=true>"ListBox"</Code>" (like the select\u{2019}s "
                    "and the combobox\u{2019}s popovers) provides "<Code inline=true>"ListBoxParent { input }"</Code>
                    " as context: the listbox then uses that configuration instead of its own props."
                </p>
                <p>
                    <Code inline=true>"ListBoxItem"</Code>" provides its state as "<Code inline=true>"ListBoxItemCtx"</Code>
                    " context ("<Code inline=true>"is_selected"</Code>", "<Code inline=true>"is_focused"</Code>", "
                    <Code inline=true>"is_focus_visible"</Code>", "<Code inline=true>"is_disabled"</Code>", "
                    <Code inline=true>"is_pressed"</Code>", "<Code inline=true>"is_hovered"</Code>"). Leptos components inside an option can read it, e.g. a check mark:"
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
                    <Link href=format!("{}#liststate", routes::doc::CollectionState.materialize())>"Collection State"</Link>
                    ". Your own options can call "<Link href=hook_section("use-option")>"use_option"</Link>" with it."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Listbox.materialize()>"Listbox overview"</Link></li>
                <li><Link href=routes::doc::listbox::Hook.materialize()>"Listbox Hooks"</Link></li>
                <li><Link href=routes::doc::select::Atom.materialize()>"Select Atoms"</Link></li>
                <li><Link href=routes::doc::combobox::Atom.materialize()>"Combobox Atoms"</Link></li>
                <li><Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link></li>
                <li><Link href=routes::doc::collection_state::Virtualizer.materialize()>"Virtualizer"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

/// A link target on the listbox hook page.
fn hook_section(id: &str) -> String {
    format!("{}#{id}", routes::doc::listbox::Hook.materialize())
}

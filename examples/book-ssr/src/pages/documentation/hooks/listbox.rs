use indoc::indoc;
use leptos::prelude::*;

use super::demos::listbox::ListboxDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseListbox() -> impl IntoView {
    view! {
        <DocPage title="Listbox Hooks">
            <p>
                "The listbox hooks build a listbox, its options and sections from your own markup. See the "
                <Link href=routes::doc::Listbox.materialize()>"Listbox overview"</Link>" for concept guidance and "
                "keyboard interaction."
            </p>

            <ReactAria hook="useListBox"/>

            <Section title="Demo">
                <p>
                    "A multiple-selection listbox built from the hooks. Click it or tab into it, then navigate with the arrow "
                    "keys, select with "<Keys keys="Space"/>", or type a letter to jump to an option. \u{201c}Date\u{201d} is disabled."
                </p>

                <Demo
                    description="Multiple-selection fruit listbox with a disabled option"
                    source=include_str!("demos/listbox.rs")
                >
                    <ListboxDemo/>
                </Demo>
            </Section>

            <Section title="Collections">
                <p>
                    "A listbox doesn\u{2019}t discover its options from what you render. You describe them up front as a "
                    <em>"collection"</em>": every option has a "<Code inline=true>"Key"</Code>" that identifies it and a text used "
                    "for type-ahead. Because the collection is plain data, the server renders exactly the listbox the browser "
                    "hydrates, and keyboard navigation knows about every option."
                </p>

                <p>
                    "Build a flat collection from a list of values with "
                    <Link href=format!("{}#use-list-collection", routes::doc::CollectionState.materialize())>
                        <Code inline=true>"use_list_collection"</Code>
                    </Link>", or use "
                    <Link href=format!("{}#use-collection", routes::doc::CollectionState.materialize())>
                        <Code inline=true>"use_collection"</Code>
                    </Link>" for sections, headers and disabled items. Then create the list state with "
                    <Link href=format!("{}#use-list-state", routes::doc::CollectionState.materialize())>
                        <Code inline=true>"use_list_state"</Code>
                    </Link>", which owns the selection and the focused option."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::hooks::collections::{Key, SelectionMode, SelectionOptions, UseListCollectionInput, UseListStateInput, use_list_collection, use_list_state};
                        use leptos::prelude::*;

                        let fruits = Signal::stored(vec![("apple", "Apple"), ("banana", "Banana")]);
                        let collection = use_list_collection(UseListCollectionInput {
                            items: fruits,
                            key: |(key, _)| Key::from(*key),
                            text_value: |(_, label)| (*label).to_owned(),
                        });
                        let state = use_list_state(UseListStateInput {
                            collection,
                            selection: SelectionOptions {
                                selection_mode: Signal::stored(SelectionMode::Multiple),
                                ..SelectionOptions::default()
                            },
                        });
                    "#)}
                </Code>

                <p>
                    "Render the options from the same data, in the same order. Read the selection with "
                    <Code inline=true>"state.selection.selected_keys()"</Code>", change it with methods like "
                    <Code inline=true>"set_selected_keys"</Code>", or bind it to app state with "
                    <Code inline=true>"selection"</Code>". "<Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link>
                    " documents the collection builders, "
                    <Link href=format!("{}#selectionoptions", routes::doc::CollectionState.materialize())>"SelectionOptions"</Link>
                    " and the list state."
                </p>

            </Section>

            <Section title="use_listbox">
                <p>
                    "Turns an element into the listbox for a list state. Besides the element\u{2019}s props, it returns "
                    <Code inline=true>"ListBoxData"</Code>": everything the options need to know about their listbox. Hand it "
                    "to each option and section."
                </p>

                <Section title="Input" id="use-listbox-input">
                    <p>
                        "Pass a "<Code inline=true>"UseListBoxInput"</Code>" with every field named; the Default column gives "
                        "the value for fields you don\u{2019}t need."
                    </p>

                    <ApiTable kind=ApiKind::Input of="UseListBoxInput">
                        <ApiRow name="state" ty="ListState">"The list state from "<Code inline=true>"use_list_state"</Code>". Required."</ApiRow>
                        <ApiRow name="element" ty="CapturedElement">"The listbox element; the hook\u{2019}s props capture it. Required."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">
                            "The element id, generated when "<Code inline=true>"None"</Code>". Set it when another element references "
                            "the listbox, e.g. a select trigger\u{2019}s "<Code inline=true>"aria-controls"</Code>"."
                        </ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"An accessible name for the listbox."</ApiRow>
                        <ApiRow name="aria_labelledby" ty="Signal<Option<String>>" default="None">
                            "The id of a visible label. The hook renders no label itself."
                        </ApiRow>
                        <ApiRow name="orientation" ty="Signal<Orientation>" default="Vertical">
                            "Decides which arrow keys move between options; set as "<Code inline=true>"aria-orientation"</Code>"."
                        </ApiRow>
                        <ApiRow name="layout" ty="ListLayout" default="Stack">
                            <Code inline=true>"Stack"</Code>": one option per row (or column). "<Code inline=true>"Grid"</Code>
                            ": options wrap into rows, and up and down find the option in the same column."
                        </ApiRow>
                        <ApiRow name="keyboard_delegate" ty="Option<Signal<Arc<dyn KeyboardDelegate>>>" default="None">
                            "Replaces the list keyboard navigation."
                        </ApiRow>
                        <ApiRow name="layout_delegate" ty="Option<Arc<dyn LayoutDelegate>>" default="None">
                            "Where options are on screen, for paging and scrolling to the focused option when only the "
                            "visible ones are rendered (virtualized): the "<Code inline=true>"layout_delegate()"</Code>" of "
                            <Link href=routes::doc::collection_state::UseVirtualizerState.materialize()>"use_virtualizer_state"</Link>
                            ". Default: the rendered option elements."
                        </ApiRow>
                        <ApiRow name="is_virtualized" ty="bool" default="false">
                            "Only the visible options are rendered: each option then announces its position ("
                            <Code inline=true>"aria-posinset"</Code>", "<Code inline=true>"aria-setsize"</Code>"). See "
                            <Link href=format!("{}#virtualizing-a-collection-hook", routes::doc::collection_state::UseVirtualizerState.materialize())>"Virtualizing a Collection Hook"</Link>"."
                        </ApiRow>
                        <ApiRow name="options" ty="CollectionOptions" default="CollectionOptions::default()">
                            "Keyboard and focus behavior, see "<Link href=format!("{}#collectionoptions", routes::doc::CollectionState.materialize())>"CollectionOptions"</Link>"."
                        </ApiRow>
                        <ApiRow name="should_select_on_press_up" ty="bool" default="false">
                            "Select when the press ends instead of when it starts, as in select popovers."
                        </ApiRow>
                        <ApiRow name="should_focus_on_hover" ty="bool" default="false">
                            "Focus options when the pointer moves over them, as in select popovers."
                        </ApiRow>
                        <ApiRow name="on_action" ty="Option<Callback<Key>>" default="None">
                            "Called with the key of an activated option: pressed without a selection mode, double-clicked, or "
                            <Keys keys="Enter"/>" in "<Code inline=true>"Replace"</Code>" selection behavior."
                        </ApiRow>
                        <ApiRow name="on_focus, on_blur" ty="Option<Callback<FocusWithinEvent>>" default="None">
                            "Called when focus moves into the listbox from outside, or leaves it."
                        </ApiRow>
                        <ApiRow name="on_focus_change" ty="Option<Callback<bool>>" default="None">
                            "Called with whether focus is within the listbox."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-listbox-return">
                    <ApiTable kind=ApiKind::Return of="UseListBoxReturn">
                        <ApiRow name="props" ty="UseListBoxProps">
                            "Role, labels, "<Code inline=true>"aria-multiselectable"</Code>" (in multiple selection mode), "
                            <Code inline=true>"aria-orientation"</Code>", and the keyboard and focus handlers. Spread "
                            <Code inline=true>"{..props.into_attrs()}"</Code>"."
                        </ApiRow>
                        <ApiRow name="data" ty="ListBoxData">
                            "What options and sections need to know about the listbox. Pass a clone to each of them."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-listbox-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            let UseListBoxReturn { props, data } = use_listbox(UseListBoxInput {
                                state,
                                element: CapturedElement::new(),
                                id: None,
                                aria_label: "Fruits".into(),
                                aria_labelledby: Signal::stored(None),
                                orientation: Orientation::Vertical.into(),
                                layout: ListLayout::Stack,
                                keyboard_delegate: None,
                                layout_delegate: None,
                                is_virtualized: false,
                                options: CollectionOptions {
                                    should_focus_wrap: true,
                                    ..CollectionOptions::default()
                                },
                                should_select_on_press_up: false,
                                should_focus_on_hover: false,
                                on_action: None,
                                on_focus: None,
                                on_blur: None,
                                on_focus_change: None,
                            });

                            view! {
                                <div {..props.into_attrs()}>
                                    // One option per collection item, in collection order.
                                </div>
                            }
                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="use_option">
                <p>
                    "Renders one option. It only needs the listbox data and the option\u{2019}s key. Whether it is "
                    "selected, focused or disabled, its text and its link all come from the list state and the collection. "
                    "It handles selection on press ("<Keys keys="Shift"/>" + click selects a range, "<Keys keys="Control"/>
                    " + click, or "<Keys keys="Meta"/>" + click on macOS, an individual option), focus and hover."
                </p>

                <Section title="Input" id="use-option-input">
                    <ApiTable kind=ApiKind::Input of="UseOptionInput">
                        <ApiRow name="list" ty="ListBoxData">"The listbox, from "<Code inline=true>"use_listbox"</Code>". Required."</ApiRow>
                        <ApiRow name="key" ty="Key">"The option\u{2019}s key in the listbox\u{2019}s collection. Required."</ApiRow>
                        <ApiRow name="on_context_menu" ty="Option<Callback<ContextMenuEvent>>" default="None">
                            "Called when a context menu is requested on the option (right click, "<Keys keys="Shift + F10"/>", the context menu key, a long press on iOS); the option\u{2019}s menu then replaces the browser\u{2019}s."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-option-return">
                    <ApiTable kind=ApiKind::Return of="UseOptionReturn">
                        <ApiRow name="props" ty="PropsWithStyles<UseOptionProps>">
                            <Code inline=true>"role=\"option\""</Code>", "<Code inline=true>"aria-selected"</Code>" (unless the "
                            "selection mode is "<Code inline=true>"None"</Code>"), "<Code inline=true>"aria-disabled"</Code>
                            ", press, focus and hover handling. Call "<Code inline=true>"props.into_parts()"</Code>" to get "
                            <Code inline=true>"(attrs, styles)"</Code>"."
                        </ApiRow>
                        <ApiRow name="label_props, description_props" ty="SlotProps">
                            "For optional elements holding the option\u{2019}s main and secondary text. The option only references "
                            "them ("<Code inline=true>"aria-labelledby"</Code>", "<Code inline=true>"aria-describedby"</Code>
                            ") while you render them."
                        </ApiRow>
                        <ApiRow name="is_selected" ty="Signal<bool>">"Whether the option is selected."</ApiRow>
                        <ApiRow name="is_focused" ty="Signal<bool>">"Whether the option has focus."</ApiRow>
                        <ApiRow name="is_focus_visible" ty="Signal<bool>">
                            "Focused, and focus should be shown (keyboard navigation). Use it for focus rings."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>">"Whether the option is disabled."</ApiRow>
                        <ApiRow name="is_pressed" ty="Signal<bool>">"Whether the option is being pressed."</ApiRow>
                        <ApiRow name="is_hovered" ty="Signal<bool>">
                            "Whether the pointer is over the option (options that can be selected or have an action, or that take "
                            "focus on hover)."
                        </ApiRow>
                        <ApiRow name="allows_selection" ty="Signal<bool>">"Whether pressing the option can select it."</ApiRow>
                        <ApiRow name="has_action" ty="Signal<bool>">"Whether the option has an action or link to perform."</ApiRow>
                        <ApiRow name="link" ty="Signal<Option<ItemLink>>">
                            "The option\u{2019}s link, if the collection item has one. Render the option as an "
                            <Code inline=true>"<a>"</Code>" with it, or keep another element: links then open through a temporary "
                            <Code inline=true>"<a>"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-option-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            let UseOptionReturn { props, is_selected, is_focus_visible, .. } =
                                use_option(UseOptionInput { list: data.clone(), key: Key::from("apple"), on_context_menu: None });

                            let (attrs, styles) = props.into_parts();

                            view! {
                                <div {..attrs} style=styles>"Apple"</div>
                            }
                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="use_listbox_section">
                <p>
                    "Groups options. The section, its heading and its label come from the collection, so you pass the "
                    "section\u{2019}s key."
                </p>

                <Section title="Input" id="use-listbox-section-input">
                    <ApiTable kind=ApiKind::Input of="UseListBoxSectionInput">
                        <ApiRow name="list" ty="ListBoxData">"The listbox, from "<Code inline=true>"use_listbox"</Code>". Required."</ApiRow>
                        <ApiRow name="key" ty="Key">"The section\u{2019}s key in the listbox\u{2019}s collection. Required."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-listbox-section-return">
                    <ApiTable kind=ApiKind::Return of="UseListBoxSectionReturn">
                        <ApiRow name="item_props" ty="UseListBoxSectionItemProps">
                            "For the element wrapping heading and group (e.g. an "<Code inline=true>"<li>"</Code>" in a "
                            <Code inline=true>"<ul>"</Code>" listbox): "<Code inline=true>"role=\"presentation\""</Code>"."
                        </ApiRow>
                        <ApiRow name="heading_props" ty="Option<UseListBoxSectionHeadingProps>">
                            "For the heading element; "<Code inline=true>"None"</Code>" when the section has no header. Clicking "
                            "the heading keeps focus in the listbox."
                        </ApiRow>
                        <ApiRow name="group_props" ty="UseListBoxSectionGroupProps">
                            "For the element containing the section\u{2019}s options: "<Code inline=true>"role=\"group\""</Code>
                            ", labelled by the heading or the section\u{2019}s "<Code inline=true>"aria_label"</Code>"."
                        </ApiRow>
                        <ApiRow name="heading" ty="Signal<Option<String>>">"The header text, if any."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-listbox-section-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            let collection = use_collection(|b| {
                                b.section("citrus", |s| {
                                    s.header("citrus-header", "Citrus fruits");
                                    s.item("lemon", "Lemon");
                                    s.item("orange", "Orange");
                                });
                            });

                            // Later, when rendering the section:
                            let UseListBoxSectionReturn { item_props, heading_props, group_props, heading } =
                                use_listbox_section(UseListBoxSectionInput {
                                    list: data.clone(),
                                    key: Key::from("citrus"),
                                });
                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="Selection Modes">
                <DocTable headers=&["SelectionMode", "Behavior"]>
                    <TableRow><TableCell><Code inline=true>"None"</Code></TableCell><TableCell>"A read-only list without selection. Options have no "<Code inline=true>"aria-selected"</Code>"."</TableCell></TableRow>
                    <TableRow><TableCell><Code inline=true>"Single"</Code></TableCell><TableCell>"Only one option can be selected."</TableCell></TableRow>
                    <TableRow><TableCell><Code inline=true>"Multiple"</Code></TableCell><TableCell>"Any number of options can be selected. Sets "<Code inline=true>"aria-multiselectable"</Code>"."</TableCell></TableRow>
                </DocTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Listbox.materialize()>"Listbox overview"</Link></li>
                <li><Link href=routes::doc::listbox::Atom.materialize()>"Listbox Atoms"</Link></li>
                <li><Link href=routes::doc::select::Hook.materialize()>"Select Hooks"</Link></li>
                <li><Link href=routes::doc::combobox::Hook.materialize()>"Combobox Hooks"</Link></li>
                <li><Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link></li>
                <li><Link href=routes::doc::collection_state::UseVirtualizerState.materialize()>"use_virtualizer_state"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

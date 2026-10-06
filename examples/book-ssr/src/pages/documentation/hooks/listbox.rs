use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::listbox::ListboxDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseListbox() -> impl IntoView {
    view! {
        <DocPage title="Listbox Hooks">
            <p>
                "The listbox hooks build an accessible listbox from your own markup: single or multiple selection, keyboard "
                "navigation and type-ahead. "<Code inline=true>"use_listbox"</Code>" handles the list, "
                <Code inline=true>"use_option"</Code>" each option and "<Code inline=true>"use_listbox_section"</Code>
                " groups of options. See the "<Link href=routes::doc::Listbox.materialize()>"Listbox overview"</Link>
                " for concept guidance."
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
                    "Build a flat collection from a list of values with "<Code inline=true>"use_list_collection"</Code>
                    ", or use "<Code inline=true>"use_collection"</Code>" for sections, headers and disabled items. "
                    "Then create the list state with "<Code inline=true>"use_list_state"</Code>
                    ", which owns the selection and the focused option."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        let fruits = Signal::stored(vec![("apple", "Apple"), ("banana", "Banana")]);
                        let collection = use_list_collection(
                            fruits,
                            |(key, _)| Key::from(*key),
                            |(_, label)| (*label).to_owned(),
                        );
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
                    <Code inline=true>"set_selected_keys"</Code>", or pass "<Code inline=true>"on_selection_change"</Code>
                    " to hear about every change. The rest of "<Code inline=true>"SelectionOptions"</Code>":"
                </p>

                <ApiTable kind=ApiKind::Input of="SelectionOptions">
                    <ApiRow name="selection_mode" ty="Signal<SelectionMode>" default="None">
                        "Whether nothing, one or many options can be selected, see "<a href="#selection-modes">"Selection Modes"</a>"."
                    </ApiRow>
                    <ApiRow name="selection_behavior" ty="SelectionBehavior" default="Toggle">
                        "How pointer presses change the selection: "<Code inline=true>"Toggle"</Code>" the option, or "
                        <Code inline=true>"Replace"</Code>" the selection with it."
                    </ApiRow>
                    <ApiRow name="default_selection" ty="Selection" default="empty">
                        "The initial selection: "<Code inline=true>"Selection::keys([..])"</Code>" or "<Code inline=true>"Selection::All"</Code>"."
                    </ApiRow>
                    <ApiRow name="selection" ty="Option<ValueBinding<Selection>>" default="None">
                        "The selection as app state ("<Code inline=true>"Some(rw_signal.into())"</Code>"), replacing "<Code inline=true>"default_selection"</Code>": the collection shows it, and selecting writes it."
                    </ApiRow>
                    <ApiRow name="on_selection_change" ty="Option<Callback<Selection>>" default="None">
                        "Called with the new selection whenever it changes."
                    </ApiRow>
                    <ApiRow name="disallow_empty_selection" ty="Signal<bool>" default="false">
                        "Prevent deselecting the last selected option."
                    </ApiRow>
                    <ApiRow name="disabled_keys" ty="Signal<HashSet<Key>>" default="empty">
                        "Options that can\u{2019}t be selected."
                    </ApiRow>
                    <ApiRow name="disabled_behavior" ty="DisabledBehavior" default="All">
                        <Code inline=true>"All"</Code>": disabled options can\u{2019}t be focused or used either. "
                        <Code inline=true>"Selection"</Code>": they can be focused and have actions, but can\u{2019}t be selected."
                    </ApiRow>
                    <ApiRow name="allow_duplicate_selection_events" ty="bool" default="false">
                        "Call "<Code inline=true>"on_selection_change"</Code>" even when an interaction leaves the selection unchanged."
                    </ApiRow>
                </ApiTable>

                <p>
                    <Link href=routes::doc::Collections.materialize()>"Collections"</Link>
                    " explains the selection state in more detail."
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
                        "Create the input with "<Code inline=true>"UseListBoxInput::new(state, element)"</Code>
                        " and change the fields you need with struct update syntax."
                    </p>

                    <ApiTable kind=ApiKind::Input of="UseListBoxInput">
                        <ApiRow name="state" ty="ListState">"The list state from "<Code inline=true>"use_list_state"</Code>". Required."</ApiRow>
                        <ApiRow name="element" ty="CapturedElement">"The listbox element; the hook\u{2019}s props capture it. Required."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">
                            "The element id, generated when "<Code inline=true>"None"</Code>". Set it when another element references "
                            "the listbox, e.g. a select trigger\u{2019}s "<Code inline=true>"aria-controls"</Code>"."
                        </ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"An accessible name for the listbox."</ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">
                            "The id of a visible label. The hook renders no label itself."
                        </ApiRow>
                        <ApiRow name="orientation" ty="Orientation" default="Vertical">
                            "Decides which arrow keys move between options; set as "<Code inline=true>"aria-orientation"</Code>"."
                        </ApiRow>
                        <ApiRow name="layout" ty="ListLayout" default="Stack">
                            <Code inline=true>"Stack"</Code>": one option per row (or column). "<Code inline=true>"Grid"</Code>
                            ": options wrap into rows, and up and down find the option in the same column."
                        </ApiRow>
                        <ApiRow name="keyboard_delegate" ty="Option<Signal<Arc<dyn KeyboardDelegate>>>" default="None">
                            "Replaces the list keyboard navigation."
                        </ApiRow>
                        <ApiRow name="options" ty="CollectionOptions" default="CollectionOptions::default()">
                            "Keyboard and focus behavior, see "<a href="#collectionoptions">"CollectionOptions"</a>"."
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

                <Section title="CollectionOptions">
                    <p>"Shared by all collection hooks. All fields have defaults."</p>

                    <ApiTable kind=ApiKind::Input of="CollectionOptions">
                        <ApiRow name="auto_focus" ty="Signal<Option<AutoFocus>>" default="None">
                            "Move focus into the listbox when it mounts: to the "<Code inline=true>"Selected"</Code>" option (or the "
                            "listbox itself), the "<Code inline=true>"First"</Code>" or the "<Code inline=true>"Last"</Code>" one. A selected option always takes precedence."
                        </ApiRow>
                        <ApiRow name="should_focus_wrap" ty="bool" default="false">"Arrow keys wrap around at the ends."</ApiRow>
                        <ApiRow name="disallow_empty_selection" ty="bool" default="false">
                            "Keep "<Keys keys="Escape"/>" from clearing the selection."
                        </ApiRow>
                        <ApiRow name="disallow_select_all" ty="bool" default="false">
                            "Disable "<Keys keys="Control + A / Command + A"/>"."
                        </ApiRow>
                        <ApiRow name="escape_key_behavior" ty="EscapeKeyBehavior" default="ClearSelection">
                            <Code inline=true>"None"</Code>" leaves "<Keys keys="Escape"/>" alone, so it can close a surrounding popover."
                        </ApiRow>
                        <ApiRow name="select_on_focus" ty="Option<bool>" default="None">
                            "Select options as keyboard focus moves to them. "<Code inline=true>"None"</Code>": only in "
                            <Code inline=true>"Replace"</Code>" selection behavior."
                        </ApiRow>
                        <ApiRow name="disallow_type_ahead" ty="bool" default="false">"Turn off type-ahead."</ApiRow>
                        <ApiRow name="allows_tab_navigation" ty="bool" default="false">
                            "Let "<Keys keys="Tab"/>" move between focusable elements inside options instead of leaving the listbox."
                        </ApiRow>
                        <ApiRow name="should_use_virtual_focus" ty="bool" default="false">
                            "DOM focus stays elsewhere (e.g. in a combo box input) and options are focused virtually, through "
                            <Code inline=true>"aria-activedescendant"</Code>"."
                        </ApiRow>
                        <ApiRow name="link_behavior" ty="LinkBehavior" default="Action">
                            "How options with a link behave. In "<Code inline=true>"Toggle"</Code>" selection behavior, "
                            <Code inline=true>"use_listbox"</Code>" turns "<Code inline=true>"Action"</Code>" into "
                            <Code inline=true>"Override"</Code>": pressing a link option opens it."
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
                                aria_label: "Fruits".into(),
                                options: CollectionOptions {
                                    should_focus_wrap: true,
                                    ..CollectionOptions::default()
                                },
                                ..UseListBoxInput::new(state, CapturedElement::new())
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
                    "It handles selection on press (with "<Keys keys="Shift"/>" and "<Keys keys="Control / Command"/>
                    " for ranges and individual options), focus and hover."
                </p>

                <Section title="Input" id="use-option-input">
                    <ApiTable kind=ApiKind::Input of="UseOptionInput">
                        <ApiRow name="list" ty="ListBoxData">"The listbox, from "<Code inline=true>"use_listbox"</Code>". Required."</ApiRow>
                        <ApiRow name="key" ty="Key">"The option\u{2019}s key in the listbox\u{2019}s collection. Required."</ApiRow>
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
                        <ApiRow name="allows_selection" ty="Signal<bool>">"Whether pressing the option can select it."</ApiRow>
                        <ApiRow name="has_action" ty="Signal<bool>">"Whether the option has an action or link to perform."</ApiRow>
                        <ApiRow name="link" ty="Option<ItemLink>">
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
                                use_option(UseOptionInput { list: data.clone(), key: Key::from("apple") });
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
                        <ApiRow name="heading" ty="Option<String>">"The header text, if any."</ApiRow>
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

            <Section title="Keyboard">
                <p>
                    "The listbox is a single tab stop: "<Keys keys="Tab"/>" moves focus into it and back out. Disabled options are "
                    "skipped. Arrow keys follow "<Code inline=true>"orientation"</Code>"; in a vertical listbox:"
                </p>

                <KeyboardTable>
                    <KeyRow keys="ArrowDown / ArrowUp">"Move focus to the next or previous option."</KeyRow>
                    <KeyRow keys="Home / End">"Move focus to the first or last option."</KeyRow>
                    <KeyRow keys="PageDown / PageUp">"Move focus by one page."</KeyRow>
                    <KeyRow keys="Shift + ArrowDown / Shift + ArrowUp">"Extend the selection (multiple selection)."</KeyRow>
                    <KeyRow keys="Space">"Toggle the selection of the focused option."</KeyRow>
                    <KeyRow keys="Enter">
                        "Select the focused option, or perform its action if the listbox has an "<Code inline=true>"on_action"</Code>"."
                    </KeyRow>
                    <KeyRow keys="Control + A / Command + A">"Select all options (multiple selection)."</KeyRow>
                    <KeyRow keys="Escape">"Clear the selection."</KeyRow>
                    <KeyRow keys="Any character">"Type-ahead: focus the next option whose text starts with the typed text."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Listbox.materialize()>"Listbox overview"</Link></li>
                <li><Link href=routes::doc::select::Hook.materialize()>"Select hooks"</Link></li>
                <li><Link href=routes::doc::Combobox.materialize()>"Combobox overview"</Link></li>
                <li><Link href=routes::doc::Collections.materialize()>"Collections"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

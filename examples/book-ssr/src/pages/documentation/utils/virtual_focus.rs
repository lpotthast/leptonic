use indoc::indoc;
use leptos::prelude::*;

use super::demos::virtual_focus::VirtualFocusDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageVirtualFocus() -> impl IntoView {
    view! {
        <DocPage title="virtual_focus">
            <p>
                "With virtual focus, DOM focus stays on one element, e.g. a combobox\u{2019}s text input, while another "
                "element, e.g. an option of its listbox, is the focused one for assistive technology: the input points at it "
                "with "<Code inline=true>"aria-activedescendant"</Code>". The functions of "
                <Code inline=true>"leptonic"</Code>" move this focus and send synthetic "
                <Code inline=true>"focus"</Code>" and "<Code inline=true>"blur"</Code>" events, so that the elements involved "
                "can react as if focus had really moved. See the "
                <Link href=routes::doc::Focus.materialize()>"Focus overview"</Link>" for the other focus building blocks."
            </p>

            <ReactAriaSource path="focus/virtualFocus.ts"/>

            <Section title="Virtual Focus or Roving Tab Index">
                <p>
                    "Composite controls move focus between their items with the arrow keys in one of two ways. Most of "
                    "leptonic\u{2019}s collections (listboxes, menus, grids, tabs) use a "<em>"roving tab index"</em>": the "
                    "focused item gets DOM focus and "<Code inline=true>"tabindex=\"0\""</Code>", all others "
                    <Code inline=true>"-1"</Code>". Use virtual focus only where DOM focus must stay where it is:"
                </p>
                <DocTable headers=&["", "Roving tab index", "Virtual focus"]>
                    <TableRow>
                        <TableCell>"DOM focus"</TableCell>
                        <TableCell>"On the focused item"</TableCell>
                        <TableCell>"On the controlling element (the input)"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Focused item for assistive technology"</TableCell>
                        <TableCell>"The element with DOM focus"</TableCell>
                        <TableCell>"The element "<Code inline=true>"aria-activedescendant"</Code>" references"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Typing"</TableCell>
                        <TableCell>"Goes to the item (type-ahead)"</TableCell>
                        <TableCell>"Goes to the input, which keeps its caret and selection"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Styling the focused item"</TableCell>
                        <TableCell><Code inline=true>":focus"</Code>", "<Code inline=true>"data-focus-visible"</Code></TableCell>
                        <TableCell>"From state ("<Code inline=true>"is_focused"</Code>"); "<Code inline=true>":focus"</Code>" doesn\u{2019}t match"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Used by"</TableCell>
                        <TableCell>"Listbox, menu, grid, grid list, tabs"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::Combobox.materialize()>"Combobox"</Link>
                            ": you keep typing while the arrow keys move through the suggestions"
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="In Collection Hooks">
                <p>
                    "The collection hooks use these functions for you. Set "
                    <Code inline=true>"should_use_virtual_focus"</Code>" in the "
                    <Link href=format!("{}#collectionoptions", routes::doc::CollectionState.materialize())>"CollectionOptions"</Link>
                    " of a listbox, and its options are focused virtually: they get no tab index, pressing one doesn\u{2019}t "
                    "move DOM focus, and when an option becomes the focused key, "<Code inline=true>"use_selectable_item"</Code>
                    " calls "<AnchorLink href="#move-virtual-focus">"move_virtual_focus"</AnchorLink>" with it. The listbox "
                    "element isn\u{2019}t focusable either. In a combobox, that call comes after the input\u{2019}s "
                    <Code inline=true>"aria-activedescendant"</Code>" already points at the option, so the options get no "
                    "synthetic events: style them from state. Only code that moves virtual focus before "
                    "updating the attribute, as the demo below does, sends them."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r"
                        use leptonic::{
                            hooks::{
                                collections::{
                                    Collection,
                                    CollectionOptions,
                                    ListLayout,
                                    use_selectable_item,
                                },
                                combobox::use_combobox,
                                focus::{use_focus, use_focus_within},
                                listbox::{UseListBoxInput, use_listbox, use_option},
                            },
                        };

                        let listbox = use_listbox(UseListBoxInput {
                            state,
                            element: listbox_element,
                            options: CollectionOptions {
                                should_use_virtual_focus: true,
                                ..CollectionOptions::default()
                            },
                            id: None,
                            aria_label: MaybeProp::default(),
                            aria_labelledby: Signal::stored(None),
                            orientation: Orientation::Vertical.into(),
                            layout: ListLayout::Stack,
                            keyboard_delegate: None,
                            layout_delegate: None,
                            is_virtualized: false,
                            should_select_on_press_up: false,
                            should_focus_on_hover: false,
                            on_action: None,
                            on_focus: None,
                            on_blur: None,
                            on_focus_change: None,
                        });

                    ")}
                </Code>
                <p>
                    "The element keeping DOM focus has to do the rest: handle the arrow keys and point at the focused "
                    "option with "<Code inline=true>"aria-activedescendant"</Code>". "
                    <Link href=routes::doc::combobox::Hook.materialize()>"use_combobox"</Link>" does both for its input and "
                    "configures its listbox this way, so a combobox needs nothing of this page."
                </p>
            </Section>

            <Section title="Demo">
                <p>
                    "Focus the input and press "<Keys keys="ArrowDown"/>", "<Keys keys="ArrowUp"/>" and "<Keys keys="Escape"/>
                    ". DOM focus stays in the input; the options react to the synthetic events, and the input loses its "
                    "focus ring while an option is focused virtually:"
                </p>

                <Demo description="A text input whose arrow keys move virtual focus through four options, listing the focus events of each move" source=include_str!("demos/virtual_focus.rs")>
                    <VirtualFocusDemo/>
                </Demo>
            </Section>

            <Section title="move_virtual_focus">
                <p>
                    <Code inline=true>"move_virtual_focus(to: Option<&Element>)"</Code>" moves virtual focus to "
                    <Code inline=true>"to"</Code>". It finds the element losing focus with "
                    <AnchorLink href="#get-virtually-focused-element">"get_virtually_focused_element"</AnchorLink>
                    ", sends it "<Code inline=true>"blur"</Code>" and "<Code inline=true>"focusout"</Code>", and sends "
                    <Code inline=true>"to"</Code>" "<Code inline=true>"focus"</Code>" and "<Code inline=true>"focusin"</Code>
                    ", each with the other element as "<Code inline=true>"relatedTarget"</Code>". Nothing happens when "
                    <Code inline=true>"to"</Code>" already has (virtual) focus; "<Code inline=true>"None"</Code>" only blurs the "
                    "focused element."
                </p>
                <p>
                    "Call it "<em>"before"</em>" "<Code inline=true>"aria-activedescendant"</Code>" changes: afterwards, "
                    "the new element already counts as focused and no events are sent. To hand focus back to the input, "
                    "move it to the input itself: the option gets "<Code inline=true>"blur"</Code>", the input "
                    <Code inline=true>"focus"</Code>"."
                </p>
            </Section>

            <Section title="dispatch_virtual_blur">
                <p>
                    <Code inline=true>"dispatch_virtual_blur(from: &Element, to: Option<&Element>)"</Code>" sends "
                    <Code inline=true>"from"</Code>" a "<Code inline=true>"blur"</Code>" and a bubbling "
                    <Code inline=true>"focusout"</Code>" event, with "<Code inline=true>"to"</Code>" as "
                    <Code inline=true>"relatedTarget"</Code>". Use it when you track the virtually focused element yourself."
                </p>
            </Section>

            <Section title="dispatch_virtual_focus">
                <p>
                    <Code inline=true>"dispatch_virtual_focus(to: &Element, from: Option<&Element>)"</Code>" sends "
                    <Code inline=true>"to"</Code>" a "<Code inline=true>"focus"</Code>" and a bubbling "
                    <Code inline=true>"focusin"</Code>" event, with "<Code inline=true>"from"</Code>" as "
                    <Code inline=true>"relatedTarget"</Code>". Sent to the input when no option is focused any more, it "
                    "brings back the input\u{2019}s focus ring."
                </p>
            </Section>

            <Section title="get_virtually_focused_element">
                <p>
                    <Code inline=true>"get_virtually_focused_element(&Document) -> Option<Element>"</Code>" returns the "
                    "element that is focused, virtually or for real: the element the active element\u{2019}s "
                    <Code inline=true>"aria-activedescendant"</Code>" references, or else the active element itself. It "
                    "looks into shadow roots for the active element."
                </p>
            </Section>

            <Section title="Accessibility">
                <ul>
                    <li>
                        "Screen readers announce the element "<Code inline=true>"aria-activedescendant"</Code>" references "
                        "like a focused element. It needs an id and a role ("<Code inline=true>"option"</Code>"), and the "
                        "focused element has to own or control it (here "<Code inline=true>"aria-controls"</Code>")."
                    </li>
                    <li>
                        "Remove "<Code inline=true>"aria-activedescendant"</Code>" when the referenced element is hidden or "
                        "removed, e.g. when a combobox\u{2019}s popover closes."
                    </li>
                    <li>
                        "Sighted keyboard users see the focused item only through your styles: "<Code inline=true>":focus"</Code>
                        " and "<Code inline=true>":focus-visible"</Code>" don\u{2019}t match a virtually focused element. "
                        "Style it from state, e.g. "<Code inline=true>"is_focused"</Code>" of "
                        <Link href=format!("{}#use-option", routes::doc::listbox::Hook.materialize())>"use_option"</Link>"."
                    </li>
                    <li>
                        <Link href=routes::doc::focus::UseFocus.materialize()>"use_focus"</Link>" and "
                        <Link href=routes::doc::focus::UseFocusWithin.materialize()>"use_focus_within"</Link>
                        " only report focus on the element that has DOM focus, so they ignore the synthetic "
                        <Code inline=true>"focus"</Code>" events an option gets. They do report the "
                        <Code inline=true>"blur"</Code>" and "<Code inline=true>"focus"</Code>" events of the element keeping "
                        "DOM focus, which is how its focus ring hides and returns."
                    </li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Focus.materialize()>"Focus overview"</Link></li>
                <li><Link href=routes::doc::combobox::Hook.materialize()>"Combobox Hooks"</Link></li>
                <li><Link href=routes::doc::listbox::Hook.materialize()>"Listbox Hooks"</Link></li>
                <li><Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusManager.materialize()>"create_focus_manager"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

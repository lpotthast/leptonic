use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::menu::MenuDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseMenuHook() -> impl IntoView {
    view! {
        <DocPage title="Menu Hooks">
            <p>
                "Hooks for creating accessible dropdown menus with full keyboard navigation, type-ahead selection, and ARIA support. "
                "See the "<Link href=routes::doc::Menu.materialize()>"Menu overview"</Link>" for concept guidance."
            </p>

            <ReactAria hook="useMenu"/>

            <Section title="Demo">
                <p>
                    "Two menus in popovers: an action menu with a disabled item, and a menu with sections whose items can be "
                    "checked. Open them with a click or from the keyboard, move with the arrow keys, activate with Enter or "
                    "Space, close with Escape, and type a letter (e.g. "<Keys keys="D"/>") to jump to a matching item."
                </p>

                <Demo description="An action menu and a menu with sections and multiple selection" source=include_str!("demos/menu.rs")>
                    <MenuDemo/>
                </Demo>
            </Section>

            <Section title="use_menu_trigger">
                <p>
                    "Provides the behavior of the button that opens a menu: when to open it, which item gets focus first, "
                    "and the ARIA attributes linking the button and the menu. It doesn\u{2019}t render the button itself. "
                    "Instead, it returns a "<Code inline=true>"UseButtonInput"</Code>" that you pass to "
                    <Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>"."
                </p>

                <Code language=Language::Rust>
                    {indoc!(
                        r#"
                        let state = use_menu_trigger_state(UseMenuTriggerStateInput::default());

                        let menu_trigger = use_menu_trigger(UseMenuTriggerInput {
                            menu_type: OverlayTriggerType::Menu,
                            is_disabled: false.into(),
                            trigger: MenuTriggerType::Press,
                            state,
                        });

                        let button = use_button(menu_trigger.button);
                        let (attrs, styles) = button.props.into_parts();

                        view! {
                            <button {..attrs} style=styles>"Actions"</button>
                        }
                    "#
                    )}
                </Code>

                <p>
                    "Render the menu in a "<Link href=routes::doc::popover::Hook.materialize()>"popover"</Link>
                    ", which positions it and closes it on Escape and outside clicks. The demo above shows the complete setup."
                </p>

                <Section title="Return" id="use-menu-trigger-return">
                    <ApiTable kind=ApiKind::Return of="UseMenuTriggerReturn">
                        <ApiRow name="button" ty="UseButtonInput">
                            "The trigger button\u{2019}s configuration (id, "<Code inline=true>"aria-haspopup"</Code>", "
                            <Code inline=true>"aria-expanded"</Code>", "<Code inline=true>"aria-controls"</Code>
                            ", press callbacks, keyboard shortcuts). Pass it to "<Code inline=true>"use_button"</Code>"."
                        </ApiRow>
                        <ApiRow name="menu_props" ty="UseMenuTriggerMenuProps">
                            "For the menu: "<Code inline=true>"id"</Code>", "<Code inline=true>"aria_labelledby"</Code>
                            ", "<Code inline=true>"auto_focus"</Code>" (first or last item when opened by keyboard, else the "
                            "menu itself) and "<Code inline=true>"on_close"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Opening the menu">
                    <p>
                        "With "<Code inline=true>"MenuTriggerType::Press"</Code>", a mouse opens the menu as soon as the button goes down, "
                        "just like native menus. Touch opens it when the finger "
                        "is lifted, so scrolling past the button doesn\u{2019}t open anything. The trigger doesn\u{2019}t take "
                        "focus on press. Screen reader users get the first item focused, mouse users get the menu itself."
                    </p>

                    <p>
                        "From the keyboard, "<Keys keys="Enter"/>", "<Keys keys="Space"/>" and "<Keys keys="ArrowDown"/>
                        " open the menu with the first item focused, "<Keys keys="ArrowUp"/>" opens it with the last one. "
                        "With "<Code inline=true>"MenuTriggerType::LongPress"</Code>", a plain press stays free for the button\u{2019}s own action. "
                        "The menu opens on a long press, or from the keyboard with "<Keys keys="Alt + ArrowDown"/>
                        " / "<Keys keys="ArrowUp"/>" ("<Keys keys="Alt + Enter"/>" and "<Keys keys="Alt + Space"/>
                        " work too). The long press is announced to screen readers as \u{201c}Long press to open menu\u{201d}."
                    </p>
                </Section>

                <Section title="Combining with your own button settings">
                    <p>
                        "Because "<Code inline=true>"menu_trigger.button"</Code>" is just a "<Code inline=true>"UseButtonInput"</Code>
                        ", you can add or override settings with struct update syntax before handing it to "<Code inline=true>"use_button"</Code>":"
                    </p>

                    <Code language=Language::Rust>
                        {indoc!(
                            r#"
                            let button = use_button(UseButtonInput {
                                aria_label: "More actions".into(),
                                on_hover_start: Some(Callback::new(|_| log!("hovered"))),
                                ..menu_trigger.button
                            });
                            let (attrs, styles) = button.props.into_parts();

                            view! {
                                <button {..attrs} style=styles>"\u{22ee}"</button>
                            }
                        "#
                        )}
                    </Code>

                    <p>
                        "The trigger then has a single press handler that does both: open the menu and run your callbacks. "
                        "You also get "<Code inline=true>"is_pressed"</Code>", "<Code inline=true>"is_hovered"</Code>", "<Code inline=true>"is_focus_visible"</Code>
                        " and a "<Code inline=true>"focus_handle"</Code>" from "<Code inline=true>"use_button"</Code>" for styling. Be careful not to replace the "
                        "press callbacks or "<Code inline=true>"shortcuts"</Code>" the trigger sets, since they are what opens the menu. If you need "
                        "your own callback on one of them, chain both with "<Code inline=true>"chain_optional_callbacks"</Code>". See "
                        <Link href=format!("{}#composing-hooks", routes::doc::Architecture.materialize())>"Composing hooks"</Link>
                        " for why leptonic composes inputs instead of merging DOM props."
                    </p>
                </Section>
            </Section>

            <Section title="use_menu_trigger_state">
                <p>
                    "Holds whether the menu is open (an "
                    <Link href=routes::doc::modal::Hook.materialize()>"overlay trigger state"</Link>
                    "), where focus goes when it opens, and which submenus are open. Takes "
                    <Code inline=true>"default_open"</Code>", a "<Code inline=true>"value"</Code>
                    " bound to app state and an "<Code inline=true>"on_open_change"</Code>" callback. "
                    <Code inline=true>"use_menu_trigger"</Code>" also accepts the states of a select or a combo box, "
                    "which add their own rules for opening (any type implementing "<Code inline=true>"MenuTriggerStateApi"</Code>")."
                </p>
                <ApiTable kind=ApiKind::Fields of="MenuTriggerState">
                    <ApiRow name="overlay" ty="OverlayTriggerState">"The open state."</ApiRow>
                    <ApiRow name="focus_strategy" ty="Signal<Option<FocusStrategy>>">
                        "Which item to focus when the menu opens, set by "<Code inline=true>"open"</Code>" and "
                        <Code inline=true>"toggle"</Code>"."
                    </ApiRow>
                    <ApiRow name="expanded_keys_stack" ty="Signal<Vec<Key>>">"The items whose submenus are open, by level."</ApiRow>
                </ApiTable>
                <DocTable headers=&["Method", "Purpose"]>
                    <TableRow>
                        <TableCell><Code inline=true>"is_open()"</Code></TableCell>
                        <TableCell>"Whether the menu is open (tracked)."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"open(focus_strategy), toggle(focus_strategy)"</Code></TableCell>
                        <TableCell>"Open or toggle the menu, focusing its first or last item, or the menu itself ("<Code inline=true>"None"</Code>")."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"close()"</Code></TableCell>
                        <TableCell>"Close the menu and its submenus."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"open_submenu(key, level), close_submenu(&key, level)"</Code></TableCell>
                        <TableCell>"Open or close the submenu of an item."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="use_menu">
                <p>
                    "Provides the behavior of the menu element: keyboard navigation, type-ahead and focus management. "
                    "A menu shows the items of a collection, like a "<Link href=routes::doc::Listbox.materialize()>"listbox"</Link>
                    ". Build the collection with "<Code inline=true>"use_collection"</Code>", hold its selection with "
                    <Code inline=true>"use_list_state"</Code>" and pass that state to "<Code inline=true>"use_menu"</Code>
                    ". Leave the selection mode at "<Code inline=true>"SelectionMode::None"</Code>" for an action menu."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        let collection = use_collection(|b| {
                            b.item("edit", "Edit");
                            b.item("archive", "Archive").disabled(true);
                            b.section("danger", |s| {
                                s.header("danger-header", "Danger zone");
                                s.item("delete", "Delete");
                            });
                        });
                        let state = use_list_state(UseListStateInput {
                            collection,
                            selection: SelectionOptions::default(),
                        });

                        let UseMenuReturn { props, data } = use_menu(UseMenuInput {
                            // From `use_menu_trigger`, when the menu opens from a button.
                            id: Some(menu_props.id.get_untracked()),
                            aria_labelledby: menu_props.aria_labelledby.into(),
                            options: CollectionOptions {
                                auto_focus: menu_props.auto_focus,
                                should_focus_wrap: true,
                                ..CollectionOptions::default()
                            },
                            on_action: Some(Callback::new(|key: Key| log!("{key}"))),
                            on_close: Some(menu_props.on_close),
                            ..UseMenuInput::new(state, CapturedElement::new())
                        });

                        view! { <ul {..props.into_attrs()}>/* items and sections, see below */</ul> }
                    "#)}
                </Code>

                <Section title="Input" id="use-menu-input">
                    <p>
                        "Create the input with "<Code inline=true>"UseMenuInput::new(state, element)"</Code>
                        " and set further fields with struct update syntax."
                    </p>
                    <ApiTable kind=ApiKind::Input of="UseMenuInput">
                        <ApiRow name="state" ty="ListState">
                            "The items and their selection, from "<Code inline=true>"use_list_state"</Code>"."
                        </ApiRow>
                        <ApiRow name="element" ty="CapturedElement">"The menu element. The props capture it."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">
                            "The element id, generated when "<Code inline=true>"None"</Code>". Use "
                            <Code inline=true>"menu_props.id"</Code>" of the menu trigger, which its "
                            <Code inline=true>"aria-controls"</Code>" points to."
                        </ApiRow>
                        <ApiRow name="aria_label, aria_labelledby" ty="MaybeProp<String>" default="None">
                            "Names the menu. A menu trigger provides "<Code inline=true>"menu_props.aria_labelledby"</Code>"."
                        </ApiRow>
                        <ApiRow name="options" ty="CollectionOptions" default="wrapping arrow keys">
                            "Keyboard and focus behavior: "<Code inline=true>"auto_focus"</Code>" (where focus goes when the menu "
                            "mounts), "<Code inline=true>"should_focus_wrap"</Code>", type-ahead, select all, Tab navigation."
                        </ApiRow>
                        <ApiRow name="keyboard_delegate" ty="Option<Signal<Arc<dyn KeyboardDelegate>>>" default="None">
                            "Replaces the list keyboard navigation."
                        </ApiRow>
                        <ApiRow name="on_action" ty="Option<Callback<Key>>" default="None">
                            "Called with the key of an activated item."
                        </ApiRow>
                        <ApiRow name="on_close" ty="Option<Callback<()>>" default="None">
                            "Called when an item closes the menu after its action."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-menu-return">
                    <ApiTable kind=ApiKind::Return of="UseMenuReturn">
                        <ApiRow name="props" ty="UseMenuProps">
                            "For the menu element: "<Code inline=true>"role=\"menu\""</Code>", id, label and the "
                            "collection\u{2019}s keyboard and focus handlers. Spread with "<Code inline=true>"{..props.into_attrs()}"</Code>"."
                        </ApiRow>
                        <ApiRow name="data" ty="MenuData">
                            "Hand this to "<Code inline=true>"use_menu_item"</Code>" and "<Code inline=true>"use_menu_section"</Code>
                            " for every item and section."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_menu_item">
                <p>
                    "Provides the behavior of one menu item: its role, press and hover handling, and activation. "
                    "Hovering an item with a pointer focuses it."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r"
                        let UseMenuItemReturn { props, label_props, is_focused, .. } = use_menu_item(UseMenuItemInput {
                            menu: data.clone(),
                            key,
                            should_close_on_select: None,
                        });
                        let (attrs, styles) = props.into_parts();

                        view! {
                            <li {..attrs} style=styles>
                                <span {..label_props.into_attrs()}>{text}</span>
                            </li>
                        }
                    ")}
                </Code>

                <Section title="Input" id="use-menu-item-input">
                    <ApiTable kind=ApiKind::Input of="UseMenuItemInput">
                        <ApiRow name="menu" ty="MenuData">"The menu, from "<Code inline=true>"use_menu"</Code>"."</ApiRow>
                        <ApiRow name="key" ty="Key">"The item\u{2019}s key in the menu\u{2019}s collection."</ApiRow>
                        <ApiRow name="should_close_on_select" ty="Option<bool>" default="None">
                            "Whether activating the item closes the menu, see "<a href="#close-behavior">"Close behavior"</a>"."
                        </ApiRow>
                    </ApiTable>
                    <p>
                        "Whether an item is disabled, and its text for type-ahead, come from the collection ("
                        <Code inline=true>"b.item(key, text).disabled(true)"</Code>")."
                    </p>
                </Section>

                <Section title="Return" id="use-menu-item-return">
                    <ApiTable kind=ApiKind::Return of="UseMenuItemReturn">
                        <ApiRow name="props" ty="PropsWithStyles<UseMenuItemProps>">
                            "For the item element: role, "<Code inline=true>"aria-disabled"</Code>", "
                            <Code inline=true>"aria-checked"</Code>", labelling and the press, hover and keyboard handlers."
                        </ApiRow>
                        <ApiRow name="label_props, description_props, keyboard_shortcut_props" ty="SlotProps">
                            "For elements holding the item\u{2019}s label, description and keyboard shortcut. The item is "
                            "labelled and described by the slots you render."
                        </ApiRow>
                        <ApiRow name="is_focused, is_focus_visible" ty="Signal<bool>">
                            "Whether the item has focus, and whether a focus ring should be shown (keyboard focus)."
                        </ApiRow>
                        <ApiRow name="is_selected" ty="Signal<bool>">"Whether the item is checked (selection menus)."</ApiRow>
                        <ApiRow name="is_pressed, is_disabled" ty="Signal<bool>">
                            "Whether the item is being pressed, and whether it is disabled."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Selection modes">
                    <p>"The selection mode of the list state decides the item role:"</p>
                    <DocTable headers=&["SelectionMode", "Item role"]>
                        <TableRow>
                            <TableCell><Code inline=true>"None"</Code></TableCell>
                            <TableCell>"An action menu: "<Code inline=true>"menuitem"</Code>"."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Single"</Code></TableCell>
                            <TableCell><Code inline=true>"menuitemradio"</Code>" with "<Code inline=true>"aria-checked"</Code>"."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Multiple"</Code></TableCell>
                            <TableCell><Code inline=true>"menuitemcheckbox"</Code>" with "<Code inline=true>"aria-checked"</Code>"."</TableCell>
                        </TableRow>
                    </DocTable>
                </Section>

                <Section title="Close behavior">
                    <p>
                        "With "<Code inline=true>"should_close_on_select: None"</Code>", activating an item calls the menu\u{2019}s "
                        <Code inline=true>"on_close"</Code>" unless the user is building up a selection: "<Keys keys="Enter"/>
                        " always closes, "<Keys keys="Space"/>" closes action menus only, and a click closes unless the menu "
                        "allows multiple selection. Links always close. "<Code inline=true>"Some(true)"</Code>" and "
                        <Code inline=true>"Some(false)"</Code>" override this."
                    </p>
                </Section>
            </Section>

            <Section title="use_menu_section">
                <p>
                    "Groups items of a collection section. The heading comes from the section\u{2019}s header in the collection, "
                    "and labels the group."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r"
                        let UseMenuSectionReturn { item_props, heading_props, group_props, heading } =
                            use_menu_section(UseMenuSectionInput { menu: data.clone(), key });

                        view! {
                            <li {..item_props.into_attrs()}>
                                {heading_props.map(|props| view! { <span {..props.into_attrs()}>{heading}</span> })}
                                <ul {..group_props.into_attrs()}>/* the section's items */</ul>
                            </li>
                        }
                    ")}
                </Code>

                <Section title="Return" id="use-menu-section-return">
                    <ApiTable kind=ApiKind::Return of="UseMenuSectionReturn">
                        <ApiRow name="item_props" ty="UseMenuSectionItemProps">
                            "For the element wrapping heading and group (an "<Code inline=true>"<li>"</Code>" in a "
                            <Code inline=true>"<ul>"</Code>" menu): "<Code inline=true>"role=\"presentation\""</Code>"."
                        </ApiRow>
                        <ApiRow name="heading_props" ty="Option<UseMenuSectionHeadingProps>">
                            "For the heading element; "<Code inline=true>"None"</Code>" when the section has no header."
                        </ApiRow>
                        <ApiRow name="group_props" ty="UseMenuSectionGroupProps">
                            "For the element containing the items: "<Code inline=true>"role=\"group\""</Code>", labelled by the heading."
                        </ApiRow>
                        <ApiRow name="heading" ty="Option<String>">"The header text."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="Enter / Space / ArrowDown">"On the trigger: open the menu and focus the first item."</KeyRow>
                    <KeyRow keys="ArrowUp">"On the trigger: open the menu and focus the last item."</KeyRow>
                    <KeyRow keys="ArrowDown / ArrowUp">"Move focus to the next or previous item, wrapping around."</KeyRow>
                    <KeyRow keys="Home / End">"Focus the first or last item."</KeyRow>
                    <KeyRow keys="Enter">"Activate the focused item and close the menu."</KeyRow>
                    <KeyRow keys="Space">"Activate the focused item; in selection menus, toggle it and keep the menu open."</KeyRow>
                    <KeyRow keys="Escape">"Close the menu (handled by the surrounding overlay)."</KeyRow>
                    <KeyRow keys="Letter keys">"Focus the next item starting with the typed text, skipping disabled items."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Menu.materialize()>"Menu overview"</Link></li>
                <li><Link href=routes::doc::Popover.materialize()>"Popover concept"</Link></li>
                <li><Link href=routes::doc::Listbox.materialize()>"Listbox concept"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

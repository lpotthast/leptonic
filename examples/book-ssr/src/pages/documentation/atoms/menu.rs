use indoc::indoc;
use leptos::prelude::*;

use super::demos::{
    menu::MenuDemo, menu_context::MenuContextDemo, menu_subdialog::MenuSubdialogDemo,
    menu_submenu::MenuSubmenuDemo,
};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomMenu() -> impl IntoView {
    view! {
        <DocPage title="Menu Atoms">
            <p>
                "The menu atoms render an unstyled menu in a "<Link href=routes::doc::popover::Atom.materialize()>"Popover"</Link>
                ", opened from a button, as a context menu or as a submenu. See the "
                <Link href=routes::doc::Menu.materialize()>"Menu overview"</Link>" for concept guidance and keyboard "
                "interaction."
            </p>

            <Section title="Hooks Used">
                <DocTable headers=&["Atom", "Hooks"]>
                    <TableRow>
                        <TableCell><Code inline=true>"MenuTrigger"</Code></TableCell>
                        <TableCell>
                            <Link href=hook_section("use-menu-trigger-state")>"use_menu_trigger_state"</Link>", "
                            <Link href=hook_section("use-menu-trigger")>"use_menu_trigger"</Link>"; its button gets the press "
                            "handlers and ARIA attributes through a "
                            <Link href=routes::doc::interactions::PressResponder.materialize()>"PressResponder"</Link>"; "
                            "context menus use "<Link href=routes::doc::interactions::UseContextMenu.materialize()>"use_context_menu"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Menu"</Code></TableCell>
                        <TableCell>
                            <Link href=format!("{}#use-list-state", routes::doc::CollectionState.materialize())>"use_list_state"</Link>
                            " (unless you pass a "<Code inline=true>"state"</Code>"), "<Link href=hook_section("use-menu")>"use_menu"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>
                            <Code inline=true>"MenuItem"</Code>", "<Code inline=true>"MenuItems"</Code>", "
                            <Code inline=true>"MenuItemLabel"</Code>", "<Code inline=true>"MenuItemDescription"</Code>", "
                            <Code inline=true>"MenuItemShortcut"</Code>
                        </TableCell>
                        <TableCell><Link href=hook_section("use-menu-item")>"use_menu_item"</Link>" and its slots"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"MenuSection"</Code></TableCell>
                        <TableCell><Link href=hook_section("use-menu-section")>"use_menu_section"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"SubmenuTrigger"</Code></TableCell>
                        <TableCell>
                            <Link href=hook_section("use-submenu-trigger-state")>"use_submenu_trigger_state"</Link>", "
                            <Link href=hook_section("use-submenu-trigger")>"use_submenu_trigger"</Link>" (with "
                            <Link href=hook_section("use-safely-mouse-to-submenu")>"use_safely_mouse_to_submenu"</Link>")"
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            atoms::prelude::*,
                            hooks::{Key, use_collection},
                        };
                        use leptos::{logging::log, prelude::*};

                        let actions = use_collection(|b| {
                            b.item("copy", "Copy");
                            b.item("paste", "Paste");
                        });

                        view! {
                            <MenuTrigger>
                                <Button>"Edit"</Button>
                                <Popover>
                                    <Menu collection=actions on_action=move |key: Key| log!("{key}")>
                                        <MenuItems let:node>{node.text_value.to_string()}</MenuItems>
                                    </Menu>
                                </Popover>
                            </MenuTrigger>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "Open a menu with a click, or focus a button and press "<Keys keys="Enter"/>" or "<Keys keys="ArrowDown"/>
                    ". \u{201c}Edit\u{201d} performs an action and closes; \u{201c}View\u{201d} has checkable items and stays open."
                </p>

                <Demo description="An action menu and a menu with checkable items, with a disabled toggle" source=include_str!("demos/menu.rs")>
                    <MenuDemo/>
                </Demo>
            </Section>

            <Section title="MenuTrigger">
                <p>
                    "Wraps the trigger button and the "<Code inline=true>"Popover"</Code>" with the menu. Pressing the button "
                    "toggles the menu; the menu is labelled by the button. The popover opens below the button, aligned with "
                    "its start edge ("<Code inline=true>"Placement::BottomStart"</Code>"), unless you give it another "
                    <Code inline=true>"placement"</Code>"."
                </p>

                <Section title="Props" id="menutrigger-props">
                <ApiTable kind=ApiKind::Props of="MenuTrigger">
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the button can\u{2019}t open the menu."</ApiRow>
                    <ApiRow name="trigger" ty="MenuTriggerType" default="Press">
                        <Code inline=true>"Press"</Code>": pressing the button opens the menu. "<Code inline=true>"LongPress"</Code>
                        ": a long press (or "<Keys keys="Alt + ArrowDown"/>") opens it, and a press performs the button\u{2019}s own "
                        <Code inline=true>"on_press"</Code>". "<Code inline=true>"ContextMenu"</Code>": the menu opens as a "
                        "context menu, see "<AnchorLink href="#context-menus">"Context Menus"</AnchorLink>"."
                    </ApiRow>
                    <ApiRow name="default_open" ty="bool" default="false">"Whether the menu starts open."</ApiRow>
                    <ApiRow name="on_open_change" ty="Option<Callback<bool>>" default="None">"Called when the menu opens or closes."</ApiRow>
                    <ApiRow name="is_open" ty="Option<Signal<bool>>" default="None">
                        "Whether the menu is open (controlled): a value or any signal."
                    </ApiRow>
                    <ApiRow name="set_open" ty="Option<Out<bool>>" default="None">
                        "Receives the new state: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                    </ApiRow>
                    <ApiRow name="children" ty="Children">"The button and the popover with the menu."</ApiRow>
                </ApiTable>
                </Section>

                <Section title="Context Menus">
                <p>
                    "With "<Code inline=true>"trigger=MenuTriggerType::ContextMenu"</Code>", the menu opens where the user "
                    "asks for a context menu on the trigger: a right click (or "<Keys keys="Control"/>" + click on macOS), "
                    <Keys keys="Shift + F10"/>", the context menu key, "<Keys keys="Control + Enter"/>" on macOS, or a long "
                    "press on iOS, which fires no context menu event. It opens where the browser reports the request: at the "
                    "pointer, or for the keyboard at the trigger ("<Keys keys="Control + Enter"/>": its center). The browser\u{2019}s "
                    "own context menu is prevented on the trigger; a right click outside the open menu closes it, so the "
                    "browser\u{2019}s menu appears there instead."
                </p>
                <p>
                    "A press doesn\u{2019}t open a context menu, so the trigger isn\u{2019}t announced as opening one: it gets no "
                    <Code inline=true>"aria-haspopup"</Code>", "<Code inline=true>"aria-expanded"</Code>" or "
                    <Code inline=true>"aria-controls"</Code>". The trigger can be any pressable atom with its own action; "
                    "context menus build on "<Link href=routes::doc::interactions::UseContextMenu.materialize()>"use_context_menu"</Link>"."
                </p>
                <Demo description="A context menu with Cut, Copy and Paste, showing the last action" source=include_str!("demos/menu_context.rs")>
                    <MenuContextDemo/>
                </Demo>
                </Section>
            </Section>

            <Section title="Menu">
                <p>
                    "The list of items. Its items come from a "<Code inline=true>"collection"</Code>": render one "
                    <Code inline=true>"MenuItem"</Code>" (or "<Code inline=true>"MenuSection"</Code>") per entry, in "
                    "collection order, or let "<Code inline=true>"MenuItems"</Code>" render all items."
                </p>

                <Section title="Props" id="menu-props">
                <ApiTable kind=ApiKind::Props of="Menu">
                    <ApiRow name="collection" ty="Option<CollectionMemo>" default="None">"The items. Required unless "<Code inline=true>"state"</Code>" is given."</ApiRow>
                    <ApiRow name="state" ty="Option<ListState>" default="None">
                        "An existing list state, instead of one created from "<Code inline=true>"collection"</Code>" and the selection props."
                    </ApiRow>
                    <ApiRow name="selection_mode" ty="Signal<SelectionMode>" default="None">
                        <Code inline=true>"None"</Code>" for a menu of actions; "<Code inline=true>"Single"</Code>" or "
                        <Code inline=true>"Multiple"</Code>" for checkable items ("<Code inline=true>"menuitemradio"</Code>" / "
                        <Code inline=true>"menuitemcheckbox"</Code>")."
                    </ApiRow>
                    <ApiRow name="default_selected_keys" ty="Vec<Key>" default="vec![]">"The initially checked items."</ApiRow>
                    <ApiRow name="selection" ty="Option<Signal<Selection>>" default="None">
                        "The checked items (controlled), replacing "<Code inline=true>"default_selected_keys"</Code>": a value or any signal."
                    </ApiRow>
                    <ApiRow name="set_selection" ty="Option<Out<Selection>>" default="None">
                        "Receives the checked items: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>
                        ", closure or "<Code inline=true>"Callback"</Code>". Bind the selection to app state to keep it "
                        "while the menu is closed: the menu is created anew on every opening."
                    </ApiRow>
                    <ApiRow name="on_selection_change" ty="Option<Callback<Selection>>" default="None">"Called when items are checked."</ApiRow>
                    <ApiRow name="disabled_keys" ty="Option<Signal<HashSet<Key>>>" default="None">"Items that can\u{2019}t be focused or activated."</ApiRow>
                    <ApiRow name="aria_label, aria_labelledby" ty="MaybeProp<String>" default="None">
                        "Names the menu. Inside a "<Code inline=true>"MenuTrigger"</Code>" the button names it by default."
                    </ApiRow>
                    <ApiRow name="auto_focus" ty="Option<AutoFocus>" default="None">
                        "Focus an item when the menu mounts. Inside a "<Code inline=true>"MenuTrigger"</Code>": the first item "
                        "(or the last, with "<Keys keys="ArrowUp"/>") when opened by keyboard, else the menu."
                    </ApiRow>
                    <ApiRow name="on_action" ty="Option<Callback<Key>>" default="None">"Called with the key of an activated item."</ApiRow>
                    <ApiRow name="on_close" ty="Option<Callback<()>>" default="None">
                        "Called when an item closes the menu after its action. Inside a "<Code inline=true>"MenuTrigger"</Code>
                        " the menu closes as well."
                    </ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the menu element."</ApiRow>
                    <ApiRow name="children" ty="Children">"The items and sections."</ApiRow>
                </ApiTable>
                </Section>
            </Section>

            <Section title="MenuItem">
                <p>
                    "An item, for the collection item "<Code inline=true>"key"</Code>": "<Code inline=true>"menuitem"</Code>", or "
                    <Code inline=true>"menuitemradio"</Code>" / "<Code inline=true>"menuitemcheckbox"</Code>" in a menu with "
                    "selection. Pressing it (or "<Keys keys="Enter"/>") performs the menu\u{2019}s "<Code inline=true>"on_action"</Code>
                    " or checks it, and closes the menu unless the menu allows multiple selection. For an item with more than "
                    "a text, use "<AnchorLink href="#menuitemlabel">"MenuItemLabel"</AnchorLink>", "
                    <AnchorLink href="#menuitemdescription">"MenuItemDescription"</AnchorLink>" and "
                    <AnchorLink href="#menuitemshortcut">"MenuItemShortcut"</AnchorLink>"."
                </p>
                <Section title="Props" id="menuitem-props">
                    <ApiTable kind=ApiKind::Props of="MenuItem">
                        <ApiRow name="key" ty="Key">"The item\u{2019}s key in the menu\u{2019}s collection. Required."</ApiRow>
                        <ApiRow name="should_close_on_select" ty="CloseOnSelect" default="Auto">
                            "Whether activating the item closes the menu: "<Code inline=true>"Always"</Code>", "
                            <Code inline=true>"Never"</Code>", or "<Code inline=true>"Auto"</Code>" (unless the menu allows multiple selection, or "
                            "the item was checked with "<Keys keys="Space"/>"). A "<Code inline=true>"bool"</Code>" converts into it."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the item element."</ApiRow>
                        <ApiRow name="children" ty="Children">"The item\u{2019}s content."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="MenuItems">
                <p>
                    "Renders one "<Code inline=true>"MenuItem"</Code>" per item of the menu\u{2019}s collection, in order: "
                    <Code inline=true>"<MenuItems let:node>{node.text_value.to_string()}</MenuItems>"</Code>"."
                </p>
                <Section title="Props" id="menuitems-props">
                    <ApiTable kind=ApiKind::Props of="MenuItems">
                        <ApiRow name="children" ty="Fn(Node) -> impl IntoView">
                            "Renders an item\u{2019}s content from its collection node ("<Code inline=true>"key"</Code>", "
                            <Code inline=true>"text_value"</Code>"). Required."
                        </ApiRow>
                        <ApiRow name="classes" ty="Classes" default="empty">"Classes of each item."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="MenuItemLabel">
                <p>
                    "The main text of an item, as a "<Code inline=true>"<span>"</Code>". While it is rendered, it labels the "
                    "item, so that a description or shortcut isn\u{2019}t part of the item\u{2019}s name. Use at most one per item."
                </p>
                <Section title="Props" id="menuitemlabel-props">
                    <ApiTable kind=ApiKind::Props of="MenuItemLabel">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<span>"</Code>"."</ApiRow>
                        <ApiRow name="children" ty="Children">"The label."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="MenuItemDescription">
                <p>"Secondary text of an item, as a "<Code inline=true>"<span>"</Code>" that describes it. Use at most one per item."</p>
                <Section title="Props" id="menuitemdescription-props">
                    <ApiTable kind=ApiKind::Props of="MenuItemDescription">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<span>"</Code>"."</ApiRow>
                        <ApiRow name="children" ty="Children">"The description."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="MenuItemShortcut">
                <p>
                    "The keyboard shortcut of an item, as a "<Code inline=true>"<span>"</Code>" announced with the item. It "
                    "only shows the shortcut: handle the keys yourself, e.g. with "
                    <Link href=routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link>". Use at most one per item."
                </p>
                <Section title="Props" id="menuitemshortcut-props">
                    <ApiTable kind=ApiKind::Props of="MenuItemShortcut">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<span>"</Code>"."</ApiRow>
                        <ApiRow name="children" ty="Children">"The shortcut, e.g. \u{201c}Ctrl+B\u{201d}."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="MenuSection">
                <p>
                    "A group of items, for the collection section "<Code inline=true>"key"</Code>": renders the section\u{2019}s "
                    "header (if the collection has one, styled with "<Code inline=true>"heading_classes"</Code>") and its children."
                </p>

                <Section title="Props" id="menusection-props">
                <ApiTable kind=ApiKind::Props of="MenuSection">
                    <ApiRow name="key" ty="Key">"The section\u{2019}s key in the menu\u{2019}s collection. Required."</ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the group element."</ApiRow>
                    <ApiRow name="heading_classes" ty="Classes" default="empty">"Classes of the heading element."</ApiRow>
                    <ApiRow name="children" ty="Children">"The section\u{2019}s items."</ApiRow>
                </ApiTable>
                </Section>
            </Section>

            <Section title="SubmenuTrigger">
                <p>
                    "Opens a submenu from an item: its children are the trigger "<Code inline=true>"MenuItem"</Code>
                    " (with the trigger\u{2019}s "<Code inline=true>"key"</Code>") and a "<Code inline=true>"Popover"</Code>
                    " with the submenu\u{2019}s "<Code inline=true>"Menu"</Code>", which has a collection of its own. The submenu "
                    "opens when the item is hovered (after "<Code inline=true>"delay"</Code>"), pressed, or on "<Keys keys="ArrowRight"/>
                    "; its popover goes next to the item and is non-modal. An action in a submenu closes the whole menu. Submenus nest."
                </p>
                <Demo description="A File menu whose Share item opens a submenu" source=include_str!("demos/menu_submenu.rs")>
                    <MenuSubmenuDemo/>
                </Demo>
                <Section title="Props" id="submenutrigger-props">
                <ApiTable kind=ApiKind::Props of="SubmenuTrigger">
                    <ApiRow name="key" ty="Key">"The key of the trigger item in the menu\u{2019}s collection. Required."</ApiRow>
                    <ApiRow name="delay" ty="Option<Duration>" default="200 ms">"How long hovering the trigger item takes to open the submenu."</ApiRow>
                    <ApiRow name="kind" ty="SubmenuKind" default="Menu">
                        "What the trigger opens: a "<Code inline=true>"Menu"</Code>" or a "<Code inline=true>"Dialog"</Code>"."
                    </ApiRow>
                    <ApiRow name="children" ty="Children">"The trigger item and the submenu\u{2019}s popover."</ApiRow>
                </ApiTable>
                </Section>
                <Section title="Subdialogs">
                    <p>
                        "With "<Code inline=true>"kind=SubmenuKind::Dialog"</Code>", the trigger item opens a dialog instead of a "
                        "submenu: the "<Code inline=true>"Popover"</Code>" holds any content, such as a form. The item announces "
                        <Code inline=true>"aria-haspopup=\"dialog\""</Code>". Focus moves into the dialog and stays there until it "
                        "closes ("<Keys keys="Escape"/>", a click outside); then it returns to the item."
                    </p>
                    <Demo description="A File menu whose Rename item opens a dialog with a text field" source=include_str!("demos/menu_subdialog.rs")>
                        <MenuSubdialogDemo/>
                    </Demo>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <p>
                    "On "<Code inline=true>"MenuItem"</Code>". Flags are rendered as "<Code inline=true>"data-selected=\"true\""</Code>
                    " while the state applies."
                </p>
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-focused" ty="true">"The item has focus, by keyboard or pointer hover."</ApiRow>
                    <ApiRow name="data-focus-visible" ty="true">"The item has keyboard focus, which should be shown."</ApiRow>
                    <ApiRow name="data-selected" ty="true">"The item is checked."</ApiRow>
                    <ApiRow name="data-disabled" ty="true">"The item is disabled."</ApiRow>
                    <ApiRow name="data-pressed" ty="true">"The item is being pressed."</ApiRow>
                    <ApiRow name="data-has-submenu" ty="true">"The item opens a submenu (or a dialog)."</ApiRow>
                    <ApiRow name="data-open" ty="true">"The item\u{2019}s submenu is open."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms add no styles. Pass "<Code inline=true>"classes"</Code>" and target the state with attribute "
                    "selectors. Hovering an item focuses it, so "<Code inline=true>"[data-focused]"</Code>" highlights the "
                    "item under the pointer and the arrow keys alike:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r#"
                        .my-menu-item[data-focused], .my-menu-item[data-open] { background: var(--surface); }
                        .my-menu-item[data-focus-visible] { outline: 2px solid var(--focus); outline-offset: -2px; }
                        .my-menu-item[data-selected] .my-check::before { content: "✓"; color: var(--accent); }
                        .my-menu-item[data-disabled] { opacity: 0.5; }
                    "#)}
                </Code>
                <p>"The popover is rendered into the document body: style it through its own classes."</p>
            </Section>

            <Section title="Composition">
                <p>
                    "A "<Code inline=true>"MenuTrigger"</Code>" takes any pressable atom as its trigger (a "
                    <Link href=routes::doc::button::Atom.materialize()>"Button"</Link>", or "
                    <Link href=format!("{}#pressable", routes::doc::interactions::PressResponder.materialize())>"Pressable"</Link>" around your own "
                    "element) and the "<Link href=routes::doc::popover::Atom.materialize()>"Popover"</Link>" atom for the menu: "
                    "give the popover a "<Code inline=true>"placement"</Code>", an "<Code inline=true>"offset"</Code>" or an "
                    <Code inline=true>"OverlayArrow"</Code>" as anywhere else. A "
                    <Link href=routes::doc::separator::Atom.materialize()>"Separator"</Link>" between items renders as "
                    <Code inline=true>"role=\"separator\""</Code>". Inside a "<Code inline=true>"SubmenuTrigger"</Code>
                    " with "<Code inline=true>"SubmenuKind::Dialog"</Code>", the popover holds a dialog with any content."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Menu.materialize()>"Menu overview"</Link></li>
                <li><Link href=routes::doc::menu::Hook.materialize()>"Menu Hooks"</Link></li>
                <li><Link href=routes::doc::popover::Atom.materialize()>"Popover Atoms"</Link></li>
                <li><Link href=routes::doc::interactions::UseContextMenu.materialize()>"use_context_menu"</Link></li>
                <li><Link href=routes::doc::listbox::Atom.materialize()>"Listbox Atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

/// A link target on the menu hook page.
fn hook_section(id: &str) -> String {
    format!("{}#{id}", routes::doc::menu::Hook.materialize())
}

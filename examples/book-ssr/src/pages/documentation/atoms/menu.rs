use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::menu::MenuDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomMenu() -> impl IntoView {
    view! {
        <DocPage title="Menu atoms">
            <p>
                "A "<Code inline=true>"MenuTrigger"</Code>" opens an unstyled "<Code inline=true>"Menu"</Code>
                " in a "<Link href=routes::doc::popover::Atom.materialize()>"Popover"</Link>" when its "
                <Code inline=true>"Button"</Code>" is pressed. See the "<Link href=routes::doc::Menu.materialize()>"Menu overview"</Link>
                " for concept guidance."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Code inline=true>"MenuTrigger"</Code>" combines "<Code inline=true>"use_menu_trigger_state"</Code>
                    " and "<Code inline=true>"use_menu_trigger"</Code>"; it gives its button the press handlers, the keys "
                    "that open the menu and "<Code inline=true>"aria-haspopup"</Code>"/"<Code inline=true>"aria-expanded"</Code>
                    "/"<Code inline=true>"aria-controls"</Code>" through a "
                    <Link href=routes::doc::interactions::PressResponder.materialize()>"PressResponder"</Link>". "
                    <Code inline=true>"Menu"</Code>", "<Code inline=true>"MenuItem"</Code>" and "<Code inline=true>"MenuSection"</Code>
                    " call "<Code inline=true>"use_menu"</Code>", "<Code inline=true>"use_menu_item"</Code>" and "
                    <Code inline=true>"use_menu_section"</Code>" (see the "<Link href=routes::doc::menu::Hook.materialize()>"menu hooks"</Link>")."
                </p>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::prelude::*;
                        use leptonic::hooks::collections::{Key, use_collection};

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
                    "toggles the menu; the menu is labelled by the button."
                </p>

                <ApiTable kind=ApiKind::Props of="MenuTrigger">
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the button can\u{2019}t open the menu."</ApiRow>
                    <ApiRow name="trigger" ty="MenuTriggerType" default="Press">
                        <Code inline=true>"Press"</Code>": pressing the button opens the menu. "<Code inline=true>"LongPress"</Code>
                        ": a long press (or "<Keys keys="Alt + ArrowDown"/>") opens it, and a press performs the button\u{2019}s own "
                        <Code inline=true>"on_press"</Code>"."
                    </ApiRow>
                    <ApiRow name="default_open" ty="bool" default="false">"Whether the menu starts open."</ApiRow>
                    <ApiRow name="on_open_change" ty="Option<Callback<bool>>" default="None">"Called when the menu opens or closes."</ApiRow>
                    <ApiRow name="state" ty="Option<ValueBinding<bool>>" default="None">
                        "The open state as app state (e.g. an "<Code inline=true>"RwSignal<bool>"</Code>"), replacing "
                        <Code inline=true>"default_open"</Code>"."
                    </ApiRow>
                    <ApiRow name="children" ty="Children">"The button and the popover with the menu."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Menu">
                <p>
                    "The list of items. Its items come from a "<Code inline=true>"collection"</Code>": render one "
                    <Code inline=true>"MenuItem"</Code>" (or "<Code inline=true>"MenuSection"</Code>") per entry, in "
                    "collection order, or let "<Code inline=true>"MenuItems"</Code>" render all items."
                </p>

                <ApiTable kind=ApiKind::Props of="Menu">
                    <ApiRow name="collection" ty="Option<CollectionMemo>" default="None">"The items."</ApiRow>
                    <ApiRow name="state" ty="Option<ListState>" default="None">
                        "An existing list state, instead of one created from "<Code inline=true>"collection"</Code>" and the selection props."
                    </ApiRow>
                    <ApiRow name="selection_mode" ty="Signal<SelectionMode>" default="None">
                        <Code inline=true>"None"</Code>" for a menu of actions; "<Code inline=true>"Single"</Code>" or "
                        <Code inline=true>"Multiple"</Code>" for checkable items ("<Code inline=true>"menuitemradio"</Code>" / "
                        <Code inline=true>"menuitemcheckbox"</Code>")."
                    </ApiRow>
                    <ApiRow name="default_selected_keys" ty="Vec<Key>" default="empty">"The initially checked items."</ApiRow>
                    <ApiRow name="selection" ty="Option<ValueBinding<Selection>>" default="None">
                        "The checked items as app state, replacing "<Code inline=true>"default_selected_keys"</Code>"."
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

            <Section title="MenuItem">
                <p>
                    "An item, for the collection item "<Code inline=true>"key"</Code>". Pressing it (or "<Keys keys="Enter"/>
                    ") performs the menu\u{2019}s "<Code inline=true>"on_action"</Code>" or checks it, and closes the menu unless "
                    "the menu allows multiple selection. Give it structured content with "<Code inline=true>"MenuItemLabel"</Code>
                    " (names the item), "<Code inline=true>"MenuItemDescription"</Code>" and "<Code inline=true>"MenuItemShortcut"</Code>
                    " (both describe it)."
                </p>

                <ApiTable kind=ApiKind::Props of="MenuItem">
                    <ApiRow name="key" ty="Key">"The item\u{2019}s key in the menu\u{2019}s collection."</ApiRow>
                    <ApiRow name="should_close_on_select" ty="Option<bool>" default="None">
                        "Whether activating the item closes the menu. Default: unless the menu allows multiple selection, or "
                        "the item was checked with "<Keys keys="Space"/>"."
                    </ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the item element."</ApiRow>
                    <ApiRow name="children" ty="Children">"The item\u{2019}s content."</ApiRow>
                </ApiTable>

                <p>
                    <Code inline=true>"MenuItems"</Code>" renders one "<Code inline=true>"MenuItem"</Code>" per collection item: "
                    <Code inline=true>"<MenuItems let:node>{node.text_value.to_string()}</MenuItems>"</Code>", with "
                    <Code inline=true>"classes"</Code>" for each item."
                </p>
            </Section>

            <Section title="MenuSection">
                <p>
                    "A group of items, for the collection section "<Code inline=true>"key"</Code>": renders the section\u{2019}s "
                    "header (if the collection has one, styled with "<Code inline=true>"heading_classes"</Code>") and its children."
                </p>

                <ApiTable kind=ApiKind::Props of="MenuSection">
                    <ApiRow name="key" ty="Key">"The section\u{2019}s key in the menu\u{2019}s collection."</ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the group element."</ApiRow>
                    <ApiRow name="heading_classes" ty="Classes" default="empty">"Classes of the heading element."</ApiRow>
                    <ApiRow name="children" ty="Children">"The section\u{2019}s items."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Data Attributes">
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-focused" ty="\"true\"">"Set on an item while it has focus."</ApiRow>
                    <ApiRow name="data-focus-visible" ty="\"true\"">"Set on an item while it has keyboard focus."</ApiRow>
                    <ApiRow name="data-selected" ty="\"true\"">"Set on a checked item."</ApiRow>
                    <ApiRow name="data-disabled" ty="\"true\"">"Set on a disabled item."</ApiRow>
                    <ApiRow name="data-pressed" ty="\"true\"">"Set on an item while it is pressed."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>"The atoms add no classes. Style items through their data attributes:"</p>
                <Code language=Language::Css>
                    {indoc!(r#"
                        .my-menu-item[data-focused] { background: #eef; }
                        .my-menu-item[data-disabled] { opacity: 0.5; }
                        .my-menu-item[aria-checked="true"]::before { content: "✓ "; }
                    "#)}
                </Code>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="Enter / Space / ArrowDown">"On the button: opens the menu with the first item focused."</KeyRow>
                    <KeyRow keys="ArrowUp">"On the button: opens the menu with the last item focused."</KeyRow>
                    <KeyRow keys="ArrowDown / ArrowUp">"Moves focus between items, wrapping around."</KeyRow>
                    <KeyRow keys="Enter / Space">"Activates the focused item."</KeyRow>
                    <KeyRow keys="Escape">"Closes the menu; focus returns to the button."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Menu.materialize()>"Menu overview"</Link></li>
                <li><Link href=routes::doc::menu::Hook.materialize()>"Menu hooks"</Link></li>
                <li><Link href=routes::doc::popover::Atom.materialize()>"Popover atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

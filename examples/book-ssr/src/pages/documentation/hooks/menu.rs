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
                "The menu hooks build a menu, its trigger, items, sections and submenus from your own markup. See the "
                <Link href=routes::doc::Menu.materialize()>"Menu overview"</Link>" for concept guidance and keyboard "
                "interaction."
            </p>

            <ReactAria hook="useMenu"/>

            <Section title="Demo">
                <p>
                    "Two menus in popovers: an action menu with a disabled item, and a menu with sections whose items can be "
                    "checked. Open them with a click or from the keyboard, move with the arrow keys, activate with "
                    <Keys keys="Enter"/>" or "<Keys keys="Space"/>", close with "<Keys keys="Escape"/>", and type a letter "
                    "(e.g. "<Keys keys="D"/>") to jump to a matching item. The checked items live in app state, so they "
                    "survive closing the menu."
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

                <Section title="Input" id="use-menu-trigger-input">
                    <ApiTable kind=ApiKind::Input of="UseMenuTriggerInput">
                        <ApiRow name="menu_type" ty="OverlayTriggerType">
                            "What the trigger opens, for its "<Code inline=true>"aria-haspopup"</Code>": "
                            <Code inline=true>"Menu"</Code>", or "<Code inline=true>"Listbox"</Code>" for a select. Required."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>">"Whether the trigger can\u{2019}t open the menu. Required."</ApiRow>
                        <ApiRow name="trigger" ty="MenuTriggerType">
                            "What opens the menu: a "<Code inline=true>"Press"</Code>", a "<Code inline=true>"LongPress"</Code>
                            " or a "<Code inline=true>"ContextMenu"</Code>" request, see "
                            <AnchorLink href="#opening-the-menu">"Opening the Menu"</AnchorLink>". Required."
                        </ApiRow>
                        <ApiRow name="state" ty="S: MenuTriggerStateApi">
                            "The state from "<Code inline=true>"use_menu_trigger_state"</Code>", or the state of a select or "
                            "combobox. Required."
                        </ApiRow>
                    </ApiTable>
                </Section>


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
                <Section title="Example" id="use-menu-trigger-example">
                <Code language=Language::Rust>
                    {indoc!(
                        r#"
                        use leptonic::hooks::*;
                        use leptos::prelude::*;

                        let state = use_menu_trigger_state(UseMenuTriggerStateInput::default());

                        let menu_trigger = use_menu_trigger(UseMenuTriggerInput {
                            menu_type: OverlayTriggerType::Menu,
                            is_disabled: Signal::stored(false),
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
                    ", which positions it and closes it on "<Keys keys="Escape"/>" and outside clicks. The demo above shows the "
                    "complete setup."
                </p>
                </Section>

                <Section title="Opening the Menu">
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
                        " work too). The long press is announced to screen readers as \u{201c}Long press or press Alt + ArrowDown "
                        "to open menu\u{201d}."
                    </p>

                    <p>
                        "With "<Code inline=true>"MenuTriggerType::ContextMenu"</Code>", the menu opens as a context menu: on a "
                        "right click ("<Keys keys="Control"/>" + click on macOS), "<Keys keys="Shift + F10"/>", the context "
                        "menu key, "<Keys keys="Control + Enter"/>" on macOS or a long press on iOS (see "
                        <Link href=routes::doc::interactions::UseContextMenu.materialize()>"use_context_menu"</Link>"), at the "
                        "point it was requested, which "<Code inline=true>"MenuTriggerState::set_point"</Code>" records for the "
                        "popover. A press doesn\u{2019}t open it, so the trigger gets no "<Code inline=true>"aria-haspopup"</Code>", "
                        <Code inline=true>"aria-expanded"</Code>" or "<Code inline=true>"aria-controls"</Code>"; a right click "
                        "outside the open menu closes it."
                    </p>
                </Section>

                <Section title="Combining with Your Own Button Settings">
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
                    <Link href=routes::doc::overlay_behavior::UseOverlayTriggerState.materialize()>"overlay trigger state"</Link>
                    "), where focus goes when it opens, and which submenus are open. "
                    <Code inline=true>"use_menu_trigger"</Code>" also accepts the states of a select or a combobox, "
                    "which add their own rules for opening (any type implementing "<Code inline=true>"MenuTriggerStateApi"</Code>")."
                </p>
                <Section title="Input" id="use-menu-trigger-state-input">
                    <ApiTable kind=ApiKind::Input of="UseMenuTriggerStateInput">
                        <ApiRow name="default_open" ty="bool" default="false">
                            "Whether the menu starts open. Ignored when "<Code inline=true>"value"</Code>" is bound."
                        </ApiRow>
                        <ApiRow name="value" ty="Option<ValueBinding<bool>>" default="None">
                            "The open state as app state, replacing "<Code inline=true>"default_open"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_open_change" ty="Option<Callback<bool>>" default="None">"Called when the menu opens or closes."</ApiRow>
                    </ApiTable>
                </Section>
                <Section title="Return" id="use-menu-trigger-state-return">
                <ApiTable kind=ApiKind::Return of="MenuTriggerState">
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
            </Section>

            <Section title="use_menu">
                <p>
                    "Provides the behavior of the menu element: keyboard navigation, type-ahead and focus management. "
                    "A menu shows the items of a collection, like a "<Link href=routes::doc::Listbox.materialize()>"listbox"</Link>
                    ". Build the collection with "<Code inline=true>"use_collection"</Code>", hold its selection with "
                    <Code inline=true>"use_list_state"</Code>" and pass that state to "<Code inline=true>"use_menu"</Code>
                    ". Leave the selection mode at "<Code inline=true>"SelectionMode::None"</Code>" for an action menu."
                </p>


                <Section title="Input" id="use-menu-input">
                    <p>
                        "Create the input with "<Code inline=true>"UseMenuInput::new(state, element)"</Code>
                        " and set further fields with struct update syntax."
                    </p>
                    <ApiTable kind=ApiKind::Input of="UseMenuInput">
                        <ApiRow name="state" ty="ListState">
                            "The items and their selection, from "<Code inline=true>"use_list_state"</Code>". Required."
                        </ApiRow>
                        <ApiRow name="element" ty="CapturedElement">"The menu element; the props capture it. Required."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">
                            "The element id, generated when "<Code inline=true>"None"</Code>". Use "
                            <Code inline=true>"menu_props.id"</Code>" of the menu trigger, which its "
                            <Code inline=true>"aria-controls"</Code>" points to."
                        </ApiRow>
                        <ApiRow name="aria_label, aria_labelledby" ty="MaybeProp<String>" default="None">
                            "Names the menu. A menu trigger provides "<Code inline=true>"menu_props.aria_labelledby"</Code>"."
                        </ApiRow>
                        <ApiRow name="options" ty="CollectionOptions" default="should_focus_wrap: true">
                            "Keyboard and focus behavior, see "<Link href=format!("{}#collectionoptions", routes::doc::CollectionState.materialize())>"CollectionOptions"</Link>
                            ". A menu trigger provides "<Code inline=true>"auto_focus"</Code>" ("<Code inline=true>"menu_props.auto_focus"</Code>")."
                        </ApiRow>
                        <ApiRow name="keyboard_delegate" ty="Option<Signal<Arc<dyn KeyboardDelegate>>>" default="None">
                            "Replaces the "<Link href=format!("{}#use-list-keyboard-delegate", routes::doc::CollectionState.materialize())>"list keyboard delegate"</Link>"."
                        </ApiRow>
                        <ApiRow name="submenu" ty="Option<SubmenuProps>" default="None">
                            "Makes the menu a submenu (from "<Code inline=true>"use_submenu_trigger"</Code>"): it takes the submenu\u{2019}s id, label and focus on opening, closes the whole menu tree after an action, and returns to its trigger on the arrow key towards the parent menu and on Escape."
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
                <Section title="Example" id="use-menu-example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            hooks::{*, collections::{CollectionOptions, SelectionOptions, UseListStateInput}},
                            utils::CapturedElement,
                        };
                        use leptos::{logging::log, prelude::*};

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
                </Section>
            </Section>

            <Section title="use_menu_item">
                <p>
                    "Provides the behavior of one menu item: its role, press and hover handling, and activation. "
                    "Hovering an item with a pointer focuses it."
                </p>


                <Section title="Input" id="use-menu-item-input">
                    <ApiTable kind=ApiKind::Input of="UseMenuItemInput">
                        <ApiRow name="menu" ty="MenuData">"The menu, from "<Code inline=true>"use_menu"</Code>". Required."</ApiRow>
                        <ApiRow name="key" ty="Key">"The item\u{2019}s key in the menu\u{2019}s collection. Required."</ApiRow>
                        <ApiRow name="should_close_on_select" ty="Option<bool>">
                            "Whether activating the item closes the menu, see "<AnchorLink href="#close-behavior">"Close Behavior"</AnchorLink>
                            ". Required; "<Code inline=true>"None"</Code>" for the default."
                        </ApiRow>
                        <ApiRow name="submenu_trigger" ty="Option<SubmenuTriggerItem>">
                            "Makes the item open a submenu (from "<Code inline=true>"use_submenu_trigger"</Code>"): it has no "
                            "action, never closes the menu and isn\u{2019}t selectable. Required; "<Code inline=true>"None"</Code>
                            " for a plain item."
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
                <Section title="Example" id="use-menu-item-example">
                <Code language=Language::Rust>
                    {indoc!(r"
                        let UseMenuItemReturn { props, label_props, is_focused, .. } = use_menu_item(UseMenuItemInput {
                            menu: data.clone(),
                            key,
                            should_close_on_select: None,
                            submenu_trigger: None,
                        });
                        let (attrs, styles) = props.into_parts();

                        view! {
                            <li {..attrs} style=styles>
                                <span {..label_props.into_attrs()}>{text}</span>
                            </li>
                        }
                    ")}
                </Code>
                </Section>

                <Section title="Selection Modes">
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

                <Section title="Close Behavior">
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

                <Section title="Input" id="use-menu-section-input">
                    <ApiTable kind=ApiKind::Input of="UseMenuSectionInput">
                        <ApiRow name="menu" ty="MenuData">"The menu, from "<Code inline=true>"use_menu"</Code>". Required."</ApiRow>
                        <ApiRow name="key" ty="Key">"The section\u{2019}s key in the menu\u{2019}s collection. Required."</ApiRow>
                    </ApiTable>
                </Section>


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
                <Section title="Example" id="use-menu-section-example">
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
                </Section>
            </Section>

            <Section title="use_submenu_trigger_state">
                <p>
                    "The state of an item that opens a submenu: whether the submenu is open, and where focus goes when it "
                    "opens. The submenus of a menu tree share the state of the root trigger, which tracks which ones are open, "
                    "so opening a submenu closes its open siblings and closing one closes the submenus it opened."
                </p>
                <Section title="Input" id="use-submenu-trigger-state-input">
                <ApiTable kind=ApiKind::Input of="UseSubmenuTriggerStateInput">
                    <ApiRow name="trigger_key" ty="Key">"The key of the item that opens the submenu. Required."</ApiRow>
                    <ApiRow name="root" ty="MenuTriggerState">"The state of the menu tree\u{2019}s root trigger. Required."</ApiRow>
                </ApiTable>
                </Section>
                <Section title="Return" id="use-submenu-trigger-state-return">
                <ApiTable kind=ApiKind::Return of="SubmenuTriggerState">
                    <ApiRow name="is_open" ty="Signal<bool>">"Whether the submenu is open."</ApiRow>
                    <ApiRow name="focus_strategy" ty="Signal<Option<FocusStrategy>>">
                        "Where focus goes when the submenu opens; "<Code inline=true>"None"</Code>": it stays on the trigger item."
                    </ApiRow>
                    <ApiRow name="level" ty="usize">"The submenu\u{2019}s level in the tree: 0 for a submenu of the root menu."</ApiRow>
                    <ApiRow name="overlay" ty="OverlayTriggerState">"The submenu as an overlay, for its popover."</ApiRow>
                </ApiTable>
                <p>
                    "Its methods: "<Code inline=true>"open(focus_strategy)"</Code>", "<Code inline=true>"close()"</Code>
                    " (also closes the submenus it opened), "<Code inline=true>"toggle(focus_strategy)"</Code>" and "
                    <Code inline=true>"close_all()"</Code>" (closes the whole menu tree)."
                </p>
                </Section>
            </Section>

            <Section title="use_submenu_trigger">
                <p>
                    "The behavior of an item that opens a submenu, or a dialog: it opens on press, on hover (after "
                    <Code inline=true>"delay"</Code>") and on the arrow key pointing into the submenu, and closes on the other "
                    "arrow key, "<Keys keys="Escape"/>" or when another item of the menu gets focus. While the pointer moves "
                    "from the item towards the open submenu, the menu ignores pointer events, so crossing other items doesn\u{2019}t "
                    "close it ("<Code inline=true>"use_safely_mouse_to_submenu"</Code>", which this hook calls)."
                </p>
                <Section title="Input" id="use-submenu-trigger-input">
                    <p>
                        <Code inline=true>"UseSubmenuTriggerInput::new(state, trigger, parent_menu, submenu)"</Code>
                        " sets the defaults; change the rest with struct update syntax."
                    </p>
                    <ApiTable kind=ApiKind::Input of="UseSubmenuTriggerInput">
                        <ApiRow name="state" ty="SubmenuTriggerState">"From "<Code inline=true>"use_submenu_trigger_state"</Code>". Required."</ApiRow>
                        <ApiRow name="trigger" ty="CapturedElement">"The trigger item. Required."</ApiRow>
                        <ApiRow name="parent_menu" ty="CapturedElement">"The menu containing the trigger item. Required."</ApiRow>
                        <ApiRow name="submenu" ty="CapturedElement">"The submenu, or the dialog, the item opens. Required."</ApiRow>
                        <ApiRow name="kind" ty="SubmenuKind" default="Menu">
                            "What the item opens: "<Code inline=true>"Menu"</Code>" (keyboard navigation continues in it) or "
                            <Code inline=true>"Dialog"</Code>" (focus moves into it and stays there)."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the item can\u{2019}t open the submenu."</ApiRow>
                        <ApiRow name="delay" ty="Duration" default="200 ms">"How long hovering the item takes to open the submenu."</ApiRow>
                    </ApiTable>
                </Section>
                <Section title="Return" id="use-submenu-trigger-return">
                    <ApiTable kind=ApiKind::Return of="UseSubmenuTriggerReturn">
                        <ApiRow name="trigger" ty="SubmenuTriggerItem">
                            "For the trigger item: pass it to "<Code inline=true>"use_menu_item"</Code>" ("
                            <Code inline=true>"submenu_trigger"</Code>")."
                        </ApiRow>
                        <ApiRow name="submenu" ty="SubmenuProps">
                            "For the submenu: pass it to "<Code inline=true>"use_menu"</Code>" ("<Code inline=true>"submenu"</Code>")."
                        </ApiRow>
                        <ApiRow name="should_close_on_interact_outside" ty="InteractOutsideFilter">
                            "For the submenu\u{2019}s popover: an interaction outside it closes it, except on the trigger item."
                        </ApiRow>
                    </ApiTable>
                    <ApiTable kind=ApiKind::Fields of="SubmenuTriggerItem">
                        <ApiRow name="id" ty="String">"The trigger item\u{2019}s id, which names the submenu."</ApiRow>
                        <ApiRow name="element" ty="CapturedElement">"The trigger item\u{2019}s element, captured by the item."</ApiRow>
                        <ApiRow name="aria_controls" ty="Signal<Option<String>>">"The submenu\u{2019}s id while it is open."</ApiRow>
                        <ApiRow name="aria_haspopup" ty="Signal<Option<AriaHasPopup>>">
                            <Code inline=true>"menu"</Code>" or "<Code inline=true>"dialog"</Code>", after the "<Code inline=true>"kind"</Code>"."
                        </ApiRow>
                        <ApiRow name="is_open" ty="Signal<bool>">"Whether the submenu is open (the item\u{2019}s "<Code inline=true>"aria-expanded"</Code>")."</ApiRow>
                        <ApiRow name="keyboard" ty="UseKeyboardProps">
                            <Keys keys="ArrowRight"/>" ("<Keys keys="ArrowLeft"/>" in right-to-left text) opens the submenu and "
                            "moves focus into it; the other arrow key closes it."
                        </ApiRow>
                        <ApiRow name="on_press_start, on_press" ty="Callback<PressEvent>">"Open the submenu on press."</ApiRow>
                        <ApiRow name="on_hover_change" ty="Callback<bool>">"Opens the submenu after hovering for "<Code inline=true>"delay"</Code>"."</ApiRow>
                    </ApiTable>
                    <ApiTable kind=ApiKind::Fields of="SubmenuProps">
                        <ApiRow name="id" ty="String">"The submenu\u{2019}s id."</ApiRow>
                        <ApiRow name="aria_labelledby" ty="String">"The trigger item names the submenu."</ApiRow>
                        <ApiRow name="level" ty="usize">"The submenu\u{2019}s level in the menu tree."</ApiRow>
                        <ApiRow name="on_close" ty="Callback<()>">"Closes the whole menu tree, after an item\u{2019}s action."</ApiRow>
                        <ApiRow name="auto_focus" ty="Signal<Option<FocusStrategy>>">"Which item to focus when the submenu opens."</ApiRow>
                        <ApiRow name="keyboard" ty="Option<UseKeyboardProps>">
                            "The arrow key towards the parent menu and "<Keys keys="Escape"/>" close the submenu, returning focus "
                            "to the trigger item. "<Code inline=true>"None"</Code>" for a subdialog."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-submenu-trigger-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use leptonic::{hooks::*, utils::CapturedElement};

                            // `root` is the state of the menu tree's trigger, `parent_menu` the element
                            // of the menu holding the "share" item, `data` that menu's `MenuData` and
                            // `share_state` the list state of the submenu's items.
                            let state = use_submenu_trigger_state(UseSubmenuTriggerStateInput {
                                trigger_key: Key::from("share"),
                                root,
                            });
                            let submenu_element = CapturedElement::new();
                            let UseSubmenuTriggerReturn { trigger, submenu, should_close_on_interact_outside } =
                                use_submenu_trigger(UseSubmenuTriggerInput::new(
                                    state,
                                    CapturedElement::new(),
                                    parent_menu,
                                    submenu_element,
                                ));

                            // The trigger item.
                            let item = use_menu_item(UseMenuItemInput {
                                menu: data.clone(),
                                key: Key::from("share"),
                                should_close_on_select: None,
                                submenu_trigger: Some(trigger),
                            });

                            // The submenu, rendered in a popover while `state.is_open` is true.
                            let share_menu = use_menu(UseMenuInput {
                                submenu: Some(submenu),
                                ..UseMenuInput::new(share_state, submenu_element)
                            });
                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="use_safely_mouse_to_submenu">
                <p>
                    "Lets the pointer travel from a trigger item to its open submenu across other items: while it moves "
                    "towards the submenu, the menu ignores pointer events; once it rests, the item under it is hovered "
                    "again. "<Code inline=true>"use_submenu_trigger"</Code>" calls it; call it yourself only for submenus "
                    "you open some other way."
                </p>
                <Section title="Input" id="use-safely-mouse-to-submenu-input">
                <ApiTable kind=ApiKind::Input of="UseSafelyMouseToSubmenuInput">
                    <ApiRow name="menu" ty="CapturedElement">"The menu containing the trigger item. Required."</ApiRow>
                    <ApiRow name="submenu" ty="CapturedElement">"The submenu. Required."</ApiRow>
                    <ApiRow name="is_open" ty="Signal<bool>">"Whether the submenu is open. Required."</ApiRow>
                    <ApiRow name="is_disabled" ty="Signal<bool>">"Whether the tracking is off. Required."</ApiRow>
                </ApiTable>
                </Section>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Menu.materialize()>"Menu overview"</Link></li>
                <li><Link href=routes::doc::menu::Atom.materialize()>"Menu Atoms"</Link></li>
                <li><Link href=routes::doc::popover::Hook.materialize()>"use_popover"</Link></li>
                <li><Link href=routes::doc::interactions::UseContextMenu.materialize()>"use_context_menu"</Link></li>
                <li><Link href=routes::doc::listbox::Hook.materialize()>"Listbox Hooks"</Link></li>
                <li><Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

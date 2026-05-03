use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::menu::MenuDemo;

#[component]
pub fn PageUseMenuHook() -> impl IntoView {
    view! {
        <Article>
            <h1 id="menu" class="anchor">
                "Menu Hooks"
                <AnchorLink href="#menu" description="Direct link to article header" />
            </h1>

            <p>
                "Hooks for creating accessible dropdown menus with full keyboard navigation, type-ahead selection, and ARIA support. "
                "See the "<Link href=crate::routes::doc::Menu.materialize()>"Menu overview"</Link>" for concept guidance."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useMenu.html" target=LinkTarget::_Blank>
                    "useMenu"
                </LinkExt>
                "."
            </p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo" />
            </h2>

            <p>"This demo uses all four menu hooks. Try:"</p>
            <ul>
                <li>"Click the button to open the menu"</li>
                <li>"Use " <kbd>"↑"</kbd> " / " <kbd>"↓"</kbd> " to navigate"</li>
                <li>"Press " <kbd>"Enter"</kbd> " or " <kbd>"Space"</kbd> " to select"</li>
                <li>"Press " <kbd>"Escape"</kbd> " to close"</li>
                <li>"Type a letter (e.g., " <kbd>"D"</kbd> ") to jump to matching items"</li>
            </ul>

            <DemoShell source=include_str!("demos/menu.rs")>
                <MenuDemo />
            </DemoShell>

            <h2 id="use_menu_trigger" class="anchor">
                "use_menu_trigger"
                <AnchorLink
                    href="#use_menu_trigger"
                    description="Direct link to use_menu_trigger"
                />
            </h2>

            <p>
                "Provides behavior for a menu trigger button. Handles press, keyboard shortcuts, and ARIA associations."
            </p>

            <Code language=Language::Rust>
                {indoc!(
                    r"
                    let state = use_menu_trigger_state(UseMenuTriggerStateInput::default());

                    let menu_trigger = use_menu_trigger(UseMenuTriggerInput {
                        menu_type: OverlayTriggerType::Menu,
                        disabled: false.into(),
                        trigger: MenuTriggerType::Press,
                        state,
                    });
                "
                )}
            </Code>

            <p>
                "Menus require overlay integration via " <code>"use_popover"</code>
                " for proper dismiss behavior (click-outside, Escape key, re-clicking the trigger). "
                "See the interactive demo above for a complete example."
            </p>

            <p>"The hook provides:"</p>
            <ul>
                <li>
                    <strong>"props"</strong>
                    " - ARIA attributes and event handlers for the button (use .into_attrs() to spread)"
                </li>
                <li>
                    <strong>"menu_props"</strong>
                    " - Props to pass to the menu (aria_labelledby, auto_focus signal, on_close)"
                </li>
            </ul>

            <p>"The state hook provides:"</p>
            <ul>
                <li>
                    <strong>"is_open"</strong>
                    " - Signal indicating if the menu is open"
                </li>
                <li>
                    <strong>"focus_strategy"</strong>
                    " - Signal with the focus strategy (First/Last) when menu opens"
                </li>
                <li>
                    <strong>"open/close/toggle"</strong>
                    " - Callbacks to control the menu"
                </li>
                <li>
                    <strong>"set_open"</strong>
                    " - Callback to set the open state directly"
                </li>
            </ul>

            <p>"The state hook accepts an optional " <code>"on_open_change"</code> " callback that fires whenever the open state changes."</p>

            <h3 id="merge_with_button" class="anchor">
                "Merging with use_button"
                <AnchorLink
                    href="#merge_with_button"
                    description="Direct link to merge_with_button"
                />
            </h3>

            <p>
                "When using " <code>"use_menu_trigger"</code> " with " <code>"use_button"</code> ", "
                "you can merge their props using the " <code>"MergeWith"</code> " trait:"
            </p>

            <Code language=Language::Rust>
                {indoc!(
                    r#"
                    let button = use_button(UseButtonInput { ... });
                    let menu_trigger = use_menu_trigger(UseMenuTriggerInput { ... });

                    // Merge using the MergeWith trait (order is irrelevant)
                    let trigger_props = button.props.merge_with(menu_trigger.props);

                    view! {
                        <button {..trigger_props.into_attrs()}>
                            "Actions"
                        </button>
                    }
                "#
                )}
            </Code>

            <p>"This combines:"</p>
            <ul>
                <li>"Button semantics: " <code>"role=\"button\""</code> ", " <code>"tabindex"</code> ", disabled state"</li>
                <li>"Menu ARIA: " <code>"aria-haspopup"</code> ", " <code>"aria-expanded"</code> ", " <code>"aria-controls"</code></li>
                <li>"Chained event handlers (both run in sequence): keydown, click, pointerdown"</li>
                <li>"Button's hover and focus handlers (for visual feedback)"</li>
            </ul>

            <p>
                "The merged return also provides " <code>"is_hovered"</code> ", " <code>"is_pressed"</code> ", "
                "and " <code>"is_focus_visible"</code> " signals from the button, plus " <code>"menu_props"</code> " from the menu trigger."
            </p>

            <h2 id="use_menu" class="anchor">
                "use_menu"
                <AnchorLink href="#use_menu" description="Direct link to use_menu" />
            </h2>

            <p>
                "Provides behavior for the menu container. Manages keyboard navigation, type-ahead, and selection."
            </p>

            <Code language=Language::Rust>
                {indoc!(
                    r#"
                    let items = vec!["Edit", "Duplicate", "Archive", "Delete"];
                    let items_signal = Signal::derive(move || items.iter().map(|s| s.to_string()).collect());

                    let menu = use_menu(UseMenuInput {
                        aria_label: Some("Actions".to_string()),
                        all_keys: items_signal,
                        disabled_keys: Signal::derive(HashSet::new),
                        get_key_label: Callback::new(|k: String| k.clone()),
                        on_action: Some(Callback::new(|key: String| {
                            // Handle item selection
                        })),
                        on_close: Some(Callback::new(|_| {
                            // Close the menu
                        })),
                        // selection_mode: SelectionMode::None (default, action-only menu)
                        // Use SelectionMode::Single for radio items,
                        // or SelectionMode::Multiple for checkbox items.
                        ..Default::default()
                    });

                    view! {
                        <ul {..menu.menu_props.into_attrs()}>
                            // Menu items here
                        </ul>
                    }
                "#
                )}
            </Code>

            <p>"The hook provides:"</p>
            <ul>
                <li>
                    <strong>"menu_props"</strong>
                    " - ARIA role, label, tabindex, and keydown handler"
                </li>
                <li>
                    <strong>"list"</strong>
                    " - Selection and focus state from use_selectable_list"
                </li>
                <li>
                    <strong>"type_select"</strong>
                    " - Type-ahead search functionality"
                </li>
                <li>
                    <strong>"selection_mode"</strong>
                    " - The selection mode, to pass through to menu items"
                </li>
            </ul>

            <h2 id="use_menu_item" class="anchor">
                "use_menu_item"
                <AnchorLink href="#use_menu_item" description="Direct link to use_menu_item" />
            </h2>

            <p>
                "Provides behavior for individual menu items. Handles focus, selection, and keyboard activation."
            </p>

            <Code language=Language::Rust>
                {indoc!(
                    r#"
                    let item = use_menu_item(UseMenuItemInput {
                        key: "edit".to_string(),
                        is_disabled: false.into(),
                        focused_key: menu.list.collection.focused_key,
                        selected_keys: menu.list.collection.selection_state.selected_keys,
                        on_focus: menu.list.collection.set_focused_key,
                        on_action: Callback::new(|key| {
                            // Handle action
                        }),
                        on_close: Some(Callback::new(|_| {
                            // Close menu
                        })),
                        close_on_select: None, // Smart default
                        selection_mode: menu.selection_mode,
                        has_description: false,
                        has_keyboard_shortcut: false,
                    });

                    view! {
                        <li {..item.item_props.into_attrs()}>
                            <span {..item.label_props.into_attrs()}>"Edit"</span>
                        </li>
                    }
                "#
                )}
            </Code>

            <p>"The hook provides:"</p>
            <ul>
                <li>
                    <strong>"item_props"</strong>
                    " - ARIA role, tabindex, disabled state, checked state, and event handlers"
                </li>
                <li>
                    <strong>"label_props"</strong>
                    " - Props for the label slot (connects via aria-labelledby)"
                </li>
                <li>
                    <strong>"description_props"</strong>
                    " - Props for the description slot (connects via aria-describedby)"
                </li>
                <li>
                    <strong>"keyboard_shortcut_props"</strong>
                    " - Props for the keyboard shortcut slot"
                </li>
                <li>
                    <strong>"is_focused"</strong>
                    " - Whether this item is currently focused"
                </li>
                <li>
                    <strong>"is_selected"</strong>
                    " - Whether this item is currently selected"
                </li>
                <li>
                    <strong>"is_disabled"</strong>
                    " - Whether this item is disabled"
                </li>
                <li>
                    <strong>"is_focus_visible"</strong>
                    " - Whether focus ring should be visible (keyboard nav only)"
                </li>
            </ul>

            <h3 id="selection_modes" class="anchor">
                "Selection Modes"
                <AnchorLink href="#selection_modes" description="Direct link to selection modes" />
            </h3>

            <p>"Menu items automatically adapt their ARIA role based on the selection mode:"</p>
            <ul>
                <li>
                    <code>"SelectionMode::None"</code>
                    " (default) - Action menu: " <code>"role=\"menuitem\""</code>
                </li>
                <li>
                    <code>"SelectionMode::Single"</code>
                    " - Radio selection: " <code>"role=\"menuitemradio\""</code> " with " <code>"aria-checked"</code>
                </li>
                <li>
                    <code>"SelectionMode::Multiple"</code>
                    " - Checkbox selection: " <code>"role=\"menuitemcheckbox\""</code> " with " <code>"aria-checked"</code>
                </li>
            </ul>

            <h3 id="close_behavior" class="anchor">
                "Close Behavior"
                <AnchorLink href="#close_behavior" description="Direct link to close behavior" />
            </h3>

            <p>
                "The " <code>"close_on_select"</code> " option controls whether the menu closes after activation:"
            </p>
            <ul>
                <li>
                    <code>"None"</code>
                    " (default) - Smart behavior: keyboard Enter always closes; multi-select click stays open; action/single-select always closes"
                </li>
                <li><code>"Some(true)"</code> " - Always close after activation"</li>
                <li><code>"Some(false)"</code> " - Never close after activation"</li>
            </ul>

            <h2 id="use_menu_section" class="anchor">
                "use_menu_section"
                <AnchorLink
                    href="#use_menu_section"
                    description="Direct link to use_menu_section"
                />
            </h2>

            <p>
                "Groups menu items with optional headings. Useful for organizing related actions."
            </p>

            <Code language=Language::Rust>
                {indoc!(
                    r#"
                    let section = use_menu_section(UseMenuSectionInput {
                        heading: Some("File Operations".to_string()),
                        aria_label: None,
                    });

                    view! {
                        <li role=section.item_props.role>
                            <span
                                id=section.heading_props.id.clone()
                                role=section.heading_props.role
                                style="padding: 0.5em 1em; font-weight: bold; color: #666;"
                            >
                                "File Operations"
                            </span>
                            <ul
                                role=section.group_props.role
                                aria-labelledby=section.group_props.aria_labelledby.clone()
                                style="list-style: none; padding: 0; margin: 0;"
                            >
                                // Section items here
                            </ul>
                        </li>
                    }
                "#
                )}
            </Code>

            <h2 id="keyboard" class="anchor">
                "Keyboard Navigation"
                <AnchorLink href="#keyboard" description="Direct link to keyboard" />
            </h2>

            <ul>
                <li>
                    <strong>"Arrow Up/Down"</strong>
                    " - Navigate between items"
                </li>
                <li>
                    <strong>"Home/End"</strong>
                    " - Jump to first/last item"
                </li>
                <li>
                    <strong>"Enter/Space"</strong>
                    " - Select focused item"
                </li>
                <li>
                    <strong>"Escape"</strong>
                    " - Close menu"
                </li>
                <li>
                    <strong>"Type characters"</strong>
                    " - Type-ahead to jump to matching items"
                </li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features" />
            </h2>

            <ul>
                <li>"Full keyboard navigation with arrow keys"</li>
                <li>"Type-ahead selection for quick item access"</li>
                <li>"Focus management with visual focus ring"</li>
                <li>"Section grouping with accessible headings"</li>
                <li>"Disabled items support"</li>
                <li>"Press and long-press trigger modes"</li>
                <li>"Selection modes: action, radio, and checkbox menu items"</li>
                <li>"Smart close behavior that varies by interaction type and selection mode"</li>
                <li>"Repeat key event filtering to prevent accidental activation"</li>
                <li>"Label, description, and keyboard shortcut slot props for accessible content"</li>
                <li>"Proper ARIA attributes for screen readers"</li>
                <li>"Mouse hover and click support"</li>
            </ul>
            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Menu.materialize()>"Menu overview"</Link></li>
                <li><Link href=crate::routes::doc::Popover.materialize()>"Popover concept"</Link></li>
                <li><Link href=crate::routes::doc::Listbox.materialize()>"Listbox concept"</Link></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf {
                    title: "Menu Hooks",
                    link: "#menu",
                },
                Toc::Leaf {
                    title: "Demo",
                    link: "#demo",
                },
                Toc::Leaf {
                    title: "use_menu_trigger",
                    link: "#use_menu_trigger",
                },
                Toc::Leaf {
                    title: "Merging with use_button",
                    link: "#merge_with_button",
                },
                Toc::Leaf {
                    title: "use_menu",
                    link: "#use_menu",
                },
                Toc::Leaf {
                    title: "use_menu_item",
                    link: "#use_menu_item",
                },
                Toc::Leaf {
                    title: "Selection Modes",
                    link: "#selection_modes",
                },
                Toc::Leaf {
                    title: "Close Behavior",
                    link: "#close_behavior",
                },
                Toc::Leaf {
                    title: "use_menu_section",
                    link: "#use_menu_section",
                },
                Toc::Leaf {
                    title: "Keyboard Navigation",
                    link: "#keyboard",
                },
                Toc::Leaf {
                    title: "Features",
                    link: "#features",
                },
                Toc::Leaf {
                    title: "See Also",
                    link: "#see-also",
                },
            ],
        } />
    }
}

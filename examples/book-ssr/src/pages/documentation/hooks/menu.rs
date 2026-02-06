use std::collections::HashSet;

use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use indoc::indoc;
use leptonic::atoms::focus_scope::FocusScope;
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptonic::prelude::{AriaExpanded, AriaHasPopup};
use leptonic::utils::MergeWith;
use leptos::prelude::*;

/// A single menu item component that uses the `use_menu_item` hook.
#[component]
fn MenuItem(
    /// The key/label for this menu item.
    item: String,
    /// Whether this item is disabled.
    #[prop(into)]
    is_disabled: Signal<bool>,
    /// The currently focused key in the menu.
    focused_key: Signal<Option<String>>,
    /// The current selection state.
    selected_keys: Signal<Selection<String>>,
    /// Callback when this item gains focus.
    on_focus: Callback<Option<String>>,
    /// Callback when this item is activated.
    on_action: Callback<String>,
    /// Callback to close the menu.
    on_close: Callback<()>,
) -> impl IntoView {
    // No need for element_ref - use_menu_item captures the element automatically!
    let UseMenuItemReturn {
        item_props,
        is_focused,
        is_selected: _,
        is_disabled,
        is_focus_visible,
    } = use_menu_item(UseMenuItemInput {
        key: item.clone(),
        is_disabled,
        focused_key,
        selected_keys,
        on_focus,
        on_action,
        on_close: Some(on_close),
        close_on_select: true,
    });

    let item_label = item.clone();

    view! {
        // No node_ref needed - just spread item_props!
        <li
            {..item_props}
            style=move || {
                format!(
                    "padding: 0.75em 1em; cursor: {}; list-style: none; transition: background 0.15s; {}{}",
                    if is_disabled.get() { "not-allowed" } else { "pointer" },
                    if is_focused.get() { "background: #e8f4fc;" } else { "" },
                    if is_focus_visible.get() {
                        "outline: 2px solid #0066cc; outline-offset: -2px;"
                    } else {
                        ""
                    },
                )
            }
        >
            {item_label}
        </li>
    }
}

#[component]
pub fn PageUseMenuHook() -> impl IntoView {
    // State for the menu - use the new state hook
    let state = use_menu_trigger_state(UseMenuTriggerStateInput::default());
    let (selected, set_selected) = signal::<Option<String>>(None);

    // Menu items
    let items = vec![
        "Edit".to_string(),
        "Duplicate".to_string(),
        "Archive".to_string(),
        "Delete".to_string(),
    ];
    let items_signal = Signal::derive({
        let items = items.clone();
        move || items.clone()
    });

    // Set up the button with hover and focus tracking
    let button = use_button(UseButtonInput {
        disabled: false.into(),
        aria_haspopup: AriaHasPopup::Menu.into(),
        aria_expanded: Signal::derive(move || match state.is_open.get() {
            true => AriaExpanded::True,
            false => AriaExpanded::False,
        }),
        use_press_input: UsePressInput {
            disabled: false.into(),
            force_prevent_default: false,
            allow_propagation: false,
            on_press: Callback::new(|_| {}),
            on_press_up: None,
            on_press_start: None,
            on_press_end: None,
        },
        use_hover_input: UseHoverInput {
            disabled: false.into(),
            on_hover_start: None,
            on_hover_end: None,
        },
        use_focus_ring_input: UseFocusRingInput::default(),
    });

    // Set up the menu trigger with menu-specific behavior
    let menu_trigger = use_menu_trigger(UseMenuTriggerInput {
        menu_type: AriaHasPopup::Menu,
        disabled: false.into(),
        trigger: MenuTriggerType::Press,
        state,
    });

    // Merge button and menu trigger into a single set of attributes.
    // This combines:
    // - Button semantics: role="button", tabindex, disabled state
    // - Menu ARIA: aria-haspopup, aria-expanded, aria-controls
    // - Menu trigger's keydown handler (handles Enter/Space/Arrow for menu)
    // - Button's hover and focus handlers (for visual feedback)
    let trigger_props = button.props.merge_with(menu_trigger.props);

    // Set up the menu with auto_focus connected to state.focus_strategy
    let menu = use_menu(UseMenuInput {
        aria_label: Some("Actions".to_string()),
        all_keys: items_signal,
        disabled_keys: Signal::derive(HashSet::new),
        get_key_label: Callback::new(|k: String| k.clone()),
        on_action: Some(Callback::new(move |key: String| {
            set_selected.set(Some(key));
        })),
        on_close: Some(state.close),
        auto_focus: state.focus_strategy,
        ..Default::default()
    });

    // Extract values we need for the MenuItem components
    let focused_key = menu.list.collection.focused_key;
    let selected_keys = menu.list.collection.selection_state.selected_keys;
    let set_focused_key = menu.list.collection.set_focused_key;

    // Clone menu props for use in the Show component (needs Fn, not FnOnce)
    let menu_props = menu.menu_props.clone();

    view! {
        <Article>
            <h1 id="menu" class="anchor">
                "Menu Hooks"
                <AnchorLink href="#menu" description="Direct link to article header" />
            </h1>

            <p>
                "Hooks for creating accessible dropdown menus with full keyboard navigation, type-ahead selection, and ARIA support."
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

            <div style="position: relative; display: inline-block; margin: 1em 0;">
                <button
                    {..trigger_props.into_attrs()}
                    style="padding: 0.75em 1.5em; border-radius: 8px; cursor: pointer; background: var(--brand-color); color: white; border: none; font-size: 1em; display: flex; align-items: center; gap: 0.5em;"
                >
                    "Actions"
                    <span style=move || {
                        format!(
                            "display: inline-block; transition: transform 0.2s; {}",
                            if state.is_open.get() { "transform: rotate(180deg);" } else { "" },
                        )
                    }>"▼"</span>
                </button>

                <Show when=move || state.is_open.get()>
                    {
                        let items = items.clone();
                        let menu_props = menu_props.clone();
                        view! {
                            <FocusScope restore_focus=true>
                                <ul
                                    {..menu_props}
                                    style="position: absolute; top: 100%; left: 0; margin: 4px 0 0 0; padding: 0.25em 0; min-width: 180px; background: white; border: 1px solid #ddd; border-radius: 8px; box-shadow: 0 4px 12px rgba(0,0,0,0.15); z-index: 100;"
                                >
                                    {items
                                        .into_iter()
                                        .map(|item| {
                                            view! {
                                                <MenuItem
                                                    item=item
                                                    is_disabled=false
                                                    focused_key=focused_key
                                                    selected_keys=selected_keys
                                                    on_focus=set_focused_key
                                                    on_action=Callback::new(move |key: String| {
                                                        set_selected.set(Some(key));
                                                    })
                                                    on_close=state.close
                                                />
                                            }
                                        })
                                        .collect_view()}
                                </ul>
                            </FocusScope>
                        }
                    }
                </Show>
            </div>

            <p>
                "Selected: "
                <strong>{move || selected.get().unwrap_or_else(|| "None".to_string())}</strong>
            </p>

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

            <Code>
                {indoc!(
                    r#"
                    // Create state using the state hook
                    let state = use_menu_trigger_state(UseMenuTriggerStateInput::default());

                    let menu_trigger = use_menu_trigger(UseMenuTriggerInput {
                        menu_type: AriaHasPopup::Menu,
                        disabled: false.into(),
                        trigger: MenuTriggerType::Press,
                        state,
                    });

                    view! {
                        <button {..menu_trigger.props.into_attrs()}>
                            "Open Menu"
                        </button>
                        <Show when=move || state.is_open.get()>
                            // Menu content here
                        </Show>
                    }
                "#
                )}
            </Code>

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
            </ul>

            <h3 id="merge_with_button" class="anchor">
                "merge_with_button"
                <AnchorLink
                    href="#merge_with_button"
                    description="Direct link to merge_with_button"
                />
            </h3>

            <p>
                "When using " <code>"use_menu_trigger"</code> " with " <code>"use_button"</code> ", "
                "you can merge their attributes using " <code>"merge_with_button"</code> ":"
            </p>

            <Code>
                {indoc!(
                    r#"
                    // Create both hooks
                    let button = use_button(UseButtonInput { ... });
                    let menu_trigger = use_menu_trigger(UseMenuTriggerInput { ... });

                    // Merge them into a single attribute set
                    let merged = menu_trigger.merge_with_button(button);

                    view! {
                        <button {..merged.props.into_attrs()}>
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

            <Code>
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
                        ..Default::default()
                    });

                    view! {
                        <ul {..menu.menu_props}>
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
            </ul>

            <h2 id="use_menu_item" class="anchor">
                "use_menu_item"
                <AnchorLink href="#use_menu_item" description="Direct link to use_menu_item" />
            </h2>

            <p>
                "Provides behavior for individual menu items. Handles focus, selection, and keyboard activation."
            </p>

            <Code>
                {indoc!(
                    r#"
                    // No element_ref needed - element is captured automatically!
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
                        close_on_select: true,
                    });

                    view! {
                        // Just spread item_props - focus management works automatically
                        <li {..item.item_props}>
                            "Edit"
                        </li>
                    }
                "#
                )}
            </Code>

            <p>"The hook provides:"</p>
            <ul>
                <li>
                    <strong>"item_props"</strong>
                    " - ARIA role, tabindex, disabled state, and event handlers"
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

            <Code>
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
                <li>"Proper ARIA attributes for screen readers"</li>
                <li>"Mouse hover and click support"</li>
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
                    title: "merge_with_button",
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
            ],
        } />
    }
}

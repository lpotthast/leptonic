use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use indoc::indoc;

use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptos::prelude::*;
use std::collections::HashSet;

/// A single option component that uses the `use_option` hook.
#[component]
fn ListboxOption(
    /// The key/label for this option.
    item_key: String,
    /// The display label for this option.
    label: &'static str,
    /// The selection state from the parent listbox.
    state: UseSelectionStateReturn<String>,
    /// The currently focused key from the parent listbox.
    focused_key: Signal<Option<String>>,
) -> impl IntoView {
    let UseOptionReturn {
        option_props,
        is_selected,
        is_focused: _,
        is_disabled,
        is_focus_visible,
        ..
    } = use_option(UseOptionInput {
        key: item_key.clone(),
        state,
        is_disabled: Signal::derive(|| false),
        should_select_on_press_up: false,
        should_use_virtual_focus: false,
        on_focus: None,
        on_press: None,
        text_value: Some(label.to_string()),
        focused_key,
    });

    view! {
        <div
            {..option_props}
            style=move || {
                format!(
                    "padding: 0.75em 1em; cursor: {}; transition: all 0.15s; display: flex; align-items: center; gap: 0.5em; {}{}",
                    if is_disabled.get() { "not-allowed" } else { "pointer" },
                    if is_selected.get() {
                        "background: var(--brand-color); color: white;"
                    } else {
                        "background: transparent;"
                    },
                    if is_focus_visible.get() {
                        "outline: 2px solid #0066cc; outline-offset: -2px;"
                    } else {
                        ""
                    },
                )
            }
        >
            <span style=move || {
                format!(
                    "width: 16px; height: 16px; border: 2px solid {}; border-radius: 3px; display: flex; align-items: center; justify-content: center;",
                    if is_selected.get() { "white" } else { "currentColor" },
                )
            }>
                <Show when=move || is_selected.get()>"✓"</Show>
            </span>
            {label}
        </div>
    }
}

#[component]
pub fn PageUseListbox() -> impl IntoView {
    let items: Vec<(&'static str, &'static str)> = vec![
        ("apple", "Apple"),
        ("banana", "Banana"),
        ("cherry", "Cherry"),
        ("date", "Date"),
        ("elderberry", "Elderberry"),
    ];

    // Create a signal for the items (just the keys)
    let item_keys: Vec<String> = items.iter().map(|(k, _)| k.to_string()).collect();
    let items_signal = Signal::derive({
        let keys = item_keys.clone();
        move || keys.clone()
    });

    // Set up the listbox with selection
    let listbox = use_listbox(UseListBoxInput {
        selection_mode: SelectionMode::Multiple,
        selection_behavior: SelectionBehavior::Toggle,
        is_disabled: Signal::derive(|| false),
        selected_keys: None,
        default_selected_keys: None,
        on_selection_change: None,
        disabled_keys: Signal::derive(HashSet::new),
        disallow_empty_selection: false,
        items: items_signal,
        should_focus_wrap: true,
        auto_focus: false,
        select_on_focus: false,
        aria_label: Some("Fruits"),
        aria_labelledby: None,
        get_text_value: Some(Callback::new(|k: String| k)),
        is_virtualized: false,
        orientation: ListBoxOrientation::Vertical,
    });

    // Get selection state for displaying and for options
    let selection_state = listbox.state.collection.selection_state;
    let selected_keys = selection_state.selected_keys;
    let focused_key = listbox.state.collection.focused_key;

    view! {
        <Article>
            <h1 id="listbox" class="anchor">
                "Listbox Hooks"
                <AnchorLink href="#listbox" description="Direct link to article header" />
            </h1>

            <p>
                "Hooks for creating accessible listboxes with single or multiple selection, keyboard navigation, and type-ahead."
            </p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo" />
            </h2>

            <p>"This demo uses the listbox hooks. Try:"</p>
            <ul>
                <li>"Click the listbox to focus it"</li>
                <li>
                    "Use " <kbd>"↑"</kbd> " / " <kbd>"↓"</kbd>
                    " to navigate (focus ring appears)"
                </li>
                <li>
                    "Press " <kbd>"Space"</kbd> " or " <kbd>"Enter"</kbd> " to toggle selection"
                </li>
                <li>"Press " <kbd>"Home"</kbd> " / " <kbd>"End"</kbd> " to jump to first/last"</li>
                <li>"Press " <kbd>"Ctrl+A"</kbd> " (or " <kbd>"Cmd+A"</kbd> ") to select all"</li>
                <li>"Press " <kbd>"Escape"</kbd> " to clear selection"</li>
                <li>"Type a letter (e.g., " <kbd>"B"</kbd> ") to jump to matching items"</li>
            </ul>

            <div
                {..listbox.listbox_props}
                style="
                border: 2px solid var(--brand-color);
                border-radius: 8px;
                max-width: 250px;
                margin: 1em 0;
                overflow: hidden;
                "
            >
                {items
                    .into_iter()
                    .map(|(key, label)| {
                        let state = selection_state.clone();
                        view! { <ListboxOption item_key=key.to_string() label=label state=state focused_key=focused_key /> }
                    })
                    .collect_view()}
            </div>

            <p>
                "Selected: "
                {move || {
                    match selected_keys.get() {
                        Selection::Keys(keys) => {
                            let mut keys_vec: Vec<_> = keys.into_iter().collect();
                            keys_vec.sort();
                            format!("{:?}", keys_vec)
                        }
                        Selection::All => "All".to_string(),
                    }
                }}
            </p>

            <h2 id="use_listbox" class="anchor">
                "use_listbox"
                <AnchorLink href="#use_listbox" description="Direct link to use_listbox" />
            </h2>

            <Code>
                {indoc!(r#"
                    let items_signal = Signal::derive(|| vec!["apple", "banana", "cherry"]);

                    let listbox = use_listbox(UseListBoxInput {
                        selection_mode: SelectionMode::Multiple,
                        selection_behavior: SelectionBehavior::Toggle,
                        items: items_signal,
                        aria_label: Some("Fruits"),
                        get_text_value: Some(Callback::new(|k: String| k)),
                        ..Default::default()
                    });

                    view! {
                        <div {..listbox.listbox_props}>
                            // Options here...
                        </div>
                    }
                "#)}
            </Code>

            <p>"Provides:"</p>
            <ul>
                <li>"ARIA listbox role"</li>
                <li>"aria-multiselectable for multiple selection"</li>
                <li>"Keyboard navigation (Arrow keys, Home/End)"</li>
                <li>"Type-ahead selection"</li>
                <li>"Ctrl+A to select all (in multiple mode)"</li>
            </ul>

            <h2 id="use_option" class="anchor">
                "use_option"
                <AnchorLink href="#use_option" description="Direct link to use_option" />
            </h2>

            <Code>
                {indoc!(r#"
                    let option = use_option(UseOptionInput {
                        key: "apple".to_string(),
                        state: listbox.state.collection.selection_state,
                        is_disabled: Signal::derive(|| false),
                        text_value: Some("Apple".to_string()),
                        ..Default::default()
                    });

                    view! {
                        <div {..option.option_props}>
                            "Apple"
                        </div>
                    }
                "#)}
            </Code>

            <p>"Provides:"</p>
            <ul>
                <li>"ARIA option role"</li>
                <li>"aria-selected state"</li>
                <li>"Focus management with " <code>"is_focus_visible"</code></li>
                <li>"Click handling for selection toggle"</li>
                <li>"Label and description props"</li>
            </ul>

            <h2 id="use_listbox_section" class="anchor">
                "use_listbox_section"
                <AnchorLink
                    href="#use_listbox_section"
                    description="Direct link to use_listbox_section"
                />
            </h2>

            <p>"Groups options with optional headings:"</p>

            <Code>
                {indoc!(r#"
                    let UseListBoxSectionReturn { group_props, heading_props, items_props } =
                        use_listbox_section(UseListBoxSectionInput {
                            heading: Some("Citrus Fruits".into()),
                        });
                "#)}
            </Code>

            <h2 id="keyboard" class="anchor">
                "Keyboard Navigation"
                <AnchorLink href="#keyboard" description="Direct link to keyboard" />
            </h2>

            <ul>
                <li>
                    <strong>"Arrow Up/Down"</strong>
                    " - Navigate between options"
                </li>
                <li>
                    <strong>"Home/End"</strong>
                    " - Jump to first/last option"
                </li>
                <li>
                    <strong>"Space/Enter"</strong>
                    " - Toggle selection"
                </li>
                <li>
                    <strong>"Ctrl+A"</strong>
                    " - Select all (multiple mode)"
                </li>
                <li>
                    <strong>"Escape"</strong>
                    " - Clear selection"
                </li>
                <li>
                    <strong>"Type characters"</strong>
                    " - Jump to matching options"
                </li>
            </ul>

            <h2 id="selection-modes" class="anchor">
                "Selection Modes"
                <AnchorLink href="#selection-modes" description="Direct link to selection modes" />
            </h2>

            <ul>
                <li>
                    <code>"SelectionMode::None"</code>
                    " - Read-only list, no selection"
                </li>
                <li>
                    <code>"SelectionMode::Single"</code>
                    " - Only one option can be selected"
                </li>
                <li>
                    <code>"SelectionMode::Multiple"</code>
                    " - Multiple options can be selected"
                </li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features" />
            </h2>

            <ul>
                <li>"Single and multiple selection"</li>
                <li>"Full keyboard navigation"</li>
                <li>"Type-ahead selection"</li>
                <li>"Section grouping"</li>
                <li>"Disabled options"</li>
                <li>"Vertical and horizontal orientations"</li>
                <li>"Proper ARIA attributes"</li>
                <li>"Focus ring for keyboard navigation"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf {
                    title: "Listbox Hooks",
                    link: "#listbox",
                },
                Toc::Leaf {
                    title: "Demo",
                    link: "#demo",
                },
                Toc::Leaf {
                    title: "use_listbox",
                    link: "#use_listbox",
                },
                Toc::Leaf {
                    title: "use_option",
                    link: "#use_option",
                },
                Toc::Leaf {
                    title: "use_listbox_section",
                    link: "#use_listbox_section",
                },
                Toc::Leaf {
                    title: "Keyboard Navigation",
                    link: "#keyboard",
                },
                Toc::Leaf {
                    title: "Selection Modes",
                    link: "#selection-modes",
                },
                Toc::Leaf {
                    title: "Features",
                    link: "#features",
                },
            ],
        } />
    }
}

use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::listbox::ListboxDemo;

#[component]
pub fn PageUseListbox() -> impl IntoView {
    view! {
        <Article>
            <h1 id="listbox" class="anchor">
                "Listbox Hooks"
                <AnchorLink href="#listbox" description="Direct link to article header" />
            </h1>

            <p>
                "Hooks for creating accessible listboxes with single or multiple selection, keyboard navigation, and type-ahead."
            </p>

            <p>
                "See the "<Link href=crate::routes::doc::Listbox.materialize()>"Listbox overview"</Link>" for concept guidance."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useListBox.html" target=LinkTarget::_Blank>
                    "useListBox"
                </LinkExt>
                "."
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

            <DemoShell source=include_str!("demos/listbox.rs")>
                <ListboxDemo />
            </DemoShell>

            <h2 id="use_listbox" class="anchor">
                "use_listbox"
                <AnchorLink href="#use_listbox" description="Direct link to use_listbox" />
            </h2>

            <Code language=Language::Rust>
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
                        <div {..listbox.listbox_props.into_attrs()}>
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

            <Code language=Language::Rust>
                {indoc!(r#"
                    let option = use_option(UseOptionInput {
                        key: "apple".to_string(),
                        state: listbox.state.collection.selection_state,
                        is_disabled: Signal::derive(|| false),
                        text_value: Some("Apple".to_string()),
                        ..Default::default()
                    });

                    view! {
                        <div {..option.option_props.into_attrs()}>
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

            <Code language=Language::Rust>
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
            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Listbox.materialize()>"Listbox overview"</Link></li>
                <li><Link href=crate::routes::doc::Select.materialize()>"Select overview"</Link></li>
                <li><Link href=crate::routes::doc::Combobox.materialize()>"Combobox overview"</Link></li>
                <li><Link href=crate::routes::doc::hooks::Selection.materialize()>"Selection hooks"</Link></li>
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
                Toc::Leaf {
                    title: "See Also",
                    link: "#see-also",
                },
            ],
        } />
    }
}

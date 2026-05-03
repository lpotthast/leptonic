use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::combobox::ComboboxDemo;
use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

#[component]
pub fn PageUseCombobox() -> impl IntoView {
    view! {
        <Article>
            <h1 id="combobox" class="anchor">
                "use_combobox"
                <AnchorLink href="#combobox" description="Direct link to article header"/>
            </h1>

            <p>
                "Hook for creating accessible comboboxes - text inputs combined with listboxes for autocomplete functionality. "
                "See the "<Link href=crate::routes::doc::Combobox.materialize()>"Combobox overview"</Link>" for concept guidance."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useComboBox.html" target=LinkTarget::_Blank>
                    "useComboBox"
                </LinkExt>
                "."
            </p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <p>"Type in the input to filter the list of fruits, or click the dropdown button to see all options."</p>

            <DemoShell source=include_str!("demos/combobox.rs")>
                <ComboboxDemo />
            </DemoShell>

            <h2 id="use_combobox" class="anchor">
                "use_combobox"
                <AnchorLink href="#use_combobox" description="Direct link to use_combobox"/>
            </h2>

            <p>"The "<code>"use_combobox"</code>" hook manages the combobox state, filtering, and provides props for the input, button, listbox, and popover elements."</p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    use leptonic::hooks::{use_combobox, UseComboBoxInput, MenuTriggerAction};

                    let items = Signal::derive(|| vec![
                        "Apple".to_string(),
                        "Banana".to_string(),
                        "Cherry".to_string(),
                    ]);

                    let combobox = use_combobox(UseComboBoxInput {
                        items,
                        aria_label: Some("Select a fruit"),
                        placeholder: Some("Search fruits..."),
                        get_text_value: Some(Callback::new(|s: String| s)),
                        menu_trigger: MenuTriggerAction::Focus,
                        ..Default::default()
                    });

                    view! {
                        <input {..combobox.input_props.into_attrs()} />
                        <button {..combobox.button_props.into_attrs()}>"▼"</button>
                        <Show when=move || combobox.is_open.get()>
                            <div {..combobox.popover_props.into_attrs()}>
                                <ul
                                    id=combobox.listbox_props.id.clone()
                                    role=combobox.listbox_props.role
                                >
                                    <For
                                        each=move || combobox.filtered_items.get()
                                        key=|item| item.clone()
                                        children={
                                            let select = combobox.select;
                                            let get_option_id = combobox.get_option_id;
                                            move |item| {
                                                let id = get_option_id.run(item.clone());
                                                let item_clone = item.clone();
                                                view! {
                                                    <li
                                                        id=id
                                                        role="option"
                                                        on:click=move |_| select.run(item_clone.clone())
                                                    >
                                                        {item}
                                                    </li>
                                                }
                                            }
                                        }
                                    />
                                </ul>
                            </div>
                        </Show>
                    }
                "#)}
            </Code>

            <p>"The hook returns:"</p>
            <ul>
                <li><code>"input_props"</code>" - Props to spread onto the input element (handles events and ARIA)"</li>
                <li><code>"button_props"</code>" - Props for the dropdown toggle button"</li>
                <li><code>"listbox_props"</code>" - Props for the listbox element (id, role, aria-labelledby)"</li>
                <li><code>"popover_props"</code>" - Props for the popover container (element capture for ariaHideOutside)"</li>
                <li><code>"is_open"</code>" - Signal indicating if the listbox is visible"</li>
                <li><code>"filtered_items"</code>" - Signal with items to display (filtered or all, depending on trigger)"</li>
                <li><code>"selected_key"</code>" - Signal with the currently selected key"</li>
                <li><code>"focused_key"</code>" - Signal with the currently focused key in the listbox"</li>
                <li><code>"select"</code>" - Callback to select an item"</li>
                <li><code>"get_option_id"</code>" - Callback to generate stable DOM IDs for option elements"</li>
                <li><code>"open"</code>" / "<code>"close"</code>" / "<code>"toggle"</code>" - Callbacks to control visibility"</li>
                <li><code>"clear"</code>" - Callback to clear input and selection"</li>
            </ul>

            <h2 id="trigger-actions" class="anchor">
                "Menu Trigger Actions"
                <AnchorLink href="#trigger-actions" description="Direct link to trigger actions"/>
            </h2>

            <p>"The "<code>"menu_trigger"</code>" option controls when the listbox appears:"</p>

            <ul>
                <li><code>"MenuTriggerAction::Input"</code>" - Open when user types (default)"</li>
                <li><code>"MenuTriggerAction::Focus"</code>" - Open when input is focused"</li>
                <li><code>"MenuTriggerAction::Manual"</code>" - Only open via button click"</li>
            </ul>

            <p>"When opened via button click or focus trigger, all items are shown (bypassing the input filter). Typing resumes filtering."</p>

            <h2 id="keyboard" class="anchor">
                "Keyboard Navigation"
                <AnchorLink href="#keyboard" description="Direct link to keyboard"/>
            </h2>

            <ul>
                <li><strong>"Arrow Down"</strong> " - Open listbox and focus first option / move to next option"</li>
                <li><strong>"Arrow Up"</strong> " - Open listbox and focus last option / move to previous option"</li>
                <li><strong>"Arrow Left/Right"</strong> " - Return to input cursor navigation"</li>
                <li><strong>"Enter"</strong> " - Select focused option and close"</li>
                <li><strong>"Tab"</strong> " - Commit current selection and move to next field"</li>
                <li><strong>"Escape"</strong> " - Revert input to selected value and close"</li>
                <li><strong>"Home/End"</strong> " - Jump to first/last option"</li>
            </ul>

            <p>"Disabled keys are automatically skipped during keyboard navigation."</p>

            <h2 id="filtering" class="anchor">
                "Filtering"
                <AnchorLink href="#filtering" description="Direct link to filtering"/>
            </h2>

            <p>"Items are filtered automatically based on the input value. The default filter performs a case-insensitive substring match using the "<code>"get_text_value"</code>" callback."</p>

            <p>"You can provide a custom filter function via the "<code>"filter"</code>" option:"</p>

            <Code language=Language::Rust>
                {indoc!(r"
                    let combobox = use_combobox(UseComboBoxInput {
                        items,
                        filter: Some(Callback::new(|(query, items): (String, Vec<String>)| {
                            // Custom filtering logic
                            items.into_iter()
                                .filter(|item| item.starts_with(&query))
                                .collect()
                        })),
                        ..Default::default()
                    });
                ")}
            </Code>

            <h2 id="accessibility" class="anchor">
                "Accessibility"
                <AnchorLink href="#accessibility" description="Direct link to accessibility"/>
            </h2>

            <ul>
                <li>"Full keyboard navigation with disabled key skipping"</li>
                <li>"ARIA combobox pattern with proper roles and attributes"</li>
                <li><code>"aria-expanded"</code>", "<code>"aria-controls"</code>" (only when open), "<code>"aria-activedescendant"</code>" managed automatically"</li>
                <li>"Stable option IDs via "<code>"get_option_id"</code>" for correct "<code>"aria-activedescendant"</code>" references"</li>
                <li>"Screen reader announcements for option count, focused item, and selection changes"</li>
                <li><code>"ariaHideOutside"</code>" hides background content from assistive technology when the menu is open"</li>
                <li>"IME composition support (keyboard events ignored during composition)"</li>
                <li><code>"spellcheck=\"false\""</code>" on input to prevent browser spell-check interference"</li>
                <li>"Commit/revert semantics: Tab commits, Escape reverts to previous selection"</li>
                <li>"Blur detection closes the menu when focus leaves the combobox"</li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Text input with dropdown suggestions"</li>
                <li>"Automatic filtering based on input value"</li>
                <li>"Multiple trigger modes (Input, Focus, Manual)"</li>
                <li>"Full keyboard navigation with focus wrapping option"</li>
                <li>"Controlled and uncontrolled usage"</li>
                <li>"Custom value support (allows_custom_value option)"</li>
                <li>"Disabled keys support"</li>
            </ul>
            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Combobox.materialize()>"Combobox overview"</Link></li>
                <li><Link href=crate::routes::doc::listbox::Hook.materialize()>"use_listbox"</Link></li>
                <li><Link href=crate::routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_combobox", link: "#combobox" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "use_combobox API", link: "#use_combobox" },
                Toc::Leaf { title: "Trigger Actions", link: "#trigger-actions" },
                Toc::Leaf { title: "Keyboard Navigation", link: "#keyboard" },
                Toc::Leaf { title: "Filtering", link: "#filtering" },
                Toc::Leaf { title: "Accessibility", link: "#accessibility" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}

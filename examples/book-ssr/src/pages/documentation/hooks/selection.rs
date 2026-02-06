use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use indoc::indoc;
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptonic::prelude::Size;
use leptos::prelude::*;
use std::collections::HashSet;

#[component]
pub fn PageUseSelection() -> impl IntoView {
    let (selection_mode, set_selection_mode) = signal(SelectionMode::Multiple);

    // We need to recreate the selection state when mode changes
    let items: Vec<&'static str> = vec!["Apple", "Banana", "Cherry", "Date", "Elderberry"];
    let items_clone: Vec<String> = items.iter().map(|s| s.to_string()).collect();

    let UseSelectionStateReturn {
        selected_keys,
        is_selected,
        toggle,
        clear_selection,
        select_all,
        selection_mode: current_mode,
        ..
    } = use_selection_state(UseSelectionStateInput {
        selection_mode: SelectionMode::Multiple,
        selection_behavior: SelectionBehavior::Toggle,
        disabled: Signal::derive(|| false),
        selected_keys: None,
        default_selected_keys: None,
        disabled_keys: Signal::derive(|| HashSet::new()),
        on_selection_change: None,
        disallow_empty_selection: false,
    });

    view! {
        <Article>
            <h1 id="selection" class="anchor">
                "Selection System"
                <AnchorLink href="#selection" description="Direct link to article header"/>
            </h1>

            <p>"The selection system provides a collection of hooks for managing item selection in lists, grids, and other collections."</p>

            <h2 id="use_selection_state" class="anchor">
                "use_selection_state"
                <AnchorLink href="#use_selection_state" description="Direct link to use_selection_state"/>
            </h2>

            <p>"Core hook for managing selection state with support for single, multiple, and no selection modes."</p>

            <Code>
                {indoc!(r#"
                    let UseSelectionStateReturn {
                        selected_keys,
                        is_selected,
                        select,
                        toggle,
                        clear_selection,
                        select_all,
                        ..
                    } = use_selection_state(UseSelectionStateInput {
                        selection_mode: SelectionMode::Multiple,
                        selection_behavior: SelectionBehavior::Toggle,
                        disabled: Signal::derive(|| false),
                        ..Default::default()
                    });
                "#)}
            </Code>

            <h3>"Selection Mode"</h3>

            <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(1.0) attr:style="margin: 1em 0;">
                <button
                    on:click=move |_| set_selection_mode.set(SelectionMode::None)
                    style=move || format!(
                        "padding: 0.5em 1em; border-radius: 4px; cursor: pointer; {}",
                        if selection_mode.get() == SelectionMode::None { "border: 2px solid var(--brand-color); background: var(--brand-color); color: white;" } else { "border: 2px solid #ccc;" }
                    )
                >
                    "None"
                </button>
                <button
                    on:click=move |_| set_selection_mode.set(SelectionMode::Single)
                    style=move || format!(
                        "padding: 0.5em 1em; border-radius: 4px; cursor: pointer; {}",
                        if selection_mode.get() == SelectionMode::Single { "border: 2px solid var(--brand-color); background: var(--brand-color); color: white;" } else { "border: 2px solid #ccc;" }
                    )
                >
                    "Single"
                </button>
                <button
                    on:click=move |_| set_selection_mode.set(SelectionMode::Multiple)
                    style=move || format!(
                        "padding: 0.5em 1em; border-radius: 4px; cursor: pointer; {}",
                        if selection_mode.get() == SelectionMode::Multiple { "border: 2px solid var(--brand-color); background: var(--brand-color); color: white;" } else { "border: 2px solid #ccc;" }
                    )
                >
                    "Multiple"
                </button>
            </Stack>

            <p style="font-size: 0.875em; color: #666;">"Current mode: " <strong>{ move || format!("{:?}", current_mode) }</strong></p>

            <h3>"Interactive List"</h3>

            <div style="border: 2px solid var(--brand-color); border-radius: 8px; padding: 0.5em; margin: 1em 0;">
                {items.into_iter().map(|item| {
                    let key = item.to_string();
                    let key_for_click = key.clone();
                    let key_for_check = key.clone();
                    view! {
                        <div
                            on:click=move |_| { toggle.run(key_for_click.clone()); }
                            style=move || format!(
                                "padding: 0.75em 1em; cursor: pointer; border-radius: 4px; margin: 0.25em; transition: all 0.2s; {}",
                                if is_selected.run(key_for_check.clone()) {
                                    "background: var(--brand-color); color: white;"
                                } else {
                                    "background: transparent;"
                                }
                            )
                        >
                            { item }
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>

            <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(0.5) attr:style="margin: 1em 0;">
                <button
                    on:click={
                        let items_clone = items_clone.clone();
                        move |_| { select_all.run(items_clone.clone()); }
                    }
                    style="padding: 0.5em 1em; border-radius: 4px; cursor: pointer; border: 1px solid #ccc;"
                >
                    "Select All"
                </button>
                <button
                    on:click=move |_| { clear_selection.run(()); }
                    style="padding: 0.5em 1em; border-radius: 4px; cursor: pointer; border: 1px solid #ccc;"
                >
                    "Clear"
                </button>
            </Stack>

            <p>"Selected: " { move || format!("{:?}", selected_keys.get()) }</p>

            <h2 id="related-hooks" class="anchor">
                "Related Hooks"
                <AnchorLink href="#related-hooks" description="Direct link to related hooks"/>
            </h2>

            <h3>"use_selectable_collection"</h3>
            <p>"Provides keyboard navigation and focus management for collections with selection. Handles arrow keys, Home/End, and type-ahead selection."</p>

            <h3>"use_selectable_list"</h3>
            <p>"Specialization for lists with vertical keyboard navigation. Handles ArrowUp/ArrowDown for single-axis navigation."</p>

            <h3>"use_selectable_item"</h3>
            <p>"Handles individual item selection. Manages focus, selection state, and click/keyboard interactions for a single item."</p>

            <h3>"use_type_select"</h3>
            <p>"Enables type-ahead selection where users can type to jump to items starting with those characters."</p>

            <h2 id="selection-modes" class="anchor">
                "Selection Modes"
                <AnchorLink href="#selection-modes" description="Direct link to selection modes"/>
            </h2>

            <ul>
                <li><strong>"None"</strong> " - No selection allowed, items are read-only"</li>
                <li><strong>"Single"</strong> " - Only one item can be selected at a time"</li>
                <li><strong>"Multiple"</strong> " - Multiple items can be selected simultaneously"</li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Three selection modes: none, single, multiple"</li>
                <li>"Disabled keys support"</li>
                <li>"Selection change callbacks"</li>
                <li>"Select all / clear operations"</li>
                <li>"Toggle individual items"</li>
                <li>"Type-ahead selection in collections"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Selection System", link: "#selection" },
                Toc::Leaf { title: "use_selection_state", link: "#use_selection_state" },
                Toc::Leaf { title: "Related Hooks", link: "#related-hooks" },
                Toc::Leaf { title: "Selection Modes", link: "#selection-modes" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}

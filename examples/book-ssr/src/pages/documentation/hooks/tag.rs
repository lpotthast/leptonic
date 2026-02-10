use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use indoc::indoc;

use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn PageUseTag() -> impl IntoView {
    let (tags, set_tags) = signal(vec![
        "Rust".to_string(),
        "Leptos".to_string(),
        "WASM".to_string(),
        "Frontend".to_string(),
    ]);
    let (focused_idx, set_focused_idx) = signal(0usize);

    let UseTagGroupReturn {
        group_props,
        label_props,
        ..
    } = use_tag_group(UseTagGroupInput {
        label: Some("Technologies".to_string()),
        selection_mode: TagGroupSelectionMode::None,
        allow_removal: true,
        on_remove: Some(Callback::new(move |key: String| {
            set_tags.update(|t| t.retain(|k| k != &key));
        })),
        ..Default::default()
    });

    view! {
        <Article>
            <h1 id="use_tag" class="anchor">
                "use_tag"
                <AnchorLink href="#use_tag" description="Direct link to article header"/>
            </h1>

            <p>"Hooks for creating accessible tag groups with selection and removal capabilities."</p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <p>"Click the × button or press Delete/Backspace to remove a tag:"</p>

            <div style="margin: 1em 0;">
                <label id={label_props.id} style="display: block; margin-bottom: 0.5em; font-weight: 500;">
                    "Technologies"
                </label>
                <div {..group_props.into_attrs()} style="display: flex; flex-wrap: wrap; gap: 0.5em;">
                    <For
                        each=move || tags.get().into_iter().enumerate()
                        key=|(_, tag)| tag.clone()
                        children=move |(idx, tag)| {
                            let tag_key = tag.clone();
                            let is_focused = Signal::derive(move || focused_idx.get() == idx);
                            let tag_for_remove = tag.clone();

                            let tag_hook = use_tag(UseTagInput {
                                tag_key: tag.clone(),
                                is_selected: Signal::derive(|| false),
                                is_focused,
                                is_disabled: Signal::derive(|| false),
                                allow_removal: true,
                                on_select: None,
                                on_remove: Some(Callback::new(move |_| {
                                    set_tags.update(|t| t.retain(|k| k != &tag_for_remove));
                                })),
                                on_focus_next: Some(Callback::new(move |_| {
                                    set_focused_idx.update(|i| {
                                        let len = tags.get_untracked().len();
                                        *i = (*i + 1).min(len.saturating_sub(1));
                                    });
                                })),
                                on_focus_previous: Some(Callback::new(move |_| {
                                    set_focused_idx.update(|i| *i = i.saturating_sub(1));
                                })),
                            });

                            view! {
                                <div
                                    {..tag_hook.row_props.into_attrs()}
                                    style="display: inline-flex; align-items: center; gap: 0.25em; padding: 0.25em 0.5em; background: #e3f2fd; border-radius: 16px; cursor: pointer; outline: none;"
                                    style:box-shadow=move || if is_focused.get() { "0 0 0 2px var(--brand-color)" } else { "none" }
                                >
                                    <span {..tag_hook.cell_props.into_attrs()}>{ tag_key }</span>
                                    <button
                                        {..tag_hook.remove_button_props.into_attrs()}
                                        style="border: none; background: none; cursor: pointer; padding: 0; width: 16px; height: 16px; border-radius: 50%; display: flex; align-items: center; justify-content: center;"
                                    >
                                        "×"
                                    </button>
                                </div>
                            }
                        }
                    />
                </div>
            </div>

            <p style="margin-top: 1em; font-size: 0.875em; opacity: 0.7;">
                "Use arrow keys to navigate, Delete/Backspace to remove"
            </p>

            <Code>
                {indoc!(r#"
                    let UseTagGroupReturn { group_props, label_props, .. } = use_tag_group(UseTagGroupInput {
                        label: Some("Categories".to_string()),
                        selection_mode: TagGroupSelectionMode::Multiple,
                        allow_removal: true,
                        on_remove: Some(Callback::new(|key| remove_tag(key))),
                        ..Default::default()
                    });

                    let tag = use_tag(UseTagInput {
                        tag_key: "rust".to_string(),
                        is_selected: Signal::derive(|| false),
                        is_focused: Signal::derive(|| true),
                        is_disabled: Signal::derive(|| false),
                        allow_removal: true,
                        on_remove: Some(Callback::new(|_| remove_this_tag())),
                        ..Default::default()
                    });

                    view! {
                        <div {..group_props}>
                            <div {..tag.row_props.into_attrs()}>
                                <span {..tag.cell_props.into_attrs()}>"Rust"</span>
                                <button {..tag.remove_button_props.into_attrs()}>"×"</button>
                            </div>
                        </div>
                    }
                "#)}
            </Code>

            <h2 id="selection-modes" class="anchor">
                "Selection Modes"
                <AnchorLink href="#selection-modes" description="Direct link to selection modes"/>
            </h2>

            <ul>
                <li><code>"TagGroupSelectionMode::None"</code> " - No selection (default)"</li>
                <li><code>"TagGroupSelectionMode::Single"</code> " - Single tag selection"</li>
                <li><code>"TagGroupSelectionMode::Multiple"</code> " - Multiple tag selection"</li>
            </ul>

            <h2 id="aria-attributes" class="anchor">
                "ARIA Attributes"
                <AnchorLink href="#aria-attributes" description="Direct link to ARIA attributes"/>
            </h2>

            <p>"The hooks automatically set:"</p>
            <ul>
                <li><code>"role=\"grid\""</code> " on the group"</li>
                <li><code>"role=\"row\""</code> " on each tag"</li>
                <li><code>"role=\"gridcell\""</code> " on the tag content"</li>
                <li><code>"aria-selected"</code> " for selection state"</li>
                <li><code>"aria-disabled"</code> " for disabled tags"</li>
                <li><code>"aria-label=\"Remove\""</code> " on remove button"</li>
            </ul>

            <h2 id="keyboard-navigation" class="anchor">
                "Keyboard Navigation"
                <AnchorLink href="#keyboard-navigation" description="Direct link to keyboard"/>
            </h2>

            <ul>
                <li><code>"Arrow Left/Up"</code> " - Focus previous tag"</li>
                <li><code>"Arrow Right/Down"</code> " - Focus next tag"</li>
                <li><code>"Enter/Space"</code> " - Select/toggle tag"</li>
                <li><code>"Delete/Backspace"</code> " - Remove tag (if removable)"</li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Multiple selection modes"</li>
                <li>"Removable tags"</li>
                <li>"Keyboard navigation"</li>
                <li>"Focus management"</li>
                <li>"Full ARIA support"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_tag", link: "#use_tag" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "Selection Modes", link: "#selection-modes" },
                Toc::Leaf { title: "ARIA Attributes", link: "#aria-attributes" },
                Toc::Leaf { title: "Keyboard Navigation", link: "#keyboard-navigation" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}

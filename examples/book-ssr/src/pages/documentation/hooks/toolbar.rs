use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use indoc::indoc;
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn PageUseToolbar() -> impl IntoView {
    let (focused_idx, set_focused_idx) = signal(0usize);
    let (bold, set_bold) = signal(false);
    let (italic, set_italic) = signal(false);
    let (underline, set_underline) = signal(false);

    let toolbar = use_toolbar(UseToolbarInput {
        label: Some("Text Formatting".to_string()),
        orientation: ToolbarOrientation::Horizontal,
        on_focus_next: Some(Callback::new(move |_| {
            set_focused_idx.update(|i| *i = (*i + 1).min(2));
        })),
        on_focus_previous: Some(Callback::new(move |_| {
            set_focused_idx.update(|i| *i = i.saturating_sub(1));
        })),
        on_focus_first: Some(Callback::new(move |_| set_focused_idx.set(0))),
        on_focus_last: Some(Callback::new(move |_| set_focused_idx.set(2))),
        ..Default::default()
    });

    let vertical_toolbar = use_toolbar(UseToolbarInput {
        label: Some("Actions".to_string()),
        orientation: ToolbarOrientation::Vertical,
        ..Default::default()
    });

    view! {
        <Article>
            <h1 id="use_toolbar" class="anchor">
                "use_toolbar"
                <AnchorLink href="#use_toolbar" description="Direct link to article header"/>
            </h1>

            <p>"Hook for creating accessible toolbars that group related controls together."</p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <h3>"Horizontal Toolbar"</h3>
            <div
                {..toolbar.toolbar_props}
                style="display: flex; gap: 0.5em; padding: 0.5em; background: #f5f5f5; border-radius: 4px; margin: 1em 0;"
            >
                <button
                    on:click=move |_| set_bold.update(|b| *b = !*b)
                    style=move || format!(
                        "padding: 0.5em 1em; border: 1px solid #ccc; border-radius: 4px; cursor: pointer; font-weight: bold; {}",
                        if bold.get() { "background: #1976d2; color: white;" } else { "background: white;" }
                    )
                    style:outline=move || if focused_idx.get() == 0 { "2px solid var(--brand-color)" } else { "none" }
                    tabindex=move || if focused_idx.get() == 0 { "0" } else { "-1" }
                >
                    "B"
                </button>
                <button
                    on:click=move |_| set_italic.update(|i| *i = !*i)
                    style=move || format!(
                        "padding: 0.5em 1em; border: 1px solid #ccc; border-radius: 4px; cursor: pointer; font-style: italic; {}",
                        if italic.get() { "background: #1976d2; color: white;" } else { "background: white;" }
                    )
                    style:outline=move || if focused_idx.get() == 1 { "2px solid var(--brand-color)" } else { "none" }
                    tabindex=move || if focused_idx.get() == 1 { "0" } else { "-1" }
                >
                    "I"
                </button>
                <button
                    on:click=move |_| set_underline.update(|u| *u = !*u)
                    style=move || format!(
                        "padding: 0.5em 1em; border: 1px solid #ccc; border-radius: 4px; cursor: pointer; text-decoration: underline; {}",
                        if underline.get() { "background: #1976d2; color: white;" } else { "background: white;" }
                    )
                    style:outline=move || if focused_idx.get() == 2 { "2px solid var(--brand-color)" } else { "none" }
                    tabindex=move || if focused_idx.get() == 2 { "0" } else { "-1" }
                >
                    "U"
                </button>
            </div>

            <p>
                "Preview: "
                <span
                    style:font-weight=move || if bold.get() { "bold" } else { "normal" }
                    style:font-style=move || if italic.get() { "italic" } else { "normal" }
                    style:text-decoration=move || if underline.get() { "underline" } else { "none" }
                >
                    "Sample formatted text"
                </span>
            </p>

            <h3>"Vertical Toolbar"</h3>
            <div
                {..vertical_toolbar.toolbar_props}
                style="display: flex; flex-direction: column; gap: 0.5em; padding: 0.5em; background: #f5f5f5; border-radius: 4px; width: fit-content; margin: 1em 0;"
            >
                <button style="padding: 0.5em 1em; border: 1px solid #ccc; border-radius: 4px; cursor: pointer; background: white;">
                    "New"
                </button>
                <button style="padding: 0.5em 1em; border: 1px solid #ccc; border-radius: 4px; cursor: pointer; background: white;">
                    "Save"
                </button>
                <button style="padding: 0.5em 1em; border: 1px solid #ccc; border-radius: 4px; cursor: pointer; background: white;">
                    "Export"
                </button>
            </div>

            <Code>
                {indoc!(r#"
                    let toolbar = use_toolbar(UseToolbarInput {
                        label: Some("Text Formatting".to_string()),
                        orientation: ToolbarOrientation::Horizontal,
                        on_focus_next: Some(Callback::new(|_| focus_next_item())),
                        on_focus_previous: Some(Callback::new(|_| focus_previous_item())),
                        on_focus_first: Some(Callback::new(|_| focus_first_item())),
                        on_focus_last: Some(Callback::new(|_| focus_last_item())),
                        ..Default::default()
                    });

                    view! {
                        <div {..toolbar.toolbar_props}>
                            <button tabindex="0">"Bold"</button>
                            <button tabindex="-1">"Italic"</button>
                            <button tabindex="-1">"Underline"</button>
                        </div>
                    }
                "#)}
            </Code>

            <h2 id="orientation" class="anchor">
                "Orientation"
                <AnchorLink href="#orientation" description="Direct link to orientation"/>
            </h2>

            <ul>
                <li><code>"ToolbarOrientation::Horizontal"</code> " - Arrow Left/Right navigation (default)"</li>
                <li><code>"ToolbarOrientation::Vertical"</code> " - Arrow Up/Down navigation"</li>
            </ul>

            <h2 id="keyboard-navigation" class="anchor">
                "Keyboard Navigation"
                <AnchorLink href="#keyboard-navigation" description="Direct link to keyboard"/>
            </h2>

            <p>"For horizontal toolbars:"</p>
            <ul>
                <li><code>"Arrow Right"</code> " - Focus next item"</li>
                <li><code>"Arrow Left"</code> " - Focus previous item"</li>
                <li><code>"Home"</code> " - Focus first item"</li>
                <li><code>"End"</code> " - Focus last item"</li>
            </ul>

            <p>"For vertical toolbars, Arrow Up/Down are used instead."</p>

            <h2 id="aria-attributes" class="anchor">
                "ARIA Attributes"
                <AnchorLink href="#aria-attributes" description="Direct link to ARIA attributes"/>
            </h2>

            <p>"The hook automatically sets:"</p>
            <ul>
                <li><code>"role=\"toolbar\""</code></li>
                <li><code>"aria-label"</code> " (from label prop)"</li>
                <li><code>"aria-orientation"</code> " (horizontal or vertical)"</li>
                <li><code>"aria-disabled"</code> " when disabled"</li>
            </ul>

            <h2 id="roving-tabindex" class="anchor">
                "Roving Tabindex"
                <AnchorLink href="#roving-tabindex" description="Direct link to roving tabindex"/>
            </h2>

            <p>
                "Toolbars typically use roving tabindex for navigation. Only one item has "
                <code>"tabindex=\"0\""</code> " while others have " <code>"tabindex=\"-1\""</code> ". "
                "This allows users to Tab into the toolbar once, then use arrow keys to navigate within."
            </p>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Horizontal and vertical orientation"</li>
                <li>"Keyboard navigation callbacks"</li>
                <li>"Home/End key support"</li>
                <li>"Disabled state"</li>
                <li>"Full ARIA toolbar support"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_toolbar", link: "#use_toolbar" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "Orientation", link: "#orientation" },
                Toc::Leaf { title: "Keyboard Navigation", link: "#keyboard-navigation" },
                Toc::Leaf { title: "ARIA Attributes", link: "#aria-attributes" },
                Toc::Leaf { title: "Roving Tabindex", link: "#roving-tabindex" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}

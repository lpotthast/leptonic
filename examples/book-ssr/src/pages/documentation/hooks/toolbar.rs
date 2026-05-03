use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::toolbar_horizontal::ToolbarHorizontalDemo;
use super::demos::toolbar_vertical::ToolbarVerticalDemo;

#[component]
pub fn PageUseToolbar() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use_toolbar" class="anchor">
                "use_toolbar"
                <AnchorLink href="#use_toolbar" description="Direct link to article header"/>
            </h1>

            <p>
                "The "<Code inline=true>"use_toolbar"</Code>" hook is a standalone hook for creating accessible toolbars that group related controls together."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useToolbar.html" target=LinkTarget::_Blank>
                    "useToolbar"
                </LinkExt>
                "."
            </p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <h3>"Horizontal Toolbar"</h3>
            <DemoShell source=include_str!("demos/toolbar_horizontal.rs")>
                <ToolbarHorizontalDemo />
            </DemoShell>

            <h3>"Vertical Toolbar"</h3>
            <DemoShell source=include_str!("demos/toolbar_vertical.rs")>
                <ToolbarVerticalDemo />
            </DemoShell>

            <Code language=Language::Rust>
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
                        <div {..toolbar.toolbar_props.into_attrs()}>
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

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Navigation.materialize()>"Navigation domain"</Link></li>
                <li><Link href=crate::routes::doc::focus::UseFocusManager.materialize()>"use_focus_manager"</Link></li>
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
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}

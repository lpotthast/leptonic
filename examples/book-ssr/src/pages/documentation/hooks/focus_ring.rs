use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use indoc::indoc;

use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn PageUseFocusRing() -> impl IntoView {
    let focus_ring = use_focus_ring(UseFocusRingInput::default());
    let focus_ring_within = use_focus_ring(UseFocusRingInput {
        within: true,
        ..Default::default()
    });

    view! {
        <Article>
            <h1 id="use_focus_ring" class="anchor">
                "use_focus_ring"
                <AnchorLink href="#use_focus_ring" description="Direct link to article header"/>
            </h1>

            <p>"Determine when to show a focus ring for accessibility. The focus ring should only be visible during keyboard navigation, not when using mouse or touch."</p>

            <Code>
                {indoc!(r#"
                    let focus_ring = use_focus_ring(UseFocusRingInput::default());

                    view! {
                        <button
                            {..focus_ring.props.into_attrs()}
                            tabindex="0"
                        >
                            "Focus me"
                        </button>
                    }

                    // CSS handles styling automatically:
                    // [data-focus-visible="true"] {
                    //     outline: 3px solid var(--brand-color);
                    //     outline-offset: 2px;
                    // }
                "#)}
            </Code>

            <h2 id="keyboard-vs-mouse" class="anchor">
                "Keyboard vs Mouse Focus"
                <AnchorLink href="#keyboard-vs-mouse" description="Direct link to keyboard vs mouse"/>
            </h2>

            <p>"Try both clicking and tabbing to the button below. The focus ring only appears when using keyboard navigation. The " <code>"data-focus-visible"</code> " attribute is automatically added when the focus ring should be visible, and CSS handles the styling:"</p>

            <button
                {..focus_ring.props.into_attrs()}
                tabindex="0"
                style="padding: 1em 2em; font-size: 1em; border-radius: 8px; cursor: pointer; border: 2px solid #ccc; background: white; transition: all 0.2s;"
            >
                "Tab to me (keyboard) or click me (mouse)"
            </button>

            <p style="margin-top: 1em;">
                "Is focused: " <strong>{ move || focus_ring.is_focused.get().to_string() }</strong>
                " | Focus ring visible: " <strong>{ move || focus_ring.is_focus_visible.get().to_string() }</strong>
            </p>

            <h2 id="within-mode" class="anchor">
                "Within Mode"
                <AnchorLink href="#within-mode" description="Direct link to within mode"/>
            </h2>

            <p>"With " <code>"within: true"</code> ", the focus ring tracks focus within the element's subtree using " <code>"focusin"</code> "/" <code>"focusout"</code> " events. The " <code>"data-focus-visible"</code> " attribute is set on the container when any descendant is focused via keyboard:"</p>

            <Code>
                {indoc!(r#"
                    let focus_ring_within = use_focus_ring(UseFocusRingInput {
                        within: true,
                        ..Default::default()
                    });

                    view! {
                        <div {..focus_ring_within.props.into_attrs()}>
                            <input type="text" placeholder="Tab here..." />
                            <button>"Or here"</button>
                        </div>
                    }
                "#)}
            </Code>

            <div
                {..focus_ring_within.props.into_attrs()}
                style="padding: 1em; border-radius: 8px; border: 2px solid #ccc; display: flex; gap: 0.5em; align-items: center; transition: all 0.2s;"
            >
                <input
                    type="text"
                    placeholder="Tab here..."
                    style="padding: 0.5em; border-radius: 4px; border: 1px solid #ccc;"
                />
                <button
                    style="padding: 0.5em 1em; border-radius: 4px; cursor: pointer; border: 1px solid #ccc; background: white;"
                >
                    "Or here"
                </button>
            </div>

            <p style="margin-top: 1em;">
                "Focus within: " <strong>{ move || focus_ring_within.is_focused.get().to_string() }</strong>
                " | Focus ring (within) visible: " <strong>{ move || focus_ring_within.is_focus_visible.get().to_string() }</strong>
            </p>

            <h2 id="use_focus_visible" class="anchor">
                "use_focus_visible"
                <AnchorLink href="#use_focus_visible" description="Direct link to use_focus_visible"/>
            </h2>

            <p><code>"use_focus_ring"</code> " internally uses " <code>"use_focus_visible"</code> " which tracks the global focus modality (keyboard vs pointer). This hook detects:"</p>

            <ul>
                <li>"Keyboard navigation (Tab, Shift+Tab, arrow keys)"</li>
                <li>"Mouse clicks and touch interactions"</li>
                <li>"The modality changes based on the last interaction type"</li>
            </ul>

            <Code>
                {indoc!(r"
                    // Lower-level hook for focus modality detection
                    let focus_visible = use_focus_visible(UseFocusVisibleInput {
                        auto_focus: false,
                    });

                    // focus_visible.is_focus_visible tracks global keyboard modality
                ")}
            </Code>

            <h2 id="data-attribute" class="anchor">
                "Data Attribute Styling"
                <AnchorLink href="#data-attribute" description="Direct link to data attribute"/>
            </h2>

            <p>"The hook automatically adds a " <code>"data-focus-visible"</code> " attribute when the focus ring should be visible. This allows centralized CSS styling:"</p>

            <Code>
                {indoc!(r#"
                    /* In your CSS */
                    [data-focus-visible="true"] {
                        outline: 3px solid var(--brand-color);
                        outline-offset: 2px;
                    }
                "#)}
            </Code>

            <p>"This approach has several benefits:"</p>

            <ul>
                <li>"No manual style computation in your components"</li>
                <li>"Consistent focus ring styling across your entire application"</li>
                <li>"Easy to customize in one place"</li>
                <li>"The signal " <code>"is_focus_visible"</code> " is still available if you need programmatic access"</li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Distinguishes between keyboard and pointer focus"</li>
                <li>"Tracks both focus state and focus visibility separately"</li>
                <li>"Automatic " <code>"data-focus-visible"</code> " attribute for CSS styling"</li>
                <li><code>"within"</code> " mode tracks focus within descendants (for containers with focusable children)"</li>
                <li>"Supports auto-focus scenarios"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_focus_ring", link: "#use_focus_ring" },
                Toc::Leaf { title: "Keyboard vs Mouse Focus", link: "#keyboard-vs-mouse" },
                Toc::Leaf { title: "Within Mode", link: "#within-mode" },
                Toc::Leaf { title: "use_focus_visible", link: "#use_focus_visible" },
                Toc::Leaf { title: "Data Attribute Styling", link: "#data-attribute" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}

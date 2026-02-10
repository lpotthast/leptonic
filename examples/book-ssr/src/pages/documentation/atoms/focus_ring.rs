use indoc::indoc;
use leptonic::atoms::focus_ring::FocusRing;

use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;

#[component]
pub fn PageAtomFocusRing() -> impl IntoView {
    view! {
        <Article>
            <h1 id="focus-ring" class="anchor">
                "FocusRing"
                <AnchorLink href="#focus-ring" description="Direct link to article header"/>
            </h1>

            <p>
                "FocusRing provides keyboard-only focus ring visibility for accessibility. "
                "It tracks whether an element is focused via keyboard navigation (as opposed to mouse/touch) and exposes this state through a data attribute for CSS styling."
            </p>

            <h2 id="props" class="anchor">
                "Props"
                <AnchorLink href="#props" description="Direct link to props section"/>
            </h2>

            <ul>
                <li><Code inline=true>"disabled"</Code>" - When true, focus tracking is disabled."</li>
                <li><Code inline=true>"within"</Code>" - When true, tracks focus within descendants, not just on the element itself."</li>
                <li><Code inline=true>"auto_focus"</Code>" - When true, the element receives focus when mounted."</li>
                <li><Code inline=true>"on_focus"</Code>" - Callback invoked when the element receives focus."</li>
                <li><Code inline=true>"on_blur"</Code>" - Callback invoked when the element loses focus."</li>
                <li><Code inline=true>"on_focus_change"</Code>" - Callback invoked with a boolean when focus state changes."</li>
            </ul>

            <h2 id="example" class="anchor">
                "Example"
                <AnchorLink href="#example" description="Direct link to example section"/>
            </h2>

            <p>
                "Wrap any focusable element with FocusRing. The element will receive a "<Code inline=true>"data-focus-visible"</Code>" attribute when focused via keyboard."
            </p>

            <Code>
                {indoc!(r#"
                    use leptonic::atoms::focus_ring::FocusRing;

                    view! {
                        <FocusRing>
                            <button class="my-button">"Click or Tab to me"</button>
                        </FocusRing>
                    }
                "#)}
            </Code>

            <h2 id="styling" class="anchor">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to styling section"/>
            </h2>

            <p>
                "Use the "<Code inline=true>"data-focus-visible"</Code>" attribute in your CSS to style the focus ring. "
                "This attribute is only present when the element is focused via keyboard navigation."
            </p>

            <Code>
                {indoc!(r"
                    .my-button {
                        outline: none;
                    }

                    .my-button[data-focus-visible] {
                        outline: 2px solid var(--brand-color);
                        outline-offset: 2px;
                    }
                ")}
            </Code>

            <h2 id="demo" class="anchor">
                "Demo"
                <AnchorLink href="#demo" description="Direct link to demo section"/>
            </h2>

            <p>
                "The button below shows a focus ring only when focused via keyboard (Tab key). "
                "Click on it first - no ring appears. Then Tab to it - the focus ring becomes visible."
            </p>

            <style>
                "
                .demo-focus-ring-button {
                    padding: 0.75em 1.5em;
                    font-size: 1em;
                    cursor: pointer;
                    background-color: var(--brand-color);
                    color: white;
                    border: none;
                    border-radius: 0.25em;
                    outline: none;
                }
                .demo-focus-ring-button[data-focus-visible] {
                    outline: 3px solid var(--brand-color);
                    outline-offset: 3px;
                }
                "
            </style>

            <FocusRing>
                <button class="demo-focus-ring-button">"Focus me with Tab"</button>
            </FocusRing>

            <h2 id="context" class="anchor">
                "FocusRingContext"
                <AnchorLink href="#context" description="Direct link to context section"/>
            </h2>

            <p>
                "FocusRing provides a "<Code inline=true>"FocusRingContext"</Code>" through Leptos context. "
                "Child components can access this context to read the current focus state."
            </p>

            <Code>
                {indoc!(r"
                    use leptonic::atoms::focus_ring::{FocusRing, FocusRingContext};

                    #[component]
                    fn MyComponent() -> impl IntoView {
                        let ctx = expect_context::<FocusRingContext>();

                        // Check if the element is focused
                        let is_focused = ctx.is_focused.get();

                        // Check if focus is visible (keyboard-focused)
                        let is_focus_visible = ctx.is_focus_visible.get();
                    }
                ")}
            </Code>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "FocusRing", link: "#focus-ring" },
                Toc::Leaf { title: "Props", link: "#props" },
                Toc::Leaf { title: "Example", link: "#example" },
                Toc::Leaf { title: "Styling", link: "#styling" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "FocusRingContext", link: "#context" },
            ]
        }/>
    }
}

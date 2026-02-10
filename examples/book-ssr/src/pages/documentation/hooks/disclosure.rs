use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use indoc::indoc;

use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn PageUseDisclosure() -> impl IntoView {
    let UseDisclosureStateReturn {
        is_expanded,
        toggle,
        ..
    } = use_disclosure_state(false);

    let UseDisclosureReturn {
        trigger_props,
        content_props,
        ..
    } = use_disclosure(UseDisclosureInput {
        is_expanded: is_expanded.into(),
        is_disabled: Signal::derive(|| false),
        on_expanded_change: None,
    });

    view! {
        <Article>
            <h1 id="use_disclosure" class="anchor">
                "use_disclosure"
                <AnchorLink href="#use_disclosure" description="Direct link to article header"/>
            </h1>

            <p>"Hook for creating collapsible/expandable content sections with proper ARIA attributes for accessibility."</p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <div style="border: 1px solid #ddd; border-radius: 8px; overflow: hidden; margin: 1em 0; max-width: 500px;">
                <button
                    {..trigger_props.into_attrs()}
                    on:click=move |_| toggle.run(())
                    style="
                        width: 100%;
                        padding: 1em;
                        background: #f5f5f5;
                        border: none;
                        cursor: pointer;
                        display: flex;
                        justify-content: space-between;
                        align-items: center;
                        font-size: 1em;
                        font-weight: 500;
                    "
                >
                    <span>"What is disclosure?"</span>
                    <span style=move || format!(
                        "transition: transform 0.2s; {}",
                        if is_expanded.get() { "transform: rotate(180deg);" } else { "" }
                    )>"▼"</span>
                </button>

                <div
                    {..content_props.into_attrs()}
                    style=move || format!(
                        "overflow: hidden; transition: all 0.3s; {}",
                        if is_expanded.get() { "max-height: 200px; padding: 1em;" } else { "max-height: 0; padding: 0 1em;" }
                    )
                >
                    <p style="margin: 0;">
                        "A disclosure is a widget that shows or hides content. It consists of a button that toggles the visibility of a panel. This is commonly used for FAQs, accordions, and collapsible sections."
                    </p>
                </div>
            </div>

            <Code>
                {indoc!(r#"
                    let UseDisclosureStateReturn { is_expanded, toggle, .. } = use_disclosure_state(false);

                    let UseDisclosureReturn { trigger_attrs, content_attrs } = use_disclosure(UseDisclosureInput {
                        is_expanded: is_expanded.into(),
                        on_expanded_change: None,
                    });

                    view! {
                        <button {..trigger_attrs} on:click=move |_| toggle.run(())>
                            "Toggle Content"
                        </button>
                        <div {..content_attrs}>
                            "Hidden content here..."
                        </div>
                    }
                "#)}
            </Code>

            <h2 id="aria-attributes" class="anchor">
                "ARIA Attributes"
                <AnchorLink href="#aria-attributes" description="Direct link to ARIA attributes"/>
            </h2>

            <p>"The hook automatically manages:"</p>
            <ul>
                <li><code>"aria-expanded"</code> " on the trigger button"</li>
                <li><code>"aria-controls"</code> " linking trigger to content"</li>
                <li><code>"id"</code> " on the content panel"</li>
                <li><code>"hidden"</code> " attribute on collapsed content"</li>
            </ul>

            <h2 id="use-cases" class="anchor">
                "Use Cases"
                <AnchorLink href="#use-cases" description="Direct link to use cases"/>
            </h2>

            <ul>
                <li>"FAQ accordions"</li>
                <li>"Collapsible sections"</li>
                <li>"Expandable cards"</li>
                <li>"Show more/less content"</li>
                <li>"Details/summary patterns"</li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Controlled expanded state"</li>
                <li>"Toggle and explicit open/close methods"</li>
                <li>"Proper ARIA attributes"</li>
                <li>"ID association between trigger and content"</li>
                <li>"Hidden attribute for collapsed state"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_disclosure", link: "#use_disclosure" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "ARIA Attributes", link: "#aria-attributes" },
                Toc::Leaf { title: "Use Cases", link: "#use-cases" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}

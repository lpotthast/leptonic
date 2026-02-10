use indoc::indoc;

use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptonic::ScrollBehavior;
use leptos::prelude::*;

use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;

#[component]
pub fn PageUseAnchorLink() -> impl IntoView {
    let (disabled, set_disabled) = signal(false);

    let UseAnchorLinkReturn {
        props,
        is_pressed: _,
        ..
    } = use_anchor_link(UseAnchorLinkInput {
        href: Href::from_str(Oco::Borrowed("#my-anchor-element")).expect("valid href"),
        scroll_behavior: Some(ScrollBehavior::Smooth),
        disabled: disabled.into(),
        element_type: Default::default(),
        description: None,
        on_press: None,
        on_press_start: None,
        on_press_end: None,
    });
    let attrs = props.into_attrs();

    view! {
        <Article>
            <h1 id="use-anchor-link" class="anchor">
                "use_anchor_link"
                <AnchorLink href="#use-anchor-link" description="Direct link to article header"/>
            </h1>

            <p>"Hook for creating accessible in-page anchor links with smooth scrolling and URL hash updates."</p>

            <p>
                "Composes " <code>"use_focusable"</code> ", " <code>"use_press"</code> ", and " <code>"use_focus_ring"</code>
                " to provide the same interaction quality as " <code>"use_link"</code> " but specialized for anchor navigation."
            </p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <Code>
                {indoc!(r##"
                    let UseAnchorLinkReturn { props, .. } = use_anchor_link(UseAnchorLinkInput {
                        href: Href::from_str(Oco::Borrowed("#my-anchor-element")).expect("valid href"),
                        scroll_behavior: Some(ScrollBehavior::Smooth),
                        disabled: disabled.into(),
                        element_type: Default::default(),
                        description: None,
                        on_press: None,
                        on_press_start: None,
                        on_press_end: None,
                    });

                    view! {
                        <a {..props.into_attrs()} class="leptonic-anchor-link" target="_self">
                            "#"
                        </a>
                    }
                "##)}
            </Code>

            <a
                {..attrs}
                class="leptonic-anchor-link"
                target="_self"
            >
                "#"
            </a>

            <FormControl attr:style="flex-direction: row; align-items: center; gap: 0.5em;">
                <Checkbox checked=disabled set_checked=set_disabled />
                <Label>"Disabled"</Label>
            </FormControl>

            <div id="my-anchor-element" style="margin-top: 1em; padding: 1em; border: 1px solid var(--brand-color); border-radius: 4px;">
                "This is the anchor target element."
            </div>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Composes " <code>"use_press"</code> " for robust press interactions (pointer, keyboard, drag cancellation)"</li>
                <li>"Composes " <code>"use_focusable"</code> " for focus/blur handling, auto-focus, and tabindex management"</li>
                <li>"Composes " <code>"use_focus_ring"</code> " for keyboard-only focus ring visibility"</li>
                <li>"Smooth or instant scrolling to target element"</li>
                <li>"URL hash update without page reload"</li>
                <li>"Disabled state handling"</li>
                <li>"Pressed state tracking (" <code>"is_pressed"</code> ")"</li>
                <li><code>"on_press_start"</code> " / " <code>"on_press_end"</code> " callbacks for press lifecycle"</li>
                <li>"Programmatic focus via " <code>"FocusHandle"</code></li>
            </ul>

            <h2 id="aria-attributes" class="anchor">
                "ARIA Attributes"
                <AnchorLink href="#aria-attributes" description="Direct link to ARIA attributes"/>
            </h2>

            <p>"The hook automatically sets:"</p>
            <ul>
                <li><code>"href"</code> " pointing to the anchor target"</li>
                <li><code>"aria-label"</code> " from the " <code>"description"</code> " input"</li>
                <li><code>"aria-disabled"</code> " for disabled state"</li>
                <li><code>"tabindex"</code> " managed by " <code>"use_focusable"</code></li>
                <li><code>"data-focus-visible"</code> " for keyboard-only focus ring"</li>
            </ul>

            <h2 id="deviations" class="anchor">
                "Deviations from react-aria"
                <AnchorLink href="#deviations" description="Direct link to deviations"/>
            </h2>

            <ul>
                <li>
                    "This hook has no direct react-aria equivalent. React-aria's " <code>"useLink"</code>
                    " does not handle in-page anchor scrolling. This hook extends that pattern with "
                    "scroll-to-element behavior and URL hash management."
                </li>
                <li>
                    <code>"use_focus_ring"</code>
                    " is composed directly in the hook. React-aria handles focus ring visibility at the component level."
                </li>
            </ul>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to see also"/>
            </h2>

            <ul>
                <li><code>"use_link"</code> " - For general-purpose links (internal, external, non-anchor elements)"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_anchor_link", link: "#use-anchor-link" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "ARIA Attributes", link: "#aria-attributes" },
                Toc::Leaf { title: "Deviations", link: "#deviations" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}

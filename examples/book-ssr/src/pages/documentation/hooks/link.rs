use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use indoc::indoc;

use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptonic::prelude::Size;
use leptos::prelude::*;

#[component]
pub fn PageUseLink() -> impl IntoView {
    let internal_link = use_link(UseLinkInput {
        href: Some("#demo".to_string()),
        target: None,
        rel: vec![],
        is_disabled: Default::default(),
        element_type: Default::default(),
        aria_current: None,
        on_press: None,
        on_press_start: None,
        on_press_end: None,
    });

    let external_link = use_link(UseLinkInput {
        href: Some("https://leptos.dev".to_string()),
        target: Some(LinkTarget::_Blank),
        rel: vec![LinkRel::NoOpener, LinkRel::NoReferrer],
        is_disabled: Default::default(),
        element_type: Default::default(),
        aria_current: None,
        on_press: None,
        on_press_start: None,
        on_press_end: None,
    });

    let (is_disabled, set_is_disabled) = signal(false);
    let disabled_link = use_link(UseLinkInput {
        href: Some("#".to_string()),
        target: None,
        rel: vec![],
        is_disabled: is_disabled.into(),
        element_type: Default::default(),
        aria_current: None,
        on_press: None,
        on_press_start: None,
        on_press_end: None,
    });

    let pressed_link = use_link(UseLinkInput {
        href: Some("#pressed-state".to_string()),
        target: None,
        rel: vec![],
        is_disabled: Default::default(),
        element_type: Default::default(),
        aria_current: None,
        on_press: None,
        on_press_start: None,
        on_press_end: None,
    });
    let pressed_link_is_pressed = pressed_link.is_pressed;

    let span_link = use_link(UseLinkInput {
        href: None,
        target: None,
        rel: vec![],
        is_disabled: Default::default(),
        element_type: LinkElementType::Span,
        aria_current: None,
        on_press: Some(Callback::new(|_| {
            leptos::logging::log!("Span link pressed!");
        })),
        on_press_start: None,
        on_press_end: None,
    });

    let focus_link = use_link(UseLinkInput {
        href: Some("#focus-handle".to_string()),
        target: None,
        rel: vec![],
        is_disabled: Default::default(),
        element_type: Default::default(),
        aria_current: None,
        on_press: None,
        on_press_start: None,
        on_press_end: None,
    });
    let focus_handle = focus_link.focus_handle;

    view! {
        <Article>
            <h1 id="use_link" class="anchor">
                "use_link"
                <AnchorLink href="#use_link" description="Direct link to article header"/>
            </h1>

            <p>"Hook for creating accessible links with robust press handling, focus management, and keyboard navigation."</p>

            <p>
                "Composes " <code>"use_focusable"</code> ", " <code>"use_press"</code> ", and " <code>"use_focus_ring"</code>
                " to provide the same interaction quality as " <code>"use_button"</code> " but for link elements."
            </p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <Stack orientation=StackOrientation::Vertical spacing=Size::Em(1.0)>
                <div>
                    <strong>"Internal Link: "</strong>
                    <a {..internal_link.link_props} style="color: var(--brand-color);">
                        "Jump to demo section"
                    </a>
                </div>

                <div>
                    <strong>"External Link: "</strong>
                    <a {..external_link.link_props} style="color: var(--brand-color);">
                        "Visit Leptos"
                        <span style="margin-left: 0.25em;">"↗"</span>
                    </a>
                    <span style="margin-left: 0.5em; font-size: 0.875em; opacity: 0.7;">
                        "(opens in new tab)"
                    </span>
                </div>

                <div>
                    <strong>"Disabled Link: "</strong>
                    <a
                        {..disabled_link.link_props}
                        style=move || format!(
                            "color: {}; cursor: {};",
                            if is_disabled.get() { "#999" } else { "var(--brand-color)" },
                            if is_disabled.get() { "not-allowed" } else { "pointer" }
                        )
                    >
                        "This link can be disabled"
                    </a>
                </div>

                <label style="display: flex; align-items: center; gap: 0.5em; cursor: pointer;">
                    <input
                        type="checkbox"
                        prop:checked=is_disabled
                        on:change=move |e| set_is_disabled.set(event_target_checked(&e))
                    />
                    "Disable link"
                </label>
            </Stack>

            <Code>
                {indoc!(r#"
                    let link = use_link(UseLinkInput {
                        href: Some("https://example.com".to_string()),
                        target: Some("_blank"),
                        rel: vec![LinkRel::NoOpener, LinkRel::NoReferrer],
                        ..Default::default()
                    });

                    view! {
                        <a {..link.link_props}>"External Link"</a>
                    }
                "#)}
            </Code>

            <h2 id="pressed-state" class="anchor">
                "Pressed State"
                <AnchorLink href="#pressed-state" description="Direct link to pressed state"/>
            </h2>

            <p>
                "The hook exposes " <code>"is_pressed"</code> " (from " <code>"use_press"</code>
                ") for visual feedback during interactions. Press and hold the link below to see the effect."
            </p>

            <Stack orientation=StackOrientation::Vertical spacing=Size::Em(1.0)>
                <div>
                    <a
                        {..pressed_link.link_props}
                        style=move || format!(
                            "color: var(--brand-color); transition: transform 100ms; transform: {};",
                            if pressed_link_is_pressed.get() { "scale(0.95)" } else { "scale(1)" }
                        )
                    >
                        "Press and hold me"
                    </a>
                    <span style="margin-left: 0.5em; font-size: 0.875em; opacity: 0.7;">
                        {move || if pressed_link_is_pressed.get() { "(pressed!)" } else { "" }}
                    </span>
                </div>
            </Stack>

            <h2 id="non-anchor-elements" class="anchor">
                "Non-Anchor Elements"
                <AnchorLink href="#non-anchor-elements" description="Direct link to non-anchor elements"/>
            </h2>

            <p>
                "Links can use non-anchor elements like " <code>"<span>"</code> " or " <code>"<button>"</code>
                ". The hook automatically adds " <code>"role=\"link\""</code>
                " and keyboard handling (Enter via " <code>"use_press"</code> ")."
            </p>

            <Stack orientation=StackOrientation::Vertical spacing=Size::Em(1.0)>
                <div>
                    <strong>"Span as Link: "</strong>
                    <span
                        {..span_link.link_props}
                        style="color: var(--brand-color); cursor: pointer; text-decoration: underline;"
                    >
                        "Click or press Enter (check console)"
                    </span>
                </div>
            </Stack>

            <h2 id="focus-handle" class="anchor">
                "Programmatic Focus"
                <AnchorLink href="#focus-handle" description="Direct link to focus handle"/>
            </h2>

            <p>
                "The hook provides a " <code>"FocusHandle"</code>
                " for programmatic focus control."
            </p>

            <Stack orientation=StackOrientation::Vertical spacing=Size::Em(1.0)>
                <div>
                    <a {..focus_link.link_props} style="color: var(--brand-color);">
                        "Target link"
                    </a>
                </div>
                <button on:click=move |_| focus_handle.focus()>
                    "Focus the link above"
                </button>
            </Stack>

            <h2 id="element-types" class="anchor">
                "Element Types"
                <AnchorLink href="#element-types" description="Direct link to element types"/>
            </h2>

            <p>"Links can be rendered as different elements:"</p>
            <ul>
                <li><code>"LinkElementType::Anchor"</code> " - Standard anchor element (default)"</li>
                <li><code>"LinkElementType::Span"</code> " - Span styled as link (gets role=\"link\")"</li>
                <li><code>"LinkElementType::Button"</code> " - Button styled as link (gets role=\"link\")"</li>
            </ul>

            <h2 id="aria-attributes" class="anchor">
                "ARIA Attributes"
                <AnchorLink href="#aria-attributes" description="Direct link to ARIA attributes"/>
            </h2>

            <p>"The hook automatically sets:"</p>
            <ul>
                <li><code>"target"</code> " and " <code>"rel"</code> " from user-provided values"</li>
                <li><code>"role=\"link\""</code> " for non-anchor elements"</li>
                <li><code>"aria-current"</code> " for marking the current item in a navigation set"</li>
                <li><code>"aria-disabled"</code> " for disabled state"</li>
                <li><code>"tabindex"</code> " managed by " <code>"use_focusable"</code> " (disabled → removed, exclude_from_tab_order → -1, normal → 0)"</li>
                <li><code>"data-focus-visible"</code> " for keyboard-only focus ring"</li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Composes " <code>"use_press"</code> " for robust press interactions (pointer, keyboard, drag cancellation)"</li>
                <li>"Composes " <code>"use_focusable"</code> " for focus/blur handling, auto-focus, and tabindex management"</li>
                <li>"Composes " <code>"use_focus_ring"</code> " for keyboard-only focus ring visibility"</li>
                <li>"Internal and external link support"</li>
                <li>"Type-safe " <code>"rel"</code> " attribute via " <code>"LinkRel"</code> " enum"</li>
                <li>"Multiple element type support"</li>
                <li>"Disabled state handling"</li>
                <li>"Pressed state tracking (" <code>"is_pressed"</code> ")"</li>
                <li><code>"on_press_start"</code> " / " <code>"on_press_end"</code> " callbacks for press lifecycle"</li>
                <li>"Programmatic focus via " <code>"FocusHandle"</code></li>
            </ul>

            <h2 id="deviations" class="anchor">
                "Deviations from react-aria"
                <AnchorLink href="#deviations" description="Direct link to deviations"/>
            </h2>

            <ul>
                <li>
                    "Client-side router integration is not handled at the hook level. "
                    "Leptos router handles this at the component level via " <code>"<A>"</code> "."
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
                <li><code>"use_anchor_link"</code> " - For in-page anchor navigation with smooth scrolling"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_link", link: "#use_link" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "Pressed State", link: "#pressed-state" },
                Toc::Leaf { title: "Non-Anchor Elements", link: "#non-anchor-elements" },
                Toc::Leaf { title: "Programmatic Focus", link: "#focus-handle" },
                Toc::Leaf { title: "Element Types", link: "#element-types" },
                Toc::Leaf { title: "ARIA Attributes", link: "#aria-attributes" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "Deviations", link: "#deviations" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}

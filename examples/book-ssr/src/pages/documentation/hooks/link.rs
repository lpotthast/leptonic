use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use indoc::indoc;
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptonic::prelude::Size;
use leptos::prelude::*;

#[component]
pub fn PageUseLink() -> impl IntoView {
    let internal_link = use_link(UseLinkInput {
        href: Some("#demo".to_string()),
        is_external: false,
        on_press: Some(Callback::new(|_| {})),
        ..Default::default()
    });

    let external_link = use_link(UseLinkInput {
        href: Some("https://leptos.dev".to_string()),
        is_external: true,
        ..Default::default()
    });

    let (is_disabled, set_is_disabled) = signal(false);
    let disabled_link = use_link(UseLinkInput {
        href: Some("#".to_string()),
        is_disabled: is_disabled.into(),
        ..Default::default()
    });

    view! {
        <Article>
            <h1 id="use_link" class="anchor">
                "use_link"
                <AnchorLink href="#use_link" description="Direct link to article header"/>
            </h1>

            <p>"Hook for creating accessible links with support for external links and custom element types."</p>

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
                    // Internal link
                    let link = use_link(UseLinkInput {
                        href: Some("/about".to_string()),
                        is_external: false,
                        on_press: Some(Callback::new(|_| navigate("/about"))),
                        ..Default::default()
                    });

                    // External link - automatically adds target="_blank" and rel="noopener noreferrer"
                    let external = use_link(UseLinkInput {
                        href: Some("https://example.com".to_string()),
                        is_external: true,
                        ..Default::default()
                    });

                    view! {
                        <a {..link.link_props}>"Internal Link"</a>
                        <a {..external.link_props}>"External Link"</a>
                    }
                "#)}
            </Code>

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
                <li><code>"target=\"_blank\""</code> " for external links"</li>
                <li><code>"rel=\"noopener noreferrer\""</code> " for security"</li>
                <li><code>"role=\"link\""</code> " for non-anchor elements"</li>
                <li><code>"aria-disabled"</code> " for disabled state"</li>
                <li><code>"tabindex"</code> " for keyboard navigation"</li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Internal and external link support"</li>
                <li>"Security attributes for external links"</li>
                <li>"Multiple element type support"</li>
                <li>"Disabled state handling"</li>
                <li>"Keyboard activation (Enter/Space for non-anchors)"</li>
                <li>"Press callback for custom behavior"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_link", link: "#use_link" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "Element Types", link: "#element-types" },
                Toc::Leaf { title: "ARIA Attributes", link: "#aria-attributes" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}

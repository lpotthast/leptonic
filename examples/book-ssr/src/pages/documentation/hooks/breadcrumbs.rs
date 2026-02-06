use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn PageUseBreadcrumbs() -> impl IntoView {
    let UseBreadcrumbsReturn { nav_props, .. } = use_breadcrumbs(UseBreadcrumbsInput {
        label: Some("Navigation".to_string()),
        ..Default::default()
    });

    let home_item = use_breadcrumb_item(UseBreadcrumbItemInput {
        href: Some("#".to_string()),
        is_current: false,
        on_press: Some(Callback::new(|_| {})),
        ..Default::default()
    });

    let products_item = use_breadcrumb_item(UseBreadcrumbItemInput {
        href: Some("#".to_string()),
        is_current: false,
        on_press: Some(Callback::new(|_| {})),
        ..Default::default()
    });

    let current_item = use_breadcrumb_item(UseBreadcrumbItemInput {
        href: None,
        is_current: true,
        ..Default::default()
    });

    view! {
        <Article>
            <h1 id="use_breadcrumbs" class="anchor">
                "use_breadcrumbs"
                <AnchorLink href="#use_breadcrumbs" description="Direct link to article header"/>
            </h1>

            <p>"Hook for creating accessible breadcrumb navigation trails."</p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <nav {..nav_props} style="margin: 1em 0;">
                <ol style="display: flex; list-style: none; padding: 0; margin: 0; gap: 0.5em;">
                    <li>
                        <a {..home_item.link_props} style="color: var(--brand-color); text-decoration: none;">
                            "Home"
                        </a>
                        <span style="margin-left: 0.5em;">"/"</span>
                    </li>
                    <li>
                        <a {..products_item.link_props} style="color: var(--brand-color); text-decoration: none;">
                            "Products"
                        </a>
                        <span style="margin-left: 0.5em;">"/"</span>
                    </li>
                    <li>
                        <span {..current_item.link_props} style="color: inherit;">
                            "Widget Pro"
                        </span>
                    </li>
                </ol>
            </nav>

            <Code>
                {r#"let UseBreadcrumbsReturn { nav_props, .. } = use_breadcrumbs(UseBreadcrumbsInput {
    label: Some("Navigation".to_string()),
    ..Default::default()
});

let home_item = use_breadcrumb_item(UseBreadcrumbItemInput {
    href: Some("/".to_string()),
    is_current: false,
    on_press: Some(Callback::new(|_| navigate("/"))),
    ..Default::default()
});

view! {
    <nav {..nav_props}>
        <ol>
            <li>
                <a {..home_item.link_props}>"Home"</a>
            </li>
            // More items...
        </ol>
    </nav>
}"#}
            </Code>

            <h2 id="aria-attributes" class="anchor">
                "ARIA Attributes"
                <AnchorLink href="#aria-attributes" description="Direct link to ARIA attributes"/>
            </h2>

            <p>"The hooks automatically set:"</p>
            <ul>
                <li><code>"aria-label"</code> " on the navigation element"</li>
                <li><code>"aria-current=\"page\""</code> " on the current item"</li>
                <li><code>"aria-disabled"</code> " for disabled items"</li>
                <li>"Proper " <code>"tabindex"</code> " management"</li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Accessible navigation landmark"</li>
                <li>"Current page indication"</li>
                <li>"Keyboard navigation support"</li>
                <li>"Disabled state handling"</li>
                <li>"Press callback for SPA navigation"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_breadcrumbs", link: "#use_breadcrumbs" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "ARIA Attributes", link: "#aria-attributes" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}

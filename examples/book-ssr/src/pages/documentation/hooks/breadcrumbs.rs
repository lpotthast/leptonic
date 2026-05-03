use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::breadcrumbs::BreadcrumbsDemo;

#[component]
pub fn PageUseBreadcrumbs() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use_breadcrumbs" class="anchor">
                "use_breadcrumbs"
                <AnchorLink href="#use_breadcrumbs" description="Direct link to article header"/>
            </h1>

            <p>
                "The "<Code inline=true>"use_breadcrumbs"</Code>" hook is a standalone hook that provides accessible breadcrumb navigation trails."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useBreadcrumbs.html" target=LinkTarget::_Blank>
                    "useBreadcrumbs"
                </LinkExt>
                "."
            </p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <DemoShell
                source=include_str!("demos/breadcrumbs.rs")
                description="Breadcrumb navigation trail"
            >
                <BreadcrumbsDemo />
            </DemoShell>

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

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Navigation.materialize()>"Navigation domain"</Link></li>
                <li><Link href=crate::routes::doc::link::UseLink.materialize()>"use_link"</Link></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_breadcrumbs", link: "#use_breadcrumbs" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "ARIA Attributes", link: "#aria-attributes" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}

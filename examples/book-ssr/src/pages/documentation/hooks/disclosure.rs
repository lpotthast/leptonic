use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::disclosure::DisclosureDemo;

#[component]
pub fn PageUseDisclosure() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use_disclosure" class="anchor">
                "use_disclosure"
                <AnchorLink href="#use_disclosure" description="Direct link to article header"/>
            </h1>

            <p>"Hook for creating collapsible/expandable content sections with proper ARIA attributes for accessibility. "
               "See the "<Link href=crate::routes::doc::Collapsible.materialize()>"Collapsible overview"</Link>" for concept guidance."</p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useDisclosure.html" target=LinkTarget::_Blank>
                    "useDisclosure"
                </LinkExt>
                "."
            </p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <DemoShell
                source=include_str!("demos/disclosure.rs")
                description="Collapsible content section"
            >
                <DisclosureDemo />
            </DemoShell>

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

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Collapsible.materialize()>"Collapsible overview"</Link></li>
                <li><Link href=crate::routes::doc::collapsible::Component.materialize()>"Collapsible component"</Link></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_disclosure", link: "#use_disclosure" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "ARIA Attributes", link: "#aria-attributes" },
                Toc::Leaf { title: "Use Cases", link: "#use-cases" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}

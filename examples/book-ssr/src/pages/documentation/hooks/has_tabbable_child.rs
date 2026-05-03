use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::demo_shell::DemoShell;
use crate::pages::documentation::{article::Article, toc::Toc};

use super::demos::has_tabbable_child::HasTabbableChildDemo;

#[component]
pub fn PageUseHasTabbableChild() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use_has_tabbable_child" class="anchor">
                "use_has_tabbable_child"
                <AnchorLink href="#use_has_tabbable_child" description="Direct link to article header"/>
            </h1>

            <p>
                "The "<code>"use_has_tabbable_child"</code>" hook detects whether an element contains any tabbable child elements. "
                "Useful for deciding whether a container should itself be focusable via Tab, ensuring focus trapping in modals has something to focus, or controlling skip-link visibility. "
                "See the "<Link href=crate::routes::doc::Focus.materialize()>"Focus overview"</Link>" for domain guidance."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useHasTabbableChild.html" target=LinkTarget::_Blank>
                    "useHasTabbableChild"
                </LinkExt>
                "."
            </p>

            <h2 id="basic-usage" class="anchor">
                "Basic Usage"
                <AnchorLink href="#basic-usage" description="Direct link to basic usage"/>
            </h2>

            <p>"A common pattern is to make a container focusable only when it has no tabbable children. When children are tabbable, the container uses " <code>"tabindex=\"-1\""</code> " so it can receive programmatic focus but is skipped during Tab navigation:"</p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    let UseHasTabbableChildReturn { has_tabbable_child, props } =
                        use_has_tabbable_child(UseHasTabbableChildInput::default());

                    view! {
                        <div
                            {..props.into_attrs()}
                            tabindex=move || if has_tabbable_child.get() { -1 } else { 0 }
                        >
                            <button>"Child button"</button>
                        </div>
                    }
                "#)}
            </Code>

            <h2 id="demo" class="anchor">
                "Dynamic Content"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <p>"Toggle the elements below to see how the tabbable child detection changes dynamically:"</p>

            <DemoShell source=include_str!("demos/has_tabbable_child.rs")>
                <HasTabbableChildDemo />
            </DemoShell>

            <h2 id="how-it-works" class="anchor">
                "How It Works"
                <AnchorLink href="#how-it-works" description="Direct link to how it works"/>
            </h2>

            <p>"The hook uses a " <code>"MutationObserver"</code> " to watch for changes in the container's subtree. It observes:"</p>
            <ul>
                <li><code>"childList"</code> " — detects added/removed child elements"</li>
                <li><code>"subtree"</code> " — watches the entire descendant tree, not just direct children"</li>
                <li><code>"attributes"</code> " — watches for " <code>"tabindex"</code> " and " <code>"disabled"</code> " attribute changes that affect tabbability"</li>
            </ul>

            <p>"When any mutation is observed, the hook rescans the container for tabbable elements using the same focusability logic as the focus management system."</p>

            <h2 id="input" class="anchor">
                "Input"
                <AnchorLink href="#input" description="Direct link to input"/>
            </h2>

            <p><code>"UseHasTabbableChildInput"</code> " fields:"</p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Default"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><code>"disabled"</code></TableCell>
                            <TableCell><code>"Signal<bool>"</code></TableCell>
                            <TableCell><code>"false"</code></TableCell>
                            <TableCell>"Disables observation when true."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="return-value" class="anchor">
                "Return Value"
                <AnchorLink href="#return-value" description="Direct link to return value"/>
            </h2>

            <p><code>"UseHasTabbableChildReturn"</code> " fields:"</p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><code>"has_tabbable_child"</code></TableCell>
                            <TableCell><code>"Signal<bool>"</code></TableCell>
                            <TableCell>"True when the container has at least one tabbable descendant."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"props"</code></TableCell>
                            <TableCell><code>"UseHasTabbableChildProps"</code></TableCell>
                            <TableCell>"Spread onto the container element via " <code>"props.into_attrs()"</code> " to enable subtree observation."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Reactive detection of tabbable children via " <code>"MutationObserver"</code> "."</li>
                <li>"Considers " <code>"tabindex"</code> " values and filters out disabled elements."</li>
                <li>"Handles dynamic content changes (added/removed children, attribute mutations)."</li>
                <li>"Automatic element capture via prop spreading."</li>
                <li>"Respects disabled state \u{2014} stops observation when disabled."</li>
            </ul>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Focus.materialize()>"Focus overview"</Link></li>
                <li><Link href=crate::routes::doc::focus::UseFocusManager.materialize()>"use_focus_manager"</Link></li>
                <li><Link href=crate::routes::doc::focus::FocusScope.materialize()>"FocusScope atom"</Link></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_has_tabbable_child", link: "#use_has_tabbable_child" },
                Toc::Leaf { title: "Basic Usage", link: "#basic-usage" },
                Toc::Leaf { title: "Dynamic Content", link: "#demo" },
                Toc::Leaf { title: "How It Works", link: "#how-it-works" },
                Toc::Leaf { title: "Input", link: "#input" },
                Toc::Leaf { title: "Return Value", link: "#return-value" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}

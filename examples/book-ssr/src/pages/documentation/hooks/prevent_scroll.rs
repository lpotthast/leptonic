use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::prevent_scroll::PreventScrollDemo;

#[component]
pub fn PageUsePreventScroll() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use-prevent-scroll" class="anchor">
                "use_prevent_scroll"
                <AnchorLink href="#use-prevent-scroll" description="Direct link to section: use_prevent_scroll"/>
            </h1>

            <p>
                "The "<Code inline=true>"use_prevent_scroll"</Code>" hook prevents the page from scrolling. "
                "See the "<Link href=crate::routes::doc::Interactions.materialize()>"Interactions overview"</Link>
                " and "<Link href=crate::routes::doc::Overlays.materialize()>"Overlays overview"</Link>" for domain guidance."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/usePreventScroll.html" target=LinkTarget::_Blank>
                    "usePreventScroll"
                </LinkExt>
                "."
            </p>

            <h2 id="input" class="anchor">
                "Input"
                <AnchorLink href="#input" description="Direct link to section: Input"/>
            </h2>

            <p><Code inline=true>"UsePreventScrollInput"</Code>" fields:"</p>

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
                            <TableCell><Code inline=true>"disabled"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell>"When true, scrolling is allowed. Set to false to prevent page scrolling."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="return" class="anchor">
                "Return"
                <AnchorLink href="#return" description="Direct link to section: Return"/>
            </h2>

            <p><Code inline=true>"UsePreventScrollReturn"</Code>" fields:"</p>

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
                            <TableCell><Code inline=true>"props"</Code></TableCell>
                            <TableCell><Code inline=true>"UsePreventScrollProps"</Code></TableCell>
                            <TableCell>"Empty props (no attributes needed). The hook operates globally on the document root."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="demo" class="anchor">
                "Demo"
                <AnchorLink href="#demo" description="Direct link to section: Demo"/>
            </h2>

            <Code language=Language::Rust>
                {indoc!(r#"
                    let (is_modal_open, set_is_modal_open) = signal(false);

                    // Scroll is prevented when disabled=false (i.e., when modal is open)
                    use_prevent_scroll(UsePreventScrollInput {
                        disabled: Signal::derive(move || !is_modal_open.get()),
                    });

                    view! {
                        <button on:click=move |_| set_is_modal_open.set(true)>
                            "Open Modal"
                        </button>
                    }
                "#)}
            </Code>

            <p>"Toggle scroll prevention to see the effect:"</p>

            <DemoShell source=include_str!("demos/prevent_scroll.rs")>
                <PreventScrollDemo />
            </DemoShell>

            <h2 id="how-it-works" class="anchor">
                "How it works"
                <AnchorLink href="#how-it-works" description="Direct link to section: How it works"/>
            </h2>

            <p>"On standard browsers, the hook adds "<Code inline=true>"overflow: hidden"</Code>" to the document root element and adjusts for the scrollbar width (preferring "<Code inline=true>"scrollbar-gutter: stable"</Code>" when supported, falling back to "<Code inline=true>"padding-right"</Code>") to prevent layout shift. Individual CSS properties are preserved and restored on cleanup."</p>

            <p>"On iOS Safari, "<Code inline=true>"overflow: hidden"</Code>" alone is not sufficient. The hook implements comprehensive workarounds including touch event interception, "<Code inline=true>"overscroll-behavior: contain"</Code>" injection, focus override with "<Code inline=true>"preventScroll"</Code>", and custom scroll-into-view logic that accounts for the visual viewport when the software keyboard is visible."</p>

            <h2 id="reference-counting" class="anchor">
                "Reference Counting"
                <AnchorLink href="#reference-counting" description="Direct link to section: Reference Counting"/>
            </h2>

            <p>"Multiple components can call "<Code inline=true>"use_prevent_scroll"</Code>" simultaneously. The hook uses reference counting \u{2014} scroll prevention is only removed when all components have cleaned up or disabled their prevention."</p>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to section: Features"/>
            </h2>

            <ul>
                <li>"Prevents page scrolling by setting overflow: hidden on document root"</li>
                <li>"Prefers "<Code inline=true>"scrollbar-gutter: stable"</Code>" when supported, falls back to "<Code inline=true>"padding-right"</Code></li>
                <li>"Preserves and restores original inline styles on cleanup"</li>
                <li>"Reference counted \u{2014} supports multiple simultaneous users"</li>
                <li>"Automatically cleans up on component unmount"</li>
                <li>"iOS Safari: touch event interception with pinch-zoom and text selection support"</li>
                <li>"iOS Safari: overscroll-behavior containment to prevent scroll chaining"</li>
                <li>"iOS Safari: focus override to prevent native page scrolling on input focus"</li>
                <li>"iOS Safari: custom scroll-into-view centered within scrollable parents, accounting for visual viewport"</li>
            </ul>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Interactions.materialize()>"Interactions overview"</Link></li>
                <li><Link href=crate::routes::doc::Overlays.materialize()>"Overlays overview"</Link></li>
                <li><Link href=crate::routes::doc::modal::Hook.materialize()>"use_modal"</Link></li>
            </ul>

            // Add some content to make the page scrollable
            <div style="margin-top: 2em;">
                <h3>"Sample content to enable scrolling"</h3>
                <p>"This content is here to make the page scrollable so you can test the scroll prevention."</p>
                {(0..10).map(|i| view! {
                    <p>"Paragraph " {i + 1} " \u{2014} Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."</p>
                }).collect::<Vec<_>>()}
            </div>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_prevent_scroll", link: "#use-prevent-scroll" },
                Toc::Leaf { title: "Input", link: "#input" },
                Toc::Leaf { title: "Return", link: "#return" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "How it works", link: "#how-it-works" },
                Toc::Leaf { title: "Reference Counting", link: "#reference-counting" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}

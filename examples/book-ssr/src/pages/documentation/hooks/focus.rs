use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::focus::FocusDemo;

#[component]
pub fn PageUseFocus() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use-focus" class="anchor">
                "use_focus"
                <AnchorLink href="#use-focus" description="Direct link to section: use_focus"/>
            </h1>

            <p>
                "The "<Code inline=true>"use_focus"</Code>" hook tracks element focus. It fires callbacks when the element itself receives or loses focus (not descendants \u{2014} see "
                <Link href=crate::routes::doc::focus::UseFocusWithin.materialize()><Code inline=true>"use_focus_within"</Code></Link>
                " for that). "
                "See the "<Link href=crate::routes::doc::Focus.materialize()>"Focus overview"</Link>" for domain guidance."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useFocus.html" target=LinkTarget::_Blank>
                    "useFocus"
                </LinkExt>
                "."
            </p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <DemoShell
                source=include_str!("demos/focus.rs")
                description="Focus event tracking"
            >
                <FocusDemo />
            </DemoShell>

            <h2 id="input" class="anchor">
                "Input"
                <AnchorLink href="#input" description="Direct link to input"/>
            </h2>

            <p><code>"UseFocusInput"</code> " fields:"</p>

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
                            <TableCell>"Disables all focus event handling when true."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"on_focus"</code></TableCell>
                            <TableCell><code>"Option<Callback<FocusEvent>>"</code></TableCell>
                            <TableCell><code>"None"</code></TableCell>
                            <TableCell>"Handler called when the element receives focus."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"on_blur"</code></TableCell>
                            <TableCell><code>"Option<Callback<FocusEvent>>"</code></TableCell>
                            <TableCell><code>"None"</code></TableCell>
                            <TableCell>"Handler called when the element loses focus."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"on_focus_change"</code></TableCell>
                            <TableCell><code>"Option<Callback<bool>>"</code></TableCell>
                            <TableCell><code>"None"</code></TableCell>
                            <TableCell>"Handler called on every focus state transition."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="return-value" class="anchor">
                "Return Value"
                <AnchorLink href="#return-value" description="Direct link to return value"/>
            </h2>

            <p><code>"UseFocusReturn"</code> " fields:"</p>

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
                            <TableCell><code>"props"</code></TableCell>
                            <TableCell><code>"UseFocusProps"</code></TableCell>
                            <TableCell>"Spread onto the target element via " <code>"props.into_attrs()"</code> " to wire up focus/blur listeners."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Fires only when the element itself is focused/blurred (not descendants)."</li>
                <li>"Verifies "<Code inline=true>"document.activeElement"</Code>" matches the target before firing."</li>
                <li>"Synthetic blur support for Firefox (form elements disabled while focused)."</li>
                <li>"Respects disabled state \u{2014} handlers are suppressed when disabled."</li>
            </ul>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Focus.materialize()>"Focus overview"</Link></li>
                <li><Link href=crate::routes::doc::focus::UseFocusWithin.materialize()>"use_focus_within"</Link></li>
                <li><Link href=crate::routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link></li>
                <li><Link href=crate::routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_focus", link: "#use-focus" },
                Toc::Leaf { title: "Interactive Demo", link: "#demo" },
                Toc::Leaf { title: "Input", link: "#input" },
                Toc::Leaf { title: "Return Value", link: "#return-value" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}

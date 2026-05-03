use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::focus_within::FocusWithinDemo;

#[component]
pub fn PageUseFocusWithin() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use_focus_within" class="anchor">
                "use_focus_within"
                <AnchorLink href="#use_focus_within" description="Direct link to article header"/>
            </h1>

            <p>
                "The "<code>"use_focus_within"</code>" hook tracks when focus is anywhere within an element tree. Unlike "
                <code>"use_focus"</code>" which only fires when the element itself receives focus, "
                <code>"use_focus_within"</code>" fires when focus enters or leaves the entire element tree. "
                "See the "<Link href=crate::routes::doc::Focus.materialize()>"Focus overview"</Link>" for domain guidance."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useFocusWithin.html" target=LinkTarget::_Blank>
                    "useFocusWithin"
                </LinkExt>
                "."
            </p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <p>"Click on any element inside the container below. Focus within is tracked even as you move between different focusable children:"</p>

            <DemoShell
                source=include_str!("demos/focus_within.rs")
                description="Focus within tracking"
            >
                <FocusWithinDemo />
            </DemoShell>

            <h2 id="is-focus-within" class="anchor">
                "is_focus_within Signal"
                <AnchorLink href="#is-focus-within" description="Direct link to is_focus_within signal"/>
            </h2>

            <p>"The hook returns a reactive " <code>"Signal<bool>"</code> " named " <code>"is_focus_within"</code>
                " that is true whenever any descendant is focused. Use it for conditional styling:"</p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    let UseFocusWithinReturn { props, is_focus_within } = use_focus_within(input);

                    view! {
                        <div
                            {..props.into_attrs()}
                            style=move || if is_focus_within.get() {
                                "border: 2px solid blue; background: lightblue;"
                            } else {
                                "border: 2px solid #ccc; background: transparent;"
                            }
                        >
                            <input type="text" />
                        </div>
                    }
                "#)}
            </Code>

            <h2 id="input" class="anchor">
                "Input"
                <AnchorLink href="#input" description="Direct link to input"/>
            </h2>

            <p><code>"UseFocusWithinInput"</code> " fields:"</p>

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
                            <TableCell>"Disables all focus-within event handling when true."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"on_focus_within"</code></TableCell>
                            <TableCell><code>"Option<Callback<FocusWithinEvent>>"</code></TableCell>
                            <TableCell><code>"None"</code></TableCell>
                            <TableCell>"Handler called when focus enters the element tree."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"on_blur_within"</code></TableCell>
                            <TableCell><code>"Option<Callback<FocusWithinEvent>>"</code></TableCell>
                            <TableCell><code>"None"</code></TableCell>
                            <TableCell>"Handler called when focus leaves the element tree."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"on_focus_within_change"</code></TableCell>
                            <TableCell><code>"Option<Callback<bool>>"</code></TableCell>
                            <TableCell><code>"None"</code></TableCell>
                            <TableCell>"Handler called on every focus-within state transition."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="return-value" class="anchor">
                "Return Value"
                <AnchorLink href="#return-value" description="Direct link to return value"/>
            </h2>

            <p><code>"UseFocusWithinReturn"</code> " fields:"</p>

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
                            <TableCell><code>"UseFocusWithinProps"</code></TableCell>
                            <TableCell>"Spread onto the container element via " <code>"props.into_attrs()"</code> " to wire up focusin/focusout listeners."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"is_focus_within"</code></TableCell>
                            <TableCell><code>"Signal<bool>"</code></TableCell>
                            <TableCell>"True whenever any descendant of the container is focused."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Tracks " <code>"focusin"</code> "/" <code>"focusout"</code> " events for the entire element tree."</li>
                <li>"Handles focus moving between children without triggering blur."</li>
                <li>"Ignores events bubbling through portals."</li>
                <li>"Global focus listener detects DOM-removal edge cases."</li>
                <li>"Respects disabled state \u{2014} handlers are suppressed when disabled."</li>
            </ul>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Focus.materialize()>"Focus overview"</Link></li>
                <li><Link href=crate::routes::doc::focus::UseFocus.materialize()>"use_focus"</Link>" \u{2014} tracks focus on the element itself (not descendants)"</li>
                <li><Link href=crate::routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link></li>
                <li><Link href=crate::routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_focus_within", link: "#use_focus_within" },
                Toc::Leaf { title: "Interactive Demo", link: "#demo" },
                Toc::Leaf { title: "is_focus_within Signal", link: "#is-focus-within" },
                Toc::Leaf { title: "Input", link: "#input" },
                Toc::Leaf { title: "Return Value", link: "#return-value" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}

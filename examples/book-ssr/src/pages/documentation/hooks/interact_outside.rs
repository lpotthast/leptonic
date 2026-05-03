use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::interact_outside::InteractOutsideDemo;

#[component]
pub fn PageUseInteractOutside() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use-interact-outside" class="anchor">
                "use_interact_outside"
                <AnchorLink href="#use-interact-outside" description="Direct link to section: use_interact_outside"/>
            </h1>

            <p>
                "The "<Code inline=true>"use_interact_outside"</Code>" hook detects interactions (clicks, touches) outside a specified element. "
                "See the "<Link href=crate::routes::doc::Interactions.materialize()>"Interactions overview"</Link>" for domain guidance."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useInteractOutside.html" target=LinkTarget::_Blank>
                    "useInteractOutside"
                </LinkExt>
                "."
            </p>

            <h2 id="input" class="anchor">
                "Input"
                <AnchorLink href="#input" description="Direct link to section: Input"/>
            </h2>

            <p><Code inline=true>"UseInteractOutsideInput"</Code>" fields:"</p>

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
                            <TableCell>"Disables outside interaction detection when true."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"on_interact_outside_start"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Callback<PointerEvent>>"</Code></TableCell>
                            <TableCell>"Called when an interaction starts outside the element (on pointer down)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"on_interact_outside"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Callback<MouseEvent>>"</Code></TableCell>
                            <TableCell>"Called when an interaction completes outside the element (on click)."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="return" class="anchor">
                "Return"
                <AnchorLink href="#return" description="Direct link to section: Return"/>
            </h2>

            <p><Code inline=true>"UseInteractOutsideReturn"</Code>" fields:"</p>

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
                            <TableCell><Code inline=true>"UseInteractOutsideProps"</Code></TableCell>
                            <TableCell>"Spread onto the target element via "<Code inline=true>"props.into_attrs()"</Code>" to define the \u{201c}inside\u{201d} boundary."</TableCell>
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
                    let (is_open, set_is_open) = signal(true);

                    let interact_outside = use_interact_outside(UseInteractOutsideInput {
                        disabled: Signal::derive(|| false),
                        on_interact_outside_start: None,
                        on_interact_outside: Some(Callback::new(move |_| {
                            set_is_open.set(false);
                        })),
                    });

                    view! {
                        <div {..interact_outside.props.into_attrs()}>
                            "Click outside to close"
                        </div>
                    }
                "#)}
            </Code>

            <DemoShell source=include_str!("demos/interact_outside.rs")>
                <InteractOutsideDemo />
            </DemoShell>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to section: Features"/>
            </h2>

            <ul>
                <li>"Handles pointer events (covers both mouse and touch)"</li>
                <li>"Distinguishes interaction start (pointer down) from completion (pointer up)"</li>
                <li>"Can be temporarily disabled"</li>
                <li>"Commonly used for closing dropdowns, modals, popovers, and context menus"</li>
            </ul>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Interactions.materialize()>"Interactions overview"</Link></li>
                <li><Link href=crate::routes::doc::Overlays.materialize()>"Overlays overview"</Link></li>
                <li><Link href=crate::routes::doc::popover::Hook.materialize()>"use_popover"</Link></li>
                <li><Link href=crate::routes::doc::modal::Hook.materialize()>"use_modal"</Link></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_interact_outside", link: "#use-interact-outside" },
                Toc::Leaf { title: "Input", link: "#input" },
                Toc::Leaf { title: "Return", link: "#return" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}

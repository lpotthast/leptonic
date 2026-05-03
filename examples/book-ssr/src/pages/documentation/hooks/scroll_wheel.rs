use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::scroll_wheel::ScrollWheelDemo;

#[component]
pub fn PageUseScrollWheel() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use-scroll-wheel" class="anchor">
                "use_scroll_wheel"
                <AnchorLink href="#use-scroll-wheel" description="Direct link to section: use_scroll_wheel"/>
            </h1>

            <p>
                "The "<Code inline=true>"use_scroll_wheel"</Code>" hook handles scroll wheel events on an element. "
                "See the "<Link href=crate::routes::doc::Interactions.materialize()>"Interactions overview"</Link>" for domain guidance."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useScrollWheel.html" target=LinkTarget::_Blank>
                    "useScrollWheel"
                </LinkExt>
                "."
            </p>

            <h2 id="input" class="anchor">
                "Input"
                <AnchorLink href="#input" description="Direct link to section: Input"/>
            </h2>

            <p><Code inline=true>"UseScrollWheelInput"</Code>" fields:"</p>

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
                            <TableCell>"Disables the scroll listener when true."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"on_scroll"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Callback<ScrollEvent>>"</Code></TableCell>
                            <TableCell>"Called when the user scrolls with the mouse wheel. Receives a ScrollEvent with delta_x and delta_y."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="return" class="anchor">
                "Return"
                <AnchorLink href="#return" description="Direct link to section: Return"/>
            </h2>

            <p><Code inline=true>"UseScrollWheelReturn"</Code>" fields:"</p>

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
                            <TableCell><Code inline=true>"UseScrollWheelProps"</Code></TableCell>
                            <TableCell>"Spread onto the target element via "<Code inline=true>"props.into_attrs()"</Code>"."</TableCell>
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
                    let (value, set_value) = signal(50.0f64);

                    let scroll_wheel = use_scroll_wheel(UseScrollWheelInput {
                        disabled: Signal::derive(|| false),
                        on_scroll: Some(Callback::new(move |e: ScrollEvent| {
                            if e.delta_y.abs() > e.delta_x.abs() {
                                set_value.update(|v| {
                                    *v = (*v - e.delta_y * 0.1).clamp(0.0, 100.0);
                                });
                            }
                        })),
                    });

                    view! {
                        <div {..scroll_wheel.props.into_attrs()} tabindex="0">
                            "Value: " { move || value.get().round() as i32 }
                        </div>
                    }
                "#)}
            </Code>

            <DemoShell source=include_str!("demos/scroll_wheel.rs") title="Scroll here to adjust value">
                <ScrollWheelDemo />
            </DemoShell>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to section: Features"/>
            </h2>

            <ul>
                <li>"Normalized scroll delta values"</li>
                <li>"Horizontal and vertical scroll support"</li>
                <li>"Can be temporarily disabled"</li>
                <li>"Useful for custom scroll behaviors, zooming, value adjustment controls, and timeline scrubbing"</li>
            </ul>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Interactions.materialize()>"Interactions overview"</Link></li>
                <li><Link href=crate::routes::doc::interactions::UsePreventScroll.materialize()>"use_prevent_scroll"</Link></li>
                <li><Link href=crate::routes::doc::interactions::UseMove.materialize()>"use_move"</Link></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_scroll_wheel", link: "#use-scroll-wheel" },
                Toc::Leaf { title: "Input", link: "#input" },
                Toc::Leaf { title: "Return", link: "#return" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}

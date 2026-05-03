use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::hover::HoverDemo;

#[component]
pub fn PageUseHover() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use-hover" class="anchor">
                "use_hover"
                <AnchorLink href="#use-hover" description="Direct link to section: use_hover"/>
            </h1>

            <p>
                "The "<Code inline=true>"use_hover"</Code>" hook tracks pointer hover state on an element. "
                "See the "<Link href=crate::routes::doc::Interactions.materialize()>"Interactions overview"</Link>" for domain guidance."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useHover.html" target=LinkTarget::_Blank>
                    "useHover"
                </LinkExt>
                "."
            </p>

            <h2 id="input" class="anchor">
                "Input"
                <AnchorLink href="#input" description="Direct link to section: Input"/>
            </h2>

            <p><Code inline=true>"UseHoverInput"</Code>" fields:"</p>

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
                            <TableCell>"Disables hover callbacks. If hovered when this becomes true, a programmatic HoverEnd fires and is_hovered resets."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"on_hover_start"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Callback<HoverStartEvent>>"</Code></TableCell>
                            <TableCell>"Called when a pointer starts hovering the element."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"on_hover_end"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Callback<HoverEndEvent>>"</Code></TableCell>
                            <TableCell>"Called when a pointer stops hovering, or when disabled transitions to true while hovered."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"on_hover_change"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Callback<bool>>"</Code></TableCell>
                            <TableCell>"Called on every hover state transition."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="return" class="anchor">
                "Return"
                <AnchorLink href="#return" description="Direct link to section: Return"/>
            </h2>

            <p><Code inline=true>"UseHoverReturn"</Code>" fields:"</p>

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
                            <TableCell><Code inline=true>"UseHoverProps"</Code></TableCell>
                            <TableCell>"Spread onto the target element via "<Code inline=true>"props.into_attrs()"</Code>"."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_hovered"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell>"Whether the element is currently hovered."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="demo" class="anchor">
                "Demo"
                <AnchorLink href="#demo" description="Direct link to section: Demo"/>
            </h2>

            <DemoShell source=include_str!("demos/hover.rs")>
                <HoverDemo />
            </DemoShell>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Interactions.materialize()>"Interactions overview"</Link></li>
                <li><Link href=crate::routes::doc::interactions::UsePress.materialize()>"use_press"</Link></li>
                <li><Link href=crate::routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link></li>
                <li><Link href=crate::routes::doc::button::Hook.materialize()>"use_button"</Link>" (composes use_hover)"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_hover", link: "#use-hover" },
                Toc::Leaf { title: "Input", link: "#input" },
                Toc::Leaf { title: "Return", link: "#return" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}

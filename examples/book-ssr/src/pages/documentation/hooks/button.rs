use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::button_basic::BasicButtonDemo;

#[component]
pub fn PageUseButton() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use-button" class="anchor">
                "use_button"
                <AnchorLink href="#use-button" description="Direct link to article header"/>
            </h1>

            <p>
                "The "<Code inline=true>"use_button"</Code>" hook creates standardized button behavior on arbitrary elements. "
                "See the "<Link href=crate::routes::doc::Button.materialize()>"Button overview"</Link>" for concept guidance."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useButton.html" target=LinkTarget::_Blank>
                    "useButton"
                </LinkExt>
                "."
            </p>

            <h2 id="input" class="anchor">
                "Input"
                <AnchorLink href="#input" description="Direct link to section: Input"/>
            </h2>

            <p><Code inline=true>"UseButtonInput"</Code>" fields:"</p>

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
                            <TableCell>"Whether the button is disabled"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"aria_haspopup"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<AriaHasPopup>"</Code></TableCell>
                            <TableCell>"Indicates the type of popup opened by the button"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"aria_expanded"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<AriaExpanded>"</Code></TableCell>
                            <TableCell>"Whether the controlled popup is expanded"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"use_press_input"</Code></TableCell>
                            <TableCell><Code inline=true>"UsePressInput"</Code></TableCell>
                            <TableCell>"Configuration for press interaction handling"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"use_hover_input"</Code></TableCell>
                            <TableCell><Code inline=true>"UseHoverInput"</Code></TableCell>
                            <TableCell>"Configuration for hover interaction handling"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"use_focus_ring_input"</Code></TableCell>
                            <TableCell><Code inline=true>"UseFocusRingInput"</Code></TableCell>
                            <TableCell>"Configuration for focus ring behavior"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="return" class="anchor">
                "Return"
                <AnchorLink href="#return" description="Direct link to section: Return"/>
            </h2>

            <p><Code inline=true>"UseButtonReturn"</Code>" fields:"</p>

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
                            <TableCell><Code inline=true>"UseButtonProps"</Code></TableCell>
                            <TableCell>"ARIA attributes and event handlers to spread onto the element"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_hovered"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell>"Whether the button is currently hovered"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_pressed"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell>"Whether the button is currently pressed"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_focus_visible"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell>"Whether the button has a visible focus ring"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="demo" class="anchor">
                "Demo"
                <AnchorLink href="#demo" description="Direct link to section: Demo"/>
            </h2>

            <DemoShell
                source=include_str!("demos/button_basic.rs")
                description="Press count tracking on a div element"
            >
                <BasicButtonDemo />
            </DemoShell>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Button.materialize()>"Button overview"</Link></li>
                <li><Link href=crate::routes::doc::button::Atom.materialize()>"Button atom"</Link></li>
                <li><Link href=crate::routes::doc::button::Component.materialize()>"Button component"</Link></li>
                <li><Link href=crate::routes::doc::interactions::UsePress.materialize()>"use_press"</Link></li>
                <li><Link href=crate::routes::doc::interactions::UseHover.materialize()>"use_hover"</Link></li>
                <li><Link href=crate::routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_button", link: "#use-button" },
                Toc::Leaf { title: "Input", link: "#input" },
                Toc::Leaf { title: "Return", link: "#return" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}

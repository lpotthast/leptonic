use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::button::ButtonDemo;
use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

#[component]
pub fn PageAtomButton() -> impl IntoView {
    view! {
        <Article>
            <h1 id="button-atom" class="anchor">
                "Button atom"
                <AnchorLink href="#button-atom" description="Direct link to article header"/>
            </h1>

            <p>
                "The Button atom is a headless, semantic "<Code inline=true>"<button>"</Code>" element "
                "with all hook behavior baked in. No visual styling is applied. "
                "See the "<Link href=crate::routes::doc::Button.materialize()>"Button overview"</Link>" for concept guidance."
            </p>

            <h2 id="hooks" class="anchor">
                "Hooks Used"
                <AnchorLink href="#hooks" description="Direct link to section: Hooks Used"/>
            </h2>

            <p>
                "The Button atom wraps "<Link href=crate::routes::doc::button::Hook.materialize()><Code inline=true>"use_button"</Code></Link>
                ", which composes "<Code inline=true>"use_press"</Code>", "<Code inline=true>"use_hover"</Code>", and "<Code inline=true>"use_focus_ring"</Code>"."
            </p>

            <h2 id="props" class="anchor">
                "Props"
                <AnchorLink href="#props" description="Direct link to section: Props"/>
            </h2>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Prop"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><Code inline=true>"on_press"</Code></TableCell>
                            <TableCell><Code inline=true>"impl Fn(PressEvent) + Send + Sync + 'static"</Code></TableCell>
                            <TableCell>"Called when the button is activated"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"disabled"</Code></TableCell>
                            <TableCell><Code inline=true>"MaybeSignal<bool>"</Code></TableCell>
                            <TableCell>"Whether the button is disabled (default: false)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"aria_haspopup"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<AriaHasPopup>"</Code></TableCell>
                            <TableCell>"Type of popup the button controls"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"aria_expanded"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<AriaExpanded>"</Code></TableCell>
                            <TableCell>"Whether the controlled popup is expanded"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"children"</Code></TableCell>
                            <TableCell><Code inline=true>"Children"</Code></TableCell>
                            <TableCell>"Button content"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="demo" class="anchor">
                "Demo"
                <AnchorLink href="#demo" description="Direct link to section: Demo"/>
            </h2>

            <DemoShell description="Unstyled button atom" source=include_str!("demos/button.rs")>
                <ButtonDemo />
            </DemoShell>

            <h2 id="link-button" class="anchor">
                "LinkButton"
                <AnchorLink href="#link-button" description="Direct link to section: LinkButton"/>
            </h2>

            <p>
                <Code inline=true>"LinkButton"</Code>" is a button-styled anchor element for navigation. "
                "Use it when an element looks like a button but navigates to another page."
            </p>

            <h2 id="button-wrapper" class="anchor">
                "ButtonWrapper"
                <AnchorLink href="#button-wrapper" description="Direct link to section: ButtonWrapper"/>
            </h2>

            <p>
                <Code inline=true>"ButtonWrapper"</Code>" is a layout container for grouping multiple buttons together."
            </p>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Button.materialize()>"Button overview"</Link></li>
                <li><Link href=crate::routes::doc::button::Hook.materialize()>"Hook deep-dive: use_button"</Link></li>
                <li><Link href=crate::routes::doc::button::Component.materialize()>"Component deep-dive: Button"</Link></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Button atom", link: "#button-atom" },
                Toc::Leaf { title: "Hooks Used", link: "#hooks" },
                Toc::Leaf { title: "Props", link: "#props" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "LinkButton", link: "#link-button" },
                Toc::Leaf { title: "ButtonWrapper", link: "#button-wrapper" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}

use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::modal::ModalConceptDemo;
use crate::{
    pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc},
    routes,
};

#[component]
pub fn PageModalOverview() -> impl IntoView {
    view! {
        <Article>
            <h1 id="modal" class="anchor">
                "Modal"
                <AnchorLink href="#modal" description="Direct link to article header"/>
            </h1>

            <p>
                "Modals are dialog overlays that demand the user's attention. "
                "They capture focus, block interaction with the rest of the page, "
                "and prevent scrolling until dismissed. Use them for confirmations, "
                "critical decisions, or multi-step flows that require acknowledgement."
            </p>

            <p>
                "Leptonic provides modals at two abstraction levels. "
                "See "<Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                " for a detailed explanation of each layer. "
                "Internally, the modal concept composes three hooks: "
                <Code inline=true>"use_dialog"</Code>" (ARIA semantics + focus), "
                <Code inline=true>"use_modal"</Code>" (marks as modal), and "
                <Code inline=true>"use_modal_backdrop"</Code>" (dismiss behavior + scroll prevention)."
            </p>

            <h2 id="when-to-use" class="anchor">
                "When to Use"
                <AnchorLink href="#when-to-use" description="Direct link to section: When to Use"/>
            </h2>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell>"If you want to\u{2026}"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Use"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell>"Block the page and require a decision"</TableCell>
                            <TableCell><b>"Modal"</b></TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Show contextual content anchored to an element"</TableCell>
                            <TableCell>"Popover"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Notify without blocking"</TableCell>
                            <TableCell>"Alert / Toast"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Show a sliding panel from the edge"</TableCell>
                            <TableCell>"Drawer"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <p>
                "Modals are disruptive by design. If the information is supplementary "
                "rather than essential, a popover or toast is less intrusive."
            </p>

            <h2 id="dive-deeper" class="anchor">
                "Dive Deeper"
                <AnchorLink href="#dive-deeper" description="Direct link to section: Dive Deeper"/>
            </h2>

            <p>
                "Not sure which layer to pick? Read the "
                <Link href=routes::doc::Architecture.materialize()>"architecture guide"</Link>
                ". Otherwise, pick a layer:"
            </p>

            <ul>
                <li><Link href=routes::doc::modal::Hook.materialize()>"Hook: use_modal"</Link></li>
                <li><Link href=routes::doc::modal::Component.materialize()>"Component: Modal"</Link></li>
            </ul>

            <h2 id="quick-start" class="anchor">
                "Quick Start"
                <AnchorLink href="#quick-start" description="Direct link to section: Quick Start"/>
            </h2>

            <p>"The simplest way to use a modal (component layer):"</p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    let (show_modal, set_show_modal) = signal(false);

                    <Button on_press=move |_| set_show_modal.set(true)>"Open Modal"</Button>

                    <Modal show_when=show_modal on_close=move |()| set_show_modal.set(false)>
                        <ModalHeader><ModalTitle>"Confirm"</ModalTitle></ModalHeader>
                        <ModalBody>"Are you sure?"</ModalBody>
                        <ModalFooter>
                            <ButtonWrapper>
                                <Button on_press=move |_| set_show_modal.set(false)
                                    color=ButtonColor::Secondary>
                                    "Close"
                                </Button>
                            </ButtonWrapper>
                        </ModalFooter>
                    </Modal>
                "#)}
            </Code>

            <DemoShell description="Modal dialog with trigger button" source=include_str!("demos/modal.rs")>
                <ModalConceptDemo />
            </DemoShell>

            <h2 id="accessibility" class="anchor">
                "Accessibility"
                <AnchorLink href="#accessibility" description="Direct link to section: Accessibility"/>
            </h2>

            <p>
                "Leptonic modals follow the WAI-ARIA Dialog (Modal) pattern. "
                "Focus is automatically trapped within the modal while it is open."
            </p>

            <h3>"ARIA attributes"</h3>

            <ul>
                <li><Code inline=true>"role=\"dialog\""</Code>" \u{2014} or "<Code inline=true>"\"alertdialog\""</Code>" for critical prompts"</li>
                <li><Code inline=true>"aria-modal=\"true\""</Code>" \u{2014} tells assistive technology to ignore content behind the modal"</li>
                <li><Code inline=true>"aria-labelledby"</Code>" \u{2014} points to the modal's title element"</li>
            </ul>

            <h3>"Keyboard interaction"</h3>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Key"</TableHeaderCell>
                            <TableHeaderCell>"Action"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><Code inline=true>"Escape"</Code></TableCell>
                            <TableCell>"Closes the modal"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Tab"</Code></TableCell>
                            <TableCell>"Cycles focus within the modal (focus is trapped)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Shift+Tab"</Code></TableCell>
                            <TableCell>"Cycles focus backward within the modal"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Modal", link: "#modal" },
                Toc::Leaf { title: "When to Use", link: "#when-to-use" },
                Toc::Leaf { title: "Dive Deeper", link: "#dive-deeper" },
                Toc::Leaf { title: "Quick Start", link: "#quick-start" },
                Toc::Leaf { title: "Accessibility", link: "#accessibility" },
            ]
        }/>
    }
}

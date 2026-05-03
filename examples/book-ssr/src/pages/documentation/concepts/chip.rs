use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::chip::ChipConceptDemo;
use crate::{
    pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc},
    routes,
};

#[component]
pub fn PageChipOverview() -> impl IntoView {
    view! {
        <Article>
            <h1 id="chip" class="anchor">
                "Chip"
                <AnchorLink href="#chip" description="Direct link to article header"/>
            </h1>

            <p>
                "Chips are compact, interactive elements that represent attributes, tags, or actions. "
                "They can be dismissible (removable by the user) and come in color variants "
                "to convey meaning \u{2014} success, warning, danger, etc."
            </p>

            <p>
                "Leptonic provides chips at two abstraction levels. "
                "See "<Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                " for a detailed explanation of each layer. "
                "The underlying hook is "<Code inline=true>"use_tag"</Code>
                " (following react-aria's TagGroup pattern)."
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
                            <TableCell>"Display a removable tag or attribute"</TableCell>
                            <TableCell><b>"Chip"</b>" (dismissible)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Display a status label"</TableCell>
                            <TableCell><b>"Chip"</b>" (with color)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Trigger an action"</TableCell>
                            <TableCell>"Button"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

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
                <li><Link href=routes::doc::chip::Hook.materialize()>"Hook: use_tag"</Link></li>
                <li><Link href=routes::doc::chip::Component.materialize()>"Component: Chip"</Link></li>
            </ul>

            <h2 id="quick-start" class="anchor">
                "Quick Start"
                <AnchorLink href="#quick-start" description="Direct link to section: Quick Start"/>
            </h2>

            <p>"The simplest way to use chips (component layer):"</p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    <Chip>"Default"</Chip>
                    <Chip color=Signal::from(ChipColor::Success)>"Success"</Chip>
                    <Chip color=Signal::from(ChipColor::Danger)>"Danger"</Chip>
                "#)}
            </Code>

            <DemoShell description="Dismissable chip" source=include_str!("demos/chip.rs")>
                <ChipConceptDemo />
            </DemoShell>

            <h2 id="accessibility" class="anchor">
                "Accessibility"
                <AnchorLink href="#accessibility" description="Direct link to section: Accessibility"/>
            </h2>

            <p>
                "Chip groups are based on the WAI-ARIA Grid pattern "
                "(tag groups use "<Code inline=true>"role=\"grid\""</Code>")."
            </p>

            <h3>"ARIA attributes"</h3>

            <ul>
                <li><Code inline=true>"role=\"row\""</Code>" on each chip, "<Code inline=true>"role=\"gridcell\""</Code>" on the content"</li>
                <li><Code inline=true>"aria-selected"</Code>" \u{2014} when chips are selectable"</li>
                <li>"Remove button has "<Code inline=true>"aria-label=\"Remove\""</Code></li>
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
                            <TableCell><Code inline=true>"Delete / Backspace"</Code></TableCell>
                            <TableCell>"Remove the chip (if dismissible)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Arrow keys"</Code></TableCell>
                            <TableCell>"Navigate between chips in a group"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Enter / Space"</Code></TableCell>
                            <TableCell>"Select the chip (if selectable)"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Chip", link: "#chip" },
                Toc::Leaf { title: "When to Use", link: "#when-to-use" },
                Toc::Leaf { title: "Dive Deeper", link: "#dive-deeper" },
                Toc::Leaf { title: "Quick Start", link: "#quick-start" },
                Toc::Leaf { title: "Accessibility", link: "#accessibility" },
            ]
        }/>
    }
}

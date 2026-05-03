use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{
    pages::documentation::{article::Article, toc::Toc},
    routes,
};

#[component]
pub fn PageListboxOverview() -> impl IntoView {
    view! {
        <Article>
            <h1 id="listbox" class="anchor">
                "Listbox"
                <AnchorLink href="#listbox" description="Direct link to article header"/>
            </h1>

            <p>
                "Listboxes present a scrollable list of selectable options. "
                "They support single and multi-selection, keyboard navigation, and type-ahead. "
                "Listboxes serve as the foundation for higher-level components like Select and Combobox."
            </p>

            <p>
                "Listboxes are currently available as hooks only. "
                "See "<Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                " for a detailed explanation of each layer."
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
                            <TableCell>"Present a visible list of selectable items"</TableCell>
                            <TableCell><b>"Listbox"</b></TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Choose from a dropdown"</TableCell>
                            <TableCell>"Select"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Toggle independent boolean options"</TableCell>
                            <TableCell>"Checkbox group"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Choose one from a visible set"</TableCell>
                            <TableCell>"Radio group"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="dive-deeper" class="anchor">
                "Dive Deeper"
                <AnchorLink href="#dive-deeper" description="Direct link to section: Dive Deeper"/>
            </h2>

            <ul>
                <li><Link href=routes::doc::listbox::Hook.materialize()>"Hook: use_listbox"</Link></li>
            </ul>

            <h2 id="quick-start" class="anchor">
                "Quick Start"
                <AnchorLink href="#quick-start" description="Direct link to section: Quick Start"/>
            </h2>

            <p>
                "Listboxes are composed from hooks. Here is a brief sketch of the API. "
                "See the "<Link href=routes::doc::listbox::Hook.materialize()>"hook deep-dive"</Link>
                " for a full interactive demo."
            </p>

            <Code language=Language::Rust>
                {indoc!(r"
                    let listbox = use_listbox(UseListboxInput {
                        // Provide items, selection mode, etc.
                        ..
                    });

                    <ul {..listbox.attrs}>
                        // Each option uses use_option
                    </ul>
                ")}
            </Code>

            <h2 id="accessibility" class="anchor">
                "Accessibility"
                <AnchorLink href="#accessibility" description="Direct link to section: Accessibility"/>
            </h2>

            <p>
                "Leptonic listboxes follow the WAI-ARIA Listbox pattern."
            </p>

            <h3>"ARIA attributes"</h3>

            <ul>
                <li><Code inline=true>"role=\"listbox\""</Code>" on the container, "<Code inline=true>"role=\"option\""</Code>" on each item"</li>
                <li><Code inline=true>"aria-selected"</Code>" \u{2014} reflects selection state"</li>
                <li><Code inline=true>"aria-multiselectable"</Code>" \u{2014} present when multi-selection is enabled"</li>
                <li>"Roving tabindex for focus management"</li>
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
                            <TableCell><Code inline=true>"Arrow Down / Up"</Code></TableCell>
                            <TableCell>"Navigate between options"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Home / End"</Code></TableCell>
                            <TableCell>"Jump to first / last option"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Space"</Code></TableCell>
                            <TableCell>"Toggle selection (multi-select)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Escape"</Code></TableCell>
                            <TableCell>"Clear selection"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Listbox", link: "#listbox" },
                Toc::Leaf { title: "When to Use", link: "#when-to-use" },
                Toc::Leaf { title: "Dive Deeper", link: "#dive-deeper" },
                Toc::Leaf { title: "Quick Start", link: "#quick-start" },
                Toc::Leaf { title: "Accessibility", link: "#accessibility" },
            ]
        }/>
    }
}

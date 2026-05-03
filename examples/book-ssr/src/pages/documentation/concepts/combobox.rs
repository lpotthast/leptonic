use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{
    pages::documentation::{article::Article, toc::Toc},
    routes,
};

#[component]
pub fn PageComboboxOverview() -> impl IntoView {
    view! {
        <Article>
            <h1 id="combobox" class="anchor">
                "Combobox"
                <AnchorLink href="#combobox" description="Direct link to article header"/>
            </h1>

            <p>
                "Comboboxes combine a text input with a dropdown listbox, "
                "letting users filter and select from options by typing. "
                "They bridge free-text entry with constrained selection \u{2014} "
                "useful for large option sets where users benefit from search."
            </p>

            <p>
                "Comboboxes are currently available as hooks only. "
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
                            <TableCell>"Search and select from a filterable list"</TableCell>
                            <TableCell><b>"Combobox"</b></TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Choose from a static dropdown"</TableCell>
                            <TableCell>"Select"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Enter free-form text"</TableCell>
                            <TableCell>"TextInput"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Choose from a visible list"</TableCell>
                            <TableCell>"Listbox"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="dive-deeper" class="anchor">
                "Dive Deeper"
                <AnchorLink href="#dive-deeper" description="Direct link to section: Dive Deeper"/>
            </h2>

            <ul>
                <li><Link href=routes::doc::combobox::Hook.materialize()>"Hook: use_combobox"</Link></li>
            </ul>

            <h2 id="quick-start" class="anchor">
                "Quick Start"
                <AnchorLink href="#quick-start" description="Direct link to section: Quick Start"/>
            </h2>

            <p>
                "Comboboxes are composed from hooks. Here is a brief sketch of the API. "
                "See the "<Link href=routes::doc::combobox::Hook.materialize()>"hook deep-dive"</Link>
                " for a full interactive demo."
            </p>

            <Code language=Language::Rust>
                {indoc!(r"
                    let combobox = use_combobox(UseComboboxInput {
                        // Provide items, selection mode, etc.
                        ..
                    });

                    // Spread input attrs on a text input
                    <input {..combobox.input_attrs} />

                    // Render the listbox overlay
                    <ul {..combobox.listbox_attrs}>
                        // Each option uses combobox.option_attrs
                    </ul>
                ")}
            </Code>

            <h2 id="accessibility" class="anchor">
                "Accessibility"
                <AnchorLink href="#accessibility" description="Direct link to section: Accessibility"/>
            </h2>

            <p>
                "Leptonic comboboxes follow the WAI-ARIA Combobox pattern."
            </p>

            <h3>"ARIA attributes"</h3>

            <ul>
                <li><Code inline=true>"role=\"combobox\""</Code>" on the input"</li>
                <li><Code inline=true>"aria-expanded"</Code>" \u{2014} reflects dropdown state"</li>
                <li><Code inline=true>"aria-autocomplete=\"list\""</Code>" \u{2014} indicates filtering behavior"</li>
                <li><Code inline=true>"aria-owns"</Code>" \u{2014} points to the listbox element"</li>
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
                            <TableCell><Code inline=true>"Arrow Down"</Code></TableCell>
                            <TableCell>"Open dropdown / move to next option"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Arrow Up"</Code></TableCell>
                            <TableCell>"Move to previous option"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Enter"</Code></TableCell>
                            <TableCell>"Select focused option"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Escape"</Code></TableCell>
                            <TableCell>"Close dropdown"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Combobox", link: "#combobox" },
                Toc::Leaf { title: "When to Use", link: "#when-to-use" },
                Toc::Leaf { title: "Dive Deeper", link: "#dive-deeper" },
                Toc::Leaf { title: "Quick Start", link: "#quick-start" },
                Toc::Leaf { title: "Accessibility", link: "#accessibility" },
            ]
        }/>
    }
}

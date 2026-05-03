use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::select::SelectConceptDemo;
use crate::{
    pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc},
    routes,
};

#[component]
pub fn PageSelectOverview() -> impl IntoView {
    view! {
        <Article>
            <h1 id="select" class="anchor">
                "Select"
                <AnchorLink href="#select" description="Direct link to article header"/>
            </h1>

            <p>
                "Select provides a dropdown for choosing from a predefined list of options. "
                "It's the go-to control when the option set is too large for radio buttons "
                "but the user still needs to pick from a constrained list. "
                "Leptonic offers three variants: "<Code inline=true>"Select"</Code>
                " (required single), "<Code inline=true>"OptionalSelect"</Code>
                " (nullable single), and "<Code inline=true>"Multiselect"</Code>"."
            </p>

            <p>
                "Leptonic provides selects at two abstraction levels. "
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
                            <TableCell>"Choose one from many predefined options"</TableCell>
                            <TableCell><b>"Select"</b></TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Choose one or none"</TableCell>
                            <TableCell>"OptionalSelect"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Choose multiple from a list"</TableCell>
                            <TableCell>"Multiselect"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Search-and-select with free text"</TableCell>
                            <TableCell>"Combobox"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Choose from 2\u{2013}5 visible options"</TableCell>
                            <TableCell>"Radio"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <p>
                "Option types must implement "<Code inline=true>"Debug + Clone + PartialEq + Send + Sync"</Code>
                ". Provide "<Code inline=true>"search_text_provider"</Code>
                " for filtering and "<Code inline=true>"render_option"</Code>
                " for display."
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
                <li><Link href=routes::doc::select::Hook.materialize()>"Hook: use_select"</Link></li>
                <li><Link href=routes::doc::select::Component.materialize()>"Component: Select"</Link></li>
            </ul>

            <h2 id="quick-start" class="anchor">
                "Quick Start"
                <AnchorLink href="#quick-start" description="Direct link to section: Quick Start"/>
            </h2>

            <p>"The simplest way to use a select (component layer):"</p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                    enum CoffeeSize { Small, Medium, Large }

                    let (selected, set_selected) = signal(CoffeeSize::Medium);

                    <Select
                        options=Signal::from(vec![CoffeeSize::Small, CoffeeSize::Medium, CoffeeSize::Large])
                        selected=selected
                        set_selected=set_selected
                        search_text_provider=move |o| format!("{o:?}")
                        render_option=move |o| format!("{o:?}")
                    />
                "#)}
            </Code>

            <DemoShell description="Select dropdown with options" source=include_str!("demos/select.rs")>
                <SelectConceptDemo />
            </DemoShell>

            <h2 id="accessibility" class="anchor">
                "Accessibility"
                <AnchorLink href="#accessibility" description="Direct link to section: Accessibility"/>
            </h2>

            <p>
                "When opened, leptonic selects use the WAI-ARIA Listbox pattern."
            </p>

            <h3>"ARIA attributes"</h3>

            <ul>
                <li><Code inline=true>"aria-expanded"</Code>" \u{2014} on the trigger, reflects dropdown state"</li>
                <li><Code inline=true>"role=\"listbox\""</Code>" \u{2014} on the options container"</li>
                <li><Code inline=true>"role=\"option\""</Code>" + "<Code inline=true>"aria-selected"</Code>" \u{2014} on each option"</li>
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
                            <TableCell><Code inline=true>"Enter / Arrow Down"</Code></TableCell>
                            <TableCell>"Opens the dropdown"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Arrow Down / Up"</Code></TableCell>
                            <TableCell>"Navigates between options"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Enter"</Code></TableCell>
                            <TableCell>"Selects the focused option"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Escape"</Code></TableCell>
                            <TableCell>"Closes the dropdown"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Select", link: "#select" },
                Toc::Leaf { title: "When to Use", link: "#when-to-use" },
                Toc::Leaf { title: "Dive Deeper", link: "#dive-deeper" },
                Toc::Leaf { title: "Quick Start", link: "#quick-start" },
                Toc::Leaf { title: "Accessibility", link: "#accessibility" },
            ]
        }/>
    }
}

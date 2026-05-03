use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::checkbox::CheckboxConceptDemo;
use crate::{
    pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc},
    routes,
};

#[component]
pub fn PageCheckboxOverview() -> impl IntoView {
    view! {
        <Article>
            <h1 id="checkbox" class="anchor">
                "Checkbox"
                <AnchorLink href="#checkbox" description="Direct link to article header"/>
            </h1>

            <p>
                "Checkboxes let users toggle individual options on or off. "
                "They're the standard control for binary choices within forms \u{2014} "
                "agreeing to terms, enabling features, or selecting items from a list. "
                "Checkboxes can also represent an indeterminate state when a parent checkbox "
                "partially represents its children."
            </p>

            <p>
                "Leptonic provides checkboxes at two abstraction levels. "
                "See "<Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                " for a detailed explanation of each layer. "
                "Leptonic also provides "<Code inline=true>"use_checkbox_group"</Code>
                " for grouping related checkboxes with shared validation."
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
                            <TableCell>"Toggle one or more independent options"</TableCell>
                            <TableCell><b>"Checkbox"</b></TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Switch an immediate on/off setting"</TableCell>
                            <TableCell>"Toggle / Switch"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Choose exactly one from a set"</TableCell>
                            <TableCell>"Radio"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Choose from a long list via dropdown"</TableCell>
                            <TableCell>"Select"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <p>
                "Checkboxes are best for form settings where changes are submitted later. "
                "If toggling should take effect immediately (like a 'Dark mode' switch), "
                "consider a Toggle instead."
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
                <li><Link href=routes::doc::checkbox::Hook.materialize()>"Hook: use_checkbox"</Link></li>
                <li><Link href=routes::doc::checkbox::Component.materialize()>"Component: Checkbox"</Link></li>
            </ul>

            <h2 id="quick-start" class="anchor">
                "Quick Start"
                <AnchorLink href="#quick-start" description="Direct link to section: Quick Start"/>
            </h2>

            <p>"The simplest way to use a checkbox (component layer):"</p>

            <Code language=Language::Rust>
                {indoc!(r"
                    let (checked, set_checked) = signal(false);

                    <Checkbox checked=checked set_checked=set_checked />
                ")}
            </Code>

            <DemoShell description="Checkbox with label and controlled state" source=include_str!("demos/checkbox.rs")>
                <CheckboxConceptDemo />
            </DemoShell>

            <h2 id="accessibility" class="anchor">
                "Accessibility"
                <AnchorLink href="#accessibility" description="Direct link to section: Accessibility"/>
            </h2>

            <p>
                "Leptonic checkboxes follow the WAI-ARIA Checkbox pattern. "
                "The component uses a native "<Code inline=true>"<input type=\"checkbox\">"</Code>
                ", which provides built-in accessibility."
            </p>

            <h3>"ARIA attributes"</h3>

            <ul>
                <li><Code inline=true>"aria-checked"</Code>" \u{2014} \"true\", \"false\", or \"mixed\" (indeterminate)"</li>
                <li><Code inline=true>"aria-invalid"</Code>" \u{2014} \"true\" when validation fails"</li>
                <li><Code inline=true>"aria-required"</Code>" \u{2014} \"true\" when the field is required"</li>
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
                            <TableCell><Code inline=true>"Space"</Code></TableCell>
                            <TableCell>"Toggles the checkbox"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Checkbox", link: "#checkbox" },
                Toc::Leaf { title: "When to Use", link: "#when-to-use" },
                Toc::Leaf { title: "Dive Deeper", link: "#dive-deeper" },
                Toc::Leaf { title: "Quick Start", link: "#quick-start" },
                Toc::Leaf { title: "Accessibility", link: "#accessibility" },
            ]
        }/>
    }
}

use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::text_field::TextFieldConceptDemo;
use crate::{
    pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc},
    routes,
};

#[component]
pub fn PageTextFieldOverview() -> impl IntoView {
    view! {
        <Article>
            <h1 id="text-field" class="anchor">
                "Text Field"
                <AnchorLink href="#text-field" description="Direct link to article header"/>
            </h1>

            <p>
                "Text fields capture free-form text input. Leptonic provides three specialized variants: "
                <Code inline=true>"TextInput"</Code>" for general text, "
                <Code inline=true>"NumberInput"</Code>" for numeric values with increment/decrement, and "
                <Code inline=true>"PasswordInput"</Code>" for masked entry."
            </p>

            <p>
                "Leptonic provides text fields at two abstraction levels. "
                "See "<Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                " for a detailed explanation of each layer. "
                <Code inline=true>"NumberInput"</Code>" uses a different hook ("
                <Code inline=true>"use_number_field"</Code>") with "
                <Code inline=true>"role=\"spinbutton\""</Code>
                ", arrow key increment/decrement, and min/max constraints."
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
                            <TableCell>"Capture free-form text"</TableCell>
                            <TableCell><b>"TextInput"</b></TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Enter a precise number with increment/decrement"</TableCell>
                            <TableCell>"NumberInput"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Enter a password (masked)"</TableCell>
                            <TableCell>"PasswordInput"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Choose from predefined options"</TableCell>
                            <TableCell>"Select"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Search and filter a list"</TableCell>
                            <TableCell>"Combobox"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <p>
                "All three variants share the same visual styling and form integration. "
                "Choose based on the input type, not the appearance."
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
                <li><Link href=routes::doc::text_field::Hook.materialize()>"Hook: use_text_field"</Link></li>
                <li><Link href=routes::doc::text_field::Component.materialize()>"Component: TextInput"</Link></li>
            </ul>

            <h2 id="quick-start" class="anchor">
                "Quick Start"
                <AnchorLink href="#quick-start" description="Direct link to section: Quick Start"/>
            </h2>

            <p>"The simplest way to use a text input (component layer):"</p>

            <Code language=Language::Rust>
                {indoc!(r"
                    let (value, set_value) = signal(String::new());

                    <TextInput get=value set=set_value />
                ")}
            </Code>

            <DemoShell description="Text input with label" source=include_str!("demos/text_field.rs")>
                <TextFieldConceptDemo />
            </DemoShell>

            <h2 id="accessibility" class="anchor">
                "Accessibility"
                <AnchorLink href="#accessibility" description="Direct link to section: Accessibility"/>
            </h2>

            <p>
                "TextInput uses native "<Code inline=true>"<input>"</Code>
                " semantics. NumberInput uses "<Code inline=true>"role=\"spinbutton\""</Code>"."
            </p>

            <h3>"ARIA attributes"</h3>

            <ul>
                <li><Code inline=true>"aria-invalid"</Code>" \u{2014} \"true\" when validation fails"</li>
                <li><Code inline=true>"aria-required"</Code>" \u{2014} \"true\" when the field is required"</li>
                <li><Code inline=true>"aria-disabled"</Code>" \u{2014} \"true\" when disabled"</li>
                <li><Code inline=true>"aria-valuenow/min/max"</Code>" \u{2014} on NumberInput only"</li>
            </ul>

            <h3>"Keyboard interaction (NumberInput)"</h3>

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
                            <TableCell><Code inline=true>"Arrow Up"</Code></TableCell>
                            <TableCell>"Increment by step"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Arrow Down"</Code></TableCell>
                            <TableCell>"Decrement by step"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Home"</Code></TableCell>
                            <TableCell>"Set to minimum (if defined)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"End"</Code></TableCell>
                            <TableCell>"Set to maximum (if defined)"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Text Field", link: "#text-field" },
                Toc::Leaf { title: "When to Use", link: "#when-to-use" },
                Toc::Leaf { title: "Dive Deeper", link: "#dive-deeper" },
                Toc::Leaf { title: "Quick Start", link: "#quick-start" },
                Toc::Leaf { title: "Accessibility", link: "#accessibility" },
            ]
        }/>
    }
}

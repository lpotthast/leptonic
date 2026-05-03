use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::toggle::ToggleConceptDemo;
use crate::{
    pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc},
    routes,
};

#[component]
pub fn PageToggleOverview() -> impl IntoView {
    view! {
        <Article>
            <h1 id="toggle" class="anchor">
                "Toggle"
                <AnchorLink href="#toggle" description="Direct link to article header"/>
            </h1>

            <p>
                "Toggles (also called switches) represent an immediate on/off choice. "
                "Unlike checkboxes, which are typically submitted with a form, "
                "toggles take effect as soon as the user flips them \u{2014} like a light switch."
            </p>

            <p>
                "Leptonic provides toggles at two abstraction levels. "
                "See "<Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                " for a detailed explanation of each layer. "
                "The underlying hook is "<Code inline=true>"use_switch"</Code>
                " (following the WAI-ARIA Switch pattern name), while the component is called Toggle."
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
                            <TableCell>"Switch an immediate on/off setting"</TableCell>
                            <TableCell><b>"Toggle"</b></TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Toggle a form option submitted later"</TableCell>
                            <TableCell>"Checkbox"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Trigger a one-time action"</TableCell>
                            <TableCell>"Button"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <p>
                "Use the visual metaphor to guide your choice: a toggle looks like a physical switch "
                "and implies instant effect. A checkbox looks like a form field and implies "
                "'save' or 'submit' later."
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
                <li><Link href=routes::doc::toggle::Hook.materialize()>"Hook: use_switch"</Link></li>
                <li><Link href=routes::doc::toggle::Component.materialize()>"Component: Toggle"</Link></li>
            </ul>

            <h2 id="quick-start" class="anchor">
                "Quick Start"
                <AnchorLink href="#quick-start" description="Direct link to section: Quick Start"/>
            </h2>

            <p>"The simplest way to use a toggle (component layer):"</p>

            <Code language=Language::Rust>
                {indoc!(r"
                    let (state, set_state) = signal(false);

                    <Toggle state=state set_state=set_state />
                ")}
            </Code>

            <DemoShell description="Toggle switch with label" source=include_str!("demos/toggle.rs")>
                <ToggleConceptDemo />
            </DemoShell>

            <h2 id="accessibility" class="anchor">
                "Accessibility"
                <AnchorLink href="#accessibility" description="Direct link to section: Accessibility"/>
            </h2>

            <p>
                "Leptonic toggles follow the WAI-ARIA Switch pattern. "
                "A hidden "<Code inline=true>"<input type=\"checkbox\">"</Code>
                " provides form submission support."
            </p>

            <h3>"ARIA attributes"</h3>

            <ul>
                <li><Code inline=true>"role=\"switch\""</Code>" \u{2014} identifies the element as a toggle switch"</li>
                <li><Code inline=true>"aria-checked"</Code>" \u{2014} \"true\" or \"false\""</li>
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
                            <TableCell>"Toggles the switch"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Enter"</Code></TableCell>
                            <TableCell>"Toggles the switch"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Toggle", link: "#toggle" },
                Toc::Leaf { title: "When to Use", link: "#when-to-use" },
                Toc::Leaf { title: "Dive Deeper", link: "#dive-deeper" },
                Toc::Leaf { title: "Quick Start", link: "#quick-start" },
                Toc::Leaf { title: "Accessibility", link: "#accessibility" },
            ]
        }/>
    }
}

use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::radio::RadioConceptDemo;
use crate::{
    pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc},
    routes,
};

#[component]
pub fn PageRadioOverview() -> impl IntoView {
    view! {
        <Article>
            <h1 id="radio" class="anchor">
                "Radio"
                <AnchorLink href="#radio" description="Direct link to article header"/>
            </h1>

            <p>
                "Radio buttons present a set of mutually exclusive options \u{2014} "
                "selecting one deselects all others. They make all choices visible at once, "
                "which helps users compare options before committing."
            </p>

            <p>
                "Leptonic provides radio buttons at two abstraction levels. "
                "See "<Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                " for a detailed explanation of each layer. "
                "The hook layer provides "<Code inline=true>"use_radio_group"</Code>
                " (group container with ARIA) and "<Code inline=true>"use_radio"</Code>
                " (individual radio within a group)."
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
                            <TableCell>"Choose exactly one from 2\u{2013}5 visible options"</TableCell>
                            <TableCell><b>"Radio"</b></TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Choose from a longer list via dropdown"</TableCell>
                            <TableCell>"Select"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Toggle multiple independent options"</TableCell>
                            <TableCell>"Checkbox"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Switch between content panels"</TableCell>
                            <TableCell>"Tabs"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <p>
                "Radio buttons work best when the options are few enough to display inline "
                "(typically 2\u{2013}5). For longer lists, a Select dropdown saves space."
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
                <li><Link href=routes::doc::radio::Hook.materialize()>"Hook: use_radio"</Link></li>
                <li><Link href=routes::doc::radio::Component.materialize()>"Component: Radio"</Link></li>
            </ul>

            <h2 id="quick-start" class="anchor">
                "Quick Start"
                <AnchorLink href="#quick-start" description="Direct link to section: Quick Start"/>
            </h2>

            <p>"The simplest way to use radio buttons (component layer):"</p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    let (checked_a, set_checked_a) = signal(true);
                    let (checked_b, set_checked_b) = signal(false);

                    <RadioGroup>
                        <FormControl>
                            <Radio checked=checked_a set_checked=set_checked_a />
                            <Label>"Option A"</Label>
                        </FormControl>
                        <FormControl>
                            <Radio checked=checked_b set_checked=set_checked_b />
                            <Label>"Option B"</Label>
                        </FormControl>
                    </RadioGroup>
                "#)}
            </Code>

            <DemoShell description="Radio group with labeled options" source=include_str!("demos/radio.rs")>
                <RadioConceptDemo />
            </DemoShell>

            <h2 id="accessibility" class="anchor">
                "Accessibility"
                <AnchorLink href="#accessibility" description="Direct link to section: Accessibility"/>
            </h2>

            <p>
                "Leptonic radio buttons follow the WAI-ARIA Radio Group pattern."
            </p>

            <h3>"ARIA attributes"</h3>

            <ul>
                <li><Code inline=true>"role=\"radiogroup\""</Code>" \u{2014} on the group container"</li>
                <li><Code inline=true>"role=\"radio\""</Code>" \u{2014} on each option (via native "<Code inline=true>"<input type=\"radio\">"</Code>")"</li>
                <li><Code inline=true>"aria-checked"</Code>" \u{2014} reflects selection state"</li>
                <li><Code inline=true>"aria-orientation"</Code>" \u{2014} \"horizontal\" or \"vertical\""</li>
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
                            <TableCell><Code inline=true>"Arrow Down / Right"</Code></TableCell>
                            <TableCell>"Select next radio"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Arrow Up / Left"</Code></TableCell>
                            <TableCell>"Select previous radio"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Tab"</Code></TableCell>
                            <TableCell>"Enter / exit the radio group"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Radio", link: "#radio" },
                Toc::Leaf { title: "When to Use", link: "#when-to-use" },
                Toc::Leaf { title: "Dive Deeper", link: "#dive-deeper" },
                Toc::Leaf { title: "Quick Start", link: "#quick-start" },
                Toc::Leaf { title: "Accessibility", link: "#accessibility" },
            ]
        }/>
    }
}

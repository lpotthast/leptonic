use leptos::prelude::*;

use super::demos::progress_bar::ProgressBarConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageProgressBarOverview() -> impl IntoView {
    view! {
        <DocPage title="Progress Bar">
            <p>
                "A progress bar shows how far an operation has advanced, such as a file upload or a form submission. It is "
                "determinate when the total is known and shows the progress as a percentage, or indeterminate when "
                "it only shows that something is going on."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Show progress toward a known total"</TableCell><TableCell><b>"Progress Bar"</b>" (determinate)"</TableCell></TableRow>
                    <TableRow>
                        <TableCell>"Indicate activity without a known endpoint"</TableCell>
                        <TableCell><b>"Progress Bar"</b>" (indeterminate: a value of "<Code inline=true>"None"</Code>")"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show a value within a known range that is not a task, e.g. disk usage"</TableCell>
                        <TableCell><Link href=routes::doc::Meter.materialize()>"Meter"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show a placeholder while content loads"</TableCell>
                        <TableCell>"A "<Link href=format!("{}#skeleton", routes::doc::Layout.materialize())>"skeleton"</Link>" drawn with CSS"</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "Progress bars exist as a hook and as atoms. See "
                    <Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::progress_bar::Hook.materialize()>"use_progress_bar"</Link></TableCell>
                        <TableCell>"Progress bar semantics and a computed percentage for an element you style yourself."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::progress_bar::Atom.materialize()>"Progress Bar Atoms"</Link></TableCell>
                        <TableCell>
                            "An unstyled "<Code inline=true>"ProgressBar"</Code>" with the parts "
                            <Code inline=true>"ProgressBarFill"</Code>" and "<Code inline=true>"ProgressBarValueText"</Code>
                            ", labelled by a "<Code inline=true>"Label"</Code>" and styled through data attributes."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "The atoms with the progress and a label; the track around the fill is your own markup. The classes "
                    "are the book\u{2019}s own; the "
                    <Link href=format!("{}#styling", routes::doc::progress_bar::Atom.materialize())>"styling section"</Link>
                    " of the atoms shows their CSS."
                </p>

                <Demo description="Progress bar at 75 percent with a visible label" source=include_str!("demos/progress_bar.rs") source_open=true>
                    <ProgressBarConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "All layers follow the WAI-ARIA "
                    <Link href="https://www.w3.org/TR/wai-aria-1.2/#progressbar" target=LinkTarget::Blank>"progressbar role"</Link>
                    ". Progress bars are informational and have no keyboard interaction."
                </p>

                <ul>
                    <li><Code inline=true>"role=\"progressbar\""</Code>", named by its label or "<Code inline=true>"aria_label"</Code>"."</li>
                    <li><Code inline=true>"aria-valuenow"</Code>" \u{2014} the current value (absent while indeterminate)."</li>
                    <li><Code inline=true>"aria-valuemin"</Code>" / "<Code inline=true>"aria-valuemax"</Code>" \u{2014} the bounds of the range."</li>
                    <li><Code inline=true>"aria-valuetext"</Code>" \u{2014} the formatted value, by default a percentage such as \u{201c}75%\u{201d}."</li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::progress_bar::Hook.materialize()>"use_progress_bar"</Link></li>
                <li><Link href=routes::doc::progress_bar::Atom.materialize()>"Progress Bar Atoms"</Link></li>
                <li><Link href=routes::doc::Meter.materialize()>"Meter"</Link></li>
                <li><Link href=routes::doc::Status.materialize()>"Status"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

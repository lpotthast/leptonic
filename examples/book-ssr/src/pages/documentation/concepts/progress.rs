use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::progress::ProgressConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageProgressOverview() -> impl IntoView {
    view! {
        <DocPage title="Progress">
            <p>
                "Progress bars communicate how far along a process has advanced. "
                "They support both determinate mode (specific percentage) and "
                "indeterminate mode (activity without a known endpoint). "
                "Use them for file uploads, form submissions, or any operation with observable progress."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Show progress toward a known total"</TableCell><TableCell><b>"ProgressBar"</b>" (determinate)"</TableCell></TableRow>
                    <TableRow>
                        <TableCell>"Indicate activity without a known endpoint"</TableCell>
                        <TableCell><b>"ProgressBar"</b>" (indeterminate, "<Code inline=true>"progress=None"</Code>")"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show a value within a known range that is not a task, e.g. disk usage"</TableCell>
                        <TableCell><Link href=routes::doc::hooks::UseMeter.materialize()>"use_meter"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Placeholder while content loads"</TableCell>
                        <TableCell><Link href=routes::doc::components::Skeleton.materialize()>"Skeleton"</Link></TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "Progress bars exist as a hook and as a component. See "
                    <Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::progress::Hook.materialize()>"use_progress_bar"</Link></TableCell>
                        <TableCell>"Progress bar semantics and a computed percentage for an element you style yourself."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::progress::Component.materialize()>"ProgressBar component"</Link></TableCell>
                        <TableCell>"A themed progress bar showing the percentage, with an animated indeterminate state."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>"The component is the quickest way to a progress bar:"</p>

                <Demo description="Progress bar at 75 percent" source=include_str!("demos/progress.rs") source_open=true>
                    <ProgressConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    <Link href=routes::doc::progress::Hook.materialize()>"use_progress_bar"</Link>" follows the WAI-ARIA "
                    <LinkExt href="https://www.w3.org/TR/wai-aria-1.2/#progressbar" target=LinkTarget::_Blank>"progressbar role"</LinkExt>
                    ". Progress bars are informational and have no keyboard interaction."
                </p>

                <ul>
                    <li><Code inline=true>"role=\"progressbar\""</Code></li>
                    <li><Code inline=true>"aria-valuenow"</Code>" \u{2014} current value (absent in indeterminate mode)"</li>
                    <li><Code inline=true>"aria-valuemin"</Code>" / "<Code inline=true>"aria-valuemax"</Code>" \u{2014} range bounds"</li>
                    <li><Code inline=true>"aria-valuetext"</Code>" \u{2014} the value as a percentage, e.g. \u{201c}75%\u{201d}"</li>
                </ul>

                <p>
                    "The "<Code inline=true>"ProgressBar"</Code>" component is not built on the hook yet and renders none of these "
                    "attributes. Use the hook when screen readers need to announce the progress."
                </p>
            </Section>
        </DocPage>
    }
}

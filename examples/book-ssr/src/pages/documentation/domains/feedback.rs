use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{
    pages::documentation::{article::Article, toc::Toc},
    routes,
};

#[component]
pub fn PageFeedback() -> impl IntoView {
    view! {
        <Article>
            <h1 id="feedback" class="anchor">
                "Feedback"
                <AnchorLink href="#feedback" description="Direct link to article header"/>
            </h1>

            <p>
                "Components that communicate status, results, or contextual information to the user \u{2014} from inline chips and tooltips to modal dialogs and toast notifications."
            </p>

            <h2 id="overview" class="anchor">
                "Overview"
                <AnchorLink href="#overview" description="Direct link to section: Overview"/>
            </h2>

            <h3 id="concepts" class="anchor">
                "Concepts"
                <AnchorLink href="#concepts" description="Direct link to section: Concepts"/>
            </h3>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Name"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                            <TableHeaderCell>"When to Use"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><Link href=routes::doc::Chip.materialize()>"Chip"</Link></TableCell>
                            <TableCell>"Compact element for tags, filters, or status"</TableCell>
                            <TableCell>"Displaying categorization, filters, or small status indicators"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::Modal.materialize()>"Modal"</Link></TableCell>
                            <TableCell>"Focused dialog requiring user action"</TableCell>
                            <TableCell>"Critical actions that need confirmation or focused input"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::Popover.materialize()>"Popover"</Link></TableCell>
                            <TableCell>"Contextual content anchored to a trigger"</TableCell>
                            <TableCell>"Rich contextual information tied to a specific element"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::Progress.materialize()>"Progress"</Link></TableCell>
                            <TableCell>"Indicate ongoing operations"</TableCell>
                            <TableCell>"Showing determinate or indeterminate progress of a task"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::Tooltip.materialize()>"Tooltip"</Link></TableCell>
                            <TableCell>"Brief contextual help on hover or focus"</TableCell>
                            <TableCell>"Short textual hints for buttons, icons, or labels"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h3 id="standalone" class="anchor">
                "Standalone"
                <AnchorLink href="#standalone" description="Direct link to section: Standalone"/>
            </h3>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Name"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><Link href=routes::doc::components::Alert.materialize()>"Alert"</Link></TableCell>
                            <TableCell>"Prominent status messages"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::components::Kbd.materialize()>"Kbd"</Link></TableCell>
                            <TableCell>"Keyboard shortcut display"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::components::Toast.materialize()>"Toast"</Link></TableCell>
                            <TableCell>"Temporary notification messages"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::hooks::UseMeter.materialize()>"use_meter"</Link></TableCell>
                            <TableCell>"Display a scalar value within a known range"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            // ── Decision Guide ──────────────────────────────────────

            <h2 id="decision-guide" class="anchor">
                "Decision Guide"
                <AnchorLink href="#decision-guide" description="Direct link to section: Decision Guide"/>
            </h2>

            <p>"Choosing between feedback mechanisms:"</p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell>"Choice"</TableHeaderCell>
                            <TableHeaderCell>"When to pick the first"</TableHeaderCell>
                            <TableHeaderCell>"When to pick the second"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><strong>"Tooltip vs Popover"</strong></TableCell>
                            <TableCell>"Brief, text-only hint (no interaction needed)"</TableCell>
                            <TableCell>"Rich content with links, buttons, or forms"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><strong>"Modal vs Toast"</strong></TableCell>
                            <TableCell>"Blocking action that requires user decision"</TableCell>
                            <TableCell>"Transient notification that auto-dismisses"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><strong>"Modal vs Alert"</strong></TableCell>
                            <TableCell>"Interactive dialog with buttons/forms"</TableCell>
                            <TableCell>"Static, persistent status message in the page flow"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><strong>"Chip vs Progress"</strong></TableCell>
                            <TableCell>"Static status label or tag"</TableCell>
                            <TableCell>"Dynamic progress of an ongoing operation"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Feedback", link: "#feedback" },
                Toc::Leaf { title: "Overview", link: "#overview" },
                Toc::Leaf { title: "Concepts", link: "#concepts" },
                Toc::Leaf { title: "Standalone", link: "#standalone" },
                Toc::Leaf { title: "Decision Guide", link: "#decision-guide" },
            ]
        }/>
    }
}

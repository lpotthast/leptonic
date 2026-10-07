use leptos::prelude::*;

use super::demos::grid::GridDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageGridLayout() -> impl IntoView {
    view! {
        <DocPage title="Grid Layout Components">
            <p>
                "A grid layout arranges content in rows and columns that adapt to the screen width: side by side on wide "
                "screens, stacked on narrow ones. The "<Code inline=true>"Grid"</Code>" component is a responsive "
                "12-column layout built from rows ("<Code inline=true>"Row"</Code>") and columns ("<Code inline=true>"Col"</Code>"). It only "
                "arranges content: it has no keyboard navigation, selection or ARIA role. For rows and cells that users "
                "navigate with the arrow keys, see the interactive "<Link href=routes::doc::Grid.materialize()>"Grid"</Link>"."
            </p>

            <Demo
                description="Responsive grid with six items; resize the window to see the columns change"
                source=include_str!("demos/grid.rs")
            >
                <GridDemo/>
            </Demo>

            <Section title="Grid">
                <p>"The container of the rows. Its "<Code inline=true>"gap"</Code>" spaces the rows and the columns in them."</p>
                <Section title="Props" id="grid-props">
                    <ApiTable kind=ApiKind::Props of="components::grid::Grid">
                        <ApiRow name="gap" ty="CssDimension">
                            "Space between rows and between the columns of a row. Required."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">"The rows."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Row">
                <p>"A row of columns, wrapping the columns that don\u{2019}t fit next to each other."</p>
                <Section title="Props" id="row-props">
                    <ApiTable kind=ApiKind::Props of="Row">
                        <ApiRow name="gap" ty="Option<CssDimension>" default="None">
                            "Overrides the grid\u{2019}s gap for this row and everything inside it."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">"The columns."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Col">
                <p>"A column spanning some of the 12 columns of its row, per "<AnchorLink href="#breakpoints">"breakpoint"</AnchorLink>"."</p>
                <Section title="Props" id="col-props">
                    <ApiTable kind=ApiKind::Props of="Col">
                        <ApiRow name="xs" ty="Option<u32>" default="None">
                            "Number of the 12 columns to span from the smallest screens on. "
                            "Without it, the column spans all 12."
                        </ApiRow>
                        <ApiRow name="sm, md, lg, xl" ty="Option<u32>" default="None">
                            "Number of columns to span from the given breakpoint on. "
                            "Without it, the next smaller breakpoint applies."
                        </ApiRow>
                        <ApiRow name="h_align" ty="ColAlign" default="Start">
                            "Horizontal alignment of the column content: "<Code inline=true>"Start"</Code>", "
                            <Code inline=true>"Center"</Code>" or "<Code inline=true>"End"</Code>"."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">"The column content."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Breakpoints">
                <p>
                    "Breakpoints are mobile first: a column size applies from its breakpoint\u{2019}s minimum width upwards, "
                    "until a larger breakpoint overrides it. A row wraps columns that don\u{2019}t fit."
                </p>
                <DocTable headers=&["Breakpoint", "Minimum viewport width"]>
                    <TableRow><TableCell><Code inline=true>"xs"</Code></TableCell><TableCell>"0"</TableCell></TableRow>
                    <TableRow><TableCell><Code inline=true>"sm"</Code></TableCell><TableCell>"48em"</TableCell></TableRow>
                    <TableRow><TableCell><Code inline=true>"md"</Code></TableCell><TableCell>"64em"</TableCell></TableRow>
                    <TableRow><TableCell><Code inline=true>"lg"</Code></TableCell><TableCell>"75em"</TableCell></TableRow>
                    <TableRow><TableCell><Code inline=true>"xl"</Code></TableCell><TableCell>"83em"</TableCell></TableRow>
                </DocTable>
            </Section>

            <Section title="Styling">
                <p>
                    "The grid and its rows set this variable from their "<Code inline=true>"gap"</Code>" props:"
                </p>
                <CssVariables prefix="--leptonic-grid-" scss=theme_scss!("grid")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Layout.materialize()>"Content & Layout"</Link></li>
                <li><Link href=routes::doc::Stack.materialize()>"Stack Component"</Link></li>
                <li><Link href=routes::doc::CardAndTile.materialize()>"Card & Tile Components"</Link></li>
                <li><Link href=routes::doc::Grid.materialize()>"Grid"</Link>" (interactive rows and cells)"</li>
            </SeeAlso>
        </DocPage>
    }
}

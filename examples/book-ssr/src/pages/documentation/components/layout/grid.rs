use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::grid::GridDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageGrid() -> impl IntoView {
    view! {
        <DocPage title="Grid component">
            <p>
                "The "<Code inline=true>"Grid"</Code>" component is a responsive 12-column layout grid built from "
                <Code inline=true>"Row"</Code>"s and "<Code inline=true>"Col"</Code>"s. It arranges content and has no "
                "keyboard navigation or selection. For an interactive data grid, see the "
                <Link href=routes::doc::Grid.materialize()>"Grid overview"</Link>"."
            </p>

            <Demo
                description="Responsive grid with six items; resize the window to see the columns change"
                source=include_str!("demos/grid.rs")
            >
                <GridDemo/>
            </Demo>

            <Section title="Props">
                <Section title="Grid">
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

                <Section title="Row">
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

                <Section title="Col">
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
                <li><Link href=routes::doc::Grid.materialize()>"Grid overview"</Link></li>
                <li><Link href=routes::doc::components::Stack.materialize()>"Stack"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::table::TableDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageTable() -> impl IntoView {
    view! {
        <DocPage title="Table component">
            <p>
                "The themed table components render a native HTML "<Code inline=true>"<table>"</Code>
                " from "<Code inline=true>"Table"</Code>", "<Code inline=true>"TableHeader"</Code>", "
                <Code inline=true>"TableBody"</Code>", "<Code inline=true>"TableRow"</Code>" and cell components. "
                "They style static tabular data; they add no keyboard navigation or selection. See the "
                <Link href=routes::doc::Table.materialize()>"Table overview"</Link>" for concept guidance."
            </p>

            <Demo description="Bordered, hoverable table rendered from a list of rows" source=include_str!("demos/table.rs")>
                <TableDemo/>
            </Demo>

            <Section title="Props">
                <Section title="Table">
                    <ApiTable kind=ApiKind::Props of="components::table::Table">
                        <ApiRow name="bordered" ty="Option<bool>" default="None">
                            "Collapses the cell borders and draws a line below the header cells. "
                            <Code inline=true>"None"</Code>" means "<Code inline=true>"false"</Code>"."
                        </ApiRow>
                        <ApiRow name="hoverable" ty="Option<bool>" default="None">
                            "Highlights the body row under the mouse pointer. "<Code inline=true>"None"</Code>
                            " means "<Code inline=true>"false"</Code>"."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles of the "<Code inline=true>"<table>"</Code>"."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">
                            "A "<Code inline=true>"TableHeader"</Code>", "<Code inline=true>"TableBody"</Code>
                            " and optionally a "<Code inline=true>"TableFooter"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="TableHeaderCell">
                    <ApiTable kind=ApiKind::Props of="TableHeaderCell">
                        <ApiRow name="min_width" ty="Option<bool>" default="None">
                            "Shrinks the column to the width of its content. "<Code inline=true>"None"</Code>
                            " behaves like "<Code inline=true>"true"</Code>"; pass "<Code inline=true>"false"</Code>
                            " to let the column grow."
                        </ApiRow>
                        <ApiRow name="on_press" ty="Option<Callback<PressEvent>>" default="None">
                            "Called when the header cell is pressed, for example to sort by this column."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles of the "<Code inline=true>"<th>"</Code>"."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">"The cell content."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="TableContainer, TableHeader, TableBody, TableFooter, TableRow, TableCell">
                    <p>
                        "These components take "<Code inline=true>"classes"</Code>", "<Code inline=true>"styles"</Code>
                        " and children, and render a "<Code inline=true>"<div>"</Code>" wrapper, "
                        <Code inline=true>"<thead>"</Code>", "<Code inline=true>"<tbody>"</Code>", "
                        <Code inline=true>"<tfoot>"</Code>", "<Code inline=true>"<tr>"</Code>" and "
                        <Code inline=true>"<td>"</Code>" respectively. Wrap a table in a "
                        <Code inline=true>"TableContainer"</Code>" to give it the theme\u{2019}s rounded frame and shadow, "
                        "and to let it scroll horizontally when it doesn\u{2019}t fit."
                    </p>
                </Section>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt tables to your design:"</p>
                <CssVariables prefix="--table-" scss=theme_scss!("table")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Table.materialize()>"Table overview"</Link></li>
                <li><Link href=routes::doc::table::Hook.materialize()>"use_table"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

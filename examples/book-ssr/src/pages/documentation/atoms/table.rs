use indoc::indoc;
use leptos::prelude::*;

use super::demos::{table::TableAtomDemo, table_resizing::TableResizingAtomDemo};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomTable() -> impl IntoView {
    view! {
        <DocPage title="Table Atoms">
            <p>
                "The table atoms render an unstyled, accessible data table with the behavior of the "
                <Link href=routes::doc::table::Hook.materialize()>"Table Hooks"</Link>": "<Code inline=true>"Table"</Code>
                " holds the state, "<Code inline=true>"TableHeader"</Code>" renders the column headers, and you render the "
                "rows and cells. See the "<Link href=routes::doc::Table.materialize()>"Table overview"</Link>" for concept "
                "guidance."
            </p>

            <Section title="Hooks Used">
                <DocTable headers=&["Atom", "Hooks"]>
                    <TableRow>
                        <TableCell><Code inline=true>"Table"</Code></TableCell>
                        <TableCell>
                            <Link href=format!("{}#use-table-state", routes::doc::table::Hook.materialize())>"use_table_state"</Link>", "
                            <Link href=format!("{}#use-table", routes::doc::table::Hook.materialize())>"use_table"</Link>
                            "; in a "<Code inline=true>"ResizableTableContainer"</Code>" "
                            <Link href=format!("{}#use-table-column-resize-state", routes::doc::table::Hook.materialize())>"use_table_column_resize_state"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"TableHeader"</Code></TableCell>
                        <TableCell>
                            <Link href=format!("{}#use-grid-row-group", routes::doc::grid::Hook.materialize())>"use_grid_row_group"</Link>", "<Link href=format!("{}#use-table-header-row", routes::doc::table::Hook.materialize())>"use_table_header_row"</Link>", "
                            <Link href=format!("{}#use-table-header-placeholder", routes::doc::table::Hook.materialize())>"use_table_header_placeholder"</Link>", "
                            <Link href=format!("{}#use-table-column-header", routes::doc::table::Hook.materialize())>"use_table_column_header"</Link>
                            ", "<Link href=format!("{}#use-table-select-all-checkbox", routes::doc::table::Hook.materialize())>"use_table_select_all_checkbox"</Link>" with "<Link href=routes::doc::checkbox::Hook.materialize()>"use_checkbox"</Link>
                            ", "<Link href=format!("{}#use-table-column-resize", routes::doc::table::Hook.materialize())>"use_table_column_resize"</Link>" for resizers"
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"ResizableTableContainer"</Code></TableCell>
                        <TableCell>"None (it measures its width with "<Code inline=true>"leptos_use::use_resize_observer"</Code>")"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"TableBody"</Code></TableCell>
                        <TableCell><Link href=format!("{}#use-grid-row-group", routes::doc::grid::Hook.materialize())>"use_grid_row_group"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"TableRow"</Code></TableCell>
                        <TableCell><Link href=format!("{}#use-table-row", routes::doc::table::Hook.materialize())>"use_table_row"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"TableCell"</Code></TableCell>
                        <TableCell>
                            <Link href=format!("{}#use-table-cell", routes::doc::table::Hook.materialize())>"use_table_cell"</Link>
                            "; in the checkbox column "<Link href=format!("{}#use-table-selection-checkbox", routes::doc::table::Hook.materialize())>"use_table_selection_checkbox"</Link>" with "
                            <Link href=routes::doc::checkbox::Hook.materialize()>"use_checkbox"</Link>
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Example">
                <p>
                    "Describe the columns and rows as a "
                    <Link href=format!("{}#tablecollection", routes::doc::table::Hook.materialize())>"TableCollection"</Link>
                    ", then render a "<Code inline=true>"TableRow"</Code>" per row with a "<Code inline=true>"TableCell"</Code>
                    " per data column, in the order of the collection. Cells name their column by key:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use std::sync::Arc;

                        use leptonic::{
                            atoms::table::{Table, TableBody, TableCell, TableHeader, TableRow},
                            hooks::{SelectionMode, TableCollection},
                        };
                        use leptos::prelude::*;

                        let table = Memo::new(|_| Arc::new(TableCollection::build(|t| {
                            t.column("name", "Name").row_header();
                            t.column("email", "Email");
                            t.row("ada", "Ada", |r| { r.cell("Ada"); r.cell("ada@example.com"); });
                            t.row("bob", "Bob", |r| { r.cell("Bob"); r.cell("bob@example.com"); });
                        })));

                        view! {
                            <Table table=table selection_mode=SelectionMode::Single aria_label="People">
                                <TableHeader/>
                                <TableBody>
                                    <TableRow key="ada">
                                        <TableCell column="name">"Ada"</TableCell>
                                        <TableCell column="email">"ada@example.com"</TableCell>
                                    </TableRow>
                                    <TableRow key="bob">
                                        <TableCell column="name">"Bob"</TableCell>
                                        <TableCell column="email">"bob@example.com"</TableCell>
                                    </TableRow>
                                </TableBody>
                            </Table>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "A team table with selection checkboxes and two sortable columns. Sorting and selection live in "
                    "signals of the demo: an "<Code inline=true>"RwSignal<Option<SortDescriptor>>"</Code>" passed as "
                    <Code inline=true>"sort_descriptor"</Code>" and "<Code inline=true>"set_sort_descriptor"</Code>", and an "
                    <Code inline=true>"RwSignal<Selection>"</Code>" passed as "<Code inline=true>"selection"</Code>" and "
                    <Code inline=true>"set_selection"</Code>". The table shows them, the user\u{2019}s sorting and "
                    "selecting writes them, and the buttons below reset them. Setting the sort descriptor to "
                    <Code inline=true>"None"</Code>" clears the sorting; the rows return to the order the members joined. "
                    "The table itself doesn\u{2019}t reorder rows: the demo sorts its data by the descriptor and rebuilds "
                    "the collection from it. Alan is on leave: with "
                    <Code inline=true>"DisabledBehavior::All"</Code>", his row can\u{2019}t be selected or focused. The "
                    "demo is styled through the data attributes listed below."
                </p>
                <Demo
                    description="Team table built from the table atoms: selection checkboxes with select all, sortable columns, a disabled row, sorting and selection controlled by app state, styled through data attributes"
                    source=include_str!("demos/table.rs")
                >
                    <TableAtomDemo/>
                </Demo>
            </Section>

            <Section title="Table">
                <p>
                    "Creates the table state and renders the "<Code inline=true>"<table>"</Code>". Its children are a "
                    <Code inline=true>"TableHeader"</Code>" and a "<Code inline=true>"TableBody"</Code>"."
                </p>
                <Section title="Props" id="table-props">
                    <ApiTable kind=ApiKind::Props of="atoms::table::Table">
                        <ApiRow name="table" ty="Memo<Arc<TableCollection>>">
                            "The columns and rows. Required. Build it with "<Code inline=true>"TableCollection::build_with"</Code>" and "
                            <Code inline=true>"show_selection_checkboxes: true"</Code>" for a checkbox column."
                        </ApiRow>
                        <ApiRow name="focus_mode" ty="GridFocusMode" default="Row">
                            <Code inline=true>"Row"</Code>": arrow keys move between rows, arrow right enters the cells. "
                            <Code inline=true>"Cell"</Code>": they move between cells."
                        </ApiRow>
                        <ApiRow name="selection_mode" ty="Signal<SelectionMode>" default="None">"No selection when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="selection_behavior" ty="SelectionBehavior" default="Toggle">"Whether clicks toggle rows or replace the selection."</ApiRow>
                        <ApiRow name="default_selected_keys" ty="Vec<Key>" default="empty">"The initially selected rows."</ApiRow>
                        <ApiRow name="selection" ty="Option<Signal<Selection>>" default="None">
                            "The selection (controlled), replacing "<Code inline=true>"default_selected_keys"</Code>": a value or any signal."
                        </ApiRow>
                        <ApiRow name="set_selection" ty="Option<Out<Selection>>" default="None">
                            "Receives the new state: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="on_selection_change" ty="Option<Callback<Selection>>" default="None">
                            "Called with the new selection. \u{201c}Select all\u{201d} reports "<Code inline=true>"Selection::All"</Code>"."
                        </ApiRow>
                        <ApiRow name="disabled_keys" ty="Option<Signal<HashSet<Key>>>" default="None">
                            "Rows that can\u{2019}t be selected, besides rows built with "<Code inline=true>".disabled(true)"</Code>"."
                        </ApiRow>
                        <ApiRow name="disabled_behavior" ty="Option<DisabledBehavior>" default="None">
                            <Code inline=true>"None"</Code>" means "<Code inline=true>"Selection"</Code>": disabled rows can be "
                            "focused, not selected. "<Code inline=true>"All"</Code>": they can\u{2019}t be focused either."
                        </ApiRow>
                        <ApiRow name="disallow_empty_selection" ty="bool" default="false">"Keep at least one row selected."</ApiRow>
                        <ApiRow name="escape_key_behavior" ty="EscapeKeyBehavior" default="ClearSelection">"What Escape does."</ApiRow>
                        <ApiRow name="default_sort_descriptor" ty="Option<SortDescriptor>" default="None">"The initial sorting."</ApiRow>
                        <ApiRow name="sort_descriptor" ty="Option<Signal<Option<SortDescriptor>>>" default="None">
                            "The sorting (controlled), replacing "<Code inline=true>"default_sort_descriptor"</Code>": a value or any signal. "<Code inline=true>"None"</Code>" clears the sorting."
                        </ApiRow>
                        <ApiRow name="set_sort_descriptor" ty="Option<Out<Option<SortDescriptor>>>" default="None">
                            "Receives the new sorting: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="on_sort_change" ty="Option<Callback<SortDescriptor>>" default="None">
                            "Called when the user sorts the table. Sort your rows accordingly."
                        </ApiRow>
                        <ApiRow name="keyboard_navigation_behavior" ty="KeyboardNavigationBehavior" default="Arrow">
                            "How the keyboard reaches interactive elements inside cells."
                        </ApiRow>
                        <ApiRow name="on_row_action, on_cell_action" ty="Option<Callback<Key>>" default="None">
                            "Called with the key of an activated row or cell."
                        </ApiRow>
                        <ApiRow name="aria_label, aria_labelledby" ty="MaybeProp<String>, Option<String>" default="None">"Names the table."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<table>"</Code>"."</ApiRow>
                        <ApiRow name="children" ty="Children">"A "<Code inline=true>"TableHeader"</Code>" and a "<Code inline=true>"TableBody"</Code>". Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="TableHeader">
                <p>
                    "Renders the "<Code inline=true>"<thead>"</Code>" with all header rows: the column headers, the headers "
                    "of column groups above their columns, and empty placeholders where a column has no group. The header of "
                    "the checkbox column holds the \u{201c}select all\u{201d} checkbox in multiple selection mode."
                </p>
                <Section title="Props" id="table-header-props">
                    <ApiTable kind=ApiKind::Props of="atoms::table::TableHeader">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<thead>"</Code>"."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="TableBody">
                <p>"Renders the "<Code inline=true>"<tbody>"</Code>" around your rows."</p>
                <Section title="Props" id="table-body-props">
                    <ApiTable kind=ApiKind::Props of="atoms::table::TableBody">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<tbody>"</Code>"."</ApiRow>
                        <ApiRow name="children" ty="Children">"One "<Code inline=true>"TableRow"</Code>" per row of the collection. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="TableRow">
                <p>
                    "A body row. With a checkbox column, it renders the selection cell itself; add a "
                    <Code inline=true>"TableCell"</Code>" for each data column."
                </p>
                <Section title="Props" id="table-row-props">
                    <ApiTable kind=ApiKind::Props of="atoms::table::TableRow">
                        <ApiRow name="key" ty="Key">"The row\u{2019}s key in the table\u{2019}s collection. Required."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<tr>"</Code>"."</ApiRow>
                        <ApiRow name="children" ty="Children">"The row\u{2019}s cells. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="TableCell">
                <p>
                    "A body cell, rendered as "<Code inline=true>"<td>"</Code>". It finds its cell in the collection by its row "
                    "and its "<Code inline=true>"column"</Code>" key. Cells of a row header column get "
                    <Code inline=true>"role=\"rowheader\""</Code>"."
                </p>
                <Section title="Props" id="table-cell-props">
                    <ApiTable kind=ApiKind::Props of="atoms::table::TableCell">
                        <ApiRow name="column" ty="Key">"The key of the cell\u{2019}s column. Required."</ApiRow>
                        <ApiRow name="focus_mode" ty="Option<CellFocusMode>" default="None">
                            "What gets focus: the cell, or its first focusable child. "<Code inline=true>"None"</Code>": the child, "
                            "or the cell with "<Code inline=true>"KeyboardNavigationBehavior::Tab"</Code>"."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<td>"</Code>"."</ApiRow>
                        <ApiRow name="children" ty="Option<Children>" default="None">"The cell\u{2019}s content."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ResizableTableContainer">
                <p>
                    "Makes the columns of the "<Code inline=true>"Table"</Code>" inside it resizable. It measures its own "
                    "width and shares it among the columns by their "<Code inline=true>"default_width"</Code>", "
                    <Code inline=true>"min_width"</Code>" and "<Code inline=true>"max_width"</Code>" (see "
                    <Link href=format!("{}#column-resizing", routes::doc::table::Hook.materialize())>"column resizing"</Link>
                    "). The table switches to "<Code inline=true>"table-layout: fixed"</Code>" with "
                    <Code inline=true>"width: min-content"</Code>", its column headers get pixel widths, and the headers of "
                    "columns built with "<Code inline=true>".allows_resizing()"</Code>" get a resizer."
                </p>
                <Section title="Props" id="resizable-table-container-props">
                    <ApiTable kind=ApiKind::Props of="ResizableTableContainer">
                        <ApiRow name="on_resize_start" ty="Option<Callback<HashMap<Key, ColumnSize>>>" default="None">
                            "Called with the size of every column when the user starts resizing a column."
                        </ApiRow>
                        <ApiRow name="on_resize" ty="Option<Callback<HashMap<Key, ColumnSize>>>" default="None">
                            "Called with the sizes whenever a column is resized."
                        </ApiRow>
                        <ApiRow name="on_resize_end" ty="Option<Callback<HashMap<Key, ColumnSize>>>" default="None">
                            "Called with the sizes when resizing ends. Columns before the resized one are reported in pixels; "
                            "the columns after it keep their sizes."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Classes and styles of the container "<Code inline=true>"<div>"</Code>". Give it "
                            <Code inline=true>"overflow: auto"</Code>": when the columns\u{2019} minimum widths don\u{2019}t fit, "
                            "the table gets wider than the container."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">"A "<Code inline=true>"Table"</Code>". Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Column Resizing">
                <p>
                    "Wrap the "<Code inline=true>"Table"</Code>" in a "<Code inline=true>"ResizableTableContainer"</Code>" and "
                    "size its columns in the builder:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        t.column("title", "Task").allows_resizing().default_width(ColumnSize::Fr(2.0));
                        t.column("assignee", "Assignee").allows_resizing().min_width(ColumnBound::Px(120.0));
                        t.column("due", "Due").default_width(ColumnSize::Px(90.0));

                        view! {
                            <ResizableTableContainer classes="my-table-scroll" on_resize_end=save_widths>
                                <Table table=table aria_label="Tasks" classes="my-table">
                                    <TableHeader/>
                                    <TableBody>/* rows */</TableBody>
                                </Table>
                            </ResizableTableContainer>
                        }
                    "#)}
                </Code>
                <p>
                    "Without "<Code inline=true>"default_width"</Code>" a column takes "<Code inline=true>"ColumnSize::Fr(1.0)"</Code>
                    ", an equal share of the space the pixel and percent columns leave; without "<Code inline=true>"min_width"</Code>
                    " it is at least 75 pixels wide. The resizer brings no styles: position it at the end of its header, "
                    "as shown under "<Link href=format!("{}#styling", routes::doc::table::Atom.materialize())>"Styling"</Link>"."
                </p>
                <p>
                    "Drag the line at the end of a column header. With the keyboard, click a row, press "<Keys keys="ArrowUp"/>
                    " to reach the column headers and move to a resizable column with "<Keys keys="ArrowLeft"/>" / "
                    <Keys keys="ArrowRight"/>": its resizer takes the focus. Press "<Keys keys="Enter"/>", change the width with the "
                    "arrow keys (10 pixels per press) and finish with "<Keys keys="Enter"/>", "<Keys keys="Escape"/>" or "
                    <Keys keys="Tab"/>". Task and Assignee share the free space 2:1: when you resize Task, Assignee takes what "
                    "is left. Resizing Assignee or Status fixes the columns before it at their current pixel widths. "
                    "Status stays between 90 and 200 pixels, and Due can\u{2019}t be resized."
                </p>
                <Demo
                    description="Task table in a ResizableTableContainer: resizable columns with minimum and maximum widths, a fixed column, and the sizes reported by the resize callbacks"
                    source=include_str!("demos/table_resizing.rs")
                >
                    <TableResizingAtomDemo/>
                </Demo>
            </Section>

            <Section title="Data Attributes">
                <p>"Flags are rendered as "<Code inline=true>"data-selected=\"true\""</Code>" while the state applies, and are absent otherwise."</p>
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-allows-sorting" ty="true">"On a column header: the table can be sorted by this column."</ApiRow>
                    <ApiRow name="data-sort-direction" ty="\"ascending\" / \"descending\"">
                        "On a column header: the table is sorted by this column, in this direction."
                    </ApiRow>
                    <ApiRow name="data-selected" ty="true">"On "<Code inline=true>"TableRow"</Code>": the row is selected."</ApiRow>
                    <ApiRow name="data-focused" ty="true">
                        "On "<Code inline=true>"TableRow"</Code>" and column headers: it has the table\u{2019}s focus (by keyboard "
                        "or pointer). Not set while a cell of the row is focused."
                    </ApiRow>
                    <ApiRow name="data-disabled" ty="true">
                        "On "<Code inline=true>"TableRow"</Code>": the row is disabled. Only with "
                        <Code inline=true>"DisabledBehavior::All"</Code>"; with the default "<Code inline=true>"Selection"</Code>
                        ", a disabled row only shows through its disabled checkbox."
                    </ApiRow>
                    <ApiRow name="data-pressed" ty="true">"On "<Code inline=true>"TableRow"</Code>", "<Code inline=true>"TableCell"</Code>" and column headers: being pressed."</ApiRow>
                    <ApiRow name="data-resizing" ty="true">
                        "In a "<Code inline=true>"ResizableTableContainer"</Code>", on a column header and its resizer: the "
                        "column is being resized."
                    </ApiRow>
                    <ApiRow name="data-column-resizer" ty="true">
                        "Marks the resizer, a "<Code inline=true>"<div role=\"presentation\">"</Code>" at the end of the header "
                        "of a resizable column, around a visually hidden range input."
                    </ApiRow>
                    <ApiRow name="data-resizable-direction" ty="\"both\" / \"left\" / \"right\"">
                        "On a resizer: "<Code inline=true>"left"</Code>" while the column is at its "
                        "minimum width (it can only grow), "<Code inline=true>"right"</Code>" at its maximum (it can only "
                        "shrink), swapped in right-to-left layouts; otherwise "<Code inline=true>"both"</Code>"."
                    </ApiRow>
                </ApiTable>
                <p>
                    "The ARIA attributes are there to style as well: "<Code inline=true>"aria-selected"</Code>" on rows, "
                    <Code inline=true>"aria-sort"</Code>" on sortable column headers, "<Code inline=true>"role=\"rowheader\""</Code>
                    " on row header cells."
                </p>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms bring no styles. They render a plain table, each element with its default class and the "<Code inline=true>"classes"</Code>" you "
                    "pass: "<Code inline=true>"Table"</Code>" a "<Code inline=true>"<table>"</Code>" ("<Code inline=true>"leptonic-Table"</Code>"), "<Code inline=true>"TableHeader"</Code>" a "<Code inline=true>"<thead>"</Code>" ("
                    <Code inline=true>"leptonic-TableHeader"</Code>") with the column headers, "<Code inline=true>"<th>"</Code>" elements with "<Code inline=true>"leptonic-TableColumnHeader"</Code>", "
                    <Code inline=true>"TableBody"</Code>" a "<Code inline=true>"<tbody>"</Code>" ("<Code inline=true>"leptonic-TableBody"</Code>"), "<Code inline=true>"TableRow"</Code>" a "<Code inline=true>"<tr>"</Code>" ("
                    <Code inline=true>"leptonic-TableRow"</Code>") and "<Code inline=true>"TableCell"</Code>" a "<Code inline=true>"<td>"</Code>" ("<Code inline=true>"leptonic-TableCell"</Code>"). "
                    <Code inline=true>"TableHeader"</Code>" renders the column headers itself, so style them through their default class. Rows, cells and "
                    "column headers receive DOM focus: "<Code inline=true>":focus-visible"</Code>" shows keyboard focus. The demo above uses this CSS:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r#"
                        .demo-table { width: 100%; border-collapse: collapse; background: var(--surface); }
                        .demo-table :is(th, td) { padding: 0.5rem 1rem; border-bottom: 1px solid var(--border); text-align: left; }
                        .demo-table :is(tr, th, td):focus-visible { outline: 2px solid var(--focus); outline-offset: -2px; }
                        .demo-atom-table .leptonic-TableColumnHeader[data-allows-sorting] { cursor: pointer; user-select: none; }
                        .demo-atom-table .leptonic-TableColumnHeader[data-allows-sorting]::after { content: "\2195"; opacity: 0.4; }
                        .demo-atom-table .leptonic-TableColumnHeader[data-sort-direction]::after { color: var(--accent); opacity: 1; }
                        .demo-atom-table .leptonic-TableColumnHeader[data-sort-direction="ascending"]::after { content: "\25B2"; }
                        .demo-atom-table .leptonic-TableColumnHeader[data-sort-direction="descending"]::after { content: "\25BC"; }
                        .demo-atom-table-row[data-selected] { background: var(--border); }
                        .demo-atom-table-row[data-disabled] { color: var(--muted); text-decoration: line-through; cursor: not-allowed; }
                    "#)}
                </Code>
                <p>
                    "In a "<Code inline=true>"ResizableTableContainer"</Code>" ("<Code inline=true>"leptonic-ResizableTableContainer"</Code>"), the table lays out its columns "
                    "by the pixel widths of their headers. Position the resizers ("<Code inline=true>"leptonic-ColumnResizer"</Code>") yourself. Keyboard focus "
                    "rests on the resizer\u{2019}s visually hidden input, so show it with "<Code inline=true>":has()"</Code>":"
                </p>
                <Code language=Language::Css>
                    {indoc!(r#"
                        .demo-table-scroll { max-width: 100%; overflow-x: auto; }
                        .demo-resizable-table :is(th, td) { box-sizing: border-box; overflow: hidden; text-overflow: ellipsis; }
                        .demo-resizable-table .leptonic-TableColumnHeader { position: relative; }
                        .demo-resizable-table .leptonic-ColumnResizer { position: absolute; top: 0; right: 0; width: 9px; height: 100%; cursor: col-resize; }
                        .demo-resizable-table .leptonic-ColumnResizer[data-resizable-direction="left"] { cursor: e-resize; }
                        .demo-resizable-table .leptonic-ColumnResizer[data-resizable-direction="right"] { cursor: w-resize; }
                        .demo-resizable-table .leptonic-ColumnResizer:is(:hover, [data-resizing], :has(input:focus-visible)) { background: var(--accent); }
                    "#)}
                </Code>
                <p>
                    "Apps that don\u{2019}t want to style from scratch can load leptonic\u{2019}s optional atom theme, "
                    <Code inline=true>"@use \"leptonic/leptonic-atoms\";"</Code>", which styles the default classes."
                </p>
            </Section>

            <Section title="Composition">
                <p>
                    "The parts find the table through the "<Code inline=true>"TableData"</Code>" context. Your own Leptos components "
                    "inside "<Code inline=true>"Table"</Code>" can read it too, e.g. to show the number of selected rows in a "
                    "cell:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::hooks::TableData;
                        use leptos::prelude::*;

                        #[component]
                        fn SelectedCount() -> impl IntoView {
                            let selection = expect_context::<TableData>().state.grid.list.selection;
                            move || format!("{} selected", selection.selected_keys().len())
                        }
                    "#)}
                </Code>
                <p>
                    <Code inline=true>"state"</Code>" is the "<Code inline=true>"TableState"</Code>" documented under "
                    <Link href=format!("{}#use-table-state", routes::doc::table::Hook.materialize())>"use_table_state"</Link>
                    " on the "<Link href=routes::doc::table::Hook.materialize()>"Table Hooks"</Link>" page."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Table.materialize()>"Table overview"</Link></li>
                <li><Link href=routes::doc::table::Hook.materialize()>"Table Hooks"</Link></li>
                <li><Link href=routes::doc::grid::Atom.materialize()>"Grid Atoms"</Link></li>
                <li><Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

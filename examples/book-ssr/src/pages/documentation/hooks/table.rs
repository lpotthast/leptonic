use indoc::indoc;
use leptos::prelude::*;

use super::demos::{table::TableHookDemo, table_resizing::TableResizingHookDemo};
use crate::{kit::*, routes};

#[component]
pub fn PageUseTableHook() -> impl IntoView {
    view! {
        <DocPage title="Table Hooks">
            <p>
                "The table hooks build an accessible data table: column headers above rows of cells, navigated in two "
                "dimensions with the arrow keys, with sortable columns and selectable rows. You render every element "
                "yourself. See the "<Link href=routes::doc::Table.materialize()>"Table overview"</Link>" for concept guidance "
                "and keyboard interaction."
            </p>
            <ReactAria hook="useTable"/>

            <TableBasics/>

            <TableCollectionSection/>

            <TableStateSections/>

            <TableHeaderSections/>

            <TableBodySections/>

            <TableResizingSections/>

            <SeeAlso>
                <li><Link href=routes::doc::Table.materialize()>"Table overview"</Link></li>
                <li><Link href=routes::doc::table::Atom.materialize()>"Table Atoms"</Link></li>
                <li><Link href=routes::doc::grid::Hook.materialize()>"Grid Hooks"</Link>" \u{2014} the grid the table builds on"</li>
                <li><Link href=routes::doc::checkbox::Hook.materialize()>"Checkbox Hooks"</Link></li>
                <li><Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link></li>
                <li><Link href=routes::doc::interactions::UseMove.materialize()>"use_move"</Link>" \u{2014} the pointer and keyboard moves behind the resizer"</li>
            </SeeAlso>
        </DocPage>
    }
}

#[component]
fn TableBasics() -> impl IntoView {
    view! {
        <Section title="Demo">
            <p>
                "A table of books built from the hooks alone. Press a column header with an arrow to sort by it (press "
                "it again to reverse the order). Select rows with their checkboxes, by clicking them or with "
                <Keys keys="Space"/>"; the checkbox in the header selects all rows. \u{201c}Neuromancer\u{201d} is out of "
                "print: it can be focused, but not selected. Click a row, then explore with the arrow keys \u{2014} "
                <Keys keys="ArrowUp"/>" from the first row reaches the column headers."
            </p>
            <Demo
                description="Book table built with the table hooks: sortable columns, a column group, multiple selection with selection checkboxes and select all, and a row that can\u{2019}t be selected"
                source=include_str!("demos/table.rs")
            >
                <TableHookDemo/>
            </Demo>
        </Section>

        <Section title="Building a Table">
            <p>
                "A table takes three steps: describe its columns and rows as a "<Code inline=true>"TableCollection"</Code>
                ", create its state with "<Code inline=true>"use_table_state"</Code>", and render it with "
                <Code inline=true>"use_table"</Code>". Then each element gets its own hook:"
            </p>
            <DocTable headers=&["Element", "Hook", "Role"]>
                <TableRow>
                    <TableCell><Code inline=true>"<table>"</Code></TableCell>
                    <TableCell><Code inline=true>"use_table"</Code></TableCell>
                    <TableCell><Code inline=true>"grid"</Code></TableCell>
                </TableRow>
                <TableRow>
                    <TableCell><Code inline=true>"<thead>"</Code>", "<Code inline=true>"<tbody>"</Code></TableCell>
                    <TableCell><Link href=format!("{}#use-grid-row-group", routes::doc::grid::Hook.materialize())>"use_grid_row_group"</Link></TableCell>
                    <TableCell><Code inline=true>"rowgroup"</Code></TableCell>
                </TableRow>
                <TableRow>
                    <TableCell>"Header row"</TableCell>
                    <TableCell><Code inline=true>"use_table_header_row"</Code></TableCell>
                    <TableCell><Code inline=true>"row"</Code></TableCell>
                </TableRow>
                <TableRow>
                    <TableCell>"Column header"</TableCell>
                    <TableCell><Code inline=true>"use_table_column_header"</Code></TableCell>
                    <TableCell><Code inline=true>"columnheader"</Code></TableCell>
                </TableRow>
                <TableRow>
                    <TableCell>"Empty header cell (above a column without group)"</TableCell>
                    <TableCell><Code inline=true>"use_table_header_placeholder"</Code></TableCell>
                    <TableCell><Code inline=true>"gridcell"</Code></TableCell>
                </TableRow>
                <TableRow>
                    <TableCell>"Body row"</TableCell>
                    <TableCell><Code inline=true>"use_table_row"</Code></TableCell>
                    <TableCell><Code inline=true>"row"</Code></TableCell>
                </TableRow>
                <TableRow>
                    <TableCell>"Body cell"</TableCell>
                    <TableCell><Code inline=true>"use_table_cell"</Code></TableCell>
                    <TableCell><Code inline=true>"gridcell"</Code>" or "<Code inline=true>"rowheader"</Code></TableCell>
                </TableRow>
                <TableRow>
                    <TableCell>"Selection checkboxes"</TableCell>
                    <TableCell>
                        <Code inline=true>"use_table_selection_checkbox"</Code>", "
                        <Code inline=true>"use_table_select_all_checkbox"</Code>
                    </TableCell>
                    <TableCell><Code inline=true>"checkbox"</Code></TableCell>
                </TableRow>
                <TableRow>
                    <TableCell>"Column resizer (inside a column header)"</TableCell>
                    <TableCell>
                        <Code inline=true>"use_table_column_resize_state"</Code>", "
                        <Code inline=true>"use_table_column_resize"</Code>
                    </TableCell>
                    <TableCell><Code inline=true>"presentation"</Code>", around a range input ("<Code inline=true>"slider"</Code>")"</TableCell>
                </TableRow>
            </DocTable>
            <p>
                "The collection is built from your data, so it exists during server-side rendering too. Render the rows "
                "from the same data, in the same order. If you don\u{2019}t need full control over the markup, the "
                <Link href=routes::doc::table::Atom.materialize()>"Table Atoms"</Link>" do all of this for you."
            </p>
        </Section>
    }
}

#[component]
fn TableCollectionSection() -> impl IntoView {
    view! {
        <Section title="TableCollection">
            <p>
                "Describes the columns and rows. Build it inside a "<Code inline=true>"Memo"</Code>": the builder function "
                "runs again when a signal it reads changes, so the table follows your data (and its order)."
            </p>
            <Code language=Language::Rust>
                {indoc!(r#"
                    let table = Memo::new(move |_| {
                        Arc::new(TableCollection::build(|t| {
                            t.column("name", "Name").row_header().allows_sorting();
                            t.column_group("contact", "Contact", |g| {
                                g.column("email", "Email");
                                g.column("phone", "Phone");
                            });
                            people.with(|people| {
                                for person in people {
                                    t.row(person.id, person.name.clone(), |r| {
                                        r.cell(person.name.clone());
                                        r.cell(person.email.clone());
                                        r.cell(person.phone.clone());
                                    })
                                    .disabled(person.locked);
                                }
                            });
                        }))
                    });
                "#)}
            </Code>

            <DocTable headers=&["Builder method", "Adds"]>
                <TableRow>
                    <TableCell><Code inline=true>"column(key, text_value)"</Code></TableCell>
                    <TableCell>
                        "A data column. Chain "<Code inline=true>".row_header()"</Code>" if its cells label their rows, "
                        <Code inline=true>".allows_sorting()"</Code>" if the table can be sorted by it, and the sizing "
                        "methods of "<Link href=format!("{}#column-resizing", routes::doc::table::Hook.materialize())>"column resizing"</Link>"."
                    </TableCell>
                </TableRow>
                <TableRow>
                    <TableCell><Code inline=true>"column_group(key, text_value, |g| ..)"</Code></TableCell>
                    <TableCell>
                        "A header spanning the columns added in the closure, in a header row above them. Groups can be "
                        "nested."
                    </TableCell>
                </TableRow>
                <TableRow>
                    <TableCell><Code inline=true>"row(key, text_value, |r| ..)"</Code></TableCell>
                    <TableCell>
                        "A body row with one "<Code inline=true>"r.cell(text_value)"</Code>" per data column. The row\u{2019}s "
                        "text value is used for type-ahead. Chain "<Code inline=true>".disabled(bool)"</Code>" for a row that "
                        "can\u{2019}t be selected."
                    </TableCell>
                </TableRow>
            </DocTable>

            <p>
                "Cell keys derive from their row and column. Use "<Code inline=true>"table.cell_key(&row, column_index)"</Code>
                " to look them up. When no column is marked as row header, the first data column labels the rows. "
                "Set "<Code inline=true>"show_selection_checkboxes"</Code>" on the table state to add a selection "
                "column while selection is enabled. Read the resulting columns from "<Code inline=true>"state.columns"</Code>"."
            </p>

            <p>
                "For a collection built outside the table state, "<Code inline=true>"with_selection_column()"</Code>
                " returns a copy with the leading selection column, preserving an existing one. "
                <Code inline=true>"has_selection_column()"</Code>" reports whether it is present."
            </p>
            <p>"Read the table through these methods:"</p>
            <DocTable headers=&["Method", "Description"]>
                <TableRow>
                    <TableCell><Code inline=true>"collection() -> &Arc<Collection>"</Code></TableCell>
                    <TableCell>"The header rows followed by the body rows, as a grid collection."</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell><Code inline=true>"rows() -> impl Iterator<Item = &Node>"</Code></TableCell>
                    <TableCell>"The body rows."</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell><Code inline=true>"size() -> usize"</Code></TableCell>
                    <TableCell>"The number of body rows."</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell><Code inline=true>"columns() -> impl Iterator<Item = &Column>"</Code></TableCell>
                    <TableCell>"The data columns in order (with the checkbox column, without groups)."</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell><Code inline=true>"column_count() -> usize"</Code></TableCell>
                    <TableCell>"The number of data columns."</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell><Code inline=true>"column(&Key), column_at(usize) -> Option<&Column>"</Code></TableCell>
                    <TableCell>"A column (or group) by key, a data column by index."</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell><Code inline=true>"cell_column(&Key) -> Option<&Column>"</Code></TableCell>
                    <TableCell>"The column of a body cell."</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell><Code inline=true>"header_rows() -> &[Key]"</Code></TableCell>
                    <TableCell>"The keys of the header rows, top to bottom."</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell><Code inline=true>"row_header_columns() -> &[Key]"</Code></TableCell>
                    <TableCell>"The columns whose cells label their rows."</TableCell>
                </TableRow>
            </DocTable>

            <Section title="Column">
                <ApiTable kind=ApiKind::Fields of="Column">
                    <ApiRow name="key" ty="Key">"The column\u{2019}s key."</ApiRow>
                    <ApiRow name="text_value" ty="Arc<str>">"The header text."</ApiRow>
                    <ApiRow name="kind" ty="ColumnKind">
                        <Code inline=true>"Data"</Code>", "<Code inline=true>"Group"</Code>" or "
                        <Code inline=true>"SelectionCheckbox"</Code>"."
                    </ApiRow>
                    <ApiRow name="is_row_header" ty="bool">"Its cells label their rows."</ApiRow>
                    <ApiRow name="allows_sorting" ty="bool">"The table can be sorted by this column."</ApiRow>
                    <ApiRow name="allows_resizing" ty="bool">"The column gets a resizer (with column resizing)."</ApiRow>
                    <ApiRow name="default_width" ty="Option<ColumnSize>">"The initial width (with column resizing)."</ApiRow>
                    <ApiRow name="min_width, max_width" ty="Option<ColumnBound>">"The width bounds (with column resizing)."</ApiRow>
                    <ApiRow name="index, col_span" ty="usize">"The first data column it covers, and how many (more than one for groups)."</ApiRow>
                    <ApiRow name="level" ty="usize">"The header row it is in."</ApiRow>
                    <ApiRow name="parent" ty="Option<Key>">"The group containing it."</ApiRow>
                    <ApiRow name="children" ty="Vec<Key>">"The columns of a group."</ApiRow>
                </ApiTable>
            </Section>
        </Section>
    }
}

#[component]
fn TableStateSections() -> impl IntoView {
    view! {
        <Section title="use_table_state">
            <p>"Holds the rows\u{2019} selection, the focus and the sorting of a table."</p>

            <Section title="Input" id="use-table-state-input">
                <p>"Pass a "<Code inline=true>"UseTableStateInput"</Code>" with every field named; the Default column gives the value for fields you don\u{2019}t need."</p>
                <ApiTable kind=ApiKind::Input of="UseTableStateInput">
                    <ApiRow name="table" ty="Memo<Arc<TableCollection>>">"The columns and rows."</ApiRow>
                    <ApiRow name="selection" ty="SelectionOptions" default="no selection">
                        "Row selection, see "<Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link>". "
                        "Tables usually set "<Code inline=true>"disabled_behavior"</Code>" to "
                        <Code inline=true>"DisabledBehavior::Selection"</Code>": disabled rows can be focused, not selected."
                    </ApiRow>
                    <ApiRow name="focus_mode" ty="GridFocusMode" default="Row">
                        <Code inline=true>"Row"</Code>": arrow keys move between rows, arrow right enters the cells. "
                        <Code inline=true>"Cell"</Code>": they move between cells."
                    </ApiRow>
                    <ApiRow name="default_sort_descriptor" ty="Option<SortDescriptor>" default="None">"The initial sorting."</ApiRow>
                    <ApiRow name="sort_descriptor" ty="Option<ValueBinding<Option<SortDescriptor>>>" default="None">
                        "The sorting as app state ("<Code inline=true>"Some(rw_signal.into())"</Code>"), replacing "<Code inline=true>"default_sort_descriptor"</Code>": the table shows it, sorting by the user writes it, and "<Code inline=true>"None"</Code>" clears it."
                    </ApiRow>
                    <ApiRow name="on_sort_change" ty="Option<Callback<SortDescriptor>>" default="None">
                        "Called when the user sorts the table by pressing a column header."
                    </ApiRow>
                    <ApiRow name="tree" ty="Option<TableTreeInput>" default="None">
                        "Makes the table a tree table: rows with child rows ("<Code inline=true>"ItemBuilder::children"</Code>
                        ") expand and collapse. "<Code inline=true>"TableTreeInput"</Code>" names the column showing the "
                        "hierarchy and holds the expanded rows ("<Code inline=true>"default_expanded_keys"</Code>", "
                        <Code inline=true>"expanded_keys"</Code>", "<Code inline=true>"on_expanded_change"</Code>")."
                    </ApiRow>
                    <ApiRow name="show_selection_checkboxes" ty="Signal<bool>" default="false">"Show the selection column while selection is enabled."</ApiRow>
                </ApiTable>
                <p>
                    <Code inline=true>"SelectionOptions::default()"</Code>" uses "<Code inline=true>"DisabledBehavior::All"</Code>
                    ", which also keeps focus off disabled rows. For a table without selection, pass "
                    <Code inline=true>"SelectionOptions { disabled_behavior: DisabledBehavior::Selection, ..SelectionOptions::default() }"</Code>"."
                </p>
            </Section>

            <Section title="Return" id="use-table-state-return">
                <p>"A "<Code inline=true>"TableState"</Code>" (it is "<Code inline=true>"Copy"</Code>"):"</p>
                <ApiTable kind=ApiKind::Return of="TableState">
                    <ApiRow name="grid" ty="GridState">
                        "The grid state. "<Code inline=true>"grid.list.selection"</Code>" is the selection manager: "
                        <Code inline=true>"selected_keys()"</Code>", "<Code inline=true>"is_selected(&key)"</Code>", "
                        <Code inline=true>"select_all()"</Code>", \u{2026}"
                    </ApiRow>
                    <ApiRow name="table" ty="Memo<Arc<TableCollection>>">
                        "The columns and rows (in a tree table also the child rows of collapsed rows; "
                        <Code inline=true>"grid.list.collection"</Code>" holds the visible rows)."
                    </ApiRow>
                    <ApiRow name="tree" ty="Option<TableTree>">
                        "Set in a tree table: "<Code inline=true>"column()"</Code>" and the "<Code inline=true>"expansion"</Code>"."
                    </ApiRow>
                    <ApiRow name="sort_descriptor" ty="Signal<Option<SortDescriptor>>">"The current sorting."</ApiRow>
                    <ApiRow name="columns" ty="Memo<Arc<[Column]>>">"The currently rendered columns, including the selection column when enabled."</ApiRow>
                </ApiTable>
                <p>
                    <Code inline=true>"sort(&column, direction: Option<SortDirection>)"</Code>" sorts by "
                    <Code inline=true>"column"</Code>": in "<Code inline=true>"direction"</Code>", or with "
                    <Code inline=true>"None"</Code>" ascending \u{2014} reversed if the table is already sorted by that "
                    "column. It calls "<Code inline=true>"on_sort_change"</Code>"."
                </p>
            </Section>

            <Section title="Sorting">
                <p>
                    "The state only tracks the "<Code inline=true>"SortDescriptor { column: Key, direction: SortDirection }"</Code>
                    " ("<Code inline=true>"SortDirection::Ascending"</Code>" or "<Code inline=true>"Descending"</Code>
                    "); the hooks never reorder your rows. Bind the descriptor to a signal of your own with "
                    <Code inline=true>"sort_descriptor"</Code>", sort the data from it and build the table from the "
                    "sorted data. The rows keep their keys, so focus and selection stay with them:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        let sort = RwSignal::new(None::<SortDescriptor>);
                        let people = Memo::new(move |_| sorted(&PEOPLE, sort.get().as_ref()));
                        let table = Memo::new(move |_| Arc::new(TableCollection::build(|t| {
                            t.column("name", "Name").allows_sorting();
                            people.with(|people| {
                                for person in people {
                                    t.row(person.id, person.name, |r| { r.cell(person.name); });
                                }
                            });
                        })));
                        let state = use_table_state(UseTableStateInput {
                            tree: None,
                            table,
                            selection: SelectionOptions {
                                disabled_behavior: DisabledBehavior::Selection,
                                ..SelectionOptions::default()
                            },
                            focus_mode: GridFocusMode::Row,
                            default_sort_descriptor: None,
                            sort_descriptor: Some(sort.into()),
                            on_sort_change: None,
                        });
                    "#)}
                </Code>
            </Section>
        </Section>

        <Section title="use_table">
            <p>"Makes the "<Code inline=true>"<table>"</Code>" element a grid and provides what rows and cells need."</p>

            <Section title="Input" id="use-table-input">
                <p>"Pass a "<Code inline=true>"UseTableInput"</Code>" with every field named; the Default column gives the value for fields you don\u{2019}t need."</p>
                <ApiTable kind=ApiKind::Input of="UseTableInput">
                    <ApiRow name="state" ty="TableState">"From "<Code inline=true>"use_table_state"</Code>"."</ApiRow>
                    <ApiRow name="element" ty="CapturedElement">"The table element. The props capture it."</ApiRow>
                    <ApiRow name="id" ty="Option<String>" default="None">
                        "The element id; cell and column header ids derive from it. Generated when "<Code inline=true>"None"</Code>"."
                    </ApiRow>
                    <ApiRow name="aria_label, aria_labelledby" ty="MaybeProp<String>, Signal<Option<String>>" default="None">"Names the table."</ApiRow>
                    <ApiRow name="keyboard_delegate" ty="Option<Signal<Arc<dyn KeyboardDelegate>>>" default="None">
                        "Replaces the table navigation ("
                        <Link href=format!("{}#use-table-keyboard-delegate", routes::doc::CollectionState.materialize())>"use_table_keyboard_delegate"</Link>")."
                    </ApiRow>
                    <ApiRow name="options" ty="CollectionOptions" default="CollectionOptions::default()">
                        "Keyboard and focus behavior: wrapping, Escape, select all, type-ahead, see "
                        <Link href=format!("{}#collectionoptions", routes::doc::CollectionState.materialize())>"CollectionOptions"</Link>"."
                    </ApiRow>
                    <ApiRow name="keyboard_navigation_behavior" ty="KeyboardNavigationBehavior" default="Arrow">
                        "How the keyboard reaches interactive elements inside cells."
                    </ApiRow>
                    <ApiRow name="should_select_on_press_up" ty="bool" default="false">"Select when the press ends instead of when it starts."</ApiRow>
                    <ApiRow name="on_row_action, on_cell_action" ty="Option<Callback<Key>>" default="None">
                        "Called with the key of an activated row or cell."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return" id="use-table-return">
                <ApiTable kind=ApiKind::Return of="UseTableReturn">
                    <ApiRow name="props" ty="UseGridProps">
                        "For the "<Code inline=true>"<table>"</Code>": id, "<Code inline=true>"role=\"grid\""</Code>", label, "
                        <Code inline=true>"aria-multiselectable"</Code>", the "<Code inline=true>"aria-describedby"</Code>" of the sort "
                        "description (\u{201c}sorted by column Type in ascending order\u{201d}), keyboard and focus handling. "
                        "Sort changes are announced."
                    </ApiRow>
                    <ApiRow name="data" ty="TableData">
                        "Hand this to the row, cell and header hooks. Holds the "<Code inline=true>"state"</Code>", the "
                        <Code inline=true>"grid"</Code>" data and the table\u{2019}s "<Code inline=true>"id"</Code>"."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example" id="use-table-example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        let UseTableReturn { props, data } = use_table(UseTableInput {
                            state,
                            element: CapturedElement::new(),
                            id: None,
                            aria_label: "People".into(),
                            aria_labelledby: None,
                            keyboard_delegate: None,
                            options: CollectionOptions::default(),
                            keyboard_navigation_behavior: KeyboardNavigationBehavior::Arrow,
                            should_select_on_press_up: false,
                            on_row_action: None,
                            on_cell_action: None,
                        });

                        view! {
                            <table {..props.into_attrs()}>
                                <thead {..use_grid_row_group().row_group_props.into_attrs()}>/* header rows */</thead>
                                <tbody {..use_grid_row_group().row_group_props.into_attrs()}>/* body rows */</tbody>
                            </table>
                        }
                    "#)}
                </Code>
            </Section>
        </Section>
    }
}

#[component]
fn TableHeaderSections() -> impl IntoView {
    view! {
        <Section title="use_table_header_row">
            <p>
                "Returns the props of a header row ("<Code inline=true>"role=\"row\""</Code>"). Render one per key in "
                <Code inline=true>"table.header_rows()"</Code>"; its children in the collection are the column headers "
                "and placeholders of that row. A table without column groups has one header row."
            </p>
            <Code language=Language::Rust>
                {indoc!(r"
                    let cells: Vec<(Key, NodeKind)> = table.with_untracked(|t| {
                        t.collection().children(&row).map(|cell| (cell.key.clone(), cell.kind)).collect()
                    });
                    view! {
                        <tr {..use_table_header_row().into_attrs()}>
                            {cells.into_iter().map(|(key, kind)| match kind {
                                NodeKind::Placeholder => view! {
                                    <th {..use_table_header_placeholder(UseTableHeaderPlaceholderInput {
                                        table: data.clone(),
                                        key,
                                    }).into_attrs()}></th>
                                }.into_any(),
                                _ => view! { <ColumnHeader table=data.clone() column=key/> }.into_any(),
                            }).collect_view()}
                        </tr>
                    }
                ")}
            </Code>
        </Section>

        <Section title="use_table_header_placeholder">
            <p>
                "With column groups, a header row has empty cells above the columns that aren\u{2019}t in a group of that "
                "level (adjacent ones are merged). "<Code inline=true>"use_table_header_placeholder"</Code>" takes a "
                <Code inline=true>"UseTableHeaderPlaceholderInput { table, key }"</Code>" (the table data and the "
                "placeholder\u{2019}s key) and returns their props: "<Code inline=true>"role=\"gridcell\""</Code>", "<Code inline=true>"aria-colindex"</Code>
                ", and "<Code inline=true>"aria-colspan"</Code>" with "<Code inline=true>"colspan"</Code>" when they span "
                "several columns. Placeholders can\u{2019}t be focused."
            </p>
        </Section>

        <Section title="use_table_column_header">
            <p>
                "A column header (or column group header). It is focused like a cell: its first focusable child (the "
                "\u{201c}select all\u{201d} checkbox or a column resizer) gets focus, otherwise the header itself. Pressing the header of a "
                "sortable column sorts the table. While the table has no rows, the headers can\u{2019}t be focused."
            </p>

            <Section title="Input" id="use-table-column-header-input">
                <p>"Pass a "<Code inline=true>"UseTableColumnHeaderInput"</Code>" with every field named; the Default column gives the value for fields you don\u{2019}t need."</p>
                <ApiTable kind=ApiKind::Input of="UseTableColumnHeaderInput">
                    <ApiRow name="table" ty="TableData">"From "<Code inline=true>"use_table"</Code>"."</ApiRow>
                    <ApiRow name="key" ty="Key">"The column\u{2019}s key."</ApiRow>
                    <ApiRow name="allows_arrow_navigation" ty="bool" default="false">
                        "Let left/right move between the header\u{2019}s children even with "
                        <Code inline=true>"KeyboardNavigationBehavior::Tab"</Code>"."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return" id="use-table-column-header-return">
                <ApiTable kind=ApiKind::Return of="UseTableColumnHeaderReturn">
                    <ApiRow name="column_header_props" ty="PropsWithStyles<UseTableColumnHeaderProps>">
                        "For the "<Code inline=true>"<th>"</Code>": "<Code inline=true>"role=\"columnheader\""</Code>", "
                        <Code inline=true>"aria-colindex"</Code>", "<Code inline=true>"colspan"</Code>" for groups, "
                        <Code inline=true>"aria-sort"</Code>" ("<Code inline=true>"none"</Code>", "
                        <Code inline=true>"ascending"</Code>", "<Code inline=true>"descending"</Code>") on sortable columns, "
                        "focus and press handling."
                    </ApiRow>
                    <ApiRow name="is_pressed" ty="Signal<bool>">"Whether the header is being pressed."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example" id="use-table-column-header-example">
                <Code language=Language::Rust>
                    {indoc!(r"
                        let header = use_table_column_header(UseTableColumnHeaderInput {
                            table: data,
                            key: column,
                            allows_arrow_navigation: false,
                        });
                        let (attrs, styles) = header.column_header_props.into_parts();
                        view! { <th {..attrs} style=styles>{text}</th> }
                    ")}
                </Code>
            </Section>
        </Section>
    }
}

#[component]
fn TableBodySections() -> impl IntoView {
    view! {
        <Section title="use_table_row">
            <p>
                "A body row, labelled by its row header cells. Pressing it selects it (or runs "
                <Code inline=true>"on_row_action"</Code>")."
            </p>

            <Section title="Input" id="use-table-row-input">
                <ApiTable kind=ApiKind::Input of="UseTableRowInput">
                    <ApiRow name="table" ty="TableData">"From "<Code inline=true>"use_table"</Code>"."</ApiRow>
                    <ApiRow name="key" ty="Key">"The row\u{2019}s key in the collection."</ApiRow>
                    <ApiRow name="on_context_menu" ty="Option<Callback<ContextMenuEvent>>" default="None">
                        "Called when a context menu is requested on the row (right click, "<Keys keys="Shift + F10"/>", the context menu key, a long press on iOS); the row\u{2019}s menu then replaces the browser\u{2019}s."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return" id="use-table-row-return">
                <ApiTable kind=ApiKind::Return of="UseTableRowReturn">
                    <ApiRow name="row_props" ty="PropsWithStyles<UseTableRowProps>">
                        "For the "<Code inline=true>"<tr>"</Code>": "<Code inline=true>"role=\"row\""</Code>", "
                        <Code inline=true>"aria-selected"</Code>" (when the table allows selection), "
                        <Code inline=true>"aria-disabled"</Code>", "<Code inline=true>"aria-labelledby"</Code>
                        " (the row header cells), press and focus handling."
                    </ApiRow>
                    <ApiRow name="is_selected, is_focused, is_pressed" ty="Signal<bool>">"The row\u{2019}s state."</ApiRow>
                    <ApiRow name="is_disabled" ty="Signal<bool>">
                        "Whether the row is disabled for interaction. Only with "<Code inline=true>"DisabledBehavior::All"</Code>
                        "; with "<Code inline=true>"Selection"</Code>", check "
                        <Code inline=true>"allows_selection"</Code>"."
                    </ApiRow>
                    <ApiRow name="allows_selection, has_action" ty="Signal<bool>">"Whether the row can be selected, and whether it has an action."</ApiRow>
                    <ApiRow name="expand_button" ty="Option<UseButtonInput>">
                        "In a tree table: the expand button\u{2019}s configuration for "<Code inline=true>"use_button"</Code>
                        ", for the tree column\u{2019}s cell of rows with child rows (labelled \u{201c}Expand\u{201d} or "
                        "\u{201c}Collapse\u{201d} and the row)."
                    </ApiRow>
                    <ApiRow name="expand_button_attrs" ty="PreventFocusAttr">"Spread onto the expand button too: focus walks skip it."</ApiRow>
                    <ApiRow name="is_expanded, has_child_rows" ty="Signal<bool>">"Tree tables: whether the row\u{2019}s child rows are shown, and whether it has any."</ApiRow>
                    <ApiRow name="level" ty="Signal<Option<usize>>">"The row\u{2019}s level in a tree table, from 1 for top-level rows; "<Code inline=true>"None"</Code>" otherwise."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example" id="use-table-row-example">
                <Code language=Language::Rust>
                    {indoc!(r"
                        let row = use_table_row(UseTableRowInput { table: data.clone(), key: key.clone(), on_context_menu: None });
                        let (attrs, styles) = row.row_props.into_parts();
                        view! {
                            <tr {..attrs} style=styles>
                                <Cell table=data.clone() key=Key::cell(&key, 0)>{person.name}</Cell>
                                <Cell table=data.clone() key=Key::cell(&key, 1)>{person.email}</Cell>
                            </tr>
                        }
                    ")}
                </Code>
            </Section>

            <Section title="Tree Tables" id="use-table-row-tree-tables">
                <p>
                    "With "<Code inline=true>"UseTableStateInput.tree"</Code>" set, the state\u{2019}s "
                    <Code inline=true>"table"</Code>" holds every row and "<Code inline=true>"grid.list.collection"</Code>
                    " the visible ones: render the visible rows, or all of them with the others "<Code inline=true>"hidden"</Code>
                    ". The row props then carry "<Code inline=true>"aria-expanded"</Code>", "<Code inline=true>"aria-level"</Code>
                    ", "<Code inline=true>"aria-posinset"</Code>" and "<Code inline=true>"aria-setsize"</Code>", and handle "
                    <Keys keys="ArrowRight"/>" and "<Keys keys="ArrowLeft"/>" on a focused row (expand, collapse, to the parent). "
                    "In the tree column\u{2019}s cell ("<Code inline=true>"state.tree"</Code>"\u{2019}s "
                    <Code inline=true>"is_tree_column(&column)"</Code>"), render "<Code inline=true>"expand_button"</Code>
                    " with "<Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>" and spread "
                    <Code inline=true>"expand_button_attrs"</Code>" too; indent by "<Code inline=true>"level"</Code>". "
                    <Code inline=true>"Collection::cells(row)"</Code>" lists a row\u{2019}s cells. The "
                    <Link href=format!("{}#tree-tables", routes::doc::table::Atom.materialize())>"Table atoms"</Link>
                    " do all of this."
                </p>
            </Section>
        </Section>

        <Section title="use_table_cell">
            <p>
                "A body cell. Cells of row header columns get "<Code inline=true>"role=\"rowheader\""</Code>" and the id "
                "the row\u{2019}s "<Code inline=true>"aria-labelledby"</Code>" refers to. Selection and focus state belong "
                "to the row."
            </p>

            <Section title="Input" id="use-table-cell-input">
                <p>"Pass a "<Code inline=true>"UseTableCellInput"</Code>" with every field named; the Default column gives the value for fields you don\u{2019}t need."</p>
                <ApiTable kind=ApiKind::Input of="UseTableCellInput">
                    <ApiRow name="table" ty="TableData">"From "<Code inline=true>"use_table"</Code>"."</ApiRow>
                    <ApiRow name="key" ty="Key">"The cell\u{2019}s key, "<Code inline=true>"Key::cell(&row, column_index)"</Code>"."</ApiRow>
                    <ApiRow name="focus_mode" ty="Option<CellFocusMode>" default="None">
                        "Whether the cell or its first focusable child gets focus. "<Code inline=true>"None"</Code>": the "
                        "child with "<Code inline=true>"KeyboardNavigationBehavior::Arrow"</Code>", the cell with "
                        <Code inline=true>"Tab"</Code>"."
                    </ApiRow>
                    <ApiRow name="allows_arrow_navigation" ty="bool" default="false">"Let left/right move between the cell\u{2019}s children."</ApiRow>
                    <ApiRow name="should_select_on_press_up" ty="bool" default="false">"Select when the press ends."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return" id="use-table-cell-return">
                <ApiTable kind=ApiKind::Return of="UseTableCellReturn">
                    <ApiRow name="grid_cell_props" ty="PropsWithStyles<UseGridCellProps>">
                        "For the "<Code inline=true>"<td>"</Code>": "<Code inline=true>"role=\"gridcell\""</Code>" or "
                        <Code inline=true>"\"rowheader\""</Code>", the id of a row header cell, focus and press handling."
                    </ApiRow>
                    <ApiRow name="is_pressed" ty="Signal<bool>">"Whether the cell is being pressed."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example" id="use-table-cell-example">
                <Code language=Language::Rust>
                    {indoc!(r"
                        #[component]
                        fn Cell(table: TableData, key: Key, children: Children) -> impl IntoView {
                            let cell = use_table_cell(UseTableCellInput {
                                table,
                                key,
                                focus_mode: None,
                                allows_arrow_navigation: false,
                                should_select_on_press_up: false,
                            });
                            let (attrs, styles) = cell.grid_cell_props.into_parts();
                            view! { <td {..attrs} style=styles>{children()}</td> }
                        }
                    ")}
                </Code>
            </Section>
        </Section>

        <Section title="use_table_selection_checkbox">
            <p>
                "With "<Code inline=true>"show_selection_checkboxes"</Code>", cell 0 of every row holds a checkbox "
                "selecting the row. "<Code inline=true>"use_table_selection_checkbox"</Code>" takes a "
                <Code inline=true>"UseTableSelectionCheckboxInput { table, key }"</Code>" (the row\u{2019}s key) and returns a "
                <Code inline=true>"UseCheckboxInput"</Code>" for "<Link href=routes::doc::checkbox::Hook.materialize()>"use_checkbox"</Link>
                ": checked while the row is selected, disabled when it can\u{2019}t be selected, and labelled "
                "\u{201c}Select\u{201d} plus the row\u{2019}s row header cells (\u{201c}Select Dune\u{201d})."
            </p>
            <Code language=Language::Rust>
                {indoc!(r"
                    let checkbox = use_checkbox(use_table_selection_checkbox(UseTableSelectionCheckboxInput {
                        table: data.clone(),
                        key: key.clone(),
                    }));
                    let (attrs, styles) = checkbox.input_props.into_parts();
                    view! {
                        <Cell table=data.clone() key=Key::cell(&key, 0)>
                            <input {..attrs} style=styles/>
                        </Cell>
                    }
                ")}
            </Code>
        </Section>

        <Section title="use_table_select_all_checkbox">
            <p>
                <Code inline=true>"use_table_select_all_checkbox(UseTableSelectAllCheckboxInput { table })"</Code>" returns the "<Code inline=true>"UseCheckboxInput"</Code>
                " of the \u{201c}select all\u{201d} checkbox in the header of the checkbox column: checked when every "
                "selectable row is selected, indeterminate when some are, labelled \u{201c}Select All\u{201d}. It is "
                "disabled unless the table has rows and allows multiple selection; with single selection, leave the "
                "header empty."
            </p>
        </Section>
    }
}

#[component]
fn TableResizingSections() -> impl IntoView {
    view! {
        <Section title="Column Resizing">
            <p>
                "Resizable columns get their widths from the table, not from your CSS. "
                <Code inline=true>"use_table_column_resize_state"</Code>" computes a pixel width for every column from "
                "the width available to the table, and "<Code inline=true>"use_table_column_resize"</Code>" turns an "
                "element in a column header into a handle that changes it. Render the table with "
                <Code inline=true>"table-layout: fixed"</Code>" and give every column header its width; the cells "
                "follow. The "<Link href=format!("{}#column-resizing", routes::doc::table::Atom.materialize())>"Table Atoms"</Link>
                " do this for you inside a "<Code inline=true>"ResizableTableContainer"</Code>"."
            </p>
            <p>"Size the columns in the builder:"</p>
            <Code language=Language::Rust>
                {indoc!(r#"
                    t.column("name", "Name")
                        .allows_resizing()
                        .default_width(ColumnSize::Fr(2.0))
                        .min_width(ColumnBound::Px(140.0))
                        .max_width(ColumnBound::Percent(50.0));
                "#)}
            </Code>
            <DocTable headers=&["Builder method", "Default", "Sets"]>
                <TableRow>
                    <TableCell><Code inline=true>"allows_resizing()"</Code></TableCell>
                    <TableCell>"not resizable"</TableCell>
                    <TableCell>
                        "That the column gets a resizer. The hooks don\u{2019}t check it: render a resizer only in the "
                        "headers of columns with "<Code inline=true>"allows_resizing"</Code>"."
                    </TableCell>
                </TableRow>
                <TableRow>
                    <TableCell><Code inline=true>"default_width(ColumnSize)"</Code></TableCell>
                    <TableCell><Code inline=true>"Fr(1.0)"</Code></TableCell>
                    <TableCell>"The width before the user resizes the column."</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell><Code inline=true>"min_width(ColumnBound)"</Code></TableCell>
                    <TableCell><Code inline=true>"Px(75.0)"</Code></TableCell>
                    <TableCell>"The narrowest the column gets."</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell><Code inline=true>"max_width(ColumnBound)"</Code></TableCell>
                    <TableCell>"unbounded"</TableCell>
                    <TableCell>"The widest the column gets."</TableCell>
                </TableRow>
            </DocTable>
            <p>
                "A "<Code inline=true>"ColumnSize"</Code>" is "<Code inline=true>"Px(pixels)"</Code>", "
                <Code inline=true>"Percent(percent)"</Code>" of the table width, or "<Code inline=true>"Fr(share)"</Code>
                ": like the "<Code inline=true>"fr"</Code>" unit of CSS grids, the fractional columns share the space the "
                "pixel and percent columns leave, by their shares. A "<Code inline=true>"ColumnBound"</Code>" is "
                <Code inline=true>"Px"</Code>" or "<Code inline=true>"Percent"</Code>"."
            </p>
            <p>
                "Resizing a column gives it its new width in pixels, within its bounds. The columns before it keep their "
                "current pixel widths, the columns after it keep their sizes, so fractional columns after it share what "
                "is left. Columns never get narrower than their minimum: when they don\u{2019}t fit, the table gets wider "
                "than the available width, so let its container scroll. A resized column keeps its width when other "
                "columns are added or removed, and gets it back when it is removed and added again."
            </p>
            <p>
                "Drag the line at the end of a column header in the demo. With the keyboard, click a row, press "
                <Keys keys="ArrowUp"/>" to reach the column headers and move to Name, Kind or Modified with "
                <Keys keys="ArrowLeft"/>" / "<Keys keys="ArrowRight"/>": the resizer takes the focus. Press "<Keys keys="Enter"/>
                ", change the width with the arrow keys and press "<Keys keys="Enter"/>" again. Name is at least 140 "
                "pixels wide, Modified at most 40% of the table, and Size keeps its 90 pixels."
            </p>
            <Demo
                description="File table built with the table hooks: three resizable columns with minimum and maximum widths, a fixed one, and the current widths"
                source=include_str!("demos/table_resizing.rs")
            >
                <TableResizingHookDemo/>
            </Demo>
        </Section>

        <Section title="use_table_column_resize_state">
            <p>
                "Computes the column widths of a table and tracks which column is being resized. Without resizing, the "
                "columns keep their builder sizes."
            </p>

            <Section title="Input" id="use-table-column-resize-state-input">
                <p>
                    "Pass a "<Code inline=true>"UseTableColumnResizeStateInput"</Code>" with every field named; the Default "
                    "column gives the value for fields you don\u{2019}t need."
                </p>
                <ApiTable kind=ApiKind::Input of="UseTableColumnResizeStateInput">
                    <ApiRow name="table_state" ty="TableState">"From "<Code inline=true>"use_table_state"</Code>"."</ApiRow>
                    <ApiRow name="table_width" ty="Signal<f64>">
                        "The width available to the columns, in pixels: measure the table\u{2019}s scroll container, e.g. "
                        "with "<Code inline=true>"leptos_use::use_element_size"</Code>"."
                    </ApiRow>
                    <ApiRow name="default_width" ty="Option<Arc<DefaultWidth>>" default="None">
                        "A function "<Code inline=true>"Fn(&Column) -> Option<ColumnSize>"</Code>" giving the width of columns built without "<Code inline=true>"default_width"</Code>". When it is "
                        <Code inline=true>"None"</Code>" or returns "<Code inline=true>"None"</Code>": "
                        <Code inline=true>"ColumnSize::Fr(1.0)"</Code>"."
                    </ApiRow>
                    <ApiRow name="default_min_width" ty="Option<Arc<DefaultMinWidth>>" default="None">
                        "A function "<Code inline=true>"Fn(&Column) -> Option<ColumnBound>"</Code>" giving the minimum width of columns built without "<Code inline=true>"min_width"</Code>". When it is "
                        <Code inline=true>"None"</Code>": "<Code inline=true>"DEFAULT_MIN_WIDTH"</Code>" ("
                        <Code inline=true>"ColumnBound::Px(75.0)"</Code>"); when it returns "<Code inline=true>"None"</Code>
                        ": no minimum."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return" id="use-table-column-resize-state-return">
                <p>"A "<Code inline=true>"TableColumnResizeState"</Code>" (it is "<Code inline=true>"Copy"</Code>"):"</p>
                <ApiTable kind=ApiKind::Return of="TableColumnResizeState">
                    <ApiRow name="table_state" ty="TableState">"The table state it was created with."</ApiRow>
                    <ApiRow name="column_widths" ty="Signal<ColumnWidths>">
                        "The pixel widths and bounds of the data columns: "<Code inline=true>"width(&key)"</Code>", "
                        <Code inline=true>"min_width(&key)"</Code>", "<Code inline=true>"max_width(&key)"</Code>", "
                        <Code inline=true>"widths()"</Code>". Widths are whole pixels."
                    </ApiRow>
                    <ApiRow name="resizing_column" ty="Signal<Option<Key>>">"The column being resized."</ApiRow>
                </ApiTable>

                <p>"Its methods:"</p>

                <DocTable headers=&["Method", "Description"]>
                    <TableRow>
                        <TableCell><Code inline=true>"column_width(&Key), column_min_width(&Key), column_max_width(&Key) -> f64"</Code></TableCell>
                        <TableCell>
                            "Read "<Code inline=true>"column_widths"</Code>" for one column (tracked). A column without maximum "
                            "reports "<Code inline=true>"UNBOUNDED_WIDTH"</Code>" (2\u{2075}\u{00b3} \u{2212} 1)."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"start_resize(Key), end_resize()"</Code></TableCell>
                        <TableCell>
                            "Mark a column as being resized, or no column. Starting it from elsewhere (e.g. a column menu) "
                            "makes the column\u{2019}s resizer focus its input and resize with the keyboard."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"update_resized_columns(&Key, f64) -> HashMap<Key, ColumnSize>"</Code></TableCell>
                        <TableCell>
                            "Resize a column to the given width in pixels (within its bounds) and return the new size of "
                            "every column. The resizer calls it; call it yourself to set a width from code."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Example" id="use-table-column-resize-state-example">
                <Code language=Language::Rust>
                    {indoc!(r"
                        let container = NodeRef::<html::Div>::new();
                        let width = use_element_size(container).width;
                        let resize = use_table_column_resize_state(UseTableColumnResizeStateInput {
                            table_state: state,
                            table_width: width,
                            default_width: None,
                            default_min_width: None,
                        });

                        // In a column header: its width, from the state (`pixels` makes it a CSS `Size`, see the demo).
                        let header = use_table_column_header(UseTableColumnHeaderInput {
                            table: data,
                            key: column.clone(),
                            allows_arrow_navigation: false,
                        });
                        let (attrs, styles) = header.column_header_props.into_parts();
                        let width = move || WidthProperty.declare(pixels(resize.column_width(&column)));
                        view! { <th {..attrs} style=styles.add_reactive(width)>{text}</th> }
                    ")}
                </Code>
                <p>
                    "The container needs "<Code inline=true>"overflow: auto"</Code>", the table "
                    <Code inline=true>"table-layout: fixed"</Code>" and "<Code inline=true>"width: min-content"</Code>
                    ", so that it is exactly as wide as its columns."
                </p>
            </Section>
        </Section>

        <Section title="use_table_column_resize">
            <p>
                "The behavior of a column resizer: drag it with a pointer, or focus its visually hidden range input, "
                "press "<Keys keys="Enter"/>" and resize with the arrow keys. Render the resizer element inside the column "
                "header and the input inside the resizer. The input is the header\u{2019}s first focusable child, so "
                "moving onto the header with the arrow keys focuses it. While a column is resized, the table ignores "
                "the arrow keys. You give the resizer its label; its other texts follow the locale: the value "
                "\u{201c}120 pixels\u{201d} and the description \u{201c}Press Enter to start resizing\u{201d} in English."
            </p>

            <Section title="Input" id="use-table-column-resize-input">
                <p>
                    "Pass a "<Code inline=true>"UseTableColumnResizeInput"</Code>" with every field named; the Default column "
                    "gives the value for fields you don\u{2019}t need."
                </p>
                <ApiTable kind=ApiKind::Input of="UseTableColumnResizeInput">
                    <ApiRow name="state" ty="TableColumnResizeState">"From "<Code inline=true>"use_table_column_resize_state"</Code>"."</ApiRow>
                    <ApiRow name="table" ty="TableData">"From "<Code inline=true>"use_table"</Code>". The input is labelled by the column header too."</ApiRow>
                    <ApiRow name="column" ty="Key">"The column this resizer resizes."</ApiRow>
                    <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                        "The input\u{2019}s label, e.g. \u{201c}Resizer\u{201d}. Its "<Code inline=true>"aria-labelledby"</Code>" adds the column "
                        "header: \u{201c}Resizer Name\u{201d}."
                    </ApiRow>
                    <ApiRow name="element" ty="CapturedElement">"The range input. The input props capture it."</ApiRow>
                    <ApiRow name="trigger" ty="Option<CapturedElement>" default="None">
                        "The element resizing starts from, e.g. the column header. It gets the focus back when resizing "
                        "ends, unless the input had it when resizing started. Without a trigger, keyboard and screen reader "
                        "users hear \u{201c}Press Enter to start resizing\u{201d} on the input."
                    </ApiRow>
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">
                        "Disables the input. Pointer resizing still works: don\u{2019}t render a resizer for a column "
                        "that can\u{2019}t be resized."
                    </ApiRow>
                    <ApiRow name="on_resize_start, on_resize, on_resize_end" ty="Option<Callback<HashMap<Key, ColumnSize>>>" default="None">
                        "Called with the size of every column when resizing starts, whenever the column is resized, and "
                        "when resizing ends. Columns before the resized one are reported in pixels."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return" id="use-table-column-resize-return">
                <ApiTable kind=ApiKind::Return of="UseTableColumnResizeReturn">
                    <ApiRow name="resizer_props" ty="PropsWithStyles<UseTableColumnResizerProps>">
                        "For the resizer element (give it "<Code inline=true>"role=\"presentation\""</Code>"): pointer, "
                        "press and keyboard handling, and "<Code inline=true>"touch-action: none"</Code>"."
                    </ApiRow>
                    <ApiRow name="input_props" ty="PropsWithStyles<UseTableColumnResizeInputProps>">
                        "For the "<Code inline=true>"<input>"</Code>": "<Code inline=true>"type=\"range\""</Code>" with the "
                        "column\u{2019}s width as "<Code inline=true>"value"</Code>" and its bounds as "
                        <Code inline=true>"min"</Code>" / "<Code inline=true>"max"</Code>" (whole pixels), "
                        <Code inline=true>"aria-valuetext"</Code>" (\u{201c}120 pixels\u{201d}), label, "
                        <Code inline=true>"aria-orientation=\"horizontal\""</Code>", visually hidden styles."
                    </ApiRow>
                    <ApiRow name="is_resizing" ty="Signal<bool>">"Whether the column is being resized."</ApiRow>
                    <ApiRow name="is_mouse_resizing" ty="Signal<bool>">
                        "Whether it is being dragged with a mouse, e.g. to show the resize cursor on the whole page."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example" id="use-table-column-resize-example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        let resizer = use_table_column_resize(UseTableColumnResizeInput {
                            state: resize,
                            table: data,
                            column,
                            aria_label: "Resizer".into(),
                            element: CapturedElement::new(),
                            trigger: None,
                            is_disabled: Signal::stored(false),
                            on_resize_start: None,
                            on_resize: None,
                            on_resize_end: None,
                        });

                        let (attrs, styles) = resizer.resizer_props.into_parts();
                        let (input_attrs, input_styles) = resizer.input_props.into_parts();
                        view! {
                            <div role="presentation" {..attrs} style=styles class="resizer">
                                <input {..input_attrs} style=input_styles/>
                            </div>
                        }
                    "#)}
                </Code>
                <p>
                    "Position the resizer at the header\u{2019}s end edge, e.g. with "
                    <Code inline=true>"position: absolute; right: 0; top: 0; height: 100%; width: 8px; cursor: col-resize"</Code>
                    " in a header with "<Code inline=true>"position: relative"</Code>"."
                </p>
            </Section>
        </Section>
    }
}

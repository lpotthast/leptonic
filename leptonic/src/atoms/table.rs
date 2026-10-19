// Upstream: react-aria-components/src/Table.tsx @ 99e6102368
// Upstream: react-aria-components/test/Table.test.js @ 99e6102368
// Upstream: react-aria-components/test/Table.ssr.test.js @ 99e6102368
// Upstream: react-aria-components/test/Treeble.test.js @ 99e6102368
// Upstream: react-aria/test/table/tableResizingTests.tsx @ 99e6102368
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use leptos::{context::Provider, either::Either, ev, html, prelude::*};
use leptos_classes::Classes;
use leptos_use::use_resize_observer;

use crate::{
    CapturedElement, IntoAttrs, Out, ValueBinding,
    hooks::{
        button::{UseButtonInput, use_button},
        collections::{
            CollectionOptions, DisabledBehavior, EscapeKeyBehavior, Key, NodeKind, Selection,
            SelectionBehavior, SelectionMode, SelectionOptions,
        },
        focus::{
            FocusRingTarget, UseFocusRingInput, UseFocusRingReturn, is_focus_visible,
            use_focus_ring,
        },
        form::use_checkbox,
        grid::{CellFocusMode, GridFocusMode, use_grid_row_group},
        gridlist::KeyboardNavigationBehavior,
        interactions::{UseHoverInput, use_hover},
        table::{
            ColumnKind, ColumnSize, SortDescriptor, SortDirection, TableCollection,
            TableColumnResizeState, TableData, TableTreeInput, UseTableCellInput,
            UseTableCellReturn, UseTableColumnHeaderInput, UseTableColumnHeaderReturn,
            UseTableColumnResizeInput, UseTableColumnResizeReturn, UseTableColumnResizeStateInput,
            UseTableHeaderPlaceholderInput, UseTableInput, UseTableReturn, UseTableRowInput,
            UseTableRowReturn, UseTableSelectAllCheckboxInput, UseTableSelectionCheckboxInput,
            UseTableStateInput, use_table, use_table_cell, use_table_column_header,
            use_table_column_resize, use_table_column_resize_state, use_table_header_placeholder,
            use_table_header_row, use_table_row, use_table_select_all_checkbox,
            use_table_selection_checkbox, use_table_state,
        },
    },
    sharing_one_effect,
    utils::{
        data_attributes::{self, data_flags},
        default_class::with_default_class,
        i18n::WritingDirection,
        intl_strings::{AtomStrings, use_localized_strings},
        scoped_context::scoped_view,
        signal_attr::{maybe_attr, signal_attr},
        styles::{
            Styles,
            css::{Size, computed_px, computed_size},
            property::WidthProperty,
        },
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Columns and rows come from a `TableCollection`; `TableHeader` renders the column headers
//   itself, `TableRow` the selection checkbox cell (`show_selection_checkboxes`).
//
// ## DIFFERENT BEHAVIOR
// - A column resizer is labelled by "Resizer" and the column's name element, not the whole column
//   header (which contains the resizer: browsers would name it "Resizer Name Resizer").
//
// ## OMITTED FEATURES
// - Custom column header content (and so a column header inside a `TooltipTrigger`),
//   virtualization, drag and drop, `renderEmptyState`, load more.
//
// =============================================================================

/// The size of each column, by column key, as [`ResizableTableContainer`]'s resize callbacks
/// report them.
pub type ColumnSizes = HashMap<Key, ColumnSize>;

/// What a [`ResizableTableContainer`] tells the [`Table`] inside it.
#[derive(Debug, Clone, Copy)]
struct ResizableTableContainerContext {
    table_width: Signal<f64>,
    on_resize_start: Option<Callback<ColumnSizes>>,
    on_resize: Option<Callback<ColumnSizes>>,
    on_resize_end: Option<Callback<ColumnSizes>>,
}

/// The column widths of a [`Table`] in a [`ResizableTableContainer`], for its column headers.
#[derive(Debug, Clone, Copy)]
struct ColumnResizeContext {
    state: TableColumnResizeState,
    container: ResizableTableContainerContext,
}

/// Makes the columns of the [`Table`] inside it resizable: the columns share the container's
/// width (by their `default_width`, `min_width` and `max_width`), and the headers of columns
/// built with `allows_resizing()` get a resizer.
///
/// A resizer is a `<div data-column-resizer>` at the end of its column header (position it,
/// e.g. at the header's right edge, with `cursor: col-resize`). It exposes `data-resizing` and
/// `data-resizable-direction` for styling, as react-aria-components does: `left` while the column
/// is at its minimum width (style e.g. `cursor: e-resize`), `right` at its maximum width
/// (`w-resize`), mirrored in right-to-left layouts, and `both` otherwise. The header exposes
/// `data-resizing` too. Drag the resizer, or focus it by keyboard (arrow onto its column header:
/// a resizable header hands the focus to its resizer, as in react-aria-components) and press
/// Enter, resize with the arrow keys, and press Enter, Escape or Tab to finish.
///
/// The container measures its own width; let it scroll (`overflow: auto`) for tables wider than
/// it.
///
/// Default class: `leptonic-ResizableTableContainer`.
#[component]
pub fn ResizableTableContainer(
    /// Called with the column sizes when a resizing starts.
    #[prop(into, optional)]
    on_resize_start: Option<Callback<ColumnSizes>>,
    /// Called with the column sizes whenever a column is resized.
    #[prop(into, optional)]
    on_resize: Option<Callback<ColumnSizes>>,
    /// Called with the column sizes when a resizing ends.
    #[prop(into, optional)]
    on_resize_end: Option<Callback<ColumnSizes>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    crate::metrics::subject!();
    let classes = with_default_class("leptonic-ResizableTableContainer", classes);
    let container = NodeRef::<html::Div>::new();
    let (width, set_width) = signal(0.0);
    let measure = move || {
        if let Some(container) = container.get_untracked() {
            set_width.set(f64::from(container.client_width()));
        }
    };
    use_resize_observer(container, move |_, _| measure());
    Effect::new(move |_| {
        if container.get().is_some() {
            measure();
        }
    });
    let context = ResizableTableContainerContext {
        table_width: width.into(),
        on_resize_start,
        on_resize,
        on_resize_end,
    };
    view! {
        <Provider value=context>
            <div node_ref=container class=classes style=styles>
                {children()}
            </div>
        </Provider>
    }
}

/// A headless table: column headers above rows of cells, navigated in two dimensions with the
/// arrow keys, with sortable columns and selectable rows.
///
/// The columns and rows come from `table` (built with `TableCollection::build`): render a
/// [`TableHeader`] (it renders the column headers itself) and a [`TableBody`] with one
/// [`TableRow`] per row, holding one [`TableCell`] per data column.
///
/// A tree table (`tree_column`): rows with child rows (`ItemBuilder::children`) expand and
/// collapse. Render every row, child rows after their parent (in collection order); rows under a
/// collapsed row are built once shown, and `hidden` while collapsed. Put a [`TableExpandButton`]
/// into the tree column's cells. Rows and cells carry `data-expanded` and `data-has-child-items`,
/// tree column cells `data-tree-column`. Rows and cells of every table carry `data-level` (1 for
/// top-level rows), rows the `--table-row-level` style (for indenting).
///
/// Data attributes: `data-focused`, `data-focus-visible` (focus on the table element itself).
///
/// Default class: `leptonic-Table`.
#[component]
#[allow(clippy::implicit_hasher)]
pub fn Table(
    /// The columns and rows.
    #[prop(into)]
    table: Memo<Arc<TableCollection>>,
    /// The table's id (its rows' and cells' ids derive from it). Generated by default.
    #[prop(into, optional)]
    id: Option<crate::Id>,

    /// Add a first column of checkboxes selecting the rows (and all rows, in its header), while
    /// `selection_mode` isn't `None`.
    #[prop(into, optional)]
    show_selection_checkboxes: Signal<bool>,
    /// Whether arrow keys focus rows (and then cells) or only cells.
    #[prop(optional)]
    focus_mode: GridFocusMode,
    #[prop(into, optional)] selection_mode: Signal<SelectionMode>,
    #[prop(into, optional)] selection_behavior: Signal<SelectionBehavior>,
    /// The initially selected rows.
    #[prop(into, optional)]
    default_selection: Selection,
    /// The selection (controlled), replacing `default_selection`: a value or any signal.
    #[prop(into, optional)]
    selection: Option<Signal<Selection>>,
    /// Receives the new state: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_selection: Option<Out<Selection>>,
    #[prop(into, optional)] on_selection_change: Option<Callback<Selection>>,
    #[prop(into, optional)] disabled_keys: Option<Signal<HashSet<Key>>>,
    /// Defaults to `DisabledBehavior::Selection`: disabled rows can be focused, not selected.
    #[prop(default = DisabledBehavior::Selection)]
    disabled_behavior: DisabledBehavior,
    #[prop(into, optional)] disallow_empty_selection: Signal<bool>,
    #[prop(optional)] escape_key_behavior: EscapeKeyBehavior,
    /// The initial sorting. Ignored with `sort_descriptor`.
    #[prop(optional)]
    default_sort_descriptor: Option<SortDescriptor>,
    /// The sorting (controlled): a value or any signal. `None` shows the table unsorted.
    #[prop(into, optional)]
    sort_descriptor: Option<Signal<Option<SortDescriptor>>>,
    /// Receives the new sorting: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_sort_descriptor: Option<Out<Option<SortDescriptor>>>,
    /// Called when the user sorts the table. Sort the rows accordingly.
    #[prop(into, optional)]
    on_sort_descriptor_change: Option<Callback<Option<SortDescriptor>>>,
    #[prop(optional)] keyboard_navigation_behavior: KeyboardNavigationBehavior,
    /// Select rows when a press ends instead of when it starts (e.g. for draggable rows).
    #[prop(optional)]
    should_select_on_press_up: bool,
    /// Called with the key of an activated row.
    #[prop(into, optional)]
    on_row_action: Option<Callback<Key>>,
    /// Called with the key of an activated cell.
    #[prop(into, optional)]
    on_cell_action: Option<Callback<Key>>,
    /// Makes the table a tree table: the column showing the hierarchy (react-aria-components'
    /// `treeColumn`).
    #[prop(into, optional)]
    tree_column: Option<Key>,
    /// The initially expanded rows of a tree table. Ignored with `expanded_keys`.
    #[prop(optional)]
    default_expanded_keys: HashSet<Key>,
    /// The expanded rows (controlled): a value or any signal.
    #[prop(into, optional)]
    expanded_keys: Option<Signal<HashSet<Key>>>,
    /// Receives the expanded rows: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_expanded_keys: Option<Out<HashSet<Key>>>,
    #[prop(into, optional)] on_expanded_change: Option<Callback<HashSet<Key>>>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    /// Ids of elements labelling it.
    #[prop(into, optional)]
    aria_labelledby: MaybeProp<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    crate::metrics::subject!();
    let classes = with_default_class("leptonic-Table", classes);
    let tree = tree_column.map(|column| {
        let (expanded_keys, on_expanded_change) =
            ValueBinding::from_state_props(expanded_keys, set_expanded_keys, on_expanded_change);
        TableTreeInput {
            column,
            default_expanded_keys,
            expanded_keys,
            on_expanded_change,
        }
    });
    let (sort_descriptor, on_sort_descriptor_change) = ValueBinding::from_state_props(
        sort_descriptor,
        set_sort_descriptor,
        on_sort_descriptor_change,
    );
    let (selection, on_selection_change) =
        ValueBinding::from_state_props(selection, set_selection, on_selection_change);
    let state = use_table_state(UseTableStateInput {
        tree,
        selection: SelectionOptions {
            selection_mode,
            selection_behavior,
            default_selection,
            selection,
            on_selection_change,
            disallow_empty_selection,
            disabled_keys: disabled_keys.unwrap_or_default(),
            disabled_behavior,
            ..SelectionOptions::default()
        },
        focus_mode,
        default_sort_descriptor,
        sort_descriptor,
        // The user's sorting is never `None`.
        on_sort_change: on_sort_descriptor_change
            .map(|on_change| Callback::new(move |sort| on_change.run(Some(sort)))),
        table,
        show_selection_checkboxes,
    });

    let UseTableReturn { table: props, data } = use_table(UseTableInput {
        aria_label,
        aria_labelledby: crate::utils::id::id_ref_prop(aria_labelledby),
        options: CollectionOptions {
            escape_key_behavior,
            ..CollectionOptions::default()
        },
        keyboard_navigation_behavior,
        on_row_action,
        on_cell_action,
        state,
        element: CapturedElement::new(),
        id,
        keyboard_delegate: None,
        should_select_on_press_up,
    });

    // In a resizable table container: fixed column widths.
    let column_resize = use_context::<ResizableTableContainerContext>().map(|container| {
        let state = use_table_column_resize_state(UseTableColumnResizeStateInput {
            table_state: state,
            table_width: container.table_width,
            default_width: None,
            default_min_width: None,
        });
        ColumnResizeContext { state, container }
    });
    let styles = if column_resize.is_some() {
        styles
            .add_unchecked("table-layout", "fixed")
            .add(WidthProperty.declare(Size::MinContent))
    } else {
        styles
    };

    // The selection column, read by every row: one memo per table. It follows the columns (the
    // selection column can come and go), not the rows.
    let columns = data.state.columns;
    let selection_column = SelectionColumn(Memo::new(move |_| {
        columns.with(|columns| {
            columns
                .first()
                .filter(|c| c.kind == ColumnKind::SelectionCheckbox)
                .map(|c| c.key.clone())
        })
    }));

    // Focus on the table element itself (react-aria-components' `Table`).
    let focus_ring = use_focus_ring(UseFocusRingInput::default());
    scoped_view(
        move || {
            provide_context(data);
            provide_context(selection_column);
            if let Some(column_resize) = column_resize {
                provide_context(column_resize);
            }
        },
        move || {
            view! {
                <table
                    {..props.attrs()}
                    {..focus_ring.target.attrs()}
                    class=classes
                    style={styles}
                    {..data_flags([("data-focused", focus_ring.is_focused)])}
                >
                    {children()}
                </table>
            }
        },
    )
}

/// The header of a [`Table`]: its header rows with the column headers (and, for a selection
/// checkbox column, a "select all" checkbox in multiple selection mode).
///
/// Data attributes: `data-hovered`. On the column headers (default class
/// `leptonic-TableColumnHeader`): `data-allows-sorting`, `data-sort-direction` (`ascending` /
/// `descending`), `data-focused`, `data-focus-visible`, `data-hovered` (sortable columns),
/// `data-pressed`, `data-resizing` (see [`ResizableTableContainer`]).
///
/// Default class: `leptonic-TableHeader`.
#[component]
pub fn TableHeader(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    crate::metrics::subject!();
    let classes = with_default_class("leptonic-TableHeader", classes);
    let Some(data) = use_context::<TableData>() else {
        crate::utils::dev_warn!("a <TableHeader> belongs in a <Table>");
        return None;
    };
    let row_group = use_grid_row_group();
    let rows = data.state.table;
    // react-aria-components' `TableHeader` reports hover.
    let hover = use_hover(UseHoverInput::default());
    Some(view! {
        <thead
            {..row_group.row_group.attrs()}
            {..hover.target.attrs()}
            class=classes
            style={styles}
            {..data_flags([("data-hovered", hover.is_hovered)])}
        >
            <For
                each=move || rows.with(|t| t.header_rows().to_vec())
                key=Clone::clone
                children=move |row: Key| {
                    let cells = move || {
                        rows.with(|t| {
                            t.collection()
                                .children(&row)
                                .map(|n| (n.key.clone(), n.kind))
                                .collect::<Vec<_>>()
                        })
                    };
                    view! {
                        <tr {..use_table_header_row().into_props().into_attrs()}>
                            <For
                                each=cells
                                key=|(key, _)| key.clone()
                                children=move |(key, kind): (Key, NodeKind)| {
                                    if kind == NodeKind::Placeholder {
                                        let data = expect_context::<TableData>();
                                        let placeholder = use_table_header_placeholder(UseTableHeaderPlaceholderInput {
                                            table: data,
                                            key,
                                        }).into_props();
                                        view! { <th {..placeholder.into_attrs()}></th> }.into_any()
                                    } else {
                                        view! { <TableColumnHeader key /> }.into_any()
                                    }
                                }
                            />
                        </tr>
                    }
                }
            />
        </thead>
    })
}

/// A column header of a [`Table`], rendered by [`TableHeader`].
///
/// Default class: `leptonic-TableColumnHeader`.
#[component]
fn TableColumnHeader(key: Key) -> impl IntoView {
    let data = expect_context::<TableData>();
    let state = data.state;
    // The column follows the table (e.g. when columns change). A header whose column is gone is
    // about to be removed: it keeps its values meanwhile (no re-render of a leaving header, which
    // the columns, notified before the header rows, would cause first).
    let column = {
        let key = key.clone();
        Memo::new(
            move |previous: Option<&(Arc<str>, ColumnKind, bool, bool)>| {
                state
                    .columns
                    .with(|columns| {
                        columns.iter().find(|c| c.key == key).map(|c| {
                            (
                                c.text_value.clone(),
                                c.kind,
                                c.allows_sorting,
                                c.allows_resizing,
                            )
                        })
                    })
                    .or_else(|| previous.cloned())
                    // Built only for a missing column (an `Arc<str>` allocates).
                    .unwrap_or_else(|| (Arc::from(""), ColumnKind::Data, false, false))
            },
        )
    };
    let allows_sorting = Memo::new(move |_| column.with(|c| c.2));
    let allows_resizing = Memo::new(move |_| column.with(|c| c.3));
    let sort_key = key.clone();
    let sort_direction = move || {
        state.sort_descriptor.with(|sort| match sort {
            Some(sort) if sort.column == sort_key => Some(match sort.direction {
                SortDirection::Ascending => "ascending",
                SortDirection::Descending => "descending",
            }),
            _ => None,
        })
    };
    let selection = state.grid.list.selection;
    let focus_key = key.clone();
    let is_focused =
        Signal::derive(move || selection.is_focused_key(&focus_key) && selection.is_focused());
    let is_focus_visible = Signal::derive(move || is_focused.get() && is_focus_visible());
    // Sortable headers show hover (react-aria-components' `Column`).
    let hover = use_hover(UseHoverInput {
        is_disabled: Signal::derive(move || !allows_sorting.get()),
        on_hover_start: None,
        on_hover_end: None,
        on_hover_change: None,
    });
    let is_selection_column =
        Memo::new(move |_| column.with(|c| c.1 == ColumnKind::SelectionCheckbox));
    // In multiple selection mode: the selection column exists only with a selection mode, so
    // "not single" is "multiple" wherever the header is shown; while a change to `None` removes
    // the header, it keeps its checkbox (instead of removing it just before the header).
    let shows_select_all = Memo::new(move |_| {
        is_selection_column.get() && selection.selection_mode() != SelectionMode::Single
    });
    let select_all_table = data.clone();
    // The column's name, in an element of its own: the resizer is labelled by it (not by the
    // whole header, which contains the resizer itself).
    let text_id = crate::utils::id::use_id("table-column-name");
    let name_id = text_id.clone();
    let content = move || {
        if is_selection_column.get() {
            shows_select_all
                .get()
                // Untracked: built again only when it is shown or hidden.
                .then(|| {
                    untrack(|| {
                        let checkbox = use_checkbox(use_table_select_all_checkbox(
                            UseTableSelectAllCheckboxInput {
                                table: select_all_table.clone(),
                            },
                        ));
                        checkbox.input.props().map(|input| {
                            let (attrs, styles) = input.into_parts();
                            view! { <input {..attrs} style=styles /> }
                        })
                    })
                })
                .into_any()
        } else {
            view! { <span id=name_id.clone()>{move || column.with(|c| c.0.to_string())}</span> }
                .into_any()
        }
    };
    // With column resizing: the column's width, and its resizer.
    let resize = use_context::<ColumnResizeContext>();
    let header = CapturedElement::new();
    let resizer_key = key.clone();
    let resizer = move || {
        resize.filter(|_| allows_resizing.get()).map(|resize| {
            view! {
                <ColumnResizer
                    resize
                    column=resizer_key.clone()
                    trigger=header
                    name_id=text_id.clone()
                />
            }
        })
    };
    let resizing_key = key.clone();
    let is_resizing = Signal::derive(move || {
        resize.is_some_and(|r| {
            r.state
                .resizing_column
                .with(|c| c.as_ref() == Some(&resizing_key))
        })
    });
    let width_key = key.clone();
    let UseTableColumnHeaderReturn {
        column_header: column_header_props,
        is_pressed,
    } = use_table_column_header(UseTableColumnHeaderInput {
        table: data,
        key,
        allows_arrow_navigation: false,
    });
    let (attrs, styles) = column_header_props.attrs();
    let styles = match resize {
        Some(resize) => styles.add_reactive(move || {
            WidthProperty.declare(computed_size(computed_px(
                resize.state.column_width(&width_key),
            )))
        }),
        None => styles,
    };

    view! {
        <th
            {..attrs}
            {..header.attr()}
            {..hover.target.attrs()}
            class="leptonic-TableColumnHeader"
            style={styles}
            {..data_flags([("data-allows-sorting", allows_sorting.into())])}
            data-sort-direction={sort_direction}
            {..data_flags([
                ("data-focused", is_focused),
                ("data-focus-visible", is_focus_visible),
                ("data-hovered", hover.is_hovered),
            ])}
            {..data_flags([
                ("data-pressed", is_pressed),
                ("data-resizing", is_resizing),
            ])}
        >
            {content}
            {resizer}
        </th>
    }
}

/// The resizer of a resizable column, rendered by [`TableColumnHeader`] (see
/// [`ResizableTableContainer`]).
///
/// Default class: `leptonic-ColumnResizer`.
#[component]
fn ColumnResizer(
    resize: ColumnResizeContext,
    column: Key,
    trigger: CapturedElement,
    /// The id of the element holding the column's name.
    name_id: crate::Id,
) -> impl IntoView {
    let data = expect_context::<TableData>();
    let direction = data.grid.direction;
    let state = resize.state;
    let strings = use_localized_strings::<AtomStrings>();
    let UseTableColumnResizeReturn {
        resizer: resizer_props,
        input: input_props,
        is_resizing,
        ..
    } = use_table_column_resize(UseTableColumnResizeInput {
        trigger: Some(trigger),
        on_resize_start: resize.container.on_resize_start,
        on_resize: resize.container.on_resize,
        on_resize_end: resize.container.on_resize_end,
        state,
        table: data,
        column: column.clone(),
        aria_label: Signal::derive(move || Some(strings.read().table_resizer())).into(),
        element: CapturedElement::new(),
        is_disabled: Signal::stored(false),
    });
    let input_props = input_props.into_props();
    // `left` at the minimum width, `right` at the maximum (react-aria-components' values).
    let resizable_direction = move || {
        let (width, min, max) = state
            .column_widths
            .with(|w| (w.width(&column), w.min_width(&column), w.max_width(&column)));
        let rtl = direction.get() == WritingDirection::Rtl;
        if min >= width {
            if rtl { "right" } else { "left" }
        } else if max <= width {
            if rtl { "left" } else { "right" }
        } else {
            "both"
        }
    };
    let (attrs, styles) = resizer_props.attrs();
    // Labelled "Resizer" plus the column's name (react-aria: plus the column header, which
    // contains the resizer, so browsers name it "Resizer Name Resizer").
    let (mut input_props, input_styles) = input_props.into_inner();
    input_props.aria_labelledby =
        crate::utils::id::join_ids([Some(input_props.id.clone()), Some(name_id.clone())]);
    let input_attrs = input_props.into_attrs();
    view! {
        <div
            role="presentation"
            {..attrs}
            class="leptonic-ColumnResizer"
            style=styles
            data-column-resizer="true"
            {..data_flags([("data-resizing", is_resizing)])}
            data-resizable-direction=resizable_direction
        >
            <input {..input_attrs} style=input_styles />
        </div>
    }
}

/// The body of a [`Table`]: its rows (none for an empty table).
///
/// Data attributes: `data-empty` (the table has no rows).
///
/// Default class: `leptonic-TableBody`.
#[component]
pub fn TableBody(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    crate::metrics::subject!();
    let classes = with_default_class("leptonic-TableBody", classes);
    let row_group = use_grid_row_group();
    // Notified only when the table becomes empty or stops being empty.
    let is_empty = use_context::<TableData>()
        .map_or_else(|| Signal::stored(false), |data| data.state.grid.is_empty());
    view! {
        <tbody
            {..row_group.row_group.attrs()}
            class=classes
            style={styles}
            {..data_flags([("data-empty", is_empty)])}
        >
            {children.map(|children| children())}
        </tbody>
    }
}

/// The key of a [`Table`]'s selection checkbox column, while it has one.
#[derive(Debug, Clone, Copy)]
struct SelectionColumn(Memo<Option<Key>>);

/// The row a [`TableCell`] is in.
#[derive(Debug, Clone)]
struct RowContext {
    key: Key,
    /// A tree table's expand button (for [`TableExpandButton`]), whether the row has child rows
    /// and whether they are shown.
    expand_button: StoredValue<Option<UseButtonInput>>,
    has_child_rows: Signal<bool>,
    is_expanded: Signal<bool>,
    level: Signal<usize>,
    is_selected: Signal<bool>,
    is_disabled: Signal<bool>,
    /// Keyboard focus on the row or inside it.
    is_focus_visible_within: Signal<bool>,
}

/// A row of a [`Table`], for the collection row `key`. With a selection checkbox column, it
/// renders the selection cell itself; add one [`TableCell`] per data column.
///
/// Data attributes: `data-selected`, `data-disabled`, `data-hovered` (rows that can be selected or
/// have an action), `data-focused`, `data-focus-visible`, `data-focus-visible-within` (keyboard
/// focus on the row or inside it), `data-pressed`, `data-selection-mode` (`single`/`multiple`,
/// absent without selection), `data-level` (the row's level, 1 for top-level rows), and in a tree
/// table `data-expanded` and `data-has-child-items`. Style: `--table-row-level` (the level, for
/// indenting).
///
/// In a tree table, a row under a collapsed row is built (its hooks and children) only once it is
/// shown, and `hidden` while collapsed again.
///
/// Default class: `leptonic-TableRow`.
#[component]
pub fn TableRow(
    /// The row's key in the table's collection.
    #[prop(into)]
    key: Key,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    crate::metrics::subject!();
    let classes = with_default_class("leptonic-TableRow", classes);
    let Some(data) = use_context::<TableData>() else {
        crate::utils::dev_warn!("a <TableRow> belongs in a <Table>");
        return None;
    };
    if data.state.tree.is_none() {
        return Some(Either::Left(table_row(
            data, key, classes, styles, children,
        )));
    }
    // A tree table: build the row the first time it is shown (react-aria-components renders only
    // the rows of expanded rows), then keep it.
    let rows = data.state.grid.list.collection;
    let row_key = StoredValue::new(key.clone());
    let mounted = Memo::new(move |was: Option<&bool>| {
        was.copied().unwrap_or(false)
            || row_key.with_value(|key| rows.with(|rows| rows.contains_key(key)))
    });
    let parts = std::sync::Mutex::new(Some((data, key, classes, styles, children)));
    Some(Either::Right(move || {
        if !mounted.get() {
            return None;
        }
        let (data, key, classes, styles, children) = parts.lock().ok()?.take()?;
        // Untracked: this closure must run only when `mounted` turns true. A signal read while
        // building the row (its hooks, its children) would run it again, and with the parts
        // taken, the row would disappear.
        Some(untrack(|| table_row(data, key, classes, styles, children)))
    }))
}

/// A [`TableRow`] in `data`'s table.
fn table_row(
    data: TableData,
    key: Key,
    classes: Classes,
    styles: Styles,
    children: Children,
) -> AnyView {
    let data_rows = data.state.grid.list.collection;
    let selection = data.state.grid.list.selection;
    let is_tree = data.state.tree.is_some();
    let selection_column = use_context::<SelectionColumn>();
    // The row's resets and its hover's react to the disabled state in one effect.
    let (row, hover) = sharing_one_effect(|| {
        let row = use_table_row(UseTableRowInput {
            // Inside a `ContextMenuTrigger`: its menu opens on this row.
            on_context_menu: super::menu::ContextMenuTargetContext::for_item(&key),
            table: data,
            key: key.clone(),
        });
        let (allows_selection, has_action) = (row.allows_selection, row.has_action);
        // Interactive rows show hover (react-aria-components' `Row`).
        let hover = use_hover(UseHoverInput {
            is_disabled: Signal::derive(move || !allows_selection.get() && !has_action.get()),
            on_hover_start: None,
            on_hover_end: None,
            on_hover_change: None,
        });
        (row, hover)
    });
    let UseTableRowReturn {
        row: row_props,
        is_selected,
        is_focused,
        is_disabled,
        is_pressed,
        expand_button,
        is_expanded,
        has_child_rows,
        level,
        ..
    } = row;
    let (attrs, row_styles) = row_props.attrs();
    // The row level, for indenting (react-aria-components' `--table-row-level`): 1 without a
    // tree column.
    let row_styles = if is_tree {
        row_styles
            .add_optional_unchecked("--table-row-level", move || Some(level.get().to_string()))
    } else {
        row_styles.add_unchecked("--table-row-level", "1")
    };
    let styles = row_styles.merge(styles);
    let is_focus_visible = Signal::derive(move || is_focused.get() && is_focus_visible());
    let focus_within = use_focus_ring(UseFocusRingInput {
        target: FocusRingTarget::Within,
        ..UseFocusRingInput::default()
    });
    let focus_within_props = focus_within.target.into_props();

    // In a tree table, rows under a collapsed row aren't part of the grid: hidden. Other tables
    // render only rows of the grid (no attribute at all).
    let is_hidden = if is_tree {
        let row_key = key.clone();
        MaybeProp::derive(move || Some(!data_rows.with(|rows| rows.contains_key(&row_key))))
    } else {
        MaybeProp::default()
    };
    let context = RowContext {
        key,
        expand_button: StoredValue::new(expand_button),
        has_child_rows,
        is_expanded,
        level,
        is_selected,
        is_disabled,
        is_focus_visible_within: focus_within.is_focus_visible,
    };

    // The row's context in an owner of its own (cheaper than a `<Provider>`, `performance.md`
    // R11).
    scoped_view(
        move || provide_context(context),
        move || {
            view! {
                <tr
                    {..attrs}
                    {..hover.target.attrs()}
                    {..(
                        focus_within_props.on_focusin.into_on(ev::focusin),
                        focus_within_props.on_focusout.into_on(ev::focusout),
                    )}
                    class=classes
                    style={styles}
                    {..maybe_attr("hidden", is_hidden)}
                    {..data_flags([
                        ("data-hovered", hover.is_hovered),
                        ("data-focused", is_focused),
                        ("data-focus-visible", is_focus_visible),
                    ])}
                    {..data_flags([
                        ("data-focus-visible-within", focus_within.is_focus_visible),
                        ("data-pressed", is_pressed),
                    ])}
                    {..data_flags([
                        ("data-selected", is_selected),
                        ("data-disabled", is_disabled),
                    ])}
                    {..data_attributes::selection_mode(selection.selection_mode_signal())}
                    {..data_flags([
                        ("data-expanded", is_expanded),
                        ("data-has-child-items", has_child_rows),
                    ])}
                    {..signal_attr("data-level", level)}
                >
                    {move || {
                        selection_column
                            .and_then(|column| column.0.get())
                            .map(|column| view! { <TableCell column /> })
                    }}
                    {children()}
                </tr>
            }
        },
    )
    .into_any()
}

/// A cell of a [`TableRow`], in the column `column`. Cells of the selection checkbox column
/// render the row's checkbox.
///
/// The cell is keyed by its row and column (`Key::cell(row, column)`): it stays (rendered once,
/// keeping its focus) when columns before it are added or removed, e.g. the selection checkbox
/// column.
///
/// Data attributes: `data-focused`, `data-focus-visible`, `data-focus-visible-within-row`
/// (keyboard focus on the cell's row or inside it), `data-hovered`, `data-pressed`,
/// `data-selected` and `data-disabled` (the row's), `data-column-index` (the index of the cell's
/// first column, the selection checkbox column included), `data-level` (the row's level, 1 for
/// top-level rows), and in a tree table `data-tree-column`, `data-expanded` and
/// `data-has-child-items`.
///
/// Default class: `leptonic-TableCell`.
#[component]
pub fn TableCell(
    /// The key of the cell's column.
    #[prop(into)]
    column: Key,
    /// What gets focus: the cell, or its first focusable child. Defaults to the cell with
    /// `KeyboardNavigationBehavior::Tab` and the child otherwise.
    #[prop(optional)]
    focus_mode: Option<CellFocusMode>,
    /// Let ArrowLeft/ArrowRight move between the cell's children (and ArrowUp/ArrowDown between
    /// rows) even with `KeyboardNavigationBehavior::Tab`.
    #[prop(optional)]
    allows_arrow_navigation: bool,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    crate::metrics::subject!();
    let classes = with_default_class("leptonic-TableCell", classes);
    let (Some(data), Some(row_context)) = (use_context::<TableData>(), use_context::<RowContext>())
    else {
        crate::utils::dev_warn!("a <TableCell> belongs in a <TableRow>");
        return None;
    };
    let row = row_context.key.clone();
    let RowContext {
        is_expanded: row_expanded,
        has_child_rows: row_has_child_rows,
        level: row_level,
        is_selected,
        is_disabled,
        is_focus_visible_within: is_focus_visible_within_row,
        ..
    } = row_context;
    let is_tree_column = data
        .state
        .tree
        .is_some_and(|tree| tree.is_tree_column(&column));
    // The cell's key follows from its row and column (a cell spanning columns has its first
    // column's), so it doesn't change with the rows or the other columns. The column's kind
    // doesn't change either (only the generated selection column has checkboxes).
    let key = Key::cell(&row, column.clone());
    let columns = data.state.columns;
    let kind = columns.with_untracked(|columns| {
        columns
            .iter()
            .find(|c| c.key == column)
            .map_or(ColumnKind::Data, |c| c.kind)
    });
    // The index of the cell's column changes when columns before it come and go.
    let column_index = Signal::derive(move || {
        columns.with(|columns| {
            columns
                .iter()
                .find(|c| c.key == column)
                .map_or(0, |c| c.index)
        })
    });
    let content = if kind == ColumnKind::SelectionCheckbox {
        let checkbox = use_checkbox(use_table_selection_checkbox(
            UseTableSelectionCheckboxInput {
                table: data.clone(),
                key: row,
                is_selected,
            },
        ));
        checkbox.input.props().map(|input| {
            let (attrs, styles) = input.into_parts();
            view! { <input {..attrs} style=styles /> }.into_any()
        })
    } else {
        children.map(|children| children())
    };
    let UseTableCellReturn {
        cell: grid_cell_props,
        is_pressed,
    } = use_table_cell(UseTableCellInput {
        focus_mode,
        should_select_on_press_up: data.grid.should_select_on_press_up,
        table: data,
        key,
        allows_arrow_navigation,
    });
    let (attrs, cell_styles) = grid_cell_props.attrs();
    let styles = cell_styles.merge(styles);
    // Focus on the cell itself, and hover (react-aria-components' `Cell`).
    let UseFocusRingReturn {
        target: focus_ring,
        is_focused,
        ..
    } = use_focus_ring(UseFocusRingInput::default());
    let hover = use_hover(UseHoverInput::default());

    Some(view! {
        <td
            {..attrs}
            {..focus_ring.attrs()}
            {..hover.target.attrs()}
            class=classes
            style={styles}
            {..data_flags([
                ("data-focused", is_focused),
                ("data-hovered", hover.is_hovered),
                ("data-pressed", is_pressed),
            ])}
            {..data_flags([
                ("data-focus-visible-within-row", is_focus_visible_within_row),
                ("data-selected", is_selected),
                ("data-disabled", is_disabled),
            ])}
            {..signal_attr("data-column-index", column_index)}
            data-tree-column=is_tree_column.then_some("true")
            {..data_flags([
                ("data-expanded", row_expanded),
                ("data-has-child-items", row_has_child_rows),
            ])}
            {..signal_attr("data-level", row_level)}
        >
            {content}
        </td>
    })
}

/// The expand button of a tree table's row (react-aria-components' `Button slot="chevron"`):
/// put it into the row's cell in the tree column. Labelled "Expand"/"Collapse" plus the row; it
/// is `hidden` while the row has no child rows.
///
/// Data attributes: `data-expanded`, `data-pressed`, `data-hovered`, `data-focus-visible`,
/// `data-disabled`.
///
/// Default class: `leptonic-TableExpandButton`.
#[component]
pub fn TableExpandButton(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    crate::metrics::subject!();
    let classes = with_default_class("leptonic-TableExpandButton", classes);
    let Some(row) = use_context::<RowContext>() else {
        crate::utils::dev_warn!("a <TableExpandButton> belongs in a <TableRow>");
        return None;
    };
    let Some(input) = row.expand_button.get_value() else {
        crate::utils::dev_warn!("a <TableExpandButton> belongs in a tree table (`tree_column`)");
        return None;
    };
    let button = use_button(input);
    let (attrs, button_styles) = button.button.attrs();
    let has_child_rows = row.has_child_rows;
    Some(view! {
        <button
            {..attrs}
            {..crate::utils::focusability::prevent_focus_attr()}
            class=classes
            style=button_styles.merge(styles)
            hidden=move || !has_child_rows.get()
            {..data_flags([
                ("data-pressed", button.is_pressed),
                ("data-hovered", button.is_hovered),
            ])}
            {..data_flags([
                ("data-expanded", row.is_expanded),
                ("data-disabled", button.is_disabled),
            ])}
        >
            {children.map(|children| children())}
        </button>
    })
}

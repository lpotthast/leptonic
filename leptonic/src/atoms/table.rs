use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use leptos::{context::Provider, html, prelude::*};
use leptos_use::use_resize_observer;

use crate::{
    Out,
    hooks::{
        CellFocusMode, ColumnKind, ColumnSize, DisabledBehavior, GridFocusMode, IntoAttrs,
        KeyboardNavigationBehavior, SelectionBehavior, SelectionMode, SortDescriptor,
        SortDirection, TableCollection, TableColumnResizeState, TableData, UseTableCellInput,
        UseTableCellReturn, UseTableColumnHeaderInput, UseTableColumnHeaderReturn,
        UseTableColumnResizeInput, UseTableColumnResizeReturn, UseTableColumnResizeStateInput,
        UseTableInput, UseTableReturn, UseTableRowInput, UseTableRowReturn, UseTableStateInput,
        collections::{
            CollectionOptions, EscapeKeyBehavior, Key, NodeKind, Selection, SelectionOptions,
        },
        use_checkbox, use_grid_row_group, use_table, use_table_cell, use_table_column_header,
        use_table_column_resize, use_table_column_resize_state, use_table_header_placeholder,
        use_table_header_row, use_table_row, use_table_select_all_checkbox,
        use_table_selection_checkbox, use_table_state,
    },
    utils::{
        CapturedElement, ValueBinding,
        classes::Classes,
        css::{Size, computed_px, computed_size},
        data_attributes::flag,
        default_class::with_default_class,
        i18n::use_direction,
        locale::WritingDirection,
        scoped_context::scoped_view,
        style::WidthProperty,
        styles::Styles,
    },
};

/// Column sizes reported by resizing (see [`ResizableTableContainer`]).
type ColumnSizes = HashMap<Key, ColumnSize>;

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
/// Default class: `leptonic-Table`.
#[component]
#[allow(clippy::too_many_lines, clippy::implicit_hasher)]
pub fn Table(
    /// The columns and rows.
    #[prop(into)]
    table: Memo<Arc<TableCollection>>,
    /// Whether arrow keys focus rows (and then cells) or only cells.
    #[prop(optional)]
    focus_mode: GridFocusMode,
    #[prop(into, optional)] selection_mode: Signal<SelectionMode>,
    #[prop(optional)] selection_behavior: SelectionBehavior,
    /// The initially selected rows.
    #[prop(into, optional)]
    default_selected_keys: Vec<Key>,
    /// The selection (controlled), replacing `default_selected_keys`: a value or any signal.
    #[prop(into, optional)]
    selection: Option<Signal<Selection>>,
    /// Receives the new state: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_selection: Option<Out<Selection>>,
    #[prop(into, optional)] on_selection_change: Option<Callback<Selection>>,
    #[prop(into, optional)] disabled_keys: Option<Signal<HashSet<Key>>>,
    /// Defaults to `DisabledBehavior::Selection`: disabled rows can be focused, not selected.
    #[prop(optional)]
    disabled_behavior: Option<DisabledBehavior>,
    #[prop(optional)] disallow_empty_selection: bool,
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
    on_sort_change: Option<Callback<SortDescriptor>>,
    #[prop(optional)] keyboard_navigation_behavior: KeyboardNavigationBehavior,
    /// Called with the key of an activated row.
    #[prop(into, optional)]
    on_row_action: Option<Callback<Key>>,
    /// Called with the key of an activated cell.
    #[prop(into, optional)]
    on_cell_action: Option<Callback<Key>>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-Table", classes);
    // Without `sort_descriptor`, the table owns the sorting and `set_sort_descriptor` receives
    // each change, like `on_sort_change`.
    let on_sort_change = match (sort_descriptor, set_sort_descriptor) {
        (None, Some(set_sort_descriptor)) => Some(Callback::new(move |sort: SortDescriptor| {
            set_sort_descriptor.set(Some(sort.clone()));
            if let Some(on_sort_change) = on_sort_change {
                on_sort_change.run(sort);
            }
        })),
        _ => on_sort_change,
    };
    let (selection, on_selection_change) =
        ValueBinding::from_state_props(selection, set_selection, on_selection_change);
    let state = use_table_state(UseTableStateInput {
        selection: SelectionOptions {
            selection_mode,
            selection_behavior,
            default_selection: Selection::keys(default_selected_keys),
            selection,
            on_selection_change,
            disallow_empty_selection: Signal::stored(disallow_empty_selection),
            disabled_keys: disabled_keys.unwrap_or_default(),
            disabled_behavior: disabled_behavior.unwrap_or(DisabledBehavior::Selection),
            ..SelectionOptions::default()
        },
        focus_mode,
        default_sort_descriptor,
        sort_descriptor: sort_descriptor
            .map(|value| ValueBinding::from_props(value, set_sort_descriptor)),
        on_sort_change,
        table,
    });

    let UseTableReturn { props, data } = use_table(UseTableInput {
        aria_label,
        aria_labelledby,
        options: CollectionOptions {
            escape_key_behavior,
            ..CollectionOptions::default()
        },
        keyboard_navigation_behavior,
        on_row_action,
        on_cell_action,
        state,
        element: CapturedElement::new(),
        id: None,
        keyboard_delegate: None,
        should_select_on_press_up: false,
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

    scoped_view(
        move || {
            provide_context(data);
            if let Some(column_resize) = column_resize {
                provide_context(column_resize);
            }
        },
        move || {
            view! {
                <table {..props.into_attrs()} class=classes style=styles>
                    {children()}
                </table>
            }
        },
    )
}

/// The header of a [`Table`]: its header rows with the column headers (and, for a selection
/// checkbox column, a "select all" checkbox in multiple selection mode).
///
/// Column headers expose `data-allows-sorting`, `data-sort-direction` (`ascending` /
/// `descending`), `data-focused` and `data-pressed` for styling.
///
/// Default class: `leptonic-TableHeader`.
#[component]
pub fn TableHeader(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let classes = with_default_class("leptonic-TableHeader", classes);
    let data = expect_context::<TableData>();
    let row_group = use_grid_row_group();
    let rows = data.state.table;
    view! {
        <thead {..row_group.row_group_props.into_attrs()} class=classes style=styles>
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
                        <tr {..use_table_header_row().into_attrs()}>
                            <For
                                each=cells
                                key=|(key, _)| key.clone()
                                children=move |(key, kind): (Key, NodeKind)| {
                                    if kind == NodeKind::Placeholder {
                                        let data = expect_context::<TableData>();
                                        view! { <th {..use_table_header_placeholder(&data, &key).into_attrs()}></th> }
                                            .into_any()
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
    }
}

/// A column header of a [`Table`], rendered by [`TableHeader`].
///
/// Default class: `leptonic-TableColumnHeader`.
#[component]
fn TableColumnHeader(key: Key) -> impl IntoView {
    let data = expect_context::<TableData>();
    let state = data.state;
    let (text, kind, allows_sorting, allows_resizing) = state.table.with_untracked(|t| {
        t.column(&key)
            .map_or((Arc::from(""), ColumnKind::Data, false, false), |c| {
                (
                    c.text_value.clone(),
                    c.kind,
                    c.allows_sorting,
                    c.allows_resizing,
                )
            })
    });
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
        move || (selection.is_focused() && selection.is_focused_key(&focus_key)).then_some("true");
    let content = if kind == ColumnKind::SelectionCheckbox {
        (selection.selection_mode() == SelectionMode::Multiple)
            .then(|| {
                let checkbox = use_checkbox(use_table_select_all_checkbox(&data));
                let (attrs, styles) = checkbox.input_props.into_parts();
                view! { <input {..attrs} style=styles /> }
            })
            .into_any()
    } else {
        text.to_string().into_any()
    };
    // With column resizing: the column's width, and its resizer.
    let resize = use_context::<ColumnResizeContext>();
    let header = CapturedElement::new();
    let resizer = resize.filter(|_| allows_resizing).map(|resize| {
        view! { <ColumnResizer resize column=key.clone() trigger=header /> }
    });
    let resizing_key = key.clone();
    let is_resizing = move || {
        resize
            .is_some_and(|r| {
                r.state
                    .resizing_column
                    .with(|c| c.as_ref() == Some(&resizing_key))
            })
            .then_some("true")
    };
    let width_key = key.clone();
    let UseTableColumnHeaderReturn {
        column_header_props,
        is_pressed,
    } = use_table_column_header(UseTableColumnHeaderInput {
        table: data,
        key,
        allows_arrow_navigation: false,
    });
    let (attrs, styles) = column_header_props.into_parts();
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
            class="leptonic-TableColumnHeader"
            style=styles
            data-allows-sorting=allows_sorting.then_some("true")
            data-sort-direction=sort_direction
            data-focused=is_focused
            data-pressed=move || is_pressed.get().then_some("true")
            data-resizing=is_resizing
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
) -> impl IntoView {
    let data = expect_context::<TableData>();
    let state = resize.state;
    let UseTableColumnResizeReturn {
        resizer_props,
        input_props,
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
        aria_label: crate::hooks::RESIZER_LABEL.to_owned(),
        element: CapturedElement::new(),
        is_disabled: Signal::stored(false),
    });
    // `left` at the minimum width, `right` at the maximum (react-aria-components' values).
    let direction = use_direction();
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
    let (attrs, styles) = resizer_props.into_parts();
    let (input_attrs, input_styles) = input_props.into_parts();
    view! {
        <div
            role="presentation"
            {..attrs}
            class="leptonic-ColumnResizer"
            style=styles
            data-column-resizer="true"
            data-resizing=move || is_resizing.get().then_some("true")
            data-resizable-direction=resizable_direction
        >
            <input {..input_attrs} style=input_styles />
        </div>
    }
}

/// The body of a [`Table`]: its rows.
///
/// Default class: `leptonic-TableBody`.
#[component]
pub fn TableBody(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-TableBody", classes);
    let row_group = use_grid_row_group();
    view! {
        <tbody {..row_group.row_group_props.into_attrs()} class=classes style=styles>
            {children()}
        </tbody>
    }
}

/// The row a [`TableCell`] is in.
#[derive(Debug, Clone)]
struct RowContext {
    key: Key,
}

/// A row of a [`Table`], for the collection row `key`. With a selection checkbox column, it
/// renders the selection cell itself; add one [`TableCell`] per data column.
///
/// Exposes `data-selected`, `data-focused`, `data-disabled` and `data-pressed` for styling.
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
    let classes = with_default_class("leptonic-TableRow", classes);
    let data = expect_context::<TableData>();
    let selection_column = data.state.table.with_untracked(|t| {
        t.column_at(0)
            .filter(|c| c.kind == ColumnKind::SelectionCheckbox)
            .map(|c| c.key.clone())
    });
    let UseTableRowReturn {
        row_props,
        is_selected,
        is_focused,
        is_disabled,
        is_pressed,
        ..
    } = use_table_row(UseTableRowInput {
        // Inside a `ContextMenuTrigger`: its menu opens on this row.
        on_context_menu: super::menu::ContextMenuTargetContext::for_item(&key),
        table: data,
        key: key.clone(),
    });
    let (attrs, row_styles) = row_props.into_parts();
    let styles = row_styles.merge(styles);

    view! {
        <Provider value=RowContext { key }>
            <tr
                {..attrs}
                class=classes
                style=styles
                data-selected=flag(is_selected)
                data-focused=flag(is_focused)
                data-disabled=flag(is_disabled)
                data-pressed=flag(is_pressed)
            >
                {selection_column.map(|column| view! { <TableCell column /> })}
                {children()}
            </tr>
        </Provider>
    }
}

/// A cell of a [`TableRow`], in the column `column`. Cells of the selection checkbox column
/// render the row's checkbox.
///
/// Exposes `data-pressed` for styling.
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
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let classes = with_default_class("leptonic-TableCell", classes);
    let data = expect_context::<TableData>();
    let row = expect_context::<RowContext>().key;
    let (index, kind) = data.state.table.with_untracked(|t| {
        t.column(&column)
            .map_or((0, ColumnKind::Data), |c| (c.index, c.kind))
    });
    let content = if kind == ColumnKind::SelectionCheckbox {
        let checkbox = use_checkbox(use_table_selection_checkbox(&data, row.clone()));
        let (attrs, styles) = checkbox.input_props.into_parts();
        Some(view! { <input {..attrs} style=styles /> }.into_any())
    } else {
        children.map(|children| children().into_any())
    };
    let UseTableCellReturn {
        grid_cell_props,
        is_pressed,
    } = use_table_cell(UseTableCellInput {
        focus_mode,
        table: data,
        key: Key::cell(&row, index),
        allows_arrow_navigation: false,
        should_select_on_press_up: false,
    });
    let (attrs, cell_styles) = grid_cell_props.into_parts();
    let styles = cell_styles.merge(styles);

    view! {
        <td
            {..attrs}
            class=classes
            style=styles
            data-pressed=move || is_pressed.get().then_some("true")
        >
            {content}
        </td>
    }
}

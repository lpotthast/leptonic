use std::collections::HashSet;
use std::hash::Hash;

use crate::hooks::*;
use crate::utils::classes::Classes;
use crate::utils::styles::Styles;
use leptos::prelude::*;

/// Private context struct sharing grid state between `Grid`, `GridRow`, and `GridCell`.
#[derive(Clone)]
struct GridCtx<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    state: UseGridState<K>,
}

// Manual Copy impl to avoid derive adding `K: Copy` bound.
impl<K: Hash + Eq + Clone + Send + Sync + 'static> Copy for GridCtx<K> {}

/// A headless 2D grid container atom.
///
/// Wraps `use_grid` and provides shared state to child `GridRowGroup`, `GridRow`,
/// and `GridCell` atoms via context.
///
/// Renders a `<div>` with proper ARIA grid attributes.
#[component]
#[allow(clippy::too_many_lines, clippy::implicit_hasher)]
pub fn Grid<K>(
    /// The grid collection describing rows and cells.
    #[prop(into)]
    collection: Signal<GridCollection<K>>,
    /// Keys of disabled rows/cells.
    #[prop(into, optional)]
    disabled_keys: Option<Signal<HashSet<K>>>,
    /// How focus moves within the grid (Row vs Cell mode).
    #[prop(into, optional)]
    focus_mode: GridFocusMode,
    /// The selection mode.
    #[prop(into, optional)]
    selection_mode: SelectionMode,
    /// The selection behavior (toggle vs replace).
    #[prop(into, optional)]
    selection_behavior: SelectionBehavior,
    /// Controlled selected keys.
    #[prop(into, optional)]
    selected_keys: Option<Signal<Selection<K>>>,
    /// Default selected keys (uncontrolled).
    #[prop(into, optional)]
    default_selected_keys: Option<Selection<K>>,
    /// Callback when selection changes.
    #[prop(into, optional)]
    on_selection_change: Option<Callback<Selection<K>>>,
    /// Whether to disallow empty selection.
    #[prop(into, optional)]
    disallow_empty_selection: bool,
    /// Whether the grid is disabled.
    #[prop(into, optional)]
    disabled: Option<Signal<bool>>,
    /// Escape key behavior.
    #[prop(into, optional)]
    escape_key_behavior: EscapeKeyBehavior,
    /// Whether arrow key navigation wraps around.
    #[prop(into, optional)]
    should_focus_wrap: bool,
    /// Callback when a row is activated (Enter key on a row key).
    #[prop(into, optional)]
    on_row_action: Option<Callback<K>>,
    /// Callback when a cell is activated (Enter key on a cell key).
    #[prop(into, optional)]
    on_cell_action: Option<Callback<K>>,
    /// An accessible label for the grid.
    #[prop(into, optional)]
    label: Option<String>,
    /// The ID of an element that labels the grid.
    #[prop(into, optional)]
    labelled_by: Option<String>,
    /// CSS classes.
    #[prop(into, optional)]
    classes: Classes,
    /// CSS styles.
    #[prop(into, optional)]
    styles: Styles,
    children: Children,
) -> impl IntoView
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    let disabled_keys = disabled_keys.unwrap_or_else(|| Signal::derive(HashSet::new));
    let is_disabled = disabled.unwrap_or_else(|| Signal::derive(|| false));

    let grid = use_grid(UseGridInput {
        label,
        labelled_by,
        collection,
        disabled_keys,
        focus_mode,
        selection_mode,
        selection_behavior,
        selected_keys,
        default_selected_keys,
        on_selection_change,
        disallow_empty_selection,
        is_disabled,
        escape_key_behavior,
        should_focus_wrap,
        on_row_action,
        on_cell_action,
    });

    provide_context(GridCtx { state: grid.state });

    view! {
        <div {..grid.props.into_attrs()} class=classes style=styles>
            {children()}
        </div>
    }
}

/// A headless grid row group atom.
///
/// Wraps `use_grid_row_group` and renders a `<div>` with `role="rowgroup"`.
#[component]
pub fn GridRowGroup(
    /// CSS classes.
    #[prop(into, optional)]
    classes: Classes,

    /// CSS styles.
    #[prop(into, optional)]
    styles: Styles,

    children: Children,
) -> impl IntoView {
    let row_group = use_grid_row_group();
    view! {
        <div {..row_group.props.into_attrs()} class=classes style=styles>
            {children()}
        </div>
    }
}

/// A headless grid row atom.
///
/// Wraps `use_grid_row` and renders a `<div>` with proper ARIA row attributes.
/// Exposes `data-selected`, `data-focused`, and `data-disabled` attributes
/// for CSS styling.
#[component]
pub fn GridRow<K>(
    /// The unique key for this row.
    #[prop(into)]
    item_key: K,
    /// The row index (0-based).
    row_index: usize,
    /// CSS classes.
    #[prop(into, optional)]
    classes: Classes,
    /// CSS styles.
    #[prop(into, optional)]
    styles: Styles,
    children: Children,
) -> impl IntoView
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    let ctx = expect_context::<GridCtx<K>>();

    let row = use_grid_row(UseGridRowInput {
        state: ctx.state,
        key: item_key,
        row_index,
    });

    let is_focused = row.is_focused;

    let data_selected = Signal::derive(move || {
        if row.is_selected.get() {
            Some("true")
        } else {
            None
        }
    });
    let data_focused = Signal::derive(move || if is_focused.get() { Some("true") } else { None });
    let data_disabled = Signal::derive(move || {
        if row.is_disabled.get() {
            Some("true")
        } else {
            None
        }
    });

    view! {
        <div
            {..row.props.into_attrs()}
            class=classes
            style=styles
            attr:data-selected=data_selected
            attr:data-focused=data_focused
            attr:data-disabled=data_disabled
        >
            {children()}
        </div>
    }
}

/// A headless grid cell atom.
///
/// Wraps `use_grid_cell` and renders a `<div>` with proper ARIA gridcell attributes.
/// Exposes `data-selected`, `data-focused`, and `data-disabled` attributes
/// for CSS styling.
#[component]
pub fn GridCell<K>(
    /// The unique key for this cell.
    #[prop(into)]
    item_key: K,
    /// The row index (0-based).
    row_index: usize,
    /// The column index (0-based).
    column_index: usize,
    /// How focus should behave when this cell receives focus.
    #[prop(into, optional)]
    focus_mode: CellFocusMode,
    /// CSS classes.
    #[prop(into, optional)]
    classes: Classes,
    /// CSS styles.
    #[prop(into, optional)]
    styles: Styles,
    children: Children,
) -> impl IntoView
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    let ctx = expect_context::<GridCtx<K>>();

    let cell = use_grid_cell(UseGridCellInput {
        state: ctx.state,
        key: item_key,
        row_index,
        column_index,
        focus_mode,
    });

    let is_focused = cell.is_focused;

    let data_selected = Signal::derive(move || {
        if cell.is_selected.get() {
            Some("true")
        } else {
            None
        }
    });
    let data_focused = Signal::derive(move || if is_focused.get() { Some("true") } else { None });
    let data_disabled = Signal::derive(move || {
        if cell.is_disabled.get() {
            Some("true")
        } else {
            None
        }
    });

    view! {
        <div
            {..cell.props.into_attrs()}
            class=classes
            style=styles
            attr:data-selected=data_selected
            attr:data-focused=data_focused
            attr:data-disabled=data_disabled
        >
            {children()}
        </div>
    }
}

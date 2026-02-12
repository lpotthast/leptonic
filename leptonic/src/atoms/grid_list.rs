use std::{collections::HashSet, hash::Hash};

use leptos::prelude::*;

use crate::{
    hooks::*,
    utils::{classes::Classes, styles::Styles},
};

/// Private context struct sharing grid list state between `GridList` and `GridListItem`.
#[derive(Clone)]
struct GridListCtx<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    state: UseGridListState<K>,
}

// Manual Copy impl to avoid derive adding `K: Copy` bound.
impl<K: Hash + Eq + Clone + Send + Sync + 'static> Copy for GridListCtx<K> {}

/// A headless 1D grid list container atom.
///
/// Wraps `use_grid_list` and provides shared state to child `GridListItem` atoms
/// via context.
///
/// Renders a `<div>` with proper ARIA grid attributes.
#[component]
#[allow(clippy::too_many_lines, clippy::implicit_hasher)]
pub fn GridList<K>(
    /// Ordered list of all row keys.
    #[prop(into)]
    all_keys: Signal<Vec<K>>,
    /// Keys of disabled rows.
    #[prop(into, optional)]
    disabled_keys: Option<Signal<HashSet<K>>>,
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
    /// Whether the grid list is disabled.
    #[prop(into, optional)]
    disabled: Option<Signal<bool>>,
    /// Escape key behavior.
    #[prop(into, optional)]
    escape_key_behavior: EscapeKeyBehavior,
    /// Whether arrow key navigation wraps around.
    #[prop(into, optional)]
    should_focus_wrap: bool,
    /// Callback when a row is activated (Enter key).
    #[prop(into, optional)]
    on_action: Option<Callback<K>>,
    /// An accessible label for the grid list.
    #[prop(into, optional)]
    label: Option<String>,
    /// The ID of an element that labels the grid list.
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

    let grid_list = use_grid_list(UseGridListInput {
        label,
        labelled_by,
        all_keys,
        disabled_keys,
        selection_mode,
        selection_behavior,
        selected_keys,
        default_selected_keys,
        on_selection_change,
        disallow_empty_selection,
        is_disabled,
        escape_key_behavior,
        should_focus_wrap,
        on_action,
    });

    provide_context(GridListCtx {
        state: grid_list.state,
    });

    view! {
        <div {..grid_list.props.into_attrs()} class=classes style=styles>
            {children()}
        </div>
    }
}

/// A headless grid list item atom.
///
/// Wraps `use_grid_list_item` and renders the required two-element ARIA structure:
/// an outer `<div role="row">` containing an inner `<div role="gridcell">`.
///
/// Exposes `data-selected`, `data-focused`, and `data-disabled` attributes
/// on the outer row element for CSS styling.
#[component]
pub fn GridListItem<K>(
    /// The unique key for this item.
    #[prop(into)]
    item_key: K,
    /// The row index (0-based).
    row_index: usize,
    /// Whether this item is disabled.
    #[prop(into, optional)]
    disabled: Option<Signal<bool>>,
    /// Accessible text value for this row.
    #[prop(into, optional)]
    text_value: Option<String>,
    /// CSS classes (applied to the outer row element).
    #[prop(into, optional)]
    classes: Classes,
    /// CSS styles (applied to the outer row element).
    #[prop(into, optional)]
    styles: Styles,
    children: Children,
) -> impl IntoView
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    let ctx = expect_context::<GridListCtx<K>>();
    let is_disabled = disabled.unwrap_or_else(|| Signal::derive(|| false));

    let item = use_grid_list_item(UseGridListItemInput {
        state: ctx.state,
        key: item_key,
        row_index,
        is_disabled,
        text_value,
    });

    let is_focused = item.is_focused;

    let data_selected = Signal::derive(move || {
        if item.is_selected.get() {
            Some("true")
        } else {
            None
        }
    });
    let data_focused = Signal::derive(move || if is_focused.get() { Some("true") } else { None });
    let data_disabled = Signal::derive(move || {
        if item.is_disabled.get() {
            Some("true")
        } else {
            None
        }
    });

    view! {
        <div
            {..item.row_props.into_attrs()}
            class=classes
            style=styles
            attr:data-selected=data_selected
            attr:data-focused=data_focused
            attr:data-disabled=data_disabled
        >
            <div {..item.gridcell_props.into_attrs()}>{children()}</div>
        </div>
    }
}

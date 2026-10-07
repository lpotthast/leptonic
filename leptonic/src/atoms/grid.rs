use std::collections::HashSet;

use leptos::{context::Provider, prelude::*};

use crate::{
    Out,
    hooks::{
        CellFocusMode, DisabledBehavior, GridData, GridFocusMode, IntoAttrs,
        KeyboardNavigationBehavior, SelectionBehavior, SelectionMode, UseFocusRingInput,
        UseFocusRingReturn, UseFocusVisibleInput, UseGridCellInput, UseGridCellReturn,
        UseGridInput, UseGridReturn, UseGridRowInput, UseGridRowReturn, UseGridStateInput,
        collections::{
            CollectionMemo, CollectionOptions, EscapeKeyBehavior, Key, Selection, SelectionOptions,
        },
        use_focus_ring, use_focus_visible, use_grid, use_grid_cell, use_grid_row,
        use_grid_row_group, use_grid_state,
    },
    utils::{
        CapturedElement, ValueBinding, classes::Classes, data_attributes::flag,
        default_class::with_default_class, styles::Styles,
    },
};

/// A headless grid: rows of cells, navigated in two dimensions with the arrow keys.
///
/// The rows and cells come from `collection` (built with `CollectionBuilder::row`): render one
/// [`GridRow`] per row and one [`GridCell`] per cell, in collection order.
///
/// Default class: `leptonic-Grid`.
#[component]
#[allow(clippy::too_many_lines, clippy::implicit_hasher)]
pub fn Grid(
    /// The rows and their cells.
    #[prop(into)]
    collection: CollectionMemo,
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
    #[prop(optional)] disabled_behavior: DisabledBehavior,
    #[prop(optional)] disallow_empty_selection: bool,
    #[prop(optional)] escape_key_behavior: EscapeKeyBehavior,
    /// Arrow keys wrap around at the ends.
    #[prop(optional)]
    should_focus_wrap: bool,
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
    let classes = with_default_class("leptonic-Grid", classes);
    let (selection, on_selection_change) =
        ValueBinding::from_state_props(selection, set_selection, on_selection_change);
    let state = use_grid_state(UseGridStateInput {
        collection,
        selection: SelectionOptions {
            selection_mode,
            selection_behavior: Signal::stored(selection_behavior),
            default_selection: Selection::keys(default_selected_keys),
            selection,
            on_selection_change,
            disallow_empty_selection: Signal::stored(disallow_empty_selection),
            disabled_keys: disabled_keys.unwrap_or_default(),
            disabled_behavior,
            ..SelectionOptions::default()
        },
        focus_mode,
    });

    let UseGridReturn { props, data } = use_grid(UseGridInput {
        aria_label,
        aria_labelledby,
        options: CollectionOptions {
            should_focus_wrap,
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

    // One keyboard-modality signal for all rows.
    let focus_visible = GridFocusVisible(
        use_focus_visible(UseFocusVisibleInput::default()).focus_should_be_visible,
    );
    view! {
        <Provider value=data>
            <Provider value=focus_visible>
                <div {..props.into_attrs()} class=classes style=styles>
                    {children()}
                </div>
            </Provider>
        </Provider>
    }
}

/// Whether focus rings should be visible (keyboard modality), for the [`GridRow`]s of a [`Grid`].
#[derive(Debug, Clone, Copy)]
struct GridFocusVisible(Signal<bool>);

/// A group of rows of a [`Grid`] (`role="rowgroup"`).
///
/// Default class: `leptonic-GridRowGroup`.
#[component]
pub fn GridRowGroup(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-GridRowGroup", classes);
    let row_group = use_grid_row_group();
    view! {
        <div {..row_group.row_group_props.into_attrs()} class=classes style=styles>
            {children()}
        </div>
    }
}

/// A row of a [`Grid`], for the collection row `key`.
///
/// Exposes `data-selected`, `data-focused`, `data-focus-visible`, `data-disabled` and
/// `data-pressed` for styling.
///
/// Default class: `leptonic-GridRow`.
#[component]
pub fn GridRow(
    /// The row's key in the grid's collection.
    #[prop(into)]
    key: Key,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-GridRow", classes);
    let grid = expect_context::<GridData>();
    let UseGridRowReturn {
        row_props,
        is_selected,
        is_focused,
        is_disabled,
        is_pressed,
        ..
    } = use_grid_row(UseGridRowInput {
        grid,
        key,
        on_context_menu: None,
    });
    let (attrs, row_styles) = row_props.into_parts();
    let styles = row_styles.merge(styles);
    // Focused by keyboard (react-aria-components: the row's `useFocusRing`).
    let focus_visible = expect_context::<GridFocusVisible>().0;
    let is_focus_visible = Signal::derive(move || is_focused.get() && focus_visible.get());

    view! {
        <div
            {..attrs}
            class=classes
            style=styles
            data-selected=flag(is_selected)
            data-focused=flag(is_focused)
            data-focus-visible=flag(is_focus_visible)
            data-disabled=flag(is_disabled)
            data-pressed=flag(is_pressed)
        >
            {children()}
        </div>
    }
}

/// A cell of a [`Grid`], for the collection cell `key` (`Key::cell(row, column)`).
///
/// Exposes `data-pressed`, `data-focused` and `data-focus-visible` for styling.
///
/// Default class: `leptonic-GridCell`.
#[component]
pub fn GridCell(
    /// The cell's key in the grid's collection.
    #[prop(into)]
    key: Key,
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
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-GridCell", classes);
    let grid = expect_context::<GridData>();
    let UseGridCellReturn {
        grid_cell_props,
        is_pressed,
    } = use_grid_cell(UseGridCellInput {
        focus_mode,
        should_select_on_press_up: grid.should_select_on_press_up,
        grid,
        key,
        id: None,
        allows_arrow_navigation,
    });

    let (attrs, cell_styles) = grid_cell_props.into_parts();
    let styles = cell_styles.merge(styles);
    // Focus on the cell itself (react-aria-components: the cell's `useFocusRing`).
    let UseFocusRingReturn {
        props: focus_ring,
        is_focused,
        is_focus_visible,
    } = use_focus_ring(UseFocusRingInput::default());

    view! {
        <div
            {..attrs}
            {..focus_ring.into_attrs()}
            class=classes
            style=styles
            data-pressed=flag(is_pressed)
            data-focused=flag(is_focused)
            data-focus-visible=flag(is_focus_visible)
        >
            {children()}
        </div>
    }
}

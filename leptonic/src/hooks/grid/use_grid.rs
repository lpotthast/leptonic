use std::{collections::HashSet, hash::Hash};

use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{FocusEvent, KeyboardEvent, MouseEvent};

use super::{
    grid_collection::GridCollection,
    grid_keyboard_delegate::{GridFocusMode, GridKeyboardDelegate},
};
use crate::{
    hooks::{
        selection::{
            keyboard_delegate::KeyboardDelegate,
            use_selection_state::{
                use_selection_state, Selection, SelectionBehavior, SelectionMode,
                UseSelectionStateInput, UseSelectionStateReturn,
            },
        },
        IntoAttrs,
    },
    utils::{
        aria::{AriaDisabled, AriaMultiselectable},
        EventAccessors, EventHandler,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/grid/src/useGrid.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## OMITTED FEATURES
// - No virtualization (`is_virtualized`, `aria-rowcount`, `aria-colcount`).
// - No selection announcements (`useGridSelectionAnnouncement`).
// - No RTL direction swapping in keyboard navigation.
//
// ## DIFFERENT BEHAVIOR
// - Selection is delegated to `use_selection_state` instead of react-aria's
//   `useGridSelectionState` + `useSelectableCollection`.
// - No `gridMap` `WeakMap` equivalent — child hooks (`use_grid_cell`) receive
//   a `UseGridState<K>` struct explicitly instead of looking up shared state
//   from a mutable `WeakMap`.
//
// ## LEPTOS-SPECIFIC ADAPTATIONS
// - Uses `EventHandler` pattern for composable event handlers.
//
// =============================================================================

/// Controls Escape key behavior in the grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EscapeKeyBehavior {
    /// Pressing Escape clears the current selection.
    #[default]
    ClearSelection,
    /// Pressing Escape does nothing.
    None,
}

/// Input parameters for the `use_grid` hook.
#[derive(Clone)]
pub struct UseGridInput<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    // --- ARIA ---
    /// An accessible label for the grid.
    pub label: Option<String>,
    /// The ID of an element that labels the grid.
    pub labelled_by: Option<String>,

    // --- Grid structure ---
    /// The grid collection describing rows and cells.
    pub collection: Signal<GridCollection<K>>,
    /// Keys of disabled rows/cells.
    pub disabled_keys: Signal<HashSet<K>>,
    /// How focus moves within the grid (Row vs Cell mode).
    pub focus_mode: GridFocusMode,

    // --- Selection (forwarded to `use_selection_state`) ---
    /// The selection mode.
    pub selection_mode: SelectionMode,
    /// The selection behavior (toggle vs replace).
    pub selection_behavior: SelectionBehavior,
    /// Controlled selected keys.
    pub selected_keys: Option<Signal<Selection<K>>>,
    /// Default selected keys (uncontrolled).
    pub default_selected_keys: Option<Selection<K>>,
    /// Callback when selection changes.
    pub on_selection_change: Option<Callback<Selection<K>>>,
    /// Whether to disallow empty selection.
    pub disallow_empty_selection: bool,

    // --- Behavior ---
    /// Whether the grid is disabled.
    pub is_disabled: Signal<bool>,
    /// Escape key behavior.
    pub escape_key_behavior: EscapeKeyBehavior,
    /// Whether arrow key navigation wraps around.
    pub should_focus_wrap: bool,
    /// Callback when a row is activated (Enter key on a row key).
    pub on_row_action: Option<Callback<K>>,
    /// Callback when a cell is activated (Enter key on a cell key).
    pub on_cell_action: Option<Callback<K>>,
}

/// Shared grid state passed to dependent hooks like `use_grid_cell`.
///
/// Analogous to react-aria's `gridMap` `WeakMap`, but uses an explicit struct
/// passed from `use_grid` to child hooks instead of a mutable `WeakMap` lookup.
#[derive(Clone)]
pub struct UseGridState<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    /// The keyboard delegate for navigation.
    pub keyboard_delegate: GridKeyboardDelegate<K>,
    /// The selection state.
    pub selection: UseSelectionStateReturn<K>,
    /// The currently focused key.
    pub focused_key: Signal<Option<K>>,
    /// Set the focused key.
    pub set_focused_key: Callback<Option<K>>,
    /// Whether the grid is disabled.
    pub is_disabled: Signal<bool>,
    /// The selection mode.
    pub selection_mode: SelectionMode,
    /// The selection behavior.
    pub selection_behavior: SelectionBehavior,
    /// Callback when a cell is activated (Enter key on a cell key).
    pub on_cell_action: Option<Callback<K>>,
    /// Callback when a row is activated (Enter key or double-click on a row key).
    pub on_row_action: Option<Callback<K>>,
}

// Manual Copy impl to avoid the derive macro adding an unnecessary `K: Copy` bound.
impl<K: Hash + Eq + Clone + Send + Sync + 'static> Copy for UseGridState<K> {}

/// The return value of the `use_grid` hook.
pub struct UseGridReturn<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    /// Props for the grid container element. Call `.into_attrs()` for view spreading.
    pub props: UseGridProps,
    /// Shared state to pass to child hooks (`use_grid_cell`, `use_grid_row`).
    pub state: UseGridState<K>,
    /// The selection state, delegated to `use_selection_state`.
    pub selection: UseSelectionStateReturn<K>,
    /// The currently focused key.
    pub focused_key: Signal<Option<K>>,
    /// Set the focused key.
    pub set_focused_key: Callback<Option<K>>,
    /// Whether the grid container has focus.
    pub is_focused: Signal<bool>,
}

/// Props from `use_grid` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseGridProps {
    pub id: String,
    pub role: &'static str,
    pub tabindex: Signal<&'static str>,
    pub aria_label: Option<String>,
    pub aria_labelledby: Option<String>,
    pub aria_multiselectable: Option<AriaMultiselectable>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_mousedown: EventHandler<MouseEvent>,
}

impl IntoAttrs for UseGridProps {
    type Attrs = UseGridAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaMultiselectable, self.aria_multiselectable),
            Attr(attr::AriaDisabled, self.aria_disabled),
            self.on_keydown.into_on(ev::keydown),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_mousedown.into_on(ev::mousedown),
        )
    }
}

/// These attributes must be spread onto the target element using the spread syntax `<div {..attrs}/>`.
pub type UseGridAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, &'static str>,
    Attr<attr::Tabindex, Signal<&'static str>>,
    Attr<attr::AriaLabel, Option<String>>,
    Attr<attr::AriaLabelledby, Option<String>>,
    Attr<attr::AriaMultiselectable, Option<AriaMultiselectable>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::mousedown, SharedEventCallback<MouseEvent>>,
);

/// Provides the behavior and accessibility for a grid.
///
/// A grid displays items in a two-dimensional layout with keyboard navigation,
/// selection, and ARIA accessibility. This hook centralizes ALL keyboard
/// navigation at the grid container level using a `GridKeyboardDelegate`.
///
/// # Example
///
/// ```ignore
/// let collection = Signal::derive(move || {
///     GridCollection::new(vec![
///         GridRow { key: "row-0".into(), cells: vec!["0-0".into(), "0-1".into()] },
///         GridRow { key: "row-1".into(), cells: vec!["1-0".into(), "1-1".into()] },
///     ])
/// });
///
/// let grid = use_grid(UseGridInput {
///     label: Some("My Grid".to_string()),
///     collection,
///     selection_mode: SelectionMode::Multiple,
///     ..Default::default()
/// });
///
/// view! {
///     <div {..grid.props.into_attrs()}>
///         // Grid rows and cells...
///     </div>
/// }
/// ```
#[allow(clippy::too_many_lines)]
pub fn use_grid<K>(input: UseGridInput<K>) -> UseGridReturn<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    let UseGridInput {
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
    } = input;

    let grid_id = format!("grid-{}", Uuid::new_v4());

    // --- Keyboard delegate ---
    let delegate = GridKeyboardDelegate::new(collection, disabled_keys, focus_mode);

    // --- Selection state (delegated) ---
    let selection = use_selection_state(UseSelectionStateInput {
        selection_mode,
        selection_behavior,
        disabled: is_disabled,
        selected_keys,
        default_selected_keys,
        on_selection_change,
        disabled_keys,
        disallow_empty_selection,
    });

    // --- Focus tracking ---
    let (focused_key, set_focused_key_signal) = signal::<Option<K>>(None);
    let (is_focused_rw, set_is_focused) = signal(false);
    let is_focused: Signal<bool> = is_focused_rw.into();

    let set_focused_key = Callback::new(move |key: Option<K>| {
        set_focused_key_signal.set(key);
    });

    // --- Tabindex: -1 when grid has internal focus, 0 otherwise ---
    let tabindex = Signal::derive(move || if is_focused.get() { "-1" } else { "0" });

    // --- ARIA attributes ---
    let aria_multiselectable = match selection_mode {
        SelectionMode::None => None,
        SelectionMode::Single => Some(AriaMultiselectable::False),
        SelectionMode::Multiple => Some(AriaMultiselectable::True),
    };

    let aria_disabled = Signal::derive(move || is_disabled.get().then_some(AriaDisabled::True));

    // --- Keyboard handler ---
    let handle_keydown = move |e: KeyboardEvent| {
        if is_disabled.get_untracked() {
            return;
        }

        let key = e.key();
        let ctrl_or_meta = e.ctrl_key() || e.meta_key();
        let shift = e.shift_key();

        match key.as_str() {
            "ArrowDown" => {
                e.prevent_default();
                if let Some(focused) = focused_key.get_untracked() {
                    if let Some(next) = delegate.get_key_below(&focused) {
                        if shift && selection_mode == SelectionMode::Multiple {
                            selection.select.run(next.clone());
                        }
                        set_focused_key_signal.set(Some(next));
                    }
                }
            }
            "ArrowUp" => {
                e.prevent_default();
                if let Some(focused) = focused_key.get_untracked() {
                    if let Some(prev) = delegate.get_key_above(&focused) {
                        if shift && selection_mode == SelectionMode::Multiple {
                            selection.select.run(prev.clone());
                        }
                        set_focused_key_signal.set(Some(prev));
                    }
                }
            }
            "ArrowRight" => {
                e.prevent_default();
                if let Some(focused) = focused_key.get_untracked() {
                    if let Some(next) = delegate.get_key_right_of(&focused) {
                        set_focused_key_signal.set(Some(next));
                    }
                }
            }
            "ArrowLeft" => {
                e.prevent_default();
                if let Some(focused) = focused_key.get_untracked() {
                    if let Some(prev) = delegate.get_key_left_of(&focused) {
                        set_focused_key_signal.set(Some(prev));
                    }
                }
            }
            "Home" => {
                e.prevent_default();
                let focused = focused_key.get_untracked();
                if let Some(first) = delegate.get_first_key(focused.as_ref(), ctrl_or_meta) {
                    set_focused_key_signal.set(Some(first));
                }
            }
            "End" => {
                e.prevent_default();
                let focused = focused_key.get_untracked();
                if let Some(last) = delegate.get_last_key(focused.as_ref(), ctrl_or_meta) {
                    set_focused_key_signal.set(Some(last));
                }
            }
            "PageUp" => {
                e.prevent_default();
                if let Some(focused) = focused_key.get_untracked() {
                    if let Some(target) = delegate.get_key_page_above(&focused) {
                        set_focused_key_signal.set(Some(target));
                    }
                }
            }
            "PageDown" => {
                e.prevent_default();
                if let Some(focused) = focused_key.get_untracked() {
                    if let Some(target) = delegate.get_key_page_below(&focused) {
                        set_focused_key_signal.set(Some(target));
                    }
                }
            }
            " " => {
                if let Some(focused) = focused_key.get_untracked() {
                    if selection_mode != SelectionMode::None {
                        e.prevent_default();
                        selection.toggle.run(focused);
                    }
                }
            }
            "Enter" => {
                if let Some(focused) = focused_key.get_untracked() {
                    let coll = collection.get_untracked();
                    if coll.is_row_key(&focused) {
                        if let Some(on_action) = on_row_action {
                            e.prevent_default();
                            on_action.run(focused);
                        }
                    } else if coll.is_cell_key(&focused) {
                        if let Some(on_action) = on_cell_action {
                            e.prevent_default();
                            on_action.run(focused);
                        }
                    }
                }
            }
            "Escape" => {
                if escape_key_behavior == EscapeKeyBehavior::ClearSelection
                    && selection_mode != SelectionMode::None
                {
                    e.prevent_default();
                    selection.clear_selection.run(());
                }
            }
            "a" if ctrl_or_meta => {
                if selection_mode == SelectionMode::Multiple {
                    e.prevent_default();
                    selection.select_all.run(vec![]);
                }
            }
            // Tab: don't intercept — let the browser handle single tab-stop exit.
            _ => {}
        }
    };

    // --- Focus handler ---
    let handle_focus = move |_e: FocusEvent| {
        set_is_focused.set(true);

        // If nothing is focused yet, focus the first item.
        if focused_key.get_untracked().is_none() {
            if let Some(first) = delegate.get_first_key(None, true) {
                set_focused_key_signal.set(Some(first));
            }
        }
    };

    // --- Blur handler ---
    let grid_id_for_blur = grid_id.clone();
    let handle_blur = move |e: FocusEvent| {
        // Only blur if focus left the grid container entirely.
        if let Some(related) = e.related_target() {
            if let Ok(el) = related.dyn_into::<web_sys::Element>() {
                if let Some(container) = web_sys::window()
                    .and_then(|w| w.document())
                    .and_then(|d| d.get_element_by_id(&grid_id_for_blur))
                {
                    if container.contains(Some(&el)) {
                        return;
                    }
                }
            }
        }
        set_is_focused.set(false);
    };

    // --- Mousedown handler (prevent scrollbar stealing focus) ---
    let handle_mousedown = move |e: MouseEvent| {
        // If the mousedown target is the grid itself (scrollbar area), prevent
        // default to avoid stealing focus from focused cells.
        if e.expect_target() == e.expect_current_target() {
            e.prevent_default();
        }
    };

    let state = UseGridState {
        keyboard_delegate: delegate,
        selection,
        focused_key: focused_key.into(),
        set_focused_key,
        is_disabled,
        selection_mode,
        selection_behavior,
        on_cell_action,
        on_row_action,
    };

    UseGridReturn {
        props: UseGridProps {
            id: grid_id,
            role: "grid",
            tabindex,
            aria_label: label,
            aria_labelledby: labelled_by,
            aria_multiselectable,
            aria_disabled,
            on_keydown: EventHandler::new(handle_keydown),
            on_focus: EventHandler::new(handle_focus),
            on_blur: EventHandler::new(handle_blur),
            on_mousedown: EventHandler::new(handle_mousedown),
        },
        state,
        selection,
        focused_key: focused_key.into(),
        set_focused_key,
        is_focused,
    }
}

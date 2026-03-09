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

use super::use_grid::EscapeKeyBehavior;
use crate::{
    hooks::{
        selection::use_selection_state::{
            use_selection_state, Selection, SelectionBehavior, SelectionMode,
            UseSelectionStateInput, UseSelectionStateReturn,
        },
        IntoAttrs,
    },
    utils::{
        aria::{AriaDisabled, AriaMultiselectable, AriaRole},
        EventAccessors, EventHandler,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/gridlist/src/useGridList.ts
// and: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/gridlist/src/useGridListItem.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## OMITTED FEATURES
// - Virtualization (`isVirtualized`, `aria-rowcount`, `aria-colcount`).
// - Tree support (`hasChildItems`, `expandedKeys`, `aria-expanded`, `aria-level`,
//   `aria-posinset`, `aria-setsize`).
// - RTL direction swapping.
// - `isPressed` state — not tracked; use `use_press` separately if needed.
// - Link behavior (`linkBehavior`).
// - `shouldSelectOnPressUp` per-item override.
// - Type-ahead search.
// - Selection announcements (`useGridSelectionAnnouncement`).
// - Drag-and-drop.
// - Capture-phase keydown re-dispatch (we use bubble phase).
//
// ## DIFFERENT BEHAVIOR
// - Selection is delegated to `use_selection_state` directly.
// - `UseGridListState<K>` struct instead of `listMap` `WeakMap`.
// - `aria_multiselectable` only sets `"true"` for Multiple mode and omits it otherwise
//   (matching React Aria, which doesn't set `"false"` for Single).
// - Bubble-phase keydown instead of capture-phase + re-dispatch.
//
// ## LEPTOS-SPECIFIC ADAPTATIONS
// - `ElementCaptureAttr` instead of React refs for DOM element access.
// - `FocusManager` from `use_focus_manager` instead of `getFocusableTreeWalker`.
// - `EventHandler<E>` for composable event handler chaining.
// - Generic `K` key type instead of React Aria's `Key`.
//
// =============================================================================

/// Input parameters for the `use_grid_list` hook.
#[derive(Clone)]
pub struct UseGridListInput<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    // --- ARIA ---
    /// An accessible label for the grid list.
    pub label: Option<String>,
    /// The ID of an element that labels the grid list.
    pub labelled_by: Option<String>,

    // --- List structure ---
    /// Ordered list of all row keys.
    pub all_keys: Signal<Vec<K>>,
    /// Keys of disabled rows.
    pub disabled_keys: Signal<HashSet<K>>,

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
    /// Whether the grid list is disabled.
    pub is_disabled: Signal<bool>,
    /// Escape key behavior.
    pub escape_key_behavior: EscapeKeyBehavior,
    /// Whether arrow key navigation wraps around.
    pub should_focus_wrap: bool,
    /// Callback when a row is activated (Enter key).
    pub on_action: Option<Callback<K>>,
}

impl<K: Hash + Eq + Clone + Send + Sync + 'static> Default for UseGridListInput<K> {
    fn default() -> Self {
        Self {
            label: None,
            labelled_by: None,
            all_keys: Signal::derive(Vec::new),
            disabled_keys: Signal::derive(HashSet::new),
            selection_mode: SelectionMode::default(),
            selection_behavior: SelectionBehavior::default(),
            selected_keys: None,
            default_selected_keys: None,
            on_selection_change: None,
            disallow_empty_selection: false,
            is_disabled: Signal::derive(|| false),
            escape_key_behavior: EscapeKeyBehavior::default(),
            should_focus_wrap: false,
            on_action: None,
        }
    }
}

/// Shared grid list state passed to child hooks like `use_grid_list_item`.
///
/// Analogous to react-aria's `listMap` `WeakMap`, but uses an explicit struct
/// passed from `use_grid_list` to child hooks instead of a mutable `WeakMap` lookup.
#[derive(Clone)]
pub struct UseGridListState<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    /// The selection state.
    pub selection: UseSelectionStateReturn<K>,
    /// The currently focused key.
    pub focused_key: Signal<Option<K>>,
    /// Set the focused key.
    pub set_focused_key: Callback<Option<K>>,
    /// Whether the grid list is disabled.
    pub is_disabled: Signal<bool>,
    /// The selection mode.
    pub selection_mode: SelectionMode,
    /// The selection behavior.
    pub selection_behavior: SelectionBehavior,
    /// Callback when a row is activated (Enter key or double-click).
    pub on_action: Option<Callback<K>>,
}

// Manual Copy impl to avoid the derive macro adding an unnecessary `K: Copy` bound.
impl<K: Hash + Eq + Clone + Send + Sync + 'static> Copy for UseGridListState<K> {}

/// The return value of the `use_grid_list` hook.
pub struct UseGridListReturn<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    /// Props for the grid list container element. Call `.into_attrs()` for view spreading.
    pub props: UseGridListProps,
    /// Shared state to pass to child hooks (`use_grid_list_item`).
    pub state: UseGridListState<K>,
    /// The selection state, delegated to `use_selection_state`.
    pub selection: UseSelectionStateReturn<K>,
    /// The currently focused key.
    pub focused_key: Signal<Option<K>>,
    /// Set the focused key.
    pub set_focused_key: Callback<Option<K>>,
    /// Whether the grid list container has focus.
    pub is_focused: Signal<bool>,
}

/// Props from `use_grid_list` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseGridListProps {
    pub id: String,
    pub role: AriaRole,
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

impl IntoAttrs for UseGridListProps {
    type Attrs = UseGridListAttrs;

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
pub type UseGridListAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, AriaRole>,
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

/// Get the next non-disabled key in the given direction.
///
/// Returns `None` if there is no valid key to move to (end of list without wrapping).
fn get_next_key<K>(
    all_keys: &[K],
    disabled_keys: &HashSet<K>,
    current: &K,
    forward: bool,
    wrap: bool,
) -> Option<K>
where
    K: Hash + Eq + Clone,
{
    let len = all_keys.len();
    if len == 0 {
        return None;
    }

    let current_idx = all_keys.iter().position(|k| k == current)?;

    let mut idx = current_idx;
    let mut checked = 0;

    loop {
        if forward {
            if idx + 1 >= len {
                if wrap {
                    idx = 0;
                } else {
                    return None;
                }
            } else {
                idx += 1;
            }
        } else if idx == 0 {
            if wrap {
                idx = len - 1;
            } else {
                return None;
            }
        } else {
            idx -= 1;
        }

        checked += 1;
        if checked >= len {
            return None;
        }

        let candidate = &all_keys[idx];
        if !disabled_keys.contains(candidate) {
            return Some(candidate.clone());
        }
    }
}

/// Get the first non-disabled key.
fn get_first_key<K>(all_keys: &[K], disabled_keys: &HashSet<K>) -> Option<K>
where
    K: Hash + Eq + Clone,
{
    all_keys
        .iter()
        .find(|k| !disabled_keys.contains(k))
        .cloned()
}

/// Get the last non-disabled key.
fn get_last_key<K>(all_keys: &[K], disabled_keys: &HashSet<K>) -> Option<K>
where
    K: Hash + Eq + Clone,
{
    all_keys
        .iter()
        .rev()
        .find(|k| !disabled_keys.contains(k))
        .cloned()
}

/// Provides the behavior and accessibility for a grid list.
///
/// A grid list is a one-dimensional list with keyboard navigation and selection,
/// using a grid role for accessibility. Each item is a row containing a single
/// gridcell, which may contain focusable children navigable with ArrowLeft/ArrowRight.
///
/// ArrowUp/ArrowDown navigate between rows at the container level.
/// ArrowLeft/ArrowRight are reserved for within-row child navigation at the item level.
///
/// # Example
///
/// ```ignore
/// let all_keys = Signal::stored(vec!["item-0".to_string(), "item-1".to_string(), "item-2".to_string()]);
///
/// let grid_list = use_grid_list(UseGridListInput {
///     label: Some("My List".to_string()),
///     all_keys: all_keys.into(),
///     selection_mode: SelectionMode::Multiple,
///     ..Default::default()
/// });
///
/// view! {
///     <div {..grid_list.props.into_attrs()}>
///         // Grid list items...
///     </div>
/// }
/// ```
#[allow(clippy::too_many_lines)]
pub fn use_grid_list<K>(input: UseGridListInput<K>) -> UseGridListReturn<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    let UseGridListInput {
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
    } = input;

    let list_id = format!("gridlist-{}", Uuid::new_v4());

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

    // --- Tabindex: -1 when list has internal focus, 0 otherwise ---
    let tabindex = Signal::derive(move || if is_focused.get() { "-1" } else { "0" });

    // --- ARIA attributes ---
    // React Aria only sets aria-multiselectable="true" for Multiple mode and omits it otherwise.
    let aria_multiselectable = match selection_mode {
        SelectionMode::Multiple => Some(AriaMultiselectable::True),
        SelectionMode::Single | SelectionMode::None => None,
    };

    let aria_disabled = Signal::derive(move || is_disabled.get().then_some(AriaDisabled::True));

    // --- Keyboard handler ---
    let handle_keydown = move |e: KeyboardEvent| {
        if is_disabled.get_untracked() {
            return;
        }

        let key = e.key();
        let ctrl_or_meta = e.ctrl_key() || e.meta_key();

        match key.as_str() {
            "ArrowDown" => {
                e.prevent_default();
                if let Some(focused) = focused_key.get_untracked() {
                    let keys = all_keys.get_untracked();
                    let disabled = disabled_keys.get_untracked();
                    if let Some(next) =
                        get_next_key(&keys, &disabled, &focused, true, should_focus_wrap)
                    {
                        set_focused_key_signal.set(Some(next));
                    }
                }
            }
            "ArrowUp" => {
                e.prevent_default();
                if let Some(focused) = focused_key.get_untracked() {
                    let keys = all_keys.get_untracked();
                    let disabled = disabled_keys.get_untracked();
                    if let Some(prev) =
                        get_next_key(&keys, &disabled, &focused, false, should_focus_wrap)
                    {
                        set_focused_key_signal.set(Some(prev));
                    }
                }
            }
            // ArrowLeft/ArrowRight: No-op at container level.
            // Items handle within-row child navigation.
            "Home" => {
                e.prevent_default();
                let keys = all_keys.get_untracked();
                let disabled = disabled_keys.get_untracked();
                if let Some(first) = get_first_key(&keys, &disabled) {
                    set_focused_key_signal.set(Some(first));
                }
            }
            "End" => {
                e.prevent_default();
                let keys = all_keys.get_untracked();
                let disabled = disabled_keys.get_untracked();
                if let Some(last) = get_last_key(&keys, &disabled) {
                    set_focused_key_signal.set(Some(last));
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
                    if let Some(on_action) = on_action {
                        e.prevent_default();
                        on_action.run(focused);
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
            let keys = all_keys.get_untracked();
            let disabled = disabled_keys.get_untracked();
            if let Some(first) = get_first_key(&keys, &disabled) {
                set_focused_key_signal.set(Some(first));
            }
        }
    };

    // --- Blur handler ---
    let list_id_for_blur = list_id.clone();
    let handle_blur = move |e: FocusEvent| {
        // Only blur if focus left the grid list container entirely.
        if let Some(related) = e.related_target() {
            if let Ok(el) = related.dyn_into::<web_sys::Element>() {
                if let Some(container) = web_sys::window()
                    .and_then(|w| w.document())
                    .and_then(|d| d.get_element_by_id(&list_id_for_blur))
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
        // If the mousedown target is the grid list itself (scrollbar area), prevent
        // default to avoid stealing focus from focused items.
        if e.expect_target() == e.expect_current_target() {
            e.prevent_default();
        }
    };

    let state = UseGridListState {
        selection,
        focused_key: focused_key.into(),
        set_focused_key,
        is_disabled,
        selection_mode,
        selection_behavior,
        on_action,
    };

    UseGridListReturn {
        props: UseGridListProps {
            id: list_id,
            role: AriaRole::Grid,
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

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn get_next_key_moves_forward() {
        let keys = vec!["a", "b", "c"];
        let disabled = HashSet::new();
        let result = get_next_key(&keys, &disabled, &"a", true, false);
        assert_that(result).is_equal_to(Some("b"));
    }

    #[test]
    fn get_next_key_moves_backward() {
        let keys = vec!["a", "b", "c"];
        let disabled = HashSet::new();
        let result = get_next_key(&keys, &disabled, &"c", false, false);
        assert_that(result).is_equal_to(Some("b"));
    }

    #[test]
    fn get_next_key_wraps_forward() {
        let keys = vec!["a", "b", "c"];
        let disabled = HashSet::new();
        let result = get_next_key(&keys, &disabled, &"c", true, true);
        assert_that(result).is_equal_to(Some("a"));
    }

    #[test]
    fn get_next_key_wraps_backward() {
        let keys = vec!["a", "b", "c"];
        let disabled = HashSet::new();
        let result = get_next_key(&keys, &disabled, &"a", false, true);
        assert_that(result).is_equal_to(Some("c"));
    }

    #[test]
    fn get_next_key_no_wrap_at_end() {
        let keys = vec!["a", "b", "c"];
        let disabled = HashSet::new();
        let result = get_next_key(&keys, &disabled, &"c", true, false);
        assert_that(result).is_equal_to(None);
    }

    #[test]
    fn get_next_key_no_wrap_at_start() {
        let keys = vec!["a", "b", "c"];
        let disabled = HashSet::new();
        let result = get_next_key(&keys, &disabled, &"a", false, false);
        assert_that(result).is_equal_to(None);
    }

    #[test]
    fn get_next_key_skips_disabled() {
        let keys = vec!["a", "b", "c"];
        let disabled: HashSet<&str> = ["b"].into_iter().collect();
        let result = get_next_key(&keys, &disabled, &"a", true, false);
        assert_that(result).is_equal_to(Some("c"));
    }

    #[test]
    fn get_next_key_all_disabled_returns_none() {
        let keys = vec!["a", "b", "c"];
        let disabled: HashSet<&str> = ["a", "b", "c"].into_iter().collect();
        let result = get_next_key(&keys, &disabled, &"a", true, true);
        assert_that(result).is_equal_to(None);
    }

    #[test]
    fn get_next_key_empty_list() {
        let keys: Vec<&str> = vec![];
        let disabled = HashSet::new();
        let result = get_next_key(&keys, &disabled, &"a", true, false);
        assert_that(result).is_equal_to(None);
    }

    #[test]
    fn get_first_key_returns_first_non_disabled() {
        let keys = vec!["a", "b", "c"];
        let disabled: HashSet<&str> = ["a"].into_iter().collect();
        let result = get_first_key(&keys, &disabled);
        assert_that(result).is_equal_to(Some("b"));
    }

    #[test]
    fn get_last_key_returns_last_non_disabled() {
        let keys = vec!["a", "b", "c"];
        let disabled: HashSet<&str> = ["c"].into_iter().collect();
        let result = get_last_key(&keys, &disabled);
        assert_that(result).is_equal_to(Some("b"));
    }
}

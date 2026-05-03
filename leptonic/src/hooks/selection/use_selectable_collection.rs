use std::collections::HashSet;

use leptos::{
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use wasm_bindgen::JsCast;
use web_sys::{FocusEvent, KeyboardEvent, MouseEvent};

use super::{
    SelectionKey,
    keyboard_delegate::KeyboardDelegate,
    use_selection_state::{
        DisabledBehavior, FocusStrategy, Selection, SelectionBehavior, SelectionMode, SelectionSet,
        UseSelectionStateInput, UseSelectionStateReturn, use_selection_state,
    },
    use_type_select::{UseTypeSelectInput, UseTypeSelectReturn, use_type_select},
    utils::{get_item_element, is_ctrl_key_pressed, is_non_contiguous_selection_modifier_keyboard},
};
use crate::{
    hooks::{IntoAttrs, focus::use_focus_visible::get_modality},
    utils::{
        CapturedElement, EventHandler,
        dom_ext::node_contains,
        focus::focus_safely,
        focusable_tree_walker::{FocusableTreeWalkerOptions, get_focusable_tree_walker},
        locale::WritingDirection,
        scroll::{
            ScrollIntoViewOpts, ScrollIntoViewportOpts, scroll_into_view, scroll_into_viewport,
        },
        shadow_dom::get_event_target,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/selection/src/useSelectableCollection.ts

// REACT-ARIA DEVIATIONS
//
// ## DEFERRED FEATURES
//
// - No virtual focus (`shouldUseVirtualFocus` / `aria-activedescendant`).
// - No link behavior (`linkBehavior` option).
// - No `isVirtualized` / `LayoutDelegate`.
// - No `scrollRef` separate from `collection_ref`.

// Re-use the existing EscapeKeyBehavior from the grid module to avoid name conflicts.
// When the grid is migrated to use_selectable_collection, we'll consolidate this.
pub use crate::hooks::grid::use_grid::EscapeKeyBehavior;

/// Input parameters for the `use_selectable_collection` hook.
#[derive(Clone)]
#[allow(clippy::struct_excessive_bools)]
pub struct UseSelectableCollectionInput<K, D>
where
    K: SelectionKey,
    D: KeyboardDelegate<K> + Copy + Send + Sync + 'static,
{
    // --- Selection state inputs (passed to use_selection_state) ---
    /// The selection mode.
    pub selection_mode: SelectionMode,
    /// The selection behavior.
    pub selection_behavior: SelectionBehavior,
    /// Whether selection is disabled.
    pub disabled: Signal<bool>,
    /// The controlled selected keys.
    pub selected_keys: Option<Signal<Selection<K>>>,
    /// The default selected keys (uncontrolled).
    pub default_selected_keys: Option<Selection<K>>,
    /// Callback when selection changes.
    pub on_selection_change: Option<Callback<Selection<K>>>,
    /// Keys that cannot be selected.
    pub disabled_keys: Signal<HashSet<K>>,
    /// Whether to disallow empty selection.
    pub disallow_empty_selection: bool,
    /// How disabled items behave in the collection.
    pub disabled_behavior: DisabledBehavior,

    // --- Collection-level inputs ---
    /// All available keys in the collection, in order.
    pub all_keys: Signal<Vec<K>>,
    /// The keyboard delegate for navigation.
    pub keyboard_delegate: D,
    /// Element ref for the collection container (needed for focus/scroll/event filtering).
    pub collection_ref: CapturedElement,
    /// Whether keyboard navigation should wrap at boundaries.
    pub should_focus_wrap: bool,
    /// Focus strategy signal. When this becomes Some(strategy), focus moves accordingly.
    pub auto_focus: Signal<Option<FocusStrategy>>,
    /// Whether to automatically select items when they receive focus via keyboard.
    /// Defaults to `true` when `selection_behavior == Replace`.
    pub select_on_focus: Option<bool>,
    /// Whether to disallow Ctrl+A / Cmd+A select-all.
    pub disallow_select_all: bool,
    /// Behavior when Escape is pressed.
    pub escape_key_behavior: EscapeKeyBehavior,
    /// Optional callback invoked when Escape is pressed (e.g. to close an overlay).
    /// Called after any `escape_key_behavior` handling.
    pub on_close: Option<Callback<()>>,
    /// The writing direction (for RTL-aware `childFocusStrategy` on ArrowLeft/Right).
    pub direction: Signal<WritingDirection>,
    /// Whether type-ahead selection is disabled.
    pub disallow_type_ahead: bool,
    /// Function to get the text label for a key (needed for type-ahead).
    /// When `None`, type-ahead is disabled regardless of `disallow_type_ahead`.
    pub get_key_label: Option<Callback<K, String>>,
    /// Whether Tab key navigation between items is allowed.
    /// When `false` (default), the collection acts as a single tab stop — Tab/Shift+Tab
    /// moves focus out of the collection rather than between items.
    pub allows_tab_navigation: bool,
}

/// The return value of the `use_selectable_collection` hook.
#[derive(Clone)]
pub struct UseSelectableCollectionReturn<K>
where
    K: SelectionKey,
{
    /// The selection state (includes `focused_key`, `is_focused`, etc.).
    pub selection_state: UseSelectionStateReturn<K>,

    /// Extend selection from the current anchor to the given key (for Shift+Arrow/Click).
    pub extend_selection: Callback<K>,

    /// Props for the collection container element. Call `.into_attrs()` for view spreading.
    pub props: UseSelectableCollectionProps,
}

/// Props from `use_selectable_collection` for the collection container element.
#[derive(Debug, Clone)]
pub struct UseSelectableCollectionProps {
    /// Tab index for the collection container.
    /// `0` when no item is focused (so the collection is tabbable),
    /// `-1` when an item has focus (the focused item has `tabindex=0`).
    pub tabindex: Signal<i32>,
    /// Keyboard event handler.
    pub on_keydown: EventHandler<KeyboardEvent>,
    /// Focus event handler.
    pub on_focus: EventHandler<FocusEvent>,
    /// Blur event handler.
    pub on_blur: EventHandler<FocusEvent>,
    /// Mousedown event handler (prevents scrollbar click from stealing focus).
    pub on_mousedown: EventHandler<MouseEvent>,
    /// Unique collection ID for `data-collection` attribute scoping.
    /// Set `data-collection={collection_id}` on the container element.
    /// This enables `get_item_element` to scope `data-key` queries to the correct
    /// collection when collections are nested (e.g., a listbox inside a table cell).
    pub collection_id: String,
}

impl IntoAttrs for UseSelectableCollectionProps {
    type Attrs = UseSelectableCollectionAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.on_keydown.into_on(ev::keydown),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_mousedown.into_on(ev::mousedown),
        )
    }
}

/// Attributes for the collection container element.
pub type UseSelectableCollectionAttrs = (
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::mousedown, SharedEventCallback<MouseEvent>>,
);

/// Manages keyboard navigation, focus, scroll, and selection for a collection component.
///
/// This is the central orchestrator hook that combines selection state management
/// with keyboard navigation (via `KeyboardDelegate`), DOM focus management,
/// scroll-into-view, Tab key single-tab-stop behavior, and extend-selection
/// for Shift+Arrow range operations.
///
/// # Example
///
/// ```ignore
/// let delegate = ListKeyboardDelegate::new(all_keys, disabled_keys, orientation, direction);
/// let collection = use_selectable_collection(UseSelectableCollectionInput {
///     selection_mode: SelectionMode::Single,
///     all_keys,
///     keyboard_delegate: delegate,
///     collection_ref,
///     ..Default::default() // requires Default on D
/// });
///
/// view! {
///     <ul
///         role="listbox"
///         tabindex=move || collection.props.tabindex.get()
///         {..collection.props.into_attrs()}
///     >
///         // Items here
///     </ul>
/// }
/// ```
#[allow(clippy::too_many_lines)]
pub fn use_selectable_collection<K, D>(
    input: UseSelectableCollectionInput<K, D>,
) -> UseSelectableCollectionReturn<K>
where
    K: SelectionKey,
    D: KeyboardDelegate<K> + Copy + Send + Sync + 'static,
{
    let UseSelectableCollectionInput {
        selection_mode,
        selection_behavior,
        disabled,
        selected_keys,
        default_selected_keys,
        on_selection_change,
        disabled_keys,
        disallow_empty_selection,
        disabled_behavior,
        all_keys,
        keyboard_delegate: delegate,
        collection_ref,
        should_focus_wrap,
        auto_focus,
        select_on_focus,
        disallow_select_all,
        escape_key_behavior,
        on_close,
        direction,
        disallow_type_ahead,
        get_key_label,
        allows_tab_navigation,
    } = input;

    let select_on_focus =
        select_on_focus.unwrap_or(selection_behavior == SelectionBehavior::Replace);

    // Create selection state (owns focused_key, is_focused, etc.)
    let state = use_selection_state(UseSelectionStateInput {
        selection_mode,
        selection_behavior,
        disabled,
        selected_keys,
        default_selected_keys,
        on_selection_change,
        disabled_keys,
        disallow_empty_selection,
        disabled_behavior,
    });

    // --- Collection ID ---
    // Unique identifier for scoped data-key queries (prevents false matches in nested collections).
    let collection_id = uuid::Uuid::new_v4().to_string();
    let collection_id_for_scroll = collection_id.clone();

    // --- Scroll position save/restore ---
    // Save scroll position so we can restore it when focus returns to the collection.
    // Uses a simple snapshot in the mousedown handler instead of a continuous scroll
    // listener, as we only need the position from just before focus changes.
    let scroll_pos: StoredValue<(f64, f64)> = StoredValue::new((0.0, 0.0));

    // --- Type-ahead ---
    let type_select: Option<UseTypeSelectReturn> = if disallow_type_ahead {
        None
    } else {
        get_key_label.map(|label_fn| {
            use_type_select(UseTypeSelectInput {
                disabled,
                all_keys,
                get_key_label: label_fn,
                focused_key: state.focused_key,
                on_focus: Callback::new(move |key: Option<K>| {
                    state.set_focused_key.run((key, None));
                }),
                timeout_ms: 1000,
            })
        })
    };

    // --- Extend Selection ---
    // Mirrors react-aria's SelectionManager.extendSelection:
    // preserves individually-selected keys outside the range,
    // removes the old range (anchor → old current), adds the new range (anchor → to_key).
    let extend_selection = Callback::new(move |to_key: K| {
        if selection_mode == SelectionMode::None {
            return;
        }
        if selection_mode == SelectionMode::Single {
            state.replace_selection_single.run(to_key);
            return;
        }

        let current_selection = state.selected_keys.get_untracked();
        let anchor = match &current_selection {
            Selection::Keys(set) => set.anchor_key.clone().unwrap_or_else(|| to_key.clone()),
            Selection::All => to_key.clone(),
        };

        let keys = all_keys.get_untracked();
        let anchor_idx = keys.iter().position(|k| k == &anchor);
        let to_idx = keys.iter().position(|k| k == &to_key);

        if let (Some(a), Some(t)) = (anchor_idx, to_idx) {
            let disabled_set = disabled_keys.get_untracked();

            // 1. Clone existing selection (preserves Ctrl+Click'd keys outside the range)
            let mut new_keys: HashSet<K> = match &current_selection {
                Selection::Keys(set) => set.keys.clone(),
                Selection::All => keys.iter().cloned().collect(),
            };

            // 2. Remove the OLD range (anchor → old current_key)
            if let Selection::Keys(ref set) = current_selection {
                if let Some(ref old_current) = set.current_key {
                    if let Some(oc) = keys.iter().position(|k| k == old_current) {
                        let (os, oe) = if a <= oc { (a, oc) } else { (oc, a) };
                        for key in &keys[os..=oe] {
                            new_keys.remove(key);
                        }
                    }
                }
            }

            // 3. Add the NEW range (anchor → to_key), filtering disabled keys
            let (start, end) = if a <= t { (a, t) } else { (t, a) };
            for key in &keys[start..=end] {
                if !disabled_set.contains(key) {
                    new_keys.insert(key.clone());
                }
            }

            let new_selection = Selection::Keys(SelectionSet::with_anchor_current(
                new_keys,
                Some(anchor),
                Some(to_key),
            ));
            state.replace_selection.run(new_selection);
        }
    });

    // --- Navigate to key helper ---
    // This is the central navigation function that handles selection-on-focus.
    // `child_focus` is the strategy for composite items (grid cells) — ArrowLeft/Right
    // pass First/Last so the receiving item focuses the correct child.
    let navigate_to_key = move |key: K, e: &KeyboardEvent, child_focus: Option<FocusStrategy>| {
        state.set_focused_key.run((Some(key.clone()), child_focus));

        // When disabled_behavior is Selection, disabled keys can be focused
        // but should not trigger selection-on-focus.
        if disabled_behavior == DisabledBehavior::Selection
            && disabled_keys.get_untracked().contains(&key)
        {
            return;
        }

        if e.shift_key() && selection_mode == SelectionMode::Multiple {
            extend_selection.run(key);
        } else if select_on_focus && !is_non_contiguous_selection_modifier_keyboard(e) {
            state.replace_selection_single.run(key);
        }
    };

    // Helper: check if an event target is contained within the collection container.
    let is_event_in_collection = move |event_target: Option<web_sys::EventTarget>| -> bool {
        let Some(container) = collection_ref.get_untracked() else {
            return true; // No container ref yet, assume it's fine
        };
        let Some(target) = event_target else {
            return true;
        };
        let container_el: &web_sys::Element = &container;
        let container_node: &web_sys::Node = container_el.unchecked_ref();
        let Some(target_node) = target.dyn_ref::<web_sys::Node>() else {
            return true;
        };
        node_contains(Some(container_node), Some(target_node)).unwrap_or(true)
    };

    // --- Keyboard handler ---
    let on_keydown = move |e: KeyboardEvent| {
        // Ignore events from portals (events that bubble through but target is outside collection)
        if !is_event_in_collection(get_event_target(&e)) {
            return;
        }

        // Alt+Tab: prevent default
        if e.alt_key() && e.key() == "Tab" {
            e.prevent_default();
            return;
        }

        let focused = state.focused_key.get_untracked();

        match e.key().as_str() {
            "ArrowDown" => {
                if let Some(next_key) = focused
                    .as_ref()
                    .and_then(|fk| delegate.get_key_below(fk))
                    .or_else(|| delegate.get_first_key(None, false))
                {
                    e.prevent_default();
                    navigate_to_key(next_key, &e, None);
                } else if should_focus_wrap {
                    if let Some(first) = delegate.get_first_key(focused.as_ref(), false) {
                        e.prevent_default();
                        navigate_to_key(first, &e, None);
                    }
                }
            }
            "ArrowUp" => {
                if let Some(next_key) = focused
                    .as_ref()
                    .and_then(|fk| delegate.get_key_above(fk))
                    .or_else(|| delegate.get_last_key(None, false))
                {
                    e.prevent_default();
                    navigate_to_key(next_key, &e, None);
                } else if should_focus_wrap {
                    if let Some(last) = delegate.get_last_key(focused.as_ref(), false) {
                        e.prevent_default();
                        navigate_to_key(last, &e, None);
                    }
                }
            }
            "ArrowLeft" => {
                let is_rtl = direction.get_untracked() == WritingDirection::Rtl;
                let child_focus = Some(if is_rtl {
                    FocusStrategy::First
                } else {
                    FocusStrategy::Last
                });
                if let Some(next_key) = focused.as_ref().and_then(|fk| delegate.get_key_left_of(fk))
                {
                    e.prevent_default();
                    navigate_to_key(next_key, &e, child_focus);
                } else if should_focus_wrap {
                    // RTL-aware wrapping is handled by the delegate
                    if let Some(key) = delegate.get_last_key(focused.as_ref(), false) {
                        e.prevent_default();
                        navigate_to_key(key, &e, child_focus);
                    }
                }
            }
            "ArrowRight" => {
                let is_rtl = direction.get_untracked() == WritingDirection::Rtl;
                let child_focus = Some(if is_rtl {
                    FocusStrategy::Last
                } else {
                    FocusStrategy::First
                });
                if let Some(next_key) = focused
                    .as_ref()
                    .and_then(|fk| delegate.get_key_right_of(fk))
                {
                    e.prevent_default();
                    navigate_to_key(next_key, &e, child_focus);
                } else if should_focus_wrap {
                    if let Some(key) = delegate.get_first_key(focused.as_ref(), false) {
                        e.prevent_default();
                        navigate_to_key(key, &e, child_focus);
                    }
                }
            }
            "Home" => {
                if focused.is_none() && e.shift_key() {
                    return;
                }
                if let Some(first_key) =
                    delegate.get_first_key(focused.as_ref(), is_ctrl_key_pressed(&e))
                {
                    e.prevent_default();
                    state.set_focused_key.run((Some(first_key.clone()), None));
                    if is_ctrl_key_pressed(&e)
                        && e.shift_key()
                        && selection_mode == SelectionMode::Multiple
                    {
                        extend_selection.run(first_key);
                    } else if select_on_focus {
                        state.replace_selection_single.run(first_key);
                    }
                }
            }
            "End" => {
                if focused.is_none() && e.shift_key() {
                    return;
                }
                if let Some(last_key) =
                    delegate.get_last_key(focused.as_ref(), is_ctrl_key_pressed(&e))
                {
                    e.prevent_default();
                    state.set_focused_key.run((Some(last_key.clone()), None));
                    if is_ctrl_key_pressed(&e)
                        && e.shift_key()
                        && selection_mode == SelectionMode::Multiple
                    {
                        extend_selection.run(last_key);
                    } else if select_on_focus {
                        state.replace_selection_single.run(last_key);
                    }
                }
            }
            "PageDown" => {
                if let Some(fk) = &focused {
                    if let Some(next_key) = delegate.get_key_page_below(fk) {
                        e.prevent_default();
                        navigate_to_key(next_key, &e, None);
                    }
                }
            }
            "PageUp" => {
                if let Some(fk) = &focused {
                    if let Some(next_key) = delegate.get_key_page_above(fk) {
                        e.prevent_default();
                        navigate_to_key(next_key, &e, None);
                    }
                }
            }
            " " | "Enter" => {
                // Skip if already handled by an item's use_press keydown handler.
                if e.default_prevented() {
                    return;
                }
                if let Some(ref key) = focused {
                    if state.can_select_item.run(key.clone()) {
                        e.prevent_default();
                        if selection_mode == SelectionMode::Single {
                            state.replace_selection_single.run(key.clone());
                        } else if selection_mode == SelectionMode::Multiple {
                            state.toggle.run(key.clone());
                        }
                    }
                }
            }
            "a" if is_ctrl_key_pressed(&e)
                && selection_mode == SelectionMode::Multiple
                && !disallow_select_all =>
            {
                e.prevent_default();
                state.select_all.run(Vec::new());
            }
            "Tab" => {
                if !allows_tab_navigation {
                    if e.shift_key() {
                        // Shift+Tab: clear focused key so the container gets tabindex=0,
                        // then focus the container. The browser's default Shift+Tab then
                        // moves focus to the element before the collection.
                        if let Some(container) = collection_ref.get_untracked() {
                            if let Some(el) = container.dyn_ref::<web_sys::HtmlElement>() {
                                state.set_focused_key.run((None, None));
                                focus_safely(el);
                            }
                        }
                    } else {
                        // Tab: focus the last tabbable element inside the collection.
                        // The browser's default Tab then moves focus to the next element
                        // after the collection, completing the single-tab-stop behavior.
                        if let Some(container) = collection_ref.get_untracked() {
                            if let Some(walker) = get_focusable_tree_walker(
                                container.as_ref(),
                                FocusableTreeWalkerOptions {
                                    tabbable: true,
                                    ..FocusableTreeWalkerOptions::default()
                                },
                            ) {
                                let mut last = None;
                                let mut w = walker;
                                while let Some(node) = w.last_child() {
                                    last = Some(node);
                                }
                                if let Some(last_node) = last {
                                    if let Some(el) = last_node.dyn_ref::<web_sys::HtmlElement>() {
                                        let _ = el.focus();
                                    }
                                }
                            }
                        }
                    }
                    // Do NOT prevent default — let the browser's Tab behavior continue
                    // from the new focus position.
                }
            }
            "Escape" => {
                if escape_key_behavior == EscapeKeyBehavior::ClearSelection
                    && !disallow_empty_selection
                    && !state.selected_keys.get_untracked().is_empty()
                {
                    e.stop_propagation();
                    e.prevent_default();
                    state.clear_selection.run(());
                }
                if let Some(on_close) = on_close {
                    e.prevent_default();
                    on_close.run(());
                }
            }
            _ => {
                // Delegate to type-select for character-based navigation.
                if let Some(ref ts) = type_select {
                    ts.on_keydown.run(e);
                }
            }
        }
    };

    // --- Focus handler ---
    let on_focus = move |e: FocusEvent| {
        // Ignore events from portals
        if !is_event_in_collection(e.target()) {
            return;
        }

        if state.is_focused.get_untracked() {
            return;
        }

        state.set_focused.run(true);

        // Restore scroll position to prevent jump on focus.
        if let Some(container) = collection_ref.get_untracked() {
            if let Some(html_el) = container.dyn_ref::<web_sys::HtmlElement>() {
                let (top, left) = scroll_pos.get_value();
                html_el.set_scroll_top(top);
                html_el.set_scroll_left(left);
            }
        }

        if state.focused_key.get_untracked().is_none() {
            // Determine if user is tabbing forward or backward into the collection
            let should_focus_last = e.related_target().is_some_and(|related| {
                if let Some(current_target) = e.current_target() {
                    if let (Some(current_node), Some(related_node)) = (
                        current_target.dyn_ref::<web_sys::Node>(),
                        related.dyn_ref::<web_sys::Node>(),
                    ) {
                        let position = current_node.compare_document_position(related_node);
                        // DOCUMENT_POSITION_FOLLOWING = 4
                        return position & 4 != 0;
                    }
                }
                false
            });

            // Focus first/last selected key, or first/last key.
            // Mirrors react-aria: manager.lastSelectedKey ?? delegate.getLastKey()
            let key = if should_focus_last {
                {
                    let sel = state.selected_keys.get_untracked();
                    let ks = all_keys.get_untracked();
                    match &sel {
                        Selection::Keys(set) if !set.is_empty() => {
                            ks.iter().rev().find(|k| set.contains(*k)).cloned()
                        }
                        Selection::All => ks.last().cloned(),
                        Selection::Keys(_) => None,
                    }
                }
                .or_else(|| delegate.get_last_key(None, false))
            } else {
                {
                    let sel = state.selected_keys.get_untracked();
                    let ks = all_keys.get_untracked();
                    match &sel {
                        Selection::Keys(set) if !set.is_empty() => {
                            ks.iter().find(|k| set.contains(*k)).cloned()
                        }
                        Selection::All => ks.first().cloned(),
                        Selection::Keys(_) => None,
                    }
                }
                .or_else(|| delegate.get_first_key(None, false))
            };

            if let Some(key) = key {
                state.set_focused_key.run((Some(key.clone()), None));
                if select_on_focus {
                    state.replace_selection_single.run(key);
                }
            }
        }
    };

    // --- Blur handler ---
    let on_blur = move |e: FocusEvent| {
        // Don't set blurred if moving focus within the collection
        let stays_within = e.related_target().is_some_and(|related| {
            if let Some(current_target) = e.current_target() {
                if let (Some(container_node), Some(related_node)) = (
                    current_target.dyn_ref::<web_sys::Node>(),
                    related.dyn_ref::<web_sys::Node>(),
                ) {
                    return container_node.contains(Some(related_node));
                }
            }
            false
        });

        if !stays_within {
            // Save scroll position before losing focus, so we can restore it
            // when focus returns (prevents scroll jump).
            if let Some(container) = collection_ref.get_untracked() {
                if let Some(html_el) = container.dyn_ref::<web_sys::HtmlElement>() {
                    scroll_pos.set_value((html_el.scroll_top(), html_el.scroll_left()));
                }
            }
            state.set_focused.run(false);
        }
    };

    // --- Mousedown handler (prevent scrollbar click from stealing focus) ---
    let on_mousedown = move |e: MouseEvent| {
        if let Some(container) = collection_ref.get_untracked() {
            if let Some(target) = get_event_target(&e) {
                // If the click target is the scroll container itself (not a child),
                // that means the user clicked the scrollbar area.
                if target.dyn_ref::<web_sys::Element>() == Some(container.as_ref()) {
                    e.prevent_default();
                }
            }
        }
    };

    // --- Tab index ---
    let tabindex = Signal::derive(move || {
        if state.focused_key.get().is_none() {
            0
        } else {
            -1
        }
    });

    // --- Scroll-into-view effect ---
    // When focused key changes, scroll the item into view
    {
        let prev_focused_key: StoredValue<Option<K>> = StoredValue::new(None);
        Effect::new(move |_| {
            let current_focused = state.focused_key.get();
            let is_focused = state.is_focused.get_untracked();

            if is_focused {
                if let Some(ref key) = current_focused {
                    let prev = prev_focused_key.get_value();
                    if prev.as_ref() != Some(key) {
                        // Only scroll on keyboard modality
                        if matches!(
                            get_modality(),
                            crate::hooks::focus::use_focus_visible::Modality::Keyboard
                        ) {
                            if let Some(container) = collection_ref.get_untracked() {
                                // Find the item element by data-key
                                if let Some(element) = get_item_element(
                                    &container,
                                    &format!("{key}"),
                                    Some(&collection_id_for_scroll),
                                ) {
                                    if let (Some(container_html), Some(element_html)) = (
                                        container.dyn_ref::<web_sys::HtmlElement>(),
                                        element.dyn_ref::<web_sys::HtmlElement>(),
                                    ) {
                                        scroll_into_view(
                                            container_html,
                                            element_html,
                                            ScrollIntoViewOpts::default(),
                                        );
                                        scroll_into_viewport(
                                            Some(&element),
                                            &ScrollIntoViewportOpts::default(),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }

            prev_focused_key.set_value(current_focused);
        });
    }

    // --- Auto-focus effect ---
    // When auto_focus signal changes to Some, apply the focus strategy.
    // Enhanced: checks selected keys first (focus first selected), then falls back.
    Effect::new(move |_| {
        if let Some(strategy) = auto_focus.get() {
            let keys = all_keys.get();
            if keys.is_empty() {
                return;
            }

            // Check if there are selected keys to focus first
            let selected = state.selected_keys.get_untracked();
            let first_selected = match &selected {
                Selection::Keys(set) => {
                    // Find first selected key that can be selected
                    keys.iter()
                        .find(|k| set.contains(*k) && state.can_select_item.run((*k).clone()))
                        .cloned()
                }
                Selection::All => keys
                    .iter()
                    .find(|k| state.can_select_item.run((*k).clone()))
                    .cloned(),
            };

            let focused_key = first_selected.or_else(|| match strategy {
                FocusStrategy::First => delegate.get_first_key(None, false),
                FocusStrategy::Last => delegate.get_last_key(None, false),
            });

            state.set_focused.run(true);
            state.set_focused_key.run((focused_key, None));
        }
    });

    // --- If focused key becomes None while focused, focus the collection container ---
    {
        let prev_focused_key2: StoredValue<Option<K>> = StoredValue::new(None);
        Effect::new(move |_| {
            let current = state.focused_key.get();
            let was_some = prev_focused_key2.get_value().is_some();

            if state.is_focused.get_untracked() && current.is_none() && was_some {
                if let Some(container) = collection_ref.get_untracked() {
                    if let Some(el) = container.dyn_ref::<web_sys::HtmlElement>() {
                        focus_safely(el);
                    }
                }
            }

            prev_focused_key2.set_value(current);
        });
    }

    UseSelectableCollectionReturn {
        selection_state: state,
        extend_selection,
        props: UseSelectableCollectionProps {
            tabindex,
            on_keydown: EventHandler::new(on_keydown),
            on_focus: EventHandler::new(on_focus),
            on_blur: EventHandler::new(on_blur),
            on_mousedown: EventHandler::new(on_mousedown),
            collection_id,
        },
    }
}

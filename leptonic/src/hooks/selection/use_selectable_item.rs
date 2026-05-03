use leptos::{
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use web_sys::{FocusEvent, MouseEvent};

use super::{
    SelectionKey,
    use_selection_state::{DisabledBehavior, Selection, SelectionBehavior, SelectionMode},
    utils::is_non_contiguous_selection_modifier,
};
use crate::{
    hooks::{
        IntoAttrs, PropsWithStyles,
        interactions::use_press::{
            LongPressEvent, PressEvent, UsePressAttrs, UsePressInput, UsePressProps, use_press,
        },
    },
    utils::{EventHandler, pointer_type::PointerType, propagation_control::Propagation},
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/selection/src/useSelectableItem.ts

// REACT-ARIA DEVIATIONS
//
// ## DIFFERENT BEHAVIOR
//
// - In react-aria's `useSelectableItem`, double-click triggers the `onAction`
//   callback provided to the collection (e.g. `useListBox`). We expose it (`on_double_click`)
//   directly on the item input so callers can wire it without a full collection.
//
// ## DEFERRED FEATURES
//
// - No link behavior handling.
// - No primary/secondary action model (hasPrimaryAction/hasSecondaryAction).
//   `on_action` is always called alongside selection; the conditional routing
//   where single-click triggers action in Replace mode is not implemented.
//
// ## INTEGRATION NOTES
//
// - `use_press` is called with `force_propagation: true` so keyboard/pointer
//   events still bubble to `use_selectable_collection` for arrow navigation.
//   `use_press` calls `prevent_default()` on Enter/Space keydown, which the
//   collection's handler checks via `e.default_prevented()` to avoid
//   double-firing selection.
// - Touch/virtual pointer types always use toggle selection behavior,
//   matching react-aria's logic for devices without modifier keys.
// - Long press on touch triggers selection and switches to toggle mode
//   when `on_selection_behavior_change` is provided.

/// Input parameters for the `use_selectable_item` hook.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone)]
pub struct UseSelectableItemInput<K>
where
    K: SelectionKey,
{
    /// The key for this item.
    pub key: K,

    /// The selection mode.
    pub selection_mode: SelectionMode,

    /// The selection behavior (reactive — may change at runtime via touch long press).
    pub selection_behavior: Signal<SelectionBehavior>,

    /// The current selection.
    pub selected_keys: Signal<Selection<K>>,

    /// The currently focused key.
    pub focused_key: Signal<Option<K>>,

    /// Whether the collection is focused (needed for DOM focus sync).
    pub is_collection_focused: Signal<bool>,

    /// Whether this item is disabled.
    pub is_disabled: Signal<bool>,

    /// How disabled items behave.
    pub disabled_behavior: DisabledBehavior,

    /// Whether empty selection is disallowed.
    pub disallow_empty_selection: bool,

    /// Callback to toggle selection for a key.
    pub on_toggle: Callback<K>,

    /// Callback to replace selection with a single key (sets anchor/current).
    pub on_replace: Callback<K>,

    /// Callback to extend selection from anchor to the given key (Shift+Click).
    pub on_extend: Option<Callback<K>>,

    /// Called when the item is activated (e.g., by a double-click).
    pub on_double_click: Option<Callback<K>>,

    /// Callback to set focused key.
    pub on_focus: Callback<Option<K>>,

    /// Whether selection should occur on pointer-up instead of pointer-down.
    /// When `false` (default): selection on press start (pointerdown/keyboard).
    /// When `true`: selection on press/press-up (for menus/selects).
    pub should_select_on_press_up: bool,

    /// Whether the item should receive focus on mouse hover.
    pub should_focus_on_hover: bool,

    /// Whether to allow drag operations.
    pub allow_drag: bool,

    /// Whether the press can originate from a different element (e.g., menu trigger).
    /// When `true`, selection fires on `on_press_up` instead of `on_press`, enabling
    /// the hold-to-select pattern in dropdown menus.
    pub allows_different_press_origin: bool,

    /// Optional callback invoked when the item is pressed (alongside selection).
    pub on_action: Option<Callback<()>>,

    /// Callback to change selection behavior at runtime (e.g., touch long press
    /// switches from `Replace` to `Toggle`). When provided, long press on touch
    /// will trigger selection and switch to toggle mode.
    pub on_selection_behavior_change: Option<Callback<SelectionBehavior>>,

    /// Optional callback to programmatically focus this item's DOM element.
    ///
    /// When provided, an Effect is created that calls this callback each time
    /// `is_focused` transitions from `false` to `true` while the collection is focused.
    pub focus: Option<Callback<()>>,

    /// The data-key attribute value for this item. Used for DOM element lookup
    /// by `use_selectable_collection`'s scroll-into-view functionality.
    /// If `None`, no data-key attribute is emitted.
    pub data_key: Option<String>,
}

/// Props from `use_selectable_item` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseSelectableItemProps {
    /// Press props (keydown, click, pointerdown, pointerup, mousedown, dragstart, dblclick).
    pub press: UsePressProps,
    /// Focus handler for the item.
    pub on_focus: EventHandler<FocusEvent>,
    /// Mouseenter handler for hover-to-focus.
    pub on_mouseenter: EventHandler<MouseEvent>,
}

impl IntoAttrs for UseSelectableItemProps {
    type Attrs = UseSelectableItemAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.press.into_attrs(),
            self.on_focus.into_on(ev::focus),
            self.on_mouseenter.into_on(ev::mouseenter),
        )
    }
}

/// The return value of the `use_selectable_item` hook.
pub struct UseSelectableItemReturn {
    /// Props for the item element. Call `.into_parts()` for view spreading and styles.
    pub props: PropsWithStyles<UseSelectableItemProps>,

    /// Whether this item is currently selected.
    pub is_selected: Signal<bool>,

    /// Whether this item is currently focused.
    pub is_focused: Signal<bool>,

    /// Whether this item is currently pressed.
    pub is_pressed: Signal<bool>,

    /// Whether this item is disabled.
    pub is_disabled: Signal<bool>,

    /// The tabindex for this item. `0` if focused, `-1` otherwise.
    pub tabindex: Signal<i32>,

    /// The data-key attribute value for DOM querying.
    pub data_key: String,
}

/// Attributes for a selectable item element.
pub type UseSelectableItemAttrs = (
    UsePressAttrs,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::mouseenter, SharedEventCallback<MouseEvent>>,
);

/// Manages selection and focus for an individual item in a collection.
///
/// This hook uses `use_press` internally for unified press event handling,
/// supporting mouse, touch, keyboard, and assistive technology interactions.
///
/// # Example
///
/// ```ignore
/// let item = use_selectable_item(UseSelectableItemInput {
///     key: "apple".to_string(),
///     selection_mode: SelectionMode::Single,
///     selection_behavior: SelectionBehavior::Toggle,
///     selected_keys: state.selected_keys,
///     focused_key: state.focused_key,
///     is_collection_focused: state.is_focused,
///     is_disabled: Signal::derive(|| false),
///     disallow_empty_selection: false,
///     on_toggle: state.toggle,
///     on_replace: state.replace_selection_single,
///     on_extend: Some(extend_selection),
///     on_double_click: None,
///     on_focus: Callback::new(move |k| state.set_focused_key.run((k, None))),
///     should_select_on_press_up: false,
///     should_focus_on_hover: false,
///     allow_drag: false,
///     allows_different_press_origin: false,
///     on_action: None,
///     focus: None,
///     data_key: None,
/// });
///
/// view! {
///     <li
///         role="option"
///         tabindex=move || item.tabindex.get()
///         data-key=item.data_key.clone()
///         aria-selected=move || item.is_selected.get()
///         {..item.props.into_attrs()}
///     >
///         "Apple"
///     </li>
/// }
/// ```
#[allow(clippy::needless_pass_by_value, clippy::too_many_lines)]
pub fn use_selectable_item<K>(input: UseSelectableItemInput<K>) -> UseSelectableItemReturn
where
    K: SelectionKey,
{
    let UseSelectableItemInput {
        key,
        selection_mode,
        selection_behavior,
        selected_keys,
        focused_key,
        is_collection_focused,
        is_disabled,
        disabled_behavior,
        disallow_empty_selection,
        on_toggle,
        on_replace,
        on_extend,
        on_double_click,
        on_focus,
        should_select_on_press_up,
        should_focus_on_hover,
        allow_drag: _allow_drag,
        allows_different_press_origin,
        on_action,
        on_selection_behavior_change,
        focus,
        data_key,
    } = input;

    let data_key = data_key.unwrap_or_default();

    // Store the key so all closures can access it without ownership issues.
    let stored_key: StoredValue<K> = StoredValue::new(key.clone());

    // Compute whether this item is selected
    let key_for_selected = key.clone();
    let is_selected = Signal::derive(move || {
        let selection = selected_keys.get();
        match selection {
            Selection::Keys(keys) => keys.contains(&key_for_selected),
            Selection::All => true,
        }
    });

    // Compute whether this item is focused
    let key_for_focused = key;
    let is_focused = Signal::derive(move || focused_key.get().as_ref() == Some(&key_for_focused));

    // Tab index: 0 if this item is focused, -1 otherwise
    let tabindex = Signal::derive(move || if is_focused.get() { 0 } else { -1 });

    // --- DOM focus synchronization ---
    // When this item becomes focused and the collection is focused,
    // call the focus callback to synchronize DOM focus.
    if let Some(focus_fn) = focus {
        Effect::new(move |prev_focused: Option<bool>| {
            let currently_focused = is_focused.get();
            let was_focused = prev_focused.unwrap_or(false);
            if currently_focused && !was_focused && is_collection_focused.get_untracked() {
                focus_fn.run(());
            }
            currently_focused
        });
    }

    // --- Disabled item auto-unfocus ---
    // Mirrors react-aria: if the focused item becomes disabled, clear focus.
    {
        let unfocus_key = stored_key;
        Effect::new(move |_| {
            if is_disabled.get() && focused_key.get().as_ref() == Some(&unfocus_key.get_value()) {
                on_focus.run(None);
            }
        });
    }

    // --- Selection logic ---
    // Shared helper for performing selection with modifier support.
    // All callbacks/signals are Copy; only the key needs StoredValue.
    let perform_selection = move |e: &PressEvent| {
        let key = stored_key.get_value();
        let modifiers = &e.modifiers;

        on_focus.run(Some(key.clone()));

        // When disabled_behavior is Selection, disabled items can be focused
        // and have actions but cannot be selected.
        if is_disabled.get_untracked() && disabled_behavior == DisabledBehavior::Selection {
            if let Some(action) = on_action {
                action.run(());
            }
            return;
        }

        if selection_mode == SelectionMode::None {
            if let Some(action) = on_action {
                action.run(());
            }
            return;
        }

        // Touch/virtual always toggles — no modifier keys available on touch devices.
        // Mirrors react-aria: `e.pointerType === 'touch' || e.pointerType === 'virtual'`
        if matches!(e.pointer_type, PointerType::Touch | PointerType::Virtual) {
            if selection_mode == SelectionMode::Single {
                if is_selected.get_untracked() && !disallow_empty_selection {
                    on_toggle.run(key.clone());
                } else {
                    on_replace.run(key.clone());
                }
            } else {
                on_toggle.run(key.clone());
            }
            if let Some(action) = on_action {
                action.run(());
            }
            return;
        }

        // Shift: extend selection (multiple mode only)
        if modifiers.shift_key && selection_mode == SelectionMode::Multiple {
            if let Some(extend) = on_extend {
                extend.run(key);
                if let Some(action) = on_action {
                    action.run(());
                }
                return;
            }
        }

        // Ctrl/Cmd (or Alt on Mac): toggle in multiple mode
        if is_non_contiguous_selection_modifier(modifiers)
            && selection_mode == SelectionMode::Multiple
        {
            on_toggle.run(key);
            if let Some(action) = on_action {
                action.run(());
            }
            return;
        }

        // Standard selection based on mode and behavior
        match selection_mode {
            SelectionMode::None => {}
            SelectionMode::Single => {
                if is_selected.get_untracked() && !disallow_empty_selection {
                    on_toggle.run(key);
                } else {
                    on_replace.run(key);
                }
            }
            SelectionMode::Multiple => match selection_behavior.get_untracked() {
                SelectionBehavior::Toggle => on_toggle.run(key),
                SelectionBehavior::Replace => on_replace.run(key),
            },
        }

        if let Some(action) = on_action {
            action.run(());
        }
    };

    // --- Build press callbacks based on should_select_on_press_up ---

    let on_press_start_handler = if should_select_on_press_up {
        // Menu/select pattern: just set focus on press start, no selection yet.
        Callback::new(move |e: PressEvent| {
            on_focus.run(Some(stored_key.get_value()));
            e.continue_propagation();
        })
    } else {
        // Default pattern: select immediately on press start for mouse/keyboard.
        // Touch waits for on_press (to avoid selecting during scroll gestures).
        Callback::new(move |e: PressEvent| {
            if matches!(e.pointer_type, PointerType::Mouse | PointerType::Keyboard) {
                perform_selection(&e);
            } else {
                // Touch/pen/virtual: just set focus, selection on on_press
                on_focus.run(Some(stored_key.get_value()));
            }
            e.continue_propagation();
        })
    };

    let on_press_handler = if should_select_on_press_up {
        if allows_different_press_origin {
            // Hold-to-select pattern: selection happens in on_press_up, not on_press.
            Callback::new(move |e: PressEvent| {
                e.continue_propagation();
            })
        } else {
            // Standard menu pattern: select on press (pointer-up timing).
            Callback::new(move |e: PressEvent| {
                perform_selection(&e);
                e.continue_propagation();
            })
        }
    } else {
        // Default pattern: handle touch/pen/virtual here (mouse/keyboard already handled in press_start).
        Callback::new(move |e: PressEvent| {
            if !matches!(e.pointer_type, PointerType::Mouse | PointerType::Keyboard) {
                perform_selection(&e);
            }
            e.continue_propagation();
        })
    };

    let on_press_up_handler = if should_select_on_press_up && allows_different_press_origin {
        Some(Callback::new(move |e: PressEvent| {
            perform_selection(&e);
            e.continue_propagation();
        }))
    } else {
        None
    };

    // --- use_press integration ---
    let press = use_press(UsePressInput {
        disabled: is_disabled,
        force_prevent_default: false,
        force_propagation: true,
        allow_text_selection_on_press: false,
        should_cancel_on_pointer_exit: false,
        prevent_focus_on_press: false,
        force_is_pressed: None,
        on_press: on_press_handler,
        on_press_start: Some(on_press_start_handler),
        on_press_up: on_press_up_handler,
        on_press_end: None,
        on_press_change: None,
        on_double_press: on_double_click.map(|cb| {
            Callback::new(move |e: PressEvent| {
                cb.run(stored_key.get_value());
                e.continue_propagation();
            })
        }),
        on_long_press_start: None,
        on_long_press: on_selection_behavior_change.map(|set_behavior| {
            Callback::new(move |e: LongPressEvent| {
                // Touch long press switches Replace → Toggle mode, matching react-aria's
                // behavior for entering multi-select mode on mobile devices.
                if e.pointer_type == PointerType::Touch
                    && selection_behavior.get_untracked() == SelectionBehavior::Replace
                {
                    // Select the current item and switch to toggle mode
                    let key = stored_key.get_value();
                    on_focus.run(Some(key.clone()));
                    on_toggle.run(key);
                    set_behavior.run(SelectionBehavior::Toggle);
                }
            })
        }),
        on_long_press_end: None,
        long_press_threshold: None,
        long_press_accessibility_description: None,
    });

    // --- Focus handler ---
    // Only set focused key when the item itself receives focus (not child elements).
    let handle_focus = move |e: FocusEvent| {
        if is_disabled.get_untracked() {
            return;
        }
        if e.target() == e.current_target() {
            on_focus.run(Some(stored_key.get_value()));
        }
    };

    // --- Mouseenter handler (hover-to-focus) ---
    let handle_mouseenter = move |_e: MouseEvent| {
        if !should_focus_on_hover || is_disabled.get_untracked() {
            return;
        }
        on_focus.run(Some(stored_key.get_value()));
    };

    let (press_props, press_styles) = press.props.into_inner();
    UseSelectableItemReturn {
        props: PropsWithStyles::new(
            UseSelectableItemProps {
                press: press_props,
                on_focus: EventHandler::new(handle_focus),
                on_mouseenter: EventHandler::new(handle_mouseenter),
            },
            press_styles,
        ),
        is_selected,
        is_focused,
        is_pressed: press.is_pressed,
        is_disabled,
        tabindex,
        data_key,
    }
}

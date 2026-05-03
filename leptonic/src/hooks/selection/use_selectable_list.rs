use std::collections::HashSet;

use leptos::prelude::*;
use web_sys::KeyboardEvent;

use super::{
    SelectionKey,
    list_keyboard_delegate::ListKeyboardDelegate,
    use_selectable_collection::{
        EscapeKeyBehavior, UseSelectableCollectionInput, UseSelectableCollectionProps,
        UseSelectableCollectionReturn, use_selectable_collection,
    },
    use_selection_state::{
        DisabledBehavior, FocusStrategy, Selection, SelectionBehavior, SelectionMode,
    },
};
use crate::{
    hooks::form::use_checkbox_group::Orientation,
    utils::{CapturedElement, locale::WritingDirection},
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/selection/src/useSelectableList.ts

// REACT-ARIA DEVIATIONS
//
// No intentional deviations from the react-aria implementation.

/// Input parameters for the `use_selectable_list` hook.
#[derive(Clone)]
#[allow(clippy::struct_excessive_bools)]
pub struct UseSelectableListInput<K>
where
    K: SelectionKey,
{
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

    /// Whether to allow empty selection.
    pub disallow_empty_selection: bool,

    /// How disabled items behave in the collection.
    pub disabled_behavior: DisabledBehavior,

    /// All available keys in the list.
    pub all_keys: Signal<Vec<K>>,

    /// Whether keyboard navigation should wrap.
    pub should_focus_wrap: bool,

    /// Focus strategy signal. When this becomes Some(strategy), focus moves accordingly.
    /// This is reactive - whenever the signal changes to Some, focus will be applied.
    pub auto_focus: Signal<Option<FocusStrategy>>,

    /// Whether to select items on focus. Defaults to `true` when `selection_behavior == Replace`.
    pub select_on_focus: Option<bool>,

    /// The orientation of the list (affects which arrow keys navigate).
    pub orientation: Orientation,

    /// The writing direction (for RTL support in horizontal lists).
    pub direction: Signal<WritingDirection>,

    /// Element ref for the list container.
    pub collection_ref: CapturedElement,

    /// Behavior when Escape is pressed.
    pub escape_key_behavior: EscapeKeyBehavior,

    /// Whether to disallow Ctrl+A / Cmd+A select-all.
    pub disallow_select_all: bool,

    /// Optional callback invoked when Escape is pressed (e.g. to close an overlay).
    pub on_close: Option<Callback<()>>,

    /// Whether type-ahead selection is disabled.
    pub disallow_type_ahead: bool,

    /// Function to get the text label for a key (needed for type-ahead).
    /// When `None`, type-ahead is disabled regardless of `disallow_type_ahead`.
    pub get_key_label: Option<Callback<K, String>>,

    /// Whether Tab key navigation between items is allowed.
    /// When `false` (default), the collection acts as a single tab stop.
    pub allows_tab_navigation: bool,
}

impl<K: SelectionKey> Default for UseSelectableListInput<K> {
    fn default() -> Self {
        Self {
            selection_mode: SelectionMode::Single,
            selection_behavior: SelectionBehavior::Toggle,
            disabled: Signal::derive(|| false),
            selected_keys: None,
            default_selected_keys: None,
            on_selection_change: None,
            disabled_keys: Signal::derive(HashSet::new),
            disallow_empty_selection: false,
            disabled_behavior: DisabledBehavior::default(),
            all_keys: Signal::derive(Vec::new),
            should_focus_wrap: true,
            auto_focus: Signal::derive(|| None),
            select_on_focus: None,
            orientation: Orientation::Vertical,
            direction: Signal::derive(|| WritingDirection::Ltr),
            collection_ref: CapturedElement::new(),
            escape_key_behavior: EscapeKeyBehavior::default(),
            disallow_select_all: false,
            on_close: None,
            disallow_type_ahead: false,
            get_key_label: None,
            allows_tab_navigation: false,
        }
    }
}

/// The return value of the `use_selectable_list` hook.
#[derive(Clone)]
pub struct UseSelectableListReturn<K>
where
    K: SelectionKey,
{
    /// The collection state with selection and focus management.
    pub collection: UseSelectableCollectionReturn<K>,

    /// Props for the list container element.
    /// Includes keyboard, focus, blur, and mousedown handlers.
    /// Call `.into_attrs()` for view spreading.
    pub list_props: UseSelectableCollectionProps,

    /// The keyboard event handler callback. Can be called directly to delegate keyboard handling.
    pub on_keydown: Callback<KeyboardEvent>,
}

/// Manages keyboard navigation and selection for a list component.
///
/// This is a thin wrapper around `use_selectable_collection` that creates
/// a `ListKeyboardDelegate` for linear list navigation and passes it through.
///
/// # Example
///
/// ```ignore
/// let items = vec!["apple", "banana", "cherry"];
/// let all_keys = Signal::derive(move || items.iter().map(|s| s.to_string()).collect());
/// let collection_ref = CapturedElement::new();
///
/// let list = use_selectable_list(UseSelectableListInput {
///     selection_mode: SelectionMode::Single,
///     all_keys,
///     collection_ref,
///     ..Default::default()
/// });
///
/// view! {
///     <ul
///         role="listbox"
///         tabindex=move || list.list_props.tabindex.get()
///         {..list.list_props.into_attrs()}
///         {..collection_ref.attr()}
///     >
///         // Items here
///     </ul>
/// }
/// ```
pub fn use_selectable_list<K>(input: UseSelectableListInput<K>) -> UseSelectableListReturn<K>
where
    K: SelectionKey,
{
    let UseSelectableListInput {
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
        should_focus_wrap,
        auto_focus,
        select_on_focus,
        orientation,
        direction,
        collection_ref,
        escape_key_behavior,
        disallow_select_all,
        on_close,
        disallow_type_ahead,
        get_key_label,
        allows_tab_navigation,
    } = input;

    // Create a ListKeyboardDelegate for linear list navigation
    let delegate = ListKeyboardDelegate::new(all_keys, disabled_keys, orientation, direction)
        .with_disabled_behavior(disabled_behavior);

    // Delegate everything to use_selectable_collection
    let collection = use_selectable_collection(UseSelectableCollectionInput {
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
    });

    // Extract a forwarding callback for backward compatibility
    let props_keydown = collection.props.on_keydown.clone();
    let on_keydown = Callback::new(move |e: KeyboardEvent| {
        props_keydown.call(e);
    });

    let list_props = collection.props.clone();

    UseSelectableListReturn {
        collection,
        list_props,
        on_keydown,
    }
}

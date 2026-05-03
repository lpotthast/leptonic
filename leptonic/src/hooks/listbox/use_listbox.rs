use std::collections::HashSet;

use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use uuid::Uuid;
use web_sys::KeyboardEvent;

use crate::{
    hooks::{
        IntoAttrs,
        form::use_checkbox_group::Orientation,
        selection::{
            SelectionKey,
            use_selectable_collection::EscapeKeyBehavior,
            use_selectable_list::{
                UseSelectableListInput, UseSelectableListReturn, use_selectable_list,
            },
            use_selection_state::FocusStrategy,
            use_selection_state::{DisabledBehavior, Selection, SelectionBehavior, SelectionMode},
        },
    },
    utils::{
        CapturedElement, EventHandler,
        aria::{AriaDisabled, AriaMultiselectable, AriaOrientation, AriaRole},
        locale::WritingDirection,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/listbox/src/useListBox.ts

// REACT-ARIA DEVIATIONS
//
// No intentional deviations from the react-aria implementation.

/// Input parameters for the `use_listbox` hook.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone)]
pub struct UseListBoxInput<K>
where
    K: SelectionKey,
{
    /// The selection mode.
    pub selection_mode: SelectionMode,

    /// The selection behavior.
    pub selection_behavior: SelectionBehavior,

    /// Whether the listbox is disabled.
    pub is_disabled: Signal<bool>,

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

    /// All available keys in the listbox.
    pub items: Signal<Vec<K>>,

    /// Whether keyboard navigation should wrap.
    pub should_focus_wrap: bool,

    /// Auto-focus strategy. When `Some`, focuses the first or last item on mount.
    pub auto_focus: Signal<Option<FocusStrategy>>,

    /// Whether to select items on focus.
    pub select_on_focus: bool,

    /// An accessibility label for the listbox.
    pub aria_label: Option<&'static str>,

    /// The ID of an element that labels the listbox.
    pub aria_labelledby: Option<String>,

    /// A function to get the text value for type-ahead.
    pub get_text_value: Option<Callback<K, String>>,

    /// Whether the listbox is virtualized.
    pub is_virtualized: bool,

    /// Orientation of the listbox.
    pub orientation: ListBoxOrientation,

    /// Element ref for the listbox container.
    pub collection_ref: CapturedElement,

    /// Optional callback invoked when Escape is pressed (e.g. to close a select dropdown).
    pub on_close: Option<Callback<()>>,

    /// Behavior when Escape is pressed.
    pub escape_key_behavior: EscapeKeyBehavior,
}

/// The orientation of a listbox.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ListBoxOrientation {
    /// Vertical layout (default for listbox).
    #[default]
    Vertical,
    /// Horizontal layout.
    Horizontal,
}

impl From<ListBoxOrientation> for AriaOrientation {
    fn from(value: ListBoxOrientation) -> Self {
        match value {
            ListBoxOrientation::Vertical => Self::Vertical,
            ListBoxOrientation::Horizontal => Self::Horizontal,
        }
    }
}

impl<K: SelectionKey> Default for UseListBoxInput<K> {
    fn default() -> Self {
        Self {
            selection_mode: SelectionMode::Single,
            selection_behavior: SelectionBehavior::Toggle,
            is_disabled: Signal::derive(|| false),
            selected_keys: None,
            default_selected_keys: None,
            on_selection_change: None,
            disabled_keys: Signal::derive(HashSet::new),
            disallow_empty_selection: false,
            items: Signal::derive(Vec::new),
            should_focus_wrap: true,
            auto_focus: Signal::derive(|| None),
            select_on_focus: false,
            aria_label: None,
            aria_labelledby: None,
            get_text_value: None,
            is_virtualized: false,
            orientation: ListBoxOrientation::Vertical,
            collection_ref: CapturedElement::new(),
            on_close: None,
            escape_key_behavior: EscapeKeyBehavior::default(),
        }
    }
}

/// The return value of the `use_listbox` hook.
pub struct UseListBoxReturn<K>
where
    K: SelectionKey,
{
    /// Props for the listbox container element. Call `.into_attrs()` for view spreading.
    pub listbox_props: UseListBoxProps,

    /// The selection and navigation state.
    pub state: UseSelectableListReturn<K>,

    /// The ID of the listbox.
    pub id: String,
}

/// Props from `use_listbox` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseListBoxProps {
    pub id: String,
    pub role: AriaRole,
    pub tabindex: &'static str,
    pub aria_label: Option<&'static str>,
    pub aria_labelledby: Option<String>,
    pub aria_multiselectable: Option<AriaMultiselectable>,
    pub aria_orientation: AriaOrientation,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub on_keydown: EventHandler<KeyboardEvent>,
}

impl IntoAttrs for UseListBoxProps {
    type Attrs = UseListBoxAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaMultiselectable, self.aria_multiselectable),
            Attr(attr::AriaOrientation, self.aria_orientation),
            Attr(attr::AriaDisabled, self.aria_disabled),
            self.on_keydown.into_on(ev::keydown),
        )
    }
}

/// These attributes must be spread onto the target element using the spread syntax `<div {..attrs}/>`.
pub type UseListBoxAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, AriaRole>,
    Attr<attr::Tabindex, &'static str>,
    Attr<attr::AriaLabel, Option<&'static str>>,
    Attr<attr::AriaLabelledby, Option<String>>,
    Attr<attr::AriaMultiselectable, Option<AriaMultiselectable>>,
    Attr<attr::AriaOrientation, AriaOrientation>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
);

/// Provides the behavior and accessibility implementation for a listbox.
///
/// A listbox displays a list of options and allows a user to select one or more of them.
///
/// # Example
///
/// ```ignore
/// let items = Signal::derive(|| vec!["Apple", "Banana", "Cherry"]);
///
/// let listbox = use_listbox(UseListBoxInput {
///     selection_mode: SelectionMode::Single,
///     items: items.into(),
///     aria_label: Some("Select a fruit"),
///     ..Default::default()
/// });
///
/// view! {
///     <ul {..listbox.listbox_props.into_attrs()}>
///         <For
///             each=move || items.get()
///             key=|item| item.to_string()
///             children=move |item| {
///                 let option = use_option(UseOptionInput {
///                     key: item.to_string(),
///                     state: listbox.state.collection.selection_state,
///                     ..Default::default()
///                 });
///                 view! {
///                     <li {..option.option_props.into_attrs()}>{item}</li>
///                 }
///             }
///         />
///     </ul>
/// }
/// ```
#[allow(clippy::too_many_lines)]
pub fn use_listbox<K>(input: UseListBoxInput<K>) -> UseListBoxReturn<K>
where
    K: SelectionKey,
{
    let UseListBoxInput {
        selection_mode,
        selection_behavior,
        is_disabled,
        selected_keys,
        default_selected_keys,
        on_selection_change,
        disabled_keys,
        disallow_empty_selection,
        items,
        should_focus_wrap,
        auto_focus,
        select_on_focus,
        aria_label,
        aria_labelledby,
        get_text_value: _get_text_value,
        is_virtualized: _is_virtualized,
        orientation,
        collection_ref,
        on_close,
        escape_key_behavior,
    } = input;

    let listbox_id = format!("listbox-{}", Uuid::new_v4());

    // Convert ListBoxOrientation to Orientation for the delegate
    let list_orientation = match orientation {
        ListBoxOrientation::Vertical => Orientation::Vertical,
        ListBoxOrientation::Horizontal => Orientation::Horizontal,
    };

    // Use selectable list for selection, navigation, and keyboard handling.
    // The collection hook now handles all keyboard events, focus, blur, and scroll.
    let state = use_selectable_list(UseSelectableListInput {
        selection_mode,
        selection_behavior,
        disabled: is_disabled,
        selected_keys,
        default_selected_keys,
        on_selection_change,
        disabled_keys,
        disallow_empty_selection,
        disabled_behavior: DisabledBehavior::default(),
        all_keys: items,
        should_focus_wrap,
        auto_focus,
        select_on_focus: Some(select_on_focus),
        orientation: list_orientation,
        direction: Signal::derive(|| WritingDirection::Ltr), // TODO: get from i18n context
        collection_ref,
        escape_key_behavior,
        disallow_select_all: false,
        on_close,
        disallow_type_ahead: false,
        get_key_label: None,
        allows_tab_navigation: false,
    });

    // Use the collection's keyboard handler (no more duplicated keyboard handling)
    let on_keydown = state.list_props.on_keydown.clone();

    // Compute aria-multiselectable
    let aria_multiselectable = match selection_mode {
        SelectionMode::None => None,
        SelectionMode::Single => Some(AriaMultiselectable::False),
        SelectionMode::Multiple => Some(AriaMultiselectable::True),
    };

    // Compute aria-disabled (reactive)
    let aria_disabled = Signal::derive(move || is_disabled.get().then_some(AriaDisabled::True));

    // Orientation
    let aria_orientation = AriaOrientation::from(orientation);

    UseListBoxReturn {
        listbox_props: UseListBoxProps {
            id: listbox_id.clone(),
            role: AriaRole::Listbox,
            tabindex: "0",
            aria_label,
            aria_labelledby,
            aria_multiselectable,
            aria_orientation,
            aria_disabled,
            on_keydown,
        },
        state,
        id: listbox_id,
    }
}

use leptos::attr;
use leptos::attr::Attr;
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use std::collections::HashSet;
use std::hash::Hash;
use uuid::Uuid;
use web_sys::KeyboardEvent;

use crate::utils::aria::{AriaDisabled, AriaMultiselectable, AriaOrientation};
use crate::utils::EventHandler;

use crate::hooks::selection::{
    use_selectable_collection::FocusStrategy,
    use_selectable_list::{use_selectable_list, UseSelectableListInput, UseSelectableListReturn},
    use_selection_state::{Selection, SelectionBehavior, SelectionMode},
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/listbox/src/useListBox.ts

/// Input parameters for the `use_listbox` hook.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone)]
pub struct UseListBoxInput<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
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

    /// Whether to auto-focus the first item.
    pub auto_focus: bool,

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

impl<K: Hash + Eq + Clone + Send + Sync + 'static> Default for UseListBoxInput<K> {
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
            auto_focus: false,
            select_on_focus: false,
            aria_label: None,
            aria_labelledby: None,
            get_text_value: None,
            is_virtualized: false,
            orientation: ListBoxOrientation::Vertical,
        }
    }
}

/// The return value of the `use_listbox` hook.
#[derive(Clone)]
pub struct UseListBoxReturn<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    /// Props for the listbox container element. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub listbox_props: UseListBoxProps,

    /// The selection and navigation state.
    pub state: UseSelectableListReturn<K>,

    /// The ID of the listbox.
    pub id: String,
}

/// Props from `use_listbox` that can be extracted and merged programmatically.
#[derive(Debug, Clone)]
pub struct UseListBoxProps {
    pub id: String,
    pub role: &'static str,
    pub tabindex: &'static str,
    pub aria_label: Option<&'static str>,
    pub aria_labelledby: Option<String>,
    pub aria_multiselectable: Option<AriaMultiselectable>,
    pub aria_orientation: AriaOrientation,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub on_keydown: EventHandler<KeyboardEvent>,
}

impl UseListBoxProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseListBoxAttrs {
        (
            Attr(attr::Id, self.id.clone()),
            Attr(attr::Role, self.role),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby.clone()),
            Attr(attr::AriaMultiselectable, self.aria_multiselectable),
            Attr(attr::AriaOrientation, self.aria_orientation),
            Attr(attr::AriaDisabled, self.aria_disabled),
            self.on_keydown.to_on(ev::keydown),
        )
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseListBoxAttrs {
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
    Attr<attr::Role, &'static str>,
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
    K: Hash + Eq + Clone + Send + Sync + 'static,
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
        get_text_value,
        is_virtualized,
        orientation,
    } = input;

    let listbox_id = format!("listbox-{}", Uuid::new_v4());

    // Convert auto_focus bool to Signal<Option<FocusStrategy>>
    // For listbox, auto_focus means focus first item on mount
    let auto_focus_signal = if auto_focus {
        Signal::derive(|| Some(FocusStrategy::First))
    } else {
        Signal::derive(|| None)
    };

    // Use selectable list for selection and navigation
    let state = use_selectable_list(UseSelectableListInput {
        selection_mode,
        selection_behavior,
        disabled: is_disabled,
        selected_keys,
        default_selected_keys,
        on_selection_change,
        disabled_keys,
        disallow_empty_selection,
        all_keys: items,
        should_focus_wrap,
        auto_focus: auto_focus_signal,
        select_on_focus,
    });

    // Extract navigation callbacks for keyboard handler
    let focus_next = state.collection.focus_next;
    let focus_previous = state.collection.focus_previous;
    let focus_first = state.collection.focus_first;
    let focus_last = state.collection.focus_last;
    let focused_key = state.collection.focused_key;
    let select_cb = state.collection.selection_state.select;
    let toggle_cb = state.collection.selection_state.toggle;
    let select_all_cb = state.collection.selection_state.select_all;
    let clear_selection_cb = state.collection.selection_state.clear_selection;

    // Type-ahead state
    let (search_string, set_search_string) = signal(String::new());

    // Handle keyboard events
    let handle_keydown = move |e: KeyboardEvent| {
        if is_disabled.get_untracked() {
            return;
        }

        let key = e.key();

        match key.as_str() {
            "ArrowDown" | "ArrowRight" => {
                e.prevent_default();
                focus_next.run(());
                if select_on_focus && selection_mode == SelectionMode::Single {
                    if let Some(focused) = focused_key.get_untracked() {
                        select_cb.run(focused);
                    }
                }
            }
            "ArrowUp" | "ArrowLeft" => {
                e.prevent_default();
                focus_previous.run(());
                if select_on_focus && selection_mode == SelectionMode::Single {
                    if let Some(focused) = focused_key.get_untracked() {
                        select_cb.run(focused);
                    }
                }
            }
            "Home" => {
                e.prevent_default();
                focus_first.run(());
                if select_on_focus && selection_mode == SelectionMode::Single {
                    if let Some(focused) = focused_key.get_untracked() {
                        select_cb.run(focused);
                    }
                }
            }
            "End" => {
                e.prevent_default();
                focus_last.run(());
                if select_on_focus && selection_mode == SelectionMode::Single {
                    if let Some(focused) = focused_key.get_untracked() {
                        select_cb.run(focused);
                    }
                }
            }
            " " | "Enter" => {
                e.prevent_default();
                if let Some(focused) = focused_key.get_untracked() {
                    match selection_behavior {
                        SelectionBehavior::Toggle => toggle_cb.run(focused),
                        SelectionBehavior::Replace => select_cb.run(focused),
                    }
                }
            }
            "a" if e.ctrl_key() || e.meta_key() => {
                if selection_mode == SelectionMode::Multiple {
                    e.prevent_default();
                    select_all_cb.run(Vec::new());
                }
            }
            "Escape" => {
                e.prevent_default();
                clear_selection_cb.run(());
            }
            _ => {
                // Type-ahead: if it's a printable character
                if key.len() == 1 && !e.ctrl_key() && !e.alt_key() && !e.meta_key() {
                    if let Some(get_text) = get_text_value {
                        let current_search = search_string.get_untracked();
                        let new_search = format!("{}{}", current_search, key.to_lowercase());
                        set_search_string.set(new_search.clone());

                        // Find matching item
                        let items_list = items.get_untracked();
                        for item_key in &items_list {
                            let text = get_text.run(item_key.clone());
                            if text.to_lowercase().starts_with(&new_search) {
                                // Focus this item
                                state.collection.set_focused_key.run(Some(item_key.clone()));
                                break;
                            }
                        }

                        // Reset search after timeout (simplified - just clear on next non-char key)
                    }
                } else {
                    // Clear search on non-printable key
                    set_search_string.set(String::new());
                }
            }
        }
    };

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
            role: "listbox",
            tabindex: "0",
            aria_label,
            aria_labelledby,
            aria_multiselectable,
            aria_orientation,
            aria_disabled,
            on_keydown: EventHandler::new(handle_keydown),
        },
        state,
        id: listbox_id,
    }
}

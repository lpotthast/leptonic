use leptos::attr;
use leptos::attr::{Attr, Attribute};
use leptos::ev;
use leptos::ev::{on, On, SharedEventCallback};
use leptos::prelude::*;
use std::collections::HashSet;
use std::hash::Hash;
use uuid::Uuid;
use web_sys::KeyboardEvent;

use crate::utils::aria::{AriaDisabled, AriaExpanded, AriaRequired};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/select/src/useSelect.ts

/// Input parameters for the `use_select` hook.
#[derive(Clone)]
pub struct UseSelectInput<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    /// Whether the select is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the select is required.
    pub is_required: bool,

    /// The controlled selected key.
    pub selected_key: Option<Signal<Option<K>>>,

    /// The default selected key (uncontrolled).
    pub default_selected_key: Option<K>,

    /// Callback when selection changes.
    pub on_selection_change: Option<Callback<Option<K>>>,

    /// Keys that cannot be selected.
    pub disabled_keys: Signal<HashSet<K>>,

    /// All available items.
    pub items: Signal<Vec<K>>,

    /// Whether the menu is open (controlled).
    pub is_open: Option<Signal<bool>>,

    /// The default open state (uncontrolled).
    pub default_open: bool,

    /// Callback when open state changes.
    pub on_open_change: Option<Callback<bool>>,

    /// An accessibility label for the select.
    pub aria_label: Option<&'static str>,

    /// The ID of an element that labels the select.
    pub aria_labelledby: Option<String>,

    /// A function to get the text value for display.
    pub get_text_value: Option<Callback<K, String>>,

    /// The name attribute for form submission.
    pub name: Option<&'static str>,

    /// Placeholder text when nothing is selected.
    pub placeholder: Option<&'static str>,
}

impl<K: Hash + Eq + Clone + Send + Sync + 'static> Default for UseSelectInput<K> {
    fn default() -> Self {
        Self {
            is_disabled: Signal::derive(|| false),
            is_required: false,
            selected_key: None,
            default_selected_key: None,
            on_selection_change: None,
            disabled_keys: Signal::derive(HashSet::new),
            items: Signal::derive(Vec::new),
            is_open: None,
            default_open: false,
            on_open_change: None,
            aria_label: None,
            aria_labelledby: None,
            get_text_value: None,
            name: None,
            placeholder: None,
        }
    }
}

/// The return value of the `use_select` hook.
#[derive(Debug, Clone)]
pub struct UseSelectReturn<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    /// Props for the trigger button element.
    pub trigger_props: UseSelectTriggerAttrs,

    /// Props for the value display element.
    pub value_props: UseSelectValueProps,

    /// Props for the listbox element.
    pub listbox_props: UseSelectListBoxProps,

    /// Props for a hidden input for form submission.
    pub hidden_select_props: UseSelectHiddenProps,

    /// The ID of the trigger element.
    pub trigger_id: String,

    /// The ID of the listbox element.
    pub listbox_id: String,

    /// Whether the menu is open.
    pub is_open: Signal<bool>,

    /// The currently selected key.
    pub selected_key: Signal<Option<K>>,

    /// The display value for the selected item.
    pub display_value: Signal<Option<String>>,

    /// Open the menu.
    pub open: Callback<()>,

    /// Close the menu.
    pub close: Callback<()>,

    /// Toggle the menu.
    pub toggle: Callback<()>,

    /// Select a key.
    pub select: Callback<K>,
}

/// Attributes for the select trigger button.
pub type UseSelectTriggerAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, &'static str>,
    Attr<attr::Tabindex, &'static str>,
    Attr<attr::AriaHaspopup, &'static str>,
    Attr<attr::AriaExpanded, Signal<Option<AriaExpanded>>>,
    Attr<attr::AriaControls, String>,
    Attr<attr::AriaLabel, Option<&'static str>>,
    Attr<attr::AriaLabelledby, Option<String>>,
    Attr<attr::AriaRequired, Option<AriaRequired>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    On<ev::click, SharedEventCallback<web_sys::MouseEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
);

/// Props for the value display element.
#[derive(Debug, Clone)]
pub struct UseSelectValueProps {
    /// The id of the value element.
    pub id: String,
}

/// Props for the listbox element.
#[derive(Debug, Clone)]
pub struct UseSelectListBoxProps {
    /// The id of the listbox.
    pub id: String,

    /// The role attribute.
    pub role: &'static str,

    /// The aria-labelledby attribute.
    pub aria_labelledby: String,

    /// The tabindex attribute.
    pub tabindex: &'static str,
}

/// Props for the hidden select element (for form submission).
#[derive(Debug, Clone)]
pub struct UseSelectHiddenProps {
    /// The name attribute.
    pub name: Option<&'static str>,

    /// Whether the field is required.
    pub required: bool,

    /// Whether the field is disabled.
    pub disabled: Signal<bool>,
}

/// Provides the behavior and accessibility implementation for a select component.
///
/// Select displays a collapsible list of options and allows a user to select one of them.
///
/// # Example
///
/// ```ignore
/// let items = Signal::derive(|| vec!["Apple", "Banana", "Cherry"]);
///
/// let select = use_select(UseSelectInput {
///     items,
///     aria_label: Some("Select a fruit"),
///     placeholder: Some("Choose a fruit"),
///     ..Default::default()
/// });
///
/// view! {
///     <div>
///         <button {..select.trigger_props}>
///             {move || select.display_value.get().unwrap_or_else(|| "Choose a fruit".to_string())}
///         </button>
///         <Show when=move || select.is_open.get()>
///             <ul {..select.listbox_props}>
///                 // Options here
///             </ul>
///         </Show>
///     </div>
/// }
/// ```
#[allow(clippy::too_many_lines)]
pub fn use_select<K>(input: UseSelectInput<K>) -> UseSelectReturn<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    let base_id = Uuid::new_v4();
    let trigger_id = format!("select-trigger-{base_id}");
    let value_id = format!("select-value-{base_id}");
    let listbox_id = format!("select-listbox-{base_id}");

    let is_disabled = input.is_disabled;
    let _items = input.items;
    let get_text_value = input.get_text_value;

    // Internal open state
    let (internal_open, set_internal_open) = signal(input.default_open);
    let is_open = input.is_open.unwrap_or_else(|| internal_open.into());
    let on_open_change = input.on_open_change;

    // Internal selected state
    let (internal_selected, set_internal_selected) = signal(input.default_selected_key);
    let selected_key = input
        .selected_key
        .unwrap_or_else(|| internal_selected.into());
    let on_selection_change = input.on_selection_change;

    // Compute display value
    let display_value = Signal::derive(move || {
        selected_key.get().map(|key| {
            if let Some(get_text) = get_text_value {
                get_text.run(key)
            } else {
                // Default: just return a placeholder
                "Selected".to_string()
            }
        })
    });

    // Open the menu
    let open = Callback::new(move |_| {
        if is_disabled.get_untracked() {
            return;
        }
        set_internal_open.set(true);
        if let Some(on_change) = on_open_change {
            on_change.run(true);
        }
    });

    // Close the menu
    let close = Callback::new(move |_| {
        set_internal_open.set(false);
        if let Some(on_change) = on_open_change {
            on_change.run(false);
        }
    });

    // Toggle the menu
    let toggle = Callback::new(move |_| {
        if is_disabled.get_untracked() {
            return;
        }
        let new_state = !is_open.get_untracked();
        set_internal_open.set(new_state);
        if let Some(on_change) = on_open_change {
            on_change.run(new_state);
        }
    });

    // Select a key
    let select = Callback::new(move |key: K| {
        if is_disabled.get_untracked() {
            return;
        }
        set_internal_selected.set(Some(key.clone()));
        if let Some(on_change) = on_selection_change {
            on_change.run(Some(key));
        }
        // Close after selection
        set_internal_open.set(false);
        if let Some(on_open_change) = on_open_change {
            on_open_change.run(false);
        }
    });

    // Handle click on trigger
    let toggle_click = toggle;
    let handle_click = move |_e: web_sys::MouseEvent| {
        toggle_click.run(());
    };

    // Handle keyboard on trigger
    let handle_keydown = move |e: KeyboardEvent| {
        if is_disabled.get_untracked() {
            return;
        }

        match e.key().as_str() {
            " " | "Enter" | "ArrowDown" | "ArrowUp" => {
                e.prevent_default();
                if !is_open.get_untracked() {
                    set_internal_open.set(true);
                    if let Some(on_change) = on_open_change {
                        on_change.run(true);
                    }
                }
            }
            "Escape" => {
                if is_open.get_untracked() {
                    e.prevent_default();
                    set_internal_open.set(false);
                    if let Some(on_change) = on_open_change {
                        on_change.run(false);
                    }
                }
            }
            _ => {}
        }
    };

    // Compute aria-expanded
    let aria_expanded = Signal::derive(move || Some(AriaExpanded::from(is_open.get())));

    // Compute aria-required
    let aria_required = input.is_required.then_some(AriaRequired::True);

    // Compute aria-disabled (reactive)
    let aria_disabled = Signal::derive(move || is_disabled.get().then_some(AriaDisabled::True));

    UseSelectReturn {
        trigger_props: (
            Attr(attr::Id, trigger_id.clone()),
            Attr(attr::Role, "combobox"),
            Attr(attr::Tabindex, "0"),
            Attr(attr::AriaHaspopup, "listbox"),
            Attr(attr::AriaExpanded, aria_expanded),
            Attr(attr::AriaControls, listbox_id.clone()),
            Attr(attr::AriaLabel, input.aria_label),
            Attr(attr::AriaLabelledby, input.aria_labelledby),
            Attr(attr::AriaRequired, aria_required),
            Attr(attr::AriaDisabled, aria_disabled),
            on(ev::click, handle_click).into_cloneable(),
            on(ev::keydown, handle_keydown).into_cloneable(),
        ),
        value_props: UseSelectValueProps { id: value_id },
        listbox_props: UseSelectListBoxProps {
            id: listbox_id.clone(),
            role: "listbox",
            aria_labelledby: trigger_id.clone(),
            tabindex: "-1",
        },
        hidden_select_props: UseSelectHiddenProps {
            name: input.name,
            required: input.is_required,
            disabled: is_disabled,
        },
        trigger_id,
        listbox_id,
        is_open,
        selected_key,
        display_value,
        open,
        close,
        toggle,
        select,
    }
}

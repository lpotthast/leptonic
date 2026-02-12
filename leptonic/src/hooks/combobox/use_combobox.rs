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
use web_sys::{Event, FocusEvent, KeyboardEvent, MouseEvent};

use crate::{
    hooks::IntoAttrs,
    utils::{
        aria::{AriaExpanded, AriaRequired},
        EventAccessors, EventHandler,
    },
};
// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/combobox/src/useComboBox.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Input parameters for the `use_combobox` hook.
#[allow(clippy::type_complexity)]
#[derive(Clone)]
pub struct UseComboBoxInput<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    /// The input value (controlled).
    pub input_value: Option<Signal<String>>,

    /// The default input value (uncontrolled).
    pub default_input_value: Option<String>,

    /// Callback when input value changes.
    pub on_input_change: Option<Callback<String>>,

    /// Whether the combobox is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the combobox is read-only.
    pub is_read_only: Signal<bool>,

    /// Whether the combobox is required.
    pub is_required: bool,

    /// The controlled selected key.
    pub selected_key: Option<Signal<Option<K>>>,

    /// The default selected key (uncontrolled).
    pub default_selected_key: Option<K>,

    /// Callback when selection changes.
    pub on_selection_change: Option<Callback<Option<K>>>,

    /// Keys that cannot be selected.
    pub disabled_keys: Signal<HashSet<K>>,

    /// All available items (for filtering).
    pub items: Signal<Vec<K>>,

    /// Whether the menu is open (controlled).
    pub is_open: Option<Signal<bool>>,

    /// The default open state (uncontrolled).
    pub default_open: bool,

    /// Callback when open state changes.
    pub on_open_change: Option<Callback<bool>>,

    /// An accessibility label for the combobox.
    pub aria_label: Option<&'static str>,

    /// The ID of an element that labels the combobox.
    pub aria_labelledby: Option<String>,

    /// A function to get the text value for display.
    pub get_text_value: Option<Callback<K, String>>,

    /// A function to filter items based on input.
    pub filter: Option<Callback<(String, Vec<K>), Vec<K>>>,

    /// The name attribute for form submission.
    pub name: Option<&'static str>,

    /// Placeholder text.
    pub placeholder: Option<&'static str>,

    /// Whether to allow custom values not in the list.
    pub allows_custom_value: bool,

    /// The menu trigger behavior.
    pub menu_trigger: MenuTriggerAction,
}

/// When to show the combobox menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MenuTriggerAction {
    /// Show menu on focus.
    Focus,
    /// Show menu on input.
    #[default]
    Input,
    /// Show menu manually (button click).
    Manual,
}

impl<K: Hash + Eq + Clone + Send + Sync + 'static> Default for UseComboBoxInput<K> {
    fn default() -> Self {
        Self {
            input_value: None,
            default_input_value: None,
            on_input_change: None,
            is_disabled: Signal::derive(|| false),
            is_read_only: Signal::derive(|| false),
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
            filter: None,
            name: None,
            placeholder: None,
            allows_custom_value: false,
            menu_trigger: MenuTriggerAction::Input,
        }
    }
}

/// The return value of the `use_combobox` hook.
pub struct UseComboBoxReturn<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    /// Props for the input element.
    pub input_props: UseComboBoxInputProps,

    /// Props for the trigger button element.
    pub button_props: UseComboBoxButtonProps,

    /// Props for the listbox element.
    pub listbox_props: UseComboBoxListBoxProps,

    /// The ID of the input element.
    pub input_id: String,

    /// The ID of the listbox element.
    pub listbox_id: String,

    /// Whether the menu is open.
    pub is_open: Signal<bool>,

    /// The current input value.
    pub input_value: Signal<String>,

    /// The currently selected key.
    pub selected_key: Signal<Option<K>>,

    /// The filtered items based on input.
    pub filtered_items: Signal<Vec<K>>,

    /// The currently focused key in the listbox.
    pub focused_key: Signal<Option<K>>,

    /// Open the menu.
    pub open: Callback<()>,

    /// Close the menu.
    pub close: Callback<()>,

    /// Toggle the menu.
    pub toggle: Callback<()>,

    /// Select a key.
    pub select: Callback<K>,

    /// Set the input value.
    pub set_input_value: Callback<String>,

    /// Clear the input and selection.
    pub clear: Callback<()>,
}

/// Props from `use_combobox` for the input element that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseComboBoxInputProps {
    pub id: String,
    pub r#type: &'static str,
    pub role: &'static str,
    pub value: Signal<String>,
    pub placeholder: Option<&'static str>,
    pub disabled: Signal<bool>,
    pub readonly: Signal<bool>,
    pub aria_autocomplete: &'static str,
    pub aria_haspopup: &'static str,
    pub aria_expanded: Signal<Option<AriaExpanded>>,
    pub aria_controls: String,
    pub aria_label: Option<&'static str>,
    pub aria_labelledby: Option<String>,
    pub aria_required: Option<AriaRequired>,
    pub aria_activedescendant: Signal<Option<String>>,
    pub on_input: EventHandler<Event>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
}

impl IntoAttrs for UseComboBoxInputProps {
    type Attrs = UseComboBoxInputAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Type, self.r#type),
            Attr(attr::Role, self.role),
            Attr(attr::Value, self.value),
            Attr(attr::Placeholder, self.placeholder),
            Attr(attr::Disabled, self.disabled),
            Attr(attr::Readonly, self.readonly),
            Attr(attr::AriaAutocomplete, self.aria_autocomplete),
            Attr(attr::AriaHaspopup, self.aria_haspopup),
            Attr(attr::AriaExpanded, self.aria_expanded),
            Attr(attr::AriaControls, self.aria_controls),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaRequired, self.aria_required),
            Attr(attr::AriaActivedescendant, self.aria_activedescendant),
            self.on_input.into_on(ev::input),
            self.on_keydown.into_on(ev::keydown),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
        )
    }
}

/// Props from `use_combobox` for the button element that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseComboBoxButtonProps {
    pub id: String,
    pub r#type: &'static str,
    pub tabindex: &'static str,
    pub aria_label: &'static str,
    pub aria_haspopup: &'static str,
    pub aria_expanded: Signal<Option<AriaExpanded>>,
    pub disabled: Signal<bool>,
    pub on_click: EventHandler<MouseEvent>,
}

impl IntoAttrs for UseComboBoxButtonProps {
    type Attrs = UseComboBoxButtonAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Type, self.r#type),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaHaspopup, self.aria_haspopup),
            Attr(attr::AriaExpanded, self.aria_expanded),
            Attr(attr::Disabled, self.disabled),
            self.on_click.into_on(ev::click),
        )
    }
}

/// Attributes for the combobox input element.
pub type UseComboBoxInputAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Type, &'static str>,
    Attr<attr::Role, &'static str>,
    Attr<attr::Value, Signal<String>>,
    Attr<attr::Placeholder, Option<&'static str>>,
    Attr<attr::Disabled, Signal<bool>>,
    Attr<attr::Readonly, Signal<bool>>,
    Attr<attr::AriaAutocomplete, &'static str>,
    Attr<attr::AriaHaspopup, &'static str>,
    Attr<attr::AriaExpanded, Signal<Option<AriaExpanded>>>,
    Attr<attr::AriaControls, String>,
    Attr<attr::AriaLabel, Option<&'static str>>,
    Attr<attr::AriaLabelledby, Option<String>>,
    Attr<attr::AriaRequired, Option<AriaRequired>>,
    Attr<attr::AriaActivedescendant, Signal<Option<String>>>,
    On<ev::input, SharedEventCallback<Event>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
);

/// Attributes for the combobox button element.
pub type UseComboBoxButtonAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Type, &'static str>,
    Attr<attr::Tabindex, &'static str>,
    Attr<attr::AriaLabel, &'static str>,
    Attr<attr::AriaHaspopup, &'static str>,
    Attr<attr::AriaExpanded, Signal<Option<AriaExpanded>>>,
    Attr<attr::Disabled, Signal<bool>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
);

/// Props for the listbox element.
#[derive(Debug)]
pub struct UseComboBoxListBoxProps {
    /// The id of the listbox.
    pub id: String,

    /// The role attribute.
    pub role: &'static str,

    /// The aria-labelledby attribute.
    pub aria_labelledby: String,

    /// The tabindex attribute.
    pub tabindex: &'static str,
}

/// Provides the behavior and accessibility implementation for a combobox.
///
/// A combobox combines a text input with a listbox, allowing users to filter
/// and select from a list of options.
///
/// # Example
///
/// ```ignore
/// let items = Signal::derive(|| vec!["Apple", "Banana", "Cherry"]);
///
/// let combobox = use_combobox(UseComboBoxInput {
///     items,
///     aria_label: Some("Select a fruit"),
///     placeholder: Some("Search fruits..."),
///     get_text_value: Some(Callback::new(|s: &str| s.to_string())),
///     ..Default::default()
/// });
///
/// view! {
///     <div>
///         <input {..combobox.input_props.into_attrs()} />
///         <button {..combobox.button_props.into_attrs()}>"▼"</button>
///         <Show when=move || combobox.is_open.get()>
///             <ul {..combobox.listbox_props}>
///                 <For
///                     each=move || combobox.filtered_items.get()
///                     key=|item| item.to_string()
///                     children=|item| view! { <li>{item}</li> }
///                 />
///             </ul>
///         </Show>
///     </div>
/// }
/// ```
#[allow(clippy::too_many_lines)]
pub fn use_combobox<K>(input: UseComboBoxInput<K>) -> UseComboBoxReturn<K>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
{
    let UseComboBoxInput {
        input_value,
        default_input_value,
        on_input_change,
        is_disabled,
        is_read_only,
        is_required,
        selected_key,
        default_selected_key,
        on_selection_change,
        disabled_keys,
        items,
        is_open,
        default_open,
        on_open_change,
        aria_label,
        aria_labelledby,
        get_text_value,
        filter,
        name,
        placeholder,
        allows_custom_value,
        menu_trigger,
    } = input;

    let base_id = Uuid::new_v4();
    let input_id = format!("combobox-input-{base_id}");
    let button_id = format!("combobox-button-{base_id}");
    let listbox_id = format!("combobox-listbox-{base_id}");

    // Internal state
    let (internal_input_value, set_internal_input_value) =
        signal(default_input_value.unwrap_or_default());
    let input_value = input_value.unwrap_or_else(|| internal_input_value.into());

    let (internal_open, set_internal_open) = signal(default_open);
    let is_open = is_open.unwrap_or_else(|| internal_open.into());

    let (internal_selected, set_internal_selected) = signal(default_selected_key);
    let selected_key = selected_key.unwrap_or_else(|| internal_selected.into());

    let (focused_key, set_focused_key) = signal::<Option<K>>(None);

    // Filter items based on input value
    let filtered_items = Signal::derive(move || {
        let current_items = items.get();
        let current_input = input_value.get();

        if current_input.is_empty() {
            return current_items;
        }

        if let Some(filter_fn) = filter {
            filter_fn.run((current_input, current_items))
        } else if let Some(get_text) = get_text_value {
            // Default filter: case-insensitive contains
            let search_lower = current_input.to_lowercase();
            current_items
                .into_iter()
                .filter(|item| {
                    let text = get_text.run(item.clone());
                    text.to_lowercase().contains(&search_lower)
                })
                .collect()
        } else {
            current_items
        }
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
        set_focused_key.set(None);
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
            on_change.run(Some(key.clone()));
        }

        // Update input value to match selection
        if let Some(get_text) = get_text_value {
            let text = get_text.run(key);
            set_internal_input_value.set(text.clone());
            if let Some(on_input_change) = on_input_change {
                on_input_change.run(text);
            }
        }

        // Close after selection
        set_internal_open.set(false);
        if let Some(on_open_change) = on_open_change {
            on_open_change.run(false);
        }
    });

    // Set input value
    let set_input_value = Callback::new(move |value: String| {
        set_internal_input_value.set(value.clone());
        if let Some(on_change) = on_input_change {
            on_change.run(value);
        }
    });

    // Clear
    let clear = Callback::new(move |_| {
        set_internal_input_value.set(String::new());
        set_internal_selected.set(None);
        if let Some(on_input_change) = on_input_change {
            on_input_change.run(String::new());
        }
        if let Some(on_selection_change) = on_selection_change {
            on_selection_change.run(None);
        }
    });

    // Handle input event
    let handle_input = move |e: Event| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }

        if let Some(input_el) = e
            .expect_target()
            .dyn_into::<web_sys::HtmlInputElement>()
            .ok()
        {
            let new_value = input_el.value();
            set_internal_input_value.set(new_value.clone());

            if let Some(on_change) = on_input_change {
                on_change.run(new_value);
            }

            // Open menu on input
            if menu_trigger == MenuTriggerAction::Input && !is_open.get_untracked() {
                set_internal_open.set(true);
                if let Some(on_open_change) = on_open_change {
                    on_open_change.run(true);
                }
            }
        }
    };

    // Handle keydown
    let handle_keydown = move |e: KeyboardEvent| {
        if is_disabled.get_untracked() {
            return;
        }

        let filtered = filtered_items.get_untracked();

        match e.key().as_str() {
            "ArrowDown" => {
                e.prevent_default();
                if !is_open.get_untracked() {
                    set_internal_open.set(true);
                    if let Some(on_change) = on_open_change {
                        on_change.run(true);
                    }
                } else if !filtered.is_empty() {
                    // Move focus down
                    let current = focused_key.get_untracked();
                    let next_index = if let Some(ref current_key) = current {
                        filtered
                            .iter()
                            .position(|k| k == current_key)
                            .map_or(0, |i| (i + 1).min(filtered.len() - 1))
                    } else {
                        0
                    };
                    set_focused_key.set(filtered.get(next_index).cloned());
                }
            }
            "ArrowUp" => {
                e.prevent_default();
                if is_open.get_untracked() && !filtered.is_empty() {
                    // Move focus up
                    let current = focused_key.get_untracked();
                    let prev_index = if let Some(ref current_key) = current {
                        filtered
                            .iter()
                            .position(|k| k == current_key)
                            .map_or(filtered.len().saturating_sub(1), |i| i.saturating_sub(1))
                    } else {
                        filtered.len().saturating_sub(1)
                    };
                    set_focused_key.set(filtered.get(prev_index).cloned());
                }
            }
            "Enter" => {
                if is_open.get_untracked() {
                    e.prevent_default();
                    if let Some(key) = focused_key.get_untracked() {
                        set_internal_selected.set(Some(key.clone()));
                        if let Some(on_change) = on_selection_change {
                            on_change.run(Some(key.clone()));
                        }

                        if let Some(get_text) = get_text_value {
                            let text = get_text.run(key);
                            set_internal_input_value.set(text.clone());
                            if let Some(on_input_change) = on_input_change {
                                on_input_change.run(text);
                            }
                        }

                        set_internal_open.set(false);
                        if let Some(on_open_change) = on_open_change {
                            on_open_change.run(false);
                        }
                    }
                }
            }
            "Escape" => {
                if is_open.get_untracked() {
                    e.prevent_default();
                    set_internal_open.set(false);
                    set_focused_key.set(None);
                    if let Some(on_change) = on_open_change {
                        on_change.run(false);
                    }
                }
            }
            "Home" => {
                if is_open.get_untracked() && !filtered.is_empty() {
                    e.prevent_default();
                    set_focused_key.set(filtered.first().cloned());
                }
            }
            "End" => {
                if is_open.get_untracked() && !filtered.is_empty() {
                    e.prevent_default();
                    set_focused_key.set(filtered.last().cloned());
                }
            }
            _ => {}
        }
    };

    // Handle focus
    let handle_focus = move |_e: FocusEvent| {
        if menu_trigger == MenuTriggerAction::Focus && !is_open.get_untracked() {
            set_internal_open.set(true);
            if let Some(on_change) = on_open_change {
                on_change.run(true);
            }
        }
    };

    // Handle blur
    let handle_blur = move |_e: FocusEvent| {
        // Close menu on blur (with a small delay to allow click on listbox)
        // In a real implementation, you'd check if focus moved to the listbox
    };

    // Handle button click
    let toggle_button = toggle;
    let handle_button_click = move |_e: MouseEvent| {
        toggle_button.run(());
    };

    // Compute aria-expanded
    let aria_expanded = Signal::derive(move || Some(AriaExpanded::from(is_open.get())));

    // Compute aria-required
    let aria_required = is_required.then_some(AriaRequired::True);

    // Compute aria-activedescendant
    let aria_activedescendant = Signal::derive(move || {
        focused_key
            .get()
            .map(|_| format!("option-{}", Uuid::new_v4()))
    });

    UseComboBoxReturn {
        input_props: UseComboBoxInputProps {
            id: input_id.clone(),
            r#type: "text",
            role: "combobox",
            value: input_value,
            placeholder,
            disabled: is_disabled,
            readonly: is_read_only,
            aria_autocomplete: "list",
            aria_haspopup: "listbox",
            aria_expanded,
            aria_controls: listbox_id.clone(),
            aria_label,
            aria_labelledby,
            aria_required,
            aria_activedescendant,
            on_input: EventHandler::new(handle_input),
            on_keydown: EventHandler::new(handle_keydown),
            on_focus: EventHandler::new(handle_focus),
            on_blur: EventHandler::new(handle_blur),
        },
        button_props: UseComboBoxButtonProps {
            id: button_id,
            r#type: "button",
            tabindex: "-1",
            aria_label: "Show suggestions",
            aria_haspopup: "listbox",
            aria_expanded,
            disabled: is_disabled,
            on_click: EventHandler::new(handle_button_click),
        },
        listbox_props: UseComboBoxListBoxProps {
            id: listbox_id.clone(),
            role: "listbox",
            aria_labelledby: input_id.clone(),
            tabindex: "-1",
        },
        input_id,
        listbox_id,
        is_open,
        input_value,
        selected_key,
        filtered_items,
        focused_key: focused_key.into(),
        open,
        close,
        toggle,
        select,
        set_input_value,
        clear,
    }
}

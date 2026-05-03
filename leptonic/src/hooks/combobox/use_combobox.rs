use std::collections::HashSet;

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
    hooks::{IntoAttrs, selection::SelectionKey},
    utils::{
        CapturedElement, ElementCaptureAttr, EventAccessors, EventHandler,
        aria::{AriaExpanded, AriaRequired, AriaRole},
        filter::{CollatorOptions, CollatorSensitivity, Filter},
        i18n::use_locale_or_default,
        node_contains,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/combobox/src/useComboBox.ts

// REACT-ARIA DEVIATIONS
//
// 1. Item keys must implement `Display` for stable option DOM IDs.
//    React-aria uses string/number keys natively.
//
// 2. No link item support (`href` on options).
//
// 3. No iOS VoiceOver virtual touch detection.
//
// 4. No form validation integration or `useFormReset` equivalent.
//
// 5. Live announcements are simplified (English only, no i18n).
//
// 6. Button uses `on_click` instead of press events with pointer type
//    differentiation. Touch-specific behavior is not implemented.
//
// 7. Collection freezing during close animation is not implemented.

/// Input parameters for the `use_combobox` hook.
#[allow(clippy::type_complexity, clippy::struct_excessive_bools)]
#[derive(Clone)]
pub struct UseComboBoxInput<K>
where
    K: SelectionKey,
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

    /// Whether keyboard navigation should wrap from last to first item (and vice versa).
    pub should_focus_wrap: bool,
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

impl<K: SelectionKey> Default for UseComboBoxInput<K> {
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
            should_focus_wrap: false,
        }
    }
}

/// The return value of the `use_combobox` hook.
pub struct UseComboBoxReturn<K>
where
    K: SelectionKey,
{
    /// Props for the input element.
    pub input_props: UseComboBoxInputProps,

    /// Props for the trigger button element.
    pub button_props: UseComboBoxButtonProps,

    /// Props for the listbox element.
    pub listbox_props: UseComboBoxListBoxProps,

    /// Props for the popover container wrapping the listbox.
    /// Spread these onto the popover element to enable `ariaHideOutside`
    /// and correct blur detection.
    pub popover_props: UseComboBoxPopoverProps,

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

    /// The items to display in the listbox.
    /// When the menu is opened via the button or focus trigger, this returns
    /// all items (bypassing the input filter). Otherwise, returns filtered items.
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

    /// Returns the stable DOM element ID for a listbox option with the given key.
    /// Use this as the `id` attribute on each rendered option element so that
    /// `aria-activedescendant` correctly references the focused option.
    pub get_option_id: Callback<K, String>,
}

/// Props from `use_combobox` for the input element.
#[derive(Debug)]
pub struct UseComboBoxInputProps {
    pub id: String,
    pub r#type: &'static str,
    pub role: AriaRole,
    pub value: Signal<String>,
    pub placeholder: Option<&'static str>,
    pub disabled: Signal<bool>,
    pub readonly: Signal<bool>,
    pub aria_autocomplete: &'static str,
    pub aria_haspopup: &'static str,
    pub aria_expanded: Signal<Option<AriaExpanded>>,
    pub aria_controls: Signal<Option<String>>,
    pub aria_label: Option<&'static str>,
    pub aria_labelledby: Option<String>,
    pub aria_required: Option<AriaRequired>,
    pub aria_activedescendant: Signal<Option<String>>,
    pub spellcheck: &'static str,
    pub on_input: EventHandler<Event>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub element_capture: ElementCaptureAttr,
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
            Attr(attr::Spellcheck, self.spellcheck),
            self.on_input.into_on(ev::input),
            self.on_keydown.into_on(ev::keydown),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.element_capture,
        )
    }
}

/// Props from `use_combobox` for the button element.
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
    Attr<attr::Role, AriaRole>,
    Attr<attr::Value, Signal<String>>,
    Attr<attr::Placeholder, Option<&'static str>>,
    Attr<attr::Disabled, Signal<bool>>,
    Attr<attr::Readonly, Signal<bool>>,
    Attr<attr::AriaAutocomplete, &'static str>,
    Attr<attr::AriaHaspopup, &'static str>,
    Attr<attr::AriaExpanded, Signal<Option<AriaExpanded>>>,
    Attr<attr::AriaControls, Signal<Option<String>>>,
    Attr<attr::AriaLabel, Option<&'static str>>,
    Attr<attr::AriaLabelledby, Option<String>>,
    Attr<attr::AriaRequired, Option<AriaRequired>>,
    Attr<attr::AriaActivedescendant, Signal<Option<String>>>,
    Attr<attr::Spellcheck, &'static str>,
    On<ev::input, SharedEventCallback<Event>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    ElementCaptureAttr,
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
    pub role: AriaRole,

    /// The aria-labelledby attribute.
    pub aria_labelledby: String,

    /// The tabindex attribute.
    pub tabindex: &'static str,
}

/// Props for the popover container wrapping the listbox.
/// Spread onto the popover element to enable `ariaHideOutside`
/// and correct blur detection.
pub struct UseComboBoxPopoverProps {
    /// Element capture attribute.
    pub element_capture: ElementCaptureAttr,
}

impl std::fmt::Debug for UseComboBoxPopoverProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UseComboBoxPopoverProps").finish()
    }
}

impl IntoAttrs for UseComboBoxPopoverProps {
    type Attrs = UseComboBoxPopoverAttrs;

    fn into_attrs(self) -> Self::Attrs {
        self.element_capture
    }
}

/// Attributes for the combobox popover element.
pub type UseComboBoxPopoverAttrs = ElementCaptureAttr;

/// Find the next or previous non-disabled key relative to `current` in `items`.
///
/// - `forward`: search direction (`true` = towards end, `false` = towards start)
/// - `wrap`: whether to wrap around boundaries
///
/// When `current` is `None`, returns the first (forward) or last (backward)
/// non-disabled key.
fn find_adjacent_enabled<K: SelectionKey>(
    items: &[K],
    current: Option<&K>,
    disabled: &HashSet<K>,
    forward: bool,
    wrap: bool,
) -> Option<K> {
    if items.is_empty() {
        return None;
    }

    let current_idx = current.and_then(|k| items.iter().position(|item| item == k));
    let len = items.len();

    let indices: Box<dyn Iterator<Item = usize>> = match (current_idx, forward) {
        (Some(idx), true) => {
            if wrap {
                Box::new((1..len).map(move |i| (idx + i) % len))
            } else {
                Box::new((idx + 1)..len)
            }
        }
        (Some(idx), false) => {
            if wrap {
                Box::new((1..len).map(move |i| (idx + len - i) % len))
            } else {
                Box::new((0..idx).rev())
            }
        }
        (None, true) => Box::new(0..len),
        (None, false) => Box::new((0..len).rev()),
    };

    for i in indices {
        if !disabled.contains(&items[i]) {
            return Some(items[i].clone());
        }
    }

    None
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
///             <div {..combobox.popover_props.into_attrs()}>
///                 <ul id=combobox.listbox_id.clone()
///                     role="listbox"
///                     aria-labelledby=combobox.input_id.clone()
///                     tabindex="-1"
///                 >
///                     <For
///                         each=move || combobox.filtered_items.get()
///                         key=|item| item.to_string()
///                         children=move |item| {
///                             let id = combobox.get_option_id.run(item.clone());
///                             view! { <li id=id role="option">{item.to_string()}</li> }
///                         }
///                     />
///                 </ul>
///             </div>
///         </Show>
///     </div>
/// }
/// ```
#[allow(clippy::too_many_lines)]
pub fn use_combobox<K>(input: UseComboBoxInput<K>) -> UseComboBoxReturn<K>
where
    K: SelectionKey,
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
        name: _name,
        placeholder,
        allows_custom_value,
        menu_trigger,
        should_focus_wrap,
    } = input;

    // ─── IDs ────────────────────────────────────────────────────────────

    let base_id = Uuid::new_v4();
    let input_id = format!("combobox-input-{base_id}");
    let button_id = format!("combobox-button-{base_id}");
    let listbox_id = format!("combobox-listbox-{base_id}");

    // ─── Element captures (for ariaHideOutside and blur detection) ──────

    let input_element = CapturedElement::new();
    let popover_element = CapturedElement::new();

    // ─── Internal state ─────────────────────────────────────────────────

    let (internal_input_value, set_internal_input_value) =
        signal(default_input_value.unwrap_or_default());
    let input_value = input_value.unwrap_or_else(|| internal_input_value.into());

    let (internal_open, set_internal_open) = signal(default_open);
    let is_open = is_open.unwrap_or_else(|| internal_open.into());

    let (internal_selected, set_internal_selected) = signal(default_selected_key);
    let selected_key = selected_key.unwrap_or_else(|| internal_selected.into());

    let (focused_key, set_focused_key) = signal::<Option<K>>(None);
    let focused_key_signal: Signal<Option<K>> = focused_key.into();

    // Whether to bypass filtering and show all items (set when opened via button/focus).
    let (show_all, set_show_all) = signal(false);

    // ─── Filtered / displayed items ─────────────────────────────────────

    let filtered_items = Signal::derive(move || {
        let current_items = items.get();

        // Show all items when opened via button or focus trigger.
        if show_all.get() {
            return current_items;
        }

        let current_input = input_value.get();
        if current_input.is_empty() {
            return current_items;
        }

        if let Some(filter_fn) = filter {
            filter_fn.run((current_input, current_items))
        } else if let Some(get_text) = get_text_value {
            // Default filter: locale-aware, case- and accent-insensitive substring match.
            let locale_filter = Filter::new(
                &use_locale_or_default(),
                &CollatorOptions {
                    sensitivity: CollatorSensitivity::Base,
                    ..CollatorOptions::default()
                },
            );
            current_items
                .into_iter()
                .filter(|item| {
                    let text = get_text.run(item.clone());
                    locale_filter.contains(&text, &current_input)
                })
                .collect()
        } else {
            current_items
        }
    });

    // Clear focused key when displayed items change and focused key is no longer present.
    Effect::new(move |_| {
        let current_items = filtered_items.get();
        if let Some(key) = focused_key.get_untracked() {
            if !current_items.contains(&key) {
                set_focused_key.set(None);
            }
        }
    });

    // ─── Internal helpers ───────────────────────────────────────────────

    // All captured values are Copy (signals, Option<Callback>), so these closures are Copy.

    let close_menu = move || {
        set_internal_open.set(false);
        set_focused_key.set(None);
        set_show_all.set(false);
        if let Some(on_change) = on_open_change {
            on_change.run(false);
        }
    };

    let update_input = move |text: String| {
        set_internal_input_value.set(text.clone());
        if let Some(on_change) = on_input_change {
            on_change.run(text);
        }
    };

    // Get the text for the currently selected key, or empty string if none.
    let get_selected_text = move || -> String {
        selected_key
            .get_untracked()
            .and_then(|key| get_text_value.map(|get_text| get_text.run(key)))
            .unwrap_or_default()
    };

    // Reset input to the selected item's text and close the menu.
    let commit_selection = move || {
        let text = get_selected_text();
        update_input(text);
        close_menu();
    };

    // Clear the selection (keep custom input value) and close the menu.
    let commit_custom_value = move || {
        set_internal_selected.set(None);
        if let Some(on_change) = on_selection_change {
            on_change.run(None);
        }
        close_menu();
    };

    // If `allows_custom_value`, decide whether to commit the selection or the custom value.
    // Otherwise, always commit the selection (resetting input).
    let commit_value = move || {
        if allows_custom_value {
            let item_text = get_selected_text();
            if input_value.get_untracked() == item_text {
                commit_selection();
            } else {
                commit_custom_value();
            }
        } else {
            commit_selection();
        }
    };

    let select_and_close = move |key: K| {
        if is_disabled.get_untracked() {
            return;
        }

        set_internal_selected.set(Some(key.clone()));
        if let Some(on_change) = on_selection_change {
            on_change.run(Some(key.clone()));
        }

        if let Some(get_text) = get_text_value {
            let text = get_text.run(key);
            update_input(text);
        }

        close_menu();
    };

    // ─── Public callbacks ───────────────────────────────────────────────

    let open = Callback::new(move |_| {
        if is_disabled.get_untracked() {
            return;
        }
        set_internal_open.set(true);
        if let Some(on_change) = on_open_change {
            on_change.run(true);
        }
    });

    let close = Callback::new(move |_| {
        close_menu();
    });

    let toggle = Callback::new(move |_| {
        if is_disabled.get_untracked() {
            return;
        }
        if is_open.get_untracked() {
            close_menu();
        } else {
            set_show_all.set(true);
            set_internal_open.set(true);
            if let Some(on_change) = on_open_change {
                on_change.run(true);
            }
        }
    });

    let select = Callback::new(move |key: K| {
        select_and_close(key);
    });

    let set_input_value = Callback::new(move |value: String| {
        update_input(value);
    });

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

    let listbox_id_for_option = listbox_id.clone();
    let get_option_id =
        Callback::new(move |key: K| format!("{listbox_id_for_option}-option-{key}"));

    // ─── Event handlers ─────────────────────────────────────────────────

    // Handle input event
    let handle_input = move |e: Event| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }

        if let Ok(input_el) = e.expect_target().dyn_into::<web_sys::HtmlInputElement>() {
            let new_value = input_el.value();
            update_input(new_value);

            // Reset show-all when the user types.
            set_show_all.set(false);

            // Clear focused key when input changes.
            set_focused_key.set(None);

            // Open menu on input (unless manual trigger mode).
            if menu_trigger != MenuTriggerAction::Manual && !is_open.get_untracked() {
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

        // Ignore events during IME composition.
        if e.is_composing() {
            return;
        }

        let filtered = filtered_items.get_untracked();
        let disabled = disabled_keys.get_untracked();

        match e.key().as_str() {
            "ArrowDown" => {
                e.prevent_default();
                if !is_open.get_untracked() {
                    // Open menu and focus first non-disabled item.
                    set_internal_open.set(true);
                    if let Some(on_change) = on_open_change {
                        on_change.run(true);
                    }
                    let first = find_adjacent_enabled(&filtered, None, &disabled, true, false);
                    set_focused_key.set(first);
                } else if !filtered.is_empty() {
                    let current = focused_key.get_untracked();
                    let next = find_adjacent_enabled(
                        &filtered,
                        current.as_ref(),
                        &disabled,
                        true,
                        should_focus_wrap,
                    );
                    if let Some(next) = next {
                        set_focused_key.set(Some(next));
                    }
                }
            }
            "ArrowUp" => {
                e.prevent_default();
                if !is_open.get_untracked() {
                    // Open menu and focus last non-disabled item.
                    set_internal_open.set(true);
                    if let Some(on_change) = on_open_change {
                        on_change.run(true);
                    }
                    let last = find_adjacent_enabled(&filtered, None, &disabled, false, false);
                    set_focused_key.set(last);
                } else if !filtered.is_empty() {
                    let current = focused_key.get_untracked();
                    let prev = find_adjacent_enabled(
                        &filtered,
                        current.as_ref(),
                        &disabled,
                        false,
                        should_focus_wrap,
                    );
                    if let Some(prev) = prev {
                        set_focused_key.set(Some(prev));
                    }
                }
            }
            "ArrowLeft" | "ArrowRight" => {
                // Clear virtual focus, returning to input cursor navigation.
                set_focused_key.set(None);
            }
            "Enter" => {
                // Prevent form submission when menu is open.
                if is_open.get_untracked() {
                    e.prevent_default();
                }
                // Commit: select the focused item, or commit current value.
                if is_open.get_untracked() {
                    if let Some(key) = focused_key.get_untracked() {
                        if selected_key.get_untracked().as_ref() == Some(&key) {
                            commit_selection();
                        } else {
                            select_and_close(key);
                        }
                    } else {
                        commit_value();
                    }
                } else {
                    commit_value();
                }
            }
            "Tab" => {
                // Commit without preventing default (allow normal tab navigation).
                if is_open.get_untracked() {
                    if let Some(key) = focused_key.get_untracked() {
                        if selected_key.get_untracked().as_ref() == Some(&key) {
                            commit_selection();
                        } else {
                            select_and_close(key);
                        }
                    } else {
                        commit_value();
                    }
                } else {
                    commit_value();
                }
            }
            "Escape" => {
                if is_open.get_untracked() {
                    e.prevent_default();
                }
                // Revert: reset input to selected item text (or commit custom value).
                if allows_custom_value && selected_key.get_untracked().is_none() {
                    commit_custom_value();
                } else {
                    commit_selection();
                }
            }
            "Home" => {
                if is_open.get_untracked() && !filtered.is_empty() {
                    e.prevent_default();
                    let first = find_adjacent_enabled(&filtered, None, &disabled, true, false);
                    set_focused_key.set(first);
                }
            }
            "End" => {
                if is_open.get_untracked() && !filtered.is_empty() {
                    e.prevent_default();
                    let last = find_adjacent_enabled(&filtered, None, &disabled, false, false);
                    set_focused_key.set(last);
                }
            }
            _ => {}
        }
    };

    // Handle focus
    let handle_focus = move |_e: FocusEvent| {
        if menu_trigger == MenuTriggerAction::Focus
            && !is_open.get_untracked()
            && !is_read_only.get_untracked()
        {
            set_show_all.set(true);
            set_internal_open.set(true);
            if let Some(on_change) = on_open_change {
                on_change.run(true);
            }
        }
    };

    // Handle blur
    let button_id_for_blur = button_id.clone();
    let handle_blur = move |e: FocusEvent| {
        // When relatedTarget is null, focus is lost to the body (e.g., tab switch).
        // We don't close on null — interact_outside handles that case.
        let Some(related_target) = e.related_target() else {
            return;
        };

        // Ignore blur if focus moved to the button.
        if let Some(el) = related_target.dyn_ref::<web_sys::Element>() {
            if el.id() == button_id_for_blur {
                return;
            }
        }

        // Ignore blur if focus moved into the popover.
        if let Some(popover_el) = popover_element.get_untracked() {
            if node_contains(
                Some(popover_el.unchecked_ref::<web_sys::Node>()),
                related_target.dyn_ref::<web_sys::Node>(),
            )
            .unwrap_or(false)
            {
                return;
            }
        }

        // Focus left the combobox — commit value.
        commit_value();
    };

    // Handle button click
    let handle_button_click = move |_e: MouseEvent| {
        if is_disabled.get_untracked() {
            return;
        }
        if is_open.get_untracked() {
            close_menu();
        } else {
            set_show_all.set(true);
            set_internal_open.set(true);
            if let Some(on_change) = on_open_change {
                on_change.run(true);
            }
        }
    };

    // ─── ARIA attributes ────────────────────────────────────────────────

    let aria_expanded = Signal::derive(move || Some(AriaExpanded::from(is_open.get())));
    let aria_required = is_required.then_some(AriaRequired::True);

    // aria-controls: only reference the listbox when it exists in the DOM (i.e., when open).
    let listbox_id_for_controls = listbox_id.clone();
    let aria_controls = Signal::derive(move || {
        if is_open.get() {
            Some(listbox_id_for_controls.clone())
        } else {
            None
        }
    });

    // aria-activedescendant: stable ID matching the focused option element.
    let listbox_id_for_aria = listbox_id.clone();
    let aria_activedescendant = Signal::derive(move || {
        focused_key
            .get()
            .map(|key| format!("{listbox_id_for_aria}-option-{key}"))
    });

    // ─── ariaHideOutside ────────────────────────────────────────────────

    #[cfg(not(feature = "ssr"))]
    {
        use crate::utils::aria_hide_outside::{AriaHideOutsideOptions, aria_hide_outside};

        let hide_cleanup: StoredValue<Option<Box<dyn FnOnce()>>, LocalStorage> =
            StoredValue::new_local(None);

        Effect::new(move |_| {
            // Clean up previous hide (if any).
            hide_cleanup.update_value(|opt| {
                if let Some(f) = opt.take() {
                    f();
                }
            });

            if is_open.get() {
                let mut targets = Vec::new();
                if let Some(el) = input_element.get() {
                    targets.push((*el).clone());
                }
                if let Some(el) = popover_element.get() {
                    targets.push((*el).clone());
                }
                if !targets.is_empty() {
                    let undo = aria_hide_outside(&targets, AriaHideOutsideOptions::default());
                    hide_cleanup.set_value(Some(undo));
                }
            }
        });

        on_cleanup(move || {
            hide_cleanup.update_value(|opt| {
                if let Some(f) = opt.take() {
                    f();
                }
            });
        });
    }

    // ─── Live announcements for screen readers ──────────────────────────

    {
        use crate::utils::live_announcer::announce_polite;

        let last_open_state = StoredValue::new(false);
        let last_option_count = StoredValue::new(0usize);

        // Announce option count when menu opens or count changes while open.
        Effect::new(move |_| {
            let open = is_open.get();
            let count = filtered_items.get().len();
            let was_open = last_open_state.get_value();
            let was_count = last_option_count.get_value();

            if open && (!was_open || count != was_count) {
                let msg = match count {
                    0 => "No options available.".to_string(),
                    1 => "1 option available.".to_string(),
                    n => format!("{n} options available."),
                };
                announce_polite(msg);
            }

            last_open_state.set_value(open);
            last_option_count.set_value(count);
        });

        // Announce focused item changes.
        let last_focused: StoredValue<Option<K>> = StoredValue::new(None);
        Effect::new(move |_| {
            let current = focused_key_signal.get();
            let prev = last_focused.get_value();

            if current != prev {
                if let Some(ref key) = current {
                    if is_open.get_untracked() {
                        if let Some(get_text) = get_text_value {
                            let text = get_text.run(key.clone());
                            let is_selected = selected_key.get_untracked().as_ref() == Some(key);
                            let msg = if is_selected {
                                format!("{text}, selected")
                            } else {
                                text
                            };
                            announce_polite(msg);
                        }
                    }
                }
            }

            last_focused.set_value(current);
        });

        // Announce selection changes.
        let last_selected: StoredValue<Option<K>> = StoredValue::new(None);
        Effect::new(move |_| {
            let current = selected_key.get();
            let prev = last_selected.get_value();

            if current != prev {
                if let Some(ref key) = current {
                    if let Some(get_text) = get_text_value {
                        let text = get_text.run(key.clone());
                        announce_polite(format!("{text}, selected"));
                    }
                }
            }

            last_selected.set_value(current);
        });
    }

    // ─── Return ─────────────────────────────────────────────────────────

    UseComboBoxReturn {
        input_props: UseComboBoxInputProps {
            id: input_id.clone(),
            r#type: "text",
            role: AriaRole::Combobox,
            value: input_value,
            placeholder,
            disabled: is_disabled,
            readonly: is_read_only,
            aria_autocomplete: "list",
            aria_haspopup: "listbox",
            aria_expanded,
            aria_controls,
            aria_label,
            aria_labelledby,
            aria_required,
            aria_activedescendant,
            spellcheck: "false",
            on_input: EventHandler::new(handle_input),
            on_keydown: EventHandler::new(handle_keydown),
            on_focus: EventHandler::new(handle_focus),
            on_blur: EventHandler::new(handle_blur),
            element_capture: input_element.attr(),
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
            role: AriaRole::Listbox,
            aria_labelledby: input_id.clone(),
            tabindex: "-1",
        },
        popover_props: UseComboBoxPopoverProps {
            element_capture: popover_element.attr(),
        },
        input_id,
        listbox_id,
        is_open,
        input_value,
        selected_key,
        filtered_items,
        focused_key: focused_key_signal,
        open,
        close,
        toggle,
        select,
        set_input_value,
        clear,
        get_option_id,
    }
}

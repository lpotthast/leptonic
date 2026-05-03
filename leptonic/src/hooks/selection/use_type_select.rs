use leptos::{
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use web_sys::KeyboardEvent;

use super::SelectionKey;
use crate::{
    hooks::IntoAttrs,
    utils::{
        EventHandler,
        filter::{CollatorOptions, CollatorSensitivity, Filter},
        i18n::use_locale_or_default,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/selection/src/useTypeSelect.ts

// REACT-ARIA DEVIATIONS
//
// No intentional deviations from the react-aria implementation.

/// Input parameters for the `use_type_select` hook.
#[derive(Clone)]
pub struct UseTypeSelectInput<K>
where
    K: SelectionKey,
{
    /// Whether type select is disabled.
    pub disabled: Signal<bool>,

    /// All available keys in the collection.
    pub all_keys: Signal<Vec<K>>,

    /// Function to get the text label for a key (used for matching).
    pub get_key_label: Callback<K, String>,

    /// The currently focused key.
    pub focused_key: Signal<Option<K>>,

    /// Callback to set the focused key.
    pub on_focus: Callback<Option<K>>,

    /// Timeout for resetting the search string (default: 1000ms, matching react-aria).
    pub timeout_ms: u64,
}

impl<K: SelectionKey> Default for UseTypeSelectInput<K> {
    fn default() -> Self {
        Self {
            disabled: Signal::derive(|| false),
            all_keys: Signal::derive(Vec::new),
            get_key_label: Callback::new(|_| String::new()),
            focused_key: Signal::derive(|| None),
            on_focus: Callback::new(|_| {}),
            timeout_ms: 1000,
        }
    }
}

/// The return value of the `use_type_select` hook.
#[derive(Debug)]
pub struct UseTypeSelectReturn {
    /// Props for the container element. Call `.into_attrs()` for view spreading.
    pub type_select_props: UseTypeSelectProps,

    /// The current search string.
    pub search_string: Signal<String>,

    /// Clear the search string.
    pub clear_search: Callback<()>,

    /// The keyboard event handler callback. Can be called directly to delegate keyboard handling.
    pub on_keydown: Callback<KeyboardEvent>,
}

/// Props from `use_type_select` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseTypeSelectProps {
    pub on_keydown: EventHandler<KeyboardEvent>,
}

impl IntoAttrs for UseTypeSelectProps {
    type Attrs = UseTypeSelectAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (self.on_keydown.into_on(ev::keydown),)
    }
}

/// Attributes for the type select container element.
pub type UseTypeSelectAttrs = (On<ev::keydown, SharedEventCallback<KeyboardEvent>>,);

/// Enables type-ahead selection in a collection.
///
/// Users can type characters to jump to items that start with those characters.
/// The search string resets after a timeout (default 500ms).
///
/// # Example
///
/// ```ignore
/// let items = vec!["Apple", "Banana", "Cherry"];
/// let all_keys = Signal::derive(move || items.iter().map(|s| s.to_string()).collect());
///
/// let type_select = use_type_select(UseTypeSelectInput {
///     disabled: Signal::derive(|| false),
///     all_keys,
///     get_key_label: Callback::new(|key: String| key.clone()),
///     focused_key: collection.focused_key,
///     on_focus: collection.set_focused_key,
///     timeout_ms: 500,
/// });
///
/// view! {
///     <ul role="listbox" {..type_select.type_select_props.into_attrs()}>
///         // Items here
///     </ul>
/// }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_type_select<K>(input: UseTypeSelectInput<K>) -> UseTypeSelectReturn
where
    K: SelectionKey,
{
    // Get current time in milliseconds using wasm-bindgen
    fn now_ms() -> f64 {
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace = Date)]
            fn now() -> f64;
        }

        now()
    }

    let UseTypeSelectInput {
        disabled,
        all_keys,
        get_key_label,
        focused_key,
        on_focus,
        timeout_ms,
    } = input;

    // Search string state
    let (search_string, set_search_string) = signal(String::new());

    // Last keypress timestamp for timeout
    let last_keypress: StoredValue<f64> = StoredValue::new(0.0);

    // Clear search string
    let clear_search = Callback::new(move |_: ()| {
        set_search_string.set(String::new());
    });

    // Handle keydown for type select - exposed as a callback for delegation
    let on_keydown = Callback::new(move |e: KeyboardEvent| {
        if disabled.get_untracked() {
            return;
        }

        let key = e.key();

        // Only handle printable characters
        if key.len() != 1 {
            return;
        }

        // Ignore if modifier keys are pressed (except shift for uppercase)
        if e.ctrl_key() || e.alt_key() || e.meta_key() {
            return;
        }

        // Check if timeout has elapsed (needed for space handling below).
        let current_time = now_ms();
        let last_time = last_keypress.get_value();
        #[allow(clippy::cast_precision_loss)]
        let timeout = timeout_ms as f64;
        let should_reset = current_time - last_time > timeout;

        // Determine if the search is effectively active. If the timeout has elapsed,
        // the search buffer will be reset by the current keystroke, so it's effectively empty.
        let search_active = !search_string.get_untracked().is_empty() && !should_reset;

        // Ignore space when there's no active search — let it bubble so the
        // selection handler can use it. Space is only consumed by type-ahead when
        // there's already accumulated text from recent keystrokes.
        if key == " " && !search_active {
            return;
        }

        // When Space is part of an active search, prevent it from triggering selection
        // and stop propagation so the collection's keyboard handler doesn't see it.
        if key == " " && search_active {
            e.stop_propagation();
        }

        e.prevent_default();

        // Update last keypress time
        last_keypress.set_value(current_time);

        // Append to search string (or start new search if timeout elapsed)
        let new_search = if should_reset {
            key.to_lowercase()
        } else {
            let current = search_string.get_untracked();
            format!("{}{}", current, key.to_lowercase())
        };
        set_search_string.set(new_search.clone());

        // Find matching item using locale-aware comparison
        let locale_filter = Filter::new(
            &use_locale_or_default(),
            &CollatorOptions {
                sensitivity: CollatorSensitivity::Base,
                ..CollatorOptions::default()
            },
        );

        let keys = all_keys.get_untracked();
        let current_focused = focused_key.get_untracked();

        // Find the current index
        let current_idx = current_focused
            .as_ref()
            .and_then(|k| keys.iter().position(|item_key| item_key == k))
            .unwrap_or(0);

        // Search from current position, wrapping around
        let len = keys.len();
        for i in 0..len {
            let idx = (current_idx + i) % len;
            let item_key = &keys[idx];
            let label = get_key_label.run(item_key.clone());

            if locale_filter.starts_with(&label, &new_search) {
                on_focus.run(Some(item_key.clone()));
                return;
            }
        }

        // If single character and no match found from current, search from beginning
        if new_search.len() == 1 {
            for item_key in &keys {
                let label = get_key_label.run(item_key.clone());
                if locale_filter.starts_with(&label, &new_search) {
                    on_focus.run(Some(item_key.clone()));
                    return;
                }
            }
        }
    });

    // Create event handler for type_select_props using the callback
    let handle_keydown = move |e: KeyboardEvent| {
        on_keydown.run(e);
    };

    UseTypeSelectReturn {
        type_select_props: UseTypeSelectProps {
            on_keydown: EventHandler::new(handle_keydown),
        },
        search_string: search_string.into(),
        clear_search,
        on_keydown,
    }
}

use leptos::{attr, attr::Attr, ev, ev::On, ev::SharedEventCallback, prelude::*};

use crate::{
    hooks::{
        IntoAttrs,
        form::{
            use_form_reset::{UseFormResetInput, use_form_reset},
            use_form_validation::{UseFormValidationInput, use_form_validation},
            use_form_validation_state::{UseFormValidationStateReturn, ValidationBehavior},
        },
        selection::{
            SelectionKey,
            use_selection_state::{Selection, SelectionMode, SelectionSet},
        },
    },
    utils::{
        EventHandler,
        aria::AriaHidden,
        element_capture::{CapturedElement, ElementCaptureAttr},
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/select/src/useHiddenSelect.ts

// REACT-ARIA DEVIATIONS
//
// ## API DIFFERENCES
//
// - Accepts `Selection<K>` instead of a state object. The caller passes the
//   selection signal from `use_select`'s return value.
// - `key_to_value` is used for form value serialization (react-aria reads
//   `textValue` from collection nodes).

/// The threshold for switching between native `<select>` and `<input>` rendering.
/// Collections with at most this many items render a native `<select>` with `<option>` elements
/// for browser autofill support. Larger collections use `<input type="hidden">` to avoid DOM bloat.
const NATIVE_SELECT_THRESHOLD: usize = 300;

/// Input parameters for the `use_hidden_select` hook.
#[derive(Clone)]
pub struct UseHiddenSelectInput<K>
where
    K: SelectionKey,
{
    /// The name attribute for form submission.
    pub name: Option<&'static str>,

    /// Whether the field is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the field is required.
    pub is_required: bool,

    /// The currently selected keys.
    pub selected_keys: Signal<Selection<K>>,

    /// The selection mode.
    pub selection_mode: SelectionMode,

    /// A function to convert a key to a string value for form submission.
    pub key_to_value: Option<Callback<K, String>>,

    /// The ID of the select trigger element.
    pub trigger_id: String,

    /// The label for the select (for accessibility).
    pub label: Option<String>,

    /// The validation behavior.
    pub validation_behavior: ValidationBehavior,

    /// The `form` attribute to associate with a specific form.
    pub form: Option<&'static str>,

    /// The `autocomplete` attribute.
    pub auto_complete: Option<&'static str>,

    /// All available items (for native `<select>` + `<option>` rendering).
    pub items: Signal<Vec<K>>,

    /// A function to get the display text for an item (for `<option>` labels).
    pub get_text_value: Option<Callback<K, String>>,

    /// Callback fired when the native `<select>` changes value (browser autofill).
    pub on_change: Option<Callback<K>>,

    /// The default selected keys (for form reset).
    pub default_selected_keys: Option<Selection<K>>,

    /// Callback to restore selection on form reset.
    pub on_reset: Option<Callback<Selection<K>>>,

    /// The validation state (for native constraint validation integration).
    pub validation_state: Option<UseFormValidationStateReturn>,
}

impl<K: SelectionKey> Default for UseHiddenSelectInput<K> {
    fn default() -> Self {
        Self {
            name: None,
            is_disabled: Signal::derive(|| false),
            is_required: false,
            selected_keys: Signal::derive(|| Selection::Keys(SelectionSet::new())),
            selection_mode: SelectionMode::Single,
            key_to_value: None,
            trigger_id: String::new(),
            label: None,
            validation_behavior: ValidationBehavior::Aria,
            form: None,
            auto_complete: None,
            items: Signal::derive(Vec::new),
            get_text_value: None,
            on_change: None,
            default_selected_keys: None,
            on_reset: None,
            validation_state: None,
        }
    }
}

/// An option item for rendering in a native `<select>`.
#[derive(Debug, Clone)]
pub struct HiddenSelectOption {
    /// The key serialized as a form value.
    pub key_value: String,
    /// The display text for the option.
    pub text: String,
    /// Whether this option is currently selected.
    pub is_selected: bool,
}

/// The return value of the `use_hidden_select` hook.
pub struct UseHiddenSelectReturn {
    /// Props for the container div that holds the hidden elements.
    pub container_props: UseHiddenSelectContainerProps,

    /// Props for the hidden input element.
    pub input_props: UseHiddenSelectInputProps,

    /// Props for the hidden select element.
    pub select_props: UseHiddenSelectSelectProps,

    /// The current value for the hidden input.
    pub value: Signal<String>,

    /// Whether the consumer should render a native `<select>` with `<option>` elements
    /// (true when items <= 300) or fall back to `<input>` elements.
    pub use_native_select: Signal<bool>,

    /// Option items for rendering native `<option>` elements.
    /// Only meaningful when `use_native_select` is true.
    pub options: Signal<Vec<HiddenSelectOption>>,
}

/// Props from `use_hidden_select` for the container that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseHiddenSelectContainerProps {
    pub aria_hidden: AriaHidden,
}

impl IntoAttrs for UseHiddenSelectContainerProps {
    type Attrs = UseHiddenSelectContainerAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::AriaHidden, self.aria_hidden),)
    }
}

/// Attributes for the hidden container.
pub type UseHiddenSelectContainerAttrs = (Attr<attr::AriaHidden, AriaHidden>,);

/// The style string for hiding the container visually.
pub const HIDDEN_SELECT_CONTAINER_STYLE: &str = "position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0, 0, 0, 0); white-space: nowrap; border: 0;";

/// Props from `use_hidden_select` for the input element that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseHiddenSelectInputProps {
    pub r#type: &'static str,
    pub name: Option<&'static str>,
    pub value: Signal<String>,
    pub disabled: Signal<bool>,
    pub required: bool,
    pub tabindex: &'static str,
    pub form: Option<&'static str>,
    pub auto_complete: Option<&'static str>,
    pub element_capture: ElementCaptureAttr,
}

impl IntoAttrs for UseHiddenSelectInputProps {
    type Attrs = UseHiddenSelectInputAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Type, self.r#type),
            Attr(attr::Name, self.name),
            Attr(attr::Value, self.value),
            Attr(attr::Disabled, self.disabled),
            Attr(attr::Required, self.required),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::Form, self.form),
            Attr(attr::Autocomplete, self.auto_complete),
            self.element_capture,
        )
    }
}

/// Attributes for the hidden input element.
pub type UseHiddenSelectInputAttrs = (
    Attr<attr::Type, &'static str>,
    Attr<attr::Name, Option<&'static str>>,
    Attr<attr::Value, Signal<String>>,
    Attr<attr::Disabled, Signal<bool>>,
    Attr<attr::Required, bool>,
    Attr<attr::Tabindex, &'static str>,
    Attr<attr::Form, Option<&'static str>>,
    Attr<attr::Autocomplete, Option<&'static str>>,
    ElementCaptureAttr,
);

/// Props from `use_hidden_select` for the select element that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseHiddenSelectSelectProps {
    pub name: Option<&'static str>,
    pub disabled: Signal<bool>,
    pub required: bool,
    pub multiple: bool,
    pub tabindex: &'static str,
    pub aria_labelledby: Option<String>,
    pub form: Option<&'static str>,
    pub auto_complete: Option<&'static str>,
    pub on_change: EventHandler<web_sys::Event>,
}

impl IntoAttrs for UseHiddenSelectSelectProps {
    type Attrs = UseHiddenSelectSelectAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Name, self.name),
            Attr(attr::Disabled, self.disabled),
            Attr(attr::Required, self.required),
            Attr(attr::Multiple, self.multiple),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::Form, self.form),
            Attr(attr::Autocomplete, self.auto_complete),
            self.on_change.into_on(ev::change),
        )
    }
}

/// Attributes for the hidden select element.
pub type UseHiddenSelectSelectAttrs = (
    Attr<attr::Name, Option<&'static str>>,
    Attr<attr::Disabled, Signal<bool>>,
    Attr<attr::Required, bool>,
    Attr<attr::Multiple, bool>,
    Attr<attr::Tabindex, &'static str>,
    Attr<attr::AriaLabelledby, Option<String>>,
    Attr<attr::Form, Option<&'static str>>,
    Attr<attr::Autocomplete, Option<&'static str>>,
    On<ev::change, SharedEventCallback<web_sys::Event>>,
);

/// Provides a hidden native select element for form submission and mobile accessibility.
///
/// This is useful for integrating with native form submission and providing
/// a better experience on mobile devices where native selects can be triggered.
///
/// # Example
///
/// ```ignore
/// let hidden = use_hidden_select(UseHiddenSelectInput {
///     name: Some("fruit"),
///     selected_keys: select.selected_keys,
///     trigger_id: select.trigger_id.clone(),
///     key_to_value: Some(Callback::new(|k: String| k)),
///     items: select.menu_config.items,
///     ..Default::default()
/// });
///
/// view! {
///     <div style=HIDDEN_SELECT_CONTAINER_STYLE {..hidden.container_props.into_attrs()}>
///         <Show when=move || hidden.use_native_select.get()>
///             <select {..hidden.select_props.into_attrs()}>
///                 <For
///                     each=move || hidden.options.get()
///                     key=|opt| opt.key_value.clone()
///                     children=move |opt| {
///                         view! { <option value=opt.key_value.clone() selected=opt.is_selected>{opt.text.clone()}</option> }
///                     }
///                 />
///             </select>
///         </Show>
///         <Show when=move || !hidden.use_native_select.get()>
///             <input {..hidden.input_props.into_attrs()} />
///         </Show>
///     </div>
/// }
/// ```
#[allow(clippy::needless_pass_by_value, clippy::too_many_lines)]
pub fn use_hidden_select<K>(input: UseHiddenSelectInput<K>) -> UseHiddenSelectReturn
where
    K: SelectionKey,
{
    let UseHiddenSelectInput {
        name,
        is_disabled,
        is_required,
        selected_keys,
        selection_mode,
        key_to_value,
        trigger_id,
        label,
        validation_behavior,
        form,
        auto_complete,
        items,
        get_text_value,
        on_change,
        default_selected_keys,
        on_reset,
        validation_state,
    } = input;

    // Element capture for form reset and validation integration.
    let input_element = CapturedElement::new();

    // Compute the value for the hidden input.
    // For single mode: the single selected key's value.
    // For multiple mode: comma-separated values.
    let value = Signal::derive(move || match selected_keys.get() {
        Selection::Keys(keys) => {
            let values: Vec<String> = keys
                .into_iter()
                .map(|key| {
                    if let Some(converter) = key_to_value {
                        converter.run(key)
                    } else {
                        format!("{key}")
                    }
                })
                .collect();
            values.join(",")
        }
        Selection::All => String::new(),
    });

    // Whether to render a native <select> with <option> elements (for browser autofill).
    let use_native_select =
        Signal::derive(move || items.with(|v| v.len() <= NATIVE_SELECT_THRESHOLD));

    // Option items for native <select> rendering.
    let options = Signal::derive(move || {
        items.with(|all_items| {
            let current = selected_keys.get();
            all_items
                .iter()
                .map(|key| {
                    let key_val = if let Some(converter) = key_to_value {
                        converter.run(key.clone())
                    } else {
                        format!("{key}")
                    };
                    let text = if let Some(get_text) = get_text_value {
                        get_text.run(key.clone())
                    } else {
                        format!("{key}")
                    };
                    let is_selected = match &current {
                        Selection::Keys(set) => set.contains(key),
                        Selection::All => true,
                    };
                    HiddenSelectOption {
                        key_value: key_val,
                        text,
                        is_selected,
                    }
                })
                .collect()
        })
    });

    // onChange handler for native <select> autofill.
    let handle_change = move |e: web_sys::Event| {
        if let Some(on_change) = on_change {
            use wasm_bindgen::JsCast;
            if let Some(select_el) = e
                .target()
                .and_then(|t| t.dyn_into::<web_sys::HtmlSelectElement>().ok())
            {
                let val = select_el.value();
                // Find the key matching this value.
                let found = items.with_untracked(|all_items| {
                    all_items
                        .iter()
                        .find(|k| {
                            let k_val = if let Some(converter) = key_to_value {
                                converter.run((*k).clone())
                            } else {
                                format!("{k}")
                            };
                            k_val == val
                        })
                        .cloned()
                });
                if let Some(key) = found {
                    on_change.run(key);
                }
            }
        }
    };

    // Build aria-labelledby.
    let aria_labelledby = if label.is_some() {
        Some(trigger_id)
    } else {
        None
    };

    // For native validation, use type="text" instead of "hidden" so HTML required works.
    let input_type = if validation_behavior == ValidationBehavior::Native && is_required {
        "text"
    } else {
        "hidden"
    };

    // --- Form reset integration ---
    if let (Some(default), Some(reset_cb)) = (default_selected_keys, on_reset) {
        use_form_reset(UseFormResetInput {
            element: input_element,
            initial_value: default,
            on_reset: reset_cb,
        });
    }

    // --- Form validation integration ---
    if let Some(vs) = validation_state {
        use_form_validation(UseFormValidationInput {
            element: input_element,
            state: vs,
            validation_behavior,
        });
    }

    UseHiddenSelectReturn {
        container_props: UseHiddenSelectContainerProps {
            aria_hidden: AriaHidden::True,
        },
        input_props: UseHiddenSelectInputProps {
            r#type: input_type,
            name,
            value,
            disabled: is_disabled,
            required: is_required,
            tabindex: "-1",
            form,
            auto_complete,
            element_capture: input_element.attr(),
        },
        select_props: UseHiddenSelectSelectProps {
            name,
            disabled: is_disabled,
            required: is_required,
            multiple: selection_mode == SelectionMode::Multiple,
            tabindex: "-1",
            aria_labelledby,
            form,
            auto_complete,
            on_change: EventHandler::new(handle_change),
        },
        value,
        use_native_select,
        options,
    }
}

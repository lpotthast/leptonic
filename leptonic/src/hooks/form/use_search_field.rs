use leptos::attr;
use leptos::attr::{Attr, Attribute};
use leptos::ev;
use leptos::ev::{on, On, SharedEventCallback};
use leptos::prelude::*;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;

use super::use_field::ValidationState;
use crate::hooks::focus::use_focus_ring::{use_focus_ring, UseFocusRingInput, UseFocusRingReturn};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/searchfield/src/useSearchField.ts

/// Input parameters for the `use_search_field` hook.
#[derive(Clone)]
pub struct UseSearchFieldInput {
    /// The current value (controlled).
    pub value: Signal<String>,

    /// Callback when the value changes.
    pub on_change: Option<Callback<String>>,

    /// Callback when the field is cleared.
    pub on_clear: Option<Callback<()>>,

    /// Callback when the user submits the search.
    pub on_submit: Option<Callback<String>>,

    /// Callback when input receives focus.
    pub on_focus: Option<Callback<()>>,

    /// Callback when input loses focus.
    pub on_blur: Option<Callback<()>>,

    /// Whether the field is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the field is read-only.
    pub is_read_only: Signal<bool>,

    /// The validation state of the field.
    pub validation_state: ValidationState,

    /// Placeholder text.
    pub placeholder: Option<&'static str>,

    /// An accessibility label for the field.
    pub aria_label: Option<&'static str>,

    /// The name attribute for form submission.
    pub name: Option<&'static str>,

    /// The label for the field.
    pub label: Option<String>,

    /// A description for the field.
    pub description: Option<String>,

    /// Maximum length for the input value.
    pub max_length: Option<u32>,

    /// Whether to auto-focus the input on mount.
    pub auto_focus: bool,
}

impl Default for UseSearchFieldInput {
    fn default() -> Self {
        Self {
            value: Signal::derive(String::new),
            on_change: None,
            on_clear: None,
            on_submit: None,
            on_focus: None,
            on_blur: None,
            is_disabled: Signal::derive(|| false),
            is_read_only: Signal::derive(|| false),
            validation_state: ValidationState::Valid,
            placeholder: None,
            aria_label: None,
            name: None,
            label: None,
            description: None,
            max_length: None,
            auto_focus: false,
        }
    }
}

/// The return value of the `use_search_field` hook.
#[derive(Clone)]
pub struct UseSearchFieldReturn {
    /// Props for the input element.
    pub input_props: UseSearchFieldInputAttrs,

    /// Props for the clear button element.
    pub clear_button_props: UseSearchFieldClearButtonAttrs,

    /// Props for the label element.
    pub label_props: UseSearchFieldLabelProps,

    /// Props for the description element.
    pub description_props: UseSearchFieldDescriptionProps,

    /// Whether there is a value to clear.
    pub show_clear_button: Signal<bool>,

    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,
}

/// Attributes for the search field input element.
pub type UseSearchFieldInputAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Type, &'static str>,
    Attr<attr::Role, &'static str>,
    Attr<attr::Name, Option<&'static str>>,
    Attr<attr::Value, Signal<String>>,
    Attr<attr::Placeholder, Option<&'static str>>,
    Attr<attr::Disabled, Signal<bool>>,
    Attr<attr::Readonly, Signal<bool>>,
    Attr<attr::AriaLabel, Option<&'static str>>,
    Attr<attr::AriaLabelledby, Option<String>>,
    Attr<attr::AriaDescribedby, Option<String>>,
    Attr<attr::AriaInvalid, Option<&'static str>>,
    Attr<attr::Maxlength, Option<u32>>,
    Attr<attr::Autofocus, bool>,
    On<ev::input, SharedEventCallback<web_sys::Event>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<web_sys::FocusEvent>>,
    On<ev::blur, SharedEventCallback<web_sys::FocusEvent>>,
    leptos::attr::custom::CustomAttr<&'static str, Signal<Option<&'static str>>>,
);

/// Attributes for the clear button element.
pub type UseSearchFieldClearButtonAttrs = (
    Attr<attr::Type, &'static str>,
    Attr<attr::AriaLabel, &'static str>,
    Attr<attr::Tabindex, &'static str>,
    Attr<attr::Disabled, Signal<bool>>,
    On<ev::click, SharedEventCallback<web_sys::MouseEvent>>,
);

/// Props for the label element.
#[derive(Debug, Clone)]
pub struct UseSearchFieldLabelProps {
    /// The id of the label element.
    pub id: String,

    /// The "for" attribute linking to the input.
    pub html_for: String,
}

/// Props for the description element.
#[derive(Debug, Clone)]
pub struct UseSearchFieldDescriptionProps {
    /// The id of the description element.
    pub id: String,
}

/// Provides the behavior and accessibility implementation for a search field.
///
/// Search fields allow users to search with a clear button and submit functionality.
///
/// # Example
///
/// ```ignore
/// let (query, set_query) = signal(String::new());
///
/// let search_field = use_search_field(UseSearchFieldInput {
///     value: query.into(),
///     on_change: Some(Callback::new(move |v| {
///         set_query.set(v);
///     })),
///     on_submit: Some(Callback::new(|query| {
///         // Perform search
///     })),
///     on_clear: Some(Callback::new(|_| {
///         // Handle clear
///     })),
///     placeholder: Some("Search..."),
///     aria_label: Some("Search"),
///     ..Default::default()
/// });
///
/// view! {
///     <div>
///         <input {..search_field.input_props} />
///         <Show when=move || search_field.show_clear_button.get()>
///             <button {..search_field.clear_button_props}>"Clear"</button>
///         </Show>
///     </div>
/// }
/// ```
#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn use_search_field(input: UseSearchFieldInput) -> UseSearchFieldReturn {
    let base_id = Uuid::new_v4();
    let input_id = format!("searchfield-{base_id}");
    let label_id = format!("searchfield-label-{base_id}");
    let description_id = format!("searchfield-description-{base_id}");

    let value = input.value;
    let on_change = input.on_change;
    let on_clear = input.on_clear;
    let on_submit = input.on_submit;
    let on_focus = input.on_focus;
    let on_blur = input.on_blur;
    let is_disabled = input.is_disabled;
    let is_read_only = input.is_read_only;

    // Whether to show the clear button
    let show_clear_button = Signal::derive(move || !value.get().is_empty());

    // Handle input event
    let handle_input = move |e: web_sys::Event| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }

        if let Some(input_el) = e
            .target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
        {
            let new_value = input_el.value();
            if let Some(on_change) = on_change {
                on_change.run(new_value);
            }
        }
    };

    // Handle keydown event (Enter to submit, Escape to clear)
    let handle_keydown = move |e: KeyboardEvent| {
        if is_disabled.get_untracked() {
            return;
        }

        match e.key().as_str() {
            "Enter" => {
                if let Some(on_submit) = on_submit {
                    e.prevent_default();
                    on_submit.run(value.get_untracked());
                }
            }
            "Escape" => {
                if !value.get_untracked().is_empty() {
                    e.prevent_default();
                    // Clear the value
                    if let Some(on_change) = on_change {
                        on_change.run(String::new());
                    }
                    if let Some(on_clear) = on_clear {
                        on_clear.run(());
                    }
                }
            }
            _ => {}
        }
    };

    // Use focus ring to track focus visibility with user callbacks
    let UseFocusRingReturn {
        props: focus_ring_props,
        is_focus_visible,
        is_focused: _,
    } = use_focus_ring(UseFocusRingInput {
        disabled: is_disabled,
        within: false,
        auto_focus: input.auto_focus,
        on_focus: on_focus.map(|cb| Callback::new(move |_| cb.run(()))),
        on_blur: on_blur.map(|cb| Callback::new(move |_| cb.run(()))),
        on_focus_change: None,
    });
    let (handle_focus, handle_blur, data_focus_visible) = focus_ring_props.into_attrs();

    // Handle clear button click
    let handle_clear = move |_e: web_sys::MouseEvent| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }

        if let Some(on_change) = on_change {
            on_change.run(String::new());
        }
        if let Some(on_clear) = on_clear {
            on_clear.run(());
        }
    };

    // Build aria-labelledby
    let aria_labelledby = if input.label.is_some() {
        Some(label_id.clone())
    } else {
        None
    };

    // Build aria-describedby
    let aria_describedby = if input.description.is_some() {
        Some(description_id.clone())
    } else {
        None
    };

    // Compute aria-invalid
    let aria_invalid = if input.validation_state == ValidationState::Invalid {
        Some("true")
    } else {
        None
    };

    UseSearchFieldReturn {
        input_props: (
            Attr(attr::Id, input_id.clone()),
            Attr(attr::Type, "search"),
            Attr(attr::Role, "searchbox"),
            Attr(attr::Name, input.name),
            Attr(attr::Value, value),
            Attr(attr::Placeholder, input.placeholder),
            Attr(attr::Disabled, is_disabled),
            Attr(attr::Readonly, is_read_only),
            Attr(attr::AriaLabel, input.aria_label),
            Attr(attr::AriaLabelledby, aria_labelledby),
            Attr(attr::AriaDescribedby, aria_describedby),
            Attr(attr::AriaInvalid, aria_invalid),
            Attr(attr::Maxlength, input.max_length),
            Attr(attr::Autofocus, input.auto_focus),
            on(ev::input, handle_input).into_cloneable(),
            on(ev::keydown, handle_keydown).into_cloneable(),
            handle_focus,
            handle_blur,
            data_focus_visible,
        ),
        clear_button_props: (
            Attr(attr::Type, "button"),
            Attr(attr::AriaLabel, "Clear search"),
            Attr(attr::Tabindex, "-1"),
            Attr(attr::Disabled, is_disabled),
            on(ev::click, handle_clear).into_cloneable(),
        ),
        label_props: UseSearchFieldLabelProps {
            id: label_id,
            html_for: input_id,
        },
        description_props: UseSearchFieldDescriptionProps { id: description_id },
        show_clear_button,
        is_focus_visible,
    }
}

/// Creates internal state for a search field component.
pub fn use_search_field_state() -> UseSearchFieldStateReturn {
    let (value, set_value) = signal(String::new());

    UseSearchFieldStateReturn {
        value: value.into(),
        set_value: Callback::new(move |v: String| {
            set_value.set(v);
        }),
        clear: Callback::new(move |_| {
            set_value.set(String::new());
        }),
    }
}

/// State for managing search field state.
#[derive(Clone, Copy)]
pub struct UseSearchFieldStateReturn {
    /// The current value.
    pub value: Signal<String>,

    /// Set the value.
    pub set_value: Callback<String>,

    /// Clear the value.
    pub clear: Callback<()>,
}

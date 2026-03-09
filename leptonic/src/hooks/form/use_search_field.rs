use leptos::{
    attr,
    attr::{
        custom::{custom_attribute, CustomAttr},
        Attr,
    },
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;

use super::use_field::ValidationState;
use crate::{
    hooks::{
        focus::use_focus_ring::{use_focus_ring, UseFocusRingInput, UseFocusRingReturn},
        IntoAttrs,
    },
    utils::{
        aria::{AriaInvalid, AriaRole},
        EventAccessors, EventHandler,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/searchfield/src/useSearchField.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

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
pub struct UseSearchFieldReturn {
    /// Props for the input element. Call `.into_attrs()` for view spreading.
    pub input_props: UseSearchFieldInputProps,

    /// Props for the clear button element. Call `.into_attrs()` for view spreading.
    pub clear_button_props: UseSearchFieldClearButtonProps,

    /// Props for the label element. Call `.into_attrs()` for view spreading.
    pub label_props: UseSearchFieldLabelProps,

    /// Props for the description element. Call `.into_attrs()` for view spreading.
    pub description_props: UseSearchFieldDescriptionProps,

    /// Whether there is a value to clear.
    pub show_clear_button: Signal<bool>,

    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,
}

/// Props from `use_search_field` for the input element.
#[derive(Debug)]
pub struct UseSearchFieldInputProps {
    pub id: String,
    pub r#type: &'static str,
    pub role: AriaRole,
    pub name: Option<&'static str>,
    pub value: Signal<String>,
    pub placeholder: Option<&'static str>,
    pub disabled: Signal<bool>,
    pub readonly: Signal<bool>,
    pub aria_label: Option<&'static str>,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Option<String>,
    pub aria_invalid: Option<AriaInvalid>,
    pub maxlength: Option<u32>,
    pub autofocus: bool,
    pub data_focus_visible: Signal<Option<&'static str>>,
    pub on_input: EventHandler<web_sys::Event>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<web_sys::FocusEvent>,
    pub on_blur: EventHandler<web_sys::FocusEvent>,
    pub on_focusin: EventHandler<web_sys::FocusEvent>,
    pub on_focusout: EventHandler<web_sys::FocusEvent>,
}

impl IntoAttrs for UseSearchFieldInputProps {
    type Attrs = UseSearchFieldInputAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Type, self.r#type),
            Attr(attr::Role, self.role),
            Attr(attr::Name, self.name),
            Attr(attr::Value, self.value),
            Attr(attr::Placeholder, self.placeholder),
            Attr(attr::Disabled, self.disabled),
            Attr(attr::Readonly, self.readonly),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            Attr(attr::AriaInvalid, self.aria_invalid),
            Attr(attr::Maxlength, self.maxlength),
            Attr(attr::Autofocus, self.autofocus),
            custom_attribute("data-focus-visible", self.data_focus_visible),
            self.on_input.into_on(ev::input),
            self.on_keydown.into_on(ev::keydown),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
        )
    }
}

/// Attributes for the search field input element.
pub type UseSearchFieldInputAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Type, &'static str>,
    Attr<attr::Role, AriaRole>,
    Attr<attr::Name, Option<&'static str>>,
    Attr<attr::Value, Signal<String>>,
    Attr<attr::Placeholder, Option<&'static str>>,
    Attr<attr::Disabled, Signal<bool>>,
    Attr<attr::Readonly, Signal<bool>>,
    Attr<attr::AriaLabel, Option<&'static str>>,
    Attr<attr::AriaLabelledby, Option<String>>,
    Attr<attr::AriaDescribedby, Option<String>>,
    Attr<attr::AriaInvalid, Option<AriaInvalid>>,
    Attr<attr::Maxlength, Option<u32>>,
    Attr<attr::Autofocus, bool>,
    CustomAttr<&'static str, Signal<Option<&'static str>>>,
    On<ev::input, SharedEventCallback<web_sys::Event>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<web_sys::FocusEvent>>,
    On<ev::blur, SharedEventCallback<web_sys::FocusEvent>>,
    On<ev::focusin, SharedEventCallback<web_sys::FocusEvent>>,
    On<ev::focusout, SharedEventCallback<web_sys::FocusEvent>>,
);

/// Props from `use_search_field` for the clear button element.
#[derive(Debug)]
pub struct UseSearchFieldClearButtonProps {
    pub r#type: &'static str,
    pub aria_label: &'static str,
    pub tabindex: &'static str,
    pub disabled: Signal<bool>,
    pub on_click: EventHandler<web_sys::MouseEvent>,
}

impl IntoAttrs for UseSearchFieldClearButtonProps {
    type Attrs = UseSearchFieldClearButtonAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Type, self.r#type),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::Disabled, self.disabled),
            self.on_click.into_on(ev::click),
        )
    }
}

/// Attributes for the clear button element.
pub type UseSearchFieldClearButtonAttrs = (
    Attr<attr::Type, &'static str>,
    Attr<attr::AriaLabel, &'static str>,
    Attr<attr::Tabindex, &'static str>,
    Attr<attr::Disabled, Signal<bool>>,
    On<ev::click, SharedEventCallback<web_sys::MouseEvent>>,
);

/// Props for the label element.
#[derive(Debug)]
pub struct UseSearchFieldLabelProps {
    /// The id of the label element.
    pub id: String,

    /// The "for" attribute linking to the input.
    pub html_for: String,
}

impl IntoAttrs for UseSearchFieldLabelProps {
    type Attrs = UseSearchFieldLabelAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Id, self.id), Attr(attr::For, self.html_for))
    }
}

/// Attributes for the search field label element.
pub type UseSearchFieldLabelAttrs = (Attr<attr::Id, String>, Attr<attr::For, String>);

/// Props for the description element.
#[derive(Debug)]
pub struct UseSearchFieldDescriptionProps {
    /// The id of the description element.
    pub id: String,
}

impl IntoAttrs for UseSearchFieldDescriptionProps {
    type Attrs = UseSearchFieldDescriptionAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Id, self.id),)
    }
}

/// Attributes for the search field description element.
pub type UseSearchFieldDescriptionAttrs = (Attr<attr::Id, String>,);

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
///         <input {..search_field.input_props.into_attrs()} />
///         <Show when=move || search_field.show_clear_button.get()>
///             <button {..search_field.clear_button_props.into_attrs()}>"Clear"</button>
///         </Show>
///     </div>
/// }
/// ```
#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn use_search_field(input: UseSearchFieldInput) -> UseSearchFieldReturn {
    let UseSearchFieldInput {
        value,
        on_change,
        on_clear,
        on_submit,
        on_focus,
        on_blur,
        is_disabled,
        is_read_only,
        validation_state,
        placeholder,
        aria_label,
        name,
        label,
        description,
        max_length,
        auto_focus,
    } = input;

    let base_id = Uuid::new_v4();
    let input_id = format!("searchfield-{base_id}");
    let label_id = format!("searchfield-label-{base_id}");
    let description_id = format!("searchfield-description-{base_id}");

    // Whether to show the clear button
    let show_clear_button = Signal::derive(move || !value.get().is_empty());

    // Handle input event
    let handle_input = move |e: web_sys::Event| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }

        if let Ok(input_el) = e.expect_target().dyn_into::<web_sys::HtmlInputElement>() {
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
        auto_focus,
        is_text_input: true,
        on_focus: on_focus.map(|cb| Callback::new(move |_| cb.run(()))),
        on_blur: on_blur.map(|cb| Callback::new(move |_| cb.run(()))),
        on_focus_change: None,
    });

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
    let aria_labelledby = if label.is_some() {
        Some(label_id.clone())
    } else {
        None
    };

    // Build aria-describedby
    let aria_describedby = if description.is_some() {
        Some(description_id.clone())
    } else {
        None
    };

    // Compute aria-invalid
    let aria_invalid = (validation_state == ValidationState::Invalid).then_some(AriaInvalid::True);

    UseSearchFieldReturn {
        input_props: UseSearchFieldInputProps {
            id: input_id.clone(),
            r#type: "search",
            role: AriaRole::Searchbox,
            name,
            value,
            placeholder,
            disabled: is_disabled,
            readonly: is_read_only,
            aria_label,
            aria_labelledby,
            aria_describedby,
            aria_invalid,
            maxlength: max_length,
            autofocus: auto_focus,
            data_focus_visible: focus_ring_props.data_focus_visible,
            on_input: EventHandler::new(handle_input),
            on_keydown: EventHandler::new(handle_keydown),
            on_focus: focus_ring_props.on_focus,
            on_blur: focus_ring_props.on_blur,
            on_focusin: focus_ring_props.on_focusin,
            on_focusout: focus_ring_props.on_focusout,
        },
        clear_button_props: UseSearchFieldClearButtonProps {
            r#type: "button",
            aria_label: "Clear search",
            tabindex: "-1",
            disabled: is_disabled,
            on_click: EventHandler::new(handle_clear),
        },
        label_props: UseSearchFieldLabelProps {
            id: label_id,
            html_for: input_id,
        },
        description_props: UseSearchFieldDescriptionProps { id: description_id },
        show_clear_button,
        is_focus_visible,
    }
}

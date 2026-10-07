// Upstream: react-aria/src/textfield/useTextField.ts @ 99e6102368
use leptos::{
    attr::{self, Attr},
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
    tachys::html::property::{Property, prop},
};
use wasm_bindgen::JsCast;
use web_sys::{Event, FocusEvent};

use super::{
    use_field::{UseFieldInput, UseFieldReturn, use_field},
    use_form_reset::{UseFormResetInput, use_form_reset},
    use_form_validation::{UseFormValidationInput, use_form_validation},
    use_form_validation_state::{
        FormValidationState, UseFormValidationStateInput, ValidateFn, ValidationBehavior,
        ValidityStateSnapshot, use_form_validation_state,
    },
    use_label::{LabelElementType, UseLabelProps},
    use_text_field_state::TextFieldState,
};
use crate::{
    hooks::{
        IntoAttrs,
        focus::{
            use_focus_ring::{UseFocusRingInput, use_focus_ring},
            use_focusable::{
                UseFocusableAttrs, UseFocusableInput, UseFocusableProps, use_focusable,
            },
        },
        interactions::use_keyboard::KeyboardEventWrapper,
    },
    utils::{
        CapturedElement, EventAccessors, EventHandler, SlotProps,
        aria::{AriaAutocomplete, AriaHasPopup, AriaInvalid, AriaRequired},
        keyboard_shortcut::KeyboardShortcuts,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Hook-owned state: the value lives in a `TextFieldState` (`use_text_field_state`, or one a
//   composite state such as a combo box provides) instead of `value`/`defaultValue`/`onChange`.
// - Typed attribute values (`InputType`, `InputMode`, `EnterKeyHint`, `AutoCapitalize`).
// - DOM event props that react-aria only passes through (`onCopy`, `onCompositionStart`,
//   `onInput`, ...) are not taken: attach such listeners to the element directly.
// - `has_label` says whether a visible label is rendered (react-aria: the `label` content).
// - The value is rendered as an attribute (server-side rendering) and kept in sync as the DOM
//   property, as a controlled React input does.
//
// =============================================================================

/// The element a text field renders.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TextFieldElement {
    #[default]
    Input,
    TextArea,
}

/// The `type` of an `<input>` text field.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum InputType {
    #[default]
    Text,
    Search,
    Url,
    Tel,
    Email,
    Password,
}

impl InputType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Search => "search",
            Self::Url => "url",
            Self::Tel => "tel",
            Self::Email => "email",
            Self::Password => "password",
        }
    }
}

/// The kind of virtual keyboard to show (`inputmode`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    None,
    Text,
    Decimal,
    Numeric,
    Tel,
    Search,
    Email,
    Url,
}

impl InputMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Text => "text",
            Self::Decimal => "decimal",
            Self::Numeric => "numeric",
            Self::Tel => "tel",
            Self::Search => "search",
            Self::Email => "email",
            Self::Url => "url",
        }
    }
}

/// The label of the virtual keyboard's enter key (`enterkeyhint`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnterKeyHint {
    Enter,
    Done,
    Go,
    Next,
    Previous,
    Search,
    Send,
}

impl EnterKeyHint {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Enter => "enter",
            Self::Done => "done",
            Self::Go => "go",
            Self::Next => "next",
            Self::Previous => "previous",
            Self::Search => "search",
            Self::Send => "send",
        }
    }
}

/// Automatic capitalization of typed text (`autocapitalize`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoCapitalize {
    None,
    Sentences,
    Words,
    Characters,
}

impl AutoCapitalize {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Sentences => "sentences",
            Self::Words => "words",
            Self::Characters => "characters",
        }
    }
}

/// Input of [`use_text_field`].
#[derive(Clone)]
#[allow(clippy::struct_excessive_bools)]
pub struct UseTextFieldInput {
    pub state: TextFieldState,
    /// The input element's id. Generated when `None`.
    pub id: Option<String>,
    pub element: TextFieldElement,
    /// `<input>` only. Reactive, e.g. for a "show password" toggle.
    pub input_type: Signal<InputType>,
    pub is_disabled: Signal<bool>,
    pub is_read_only: Signal<bool>,
    pub is_required: Signal<bool>,
    /// Marks the value invalid, regardless of `validate`.
    pub is_invalid: Signal<bool>,
    pub validate: Option<ValidateFn<String>>,
    pub validation_behavior: ValidationBehavior,
    /// The validation state of a field built on this text field (e.g. a number field, whose
    /// value isn't the text), replacing the text field's own (`is_invalid` and `validate` are
    /// then the composite field's business).
    pub validation: Option<FormValidationState>,
    pub name: Option<String>,
    pub form: Option<String>,
    pub placeholder: MaybeProp<String>,
    /// `<input>` only.
    pub pattern: Option<String>,
    pub min_length: Option<u32>,
    pub max_length: Option<u32>,
    /// The `autocomplete` hint, e.g. `"email"` or `"off"`.
    pub auto_complete: Option<String>,
    pub auto_capitalize: Option<AutoCapitalize>,
    pub auto_correct: Option<bool>,
    pub spell_check: Option<bool>,
    pub input_mode: Option<InputMode>,
    pub enter_key_hint: Option<EnterKeyHint>,
    pub auto_focus: bool,
    pub exclude_from_tab_order: bool,
    /// The label element's id. Generated when `None`.
    pub label_id: Option<String>,
    /// Whether a visible label is rendered (with `label_props`).
    pub has_label: Signal<bool>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Option<String>,
    pub aria_errormessage: Option<String>,
    pub aria_activedescendant: Signal<Option<String>>,
    pub aria_autocomplete: Option<AriaAutocomplete>,
    pub aria_haspopup: Option<AriaHasPopup>,
    pub aria_controls: Signal<Option<String>>,
    pub on_focus: Option<Callback<FocusEvent>>,
    pub on_blur: Option<Callback<FocusEvent>>,
    pub on_focus_change: Option<Callback<bool>>,
    pub on_key_down: Option<Callback<KeyboardEventWrapper>>,
    pub on_key_up: Option<Callback<KeyboardEventWrapper>>,
    /// Keyboard shortcuts handled while the field has focus.
    pub shortcuts: Option<KeyboardShortcuts>,
}

/// Return value of [`use_text_field`].
pub struct UseTextFieldReturn {
    pub label_props: UseLabelProps,
    pub input_props: UseTextFieldInputProps,
    pub description_props: SlotProps,
    /// For the error message element. Render it only while the field is invalid.
    pub error_message_props: SlotProps,
    /// The `<input>` or `<textarea>`, once rendered.
    pub element: CapturedElement,
    pub is_focused: Signal<bool>,
    pub is_focus_visible: Signal<bool>,
    pub is_invalid: Signal<bool>,
    pub validation_errors: Signal<Vec<String>>,
    pub validation_details: Signal<ValidityStateSnapshot>,
}

/// Props for the `<input>` or `<textarea>` element.
#[derive(Debug, Clone)]
pub struct UseTextFieldInputProps {
    pub id: String,
    pub r#type: Signal<Option<&'static str>>,
    pub pattern: Option<String>,
    /// The `value` attribute when rendered (server-side rendering); `value_property` keeps it in
    /// sync. `None` for a `<textarea>`, which renders its initial value as its content instead.
    pub value: Option<String>,
    pub value_property: Signal<String>,
    pub disabled: Signal<bool>,
    pub readonly: Signal<bool>,
    pub required: Signal<bool>,
    pub name: Option<String>,
    pub form: Option<String>,
    pub placeholder: MaybeProp<String>,
    pub minlength: Option<u32>,
    pub maxlength: Option<u32>,
    pub autocomplete: Option<String>,
    pub autocapitalize: Option<&'static str>,
    pub autocorrect: Option<&'static str>,
    pub spellcheck: Option<&'static str>,
    pub inputmode: Option<&'static str>,
    pub enterkeyhint: Option<&'static str>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<String>>,
    pub aria_describedby: Signal<Option<String>>,
    pub aria_required: Signal<Option<AriaRequired>>,
    pub aria_invalid: Signal<Option<AriaInvalid>>,
    pub aria_errormessage: Option<String>,
    pub aria_activedescendant: Signal<Option<String>>,
    pub aria_autocomplete: Option<AriaAutocomplete>,
    pub aria_haspopup: Option<AriaHasPopup>,
    pub aria_controls: Signal<Option<String>>,
    /// Focus, keyboard and tab order (`use_focusable`); captures the element.
    pub focusable: UseFocusableProps,
    pub on_input: EventHandler<Event>,
}

pub type UseTextFieldInputAttrs = (
    (
        Attr<attr::Id, String>,
        Attr<attr::Type, Signal<Option<&'static str>>>,
        Attr<attr::Pattern, Option<String>>,
        Attr<attr::Value, Option<String>>,
        Property<&'static str, Signal<String>>,
        Attr<attr::Disabled, Signal<bool>>,
        Attr<attr::Readonly, Signal<bool>>,
        Attr<attr::Required, Signal<bool>>,
        Attr<attr::Name, Option<String>>,
        Attr<attr::Form, Option<String>>,
        Attr<attr::Placeholder, MaybeProp<String>>,
        Attr<attr::Minlength, Option<u32>>,
        Attr<attr::Maxlength, Option<u32>>,
        Attr<attr::Autocomplete, Option<String>>,
        Attr<attr::Autocapitalize, Option<&'static str>>,
        leptos::attr::custom::CustomAttr<&'static str, Option<&'static str>>,
        Attr<attr::Spellcheck, Option<&'static str>>,
        Attr<attr::Inputmode, Option<&'static str>>,
        Attr<attr::Enterkeyhint, Option<&'static str>>,
    ),
    (
        Attr<attr::AriaLabel, MaybeProp<String>>,
        Attr<attr::AriaLabelledby, Signal<Option<String>>>,
        Attr<attr::AriaDescribedby, Signal<Option<String>>>,
        Attr<attr::AriaRequired, Signal<Option<AriaRequired>>>,
        Attr<attr::AriaInvalid, Signal<Option<AriaInvalid>>>,
        Attr<attr::AriaErrormessage, Option<String>>,
        Attr<attr::AriaActivedescendant, Signal<Option<String>>>,
        Attr<attr::AriaAutocomplete, Option<AriaAutocomplete>>,
        Attr<attr::AriaHaspopup, Option<AriaHasPopup>>,
        Attr<attr::AriaControls, Signal<Option<String>>>,
    ),
    UseFocusableAttrs,
    On<ev::input, SharedEventCallback<Event>>,
);

impl IntoAttrs for UseTextFieldInputProps {
    type Attrs = UseTextFieldInputAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            (
                Attr(attr::Id, self.id),
                Attr(attr::Type, self.r#type),
                Attr(attr::Pattern, self.pattern),
                Attr(attr::Value, self.value),
                prop("value", self.value_property),
                Attr(attr::Disabled, self.disabled),
                Attr(attr::Readonly, self.readonly),
                Attr(attr::Required, self.required),
                Attr(attr::Name, self.name),
                Attr(attr::Form, self.form),
                Attr(attr::Placeholder, self.placeholder),
                Attr(attr::Minlength, self.minlength),
                Attr(attr::Maxlength, self.maxlength),
                Attr(attr::Autocomplete, self.autocomplete),
                Attr(attr::Autocapitalize, self.autocapitalize),
                leptos::attr::custom::custom_attribute("autocorrect", self.autocorrect),
                Attr(attr::Spellcheck, self.spellcheck),
                Attr(attr::Inputmode, self.inputmode),
                Attr(attr::Enterkeyhint, self.enterkeyhint),
            ),
            (
                Attr(attr::AriaLabel, self.aria_label),
                Attr(attr::AriaLabelledby, self.aria_labelledby),
                Attr(attr::AriaDescribedby, self.aria_describedby),
                Attr(attr::AriaRequired, self.aria_required),
                Attr(attr::AriaInvalid, self.aria_invalid),
                Attr(attr::AriaErrormessage, self.aria_errormessage),
                Attr(attr::AriaActivedescendant, self.aria_activedescendant),
                Attr(attr::AriaAutocomplete, self.aria_autocomplete),
                Attr(attr::AriaHaspopup, self.aria_haspopup),
                Attr(attr::AriaControls, self.aria_controls),
            ),
            self.focusable.into_attrs(),
            self.on_input.into_on(ev::input),
        )
    }
}

/// A text field: an `<input>` or `<textarea>` with a label, description and error message,
/// validation, form reset and focus handling.
///
/// ```ignore
/// let state = use_text_field_state(UseTextFieldStateInput::default());
/// // Every field named: `state`, `has_label: Signal::stored(true)`, the others their defaults
/// // (`element: TextFieldElement::Input`, `input_type: Signal::stored(InputType::Text)`, ...).
/// let field = use_text_field(input);
/// view! {
///     <label {..field.label_props.into_attrs()}>"Name"</label>
///     <input {..field.input_props.into_attrs()} />
/// }
/// ```
///
/// For a `<textarea>` (`element: TextFieldElement::TextArea`), render the state's initial value
/// as its content: `<textarea {..props}>{initial_value}</textarea>`.
#[allow(clippy::too_many_lines)]
pub fn use_text_field(input: UseTextFieldInput) -> UseTextFieldReturn {
    let UseTextFieldInput {
        state,
        id,
        element: element_type,
        input_type,
        is_disabled,
        is_read_only,
        is_required,
        is_invalid,
        validate,
        validation_behavior,
        validation,
        name,
        form,
        placeholder,
        pattern,
        min_length,
        max_length,
        auto_complete,
        auto_capitalize,
        auto_correct,
        spell_check,
        input_mode,
        enter_key_hint,
        auto_focus,
        exclude_from_tab_order,
        label_id,
        has_label,
        aria_label,
        aria_labelledby,
        aria_describedby,
        aria_errormessage,
        aria_activedescendant,
        aria_autocomplete,
        aria_haspopup,
        aria_controls,
        on_focus,
        on_blur,
        on_focus_change,
        on_key_down,
        on_key_up,
        shortcuts,
    } = input;

    let focusable = use_focusable(UseFocusableInput {
        is_disabled,
        auto_focus,
        exclude_from_tab_order: Signal::stored(exclude_from_tab_order),
        on_focus,
        on_blur,
        on_focus_change,
        on_key_down,
        on_key_up,
        shortcuts,
        ..UseFocusableInput::default()
    });
    let element = focusable.focus_handle.element();
    let mut focusable_props = focusable.props;

    // Focus ring state (keyboard focus), as react-aria-components adds it.
    let focus_ring = use_focus_ring(UseFocusRingInput {
        is_disabled,
        within: false,
        auto_focus,
        is_text_input: true,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });
    focusable_props.on_focus = focusable_props.on_focus.chain(focus_ring.props.on_focus);
    focusable_props.on_blur = focusable_props.on_blur.chain(focus_ring.props.on_blur);

    let validation = validation.unwrap_or_else(|| {
        use_form_validation_state(UseFormValidationStateInput {
            builtin_validation: Signal::default(),
            is_invalid,
            value: state.value,
            validate,
            validation_behavior,
            name: name.clone(),
        })
    });
    let initial_value = state.value.get_untracked();
    use_form_reset(UseFormResetInput {
        element,
        initial_value: initial_value.clone(),
        on_reset: Callback::new(move |value| state.set_value(value)),
    });
    use_form_validation(UseFormValidationInput {
        element,
        state: validation,
        validation_behavior,
        focus: None,
    });

    let UseFieldReturn {
        label_props,
        field_props,
        description_props,
        error_message_props,
        ..
    } = use_field(UseFieldInput {
        id,
        label_id,
        has_label,
        label_element_type: LabelElementType::Label,
        aria_label,
        aria_labelledby,
        aria_describedby,
    });

    let on_input = EventHandler::new(move |e: Event| {
        let target = e.expect_target();
        let input = target.dyn_ref::<web_sys::HtmlInputElement>();
        let text_area = target.dyn_ref::<web_sys::HtmlTextAreaElement>();
        let Some(value) = input
            .map(web_sys::HtmlInputElement::value)
            .or_else(|| text_area.map(web_sys::HtmlTextAreaElement::value))
        else {
            return;
        };
        state.set_value(value.clone());
        // The state may have rejected or changed the text (a bound value): show what it holds,
        // as React does for a controlled input.
        let held = state.value.get_untracked();
        if held != value {
            if let Some(input) = input {
                input.set_value(&held);
            } else if let Some(text_area) = text_area {
                text_area.set_value(&held);
            }
        }
    });

    let is_input = element_type == TextFieldElement::Input;
    let on_off = |on: bool| if on { "on" } else { "off" };

    UseTextFieldReturn {
        label_props,
        input_props: UseTextFieldInputProps {
            id: field_props.id,
            r#type: Signal::derive(move || is_input.then(|| input_type.get().as_str())),
            pattern: pattern.filter(|_| is_input),
            value: is_input.then_some(initial_value),
            value_property: state.value,
            disabled: is_disabled,
            readonly: is_read_only,
            required: Signal::derive(move || {
                is_required.get() && validation_behavior == ValidationBehavior::Native
            }),
            name,
            form,
            placeholder,
            minlength: min_length,
            maxlength: max_length,
            autocomplete: auto_complete,
            autocapitalize: auto_capitalize.map(AutoCapitalize::as_str),
            autocorrect: auto_correct.map(on_off),
            spellcheck: spell_check.map(|on| if on { "true" } else { "false" }),
            inputmode: input_mode.map(InputMode::as_str),
            enterkeyhint: enter_key_hint.map(EnterKeyHint::as_str),
            aria_label: field_props.aria_label,
            aria_labelledby: field_props.aria_labelledby,
            aria_describedby: {
                // The field's own description, then a `FocusableContext`'s (e.g. a tooltip).
                let own = field_props.aria_describedby;
                let context = focusable_props.context_aria_describedby;
                Signal::derive(move || {
                    let ids: Vec<String> = own.get().into_iter().chain(context.get()).collect();
                    (!ids.is_empty()).then(|| ids.join(" "))
                })
            },
            aria_required: Signal::derive(move || {
                (is_required.get() && validation_behavior == ValidationBehavior::Aria)
                    .then_some(AriaRequired::True)
            }),
            aria_invalid: Signal::derive(move || {
                validation.is_invalid.get().then_some(AriaInvalid::True)
            }),
            aria_errormessage,
            aria_activedescendant,
            aria_autocomplete,
            aria_haspopup,
            aria_controls,
            focusable: focusable_props,
            on_input,
        },
        description_props,
        error_message_props,
        element,
        is_focused: focus_ring.is_focused,
        is_focus_visible: focus_ring.is_focus_visible,
        is_invalid: validation.is_invalid,
        validation_errors: validation.validation_errors,
        validation_details: Signal::derive(move || {
            validation.display_validation.get().validation_details
        }),
    }
}

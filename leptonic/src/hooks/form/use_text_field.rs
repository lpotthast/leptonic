// Upstream: react-aria/src/textfield/useTextField.ts @ 99e6102368
// Upstream: react-aria/test/textfield/useTextField.test.js @ 99e6102368
// Upstream: react-aria-components/test/TextField.test.js @ 99e6102368
use leptos::{
    ev,
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
    CapturedElement, EventHandler, Id, IdRefs, IntoAttrs, OnEvent, Part, SlotProps,
    hooks::{
        focus::{
            FocusRingTarget, UseFocusRingInput, UseFocusableAttrs, UseFocusableInput,
            UseFocusableProps, use_focus_ring, use_focusable,
        },
        interactions::KeyboardEventWrapper,
    },
    utils::{
        aria::{AriaAutocomplete, AriaHasPopup, AriaInvalid, AriaRequired, aria_true},
        dom_ext::EventAccessors,
        keyboard_shortcut::KeyboardShortcuts,
        part::Remount,
        signal_attr::{FixedAttr, SignalAttr, fixed_attr, maybe_attr, signal_attr},
        spread::Spread,
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
pub struct UseTextFieldInput {
    pub state: TextFieldState,
    /// The input element's id. Generated when `None`.
    pub id: Option<Id>,
    /// The element rendering the field (react-aria's `inputElementType`): `type`, `pattern` and
    /// the `value` attribute exist only on an `<input>`. Reactive: a field whose element is
    /// chosen when it mounts (the `TextField` atom's `Input` or `TextArea`) sets it then.
    pub element: Signal<TextFieldElement>,
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
    /// The virtual keyboard (`inputmode`); a signal, as a number field's depends on the platform.
    pub input_mode: Signal<Option<InputMode>>,
    pub enter_key_hint: Option<EnterKeyHint>,
    pub auto_focus: bool,
    pub exclude_from_tab_order: bool,
    /// The label element's id. Generated when `None`.
    pub label_id: Option<Id>,
    /// Whether a visible label is rendered (with `label_props`).
    pub has_label: Signal<bool>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<Id>>,
    pub aria_describedby: Signal<Option<Id>>,
    pub aria_errormessage: Signal<Option<Id>>,
    pub aria_activedescendant: Signal<Option<Id>>,
    pub aria_autocomplete: Option<AriaAutocomplete>,
    pub aria_haspopup: Option<AriaHasPopup>,
    pub aria_controls: Signal<Option<Id>>,
    pub on_focus: Option<Callback<FocusEvent>>,
    pub on_blur: Option<Callback<FocusEvent>>,
    pub on_focus_change: Option<Callback<bool>>,
    pub on_key_down: Option<Callback<KeyboardEventWrapper>>,
    pub on_key_up: Option<Callback<KeyboardEventWrapper>>,
    /// Keyboard shortcuts handled while the field has focus.
    pub shortcuts: Option<KeyboardShortcuts>,
}

/// The return value of [`use_text_field`].
#[derive(Debug)]
pub struct UseTextFieldReturn {
    /// The label element.
    pub label: Part<UseLabelProps>,
    /// The input (or text area) element.
    pub input: Part<UseTextFieldInputProps>,
    pub description: Part<SlotProps>,
    /// For the error message element. Render it only while the field is invalid.
    pub error_message: Part<SlotProps>,
    /// The `<input>` or `<textarea>`, once rendered.
    pub element: CapturedElement,
    pub is_focused: Signal<bool>,
    pub is_focus_visible: Signal<bool>,
    pub is_invalid: Signal<bool>,
    pub validation_errors: Signal<Vec<String>>,
    pub validation_details: Signal<ValidityStateSnapshot>,
}

/// Props for the `<input>` or `<textarea>` element.
#[derive(Debug)]
pub struct UseTextFieldInputProps {
    pub id: Id,
    pub r#type: Signal<Option<&'static str>>,
    pub pattern: Signal<Option<String>>,
    /// The `value` attribute when rendered (server-side rendering); `value_property` keeps it in
    /// sync. `None` for a `<textarea>`, which renders its initial value as its content instead.
    pub value: Signal<Option<String>>,
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
    pub inputmode: Signal<Option<&'static str>>,
    pub enterkeyhint: Option<&'static str>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<Id>>,
    pub aria_describedby: Signal<Option<Id>>,
    pub aria_required: Signal<Option<AriaRequired>>,
    pub aria_invalid: Signal<Option<AriaInvalid>>,
    pub aria_errormessage: Signal<Option<Id>>,
    pub aria_activedescendant: Signal<Option<Id>>,
    pub aria_autocomplete: Option<AriaAutocomplete>,
    pub aria_haspopup: Option<AriaHasPopup>,
    pub aria_controls: Signal<Option<Id>>,
    /// Focus, keyboard and tab order (`use_focusable`); captures the element.
    pub focusable: UseFocusableProps,
    pub on_input: EventHandler<Event>,
}

impl Remount for UseTextFieldInputProps {
    fn remount(&self) -> Self {
        Self {
            id: self.id.clone(),
            r#type: self.r#type,
            pattern: self.pattern,
            value: self.value,
            value_property: self.value_property,
            disabled: self.disabled,
            readonly: self.readonly,
            required: self.required,
            name: self.name.clone(),
            form: self.form.clone(),
            placeholder: self.placeholder,
            minlength: self.minlength,
            maxlength: self.maxlength,
            autocomplete: self.autocomplete.clone(),
            autocapitalize: self.autocapitalize,
            autocorrect: self.autocorrect,
            spellcheck: self.spellcheck,
            inputmode: self.inputmode,
            enterkeyhint: self.enterkeyhint,
            aria_label: self.aria_label,
            aria_labelledby: self.aria_labelledby,
            aria_describedby: self.aria_describedby,
            aria_required: self.aria_required,
            aria_invalid: self.aria_invalid,
            aria_errormessage: self.aria_errormessage,
            aria_activedescendant: self.aria_activedescendant,
            aria_autocomplete: self.aria_autocomplete,
            aria_haspopup: self.aria_haspopup,
            aria_controls: self.aria_controls,
            focusable: self.focusable.remount(),
            on_input: self.on_input.clone(),
        }
    }
}

pub type UseTextFieldInputAttrs = (
    Spread<(
        (
            FixedAttr,
            SignalAttr<Option<&'static str>>,
            SignalAttr<Option<String>>,
            SignalAttr<Option<String>>,
            SignalAttr<bool>,
            SignalAttr<bool>,
            SignalAttr<bool>,
            FixedAttr,
            FixedAttr,
            SignalAttr<Option<String>>,
        ),
        (
            FixedAttr,
            FixedAttr,
            FixedAttr,
            FixedAttr,
            FixedAttr,
            FixedAttr,
            SignalAttr<Option<&'static str>>,
            FixedAttr,
        ),
        (
            SignalAttr<Option<String>>,
            SignalAttr<Option<Id>>,
            SignalAttr<Option<Id>>,
            SignalAttr<Option<AriaRequired>>,
            SignalAttr<Option<AriaInvalid>>,
            SignalAttr<Option<Id>>,
            SignalAttr<Option<Id>>,
            FixedAttr,
            FixedAttr,
            SignalAttr<Option<Id>>,
        ),
    )>,
    Property<&'static str, Signal<String>>,
    UseFocusableAttrs,
    OnEvent<ev::input>,
);

impl IntoAttrs for UseTextFieldInputProps {
    type Attrs = UseTextFieldInputAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Spread((
                (
                    fixed_attr("id", self.id),
                    signal_attr("type", self.r#type),
                    signal_attr("pattern", self.pattern),
                    signal_attr("value", self.value),
                    signal_attr("disabled", self.disabled),
                    signal_attr("readonly", self.readonly),
                    signal_attr("required", self.required),
                    fixed_attr("name", self.name),
                    fixed_attr("form", self.form),
                    maybe_attr("placeholder", self.placeholder),
                ),
                (
                    fixed_attr("minlength", self.minlength),
                    fixed_attr("maxlength", self.maxlength),
                    fixed_attr("autocomplete", self.autocomplete),
                    fixed_attr("autocapitalize", self.autocapitalize),
                    fixed_attr("autocorrect", self.autocorrect),
                    fixed_attr("spellcheck", self.spellcheck),
                    signal_attr("inputmode", self.inputmode),
                    fixed_attr("enterkeyhint", self.enterkeyhint),
                ),
                (
                    maybe_attr("aria-label", self.aria_label),
                    signal_attr("aria-labelledby", self.aria_labelledby),
                    signal_attr("aria-describedby", self.aria_describedby),
                    signal_attr("aria-required", self.aria_required),
                    signal_attr("aria-invalid", self.aria_invalid),
                    signal_attr("aria-errormessage", self.aria_errormessage),
                    signal_attr("aria-activedescendant", self.aria_activedescendant),
                    fixed_attr("aria-autocomplete", self.aria_autocomplete),
                    fixed_attr("aria-haspopup", self.aria_haspopup),
                    signal_attr("aria-controls", self.aria_controls),
                ),
            )),
            prop("value", self.value_property),
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
/// // (`element: Signal::stored(TextFieldElement::Input)`,
/// // `input_type: Signal::stored(InputType::Text)`, ...).
/// let field = use_text_field(input);
/// view! {
///     <label {..field.label.attrs()}>"Name"</label>
///     <input {..field.input.attrs()} />
/// }
/// ```
///
/// For a `<textarea>` (`element` is `TextFieldElement::TextArea`), render the state's value as its
/// initial content: `<textarea {..props}>{state.value.get_untracked()}</textarea>`.
pub fn use_text_field(input: UseTextFieldInput) -> UseTextFieldReturn {
    crate::metrics::subject!();
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
        allow_shortcut_repeats: false,
    });
    let mut focusable_props = focusable.target.into_props();
    let element = focusable.focus_handle.element();

    // Focus ring state (keyboard focus), as react-aria-components adds it.
    let focus_ring = use_focus_ring(UseFocusRingInput {
        target: FocusRingTarget::Element,
        is_disabled,
        is_text_input: true,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });
    let focus_ring_props = focus_ring.target.into_props();
    focusable_props.on_focus = focusable_props.on_focus.chain(focus_ring_props.on_focus);
    focusable_props.on_blur = focusable_props.on_blur.chain(focus_ring_props.on_blur);

    let validation = validation.unwrap_or_else(|| {
        use_form_validation_state(UseFormValidationStateInput {
            builtin_validation: Signal::default(),
            is_invalid,
            value: state.value,
            validate,
            validation_behavior,
            names: name.clone().into_iter().collect(),
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
        label: label_props,
        field: field_props,
        description: description_props,
        error_message: error_message_props,
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
    let field_props = field_props.into_props();

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

    let is_input = move || element_type.get() == TextFieldElement::Input;
    let pattern = StoredValue::new(pattern);
    let on_off = |on: bool| if on { "on" } else { "off" };

    UseTextFieldReturn {
        label: label_props,
        input: Part::stored(UseTextFieldInputProps {
            id: field_props.id,
            r#type: Signal::derive(move || is_input().then(|| input_type.get().as_str())),
            pattern: Signal::derive(move || pattern.get_value().filter(|_| is_input())),
            value: Signal::derive(move || is_input().then(|| initial_value.clone())),
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
            inputmode: Signal::derive(move || input_mode.get().map(InputMode::as_str)),
            enterkeyhint: enter_key_hint.map(EnterKeyHint::as_str),
            aria_label: field_props.aria_label,
            aria_labelledby: field_props.aria_labelledby,
            // The field's own description, then a `FocusableContext`'s (e.g. a tooltip).
            aria_describedby: IdRefs::derive([
                field_props.aria_describedby,
                focusable_props.context_aria_describedby,
            ]),
            aria_required: Signal::derive(move || {
                (is_required.get() && validation_behavior == ValidationBehavior::Aria)
                    .then_some(AriaRequired::True)
            }),
            aria_invalid: aria_true(validation.is_invalid),
            aria_errormessage,
            aria_activedescendant,
            aria_autocomplete,
            aria_haspopup,
            aria_controls,
            focusable: focusable_props,
            on_input,
        }),
        description: description_props,
        error_message: error_message_props,
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

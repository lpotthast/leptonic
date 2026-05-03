use leptos::{
    attr,
    attr::{
        Attr,
        custom::{CustomAttr, custom_attribute},
    },
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{ClipboardEvent, CompositionEvent, Event, FocusEvent, InputEvent, KeyboardEvent};

use super::{
    use_form_reset::{UseFormResetInput, use_form_reset},
    use_form_validation::{UseFormValidationInput, use_form_validation},
    use_form_validation_state::{
        UseFormValidationStateInput, ValidateFn, ValidationBehavior, ValidityStateSnapshot,
        use_form_validation_state,
    },
};
use crate::{
    hooks::{
        IntoAttrs,
        focus::use_focus_ring::{UseFocusRingInput, UseFocusRingReturn, use_focus_ring},
    },
    utils::{
        CapturedElement, ElementCaptureAttr, EventAccessors, EventHandler,
        aria::{AriaAutocomplete, AriaHasPopup, AriaInvalid, AriaLive, AriaRequired, AriaRole},
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/textfield/src/useTextField.ts

// No intentional deviations from the react-aria implementation.

/// Extracts the string value from an input or textarea element.
fn get_input_value(target: &web_sys::EventTarget) -> Option<String> {
    target
        .dyn_ref::<web_sys::HtmlInputElement>()
        .map(web_sys::HtmlInputElement::value)
        .or_else(|| {
            target
                .dyn_ref::<web_sys::HtmlTextAreaElement>()
                .map(web_sys::HtmlTextAreaElement::value)
        })
}

/// Input parameters for the `use_text_field` hook.
#[derive(Clone)]
#[allow(clippy::struct_excessive_bools)]
pub struct UseTextFieldInput {
    /// The current value (controlled).
    pub value: Signal<String>,

    /// The default value to restore on form reset.
    /// If `None`, the initial value of `value` at hook creation time is used.
    pub default_value: Option<String>,

    /// Callback when the value changes.
    pub on_change: Option<Callback<String>>,

    /// Callback when input receives focus.
    pub on_focus: Option<Callback<()>>,

    /// Callback when input loses focus.
    pub on_blur: Option<Callback<()>>,

    /// Callback when a key is pressed down.
    pub on_key_down: Option<Callback<KeyboardEvent>>,

    /// Callback when a key is released.
    pub on_key_up: Option<Callback<KeyboardEvent>>,

    /// User-provided input event callback, called in addition to the internal handler.
    pub on_input: Option<Callback<Event>>,

    /// Callback when the user copies text.
    pub on_copy: Option<Callback<ClipboardEvent>>,

    /// Callback when the user cuts text.
    pub on_cut: Option<Callback<ClipboardEvent>>,

    /// Callback when the user pastes text.
    pub on_paste: Option<Callback<ClipboardEvent>>,

    /// Callback when a text composition system starts a new session.
    pub on_composition_start: Option<Callback<CompositionEvent>>,

    /// Callback when a text composition system completes or cancels a session.
    pub on_composition_end: Option<Callback<CompositionEvent>>,

    /// Callback when a new character is received in a composition session.
    pub on_composition_update: Option<Callback<CompositionEvent>>,

    /// Callback when text is selected.
    pub on_select: Option<Callback<Event>>,

    /// Callback before the input value is modified.
    pub on_before_input: Option<Callback<InputEvent>>,

    /// Whether the field is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the field is read-only.
    pub is_read_only: Signal<bool>,

    /// Whether the field is required.
    pub is_required: bool,

    /// Whether the field is explicitly marked as invalid (controlled validation).
    ///
    /// - `None` — not controlled; validation comes from `validate`, server errors,
    ///   or native constraint validation.
    /// - `Some(signal)` — controlled; the signal value determines valid/invalid
    ///   and overrides all other validation sources.
    pub is_invalid: Option<Signal<bool>>,

    /// Custom client-side validation function.
    ///
    /// Returns `Ok(())` for valid, `Err(messages)` for invalid.
    pub validate: Option<ValidateFn<String>>,

    /// Validation behavior mode.
    pub validation_behavior: ValidationBehavior,

    /// Whether this text field renders a `<textarea>` instead of `<input>`.
    /// When true, `input_type` and `pattern` are not emitted as attributes
    /// (textarea doesn't support them).
    pub is_multiline: bool,

    /// The type of input (text, email, password, etc.).
    /// Only applied when `is_multiline` is false.
    pub input_type: &'static str,

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

    /// Minimum length for the input value.
    pub min_length: Option<u32>,

    /// Maximum length for the input value.
    pub max_length: Option<u32>,

    /// Pattern for validation. Only applied when `is_multiline` is false.
    pub pattern: Option<&'static str>,

    /// Whether to enable autocomplete.
    pub auto_complete: Option<&'static str>,

    /// Whether to auto-focus the input on mount.
    pub auto_focus: bool,

    /// A hint for the virtual keyboard layout (e.g. "numeric", "email", "tel").
    pub input_mode: Option<&'static str>,

    /// Whether to enable spellcheck.
    pub spell_check: Option<&'static str>,

    /// Controls automatic capitalization of text (e.g. "off", "on", "words", "characters").
    pub auto_capitalize: Option<&'static str>,

    /// Customize the enter key label on virtual keyboards (e.g. "done", "go", "next", "search", "send").
    pub enter_key_hint: Option<&'static str>,

    /// Controls autocorrect behavior (non-standard, Safari-only).
    pub auto_correct: Option<&'static str>,

    /// Associates the input with a `<form>` element by id.
    pub form: Option<&'static str>,

    /// Whether to exclude the input from the tab order.
    pub exclude_from_tab_order: bool,

    /// User-provided `aria-describedby` IDs to merge with generated ones.
    pub aria_describedby: Option<String>,

    /// Identifies the currently active element when DOM focus is on a composite widget.
    pub aria_activedescendant: Option<Signal<Option<String>>>,

    /// Indicates whether input could trigger display of predictions.
    pub aria_autocomplete: Option<AriaAutocomplete>,

    /// Indicates the availability and type of interactive popup element.
    pub aria_haspopup: Option<AriaHasPopup>,

    /// Identifies the element(s) whose contents or presence are controlled by this input.
    pub aria_controls: Option<Signal<Option<String>>>,

    /// Identifies the element that provides an error message for this input.
    pub aria_errormessage: Option<&'static str>,
}

impl Default for UseTextFieldInput {
    fn default() -> Self {
        Self {
            value: Signal::derive(String::new),
            default_value: None,
            on_change: None,
            on_focus: None,
            on_blur: None,
            on_key_down: None,
            on_key_up: None,
            on_input: None,
            on_copy: None,
            on_cut: None,
            on_paste: None,
            on_composition_start: None,
            on_composition_end: None,
            on_composition_update: None,
            on_select: None,
            on_before_input: None,
            is_disabled: Signal::derive(|| false),
            is_read_only: Signal::derive(|| false),
            is_required: false,
            is_invalid: None,
            validate: None,
            validation_behavior: ValidationBehavior::default(),
            is_multiline: false,
            input_type: "text",
            placeholder: None,
            aria_label: None,
            name: None,
            label: None,
            description: None,
            min_length: None,
            max_length: None,
            pattern: None,
            auto_complete: None,
            auto_focus: false,
            input_mode: None,
            spell_check: None,
            auto_capitalize: None,
            enter_key_hint: None,
            auto_correct: None,
            form: None,
            exclude_from_tab_order: false,
            aria_describedby: None,
            aria_activedescendant: None,
            aria_autocomplete: None,
            aria_haspopup: None,
            aria_controls: None,
            aria_errormessage: None,
        }
    }
}

/// The return value of the `use_text_field` hook.
pub struct UseTextFieldReturn {
    /// Props for the input element. Call `.into_attrs()` for view spreading.
    pub input_props: UseTextFieldInputProps,

    /// Props for the label element.
    pub label_props: UseTextFieldLabelProps,

    /// Props for the description element.
    pub description_props: UseTextFieldDescriptionProps,

    /// Props for the error message element.
    pub error_props: UseTextFieldErrorProps,

    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,

    /// Whether the displayed validation is invalid.
    pub is_invalid: Signal<bool>,

    /// The displayed validation error messages.
    pub validation_errors: Signal<Vec<String>>,

    /// Detailed validity state (mirrors native `ValidityState`).
    pub validation_details: Signal<ValidityStateSnapshot>,
}

/// Props from `use_text_field` for the input element that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseTextFieldInputProps {
    // -- Standard HTML attributes --
    pub id: String,
    pub r#type: Option<&'static str>,
    pub name: Option<&'static str>,
    pub value: Signal<String>,
    pub placeholder: Option<&'static str>,
    pub disabled: Signal<bool>,
    pub readonly: Signal<bool>,
    pub required: bool,
    pub form: Option<&'static str>,
    pub inputmode: Option<&'static str>,
    pub spellcheck: Option<&'static str>,
    pub autocapitalize: Option<&'static str>,
    pub autocorrect: Option<&'static str>,
    pub enterkeyhint: Option<&'static str>,
    pub minlength: Option<u32>,
    pub maxlength: Option<u32>,
    pub pattern: Option<&'static str>,
    pub autocomplete: Option<&'static str>,
    pub autofocus: bool,
    pub tabindex: Signal<Option<&'static str>>,
    // -- ARIA attributes --
    pub aria_label: Option<&'static str>,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Signal<Option<String>>,
    pub aria_invalid: Signal<Option<AriaInvalid>>,
    pub aria_required: Option<AriaRequired>,
    pub aria_activedescendant: Signal<Option<String>>,
    pub aria_autocomplete: Option<AriaAutocomplete>,
    pub aria_haspopup: Option<AriaHasPopup>,
    pub aria_controls: Signal<Option<String>>,
    pub aria_errormessage: Option<&'static str>,
    // -- Custom data attributes --
    pub data_focus_visible: Signal<Option<&'static str>>,
    // -- Element capture --
    pub element_capture: ElementCaptureAttr,
    // -- Event handlers --
    pub on_input: EventHandler<Event>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_keyup: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
    pub on_copy: EventHandler<ClipboardEvent>,
    pub on_cut: EventHandler<ClipboardEvent>,
    pub on_paste: EventHandler<ClipboardEvent>,
    pub on_compositionstart: EventHandler<CompositionEvent>,
    pub on_compositionend: EventHandler<CompositionEvent>,
    pub on_compositionupdate: EventHandler<CompositionEvent>,
    pub on_select: EventHandler<Event>,
    pub on_beforeinput: EventHandler<InputEvent>,
}

impl IntoAttrs for UseTextFieldInputProps {
    type Attrs = UseTextFieldInputAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            // Standard HTML attributes
            Attr(attr::Id, self.id),
            Attr(attr::Type, self.r#type),
            Attr(attr::Name, self.name),
            Attr(attr::Value, self.value),
            Attr(attr::Placeholder, self.placeholder),
            Attr(attr::Disabled, self.disabled),
            Attr(attr::Readonly, self.readonly),
            Attr(attr::Required, self.required),
            Attr(attr::Form, self.form),
            Attr(attr::Inputmode, self.inputmode),
            Attr(attr::Spellcheck, self.spellcheck),
            Attr(attr::Autocapitalize, self.autocapitalize),
            custom_attribute("autocorrect", self.autocorrect),
            Attr(attr::Enterkeyhint, self.enterkeyhint),
            Attr(attr::Minlength, self.minlength),
            Attr(attr::Maxlength, self.maxlength),
            Attr(attr::Pattern, self.pattern),
            Attr(attr::Autocomplete, self.autocomplete),
            Attr(attr::Autofocus, self.autofocus),
            Attr(attr::Tabindex, self.tabindex),
            // ARIA attributes
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            Attr(attr::AriaInvalid, self.aria_invalid),
            Attr(attr::AriaRequired, self.aria_required),
            Attr(attr::AriaActivedescendant, self.aria_activedescendant),
            Attr(attr::AriaAutocomplete, self.aria_autocomplete),
            Attr(attr::AriaHaspopup, self.aria_haspopup),
            Attr(attr::AriaControls, self.aria_controls),
            Attr(attr::AriaErrormessage, self.aria_errormessage),
            // Custom data attributes
            custom_attribute("data-focus-visible", self.data_focus_visible),
            // Element capture
            self.element_capture,
            // Event handlers
            self.on_input.into_on(ev::input),
            self.on_keydown.into_on(ev::keydown),
            self.on_keyup.into_on(ev::keyup),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
            self.on_copy.into_on(ev::copy),
            self.on_cut.into_on(ev::cut),
            self.on_paste.into_on(ev::paste),
            self.on_compositionstart.into_on(ev::compositionstart),
            self.on_compositionend.into_on(ev::compositionend),
            self.on_compositionupdate.into_on(ev::compositionupdate),
            self.on_select.into_on(ev::select),
            self.on_beforeinput.into_on(ev::beforeinput),
        )
    }
}

/// Attributes for the text field input element.
#[allow(clippy::type_complexity)]
pub type UseTextFieldInputAttrs = (
    // Standard HTML attributes
    Attr<attr::Id, String>,
    Attr<attr::Type, Option<&'static str>>,
    Attr<attr::Name, Option<&'static str>>,
    Attr<attr::Value, Signal<String>>,
    Attr<attr::Placeholder, Option<&'static str>>,
    Attr<attr::Disabled, Signal<bool>>,
    Attr<attr::Readonly, Signal<bool>>,
    Attr<attr::Required, bool>,
    Attr<attr::Form, Option<&'static str>>,
    Attr<attr::Inputmode, Option<&'static str>>,
    Attr<attr::Spellcheck, Option<&'static str>>,
    Attr<attr::Autocapitalize, Option<&'static str>>,
    CustomAttr<&'static str, Option<&'static str>>,
    Attr<attr::Enterkeyhint, Option<&'static str>>,
    Attr<attr::Minlength, Option<u32>>,
    Attr<attr::Maxlength, Option<u32>>,
    Attr<attr::Pattern, Option<&'static str>>,
    Attr<attr::Autocomplete, Option<&'static str>>,
    Attr<attr::Autofocus, bool>,
    Attr<attr::Tabindex, Signal<Option<&'static str>>>,
    // ARIA attributes
    Attr<attr::AriaLabel, Option<&'static str>>,
    Attr<attr::AriaLabelledby, Option<String>>,
    Attr<attr::AriaDescribedby, Signal<Option<String>>>,
    Attr<attr::AriaInvalid, Signal<Option<AriaInvalid>>>,
    Attr<attr::AriaRequired, Option<AriaRequired>>,
    Attr<attr::AriaActivedescendant, Signal<Option<String>>>,
    Attr<attr::AriaAutocomplete, Option<AriaAutocomplete>>,
    Attr<attr::AriaHaspopup, Option<AriaHasPopup>>,
    Attr<attr::AriaControls, Signal<Option<String>>>,
    Attr<attr::AriaErrormessage, Option<&'static str>>,
    // Custom data attributes
    CustomAttr<&'static str, Signal<Option<&'static str>>>,
    // Element capture
    ElementCaptureAttr,
    // Event handlers
    On<ev::input, SharedEventCallback<Event>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::keyup, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
    On<ev::copy, SharedEventCallback<ClipboardEvent>>,
    On<ev::cut, SharedEventCallback<ClipboardEvent>>,
    On<ev::paste, SharedEventCallback<ClipboardEvent>>,
    On<ev::compositionstart, SharedEventCallback<CompositionEvent>>,
    On<ev::compositionend, SharedEventCallback<CompositionEvent>>,
    On<ev::compositionupdate, SharedEventCallback<CompositionEvent>>,
    On<ev::select, SharedEventCallback<Event>>,
    On<ev::beforeinput, SharedEventCallback<InputEvent>>,
);

/// Props for the label element.
#[derive(Debug)]
pub struct UseTextFieldLabelProps {
    /// The id of the label element.
    pub id: String,

    /// The "for" attribute linking to the input.
    pub html_for: String,
}

impl IntoAttrs for UseTextFieldLabelProps {
    type Attrs = UseTextFieldLabelAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Id, self.id), Attr(attr::For, self.html_for))
    }
}

/// Attributes for the text field label element.
pub type UseTextFieldLabelAttrs = (Attr<attr::Id, String>, Attr<attr::For, String>);

/// Props for the description element.
#[derive(Debug)]
pub struct UseTextFieldDescriptionProps {
    /// The id of the description element.
    pub id: String,
}

impl IntoAttrs for UseTextFieldDescriptionProps {
    type Attrs = UseTextFieldDescriptionAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Id, self.id),)
    }
}

/// Attributes for the text field description element.
pub type UseTextFieldDescriptionAttrs = (Attr<attr::Id, String>,);

/// Props for the error message element.
#[derive(Debug)]
pub struct UseTextFieldErrorProps {
    /// The id of the error message element.
    pub id: String,

    /// The role attribute.
    pub role: AriaRole,

    /// The aria-live attribute.
    pub aria_live: AriaLive,
}

impl IntoAttrs for UseTextFieldErrorProps {
    type Attrs = UseTextFieldErrorAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaLive, self.aria_live),
        )
    }
}

/// Attributes for the text field error message element.
pub type UseTextFieldErrorAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaLive, AriaLive>,
);

/// Provides the behavior and accessibility implementation for a text field.
///
/// Text fields allow users to input text with various configurations.
/// Supports both `<input>` and `<textarea>` elements via the `is_multiline` flag.
///
/// # Example
///
/// ```ignore
/// let (value, set_value) = signal(String::new());
///
/// let text_field = use_text_field(UseTextFieldInput {
///     value: value.into(),
///     on_change: Some(Callback::new(move |v| {
///         set_value.set(v);
///     })),
///     label: Some("Email".to_string()),
///     placeholder: Some("Enter your email"),
///     input_type: "email",
///     is_required: true,
///     validate: Some(Arc::new(|v: &String| {
///         if v.contains('@') { Ok(()) } else { Err(vec!["Must be a valid email".into()]) }
///     })),
///     ..Default::default()
/// });
///
/// view! {
///     <div>
///         <label {..text_field.label_props.into_attrs()}>"Email"</label>
///         <input {..text_field.input_props.into_attrs()} />
///         <p {..text_field.description_props.into_attrs()}>"We'll never share your email"</p>
///         <Show when=move || text_field.is_invalid.get()>
///             <p {..text_field.error_props.into_attrs()}>
///                 {move || text_field.validation_errors.get().join(", ")}
///             </p>
///         </Show>
///     </div>
/// }
/// ```
#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn use_text_field(input: UseTextFieldInput) -> UseTextFieldReturn {
    let UseTextFieldInput {
        value,
        default_value,
        on_change,
        on_focus,
        on_blur,
        on_key_down,
        on_key_up,
        on_input: user_on_input,
        on_copy,
        on_cut,
        on_paste,
        on_composition_start,
        on_composition_end,
        on_composition_update,
        on_select,
        on_before_input,
        is_disabled,
        is_read_only,
        is_required,
        is_invalid,
        validate,
        validation_behavior,
        is_multiline,
        input_type,
        placeholder,
        aria_label,
        name,
        label,
        description,
        min_length,
        max_length,
        pattern,
        auto_complete,
        auto_focus,
        input_mode,
        spell_check,
        auto_capitalize,
        enter_key_hint,
        auto_correct,
        form,
        exclude_from_tab_order,
        aria_describedby: user_aria_describedby,
        aria_activedescendant: user_aria_activedescendant,
        aria_autocomplete,
        aria_haspopup,
        aria_controls: user_aria_controls,
        aria_errormessage,
    } = input;

    // ---- Element capture for DOM access ----
    let element = CapturedElement::new();

    // ---- Form validation state ----
    let validation = use_form_validation_state(UseFormValidationStateInput {
        is_invalid,
        value,
        validate,
        validation_behavior,
        name: name.map(ToString::to_string),
    });

    // ---- Form reset (restores value on form reset) ----
    let initial_value = default_value.unwrap_or_else(|| value.get_untracked());
    use_form_reset(UseFormResetInput {
        element,
        initial_value,
        on_reset: Callback::new(move |val: String| {
            if let Some(on_change) = on_change {
                on_change.run(val);
            }
        }),
    });

    // ---- Form validation DOM connection ----
    use_form_validation(UseFormValidationInput {
        element,
        state: validation,
        validation_behavior,
    });

    // ---- IDs ----
    let base_id = Uuid::new_v4();
    let input_id = format!("textfield-{base_id}");
    let label_id = format!("textfield-label-{base_id}");
    let description_id = format!("textfield-description-{base_id}");
    let error_id = format!("textfield-error-{base_id}");

    // ---- Input event handler ----
    let handle_input = move |e: Event| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }

        if let Some(new_value) = get_input_value(&e.expect_target()) {
            if let Some(on_change) = on_change {
                on_change.run(new_value);
            }
        }
    };

    // Chain internal input handler with user-provided on_input callback.
    let internal_input_handler = EventHandler::new(handle_input);
    let on_input_handler = match user_on_input {
        Some(cb) => internal_input_handler.chain(EventHandler::new(move |e: Event| cb.run(e))),
        None => internal_input_handler,
    };

    // ---- Focus ring ----
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

    // ---- Keyboard event handlers ----
    let on_keydown = on_key_down
        .map(|cb| EventHandler::new(move |e: KeyboardEvent| cb.run(e)))
        .unwrap_or_default();
    let on_keyup = on_key_up
        .map(|cb| EventHandler::new(move |e: KeyboardEvent| cb.run(e)))
        .unwrap_or_default();

    // ---- Clipboard event handlers ----
    let on_copy_handler = on_copy
        .map(|cb| EventHandler::new(move |e: ClipboardEvent| cb.run(e)))
        .unwrap_or_default();
    let on_cut_handler = on_cut
        .map(|cb| EventHandler::new(move |e: ClipboardEvent| cb.run(e)))
        .unwrap_or_default();
    let on_paste_handler = on_paste
        .map(|cb| EventHandler::new(move |e: ClipboardEvent| cb.run(e)))
        .unwrap_or_default();

    // ---- Composition event handlers ----
    let on_compositionstart = on_composition_start
        .map(|cb| EventHandler::new(move |e: CompositionEvent| cb.run(e)))
        .unwrap_or_default();
    let on_compositionend = on_composition_end
        .map(|cb| EventHandler::new(move |e: CompositionEvent| cb.run(e)))
        .unwrap_or_default();
    let on_compositionupdate = on_composition_update
        .map(|cb| EventHandler::new(move |e: CompositionEvent| cb.run(e)))
        .unwrap_or_default();

    // ---- Selection and beforeinput event handlers ----
    let on_select_handler = on_select
        .map(|cb| EventHandler::new(move |e: Event| cb.run(e)))
        .unwrap_or_default();
    let on_beforeinput = on_before_input
        .map(|cb| EventHandler::new(move |e: InputEvent| cb.run(e)))
        .unwrap_or_default();

    // ---- Reactive ARIA attributes ----
    let has_description = description.is_some();
    let has_label = label.is_some();

    let description_id_for_signal = description_id.clone();
    let error_id_for_signal = error_id.clone();
    let aria_describedby = Signal::derive(move || {
        let mut parts = Vec::new();
        if has_description {
            parts.push(description_id_for_signal.clone());
        }
        if validation.is_invalid.get() {
            parts.push(error_id_for_signal.clone());
        }
        if let Some(ref user_desc) = user_aria_describedby {
            parts.push(user_desc.clone());
        }
        if parts.is_empty() {
            None
        } else {
            Some(parts.join(" "))
        }
    });

    let aria_labelledby = if has_label {
        Some(label_id.clone())
    } else {
        None
    };

    let aria_invalid =
        Signal::derive(move || validation.is_invalid.get().then_some(AriaInvalid::True));

    // React-aria sets `required` for native validation, `aria-required` for ARIA mode.
    let required = is_required && validation_behavior == ValidationBehavior::Native;
    let aria_required = (is_required && validation_behavior == ValidationBehavior::Aria)
        .then_some(AriaRequired::True);

    // ---- Textarea-conditional attributes ----
    // `type` and `pattern` are only valid on <input>, not <textarea>.
    let effective_type = if is_multiline { None } else { Some(input_type) };
    let effective_pattern = if is_multiline { None } else { pattern };

    // ---- Tab index ----
    let tabindex = Signal::derive(move || {
        if is_disabled.get() {
            None
        } else if exclude_from_tab_order {
            Some("-1")
        } else {
            None
        }
    });

    // ---- ARIA passthroughs ----
    let aria_activedescendant =
        user_aria_activedescendant.unwrap_or_else(|| Signal::derive(|| None));
    let aria_controls = user_aria_controls.unwrap_or_else(|| Signal::derive(|| None));

    // ---- Validation details convenience signal ----
    let validation_details =
        Signal::derive(move || validation.display_validation.get().validation_details);

    UseTextFieldReturn {
        input_props: UseTextFieldInputProps {
            id: input_id.clone(),
            r#type: effective_type,
            name,
            value,
            placeholder,
            disabled: is_disabled,
            readonly: is_read_only,
            required,
            form,
            inputmode: input_mode,
            spellcheck: spell_check,
            autocapitalize: auto_capitalize,
            autocorrect: auto_correct,
            enterkeyhint: enter_key_hint,
            minlength: min_length,
            maxlength: max_length,
            pattern: effective_pattern,
            autocomplete: auto_complete,
            autofocus: auto_focus,
            tabindex,
            aria_label,
            aria_labelledby,
            aria_describedby,
            aria_invalid,
            aria_required,
            aria_activedescendant,
            aria_autocomplete,
            aria_haspopup,
            aria_controls,
            aria_errormessage,
            data_focus_visible: focus_ring_props.data_focus_visible,
            element_capture: element.attr(),
            on_input: on_input_handler,
            on_keydown,
            on_keyup,
            on_focus: focus_ring_props.on_focus,
            on_blur: focus_ring_props.on_blur,
            on_focusin: focus_ring_props.on_focusin,
            on_focusout: focus_ring_props.on_focusout,
            on_copy: on_copy_handler,
            on_cut: on_cut_handler,
            on_paste: on_paste_handler,
            on_compositionstart,
            on_compositionend,
            on_compositionupdate,
            on_select: on_select_handler,
            on_beforeinput,
        },
        label_props: UseTextFieldLabelProps {
            id: label_id,
            html_for: input_id,
        },
        description_props: UseTextFieldDescriptionProps { id: description_id },
        error_props: UseTextFieldErrorProps {
            id: error_id,
            role: AriaRole::Alert,
            aria_live: AriaLive::Polite,
        },
        is_focus_visible,
        is_invalid: validation.is_invalid,
        validation_errors: validation.validation_errors,
        validation_details,
    }
}

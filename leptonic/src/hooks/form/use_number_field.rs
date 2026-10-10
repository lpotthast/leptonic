// Upstream: react-aria/src/numberfield/useNumberField.ts @ 99e6102368
// Upstream: react-aria/test/numberfield/useNumberField.test.ts @ 99e6102368
// Upstream: react-aria-components/test/NumberField.test.js @ 99e6102368
use leptos::{
    attr::{self, Attr},
    ev::{self},
    prelude::*,
};
use leptos_use::use_document;
use wasm_bindgen::JsCast;
use web_sys::{ClipboardEvent, CompositionEvent, FocusEvent, InputEvent, WheelEvent};

use super::{
    use_form_reset::{UseFormResetInput, use_form_reset},
    use_form_validation_state::{ValidationBehavior, ValidationResult, ValidityStateSnapshot},
    use_formatted_text_field::{
        FormattedTextFieldHandlers, UseFormattedTextFieldInput, use_formatted_text_field,
    },
    use_label::UseLabelProps,
    use_number_field_state::{CommitBehavior, NumberFieldState},
    use_text_field::{
        InputMode, UseTextFieldInput, UseTextFieldInputAttrs, UseTextFieldInputProps,
        UseTextFieldReturn, use_text_field,
    },
    use_text_field_state::{UseTextFieldStateInput, use_text_field_state},
};
use crate::{
    CapturedElement, EventHandler, IntoAttrs, NumberValue, OnEvent, SlotProps,
    hooks::{
        button::UseButtonInput,
        focus::use_focus_within::{UseFocusWithinInput, UseFocusWithinReturn, use_focus_within},
        form::{InputType, TextFieldElement},
        interactions::{
            use_keyboard::KeyboardEventWrapper,
            use_press::{PressEvent, chain_optional_callbacks},
            use_scroll_wheel::{ScrollEvent, UseScrollWheelInput, use_scroll_wheel},
        },
        spinbutton::use_spin_button::{UseSpinButtonInput, UseSpinButtonReturn, use_spin_button},
    },
    utils::{
        aria::{AriaDisabled, AriaInvalid, AriaRole},
        dom_ext::EventAccessors,
        focus::focus_event_target,
        id::use_id,
        intl_strings::{NumberFieldStrings, use_localized_strings},
        key::KeyboardKey,
        keyboard_shortcut::{KeyboardShortcuts, Shortcut, ShortcutOutcome},
        live_announcer::announce_assertive,
        number_formatter::{CurrencySign, NumberFormatOptions, use_number_formatter},
        platform::{device, use_platform_check},
        pointer_type::PointerType,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Generic over the value type (C15), the state from `use_number_field_state`; settings that
//   live on the state (range, step, format, disabled, read-only, validation) are read from it
//   (C8), not repeated on the input.
// - Returns the stepper buttons' `UseButtonInput`s (compose with `use_button`) instead of button
//   props (project convention).
// - `name`/`form` belong to a hidden input holding the value, which the caller renders (as
//   react-aria-components does); the hook only sets up the visible, formatted input.
// - `useFormattedTextField` is `use_formatted_text_field`, whose handlers this hook merges.
//
// ## DIFFERENT BEHAVIOR
// - The stepper buttons always have an id (react-aria: only while labelled by other elements):
//   their labelling follows the field's label, which may appear later, and ids are static.
//
// ## OMITTED FEATURES
// - The `beforeinput` fallback for browsers without it (all supported browsers have it).
//
// =============================================================================

/// Input of [`use_number_field`].
#[derive(Clone)]
pub struct UseNumberFieldInput<T: NumberValue> {
    pub state: NumberFieldState<T>,
    /// The input's id. Generated when `None`.
    pub id: Option<String>,
    /// Whether a visible label is rendered (with `label_props`).
    pub has_label: Signal<bool>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Option<String>,
    pub is_required: Signal<bool>,
    pub placeholder: MaybeProp<String>,
    pub auto_focus: bool,
    /// Whether the scroll wheel leaves the value alone (it steps while the field has focus).
    pub is_wheel_disabled: bool,
    /// Replaces "Increase `<field label>`".
    pub increment_aria_label: MaybeProp<String>,
    /// Replaces "Decrease `<field label>`".
    pub decrement_aria_label: MaybeProp<String>,
    pub on_focus: Option<Callback<FocusEvent>>,
    pub on_blur: Option<Callback<FocusEvent>>,
    pub on_focus_change: Option<Callback<bool>>,
    pub on_key_down: Option<Callback<KeyboardEventWrapper>>,
    pub on_key_up: Option<Callback<KeyboardEventWrapper>>,
}

/// Return value of [`use_number_field`].
pub struct UseNumberFieldReturn {
    /// For the element around the input and the stepper buttons.
    pub group_props: UseNumberFieldGroupProps,
    pub label_props: UseLabelProps,
    pub input_props: UseNumberFieldInputProps,
    /// The increment button's configuration, for `use_button`.
    pub increment_button: UseButtonInput,
    /// The decrement button's configuration, for `use_button`.
    pub decrement_button: UseButtonInput,
    pub description_props: SlotProps,
    /// For the error message element. Render it only while the field is invalid.
    pub error_message_props: SlotProps,
    /// The input, once rendered.
    pub element: CapturedElement,
    pub is_focused: Signal<bool>,
    pub is_focus_visible: Signal<bool>,
    pub is_invalid: Signal<bool>,
    pub validation_errors: Signal<Vec<String>>,
    pub validation_details: Signal<ValidityStateSnapshot>,
}

/// Props for the group around the input and its buttons.
#[derive(Debug, Clone)]
pub struct UseNumberFieldGroupProps {
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub aria_invalid: Signal<Option<AriaInvalid>>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
}

pub type UseNumberFieldGroupAttrs = (
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    Attr<attr::AriaInvalid, Signal<Option<AriaInvalid>>>,
    OnEvent<ev::focusin>,
    OnEvent<ev::focusout>,
);

impl IntoAttrs for UseNumberFieldGroupProps {
    type Attrs = UseNumberFieldGroupAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, AriaRole::Group),
            Attr(attr::AriaDisabled, self.aria_disabled),
            Attr(attr::AriaInvalid, self.aria_invalid),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
        )
    }
}

/// Props for the number field's `<input>`: the text field's, plus input filtering, paste and
/// scroll wheel handling.
#[derive(Debug, Clone)]
pub struct UseNumberFieldInputProps {
    pub text_field: UseTextFieldInputProps,
    pub aria_roledescription: Signal<Option<String>>,
    pub on_beforeinput: EventHandler<InputEvent>,
    pub on_compositionstart: EventHandler<CompositionEvent>,
    pub on_compositionend: EventHandler<CompositionEvent>,
    pub on_paste: EventHandler<ClipboardEvent>,
    pub on_wheel: EventHandler<WheelEvent>,
}

pub type UseNumberFieldInputAttrs = (
    UseTextFieldInputAttrs,
    (
        Attr<attr::AriaRoledescription, Signal<Option<String>>>,
        OnEvent<ev::beforeinput>,
        OnEvent<ev::compositionstart>,
        OnEvent<ev::compositionend>,
        OnEvent<ev::paste>,
        OnEvent<ev::wheel>,
    ),
);

impl IntoAttrs for UseNumberFieldInputProps {
    type Attrs = UseNumberFieldInputAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.text_field.into_attrs(),
            (
                Attr(attr::AriaRoledescription, self.aria_roledescription),
                self.on_beforeinput.into_on(ev::beforeinput),
                self.on_compositionstart.into_on(ev::compositionstart),
                self.on_compositionend.into_on(ev::compositionend),
                self.on_paste.into_on(ev::paste),
                self.on_wheel.into_on(ev::wheel),
            ),
        )
    }
}

/// A number field: a text input for a formatted number, with stepper buttons, the arrow, Page
/// and Home/End keys, the scroll wheel, a label, description and error message, validation and
/// form reset.
///
/// ```ignore
/// let state = use_number_field_state(UseNumberFieldStateInput::<u8>::default());
/// let field = use_number_field(UseNumberFieldInput {
///     state,
///     id: None,
///     has_label: Signal::stored(true),
///     aria_label: MaybeProp::default(),
///     aria_labelledby: None,
///     aria_describedby: None,
///     is_required: Signal::stored(false),
///     placeholder: MaybeProp::default(),
///     auto_focus: false,
///     is_wheel_disabled: false,
///     increment_aria_label: MaybeProp::default(),
///     decrement_aria_label: MaybeProp::default(),
///     on_focus: None,
///     on_blur: None,
///     on_focus_change: None,
///     on_key_down: None,
///     on_key_up: None,
/// });
/// let increment = use_button(field.increment_button);
/// view! {
///     <label {..field.label_props.into_attrs()}>"Quantity"</label>
///     <div {..field.group_props.into_attrs()}>
///         <input {..field.input_props.into_attrs()} />
///         <button {..increment.props.into_attrs()}>"+"</button>
///     </div>
/// }
/// ```
///
/// Render a hidden input (`type="hidden"`, `name`, `value` = the state's `number_value`) to
/// submit the value with a form.
#[allow(clippy::too_many_lines)]
pub fn use_number_field<T: NumberValue>(input: UseNumberFieldInput<T>) -> UseNumberFieldReturn {
    let UseNumberFieldInput {
        state,
        id,
        has_label,
        aria_label,
        aria_labelledby,
        aria_describedby,
        is_required,
        placeholder,
        auto_focus,
        is_wheel_disabled,
        increment_aria_label,
        decrement_aria_label,
        on_focus,
        on_blur,
        on_focus_change,
        on_key_down,
        on_key_up,
    } = input;
    let is_disabled = state.is_disabled;
    let is_read_only = state.is_read_only;
    let inactive = move || is_disabled.get_untracked() || is_read_only.get_untracked();
    let input_id = id.unwrap_or_else(|| use_id("number-field"));

    // On blur: commit, and announce the value if committing changed the text.
    let commit_and_announce = Callback::new(move |e: FocusEvent| {
        let old = e
            .expect_target()
            .dyn_into::<web_sys::HtmlInputElement>()
            .map(|input| input.value())
            .unwrap_or_default();
        state.commit(None);
        let new = state.input_value.get_untracked();
        if new != old {
            announce_assertive(new);
        }
        if let Some(on_blur) = on_blur {
            on_blur.try_run(e);
        }
    });

    // Enter commits; its default action (submitting the form) is kept.
    let shortcuts = KeyboardShortcuts::new().on(Shortcut::new(KeyboardKey::Enter), move |_| {
        if inactive() {
            return ShortcutOutcome::Ignored;
        }
        state.commit(None);
        state.validation.commit_validation();
        ShortcutOutcome::Custom {
            prevent_default: false,
            continue_propagation: false,
        }
    });

    // The virtual keyboard: numeric or decimal, by whether negative and fractional values are
    // possible (tested by react-aria on many devices).
    let is_iphone = use_platform_check(device::is_iphone);
    let is_android = use_platform_check(device::is_android);
    let input_mode = Signal::derive(move || {
        let has_negative = T::lower_bound(state.min_value.get()).is_none_or(|min| min < T::ZERO);
        let has_decimals = !T::IS_INTEGER
            && state
                .format_options
                .with(|options| options.fraction_digits().1 > 0);
        Some(if is_iphone.get() {
            if has_negative {
                InputMode::Text
            } else if has_decimals {
                InputMode::Decimal
            } else {
                InputMode::Numeric
            }
        } else if is_android.get() && !has_negative && has_decimals {
            InputMode::Decimal
        } else {
            InputMode::Numeric
        })
    });
    let is_ios = use_platform_check(device::is_ios);

    // Typing changes the text only while it is (the beginning of) a valid number.
    let text_state = use_text_field_state(UseTextFieldStateInput {
        default_value: String::new(),
        value: Some(crate::ValueBinding::new(
            state.input_value,
            Callback::new(move |text: String| {
                if state.validate(&text) {
                    state.set_input_value(text);
                }
            }),
        )),
        on_change: None,
    });
    let UseTextFieldReturn {
        label_props,
        input_props: mut text_field_props,
        description_props,
        error_message_props,
        element,
        is_focused,
        is_focus_visible,
        is_invalid,
        validation_errors,
        validation_details,
    } = use_text_field(UseTextFieldInput {
        id: Some(input_id.clone()),
        is_disabled,
        is_read_only,
        is_required,
        validation: Some(state.validation),
        validation_behavior: state.validation_behavior,
        placeholder,
        auto_complete: Some("off".to_owned()),
        auto_correct: Some(false),
        spell_check: Some(false),
        input_mode,
        auto_focus,
        has_label,
        aria_label,
        aria_labelledby: aria_labelledby.clone(),
        aria_describedby,
        on_focus,
        on_blur: Some(commit_and_announce),
        on_focus_change,
        on_key_down,
        on_key_up,
        shortcuts: Some(shortcuts),
        state: text_state,
        element: TextFieldElement::Input,
        input_type: Signal::stored(InputType::Text),
        is_invalid: Signal::stored(false),
        validate: None,
        name: None,
        form: None,
        pattern: None,
        min_length: None,
        max_length: None,
        auto_capitalize: None,
        enter_key_hint: None,
        exclude_from_tab_order: false,
        label_id: None,
        aria_errormessage: None,
        aria_activedescendant: Signal::stored(None),
        aria_autocomplete: None,
        aria_haspopup: None,
        aria_controls: Signal::stored(None),
    });

    use_form_reset(UseFormResetInput {
        element,
        initial_value: state.default_number_value,
        on_reset: Callback::new(move |value| state.set_number_value(value)),
    });
    if state.commit_behavior == CommitBehavior::Validate {
        use_native_range_validation(state, element);
    }

    // Announced as the value formatted with a minus sign (an accounting format's parentheses
    // aren't read as negative), not as the text being typed.
    let text_value_formatter = use_number_formatter(Signal::derive(move || NumberFormatOptions {
        currency_sign: CurrencySign::Standard,
        ..state.format_options.get()
    }));
    // The spin button's keyboard handling (arrows, Page Up/Down, Home/End), not its role.
    let UseSpinButtonReturn {
        props: spin,
        increment_button: spin_increment_button,
        decrement_button: spin_decrement_button,
    } = use_spin_button(UseSpinButtonInput {
        value: state.number_value,
        text_value: Signal::derive(move || {
            Some(state.number_value.get().map_or_else(String::new, |value| {
                text_value_formatter.with(|formatter| formatter.format(value))
            }))
        }),
        min_value: Signal::derive(move || T::lower_bound(state.min_value.get())),
        max_value: Signal::derive(move || T::upper_bound(state.max_value.get())),
        is_disabled,
        is_read_only,
        is_required,
        on_increment: Some(Callback::new(move |()| state.increment())),
        on_decrement: Some(Callback::new(move |()| state.decrement())),
        on_increment_to_max: Some(Callback::new(move |()| state.increment_to_max())),
        on_decrement_to_min: Some(Callback::new(move |()| state.decrement_to_min())),
        ..UseSpinButtonInput::default()
    });
    let focusable = &mut text_field_props.focusable;
    focusable.on_keydown = spin.on_keydown.chain(focusable.on_keydown.clone());
    focusable.on_keyup = spin.on_keyup.chain(focusable.on_keyup.clone());
    focusable.on_focus = spin.on_focus.chain(focusable.on_focus.clone());
    focusable.on_blur = spin.on_blur.chain(focusable.on_blur.clone());

    // The scroll wheel steps while the field has focus.
    let UseFocusWithinReturn {
        props: focus_within,
        is_focus_within,
    } = use_focus_within(UseFocusWithinInput {
        is_disabled,
        ..UseFocusWithinInput::default()
    });
    let wheel = use_scroll_wheel(UseScrollWheelInput {
        is_disabled: Signal::derive(move || {
            is_wheel_disabled || is_disabled.get() || is_read_only.get() || !is_focus_within.get()
        }),
        on_scroll: Some(Callback::new(move |e: ScrollEvent| {
            // Mostly horizontal (a trackpad): probably not meant to step.
            if e.delta_y.abs() <= e.delta_x.abs() {
                return;
            }
            if e.delta_y > 0.0 {
                state.increment();
            } else if e.delta_y < 0.0 {
                state.decrement();
            }
        })),
    });

    let FormattedTextFieldHandlers {
        on_beforeinput,
        on_compositionstart,
        on_compositionend,
    } = use_formatted_text_field(UseFormattedTextFieldInput { element, state });

    // Pasting over the whole text commits the pasted text right away, so it shows formatted.
    let on_paste = EventHandler::new(move |e: ClipboardEvent| {
        let Ok(input) = e.expect_target().dyn_into::<web_sys::HtmlInputElement>() else {
            return;
        };
        let start = input.selection_start().ok().flatten().unwrap_or(0);
        let end = input.selection_end().ok().flatten().unwrap_or(0);
        let length = u32::try_from(input.value().encode_utf16().count()).unwrap_or(u32::MAX);
        if end.saturating_sub(start) == length {
            e.prevent_default();
            let text = e
                .clipboard_data()
                .and_then(|data| data.get_data("text/plain").ok())
                .unwrap_or_default();
            state.commit(Some(text.trim().to_owned()));
        }
    });

    // Stepper buttons: named "Increase <field label>", or "Increase" plus the labelling elements.
    let field_label = aria_label;
    let labelled_by = {
        let label_id = label_props.id.clone();
        Signal::derive(move || {
            field_label
                .get()
                .is_none()
                .then(|| {
                    if has_label.get() {
                        Some(label_id.clone())
                    } else {
                        aria_labelledby.clone()
                    }
                })
                .flatten()
        })
    };
    // Keeps focus in the input while it has it (the virtual keyboard stays); a mouse moves focus
    // to the input, touch and screen readers focus the button.
    let on_button_press_start = Callback::new(move |e: PressEvent| {
        let input = element.get_untracked().map(|el| (*el).clone());
        let active = use_document()
            .as_ref()
            .and_then(web_sys::Document::active_element);
        if input.is_some() && active == input {
            return;
        }
        if e.pointer_type == PointerType::Mouse {
            if let Some(input) = input.and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok()) {
                let _ = input.focus();
            }
        } else {
            focus_event_target(&e.target, false);
        }
    });
    let strings = use_localized_strings::<NumberFieldStrings>();
    let stepper = |spin_button: UseButtonInput,
                   verb: fn(&NumberFieldStrings, &str) -> String,
                   custom_label: MaybeProp<String>,
                   can_step: Signal<bool>| {
        let button_id = use_id("number-field-stepper");
        // The id is always rendered (it may be needed once a label appears).
        let own_id = button_id.clone();
        UseButtonInput {
            id: Some(own_id),
            aria_label: MaybeProp::derive(move || {
                custom_label.get().or_else(|| {
                    let label = field_label.get().unwrap_or_default();
                    Some(verb(&strings.read(), &label).trim().to_owned())
                })
            }),
            aria_labelledby: Signal::derive(move || {
                if custom_label.get().is_some() {
                    return None;
                }
                labelled_by
                    .get()
                    .map(|labelled_by| format!("{button_id} {labelled_by}"))
            }),
            aria_controls: Signal::stored(Some(input_id.clone())),
            exclude_from_tab_order: Signal::stored(true),
            prevent_focus_on_press: true.into(),
            allow_focus_when_disabled: true.into(),
            is_disabled: Signal::derive(move || !can_step.get()),
            on_press_start: chain_optional_callbacks(
                spin_button.on_press_start,
                Some(on_button_press_start),
            ),
            ..spin_button
        }
    };
    let increment_button = stepper(
        spin_increment_button,
        NumberFieldStrings::increase,
        increment_aria_label,
        state.can_increment,
    );
    let decrement_button = stepper(
        spin_decrement_button,
        NumberFieldStrings::decrease,
        decrement_aria_label,
        state.can_decrement,
    );

    UseNumberFieldReturn {
        group_props: UseNumberFieldGroupProps {
            aria_disabled: Signal::derive(move || is_disabled.get().then_some(AriaDisabled::True)),
            aria_invalid: Signal::derive(move || is_invalid.get().then_some(AriaInvalid::True)),
            on_focusin: focus_within.on_focusin,
            on_focusout: focus_within.on_focusout,
        },
        label_props,
        input_props: UseNumberFieldInputProps {
            text_field: text_field_props,
            // Not on iOS, so that VoiceOver announces the required state.
            aria_roledescription: Signal::derive(move || {
                (!is_ios.get()).then(|| strings.read().number_field())
            }),
            on_beforeinput,
            on_compositionstart,
            on_compositionend,
            on_paste,
            on_wheel: wheel.props.on_wheel,
        },
        increment_button,
        decrement_button,
        description_props,
        error_message_props,
        element,
        is_focused,
        is_focus_visible,
        is_invalid,
        validation_errors,
        validation_details,
    }
}

/// With `CommitBehavior::Validate`: validates the range and step as a native number input
/// would, for the browser's messages (react-aria's `useNativeValidation`).
fn use_native_range_validation<T: NumberValue>(
    state: NumberFieldState<T>,
    element: CapturedElement,
) {
    let sync = move || {
        let realtime = state.validation.realtime_validation.get_untracked();
        let value = state.number_value.get_untracked();
        let (min, max, step) = (
            state.min_value.get_untracked(),
            state.max_value.get_untracked(),
            state.step.get_untracked(),
        );
        let Some(input) = element
            .get_untracked()
            .and_then(|el| el.dyn_ref::<web_sys::HtmlInputElement>().cloned())
        else {
            return;
        };
        if realtime.is_invalid || input.disabled() {
            return;
        }
        let Some(probe) = range_probe() else {
            return;
        };
        let native = state.validation_behavior == ValidationBehavior::Native;
        // Our own message from the last sync (react-aria: `useFormValidation` resets the custom
        // validity after every render, before this runs). The realtime validation is valid, so
        // there is no other custom message to keep.
        if native {
            input.set_custom_validity("");
        }
        probe.set_min(&min.map(|v| v.to_string()).unwrap_or_default());
        probe.set_max(&max.map(|v| v.to_string()).unwrap_or_default());
        probe.set_step(&step.to_string());
        probe.set_value(&value.map(|v| v.to_string()).unwrap_or_default());

        let (own, range) = (input.validity(), probe.validity());
        let valid = own.valid() && range.valid();
        let message = Some(input.validation_message().unwrap_or_default())
            .filter(|m| !m.is_empty())
            .or_else(|| probe.validation_message().ok().filter(|m| !m.is_empty()));
        state.validation.update_validation(ValidationResult {
            is_invalid: !valid,
            validation_errors: message.into_iter().collect(),
            validation_details: ValidityStateSnapshot {
                bad_input: own.bad_input(),
                custom_error: own.custom_error(),
                pattern_mismatch: own.pattern_mismatch(),
                range_overflow: range.range_overflow(),
                range_underflow: range.range_underflow(),
                step_mismatch: range.step_mismatch(),
                too_long: own.too_long(),
                too_short: own.too_short(),
                type_mismatch: own.type_mismatch(),
                value_missing: own.value_missing(),
                valid,
            },
        });
        // Block native form submission (doesn't overwrite custom messages: checked above).
        if native && !range.valid() {
            input.set_custom_validity(&probe.validation_message().unwrap_or_default());
        }
    };
    // When the value or range changes, and right before each commit (react-aria: in a layout
    // effect, which runs before the commit's effect). Registered after the text field's reader,
    // so its merged result wins.
    // The inputs before the validation memo derived from them ("Effect Read Order").
    Effect::new(move |_| {
        state.number_value.track();
        state.min_value.track();
        state.max_value.track();
        state.step.track();
        state.validation.realtime_validation.track();
        if element.get().is_some() {
            sync();
        }
    });
    state
        .validation
        .native_validity_readers
        .register(Callback::new(move |()| sync()));
}

thread_local! {
    /// The `<input type="number">` validating ranges and steps, shared by all number fields (as
    /// react-aria's).
    static RANGE_PROBE: std::cell::OnceCell<Option<web_sys::HtmlInputElement>> =
        const { std::cell::OnceCell::new() };
}

/// The shared range probe, created on first use; `None` without a document.
fn range_probe() -> Option<web_sys::HtmlInputElement> {
    RANGE_PROBE.with(|probe| {
        probe
            .get_or_init(|| {
                let probe = use_document()
                    .as_ref()
                    .and_then(|d| d.create_element("input").ok())
                    .and_then(|el| el.dyn_into::<web_sys::HtmlInputElement>().ok())?;
                probe.set_type("number");
                Some(probe)
            })
            .clone()
    })
}

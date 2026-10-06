// Upstream: react-aria/src/numberfield/useNumberField.ts @ 99e6102368
// Upstream: react-aria/src/textfield/useFormattedTextField.ts @ 99e6102368
use leptos::{
    attr::{self, Attr},
    ev::{self, On, SharedEventCallback},
    prelude::*,
};
use leptos_use::use_document;
use wasm_bindgen::JsCast;
use web_sys::{ClipboardEvent, CompositionEvent, FocusEvent, InputEvent, WheelEvent};

use super::{
    use_form_reset::{UseFormResetInput, use_form_reset},
    use_form_validation_state::{ValidationBehavior, ValidationResult, ValidityStateSnapshot},
    use_label::UseLabelProps,
    use_number_field_state::{CommitBehavior, NumberFieldState},
    use_text_field::{
        InputMode, UseTextFieldInput, UseTextFieldInputAttrs, UseTextFieldInputProps,
        UseTextFieldReturn, use_text_field,
    },
    use_text_field_state::TextFieldState,
};
use crate::{
    hooks::{
        IntoAttrs, UseButtonInput,
        focus::use_focus_within::{UseFocusWithinInput, UseFocusWithinReturn, use_focus_within},
        interactions::{
            use_keyboard::KeyboardEventWrapper,
            use_press::{PressEvent, chain_optional_callbacks},
            use_scroll_wheel::{ScrollEvent, UseScrollWheelInput, use_scroll_wheel},
        },
        spinbutton::use_spin_button::{UseSpinButtonInput, UseSpinButtonReturn, use_spin_button},
    },
    utils::{
        CapturedElement, EventHandler, NumberValue, SlotProps,
        aria::{AriaInvalid, AriaRole},
        focus::focus_event_target,
        id::use_id,
        keyboard_shortcut::{KeyboardShortcuts, Shortcut, ShortcutOutcome},
        live_announcer::announce_assertive,
        number_formatter::NumberStyle,
        platform::device,
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
// - `useFormattedTextField` is part of this hook (no other field uses it yet).
//
// ## OMITTED FEATURES
// - Localized strings: "Increase"/"Decrease"/"Number field" are English until leptonic has a
//   localized string formatter.
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
    pub has_label: bool,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Option<String>,
    pub is_required: Signal<bool>,
    pub placeholder: MaybeProp<String>,
    pub auto_focus: bool,
    /// Whether the scroll wheel leaves the value alone (it steps while the field has focus).
    pub is_wheel_disabled: bool,
    /// Replaces "Increase <field label>".
    pub increment_aria_label: MaybeProp<String>,
    /// Replaces "Decrease <field label>".
    pub decrement_aria_label: MaybeProp<String>,
    pub on_focus: Option<Callback<FocusEvent>>,
    pub on_blur: Option<Callback<FocusEvent>>,
    pub on_focus_change: Option<Callback<bool>>,
    pub on_key_down: Option<Callback<KeyboardEventWrapper>>,
    pub on_key_up: Option<Callback<KeyboardEventWrapper>>,
}

impl<T: NumberValue> UseNumberFieldInput<T> {
    pub fn new(state: NumberFieldState<T>) -> Self {
        Self {
            state,
            id: None,
            has_label: false,
            aria_label: MaybeProp::default(),
            aria_labelledby: None,
            aria_describedby: None,
            is_required: Signal::stored(false),
            placeholder: MaybeProp::default(),
            auto_focus: false,
            is_wheel_disabled: false,
            increment_aria_label: MaybeProp::default(),
            decrement_aria_label: MaybeProp::default(),
            on_focus: None,
            on_blur: None,
            on_focus_change: None,
            on_key_down: None,
            on_key_up: None,
        }
    }
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
    pub aria_disabled: Signal<Option<&'static str>>,
    pub aria_invalid: Signal<Option<AriaInvalid>>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
}

pub type UseNumberFieldGroupAttrs = (
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaDisabled, Signal<Option<&'static str>>>,
    Attr<attr::AriaInvalid, Signal<Option<AriaInvalid>>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
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
    pub aria_roledescription: Option<&'static str>,
    pub on_beforeinput: EventHandler<InputEvent>,
    pub on_compositionstart: EventHandler<CompositionEvent>,
    pub on_compositionend: EventHandler<CompositionEvent>,
    pub on_paste: EventHandler<ClipboardEvent>,
    pub on_wheel: EventHandler<WheelEvent>,
}

pub type UseNumberFieldInputAttrs = (
    UseTextFieldInputAttrs,
    (
        Attr<attr::AriaRoledescription, Option<&'static str>>,
        On<ev::beforeinput, SharedEventCallback<InputEvent>>,
        On<ev::compositionstart, SharedEventCallback<CompositionEvent>>,
        On<ev::compositionend, SharedEventCallback<CompositionEvent>>,
        On<ev::paste, SharedEventCallback<ClipboardEvent>>,
        On<ev::wheel, SharedEventCallback<WheelEvent>>,
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
/// let field = use_number_field(UseNumberFieldInput { has_label: true, ..UseNumberFieldInput::new(state) });
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
            .target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
            .map(|input| input.value())
            .unwrap_or_default();
        state.commit(None);
        let new = state.input_value.get_untracked();
        if new != old {
            announce_assertive(new);
        }
        if let Some(on_blur) = on_blur {
            on_blur.run(e);
        }
    });

    // Enter commits; its default action (submitting the form) is kept.
    let shortcuts = KeyboardShortcuts::new().on(Shortcut::key("Enter"), move |_| {
        if inactive() {
            return ShortcutOutcome::Ignored;
        }
        state.commit(None);
        state.validation.commit_validation.run(());
        ShortcutOutcome::Custom {
            prevent_default: false,
            continue_propagation: false,
        }
    });

    // The virtual keyboard: numeric or decimal, by whether negative and fractional values are
    // possible (tested by react-aria on many devices).
    let has_negative =
        T::lower_bound(state.min_value.get_untracked()).is_none_or(|min| min < T::ZERO);
    let has_decimals = !T::IS_INTEGER
        && state.format_options.with_untracked(|options| {
            options
                .maximum_fraction_digits
                .unwrap_or(match options.style {
                    NumberStyle::Percent => 0,
                    NumberStyle::Currency => 2,
                    NumberStyle::Decimal | NumberStyle::Unit => 3,
                })
                > 0
        });
    let input_mode = if device::is_iphone() {
        if has_negative {
            InputMode::Text
        } else if has_decimals {
            InputMode::Decimal
        } else {
            InputMode::Numeric
        }
    } else if device::is_android() && !has_negative && has_decimals {
        InputMode::Decimal
    } else {
        InputMode::Numeric
    };

    // Typing changes the text only while it is (the beginning of) a valid number.
    let text_state = TextFieldState::new(
        state.input_value,
        Callback::new(move |text: String| {
            if state.validate(text.clone()) {
                state.set_input_value(text);
            }
        }),
    );
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
        input_mode: Some(input_mode),
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
        ..UseTextFieldInput::new(text_state)
    });

    use_form_reset(UseFormResetInput {
        element,
        initial_value: state.default_number_value,
        on_reset: Callback::new(move |value| state.set_number_value(value)),
    });
    if state.commit_behavior == CommitBehavior::Validate {
        use_native_range_validation(state, element);
    }

    // The spin button's keyboard handling (arrows, Page Up/Down, Home/End), not its role.
    let UseSpinButtonReturn {
        props: spin,
        increment_button: spin_increment_button,
        decrement_button: spin_decrement_button,
    } = use_spin_button(UseSpinButtonInput {
        value: Signal::derive(move || state.number_value.get().map(NumberValue::to_f64)),
        text_value: Signal::derive(move || Some(state.input_value.get())),
        min_value: Signal::derive(move || {
            T::lower_bound(state.min_value.get()).map(NumberValue::to_f64)
        }),
        max_value: Signal::derive(move || {
            T::upper_bound(state.max_value.get()).map(NumberValue::to_f64)
        }),
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

    // Rejects edits that would make the text invalid, before the browser applies them.
    let on_beforeinput = EventHandler::new(move |e: InputEvent| {
        let Some(input) = element
            .get_untracked()
            .and_then(|el| el.dyn_ref::<web_sys::HtmlInputElement>().cloned())
        else {
            return;
        };
        let next = next_input_value(&input, &e.input_type(), e.data());
        let allowed = match next {
            NextValue::Allowed => true,
            NextValue::Text(text) => state.validate(text),
            NextValue::Unknown => false,
        };
        if !allowed {
            e.prevent_default();
        }
    });

    // Composed text (IMEs, autocorrect) can't be rejected while composing: restore the text from
    // before the composition if the result is invalid.
    let composition_start = StoredValue::new(None::<(String, Option<u32>, Option<u32>)>);
    let on_compositionstart = EventHandler::new(move |_: CompositionEvent| {
        if let Some(input) = element
            .get_untracked()
            .and_then(|el| el.dyn_ref::<web_sys::HtmlInputElement>().cloned())
        {
            composition_start.set_value(Some((
                input.value(),
                input.selection_start().ok().flatten(),
                input.selection_end().ok().flatten(),
            )));
        }
    });
    let on_compositionend = EventHandler::new(move |_: CompositionEvent| {
        let Some(input) = element
            .get_untracked()
            .and_then(|el| el.dyn_ref::<web_sys::HtmlInputElement>().cloned())
        else {
            return;
        };
        if state.validate(input.value()) {
            return;
        }
        if let Some((value, start, end)) = composition_start.get_value() {
            input.set_value(&value);
            let _ = input.set_selection_range_with_direction(
                start.unwrap_or(0),
                end.unwrap_or(0),
                "none",
            );
            state.set_input_value(value);
        }
    });

    // Pasting over the whole text commits the pasted text right away, so it shows formatted.
    let on_paste = EventHandler::new(move |e: ClipboardEvent| {
        let Some(input) = e
            .target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
        else {
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
    let labelled_by = (field_label.get_untracked().is_none())
        .then(|| {
            if has_label {
                Some(label_props.id.clone())
            } else {
                aria_labelledby
            }
        })
        .flatten();
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
    let stepper = |spin_button: UseButtonInput,
                   verb: &'static str,
                   custom_label: MaybeProp<String>,
                   can_step: Signal<bool>| {
        let button_id = use_id("number-field-stepper");
        let uses_labelled_by = labelled_by.is_some() && custom_label.get_untracked().is_none();
        UseButtonInput {
            id: uses_labelled_by.then(|| button_id.clone().into()),
            aria_label: MaybeProp::derive(move || {
                custom_label.get().or_else(|| {
                    Some(match field_label.get() {
                        Some(label) => format!("{verb} {label}"),
                        None => verb.to_owned(),
                    })
                })
            }),
            aria_labelledby: uses_labelled_by
                .then(|| format!("{button_id} {}", labelled_by.clone().unwrap_or_default()).into()),
            aria_controls: Signal::stored(Some(input_id.clone())),
            exclude_from_tab_order: Signal::stored(true),
            prevent_focus_on_press: true,
            allow_focus_when_disabled: true,
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
        "Increase",
        increment_aria_label,
        state.can_increment,
    );
    let decrement_button = stepper(
        spin_decrement_button,
        "Decrease",
        decrement_aria_label,
        state.can_decrement,
    );

    UseNumberFieldReturn {
        group_props: UseNumberFieldGroupProps {
            aria_disabled: Signal::derive(move || is_disabled.get().then_some("true")),
            aria_invalid: Signal::derive(move || is_invalid.get().then_some(AriaInvalid::True)),
            on_focusin: focus_within.on_focusin,
            on_focusout: focus_within.on_focusout,
        },
        label_props,
        input_props: UseNumberFieldInputProps {
            text_field: text_field_props,
            // Not on iOS, so that VoiceOver announces the required state.
            aria_roledescription: (!device::is_ios()).then_some("Number field"),
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

/// What a `beforeinput` would make of the input's text.
enum NextValue {
    /// Always allowed (undo/redo, line breaks submitting the form).
    Allowed,
    Text(String),
    /// Not computable: rejected.
    Unknown,
}

/// The input's text after the edit `input_type` (with `data`) would apply (react-aria's
/// `useFormattedTextField`). Selections are UTF-16 offsets.
fn next_input_value(
    input: &web_sys::HtmlInputElement,
    input_type: &str,
    data: Option<String>,
) -> NextValue {
    let value: Vec<u16> = input.value().encode_utf16().collect();
    let clamp = |offset: Option<u32>| {
        usize::try_from(offset.unwrap_or(0))
            .unwrap_or(usize::MAX)
            .min(value.len())
    };
    let start = clamp(input.selection_start().ok().flatten());
    let end = clamp(input.selection_end().ok().flatten()).max(start);
    let text = |parts: &[&[u16]]| String::from_utf16_lossy(&parts.concat());
    NextValue::Text(match input_type {
        "historyUndo" | "historyRedo" | "insertLineBreak" => return NextValue::Allowed,
        "deleteContentForward" if start == end => {
            text(&[&value[..start], &value[(end + 1).min(value.len())..]])
        }
        "deleteContentBackward" if start == end => {
            text(&[&value[..start.saturating_sub(1)], &value[start..]])
        }
        // Deleting the selection.
        "deleteContent"
        | "deleteByCut"
        | "deleteByDrag"
        | "deleteContentForward"
        | "deleteContentBackward" => text(&[&value[..start], &value[end..]]),
        "deleteSoftLineBackward" | "deleteHardLineBackward" => text(&[&value[start..]]),
        _ => match data {
            Some(data) => {
                let data: Vec<u16> = data.encode_utf16().collect();
                text(&[&value[..start], &data, &value[end..]])
            }
            None => return NextValue::Unknown,
        },
    })
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
        let Some(probe) = use_document()
            .as_ref()
            .and_then(|d| d.create_element("input").ok())
            .and_then(|el| el.dyn_into::<web_sys::HtmlInputElement>().ok())
        else {
            return;
        };
        probe.set_type("number");
        probe.set_min(&min.map(|v| v.to_string()).unwrap_or_default());
        probe.set_max(&max.map(|v| v.to_string()).unwrap_or_default());
        probe.set_step(&step.to_string());
        probe.set_value(&value.map(|v| v.to_string()).unwrap_or_default());

        let (own, range) = (input.validity(), probe.validity());
        let valid = own.valid() && range.valid();
        let message = Some(input.validation_message().unwrap_or_default())
            .filter(|m| !m.is_empty())
            .or_else(|| probe.validation_message().ok().filter(|m| !m.is_empty()));
        state.validation.update_validation.run(ValidationResult {
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
        if state.validation_behavior == ValidationBehavior::Native && !range.valid() {
            input.set_custom_validity(&probe.validation_message().unwrap_or_default());
        }
    };
    // When the value or range changes, and right before each commit (react-aria: in a layout
    // effect, which runs before the commit's effect). Registered after the text field's reader,
    // so its merged result wins.
    Effect::new(move |_| {
        state.validation.realtime_validation.track();
        state.number_value.track();
        state.min_value.track();
        state.max_value.track();
        state.step.track();
        if element.get().is_some() {
            sync();
        }
    });
    state
        .validation
        .native_validity_readers
        .register(Callback::new(move |()| sync()));
}

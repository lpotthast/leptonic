// Upstream: react-aria/src/color/useColorField.ts @ 99e6102368
use leptos::{
    ev::{self},
    prelude::*,
};
use web_sys::{CompositionEvent, FocusEvent, InputEvent, WheelEvent};

use super::use_color_field_state::ColorFieldState;
use crate::{
    CapturedElement, EventHandler, IntoAttrs, OnEvent, SlotProps,
    hooks::{
        focus::{UseFocusWithinInput, UseFocusWithinReturn, use_focus_within},
        form::{
            InputType, TextFieldElement,
            use_form_reset::{UseFormResetInput, use_form_reset},
            use_form_validation_state::ValidityStateSnapshot,
            use_formatted_text_field::{
                FormattedTextFieldHandlers, UseFormattedTextFieldInput, use_formatted_text_field,
            },
            use_label::UseLabelProps,
            use_text_field::{
                UseTextFieldInput, UseTextFieldInputAttrs, UseTextFieldInputProps,
                UseTextFieldReturn, use_text_field,
            },
            use_text_field_state::{UseTextFieldStateInput, use_text_field_state},
        },
        interactions::{
            UseScrollWheelInput, use_keyboard::KeyboardEventWrapper, use_scroll_wheel,
            use_scroll_wheel::ScrollEvent,
        },
        spinbutton::{UseSpinButtonInput, UseSpinButtonReturn, use_spin_button},
    },
    utils::{
        color::{ColorValue, RGB8},
        id::use_id,
        key::KeyboardKey,
        keyboard_shortcut::{KeyboardShortcuts, Shortcut, ShortcutOutcome},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Returns the label, description and error message props with the input's (as
//   `use_text_field`), and the captured input element.
//
// =============================================================================

/// Input of [`use_color_field`].
#[derive(Clone)]
pub struct UseColorFieldInput<C: ColorValue = RGB8> {
    pub state: ColorFieldState<C>,
    /// The input's id. Generated when `None`.
    pub id: Option<String>,
    /// Whether a visible label is rendered (with `label_props`).
    pub has_label: Signal<bool>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Option<String>,
    pub is_disabled: Signal<bool>,
    pub is_read_only: Signal<bool>,
    pub is_required: Signal<bool>,
    pub placeholder: MaybeProp<String>,
    pub auto_focus: bool,
    /// Whether the scroll wheel leaves the color alone (it steps while the field has focus).
    pub is_wheel_disabled: bool,
    pub on_focus: Option<Callback<FocusEvent>>,
    pub on_blur: Option<Callback<FocusEvent>>,
    pub on_focus_change: Option<Callback<bool>>,
    pub on_key_down: Option<Callback<KeyboardEventWrapper>>,
    pub on_key_up: Option<Callback<KeyboardEventWrapper>>,
}

/// Return value of [`use_color_field`].
pub struct UseColorFieldReturn {
    pub label_props: UseLabelProps,
    pub input_props: UseColorFieldInputProps,
    pub description_props: SlotProps,
    pub error_message_props: SlotProps,
    /// The input element.
    pub element: CapturedElement,
    pub is_focused: Signal<bool>,
    pub is_focus_visible: Signal<bool>,
    pub is_invalid: Signal<bool>,
    pub validation_errors: Signal<Vec<String>>,
    pub validation_details: Signal<ValidityStateSnapshot>,
}

/// Props of the color field's input.
#[derive(Debug, Clone)]
pub struct UseColorFieldInputProps {
    pub text_field: UseTextFieldInputProps,
    pub on_beforeinput: EventHandler<InputEvent>,
    pub on_compositionstart: EventHandler<CompositionEvent>,
    pub on_compositionend: EventHandler<CompositionEvent>,
    pub on_wheel: EventHandler<WheelEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
}

pub type UseColorFieldInputAttrs = (
    UseTextFieldInputAttrs,
    (
        OnEvent<ev::beforeinput>,
        OnEvent<ev::compositionstart>,
        OnEvent<ev::compositionend>,
        OnEvent<ev::wheel>,
        OnEvent<ev::focusin>,
        OnEvent<ev::focusout>,
    ),
);

impl IntoAttrs for UseColorFieldInputProps {
    type Attrs = UseColorFieldInputAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.text_field.into_attrs(),
            (
                self.on_beforeinput.into_on(ev::beforeinput),
                self.on_compositionstart.into_on(ev::compositionstart),
                self.on_compositionend.into_on(ev::compositionend),
                self.on_wheel.into_on(ev::wheel),
                self.on_focusin.into_on(ev::focusin),
                self.on_focusout.into_on(ev::focusout),
            ),
        )
    }
}

/// Behavior and accessibility of a field for a color as hex text: typing is limited to hex
/// digits, the color is committed on blur and Enter, arrow keys, Page Up/Down, Home/End and the
/// scroll wheel step it (a text field with a spin button's keys).
#[allow(clippy::too_many_lines)]
pub fn use_color_field<C: ColorValue>(input: UseColorFieldInput<C>) -> UseColorFieldReturn {
    let UseColorFieldInput {
        state,
        id,
        has_label,
        aria_label,
        aria_labelledby,
        aria_describedby,
        is_disabled,
        is_read_only,
        is_required,
        placeholder,
        auto_focus,
        is_wheel_disabled,
        on_focus,
        on_blur,
        on_focus_change,
        on_key_down,
        on_key_up,
    } = input;
    let inactive = move || is_disabled.get_untracked() || is_read_only.get_untracked();

    // Enter commits; its default action (submitting the form) is kept.
    let shortcuts = KeyboardShortcuts::new().on(Shortcut::new(KeyboardKey::Enter), move |_| {
        if inactive() {
            return ShortcutOutcome::Ignored;
        }
        state.commit();
        state.validation.commit_validation();
        ShortcutOutcome::Custom {
            prevent_default: false,
            continue_propagation: false,
        }
    });
    let commit_on_blur = Callback::new(move |e: FocusEvent| {
        state.commit();
        if let Some(on_blur) = on_blur {
            on_blur.try_run(e);
        }
    });

    // Typing changes the text only while it is (the beginning of) a hex color.
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
        id: Some(id.unwrap_or_else(|| use_id("color-field"))),
        is_disabled,
        is_read_only,
        is_required,
        validation: Some(state.validation),
        validation_behavior: state.validation_behavior,
        placeholder,
        auto_complete: Some("off".to_owned()),
        auto_correct: Some(false),
        spell_check: Some(false),
        auto_focus,
        has_label,
        aria_label,
        aria_labelledby,
        aria_describedby,
        on_focus,
        on_blur: Some(commit_on_blur),
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
        input_mode: Signal::stored(None),
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
        initial_value: state.default_color_value(),
        on_reset: Callback::new(move |color| state.set_color_value(color)),
    });

    let FormattedTextFieldHandlers {
        on_beforeinput,
        on_compositionstart,
        on_compositionend,
    } = use_formatted_text_field(UseFormattedTextFieldInput { element, state });

    // The spin button's keys (arrows, Page Up/Down, Home/End), not its role: the input stays a
    // text box without value attributes (react-aria).
    let UseSpinButtonReturn { props: spin, .. } = use_spin_button(UseSpinButtonInput {
        value: Signal::derive(move || state.color_value.get().map(|c| c.to_rgb8().to_hex_int())),
        text_value: Signal::derive(move || {
            state.color_value.get().map(|c| c.to_rgb8().to_string())
        }),
        min_value: Signal::stored(Some(0)),
        max_value: Signal::stored(Some(0xFF_FF_FF_u32)),
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

    UseColorFieldReturn {
        label_props,
        input_props: UseColorFieldInputProps {
            text_field: text_field_props,
            on_beforeinput,
            on_compositionstart,
            on_compositionend,
            on_wheel: wheel.props.on_wheel,
            on_focusin: focus_within.on_focusin,
            on_focusout: focus_within.on_focusout,
        },
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

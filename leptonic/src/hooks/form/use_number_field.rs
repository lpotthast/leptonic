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
use web_sys::{Event, FocusEvent, KeyboardEvent, PointerEvent, WheelEvent};

use super::{
    use_form_reset::{UseFormResetInput, use_form_reset},
    use_form_validation::{UseFormValidationInput, use_form_validation},
    use_form_validation_state::{
        UseFormValidationStateInput, ValidateFn, ValidationBehavior, ValidityStateSnapshot,
        use_form_validation_state,
    },
    use_number_field_state::UseNumberFieldStateReturn,
};
use crate::{
    hooks::{
        IntoAttrs,
        focus::{
            use_focus_ring::{UseFocusRingInput, UseFocusRingReturn, use_focus_ring},
            use_focus_within::{UseFocusWithinInput, UseFocusWithinReturn, use_focus_within},
        },
        interactions::use_scroll_wheel::{
            ScrollEvent, UseScrollWheelInput, UseScrollWheelReturn, use_scroll_wheel,
        },
        spinbutton::use_spin_button::{UseSpinButtonInput, UseSpinButtonReturn, use_spin_button},
    },
    utils::{
        CapturedElement, ElementCaptureAttr, EventHandler,
        aria::{AriaInvalid, AriaLive, AriaRequired, AriaRole},
        platform::device,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/numberfield/src/useNumberField.ts

// REACT-ARIA DEVIATIONS
//
// 1. Hook-owned state: Callers must pass `UseNumberFieldStateReturn` from
//    `use_number_field_state` instead of inline value/onChange.
//
// 2. ICU4X instead of `Intl.NumberFormat` for SSR safety.
//
// 3. No separate `useFormattedTextField` hook: input filtering and
//    composition handling are integrated directly.
//
// 4. PointerEvent-only for button auto-repeat (per CLAUDE.md: PointerEvent
//    is always available).
//
// 5. `role="spinbutton"` is removed from the input element (matching
//    react-aria which explicitly nullifies it for VoiceOver compatibility).

/// Input parameters for the `use_number_field` hook.
#[derive(Clone)]
pub struct UseNumberFieldInput {
    /// The number field state (from `use_number_field_state`).
    pub state: UseNumberFieldStateReturn,

    /// Callback when input receives focus.
    pub on_focus: Option<Callback<()>>,

    /// Callback when input loses focus.
    pub on_blur: Option<Callback<()>>,

    /// Whether the field is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the field is read-only.
    pub is_read_only: Signal<bool>,

    /// Whether the field is required.
    pub is_required: bool,

    /// Whether the field is explicitly marked as invalid (controlled validation).
    pub is_invalid: Option<Signal<bool>>,

    /// Custom client-side validation function.
    pub validate: Option<ValidateFn<Option<f64>>>,

    /// Validation behavior mode.
    pub validation_behavior: ValidationBehavior,

    /// The default value to restore on form reset.
    pub default_value: Option<Option<f64>>,

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

    /// The minimum value (for ARIA and inputmode computation).
    pub min_value: Option<f64>,

    /// The maximum value (for ARIA and inputmode computation).
    pub max_value: Option<f64>,

    /// The step value (for inputmode computation).
    pub step: f64,

    /// Whether to auto-focus the input on mount.
    pub auto_focus: bool,

    /// Whether scroll wheel increment/decrement is disabled.
    pub is_wheel_disabled: bool,

    /// Custom aria-label for the increment button.
    pub increment_aria_label: Option<String>,

    /// Custom aria-label for the decrement button.
    pub decrement_aria_label: Option<String>,
}

impl Default for UseNumberFieldInput {
    fn default() -> Self {
        Self {
            state: UseNumberFieldStateReturn::empty(),
            on_focus: None,
            on_blur: None,
            is_disabled: Signal::derive(|| false),
            is_read_only: Signal::derive(|| false),
            is_required: false,
            is_invalid: None,
            validate: None,
            validation_behavior: ValidationBehavior::default(),
            default_value: None,
            placeholder: None,
            aria_label: None,
            name: None,
            label: None,
            description: None,
            min_value: None,
            max_value: None,
            step: 1.0,
            auto_focus: false,
            is_wheel_disabled: false,
            increment_aria_label: None,
            decrement_aria_label: None,
        }
    }
}

/// The return value of the `use_number_field` hook.
pub struct UseNumberFieldReturn {
    /// Props for the group wrapper element.
    pub group_props: UseNumberFieldGroupProps,

    /// Props for the input element.
    pub input_props: UseNumberFieldInputProps,

    /// Props for the increment button element.
    pub increment_button_props: UseNumberFieldButtonProps,

    /// Props for the decrement button element.
    pub decrement_button_props: UseNumberFieldButtonProps,

    /// Props for the label element.
    pub label_props: UseNumberFieldLabelProps,

    /// Props for the description element.
    pub description_props: UseNumberFieldDescriptionProps,

    /// Props for the error message element.
    pub error_props: UseNumberFieldErrorProps,

    /// The formatted display value (from `state.input_value`).
    pub display_value: Signal<String>,

    /// Whether increment is allowed.
    pub can_increment: Signal<bool>,

    /// Whether decrement is allowed.
    pub can_decrement: Signal<bool>,

    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,

    /// Whether the displayed validation is invalid.
    pub is_invalid: Signal<bool>,

    /// The displayed validation error messages.
    pub validation_errors: Signal<Vec<String>>,

    /// Detailed validity state (mirrors native `ValidityState`).
    pub validation_details: Signal<ValidityStateSnapshot>,
}

/// Props for the group wrapper element (wraps input + buttons).
#[derive(Debug)]
pub struct UseNumberFieldGroupProps {
    pub role: AriaRole,
    pub aria_disabled: Signal<Option<&'static str>>,
    pub aria_invalid: Signal<Option<AriaInvalid>>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
}

impl IntoAttrs for UseNumberFieldGroupProps {
    type Attrs = UseNumberFieldGroupAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            custom_attribute("aria-disabled", self.aria_disabled),
            Attr(attr::AriaInvalid, self.aria_invalid),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
        )
    }
}

pub type UseNumberFieldGroupAttrs = (
    Attr<attr::Role, AriaRole>,
    CustomAttr<&'static str, Signal<Option<&'static str>>>,
    Attr<attr::AriaInvalid, Signal<Option<AriaInvalid>>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
);

/// Props from `use_number_field` for the input element.
#[derive(Debug)]
pub struct UseNumberFieldInputProps {
    pub id: String,
    pub r#type: &'static str,
    pub name: Option<&'static str>,
    pub placeholder: Option<&'static str>,
    pub disabled: Signal<bool>,
    pub readonly: Signal<bool>,
    pub aria_label: Option<&'static str>,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Signal<Option<String>>,
    pub aria_invalid: Signal<Option<AriaInvalid>>,
    pub aria_required: Option<AriaRequired>,
    pub aria_roledescription: Option<&'static str>,
    pub autofocus: bool,
    pub autocorrect: &'static str,
    pub spellcheck: &'static str,
    pub inputmode: &'static str,
    pub data_focus_visible: Signal<Option<&'static str>>,
    pub element_capture: ElementCaptureAttr,
    pub on_input: EventHandler<Event>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
    pub on_wheel: EventHandler<WheelEvent>,
}

impl IntoAttrs for UseNumberFieldInputProps {
    type Attrs = UseNumberFieldInputAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Type, self.r#type),
            Attr(attr::Name, self.name),
            Attr(attr::Placeholder, self.placeholder),
            Attr(attr::Disabled, self.disabled),
            Attr(attr::Readonly, self.readonly),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            Attr(attr::AriaInvalid, self.aria_invalid),
            Attr(attr::AriaRequired, self.aria_required),
            Attr(attr::AriaRoledescription, self.aria_roledescription),
            Attr(attr::Autofocus, self.autofocus),
            custom_attribute("autocorrect", self.autocorrect),
            Attr(attr::Spellcheck, self.spellcheck),
            Attr(attr::Inputmode, self.inputmode),
            custom_attribute("data-focus-visible", self.data_focus_visible),
            self.element_capture,
            self.on_input.into_on(ev::input),
            self.on_keydown.into_on(ev::keydown),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
            self.on_wheel.into_on(ev::wheel),
        )
    }
}

pub type UseNumberFieldInputAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Type, &'static str>,
    Attr<attr::Name, Option<&'static str>>,
    Attr<attr::Placeholder, Option<&'static str>>,
    Attr<attr::Disabled, Signal<bool>>,
    Attr<attr::Readonly, Signal<bool>>,
    Attr<attr::AriaLabel, Option<&'static str>>,
    Attr<attr::AriaLabelledby, Option<String>>,
    Attr<attr::AriaDescribedby, Signal<Option<String>>>,
    Attr<attr::AriaInvalid, Signal<Option<AriaInvalid>>>,
    Attr<attr::AriaRequired, Option<AriaRequired>>,
    Attr<attr::AriaRoledescription, Option<&'static str>>,
    Attr<attr::Autofocus, bool>,
    CustomAttr<&'static str, &'static str>,
    Attr<attr::Spellcheck, &'static str>,
    Attr<attr::Inputmode, &'static str>,
    CustomAttr<&'static str, Signal<Option<&'static str>>>,
    ElementCaptureAttr,
    On<ev::input, SharedEventCallback<Event>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
    On<ev::wheel, SharedEventCallback<WheelEvent>>,
);

/// Props for increment/decrement button elements.
#[derive(Debug)]
pub struct UseNumberFieldButtonProps {
    pub r#type: &'static str,
    pub aria_label: String,
    pub aria_controls: String,
    pub tabindex: &'static str,
    pub disabled: Signal<bool>,
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub on_pointerup: EventHandler<PointerEvent>,
    pub on_pointerleave: EventHandler<PointerEvent>,
}

impl IntoAttrs for UseNumberFieldButtonProps {
    type Attrs = UseNumberFieldButtonAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Type, self.r#type),
            Attr(attr::AriaLabel, self.aria_label),
            custom_attribute("aria-controls", self.aria_controls),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::Disabled, self.disabled),
            self.on_pointerdown.into_on(ev::pointerdown),
            self.on_pointerup.into_on(ev::pointerup),
            self.on_pointerleave.into_on(ev::pointerleave),
        )
    }
}

pub type UseNumberFieldButtonAttrs = (
    Attr<attr::Type, &'static str>,
    Attr<attr::AriaLabel, String>,
    CustomAttr<&'static str, String>,
    Attr<attr::Tabindex, &'static str>,
    Attr<attr::Disabled, Signal<bool>>,
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
    On<ev::pointerup, SharedEventCallback<PointerEvent>>,
    On<ev::pointerleave, SharedEventCallback<PointerEvent>>,
);

/// Props for the label element.
#[derive(Debug)]
pub struct UseNumberFieldLabelProps {
    pub id: String,
    pub html_for: String,
}

impl IntoAttrs for UseNumberFieldLabelProps {
    type Attrs = UseNumberFieldLabelAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Id, self.id), Attr(attr::For, self.html_for))
    }
}

pub type UseNumberFieldLabelAttrs = (Attr<attr::Id, String>, Attr<attr::For, String>);

/// Props for the description element.
#[derive(Debug)]
pub struct UseNumberFieldDescriptionProps {
    pub id: String,
}

impl IntoAttrs for UseNumberFieldDescriptionProps {
    type Attrs = UseNumberFieldDescriptionAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Id, self.id),)
    }
}

pub type UseNumberFieldDescriptionAttrs = (Attr<attr::Id, String>,);

/// Props for the error message element.
#[derive(Debug)]
pub struct UseNumberFieldErrorProps {
    pub id: String,
    pub role: AriaRole,
    pub aria_live: AriaLive,
}

impl IntoAttrs for UseNumberFieldErrorProps {
    type Attrs = UseNumberFieldErrorAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaLive, self.aria_live),
        )
    }
}

pub type UseNumberFieldErrorAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaLive, AriaLive>,
);

/// Provides the behavior and accessibility implementation for a number field.
#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn use_number_field(input: UseNumberFieldInput) -> UseNumberFieldReturn {
    let UseNumberFieldInput {
        state,
        on_focus,
        on_blur,
        is_disabled,
        is_read_only,
        is_required,
        is_invalid,
        validate,
        validation_behavior,
        default_value,
        placeholder,
        aria_label,
        name,
        label,
        description,
        min_value,
        max_value: _,
        step,
        auto_focus,
        is_wheel_disabled,
        increment_aria_label,
        decrement_aria_label,
    } = input;

    // ---- Element capture for DOM access ----
    let element = CapturedElement::new();

    // ---- Form validation state ----
    let validation = use_form_validation_state(UseFormValidationStateInput {
        is_invalid,
        value: state.number_value,
        validate,
        validation_behavior,
        name: name.map(ToString::to_string),
    });

    // ---- Form reset ----
    let initial_value = default_value.unwrap_or_else(|| state.number_value.get_untracked());
    use_form_reset(UseFormResetInput {
        element,
        initial_value,
        on_reset: Callback::new(move |val: Option<f64>| {
            state.set_number_value.run(val);
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
    let input_id = format!("numberfield-{base_id}");
    let label_id = format!("numberfield-label-{base_id}");
    let description_id = format!("numberfield-description-{base_id}");
    let error_id = format!("numberfield-error-{base_id}");

    // ---- Focus ring (on input) ----
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
        on_blur: on_blur.map(|cb| {
            Callback::new(move |_| {
                // Commit value on blur.
                state.commit.run(());
                cb.run(());
            })
        }),
        on_focus_change: None,
    });

    // If no on_blur callback, still commit on blur via a wrapper.
    // The focus_ring on_blur already handles this when on_blur is Some.
    // When on_blur is None, we need to commit on blur separately.
    // This is handled by overriding on_blur in focus_ring_props if needed.

    // ---- Focus within (on group) ----
    let UseFocusWithinReturn {
        props: focus_within_props,
        is_focus_within,
    } = use_focus_within(UseFocusWithinInput {
        disabled: is_disabled,
        on_focus_within: None,
        on_blur_within: None,
        on_focus_within_change: None,
    });

    // ---- Spin button ----
    let UseSpinButtonReturn {
        on_keydown: spin_keydown,
        increment_button_props: spin_inc_props,
        decrement_button_props: spin_dec_props,
    } = use_spin_button(UseSpinButtonInput {
        text_value: state.input_value,
        is_disabled,
        is_read_only,
        on_increment: state.increment,
        on_decrement: state.decrement,
        on_increment_to_max: state.increment_to_max,
        on_decrement_to_min: state.decrement_to_min,
    });

    // ---- Scroll wheel ----
    let scroll_disabled = Signal::derive(move || {
        is_wheel_disabled || is_disabled.get() || is_read_only.get() || !is_focus_within.get()
    });

    let UseScrollWheelReturn {
        props: scroll_wheel_props,
    } = use_scroll_wheel(UseScrollWheelInput {
        disabled: scroll_disabled,
        on_scroll: Some(Callback::new(move |e: ScrollEvent| {
            // Only respond to primarily vertical scrolling.
            if e.delta_y.abs() <= e.delta_x.abs() {
                return;
            }
            if e.delta_y > 0.0 {
                state.decrement.run(());
            } else if e.delta_y < 0.0 {
                state.increment.run(());
            }
        })),
    });

    // ---- Input handler (text changes during typing) ----
    let handle_input = move |e: Event| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }
        if let Some(input_el) = e.target().and_then(|t| {
            use wasm_bindgen::JsCast;
            t.dyn_into::<web_sys::HtmlInputElement>().ok()
        }) {
            let text = input_el.value();
            // Validate partial input before accepting.
            if state.validate.run(text.clone()) {
                state.set_input_value.run(text);
            } else {
                // Revert the DOM input to the current input_value.
                input_el.set_value(&state.input_value.get_untracked());
            }
        }
    };

    // ---- Keyboard handler (merged: spin button + Enter commit) ----
    let handle_keydown = move |e: KeyboardEvent| {
        if e.key() == "Enter" && !e.is_composing() {
            e.prevent_default();
            state.commit.run(());
            return;
        }
        // Delegate to spin button for ArrowUp/Down, Home/End, PageUp/Down.
        spin_keydown.call(e);
    };

    // ---- Blur handler (commit on blur) ----
    // If on_blur is None, we need to ensure commit happens on blur.
    // The focus_ring on_blur already handles commit when on_blur is Some.
    // For the None case, we wrap it here.
    let blur_handler = if on_blur.is_none() {
        EventHandler::new(move |e: FocusEvent| {
            state.commit.run(());
            focus_ring_props.on_blur.call(e);
        })
    } else {
        focus_ring_props.on_blur
    };

    // ---- Dynamic inputmode ----
    let allows_negative = min_value.is_none_or(|m| m < 0.0);
    let step_is_integer = step.fract() == 0.0;
    let inputmode = if device::is_iphone() {
        // iPhone lacks minus key in numeric/decimal keyboards.
        if allows_negative {
            "text"
        } else if step_is_integer {
            "numeric"
        } else {
            "decimal"
        }
    } else if device::is_android() || (step_is_integer && !allows_negative) {
        "numeric"
    } else {
        "decimal"
    };

    // ---- ARIA ----
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

    let aria_required = is_required.then_some(AriaRequired::True);

    // aria-roledescription: "Number field" except on iOS.
    let aria_roledescription = if device::is_ios() {
        None
    } else {
        Some("Number field")
    };

    // ---- Group ARIA ----
    let group_aria_disabled = Signal::derive(move || is_disabled.get().then_some("true"));
    let group_aria_invalid = aria_invalid;

    // ---- Button labels ----
    let field_label = label.as_deref().or(aria_label).unwrap_or("value");
    let inc_label = increment_aria_label.unwrap_or_else(|| format!("Increase {field_label}"));
    let dec_label = decrement_aria_label.unwrap_or_else(|| format!("Decrease {field_label}"));

    // ---- Button disabled signals ----
    let increment_disabled = Signal::derive(move || !state.can_increment.get());
    let decrement_disabled = Signal::derive(move || !state.can_decrement.get());

    // ---- Validation details ----
    let validation_details =
        Signal::derive(move || validation.display_validation.get().validation_details);

    let input_id_clone = input_id.clone();

    UseNumberFieldReturn {
        group_props: UseNumberFieldGroupProps {
            role: AriaRole::Group,
            aria_disabled: group_aria_disabled,
            aria_invalid: group_aria_invalid,
            on_focusin: focus_within_props.on_focusin,
            on_focusout: focus_within_props.on_focusout,
        },
        input_props: UseNumberFieldInputProps {
            id: input_id.clone(),
            r#type: "text",
            name,
            placeholder,
            disabled: is_disabled,
            readonly: is_read_only,
            aria_label,
            aria_labelledby,
            aria_describedby,
            aria_invalid,
            aria_required,
            aria_roledescription,
            autofocus: auto_focus,
            autocorrect: "off",
            spellcheck: "false",
            inputmode,
            data_focus_visible: focus_ring_props.data_focus_visible,
            element_capture: element.attr(),
            on_input: EventHandler::new(handle_input),
            on_keydown: EventHandler::new(handle_keydown),
            on_focus: focus_ring_props.on_focus,
            on_blur: blur_handler,
            on_focusin: focus_ring_props.on_focusin,
            on_focusout: focus_ring_props.on_focusout,
            on_wheel: scroll_wheel_props.on_wheel,
        },
        increment_button_props: UseNumberFieldButtonProps {
            r#type: "button",
            aria_label: inc_label,
            aria_controls: input_id_clone.clone(),
            tabindex: "-1",
            disabled: increment_disabled,
            on_pointerdown: spin_inc_props.on_pointerdown,
            on_pointerup: spin_inc_props.on_pointerup,
            on_pointerleave: spin_inc_props.on_pointerleave,
        },
        decrement_button_props: UseNumberFieldButtonProps {
            r#type: "button",
            aria_label: dec_label,
            aria_controls: input_id_clone,
            tabindex: "-1",
            disabled: decrement_disabled,
            on_pointerdown: spin_dec_props.on_pointerdown,
            on_pointerup: spin_dec_props.on_pointerup,
            on_pointerleave: spin_dec_props.on_pointerleave,
        },
        label_props: UseNumberFieldLabelProps {
            id: label_id,
            html_for: input_id,
        },
        description_props: UseNumberFieldDescriptionProps { id: description_id },
        error_props: UseNumberFieldErrorProps {
            id: error_id,
            role: AriaRole::Alert,
            aria_live: AriaLive::Polite,
        },
        display_value: state.input_value,
        can_increment: state.can_increment,
        can_decrement: state.can_decrement,
        is_focus_visible,
        is_invalid: validation.is_invalid,
        validation_errors: validation.validation_errors,
        validation_details,
    }
}

// Upstream: react-aria/src/radio/useRadioGroup.ts @ 99e6102368
// Upstream: react-aria-components/test/RadioGroup.test.js @ 99e6102368
use std::sync::Arc;

use leptos::{ev, prelude::*};
use wasm_bindgen::JsCast;
use web_sys::{FocusEvent, KeyboardEvent};

use super::{
    use_field::{UseFieldInput, UseFieldReturn, use_field},
    use_form_validation_state::ValidityStateSnapshot,
    use_label::{LabelElementType, UseLabelProps},
    use_radio_group_state::RadioGroupState,
};
use crate::{
    CapturedElement, ElementCaptureAttr, EventHandler, Focusability, Id, IntoAttrs, OnEvent, Part,
    SlotProps,
    hooks::{
        collections::Key,
        focus::{
            FocusManager, FocusManagerOptions, FocusWithinEvent, UseFocusWithinInput,
            use_focus_within,
        },
        interactions::{UseKeyboardInput, use_keyboard},
    },
    utils::{
        aria::{
            AriaDisabled, AriaInvalid, AriaOrientation, AriaReadonly, AriaRequired, AriaRole,
            aria_true,
        },
        i18n::{WritingDirection, use_direction},
        key::KeyboardKey,
        keyboard_shortcut::{KeyboardShortcuts, Shortcut},
        orientation::Orientation,
        part::Remount,
        shadow_dom::get_event_target,
        signal_attr::{FixedAttr, SignalAttr, fixed_attr, maybe_attr, signal_attr},
        spread::Spread,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The group hands its radios a `RadioGroupData` (react-aria: a `WeakMap` keyed by the state).
//   Radios register their element with it, so the arrow keys select the radio's `Key` (react-aria
//   reads the input's string `value`).
//
// =============================================================================

/// Input of [`use_radio_group`].
#[derive(Debug, Clone)]
pub struct UseRadioGroupInput {
    pub state: RadioGroupState,
    /// The group element's id. Generated when `None`.
    pub id: Option<Id>,
    /// Whether a visible label is rendered (with `label_props`).
    pub has_label: Signal<bool>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<Id>>,
    pub aria_describedby: Signal<Option<Id>>,
    pub aria_errormessage: Signal<Option<Id>>,
    /// The axis of the arrow keys (react-aria's default: vertical).
    pub orientation: Signal<Orientation>,
    /// The id of the form the radios belong to, when not their ancestor.
    pub form: Option<String>,
    pub on_focus: Option<Callback<FocusEvent>>,
    pub on_blur: Option<Callback<FocusEvent>>,
    pub on_focus_change: Option<Callback<bool>>,
}

/// What the radios of a group need from it.
#[derive(Debug, Clone, Copy)]
pub struct RadioGroupData {
    pub state: RadioGroupState,
    pub(crate) form: StoredValue<Option<String>>,
    pub(crate) description_id: Signal<Option<Id>>,
    pub(crate) error_message_id: Signal<Option<Id>>,
    /// The radios' input elements, by value (for keyboard navigation).
    pub(crate) radios: StoredValue<Vec<(Key, CapturedElement)>>,
}

/// The return value of [`use_radio_group`].
#[derive(Debug)]
pub struct UseRadioGroupReturn {
    /// The radio group element.
    pub group: Part<UseRadioGroupProps>,
    /// The group's label element.
    pub label: Part<UseLabelProps>,
    pub description: Part<SlotProps>,
    pub error_message: Part<SlotProps>,
    /// For [`use_radio`](fn@super::use_radio).
    pub data: RadioGroupData,
    pub is_invalid: Signal<bool>,
    pub validation_errors: Signal<Vec<String>>,
    pub validation_details: Signal<ValidityStateSnapshot>,
}

/// Props for the radio group element.
#[derive(Debug)]
pub struct UseRadioGroupProps {
    pub id: Id,
    pub aria_invalid: Signal<Option<AriaInvalid>>,
    pub aria_errormessage: Signal<Option<Id>>,
    pub aria_readonly: Signal<Option<AriaReadonly>>,
    pub aria_required: Signal<Option<AriaRequired>>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub aria_orientation: Signal<AriaOrientation>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<Id>>,
    pub aria_describedby: Signal<Option<Id>>,
    pub element_capture: ElementCaptureAttr,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
}

impl Remount for UseRadioGroupProps {
    fn remount(&self) -> Self {
        Self {
            id: self.id.clone(),
            aria_invalid: self.aria_invalid,
            aria_errormessage: self.aria_errormessage,
            aria_readonly: self.aria_readonly,
            aria_required: self.aria_required,
            aria_disabled: self.aria_disabled,
            aria_orientation: self.aria_orientation,
            aria_label: self.aria_label,
            aria_labelledby: self.aria_labelledby,
            aria_describedby: self.aria_describedby,
            element_capture: self.element_capture.clone(),
            on_keydown: self.on_keydown.clone(),
            on_focusin: self.on_focusin.clone(),
            on_focusout: self.on_focusout.clone(),
        }
    }
}

pub type UseRadioGroupAttrs = Spread<(
    (
        FixedAttr,
        FixedAttr,
        SignalAttr<Option<AriaInvalid>>,
        SignalAttr<Option<Id>>,
        SignalAttr<Option<AriaReadonly>>,
        SignalAttr<Option<AriaRequired>>,
        SignalAttr<Option<AriaDisabled>>,
        SignalAttr<AriaOrientation>,
        SignalAttr<Option<String>>,
        SignalAttr<Option<Id>>,
        SignalAttr<Option<Id>>,
        ElementCaptureAttr,
    ),
    OnEvent<ev::keydown>,
    OnEvent<ev::focusin>,
    OnEvent<ev::focusout>,
)>;

impl IntoAttrs for UseRadioGroupProps {
    type Attrs = UseRadioGroupAttrs;

    fn into_attrs(self) -> Self::Attrs {
        Spread((
            (
                fixed_attr("role", AriaRole::Radiogroup),
                fixed_attr("id", self.id),
                signal_attr("aria-invalid", self.aria_invalid),
                signal_attr("aria-errormessage", self.aria_errormessage),
                signal_attr("aria-readonly", self.aria_readonly),
                signal_attr("aria-required", self.aria_required),
                signal_attr("aria-disabled", self.aria_disabled),
                signal_attr("aria-orientation", self.aria_orientation),
                maybe_attr("aria-label", self.aria_label),
                signal_attr("aria-labelledby", self.aria_labelledby),
                signal_attr("aria-describedby", self.aria_describedby),
                self.element_capture,
            ),
            self.on_keydown.into_on(ev::keydown),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
        ))
    }
}

/// Provides the behavior and accessibility of a radio group (`role="radiogroup"`): the arrow
/// keys move the selection between its radios. Render the radios with
/// [`use_radio`](fn@super::use_radio).
pub fn use_radio_group(input: UseRadioGroupInput) -> UseRadioGroupReturn {
    crate::metrics::subject!();
    let UseRadioGroupInput {
        state,
        id,
        has_label,
        aria_label,
        aria_labelledby,
        aria_describedby,
        aria_errormessage,
        orientation,
        form,
        on_focus,
        on_blur,
        on_focus_change,
    } = input;
    let UseFieldReturn {
        label: label_props,
        field: field_props,
        description: description_props,
        error_message: error_message_props,
        description_id,
        error_message_id,
    } = use_field(UseFieldInput {
        id,
        has_label,
        label_element_type: LabelElementType::Span,
        aria_label,
        aria_labelledby,
        aria_describedby,
        ..UseFieldInput::default()
    });
    let field_props = field_props.into_props();

    let focus_within = use_focus_within(UseFocusWithinInput {
        on_blur_within: Some(Callback::new(move |e: FocusWithinEvent| {
            if let Some(on_blur) = on_blur {
                on_blur.try_run(e.event);
            }
            if state.selected_value.get_untracked().is_none() {
                state.set_last_focused_value(None);
            }
        })),
        on_focus_within: on_focus
            .map(|on_focus| Callback::new(move |e: FocusWithinEvent| on_focus.run(e.event))),
        on_focus_within_change: on_focus_change,
        ..UseFocusWithinInput::default()
    });
    let focus_within_props = focus_within.target.into_props();

    let group = CapturedElement::new();
    let radios: StoredValue<Vec<(Key, CapturedElement)>> = StoredValue::new(Vec::new());
    let direction = use_direction();
    // Created once (its state lives in this owner), not per key event.
    let focus_manager = FocusManager::new(move || group.get_untracked().map(|g| (*g).clone()));
    // Focus and select the next (or previous) radio, wrapping around.
    let select_next = move |e: &KeyboardEvent, next: bool| -> bool {
        let options = FocusManagerOptions {
            from: get_event_target(e).and_then(|target| target.dyn_into::<web_sys::Element>().ok()),
            wrap: true,
            focusability: Focusability::Focusable,
            accept: Some(Arc::new(|el: &web_sys::Element| {
                el.tag_name().eq_ignore_ascii_case("input")
                    && el.get_attribute("type").as_deref() == Some("radio")
            })),
        };
        let focused = if next {
            focus_manager.focus_next(options)
        } else {
            focus_manager.focus_previous(options)
        };
        let Some(focused) = focused else {
            return false;
        };
        let key = radios.with_value(|radios| {
            radios.iter().find_map(|(key, element)| {
                element
                    .get_untracked()
                    .is_some_and(|el| *el == focused)
                    .then(|| key.clone())
            })
        });
        if let Some(key) = key {
            state.set_selected_value(Some(key));
        }
        true
    };
    let horizontal_next = move || {
        !(direction.get_untracked() == WritingDirection::Rtl
            && orientation.get_untracked() != Orientation::Vertical)
    };
    let shortcuts = KeyboardShortcuts::new()
        .on(Shortcut::new(KeyboardKey::ArrowRight), move |e| {
            select_next(e, horizontal_next())
        })
        .on(Shortcut::new(KeyboardKey::ArrowLeft), move |e| {
            select_next(e, !horizontal_next())
        })
        .on(Shortcut::new(KeyboardKey::ArrowDown), move |e| {
            select_next(e, true)
        })
        .on(Shortcut::new(KeyboardKey::ArrowUp), move |e| {
            select_next(e, false)
        });
    let keyboard = use_keyboard(UseKeyboardInput {
        shortcuts: Some(shortcuts),
        allow_repeats: true,
        ..UseKeyboardInput::default()
    });
    let keyboard_props = keyboard.target.into_props();

    let validation = state.validation;
    let is_invalid = state.is_invalid;
    let is_read_only = state.is_read_only;
    let is_required = state.is_required;
    let is_disabled = state.is_disabled;
    UseRadioGroupReturn {
        group: Part::stored(UseRadioGroupProps {
            id: field_props.id,
            aria_invalid: aria_true(is_invalid),
            aria_errormessage,
            aria_readonly: aria_true(is_read_only),
            aria_required: aria_true(is_required),
            aria_disabled: aria_true(is_disabled),
            aria_orientation: Signal::derive(move || orientation.get().into()),
            aria_label: field_props.aria_label,
            aria_labelledby: field_props.aria_labelledby,
            aria_describedby: field_props.aria_describedby,
            element_capture: group.attr(),
            on_keydown: keyboard_props.on_keydown,
            on_focusin: focus_within_props.on_focusin,
            on_focusout: focus_within_props.on_focusout,
        }),
        label: label_props,
        description: description_props,
        error_message: error_message_props,
        data: RadioGroupData {
            state,
            form: StoredValue::new(form),
            description_id,
            error_message_id,
            radios,
        },
        is_invalid,
        validation_errors: validation.validation_errors,
        validation_details: Signal::derive(move || {
            validation.display_validation.get().validation_details
        }),
    }
}

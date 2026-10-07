// Upstream: react-aria/src/radio/useRadioGroup.ts @ 99e6102368
use std::sync::Arc;

use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use web_sys::{FocusEvent, KeyboardEvent};

use super::{
    use_field::{UseFieldInput, UseFieldReturn, use_field},
    use_form_validation_state::ValidityStateSnapshot,
    use_label::{LabelElementType, UseLabelProps},
    use_radio_group_state::RadioGroupState,
};
use crate::{
    hooks::{
        FocusManager, FocusManagerOptions, FocusWithinEvent, IntoAttrs, UseFocusWithinInput,
        UseKeyboardInput, collections::Key, use_focus_within, use_keyboard,
    },
    utils::{
        CapturedElement, ElementCaptureAttr, EventHandler, SlotProps,
        aria::{AriaDisabled, AriaInvalid, AriaOrientation, AriaReadonly, AriaRequired, AriaRole},
        i18n::use_direction,
        keyboard_shortcut::{KeyboardShortcuts, Shortcut},
        locale::WritingDirection,
        orientation::Orientation,
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
    pub id: Option<String>,
    /// Whether a visible label is rendered (with `label_props`).
    pub has_label: Signal<bool>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Option<String>,
    pub aria_errormessage: Option<String>,
    /// The axis of the arrow keys (react-aria's default: vertical).
    pub orientation: Orientation,
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
    pub(crate) description_id: Signal<Option<String>>,
    pub(crate) error_message_id: Signal<Option<String>>,
    /// The radios' input elements, by value (for keyboard navigation).
    pub(crate) radios: StoredValue<Vec<(Key, CapturedElement)>>,
}

/// Output of [`use_radio_group`].
#[derive(Debug)]
pub struct UseRadioGroupReturn {
    /// Props for the group element.
    pub props: UseRadioGroupProps,
    /// Props for the group's label (a `<span>`).
    pub label_props: UseLabelProps,
    pub description_props: SlotProps,
    pub error_message_props: SlotProps,
    /// For [`use_radio`](super::use_radio).
    pub data: RadioGroupData,
    pub is_invalid: Signal<bool>,
    pub validation_errors: Signal<Vec<String>>,
    pub validation_details: Signal<ValidityStateSnapshot>,
}

/// Props for the radio group element.
#[derive(Debug)]
pub struct UseRadioGroupProps {
    pub id: String,
    pub aria_invalid: Signal<Option<AriaInvalid>>,
    pub aria_errormessage: Option<String>,
    pub aria_readonly: Signal<Option<AriaReadonly>>,
    pub aria_required: Signal<Option<AriaRequired>>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub aria_orientation: AriaOrientation,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<String>>,
    pub aria_describedby: Signal<Option<String>>,
    pub element_capture: ElementCaptureAttr,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
}

pub type UseRadioGroupAttrs = (
    Attr<attr::Role, AriaRole>,
    Attr<attr::Id, String>,
    Attr<attr::AriaInvalid, Signal<Option<AriaInvalid>>>,
    Attr<attr::AriaErrormessage, Option<String>>,
    Attr<attr::AriaReadonly, Signal<Option<AriaReadonly>>>,
    Attr<attr::AriaRequired, Signal<Option<AriaRequired>>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    Attr<attr::AriaOrientation, AriaOrientation>,
    Attr<attr::AriaLabel, MaybeProp<String>>,
    Attr<attr::AriaLabelledby, Signal<Option<String>>>,
    Attr<attr::AriaDescribedby, Signal<Option<String>>>,
    ElementCaptureAttr,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
);

impl IntoAttrs for UseRadioGroupProps {
    type Attrs = UseRadioGroupAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, AriaRole::Radiogroup),
            Attr(attr::Id, self.id),
            Attr(attr::AriaInvalid, self.aria_invalid),
            Attr(attr::AriaErrormessage, self.aria_errormessage),
            Attr(attr::AriaReadonly, self.aria_readonly),
            Attr(attr::AriaRequired, self.aria_required),
            Attr(attr::AriaDisabled, self.aria_disabled),
            Attr(attr::AriaOrientation, self.aria_orientation),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            self.element_capture,
            self.on_keydown.into_on(ev::keydown),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
        )
    }
}

/// Provides the behavior and accessibility of a radio group (`role="radiogroup"`): the arrow
/// keys move the selection between its radios. Render the radios with
/// [`use_radio`](super::use_radio).
#[allow(clippy::too_many_lines)]
pub fn use_radio_group(input: UseRadioGroupInput) -> UseRadioGroupReturn {
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
        label_props,
        field_props,
        description_props,
        error_message_props,
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

    let group = CapturedElement::new();
    let radios: StoredValue<Vec<(Key, CapturedElement)>> = StoredValue::new(Vec::new());
    let direction = use_direction();
    // Focus and select the next (or previous) radio, wrapping around.
    let select_next = move |e: &KeyboardEvent, next: bool| -> bool {
        let focus_manager = FocusManager::new(move || group.get_untracked().map(|g| (*g).clone()));
        let options = FocusManagerOptions {
            from: e
                .target()
                .and_then(|t| wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(t).ok()),
            wrap: true,
            tabbable: false,
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
            && orientation != Orientation::Vertical)
    };
    let shortcuts = KeyboardShortcuts::new()
        .on(Shortcut::key("ArrowRight"), move |e| {
            select_next(e, horizontal_next())
        })
        .on(Shortcut::key("ArrowLeft"), move |e| {
            select_next(e, !horizontal_next())
        })
        .on(Shortcut::key("ArrowDown"), move |e| select_next(e, true))
        .on(Shortcut::key("ArrowUp"), move |e| select_next(e, false));
    let keyboard = use_keyboard(UseKeyboardInput {
        shortcuts: Some(shortcuts),
        allow_repeats: true,
        ..UseKeyboardInput::default()
    });

    let validation = state.validation;
    let is_invalid = state.is_invalid;
    let is_read_only = state.is_read_only;
    let is_required = state.is_required;
    let is_disabled = state.is_disabled;
    UseRadioGroupReturn {
        props: UseRadioGroupProps {
            id: field_props.id,
            aria_invalid: Signal::derive(move || is_invalid.get().then_some(AriaInvalid::True)),
            aria_errormessage,
            aria_readonly: Signal::derive(move || is_read_only.get().then_some(AriaReadonly::True)),
            aria_required: Signal::derive(move || is_required.get().then_some(AriaRequired::True)),
            aria_disabled: Signal::derive(move || is_disabled.get().then_some(AriaDisabled::True)),
            aria_orientation: orientation.into(),
            aria_label: field_props.aria_label,
            aria_labelledby: field_props.aria_labelledby,
            aria_describedby: field_props.aria_describedby,
            element_capture: group.attr(),
            on_keydown: keyboard.props.on_keydown,
            on_focusin: focus_within.props.on_focusin,
            on_focusout: focus_within.props.on_focusout,
        },
        label_props,
        description_props,
        error_message_props,
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

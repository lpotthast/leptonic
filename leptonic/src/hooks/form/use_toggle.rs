// Upstream: react-aria/src/toggle/useToggle.ts @ 99e6102368
use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
    tachys::html::property::{Property, prop},
};
use wasm_bindgen::JsCast;
use web_sys::{DragEvent, Event, FocusEvent, KeyboardEvent, MouseEvent, PointerEvent};

use super::{
    use_form_reset::{UseFormResetInput, use_form_reset},
    use_form_validation::{UseFormValidationInput, use_form_validation},
    use_form_validation_state::{
        FormValidationState, UseFormValidationStateInput, ValidateFn, ValidationBehavior,
        ValidityStateSnapshot, use_form_validation_state,
    },
    use_toggle_state::ToggleState,
};
use crate::{
    hooks::{
        FocusableContextAttr, FocusableContextAttrs, IntoAttrs, PressEvent, PropsWithStyles,
        UseFocusRingInput, UseFocusableInput, UsePressInput, use_focus_ring, use_focusable,
        use_press,
    },
    utils::{
        ElementCaptureAttr, EventAccessors, EventHandler, SlotProps,
        aria::{AriaInvalid, AriaReadonly, AriaRequired, AriaRole},
        join_slot_ids,
        pointer_type::PointerType,
        propagation_control::Propagation,
        use_slot,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The settings shared by all toggles (checkbox, switch, group items) are one
//   `ToggleOptions` struct, so they aren't repeated per hook.
// - `checked` is set as a DOM property (and re-synced after a rejected change, e.g. while
//   read-only), as React does for controlled inputs; an attribute would stop applying once
//   the user toggled the input. The `checked` attribute holds the initial selection (React's
//   `defaultChecked`), which a native form reset restores after the state's own reset.
//
// ## DIFFERENT BEHAVIOR
// - A virtual click on the label (`label.click()`, assistive technology) toggles, as with a
//   native `<label>`. React-aria leaves virtual label presses to the input, but prevents the
//   label's click, so nothing toggles.
// - `is_read_only` also blocks changes here (react-aria: only in `useToggleState`), so a state
//   from elsewhere (`ToggleState::new`, a bound signal) stays read-only too.
//
// ## ADDITIONS
// - Focus ring state (`is_focused`, `is_focus_visible`), which react-aria-components adds with
//   `useFocusRing`: our atoms render it as data attributes.
//
// =============================================================================

/// The settings of a toggle (checkbox, switch), besides its state.
#[derive(Clone)]
pub struct ToggleOptions {
    /// The input's id.
    pub id: Option<String>,
    pub is_disabled: Signal<bool>,
    /// While `true`, the toggle can be focused but not changed.
    pub is_read_only: Signal<bool>,
    pub is_required: Signal<bool>,
    pub is_invalid: Signal<bool>,
    pub validate: Option<ValidateFn<bool>>,
    /// When errors show. `None`: `Aria`; for a checkbox group item, the group's.
    pub validation_behavior: Option<ValidationBehavior>,
    /// The input's `name` (for form submission).
    pub name: Option<String>,
    /// The id of the form the input belongs to, when not its ancestor.
    pub form: Option<String>,
    /// The value submitted with the form while selected (the browser submits `on` otherwise).
    pub value: Option<String>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Option<String>,
    pub aria_errormessage: Option<String>,
    /// The element the toggle controls.
    pub aria_controls: Option<String>,
    pub auto_focus: bool,
    pub exclude_from_tab_order: Signal<bool>,
    pub on_focus: Option<Callback<FocusEvent>>,
    pub on_blur: Option<Callback<FocusEvent>>,
    pub on_focus_change: Option<Callback<bool>>,
    pub on_press_start: Option<Callback<PressEvent>>,
    pub on_press_end: Option<Callback<PressEvent>>,
    pub on_press_up: Option<Callback<PressEvent>>,
    pub on_press: Option<Callback<PressEvent>>,
    pub on_press_change: Option<Callback<bool>>,
}

impl Default for ToggleOptions {
    fn default() -> Self {
        Self {
            id: None,
            is_disabled: Signal::stored(false),
            is_read_only: Signal::stored(false),
            is_required: Signal::stored(false),
            is_invalid: Signal::stored(false),
            validate: None,
            validation_behavior: None,
            name: None,
            form: None,
            value: None,
            aria_label: MaybeProp::default(),
            aria_labelledby: None,
            aria_describedby: None,
            aria_errormessage: None,
            aria_controls: None,
            auto_focus: false,
            exclude_from_tab_order: Signal::stored(false),
            on_focus: None,
            on_blur: None,
            on_focus_change: None,
            on_press_start: None,
            on_press_end: None,
            on_press_up: None,
            on_press: None,
            on_press_change: None,
        }
    }
}

impl std::fmt::Debug for ToggleOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ToggleOptions")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("value", &self.value)
            .finish_non_exhaustive()
    }
}

/// Input of [`use_toggle`].
#[derive(Debug, Clone)]
pub struct UseToggleInput {
    pub state: ToggleState,
    pub options: ToggleOptions,
}

/// Output of [`use_toggle`] (and [`use_checkbox`](fn@super::use_checkbox),
/// [`use_switch`](fn@super::use_switch)).
#[derive(Debug)]
pub struct UseToggleReturn {
    /// Props for the `<label>` wrapping the input: pressing it toggles the input.
    pub label_props: PropsWithStyles<UseToggleLabelProps>,
    /// Props for the `<input type="checkbox">`.
    pub input_props: PropsWithStyles<UseToggleInputProps>,
    /// Props for the description element.
    pub description_props: SlotProps,
    /// Props for the error message element (render it only while invalid).
    pub error_message_props: SlotProps,
    pub is_selected: Signal<bool>,
    /// Whether the input or its label is pressed.
    pub is_pressed: Signal<bool>,
    pub is_disabled: Signal<bool>,
    pub is_read_only: Signal<bool>,
    pub is_invalid: Signal<bool>,
    pub is_focused: Signal<bool>,
    pub is_focus_visible: Signal<bool>,
    pub validation_errors: Signal<Vec<String>>,
    pub validation_details: Signal<ValidityStateSnapshot>,
}

/// Props for the label wrapping the toggle's input.
#[derive(Debug)]
pub struct UseToggleLabelProps {
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_click: EventHandler<MouseEvent>,
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub on_pointerup: EventHandler<PointerEvent>,
    pub on_mousedown: EventHandler<MouseEvent>,
    pub on_dragstart: EventHandler<DragEvent>,
    pub on_dblclick: EventHandler<MouseEvent>,
}

pub type UseToggleLabelAttrs = (
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
    On<ev::pointerup, SharedEventCallback<PointerEvent>>,
    On<ev::mousedown, SharedEventCallback<MouseEvent>>,
    On<ev::dragstart, SharedEventCallback<DragEvent>>,
    On<ev::dblclick, SharedEventCallback<MouseEvent>>,
);

impl IntoAttrs for UseToggleLabelProps {
    type Attrs = UseToggleLabelAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.on_keydown.into_on(ev::keydown),
            self.on_click.into_on(ev::click),
            self.on_pointerdown.into_on(ev::pointerdown),
            self.on_pointerup.into_on(ev::pointerup),
            self.on_mousedown.into_on(ev::mousedown),
            self.on_dragstart.into_on(ev::dragstart),
            self.on_dblclick.into_on(ev::dblclick),
        )
    }
}

/// Props for the toggle's `<input type="checkbox">`.
#[derive(Debug)]
pub struct UseToggleInputProps {
    pub role: Option<AriaRole>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub form: Option<String>,
    pub value: Option<String>,
    pub checked: Signal<bool>,
    /// The `checked` attribute: the initial selection, which a form reset restores.
    pub default_checked: bool,
    /// The `indeterminate` DOM property (checkboxes; there is no attribute).
    pub indeterminate: Signal<bool>,
    pub disabled: Signal<bool>,
    pub required: Signal<bool>,
    pub tabindex: Signal<Option<i32>>,
    pub aria_required: Signal<Option<AriaRequired>>,
    pub aria_invalid: Signal<Option<AriaInvalid>>,
    pub aria_readonly: Signal<Option<AriaReadonly>>,
    pub aria_errormessage: Option<String>,
    pub aria_controls: Option<String>,
    pub aria_describedby: Signal<Option<String>>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
    pub element_capture: ElementCaptureAttr,
    pub on_change: EventHandler<Event>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_keyup: EventHandler<KeyboardEvent>,
    pub on_click: EventHandler<MouseEvent>,
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub on_pointerup: EventHandler<PointerEvent>,
    pub on_mousedown: EventHandler<MouseEvent>,
    pub on_dragstart: EventHandler<DragEvent>,
    pub on_dblclick: EventHandler<MouseEvent>,
    /// A `FocusableContext`'s further attributes (e.g. a tooltip trigger's pointer handlers).
    pub context_attrs: Option<FocusableContextAttrs>,
}

pub type UseToggleInputAttrs = (
    (
        Attr<attr::Type, &'static str>,
        Attr<attr::Role, Option<AriaRole>>,
        Attr<attr::Id, Option<String>>,
        Attr<attr::Name, Option<String>>,
        Attr<attr::Form, Option<String>>,
        Attr<attr::Value, Option<String>>,
        Property<&'static str, Signal<bool>>,
        Attr<attr::Checked, bool>,
        Property<&'static str, Signal<bool>>,
        Attr<attr::Disabled, Signal<bool>>,
        Attr<attr::Required, Signal<bool>>,
        Attr<attr::Tabindex, Signal<Option<i32>>>,
        Attr<attr::AriaRequired, Signal<Option<AriaRequired>>>,
        Attr<attr::AriaInvalid, Signal<Option<AriaInvalid>>>,
        Attr<attr::AriaReadonly, Signal<Option<AriaReadonly>>>,
        Attr<attr::AriaErrormessage, Option<String>>,
        Attr<attr::AriaControls, Option<String>>,
        Attr<attr::AriaDescribedby, Signal<Option<String>>>,
        Attr<attr::AriaLabel, MaybeProp<String>>,
        Attr<attr::AriaLabelledby, Option<String>>,
        ElementCaptureAttr,
    ),
    (
        On<ev::change, SharedEventCallback<Event>>,
        On<ev::focus, SharedEventCallback<FocusEvent>>,
        On<ev::blur, SharedEventCallback<FocusEvent>>,
        On<ev::focusin, SharedEventCallback<FocusEvent>>,
        On<ev::focusout, SharedEventCallback<FocusEvent>>,
        On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
        On<ev::keyup, SharedEventCallback<KeyboardEvent>>,
        On<ev::click, SharedEventCallback<MouseEvent>>,
        On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
        On<ev::pointerup, SharedEventCallback<PointerEvent>>,
        On<ev::mousedown, SharedEventCallback<MouseEvent>>,
        On<ev::dragstart, SharedEventCallback<DragEvent>>,
        On<ev::dblclick, SharedEventCallback<MouseEvent>>,
    ),
    FocusableContextAttr,
);

impl IntoAttrs for UseToggleInputProps {
    type Attrs = UseToggleInputAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            (
                Attr(attr::Type, "checkbox"),
                Attr(attr::Role, self.role),
                Attr(attr::Id, self.id),
                Attr(attr::Name, self.name),
                Attr(attr::Form, self.form),
                Attr(attr::Value, self.value),
                prop("checked", self.checked),
                Attr(attr::Checked, self.default_checked),
                prop("indeterminate", self.indeterminate),
                Attr(attr::Disabled, self.disabled),
                Attr(attr::Required, self.required),
                Attr(attr::Tabindex, self.tabindex),
                Attr(attr::AriaRequired, self.aria_required),
                Attr(attr::AriaInvalid, self.aria_invalid),
                Attr(attr::AriaReadonly, self.aria_readonly),
                Attr(attr::AriaErrormessage, self.aria_errormessage),
                Attr(attr::AriaControls, self.aria_controls),
                Attr(attr::AriaDescribedby, self.aria_describedby),
                Attr(attr::AriaLabel, self.aria_label),
                Attr(attr::AriaLabelledby, self.aria_labelledby),
                self.element_capture,
            ),
            (
                self.on_change.into_on(ev::change),
                self.on_focus.into_on(ev::focus),
                self.on_blur.into_on(ev::blur),
                self.on_focusin.into_on(ev::focusin),
                self.on_focusout.into_on(ev::focusout),
                self.on_keydown.into_on(ev::keydown),
                self.on_keyup.into_on(ev::keyup),
                self.on_click.into_on(ev::click),
                self.on_pointerdown.into_on(ev::pointerdown),
                self.on_pointerup.into_on(ev::pointerup),
                self.on_mousedown.into_on(ev::mousedown),
                self.on_dragstart.into_on(ev::dragstart),
                self.on_dblclick.into_on(ev::dblclick),
            ),
            FocusableContextAttr(self.context_attrs),
        )
    }
}

/// Provides the behavior and accessibility of a toggle on an `<input type="checkbox">` (the
/// base of [`use_checkbox`](fn@super::use_checkbox) and [`use_switch`](fn@super::use_switch)).
pub fn use_toggle(input: UseToggleInput) -> UseToggleReturn {
    use_toggle_with(input, None, None)
}

/// [`use_toggle`] with an input `role` and, for checkbox group items, the validation to use
/// instead of the toggle's own (react-aria's `privateValidationStateProp`).
#[allow(clippy::too_many_lines)]
pub(crate) fn use_toggle_with(
    input: UseToggleInput,
    role: Option<AriaRole>,
    group_validation: Option<FormValidationState>,
) -> UseToggleReturn {
    let UseToggleInput { state, options } = input;
    let ToggleOptions {
        id,
        is_disabled,
        is_read_only,
        is_required,
        is_invalid,
        validate,
        validation_behavior,
        name,
        form,
        value,
        aria_label,
        aria_labelledby,
        aria_describedby,
        aria_errormessage,
        aria_controls,
        auto_focus,
        exclude_from_tab_order,
        on_focus,
        on_blur,
        on_focus_change,
        on_press_start,
        on_press_end,
        on_press_up,
        on_press,
        on_press_change,
    } = options;

    let focusable = use_focusable(UseFocusableInput {
        is_disabled,
        auto_focus,
        exclude_from_tab_order,
        on_focus,
        on_blur,
        on_focus_change,
        ..UseFocusableInput::default()
    });
    let element = focusable.focus_handle.element();
    let focus_handle = focusable.focus_handle;
    let mut focusable_props = focusable.props;
    let focus_ring = use_focus_ring(UseFocusRingInput {
        is_disabled,
        within: false,
        auto_focus,
        is_text_input: false,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });
    focusable_props.on_focus = focusable_props.on_focus.chain(focus_ring.props.on_focus);
    focusable_props.on_blur = focusable_props.on_blur.chain(focus_ring.props.on_blur);

    let validation_behavior = validation_behavior.unwrap_or_default();
    let validation = group_validation.unwrap_or_else(|| {
        use_form_validation_state(UseFormValidationStateInput {
            builtin_validation: Signal::default(),
            is_invalid,
            value: state.is_selected,
            validate,
            validation_behavior,
            name: name.clone(),
        })
    });
    use_form_validation(UseFormValidationInput {
        element,
        state: validation,
        validation_behavior,
        focus: None,
    });
    use_form_reset(UseFormResetInput {
        element,
        initial_value: state.default_selected,
        on_reset: Callback::new(move |selected| state.set_selected(selected)),
    });

    // The input itself: pressing it toggles natively (the `change` event).
    let press = use_press(UsePressInput {
        is_disabled,
        on_press,
        on_press_up,
        on_press_start,
        on_press_end,
        on_press_change,
        ..UsePressInput::default()
    });
    let (press_props, press_styles) = press.props.into_inner();

    // The label: mouse, touch and virtual presses (`label.click()`, assistive technology)
    // toggle the input and focus it; its click must not reach the input as well. Keyboard
    // presses happen on the input itself.
    let (is_label_pressed, set_label_pressed) = signal(false);
    let is_native_press = |e: &PressEvent| e.pointer_type == PointerType::Keyboard;
    let label_press = use_press(UsePressInput {
        is_disabled: Signal::derive(move || is_disabled.get() || is_read_only.get()),
        on_press_start: Some(Callback::new(move |e: PressEvent| {
            if is_native_press(&e) {
                e.continue_propagation();
                return;
            }
            if let Some(on_press_start) = on_press_start {
                on_press_start.run(e);
            }
            if let Some(on_press_change) = on_press_change {
                on_press_change.run(true);
            }
            set_label_pressed.set(true);
        })),
        on_press_end: Some(Callback::new(move |e: PressEvent| {
            if is_native_press(&e) {
                e.continue_propagation();
                return;
            }
            if let Some(on_press_end) = on_press_end {
                on_press_end.run(e);
            }
            if let Some(on_press_change) = on_press_change {
                on_press_change.run(false);
            }
            set_label_pressed.set(false);
        })),
        on_press_up: Some(Callback::new(move |e: PressEvent| {
            if is_native_press(&e) {
                e.continue_propagation();
                return;
            }
            if let Some(on_press_up) = on_press_up {
                on_press_up.run(e);
            }
        })),
        on_press: Some(Callback::new(move |e: PressEvent| {
            if is_native_press(&e) {
                e.continue_propagation();
                return;
            }
            if let Some(on_press) = on_press {
                on_press.run(e);
            }
            state.toggle();
            focus_handle.focus();
            validation.commit_validation();
        })),
        ..UsePressInput::default()
    });
    let (label_press_props, label_press_styles) = label_press.props.into_inner();

    let on_change = EventHandler::new(move |e: Event| {
        e.stop_propagation();
        let Some(input) = e
            .expect_target()
            .dyn_into::<web_sys::HtmlInputElement>()
            .ok()
        else {
            return;
        };
        if !is_read_only.get_untracked() {
            state.set_selected(input.checked());
        }
        // The change may have been rejected (read-only): keep the DOM in sync.
        input.set_checked(state.is_selected.get_untracked());
    });

    let description = use_slot("description");
    let error_message = use_slot("error-message");
    let user_describedby = Signal::stored(aria_describedby);
    let aria_describedby = join_slot_ids(&[
        description.referenced_id,
        error_message.referenced_id,
        user_describedby,
        focusable_props.context_aria_describedby,
    ]);

    let is_validation_invalid = validation.is_invalid;
    let validation_details =
        Signal::derive(move || validation.display_validation.get().validation_details);
    let native_required = validation_behavior == ValidationBehavior::Native;

    UseToggleReturn {
        label_props: PropsWithStyles::new(
            UseToggleLabelProps {
                on_keydown: label_press_props.on_keydown,
                // The label must not forward its click (the press toggled already).
                on_click: label_press_props
                    .on_click
                    .chain(EventHandler::new(|e: MouseEvent| e.prevent_default())),
                on_pointerdown: label_press_props.on_pointerdown,
                on_pointerup: label_press_props.on_pointerup,
                on_mousedown: label_press_props.on_mousedown,
                on_dragstart: label_press_props.on_dragstart,
                on_dblclick: label_press_props.on_dblclick,
            },
            label_press_styles,
        ),
        input_props: PropsWithStyles::new(
            UseToggleInputProps {
                role,
                id,
                name,
                form,
                value,
                checked: state.is_selected,
                default_checked: state.default_selected,
                indeterminate: Signal::stored(false),
                disabled: is_disabled,
                required: Signal::derive(move || native_required && is_required.get()),
                tabindex: focusable_props.tabindex,
                aria_required: Signal::derive(move || {
                    (!native_required && is_required.get()).then_some(AriaRequired::True)
                }),
                aria_invalid: Signal::derive(move || {
                    is_validation_invalid.get().then_some(AriaInvalid::True)
                }),
                aria_readonly: Signal::derive(move || {
                    is_read_only.get().then_some(AriaReadonly::True)
                }),
                aria_errormessage,
                aria_controls,
                aria_describedby,
                aria_label,
                aria_labelledby,
                element_capture: focusable_props.element_capture,
                on_change,
                on_focus: focusable_props.on_focus,
                on_blur: focusable_props.on_blur,
                on_focusin: focus_ring.props.on_focusin,
                on_focusout: focus_ring.props.on_focusout,
                on_keydown: press_props.on_keydown.chain(focusable_props.on_keydown),
                on_keyup: focusable_props.on_keyup,
                on_click: press_props.on_click,
                on_pointerdown: press_props.on_pointerdown,
                on_pointerup: press_props.on_pointerup,
                on_mousedown: press_props.on_mousedown,
                on_dragstart: press_props.on_dragstart,
                on_dblclick: press_props.on_dblclick,
                context_attrs: focusable_props.context_attrs.clone(),
            },
            press_styles,
        ),
        description_props: description.props,
        error_message_props: error_message.props,
        is_selected: state.is_selected,
        is_pressed: Signal::derive(move || press.is_pressed.get() || is_label_pressed.get()),
        is_disabled,
        is_read_only,
        is_invalid: is_validation_invalid,
        is_focused: focus_ring.is_focused,
        is_focus_visible: focus_ring.is_focus_visible,
        validation_errors: validation.validation_errors,
        validation_details,
    }
}

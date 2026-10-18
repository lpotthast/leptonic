// Upstream: react-aria/src/toggle/useToggle.ts @ 99e6102368
// Upstream: react-aria-components/test/Checkbox.test.js @ 99e6102368
use leptos::{
    ev,
    oco::Oco,
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
    ElementCaptureAttr, EventHandler, Id, IdRefs, IntoAttrs, OnEvent, Part, PropsWithStyles,
    SlotProps,
    hooks::{
        focus::{
            FocusRingTarget, FocusableContextAttr, FocusableContextAttrs, UseFocusRingInput,
            UseFocusableInput, use_focus_ring, use_focusable,
        },
        interactions::{PressEvent, UsePressInput, use_press},
    },
    sharing_one_effect, use_slot,
    utils::{
        aria::{AriaInvalid, AriaReadonly, AriaRequired, AriaRole, aria_true},
        dom_ext::EventAccessors,
        pointer_type::PointerType,
        probe::either,
        propagation_control::Propagation,
        signal_attr::{FixedAttr, SignalAttr, fixed_attr, maybe_attr, signal_attr},
        spread::Spread,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `role: ToggleRole` (the input's role: checkbox or switch). React-aria's `useToggle` has none:
//   `useSwitch` adds `role: 'switch'` to its returned `inputProps`.
// - `validation: Option<FormValidationState>`, the validation of a checkbox group's item, is an
//   input field (react-aria: the private `privateValidationStateProp` prop).
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
// - `is_read_only` also blocks changes here (react-aria: only in `useToggleState`), so a toggle
//   is read-only also with a state created without `is_read_only`.
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
    pub id: Option<Id>,
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
    pub aria_labelledby: Signal<Option<Id>>,
    pub aria_describedby: Signal<Option<Id>>,
    pub aria_errormessage: Signal<Option<Id>>,
    /// The element the toggle controls.
    pub aria_controls: Signal<Option<Id>>,
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
            aria_labelledby: Signal::default(),
            aria_describedby: Signal::default(),
            aria_errormessage: Signal::default(),
            aria_controls: Signal::default(),
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

/// The role of a toggle's `<input type="checkbox">`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ToggleRole {
    /// A checkbox: the input's native role (no `role` attribute).
    #[default]
    Checkbox,
    /// A switch (`role="switch"`): on or off, like a light switch.
    Switch,
}

impl ToggleRole {
    /// The `role` attribute: none for the native checkbox role.
    fn aria_role(self) -> Option<AriaRole> {
        match self {
            Self::Checkbox => None,
            Self::Switch => Some(AriaRole::Switch),
        }
    }
}

/// Input of [`use_toggle`].
#[derive(Debug, Clone)]
pub struct UseToggleInput {
    pub state: ToggleState,
    /// The input's role. [`use_switch`](fn@super::use_switch) sets `Switch`. Default:
    /// `Checkbox`.
    pub role: ToggleRole,
    /// A validation managed by someone else, used instead of the toggle's own (which
    /// `options.is_invalid`, `options.validate` and the toggle's name feed): a checkbox group
    /// item's, which shows the group's errors (see
    /// [`use_checkbox_group_item`](fn@super::use_checkbox_group_item)). `None`: the toggle
    /// validates itself.
    pub validation: Option<FormValidationState>,
    pub options: ToggleOptions,
}

/// Output of [`use_toggle`] (and [`use_checkbox`](fn@super::use_checkbox),
/// [`use_switch`](fn@super::use_switch)).
#[derive(Debug)]
pub struct UseToggleReturn {
    /// The `<label>` wrapping the input: pressing it toggles the input.
    pub label: Part<PropsWithStyles<UseToggleLabelProps>>,
    /// The `<input type="checkbox">`.
    pub input: Part<PropsWithStyles<UseToggleInputProps>>,
    /// Props for the description element.
    pub description: Part<SlotProps>,
    /// Props for the error message element (render it only while invalid).
    pub error_message: Part<SlotProps>,
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
    OnEvent<ev::keydown>,
    OnEvent<ev::click>,
    OnEvent<ev::pointerdown>,
    OnEvent<ev::pointerup>,
    OnEvent<ev::mousedown>,
    OnEvent<ev::dragstart>,
    OnEvent<ev::dblclick>,
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
    pub id: Option<Id>,
    pub name: Option<Oco<'static, str>>,
    pub form: Option<Oco<'static, str>>,
    pub value: Option<Oco<'static, str>>,
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
    pub aria_errormessage: Signal<Option<Id>>,
    pub aria_controls: Signal<Option<Id>>,
    pub aria_describedby: Signal<Option<Id>>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<Id>>,
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
    Spread<(
        (
            FixedAttr,
            FixedAttr,
            FixedAttr,
            FixedAttr,
            FixedAttr,
            FixedAttr,
            FixedAttr,
            SignalAttr<bool>,
            SignalAttr<bool>,
            SignalAttr<Option<i32>>,
        ),
        (
            SignalAttr<Option<AriaRequired>>,
            SignalAttr<Option<AriaInvalid>>,
            SignalAttr<Option<AriaReadonly>>,
            SignalAttr<Option<Id>>,
            SignalAttr<Option<Id>>,
            SignalAttr<Option<Id>>,
            SignalAttr<Option<String>>,
            SignalAttr<Option<Id>>,
            ElementCaptureAttr,
        ),
    )>,
    Property<&'static str, Signal<bool>>,
    Property<&'static str, Signal<bool>>,
    (
        OnEvent<ev::change>,
        OnEvent<ev::focus>,
        OnEvent<ev::blur>,
        OnEvent<ev::focusin>,
        OnEvent<ev::focusout>,
        OnEvent<ev::keydown>,
        OnEvent<ev::keyup>,
        OnEvent<ev::click>,
        OnEvent<ev::pointerdown>,
        OnEvent<ev::pointerup>,
        OnEvent<ev::mousedown>,
        OnEvent<ev::dragstart>,
        OnEvent<ev::dblclick>,
    ),
    FocusableContextAttr,
);

impl IntoAttrs for UseToggleInputProps {
    type Attrs = UseToggleInputAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Spread((
                (
                    fixed_attr("type", "checkbox"),
                    fixed_attr("role", self.role),
                    fixed_attr("id", self.id),
                    fixed_attr("name", self.name),
                    fixed_attr("form", self.form),
                    fixed_attr("value", self.value),
                    fixed_attr("checked", self.default_checked),
                    signal_attr("disabled", self.disabled),
                    signal_attr("required", self.required),
                    signal_attr("tabindex", self.tabindex),
                ),
                (
                    signal_attr("aria-required", self.aria_required),
                    signal_attr("aria-invalid", self.aria_invalid),
                    signal_attr("aria-readonly", self.aria_readonly),
                    signal_attr("aria-errormessage", self.aria_errormessage),
                    signal_attr("aria-controls", self.aria_controls),
                    signal_attr("aria-describedby", self.aria_describedby),
                    maybe_attr("aria-label", self.aria_label),
                    signal_attr("aria-labelledby", self.aria_labelledby),
                    self.element_capture,
                ),
            )),
            prop("checked", self.checked),
            prop("indeterminate", self.indeterminate),
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

/// A text attribute value shared by every rendering (cloned without copying).
fn shared_text(text: String) -> Oco<'static, str> {
    Oco::Counted(std::sync::Arc::from(text))
}

/// Provides the behavior and accessibility of a toggle on an `<input type="checkbox">` (the
/// base of [`use_checkbox`](fn@super::use_checkbox) and [`use_switch`](fn@super::use_switch)).
pub fn use_toggle(input: UseToggleInput) -> UseToggleReturn {
    crate::metrics::subject!();
    // The presses of the input and the label and the focus ring react to the disabled state in
    // one effect.
    sharing_one_effect(|| toggle(input))
}

/// The body of [`use_toggle`].
fn toggle(input: UseToggleInput) -> UseToggleReturn {
    let UseToggleInput {
        state,
        role,
        validation: group_validation,
        options,
    } = input;
    let role = role.aria_role();
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
        // Every field named (`performance.md` R14).
        on_key_down: None,
        on_key_up: None,
        shortcuts: None,
        allow_shortcut_repeats: false,
        mark: None,
    });
    let mut focusable_props = focusable.target.into_props();
    let element = focusable.focus_handle.element();
    let focus_handle = focusable.focus_handle;
    let focus_ring = use_focus_ring(UseFocusRingInput {
        target: FocusRingTarget::Element,
        is_disabled,
        is_text_input: false,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });
    let focus_ring_props = focus_ring.target.into_props();
    focusable_props.on_focus = focusable_props.on_focus.chain(focus_ring_props.on_focus);
    focusable_props.on_blur = focusable_props.on_blur.chain(focus_ring_props.on_blur);

    let validation_behavior = validation_behavior.unwrap_or_default();
    let validation = group_validation.unwrap_or_else(|| {
        use_form_validation_state(UseFormValidationStateInput {
            builtin_validation: Signal::default(),
            is_invalid,
            value: state.is_selected,
            validate,
            validation_behavior,
            names: name.clone().into_iter().collect(),
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
        // Every field named (`performance.md` R14).
        propagation: crate::hooks::interactions::PressPropagation::Stop,
        on_double_press: None,
        long_press: None,
        prevent_focus_on_press: Signal::stored(false),
        allow_text_selection_on_press: Signal::stored(false),
        should_cancel_on_pointer_exit: Signal::stored(false),
        force_is_pressed: Signal::stored(false),
    });
    let (press_props, press_styles) = press.target.into_props().into_inner();

    // The label: mouse, touch and virtual presses (`label.click()`, assistive technology)
    // toggle the input and focus it; its click must not reach the input as well. Keyboard
    // presses happen on the input itself.
    let (is_label_pressed, set_label_pressed) = signal(false);
    let is_native_press = |e: &PressEvent| e.pointer_type == PointerType::Keyboard;
    let label_press = use_press(UsePressInput {
        is_disabled: either(is_disabled, is_read_only),
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
        propagation: crate::hooks::interactions::PressPropagation::Stop,
        allow_text_selection_on_press: Signal::stored(false),
        should_cancel_on_pointer_exit: Signal::stored(false),
        prevent_focus_on_press: Signal::stored(false),
        force_is_pressed: Signal::stored(false),
        on_press_change: None,
        on_double_press: None,
        long_press: None,
    });
    let (label_press_props, label_press_styles) = label_press.target.into_props().into_inner();

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
    let user_describedby = aria_describedby;
    let aria_describedby = IdRefs::derive([
        description.referenced_id,
        error_message.referenced_id,
        user_describedby,
        focusable_props.context_aria_describedby,
    ]);

    let is_validation_invalid = validation.is_invalid;
    let validation_details =
        Signal::derive(move || validation.display_validation.get().validation_details);
    let native_required = validation_behavior == ValidationBehavior::Native;

    // The label and the input are parts: an atom renders them in its button, which may mount
    // again (each mount builds fresh props, one element at a time).
    let label = {
        let on_keydown = label_press_props.on_keydown;
        // The label must not forward its click (the press toggled already).
        let on_click = label_press_props
            .on_click
            .chain(EventHandler::new(|e: MouseEvent| e.prevent_default()));
        let on_pointerdown = label_press_props.on_pointerdown;
        let on_pointerup = label_press_props.on_pointerup;
        let on_mousedown = label_press_props.on_mousedown;
        let on_dragstart = label_press_props.on_dragstart;
        let on_dblclick = label_press_props.on_dblclick;
        Part::new(move || {
            PropsWithStyles::new(
                UseToggleLabelProps {
                    on_keydown: on_keydown.clone(),
                    on_click: on_click.clone(),
                    on_pointerdown: on_pointerdown.clone(),
                    on_pointerup: on_pointerup.clone(),
                    on_mousedown: on_mousedown.clone(),
                    on_dragstart: on_dragstart.clone(),
                    on_dblclick: on_dblclick.clone(),
                },
                label_press_styles.clone(),
            )
        })
    };
    let input = {
        let (checked, default_checked) = (state.is_selected, state.default_selected);
        let required = Signal::derive(move || native_required && is_required.get());
        let aria_required = Signal::derive(move || {
            (!native_required && is_required.get()).then_some(AriaRequired::True)
        });
        let aria_invalid = aria_true(is_validation_invalid);
        let aria_readonly = aria_true(is_read_only);
        let tabindex = focusable_props.tabindex;
        let element_capture = focusable_props.element_capture;
        let on_focus = focusable_props.on_focus;
        let on_blur = focusable_props.on_blur;
        let on_focusin = focus_ring_props.on_focusin;
        let on_focusout = focus_ring_props.on_focusout;
        let on_keydown = press_props.on_keydown.chain(focusable_props.on_keydown);
        let on_keyup = focusable_props.on_keyup;
        let on_click = press_props.on_click;
        let on_pointerdown = press_props.on_pointerdown;
        let on_pointerup = press_props.on_pointerup;
        let on_mousedown = press_props.on_mousedown;
        let on_dragstart = press_props.on_dragstart;
        let on_dblclick = press_props.on_dblclick;
        let context_attrs = focusable_props.context_attrs;
        // Shared by every mount: one copy of the texts, one stored flag.
        let name = name.map(shared_text);
        let form = form.map(shared_text);
        let value = value.map(shared_text);
        let indeterminate = Signal::stored(false);
        Part::new(move || {
            PropsWithStyles::new(
                UseToggleInputProps {
                    role,
                    id: id.clone(),
                    name: name.clone(),
                    form: form.clone(),
                    value: value.clone(),
                    checked,
                    default_checked,
                    indeterminate,
                    disabled: is_disabled,
                    required,
                    tabindex,
                    aria_required,
                    aria_invalid,
                    aria_readonly,
                    aria_errormessage,
                    aria_controls,
                    aria_describedby,
                    aria_label,
                    aria_labelledby,
                    element_capture: element_capture.clone(),
                    on_change: on_change.clone(),
                    on_focus: on_focus.clone(),
                    on_blur: on_blur.clone(),
                    on_focusin: on_focusin.clone(),
                    on_focusout: on_focusout.clone(),
                    on_keydown: on_keydown.clone(),
                    on_keyup: on_keyup.clone(),
                    on_click: on_click.clone(),
                    on_pointerdown: on_pointerdown.clone(),
                    on_pointerup: on_pointerup.clone(),
                    on_mousedown: on_mousedown.clone(),
                    on_dragstart: on_dragstart.clone(),
                    on_dblclick: on_dblclick.clone(),
                    context_attrs: context_attrs.clone(),
                },
                press_styles.clone(),
            )
        })
    };

    UseToggleReturn {
        label,
        input,
        description: description.part,
        error_message: error_message.part,
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

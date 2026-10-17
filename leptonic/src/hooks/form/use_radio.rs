// Upstream: react-aria/src/radio/useRadio.ts @ 99e6102368
// Upstream: react-aria-components/test/RadioGroup.test.js @ 99e6102368
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
    use_form_validation_state::ValidationBehavior,
    use_radio_group::RadioGroupData,
    use_toggle::UseToggleLabelProps,
};
use crate::{
    ElementCaptureAttr, EventHandler, Id, IdRefs, IntoAttrs, OnEvent, Part, PropsWithStyles,
    SlotProps,
    hooks::{
        collections::Key,
        focus::{
            FocusRingTarget, FocusableContextAttr, FocusableContextAttrs, UseFocusRingInput,
            UseFocusableInput, use_focus_ring, use_focusable,
        },
        interactions::{PressEvent, PressPropagation, UsePressInput, use_press},
    },
    use_slot,
    utils::{
        probe::either,
        signal_attr::{FixedAttr, SignalAttr, fixed_attr, maybe_attr, signal_attr},
        spread::Spread,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `checked` is set as a DOM property (and the group's radios re-synced after a rejected
//   change, e.g. while read-only), as React does for controlled radio groups. The `checked`
//   attribute holds the initial selection (React's `defaultChecked`), which a native form reset
//   restores after the state's own reset.
//
// ## ADDITIONS
// - Focus ring state (`is_focused`, `is_focus_visible`), which react-aria-components adds with
//   `useFocusRing`.
//
// =============================================================================

/// Input of [`use_radio`].
#[derive(Debug, Clone)]
pub struct UseRadioInput {
    /// The radio group (from [`use_radio_group`](fn@super::use_radio_group)).
    pub group: RadioGroupData,
    /// The value the radio selects.
    pub value: Key,
    pub is_disabled: Signal<bool>,
    /// The input's id.
    pub id: Option<Id>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<Id>>,
    pub aria_describedby: Signal<Option<Id>>,
    pub auto_focus: bool,
    pub on_focus: Option<Callback<FocusEvent>>,
    pub on_blur: Option<Callback<FocusEvent>>,
    pub on_focus_change: Option<Callback<bool>>,
    pub on_press_start: Option<Callback<PressEvent>>,
    pub on_press_end: Option<Callback<PressEvent>>,
    pub on_press_up: Option<Callback<PressEvent>>,
    pub on_press: Option<Callback<PressEvent>>,
    pub on_press_change: Option<Callback<bool>>,
}

/// Output of [`use_radio`].
#[derive(Debug)]
pub struct UseRadioReturn {
    /// The `<label>` wrapping the input: pressing it selects the radio.
    pub label: Part<PropsWithStyles<UseToggleLabelProps>>,
    /// The `<input type="radio">`.
    pub input: Part<PropsWithStyles<UseRadioInputProps>>,
    /// Props for the radio's own description.
    pub description: Part<SlotProps>,
    pub is_selected: Signal<bool>,
    pub is_disabled: Signal<bool>,
    /// Whether the input or its label is pressed.
    pub is_pressed: Signal<bool>,
    pub is_focused: Signal<bool>,
    pub is_focus_visible: Signal<bool>,
}

/// Props for the radio's `<input type="radio">`.
#[derive(Debug)]
pub struct UseRadioInputProps {
    pub id: Option<Id>,
    pub name: Oco<'static, str>,
    pub form: Option<Oco<'static, str>>,
    pub value: Oco<'static, str>,
    pub checked: Signal<bool>,
    /// The `checked` attribute: the initial selection, which a form reset restores.
    pub default_checked: bool,
    pub disabled: Signal<bool>,
    pub required: Signal<bool>,
    pub tabindex: Signal<Option<i32>>,
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

pub type UseRadioInputAttrs = (
    Spread<(
        (
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
            SignalAttr<Option<Id>>,
            SignalAttr<Option<String>>,
            SignalAttr<Option<Id>>,
            ElementCaptureAttr,
        ),
    )>,
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

impl IntoAttrs for UseRadioInputProps {
    type Attrs = UseRadioInputAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Spread((
                (
                    fixed_attr("type", "radio"),
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
                    signal_attr("aria-describedby", self.aria_describedby),
                    maybe_attr("aria-label", self.aria_label),
                    signal_attr("aria-labelledby", self.aria_labelledby),
                    self.element_capture,
                ),
            )),
            prop("checked", self.checked),
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

/// Provides the behavior and accessibility of a radio: an `<input type="radio">` inside a
/// `<label>`, in a [`use_radio_group`](fn@super::use_radio_group). Only the selected radio (or,
/// while none is, the one focused last or each) is a tab stop.
pub fn use_radio(input: UseRadioInput) -> UseRadioReturn {
    crate::metrics::subject!();
    let UseRadioInput {
        group,
        value,
        is_disabled,
        id,
        aria_label,
        aria_labelledby,
        aria_describedby,
        auto_focus,
        on_focus,
        on_blur,
        on_focus_change,
        on_press_start,
        on_press_end,
        on_press_up,
        on_press,
        on_press_change,
    } = input;
    let state = group.state;
    let is_disabled = either(is_disabled, state.is_disabled);

    // Notified when this radio's selection or tab stop changes, not on every change of the
    // group's state (`documentation/performance.md`, R1).
    let changed = state.subscribe_radio(value.clone());
    let checked_value = value.clone();
    let selected_trigger = changed.clone();
    let is_selected = Signal::derive(move || {
        selected_trigger.track();
        state
            .selected_value
            .with_untracked(|selected| selected.as_ref() == Some(&checked_value))
    });

    let focused_value = value.clone();
    let focusable = use_focusable(UseFocusableInput {
        is_disabled,
        auto_focus,
        on_focus: Some(Callback::new(move |e: FocusEvent| {
            state.set_last_focused_value(Some(focused_value.clone()));
            if let Some(on_focus) = on_focus {
                on_focus.run(e);
            }
        })),
        on_blur,
        on_focus_change,
        // Every field named (`performance.md` R14).
        exclude_from_tab_order: Signal::stored(false),
        on_key_down: None,
        on_key_up: None,
        shortcuts: None,
        allow_shortcut_repeats: false,
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

    // Register with the group for keyboard navigation.
    let registered = value.clone();
    group
        .radios
        .update_value(|radios| radios.push((registered, element)));
    let unregistered = value.clone();
    on_cleanup(move || {
        group.radios.try_update_value(|radios| {
            radios.retain(|(key, _)| *key != unregistered);
        });
    });

    let press = use_press(UsePressInput {
        is_disabled,
        on_press,
        on_press_up,
        on_press_start,
        on_press_end,
        on_press_change,
        // Every field named (`performance.md` R14).
        propagation: PressPropagation::Stop,
        on_double_press: None,
        long_press: None,
        prevent_focus_on_press: Signal::stored(false),
        allow_text_selection_on_press: Signal::stored(false),
        should_cancel_on_pointer_exit: Signal::stored(false),
        force_is_pressed: Signal::stored(false),
    });
    let (press_props, press_styles) = press.target.into_props().into_inner();

    let pressed_value = value.clone();
    let label_press = use_press(UsePressInput {
        is_disabled,
        on_press_start,
        on_press_end,
        on_press_up,
        on_press: Some(Callback::new(move |e: PressEvent| {
            if let Some(on_press) = on_press {
                on_press.run(e);
            }
            state.set_selected_value(Some(pressed_value.clone()));
            focus_handle.focus();
        })),
        on_press_change,
        propagation: PressPropagation::Stop,
        on_double_press: None,
        long_press: None,
        prevent_focus_on_press: Signal::stored(false),
        allow_text_selection_on_press: Signal::stored(false),
        should_cancel_on_pointer_exit: Signal::stored(false),
        force_is_pressed: Signal::stored(false),
    });
    let (label_press_props, label_press_styles) = label_press.target.into_props().into_inner();

    let changed_value = value.clone();
    let on_change = EventHandler::new(move |e: Event| {
        e.stop_propagation();
        state.set_selected_value(Some(changed_value.clone()));
        // The state may have rejected the change (read-only). The browser already unchecked
        // the selected radio (radios of one name are exclusive): restore the whole group.
        if !is_selected.get_untracked() {
            let selected = state.selected_value.get_untracked();
            group.radios.with_value(|radios| {
                for (key, element) in radios {
                    if let Some(el) = element.get_untracked()
                        && let Some(input) = el.dyn_ref::<web_sys::HtmlInputElement>()
                    {
                        input.set_checked(selected.as_ref() == Some(key));
                    }
                }
            });
        }
    });

    // Roving tab stop: the selected radio, else the one focused last, else every radio.
    let tab_value = value.clone();
    let tabindex = Signal::derive(move || {
        changed.track();
        state.track_every_radio_tab_stop();
        if is_disabled.get() {
            return None;
        }
        let is_tab_stop = state.is_tab_stop_untracked(&tab_value);
        Some(if is_tab_stop { 0 } else { -1 })
    });

    let validation_behavior = state.validation_behavior;
    use_form_reset(UseFormResetInput {
        element,
        initial_value: state.default_selected_value(),
        on_reset: Callback::new(move |value| state.set_selected_value(value)),
    });
    use_form_validation(UseFormValidationInput {
        element,
        state: state.validation,
        validation_behavior,
        focus: None,
    });

    let description = use_slot("description");
    let group_error = Signal::derive(move || {
        if state.is_invalid.get() {
            group.error_message_id.get()
        } else {
            None
        }
    });
    let aria_describedby = IdRefs::derive([
        aria_describedby,
        description.referenced_id,
        group_error,
        group.description_id,
        focusable_props.context_aria_describedby,
    ]);
    let is_required = state.is_required;
    let native_required = validation_behavior == ValidationBehavior::Native;

    // The label and the input are parts: the atom renders them in its button, which may mount
    // again (each mount builds fresh props, one element at a time).
    let label = {
        let on_keydown = label_press_props.on_keydown;
        // The label must not forward its click (the press selected already).
        let on_click = label_press_props
            .on_click
            .chain(EventHandler::new(|e: MouseEvent| e.prevent_default()));
        let on_pointerdown = label_press_props.on_pointerdown;
        let on_pointerup = label_press_props.on_pointerup;
        let on_mousedown = label_press_props
            .on_mousedown
            .chain(EventHandler::new(|e: MouseEvent| e.prevent_default()));
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
        // Shared by every mount (cloned without copying).
        let name: Oco<'static, str> = Oco::Counted(std::sync::Arc::from(state.name()));
        let form = group
            .form
            .get_value()
            .map(|form| Oco::Counted(std::sync::Arc::from(form)));
        let value_text: Oco<'static, str> = Oco::Counted(std::sync::Arc::from(value.to_string()));
        let default_checked = state.default_selected_value().as_ref() == Some(&value);
        let required = Signal::derive(move || native_required && is_required.get());
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
        Part::new(move || {
            PropsWithStyles::new(
                UseRadioInputProps {
                    id: id.clone(),
                    name: name.clone(),
                    form: form.clone(),
                    value: value_text.clone(),
                    checked: is_selected,
                    default_checked,
                    disabled: is_disabled,
                    required,
                    tabindex,
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

    UseRadioReturn {
        label,
        input,
        description: description.part,
        is_selected,
        is_disabled,
        is_pressed: Signal::derive(move || press.is_pressed.get() || label_press.is_pressed.get()),
        is_focused: focus_ring.is_focused,
        is_focus_visible: focus_ring.is_focus_visible,
    }
}

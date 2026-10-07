// Upstream: react-aria/src/radio/useRadio.ts @ 99e6102368
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
    use_form_validation_state::ValidationBehavior,
    use_radio_group::RadioGroupData,
    use_toggle::UseToggleLabelProps,
};
use crate::{
    hooks::{
        FocusableContextAttr, FocusableContextAttrs, IntoAttrs, PressEvent, PropsWithStyles,
        UseFocusRingInput, UseFocusableInput, UsePressInput, collections::Key, use_focus_ring,
        use_focusable, use_press,
    },
    utils::{ElementCaptureAttr, EventHandler, SlotProps, join_slot_ids, use_slot},
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
    /// The radio group (from [`use_radio_group`](super::use_radio_group)).
    pub group: RadioGroupData,
    /// The value the radio selects.
    pub value: Key,
    pub is_disabled: Signal<bool>,
    /// The input's id.
    pub id: Option<String>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Option<String>,
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
    /// Props for the `<label>` wrapping the input: pressing it selects the radio.
    pub label_props: PropsWithStyles<UseToggleLabelProps>,
    /// Props for the `<input type="radio">`.
    pub input_props: PropsWithStyles<UseRadioInputProps>,
    /// Props for the radio's own description.
    pub description_props: SlotProps,
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
    pub id: Option<String>,
    pub name: String,
    pub form: Option<String>,
    pub value: String,
    pub checked: Signal<bool>,
    /// The `checked` attribute: the initial selection, which a form reset restores.
    pub default_checked: bool,
    pub disabled: Signal<bool>,
    pub required: Signal<bool>,
    pub tabindex: Signal<Option<i32>>,
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

pub type UseRadioInputAttrs = (
    (
        Attr<attr::Type, &'static str>,
        Attr<attr::Id, Option<String>>,
        Attr<attr::Name, String>,
        Attr<attr::Form, Option<String>>,
        Attr<attr::Value, String>,
        Property<&'static str, Signal<bool>>,
        Attr<attr::Checked, bool>,
        Attr<attr::Disabled, Signal<bool>>,
        Attr<attr::Required, Signal<bool>>,
        Attr<attr::Tabindex, Signal<Option<i32>>>,
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

impl IntoAttrs for UseRadioInputProps {
    type Attrs = UseRadioInputAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            (
                Attr(attr::Type, "radio"),
                Attr(attr::Id, self.id),
                Attr(attr::Name, self.name),
                Attr(attr::Form, self.form),
                Attr(attr::Value, self.value),
                prop("checked", self.checked),
                Attr(attr::Checked, self.default_checked),
                Attr(attr::Disabled, self.disabled),
                Attr(attr::Required, self.required),
                Attr(attr::Tabindex, self.tabindex),
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

/// Provides the behavior and accessibility of a radio: an `<input type="radio">` inside a
/// `<label>`, in a [`use_radio_group`](super::use_radio_group). Only the selected radio (or,
/// while none is, the one focused last or each) is a tab stop.
#[allow(clippy::too_many_lines)]
pub fn use_radio(input: UseRadioInput) -> UseRadioReturn {
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
    let is_disabled = Signal::derive(move || is_disabled.get() || state.is_disabled.get());

    let checked_value = value.clone();
    let is_selected = Signal::derive(move || {
        state
            .selected_value
            .with(|selected| selected.as_ref() == Some(&checked_value))
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
        ..UsePressInput::default()
    });
    let (press_props, press_styles) = press.props.into_inner();

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
        ..UsePressInput::default()
    });
    let (label_press_props, label_press_styles) = label_press.props.into_inner();

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
                    if let Some(input) = element
                        .get_untracked()
                        .and_then(|el| (*el).clone().dyn_into::<web_sys::HtmlInputElement>().ok())
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
        if is_disabled.get() {
            return None;
        }
        let selected = state.selected_value.get();
        let last_focused = state.last_focused_value.get();
        let is_tab_stop = match selected {
            Some(selected) => selected == tab_value,
            None => last_focused.is_none_or(|last| last == tab_value),
        };
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
    let aria_describedby = join_slot_ids(&[
        Signal::stored(aria_describedby),
        description.referenced_id,
        group_error,
        group.description_id,
        focusable_props.context_aria_describedby,
    ]);
    let is_required = state.is_required;
    let native_required = validation_behavior == ValidationBehavior::Native;

    UseRadioReturn {
        label_props: PropsWithStyles::new(
            UseToggleLabelProps {
                on_keydown: label_press_props.on_keydown,
                // The label must not forward its click (the press selected already).
                on_click: label_press_props
                    .on_click
                    .chain(EventHandler::new(|e: MouseEvent| e.prevent_default())),
                on_pointerdown: label_press_props.on_pointerdown,
                on_pointerup: label_press_props.on_pointerup,
                on_mousedown: label_press_props
                    .on_mousedown
                    .chain(EventHandler::new(|e: MouseEvent| e.prevent_default())),
                on_dragstart: label_press_props.on_dragstart,
                on_dblclick: label_press_props.on_dblclick,
            },
            label_press_styles,
        ),
        input_props: PropsWithStyles::new(
            UseRadioInputProps {
                id,
                name: state.name(),
                form: group.form.get_value(),
                value: value.to_string(),
                checked: is_selected,
                default_checked: state.default_selected_value().as_ref() == Some(&value),
                disabled: is_disabled,
                required: Signal::derive(move || native_required && is_required.get()),
                tabindex,
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
        is_selected,
        is_disabled,
        is_pressed: Signal::derive(move || press.is_pressed.get() || label_press.is_pressed.get()),
        is_focused: focus_ring.is_focused,
        is_focus_visible: focus_ring.is_focus_visible,
    }
}

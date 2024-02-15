use std::cell::RefCell;

use educe::Educe;
use leptos_reactive::{
    create_signal, store_value, Callable, Callback, MaybeSignal, Signal, SignalGetUntracked,
};
use wasm_bindgen::JsCast;
use web_sys::{KeyboardEvent, MouseEvent, PointerEvent, TouchEvent};

use crate::utils::{current_target_contains_target, props::Attributes, EventExt, EventTargetExt};

use super::{disable_text_selection, restore_text_selection, KeyModifiers, Modifiers};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/interactions/src/usePress.ts

#[derive(Debug, Clone)]
pub enum PointerType {
    Mouse,
    Pen,
    Touch,
    Keyboard,
    Virtual,
    Other(String),
}

impl From<String> for PointerType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "mouse" => PointerType::Mouse,
            "touch" => PointerType::Touch,
            "pen" => PointerType::Pen,
            _other => PointerType::Other(value),
        }
    }
}

#[derive(Debug)]
pub enum PressEvents {
    PressStart(PressStartEvent),
    PressEnd(PressEndEvent),
    PressUp(PressUpEvent),
    Press(PressEvent),
}

#[derive(Educe)]
#[educe(Debug)]
pub struct PressStartEvent {
    /// The pointer type that triggered the press event.
    pointer_type: PointerType,

    /// The target element of the press event.
    target: Option<web_sys::EventTarget>,

    /// Sates which modifier keys were held during the press event.
    modifiers: KeyModifiers,

    /// By default, press events stop propagation to parent elements.
    /// In cases where a handler decides not to handle a specific event,
    /// it can call `continuePropagation()` to allow a parent to handle it.
    #[educe(Debug(ignore))]
    continue_propagation: Box<dyn FnOnce()>,
}

#[derive(Educe)]
#[educe(Debug)]
pub struct PressEndEvent {
    /// The pointer type that triggered the press event.
    pointer_type: PointerType,

    /// The target element of the press event.
    target: Option<web_sys::EventTarget>,

    /// Sates which modifier keys were held during the press event.
    modifiers: KeyModifiers,

    /// By default, press events stop propagation to parent elements.
    /// In cases where a handler decides not to handle a specific event,
    /// it can call `continuePropagation()` to allow a parent to handle it.
    #[educe(Debug(ignore))]
    continue_propagation: Box<dyn FnOnce()>,
}

#[derive(Debug)]
pub struct PressUpEvent {}

#[derive(Educe)]
#[educe(Debug)]
pub struct PressEvent {
    /// The pointer type that triggered the press event.
    pointer_type: PointerType,

    /// The target element of the press event.
    target: Option<web_sys::EventTarget>,

    /// Sates which modifier keys were held during the press event.
    modifiers: KeyModifiers,

    /// By default, press events stop propagation to parent elements.
    /// In cases where a handler decides not to handle a specific event,
    /// it can call `continuePropagation()` to allow a parent to handle it.
    #[educe(Debug(ignore))]
    continue_propagation: Box<dyn FnOnce()>,
}

#[derive(Debug, Clone, Copy)]
pub struct UsePressInput {
    /// Wether the targeted element is currently disabled.
    pub disabled: MaybeSignal<bool>,

    pub on_press: Callback<PressEvent>,
    pub on_press_up: Option<Callback<PressUpEvent>>,
    pub on_press_start: Option<Callback<PressStartEvent>>,
    pub on_press_end: Option<Callback<PressEndEvent>>,
}

impl UsePressInput {
    pub fn send_on_press_start(
        &self,
        event_builder: impl Fn(Box<dyn FnOnce()>) -> PressStartEvent,
    ) -> bool {
        if let Some(on_press_start) = self.on_press_start {
            let mut continue_propagation_state = RefCell::new(false);
            let state_clone = continue_propagation_state.clone();
            let continue_propagation = Box::new(move || {
                state_clone.replace(false);
            });
            on_press_start.call(event_builder(continue_propagation));
            continue_propagation_state.into_inner()
        } else {
            true
        }
    }

    pub fn send_on_press_end(
        &self,
        event_builder: impl Fn(Box<dyn FnOnce()>) -> PressEndEvent,
    ) -> bool {
        if let Some(on_press_end) = self.on_press_end {
            let mut continue_propagation_state = RefCell::new(false);
            let state_clone = continue_propagation_state.clone();
            let continue_propagation = Box::new(move || {
                state_clone.replace(false);
            });
            on_press_end.call(event_builder(continue_propagation));
            continue_propagation_state.into_inner()
        } else {
            true
        }
    }

    pub fn send_on_press(&self, event_builder: impl Fn(Box<dyn FnOnce()>) -> PressEvent) -> bool {
        let mut continue_propagation_state = RefCell::new(false);
        let state_clone = continue_propagation_state.clone();
        let continue_propagation = Box::new(move || {
            state_clone.replace(false);
        });
        self.on_press.call(event_builder(continue_propagation));
        continue_propagation_state.into_inner()
    }
}

#[derive(Educe)]
#[educe(Debug)]
pub struct UsePressProps {
    /// These attributes must be spread onto the target element: `<foo {..attrs} />`
    pub attrs: Attributes,

    #[educe(Debug(ignore))]
    pub on_key_down: Box<dyn Fn(KeyboardEvent)>,
    /// This handler must be attached to the target element: `<foo on:click=on_click />`
    #[educe(Debug(ignore))]
    pub on_click: Box<dyn Fn(MouseEvent)>,

    #[educe(Debug(ignore))]
    pub on_pointer_down: Box<dyn Fn(PointerEvent)>,
    #[educe(Debug(ignore))]
    pub on_pointer_enter: Box<dyn Fn(PointerEvent)>,
    #[educe(Debug(ignore))]
    pub on_pointer_leave: Box<dyn Fn(PointerEvent)>,
    #[educe(Debug(ignore))]
    pub on_pointer_up: Box<dyn Fn(PointerEvent)>,
}

#[derive(Debug)]
pub struct UsePressReturn {
    pub props: UsePressProps,
    pub is_pressed: Signal<bool>,
}

struct PressState {
    pointer_type: PointerType,
    target: web_sys::EventTarget,
}

pub fn use_press(input: UsePressInput) -> UsePressReturn {
    let attrs = Attributes::new();

    let (is_pressed, set_is_pressed) = create_signal(false);

    let state = store_value::<Option<PressState>>(None);

    let on_key_down = Box::new(move |e: KeyboardEvent| {
        if !current_target_contains_target(e.current_target().as_ref(), e.target().as_ref())
            .unwrap_or(true)
        {
            tracing::debug!("Aborting on_key_down, as current_target did not contain target.");
            return;
        }

        let key = e.key();

        if e.target()
            .and_then(|t| t.as_element())
            .map(|t| should_prevent_default_keyboard(t, &key))
            .unwrap_or(false)
        {
            e.prevent_default();
        }

        let is_link = e
            .current_target()
            .and_then(|t| t.as_html_element())
            .and_then(|el| el.get_attribute("role"))
            .map_or(false, |r| r == "link");

        let resembles_press = match is_link {
            true => match key.as_str() {
                "Enter" => true,
                _ => false,
            },
            false => match key.as_str() {
                "Enter" | "Spacebar" | " " => true,
                _ => false,
            },
        };

        if resembles_press {
            let mut continue_propagation_state = RefCell::new(false);
            let state_clone = continue_propagation_state.clone();
            let continue_propagation = Box::new(move || {
                state_clone.replace(false);
            });
            input.on_press.call(PressEvent {
                pointer_type: PointerType::Keyboard,
                target: e.target(),
                modifiers: e.modifiers(),
                continue_propagation,
            });
            if !continue_propagation_state.into_inner() {
                e.stop_propagation();
            }
        }
    });

    let on_click = Box::new(move |e: MouseEvent| {
        if !current_target_contains_target(e.current_target().as_ref(), e.target().as_ref())
            .unwrap_or(true)
        {
            tracing::debug!("Aborting on_click, as current_target did not contain target.");
            return;
        }

        if input.disabled.get_untracked() {
            e.prevent_default();
        }
        e.stop_propagation();
    });

    // Start a press.
    let on_pointer_down = Box::new(move |e: PointerEvent| {
        if e.button() != 0 {
            return;
        }

        if !e.current_target_contains_target() {
            tracing::trace!("Aborting on_click, as current_target did not contain target.");
            return;
        }

        // TODO: Special handling of virtual pointer events.

        // TODO: Maybe prevent default and handle element focus manually.
        //e.prevent_default();

        let pointer_type = PointerType::from(e.pointer_type());

        if let Some(target) = e.target() {
            if let Some(target) = target.as_element() {
                disable_text_selection(target);
            }
        }

        let mut continue_propagation_state = RefCell::new(false);
        let state_clone = continue_propagation_state.clone();
        let continue_propagation = Box::new(move || {
            state_clone.replace(false);
        });
        if let Some(on_press_start) = input.on_press_start {
            on_press_start.call(PressStartEvent {
                pointer_type: PointerType::Touch,
                target: e.target(),
                modifiers: e.modifiers(),
                continue_propagation,
            });
        }
        if !continue_propagation_state.into_inner() {
            e.stop_propagation();
        }
    });

    // Reset press state.
    let on_pointer_enter = Box::new(move |e: PointerEvent| {
        // if e.pointer_id() !==
        tracing::info!("pointer enter");
        if state.with_value(|s| s.is_some()) {

            state.with_value(|s| {
                let s = s.expect("present");
                input.send_on_press_start(move || PressStartEvent {
                    pointer_type: s.pointer_type.clone(),
                    target: todo!(),
                    modifiers: todo!(),
                    continue_propagation: todo!(),
                })
            });
        }
    });

    // Cancel the ongoing press.
    let on_pointer_leave = Box::new(move |e: PointerEvent| {
        tracing::info!("pointer leave");

        let ec = e.clone();
        let continue_propagation =
            input.send_on_press_end(move |continue_propagation| PressEndEvent {
                pointer_type: PointerType::Touch,
                target: ec.target(),
                modifiers: ec.modifiers(),
                continue_propagation,
            });
        if !continue_propagation {
            e.stop_propagation();
        }
    });

    // Finish a press.
    let on_pointer_up = Box::new(move |e: PointerEvent| {
        tracing::info!("pointer up");

        let ty = e.pointer_type();

        let ec = e.clone();
        let continue_propagation =
            input.send_on_press_end(move |continue_propagation| PressEndEvent {
                pointer_type: PointerType::Touch,
                target: ec.target(),
                modifiers: ec.modifiers(),
                continue_propagation,
            });
        if !continue_propagation {
            e.stop_propagation();
        }

        if !e.current_target_contains_target() {
            tracing::debug!("Aborting on_touchend, as current_target did not contain target.");
            return;
        }
        e.stop_propagation();

        let ec = e.clone();
        let continue_propagation = input.send_on_press(move |continue_propagation| PressEvent {
            pointer_type: PointerType::Touch,
            target: ec.target(),
            modifiers: ec.modifiers(),
            continue_propagation,
        });
        if !continue_propagation {
            e.stop_propagation();
        }

        if let Some(target) = e.target() {
            if let Some(target) = target.as_element() {
                restore_text_selection(target);
            }
        }
    });

    UsePressReturn {
        props: UsePressProps {
            attrs,
            on_key_down,
            on_click,
            on_pointer_down,
            on_pointer_enter,
            on_pointer_leave,
            on_pointer_up,
        },
        is_pressed: is_pressed.into(),
    }
}

// TODO: This should also work for pointer events!
fn is_virtual_mouse_event(event: &web_sys::MouseEvent) -> bool {
    let page_x = event.page_x();
    let page_y = event.page_y();
    let buttons = event.buttons();
    let detail = event.detail();
    let is_trusted = event.is_trusted();

    tracing::info!(page_x, page_y, buttons, detail, is_trusted);

    !is_trusted
}

// TODO: This should also work for pointer events!
fn is_virtual_pointer_event(event: &web_sys::PointerEvent) -> bool {
    let pointer_type = event.pointer_type();
    let buttons = event.buttons();
    let detail = event.detail();
    let is_trusted = event.is_trusted();

    tracing::info!(pointer_type, buttons, detail, is_trusted);

    !is_trusted
}

/// Tests for `<a href=[...]>`
fn is_html_anchor(element: web_sys::Element) -> bool {
    element.tag_name().as_str() == "A" && element.has_attribute("href")
}

/// Tests whether a keyboard event's default action should be presented when the given `key` was pressed.
fn should_prevent_default_keyboard(element: web_sys::Element, key: &str) -> bool {
    if element.is_instance_of::<web_sys::HtmlInputElement>() {
        return !is_valid_input_key(element.unchecked_into::<web_sys::HtmlInputElement>(), key);
    }

    if element.is_instance_of::<web_sys::HtmlButtonElement>() {
        return match element.get_attribute("type") {
            Some(ty) => ty != "submit" && ty != "reset",
            None => false,
        };
    }

    if is_html_anchor(element) {
        return false;
    }

    true
}

const NON_TEXT_INPUT_TYPES: [&'static str; 9] = [
    "checkbox", "radio", "range", "color", "file", "image", "button", "submit", "reset",
];

fn is_valid_input_key(element: web_sys::HtmlInputElement, key: &str) -> bool {
    // Checkboxes and radio-buttons should only toggle with space, not enter.
    // TODO: Whats with custom implementations for checkboxes and radio buttons??
    match element.get_attribute("type") {
        Some(ty) => match ty.as_str() {
            "checkbox" | "radio" => key == " " || key == "Spacebar",
            other => NON_TEXT_INPUT_TYPES.contains(&other),
        },
        None => true,
    }
}

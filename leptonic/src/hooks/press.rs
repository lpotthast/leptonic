use std::cell::RefCell;

use educe::Educe;
use leptos_reactive::{
    create_signal, store_value, Callable, Callback, MaybeSignal, Signal, SignalGetUntracked, SignalSet
};
use leptos_use::use_event_listener;
use wasm_bindgen::JsCast;
use web_sys::{KeyboardEvent, MouseEvent, PointerEvent};

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
    PressStart(PressEvent),
    PressEnd(PressEvent),
    PressUp(PressEvent),
    Press(PressEvent),
}

#[derive(Educe)]
#[educe(Debug)]
pub struct PressEvent {
    /// The pointer type that triggered the press event.
    pub pointer_type: PointerType,

    /// The target element of the press event.
    pub target: Option<web_sys::EventTarget>,

    /// Sates which modifier keys were held during the press event.
    pub modifiers: KeyModifiers,

    /// By default, press events stop propagation to parent elements.
    /// In cases where a handler decides not to handle a specific event,
    /// it can call `continuePropagation()` to allow a parent to handle it.
    #[educe(Debug(ignore))]
    pub continue_propagation: Box<dyn FnOnce()>,
}

#[derive(Debug, Clone, Copy)]
pub struct UsePressInput {
    /// Wether the targeted element is currently disabled.
    pub disabled: MaybeSignal<bool>,

    pub on_press: Callback<PressEvent>,
    pub on_press_up: Option<Callback<PressEvent>>,
    pub on_press_start: Option<Callback<PressEvent>>,
    pub on_press_end: Option<Callback<PressEvent>>,
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
}

#[derive(Debug)]
pub struct UsePressReturn {
    pub props: UsePressProps,
    pub is_pressed: Signal<bool>,
}

struct PressState {
    pointer_id: i32,
    pointer_type: PointerType,
    target: Option<web_sys::EventTarget>,
    is_over_target: bool,

    global_on_pointer_move_cleanup: Box<dyn Fn()>,
    global_on_pointer_up_cleanup: Box<dyn Fn()>,
    global_on_pointer_cancel_cleanup: Box<dyn Fn()>,
}

fn use_continue_propagation() -> (RefCell<bool>, Box<dyn FnOnce()>) {
    let continue_propagation_state = RefCell::new(false);
    let state_clone = continue_propagation_state.clone();
    let continue_propagation = Box::new(move || {
        state_clone.replace(false);
    });
    (continue_propagation_state, continue_propagation)
}

pub fn use_press(input: UsePressInput) -> UsePressReturn {
    let attrs = Attributes::new();

    let (is_pressed, set_is_pressed) = create_signal(false);

    let state = store_value::<Option<PressState>>(None);

    let initialize_press_state =
        move |e: &PointerEvent,
              on_pointer_move: Box<dyn Fn(PointerEvent)>,
              on_pointer_up: Box<dyn Fn(PointerEvent)>,
              on_pointer_cancel: Box<dyn Fn(PointerEvent)>| {
            if !input.disabled.get_untracked() {
                debug_assert_eq!(state.with_value(|s| s.is_none()), true, "Implicit cleanup ist not supported. Forgot to call cleanup() before initializing a new PressState?");

                state.set_value(Some(PressState {
                    pointer_id: e.pointer_id(),
                    pointer_type: PointerType::from(e.pointer_type()),
                    target: e.target(),
                    is_over_target: e
                        .current_target()
                        .unwrap()
                        .is_over(e, e.target().as_ref().unwrap().as_element().unwrap()),
                    global_on_pointer_move_cleanup: Box::new(use_event_listener(
                        e.current_target().unwrap().get_owner_document(),
                        leptos::ev::pointermove,
                        on_pointer_move,
                    )),
                    global_on_pointer_up_cleanup: Box::new(use_event_listener(
                        e.current_target().unwrap().get_owner_document(),
                        leptos::ev::pointerup,
                        on_pointer_up,
                    )),
                    global_on_pointer_cancel_cleanup: Box::new(use_event_listener(
                        e.current_target().unwrap().get_owner_document(),
                        leptos::ev::pointercancel,
                        on_pointer_cancel,
                    )),
                }));
            }
        };

    // Has no effect if press is already started. Calling this multiple times only executes the effect once.
    let trigger_press_start = move |s: &PressState, e: &PointerEvent| {
        if !is_pressed.get_untracked() {
            if let Some(on_press_start) = input.on_press_start {
                let (continue_propagation_state, continue_propagation) = use_continue_propagation();
                on_press_start.call(PressEvent {
                    pointer_type: s.pointer_type.clone(),
                    target: s.target.clone(),
                    modifiers: e.modifiers(),
                    continue_propagation,
                });
                if !continue_propagation_state.into_inner() {
                    e.stop_propagation();
                }
            }
            set_is_pressed.set(true);
        }
    };

    // Has no effect if press is not yet started. Calling this multiple times only executes the effect once.
    let trigger_press_end = move |s: &PressState, e: &PointerEvent| {
        if is_pressed.get_untracked() {
            if let Some(on_press_end) = input.on_press_end {
                let (continue_propagation_state, continue_propagation) = use_continue_propagation();
                on_press_end.call(PressEvent {
                    pointer_type: s.pointer_type.clone(),
                    target: s.target.clone(),
                    modifiers: e.modifiers(),
                    continue_propagation,
                });
                if !continue_propagation_state.into_inner() {
                    e.stop_propagation();
                }
            }
            set_is_pressed.set(false);
        }
    };

    let trigger_press = move |s: &PressState, e: &PointerEvent| {
        debug_assert_eq!(
            is_pressed.get_untracked(),
            false,
            "Only call trigger_press after triggering a trigger_press_end!"
        );

        let (continue_propagation_state, continue_propagation) = use_continue_propagation();
        input.on_press.call(PressEvent {
            pointer_type: s.pointer_type.clone(),
            target: s.target.clone(),
            modifiers: e.modifiers(),
            continue_propagation,
        });
        if !continue_propagation_state.into_inner() {
            e.stop_propagation();
        }
    };

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
            let (continue_propagation_state, continue_propagation) = use_continue_propagation();
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

    // Reset press state.
    let on_pointer_move = Box::new(move |e: PointerEvent| {
        // Re-emit a "start" event, when we have a state.
        // This means: The user already started an interaction but let the pointer leave the target and let it re-enter.
        state.update_value(|s| {
            if let Some(s) = s.as_mut() {
                if e.pointer_id() != s.pointer_id {
                    return;
                }
                let is_over_target = e
                    .current_target()
                    .unwrap()
                    .is_over(&e, s.target.as_ref().and_then(|t| t.as_element()).unwrap());
                match (s.is_over_target, is_over_target) {
                    (true, false) => trigger_press_end(s, &e),
                    (false, true) => trigger_press_start(s, &e),
                    _ => {}
                }
                s.is_over_target = is_over_target;
            }
        });
    });

    // Finish a press.
    let on_pointer_up = Box::new(move |e: PointerEvent| {
        if !e.current_target_contains_target() {
            return;
        }
        state.with_value(|s| {
            if let Some(s) = s.as_ref() {
                trigger_press_end(s, &e);

                e.stop_propagation();

                let is_over_target = e
                    .current_target()
                    .unwrap()
                    .is_over(&e, s.target.as_ref().and_then(|t| t.as_element()).unwrap());

                if is_over_target {
                    trigger_press(s, &e);
                }

                if let Some(target) = e.target() {
                    if let Some(target) = target.as_element() {
                        restore_text_selection(target);
                    }
                }
                (s.global_on_pointer_move_cleanup)();
                (s.global_on_pointer_up_cleanup)();
                (s.global_on_pointer_cancel_cleanup)();
            }
        });
        state.set_value(None);
    });

    // Cancel the ongoing press.
    let on_pointer_cancel = Box::new(move |e: PointerEvent| {
        if !e.current_target_contains_target() {
            return;
        }
        state.update_value(|s| {
            if let Some(s) = s.as_mut() {
                if is_pressed.get_untracked() {
                    trigger_press_end(s, &e);
                }
                (s.global_on_pointer_move_cleanup)();
                (s.global_on_pointer_up_cleanup)();
                (s.global_on_pointer_cancel_cleanup)();
            }
        });
        state.set_value(None);
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

        let target = e.target();

        if let Some(target) = target.as_ref() {
            if let Some(target) = target.as_element() {
                disable_text_selection(target);
            }
        }

        initialize_press_state(
            &e,
            on_pointer_move.clone(),
            on_pointer_up.clone(),
            on_pointer_cancel.clone(),
        );

        state.update_value(move |s| {
            if let Some(s) = s {
                trigger_press_start(s, &e);
            }
        });
    });

    UsePressReturn {
        props: UsePressProps {
            attrs,
            on_key_down,
            on_click,
            on_pointer_down,
        },
        is_pressed: is_pressed.into(),
    }
}

// TODO: This should also work for pointer events!
fn is_virtual_mouse_event(event: &MouseEvent) -> bool {
    let page_x = event.page_x();
    let page_y = event.page_y();
    let buttons = event.buttons();
    let detail = event.detail();
    let is_trusted = event.is_trusted();

    tracing::info!(page_x, page_y, buttons, detail, is_trusted);

    !is_trusted
}

// TODO: This should also work for pointer events!
fn is_virtual_pointer_event(event: &PointerEvent) -> bool {
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

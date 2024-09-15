use std::{cell::RefCell, rc::Rc};

use educe::Educe;
use leptos::StoredValue;
use leptos_reactive::{
    create_signal, store_value, Callable, Callback, MaybeSignal, Signal, SignalGetUntracked,
    SignalSet,
};
use leptos_use::use_event_listener;
use typed_builder::TypedBuilder;
use wasm_bindgen::JsCast;
use web_sys::{KeyboardEvent, MouseEvent, PointerEvent};

use crate::utils::{
    attributes::Attributes, current_target_contains_target, event_handlers::EventHandlers,
    pointer_type::PointerType, ElementExt, EventExt, EventModifiers, EventTargetExt, Modifiers,
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/interactions/src/usePress.ts

#[derive(Debug, Clone, Copy)]
pub struct PressResponder {
    on_press_start_handlers: StoredValue<Vec<Callback<PressEvent>>>,
    on_press_handlers: StoredValue<Vec<Callback<PressEvent>>>,
    has_delay: bool,
}

impl PressResponder {
    pub(crate) fn new(has_delay: bool) -> Self {
        PressResponder {
            on_press_start_handlers: store_value(Vec::new()),
            on_press_handlers: store_value(Vec::new()),
            has_delay,
        }
    }

    pub(crate) fn invoke_on_press_start(&self, e: PressEvent) {
        self.on_press_start_handlers.with_value(move |handlers| {
            for h in handlers { 
                h.call(e.clone());
            }
        });
    }

    pub(crate) fn invoke_on_press(&self, e: PressEvent) {
        self.on_press_handlers.with_value(move |handlers| {
            for h in handlers {
                h.call(e.clone());
            }
        });
    }

    /// Adds an event handler to the end of the handler chain.
    pub fn add_on_press_start(&self, handler: Callback<PressEvent>) {
        self.on_press_start_handlers.update_value(move |handlers| {
            handlers.push(handler);
        });
    }

    /// Adds an event handler to the end of the handler chain.
    pub fn add_on_press(&self, handler: Callback<PressEvent>) {
        self.on_press_handlers.update_value(move |handlers| {
            handlers.push(handler);
        });
    }

    pub fn has_delay(&self) -> bool {
        self.has_delay
    }
}

#[derive(Debug)]
pub enum PressEvents {
    PressStart(PressEvent),
    PressEnd(PressEvent),
    PressUp(PressEvent),
    Press(PressEvent),
}

#[derive(Educe, Clone)]
#[educe(Debug)]
pub struct PressEvent {
    /// The pointer type that triggered the press event.
    pub pointer_type: PointerType,

    /// The target element of the press event.
    pub target: Option<web_sys::EventTarget>,

    /// Sates which modifier keys were held during the press event.
    pub modifiers: Modifiers,

    /// By default, press events stop propagation to parent elements.
    /// In cases where a handler decides not to handle a specific event,
    /// it can call `continuePropagation()` to allow a parent to handle it.
    #[educe(Debug(ignore))]
    pub continue_propagation: Rc<dyn Fn()>,
}

#[derive(Debug, Clone, Copy, TypedBuilder)]
pub struct UsePressInput {
    /// Wether presses on the element should be disabled.
    #[builder(setter(into))]
    pub(crate) disabled: MaybeSignal<bool>,

    /// Set this to true if you want controlled press behavior
    /// with the guarantee of no browser-specific behavior happening on user interactions.
    #[builder(default = false)]
    pub(crate) force_prevent_default: bool,

    #[builder(default, setter(into, strip_option))]
    pub(crate) on_press: Option<Callback<PressEvent>>,
    #[builder(default, setter(into, strip_option))]
    pub(crate) on_press_up: Option<Callback<PressEvent>>, // TODO: Call this
    #[builder(default, setter(into, strip_option))]
    pub(crate) on_press_start: Option<Callback<PressEvent>>,
    #[builder(default, setter(into, strip_option))]
    pub(crate) on_press_end: Option<Callback<PressEvent>>,
}

#[derive(Debug)]
pub struct UsePressProps {
    /// These attributes must be spread onto the target element: `<foo {..props.attrs} />`
    pub attrs: Attributes,
    /// These handlers must be spread onto the target element: `<foo {..props.handlers} />`
    pub handlers: EventHandlers,
}

#[derive(Debug)]
pub struct UsePressReturn {
    pub props: UsePressProps,
    pub is_pressed: Signal<bool>,
    pub press_responder: PressResponder,
}

enum GlobalEventHandlers {
    PointerEvents {
        global_on_pointer_move_cleanup: Box<dyn Fn()>,
        global_on_pointer_up_cleanup: Box<dyn Fn()>,
        global_on_pointer_cancel_cleanup: Box<dyn Fn()>,
    },
    KeyboardEvents {
        global_on_key_up_cleanup: Box<dyn Fn()>,
    },
}

struct PressState {
    pointer_id: i32,
    pointer_type: PointerType,
    target: Option<web_sys::EventTarget>,
    is_over_target: bool,

    event_handlers: GlobalEventHandlers,
}

impl PressState {
    fn cleanup_event_handlers(&self) {
        match &self.event_handlers {
            GlobalEventHandlers::PointerEvents {
                global_on_pointer_move_cleanup,
                global_on_pointer_up_cleanup,
                global_on_pointer_cancel_cleanup,
            } => {
                global_on_pointer_move_cleanup();
                global_on_pointer_up_cleanup();
                global_on_pointer_cancel_cleanup();
            }
            GlobalEventHandlers::KeyboardEvents {
                global_on_key_up_cleanup,
            } => {
                global_on_key_up_cleanup();
            }
        }
    }
}

fn use_continue_propagation() -> (RefCell<bool>, Rc<dyn Fn()>) {
    let continue_propagation_state = RefCell::new(false);
    let state_clone = continue_propagation_state.clone();
    let continue_propagation = Rc::new(move || {
        state_clone.replace(true);
    });
    (continue_propagation_state, continue_propagation)
}

enum EventRef<'a> {
    Pointer(&'a PointerEvent),
    Keyboard(&'a KeyboardEvent),
}

#[allow(clippy::too_many_lines)]
pub fn use_press(input: UsePressInput) -> UsePressReturn {
    let attrs = Attributes::new();

    let (is_pressed, set_is_pressed) = create_signal(false);

    let state = store_value::<Option<PressState>>(None);

    let press_responder = PressResponder::new(false);

    let initialize_press_state = move |e: EventRef<'_>, event_handlers: GlobalEventHandlers| {
        debug_assert_eq!(state.with_value(|s| s.is_none()), true, "Implicit cleanup ist not supported. Forgot to call cleanup() before initializing a new PressState?");

        state.set_value(Some(PressState {
            pointer_id: match e {
                EventRef::Pointer(e) => e.pointer_id(),
                EventRef::Keyboard(_e) => 0,
            },
            pointer_type: match e {
                EventRef::Pointer(e) => PointerType::from(e.pointer_type()),
                EventRef::Keyboard(_e) => PointerType::Keyboard,
            },
            target: match e {
                EventRef::Pointer(e) => e.target(),
                EventRef::Keyboard(e) => e.target(),
            },
            is_over_target: match e {
                EventRef::Pointer(e) => e
                    .current_target()
                    .unwrap()
                    .is_over(e, e.target().as_ref().unwrap().as_element().unwrap()),
                EventRef::Keyboard(_e) => false,
            },
            event_handlers,
        }));
    };

    // Has no effect if press is already started. Calling this multiple times only executes the effect once.
    let trigger_press_start = move |s: &PressState, e: EventRef<'_>| {
        if !is_pressed.get_untracked() {
            let (continue_propagation_state, continue_propagation) = use_continue_propagation();
            let press_event = PressEvent {
                pointer_type: s.pointer_type.clone(),
                target: s.target.clone(),
                modifiers: match e {
                    EventRef::Pointer(e) => e.modifiers(),
                    EventRef::Keyboard(e) => e.modifiers(),
                },
                continue_propagation,
            };
            press_responder.invoke_on_press_start(press_event.clone());
            if let Some(on_press_start) = input.on_press_start {
                Callable::call(
                    &on_press_start,
                    press_event,
                );
                if !continue_propagation_state.into_inner() {
                    match e {
                        EventRef::Pointer(e) => e.stop_propagation(),
                        EventRef::Keyboard(e) => e.stop_propagation(),
                    }
                }
            }
            set_is_pressed.set(true);
        }
    };

    // Has no effect if press is not yet started. Calling this multiple times only executes the effect once.
    let trigger_press_end = move |s: &PressState, e: EventRef<'_>| {
        if is_pressed.get_untracked() {
            if let Some(on_press_end) = input.on_press_end {
                let (continue_propagation_state, continue_propagation) = use_continue_propagation();
                Callable::call(
                    &on_press_end,
                    PressEvent {
                        pointer_type: s.pointer_type.clone(),
                        target: s.target.clone(),
                        modifiers: match e {
                            EventRef::Pointer(e) => e.modifiers(),
                            EventRef::Keyboard(e) => e.modifiers(),
                        },
                        continue_propagation,
                    },
                );
                if !continue_propagation_state.into_inner() {
                    match e {
                        EventRef::Pointer(e) => e.stop_propagation(),
                        EventRef::Keyboard(e) => e.stop_propagation(),
                    }
                }
            }
            set_is_pressed.set(false);
        }
    };

    let trigger_press = move |s: &PressState, e: EventRef<'_>| {
        debug_assert_eq!(
            is_pressed.get_untracked(),
            false,
            "Only call trigger_press after triggering a trigger_press_end!"
        );

        let (continue_propagation_state, continue_propagation) = use_continue_propagation();
        let press_event = PressEvent {
            pointer_type: s.pointer_type.clone(),
            target: s.target.clone(),
            modifiers: match e {
                EventRef::Pointer(e) => e.modifiers(),
                EventRef::Keyboard(e) => e.modifiers(),
            },
            continue_propagation,
        };
        press_responder.invoke_on_press(press_event.clone());
        if let Some(on_press) = input.on_press {
            Callable::call(&on_press, press_event);
            if !continue_propagation_state.into_inner() {
                match e {
                    EventRef::Pointer(e) => e.stop_propagation(),
                    EventRef::Keyboard(e) => e.stop_propagation(),
                }
            }
        }
    };

    let on_key_up = Box::new(move |e: KeyboardEvent| {
        let handled = state.with_value(move |s| {
            if let Some(s) = s {
                if !input.disabled.get_untracked()
                    && is_valid_keyboard_event(&e, e.current_target().unwrap())
                {
                    let key = e.key();
                    if e.target()
                        .and_then(|t| t.as_element())
                        .map(|t| should_prevent_default_keyboard(t, key.as_str()))
                        .unwrap_or(false)
                    {
                        e.prevent_default();
                    }

                    // TODO: Spacial handling of link elements?
                    // TODO: Special handling of meta keys?

                    trigger_press_end(s, EventRef::Keyboard(&e));
                    trigger_press(s, EventRef::Keyboard(&e));

                    s.cleanup_event_handlers();
                    true
                } else {
                    false
                }
            } else {
                false
            }
        });
        if handled {
            state.set_value(None);
        }
    });

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

        if state.with_value(|s| s.is_none())
            && !input.disabled.get_untracked()
            && is_valid_keyboard_event(&e, e.current_target().unwrap())
        {
            initialize_press_state(
                EventRef::Keyboard(&e),
                GlobalEventHandlers::KeyboardEvents {
                    global_on_key_up_cleanup: Box::new(use_event_listener(
                        e.current_target().unwrap().get_owner_document(),
                        leptos::ev::keyup,
                        on_key_up.clone(),
                    )),
                },
            );

            state.with_value(move |s| {
                if let Some(s) = s {
                    trigger_press_start(s, EventRef::Keyboard(&e));
                }
            });
        }
    });

    let on_click = Box::new(move |e: MouseEvent| {
        if !current_target_contains_target(e.current_target().as_ref(), e.target().as_ref())
            .unwrap_or(true)
        {
            tracing::debug!("Aborting on_click, as current_target did not contain target.");
            return;
        }

        if input.disabled.get_untracked() || input.force_prevent_default {
            e.prevent_default();
        }
        e.stop_propagation();
    });

    // Reset press state.
    let on_pointer_move = Rc::new(move |e: PointerEvent| {
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
                    (true, false) => trigger_press_end(s, EventRef::Pointer(&e)),
                    (false, true) => trigger_press_start(s, EventRef::Pointer(&e)),
                    _ => {}
                }
                s.is_over_target = is_over_target;
            }
        });
    });

    // Finish a press.
    let on_pointer_up = Rc::new(move |e: PointerEvent| {
        if !e.current_target_contains_target() {
            return;
        }
        state.with_value(|s| {
            if let Some(s) = s.as_ref() {
                trigger_press_end(s, EventRef::Pointer(&e));

                if input.force_prevent_default {
                    e.prevent_default();
                }
                e.stop_propagation();

                let is_over_target = e
                    .current_target()
                    .unwrap()
                    .is_over(&e, s.target.as_ref().and_then(|t| t.as_element()).unwrap());

                if is_over_target {
                    trigger_press(s, EventRef::Pointer(&e));
                }

                if let Some(target) = e.target() {
                    if let Some(target) = target.as_element() {
                        target.restore_text_selection();
                    }
                }
                s.cleanup_event_handlers();
            }
        });
        state.set_value(None);
    });

    // Cancel the ongoing press.
    let on_pointer_cancel = Rc::new(move |e: PointerEvent| {
        state.with_value(|s| {
            if let Some(s) = s.as_ref() {
                trigger_press_end(s, EventRef::Pointer(&e));
                if let Some(target) = e.target() {
                    if let Some(target) = target.as_element() {
                        target.restore_text_selection();
                    }
                }
                s.cleanup_event_handlers();
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
                target.disable_text_selection();
            }
        }

        if !input.disabled.get_untracked() {
            initialize_press_state(
                EventRef::Pointer(&e),
                GlobalEventHandlers::PointerEvents {
                    global_on_pointer_move_cleanup: Box::new(use_event_listener(
                        e.current_target().unwrap().get_owner_document(),
                        leptos::ev::pointermove,
                        *on_pointer_move,
                    )),
                    global_on_pointer_up_cleanup: Box::new(use_event_listener(
                        e.current_target().unwrap().get_owner_document(),
                        leptos::ev::pointerup,
                        *on_pointer_up,
                    )),
                    global_on_pointer_cancel_cleanup: Box::new(use_event_listener(
                        e.current_target().unwrap().get_owner_document(),
                        leptos::ev::pointercancel,
                        *on_pointer_cancel,
                    )),
                },
            );

            state.update_value(move |s| {
                if let Some(s) = s {
                    trigger_press_start(s, EventRef::Pointer(&e));
                }
            });
        }
    });

    UsePressReturn {
        props: UsePressProps {
            attrs,
            handlers: EventHandlers::builder()
                .on_click(on_click)
                .on_pointer_down(on_pointer_down)
                .on_key_down(on_key_down)
                .build(),
        },
        is_pressed: is_pressed.into(),
        press_responder,
    }
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

    !element.is_anchor_link()
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

/// Accessibility for keyboards. Space and Enter only.
fn is_valid_keyboard_event(e: &KeyboardEvent, current_target: web_sys::EventTarget) -> bool {
    let key = e.key();
    let resembles_press = match key.as_str() {
        "Enter" | " " | "Spacebar" | "Space" => true, // "Spacebar" is for IE 11
        _ => false,
    };

    if !resembles_press {
        return false;
    }

    match current_target.as_element() {
        Some(element) => {
            let is_input = element.is_instance_of::<web_sys::HtmlInputElement>();
            let is_text_area = element.is_instance_of::<web_sys::HtmlTextAreaElement>();
            let is_content_editable = false; // TODO: element.isContentEditable
            let is_link = element.is_link();

            !((is_input
                && !is_valid_input_key(
                    element.unchecked_into::<web_sys::HtmlInputElement>(),
                    key.as_str(),
                ))
                || is_text_area
                || is_content_editable)
                && !(is_link && key.as_str() != "Enter") // Links should only trigger with Enter key
        }
        None => true,
    }
}

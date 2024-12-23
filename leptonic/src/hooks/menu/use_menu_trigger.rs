use educe::Educe;
use leptos::{Callback, Oco};
use leptos_reactive::{MaybeSignal, SignalGetUntracked};
use leptos_use::core::ElementMaybeSignal;
use typed_builder::TypedBuilder;
use web_sys::KeyboardEvent;

use crate::{
    hooks::{
        interactions::use_press::PressResponder, use_overlay_trigger, PressEvent,
        UseOverlayTriggerInput, UseOverlayTriggerReturn,
    },
    prelude::AriaHasPopup,
    state::{
        menu::{MenuTriggerState, OnOpen},
        overlay::OverlayTriggerState,
    },
    utils::{attributes::Attributes, event_handlers::EventHandlers, pointer_type::PointerType},
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/menu/src/useMenuTrigger.ts

#[derive(Debug, Clone, Copy)]
pub enum MenuTriggerType {
    Press,
    LongPress,
}

#[derive(Clone, Educe, TypedBuilder)]
#[educe(Debug)]
pub struct UseMenuTriggerInput {
    /// Wether the trigger should be disabled.
    #[builder(setter(into))]
    pub(crate) disabled: MaybeSignal<bool>,

    /// Id of the menu trigger.
    #[builder(setter(into))]
    pub(crate) menu_trigger_id: Oco<'static, str>,

    /// Id of the menu being opened.
    #[builder(setter(into))]
    pub(crate) menu_id: Oco<'static, str>,

    /// The type of overlay opened by this trigger.
    /// Using the variants `False` or `True` will result in a runtime warning on debug builds!
    /// Prefer `AriaHasPopup::Menu` if you are unsure what to use otherwise.
    #[builder(default = AriaHasPopup::Menu, setter(into))]
    pub(crate) overlay_type: AriaHasPopup, // TODO: Should we only support "menu" and "listbox" instead?
}

#[derive(Debug)]
pub struct UseMenuTriggerProps {
    /// These attributes must be spread onto the target element: `<foo {..props.attrs} />`
    pub attrs: Attributes,
    /// These handlers must be spread onto the target element: `<foo {..props.handlers} />`
    pub handlers: EventHandlers,
}

#[derive(Debug)]
pub struct UseMenuTriggerReturn {
    pub props: UseMenuTriggerProps,
}

/// Provides the behavior and accessibility implementation for a menu trigger.
pub fn use_menu_trigger<El, T>(
    el: El,
    press_responder: PressResponder,
    state: MenuTriggerState,
    input: UseMenuTriggerInput,
) -> UseMenuTriggerReturn
where
    El: Into<ElementMaybeSignal<T, web_sys::EventTarget>> + 'static,
    T: Into<web_sys::EventTarget> + Clone + 'static,
{
    let el = el.into();

    // Both `state` and `overlay_trigger_state` effectively control the same signal.
    let overlay_trigger_state = OverlayTriggerState {
        show: state.open,
        set_show: state.set_open,
    };

    // Menus are overlays, so we leverage our existing overlay trigger logic behavior.
    let UseOverlayTriggerReturn {
        props: trigger_props,
    } = use_overlay_trigger(
        overlay_trigger_state,
        None, // We do not supply a press responder here, as we will implement the open/close logic ourselves.
        UseOverlayTriggerInput::builder()
            .overlay_id(input.menu_id)
            .overlay_type(input.overlay_type)
            .build(),
    );

    let on_key_down = Box::new(move |e: KeyboardEvent| {
        if input.disabled.get_untracked() {
            return;
        }

        // When press input is only captured after a delay, only allow keyboard interaction when the alt key is pressed.
        // This brings the keyboard and press interactions more inline with one another.
        if press_responder.has_delay() && !e.alt_key() {
            return;
        }

        // TODO: Do we really need to gate this on `el`?
        if let Some(_el) = el.get_untracked() {
            match e.key().as_str() {
                "Enter" | " " => {
                    // When press input is only captured after a delay, never allow the menu to be triggered with Enter and space.
                    if press_responder.has_delay() {
                        return;
                    }
                    e.prevent_default();
                    e.stop_propagation();
                    state.toggle(OnOpen::First);
                }
                "ArrowDown" => {
                    e.prevent_default();
                    e.stop_propagation();
                    state.toggle(OnOpen::First);
                }
                "ArrowUp" => {
                    e.prevent_default();
                    e.stop_propagation();
                    state.toggle(OnOpen::Last);
                }
                _ => {
                    // Allow other keys. Propagation continues...
                }
            }
        }
    });

    press_responder.add_on_press_start(Callback::new(move |e: PressEvent| {
        if input.disabled.get_untracked() {
            return;
        }

        match press_responder.has_delay() {
            true => {
                // "Long press"
                state.close();
            }
            false => {
                // For consistency with native, open the menu on mouse/key down, but touch up.
                if e.pointer_type != PointerType::Touch && e.pointer_type != PointerType::Keyboard {
                    // If opened with a screen reader, auto focus the first item.
                    // Otherwise, the menu itself will be focused.
                    state.open(match e.pointer_type {
                        PointerType::Virtual => Some(OnOpen::First),
                        _ => None,
                    });
                }
            }
        }
    }));

    press_responder.add_on_press(Callback::new(move |e: PressEvent| {
        if input.disabled.get_untracked() {
            return;
        }

        match press_responder.has_delay() {
            true => {
                // "Long press"
                state.open(Some(OnOpen::First));
            }
            false => {
                if e.pointer_type == PointerType::Touch {
                    state.toggle(OnOpen::First);
                }
            }
        }
    }));

    UseMenuTriggerReturn {
        props: UseMenuTriggerProps {
            attrs: trigger_props.attrs.insert("id", input.menu_trigger_id),
            handlers: EventHandlers::builder().on_key_down(on_key_down).build(),
        },
        // TODO: investigate
        /*menuProps: {
            ...overlayProps,
            'aria-labelledby': menuTriggerId,
            autoFocus: state.focusStrategy || true,
            onClose: state.close
          }*/
    }
}

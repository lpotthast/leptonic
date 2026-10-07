// Upstream: react-aria/src/interactions/useContextMenu.ts @ 99e6102368
use std::time::Duration;

use leptos::{
    ev::{self, On, SharedEventCallback},
    prelude::*,
};
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use web_sys::{KeyboardEvent, MouseEvent};

use crate::{
    hooks::{IntoAttrs, LongPressEvent},
    utils::{
        EventAccessors, EventHandler,
        key::{KeyboardEventKey, KeyboardKey},
        platform::device::{is_ios, is_mac},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The long press opening context menus on iOS (which fires no `contextmenu` event) is returned
//   as `on_long_press_start`/`on_long_press` callbacks for the element's own `use_press` (as
//   `use_button` does with `on_context_menu`), instead of `useLongPress` props: long presses are
//   part of `use_press`, and an element has one press handler.
//
// =============================================================================

/// A request for a context menu.
#[derive(Debug, Clone)]
pub struct ContextMenuEvent {
    /// The element the context menu is for.
    pub target: SendWrapper<web_sys::Element>,
    /// The position relative to the target's left edge, in pixels.
    pub x: f64,
    /// The position relative to the target's top edge, in pixels.
    pub y: f64,
}

/// Input of [`use_context_menu`].
#[derive(Debug, Clone, Copy, Default)]
pub struct UseContextMenuInput {
    /// Called when a context menu is requested.
    pub on_context_menu: Option<Callback<ContextMenuEvent>>,
}

/// Return value of [`use_context_menu`].
#[derive(Debug)]
pub struct UseContextMenuReturn {
    pub props: UseContextMenuProps,
    /// For the element's `use_press` (on iOS): a long press requests the context menu.
    pub on_long_press_start: Option<Callback<LongPressEvent>>,
    pub on_long_press: Option<Callback<LongPressEvent>>,
}

/// Props for the target element.
#[derive(Debug, Clone)]
pub struct UseContextMenuProps {
    pub on_contextmenu: EventHandler<MouseEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
}

pub type UseContextMenuAttrs = (
    On<ev::contextmenu, SharedEventCallback<MouseEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
);

impl IntoAttrs for UseContextMenuProps {
    type Attrs = UseContextMenuAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.on_contextmenu.into_on(ev::contextmenu),
            self.on_keydown.into_on(ev::keydown),
        )
    }
}

/// Handles requests for a context menu across mouse (right click, Control+click on macOS),
/// keyboard (Shift+F10, the context menu key, Control+Enter on macOS), touch (long press) and
/// screen readers. The target's own context menu is prevented.
pub fn use_context_menu(input: UseContextMenuInput) -> UseContextMenuReturn {
    let UseContextMenuInput { on_context_menu } = input;
    let Some(on_context_menu) = on_context_menu else {
        return UseContextMenuReturn {
            props: UseContextMenuProps {
                on_contextmenu: EventHandler::empty(),
                on_keydown: EventHandler::empty(),
            },
            on_long_press_start: None,
            on_long_press: None,
        };
    };
    // Whether the browser fired `contextmenu` itself (so the fallbacks stay quiet).
    let fired = StoredValue::new(false);

    let at = move |target: &web_sys::Element, x: f64, y: f64| {
        on_context_menu.run(ContextMenuEvent {
            target: SendWrapper::new(target.clone()),
            x,
            y,
        });
    };

    let on_contextmenu = EventHandler::new(move |e: MouseEvent| {
        e.stop_propagation();
        e.prevent_default();
        fired.set_value(true);
        let Ok(target) = e.expect_current_target().dyn_into::<web_sys::Element>() else {
            return;
        };
        let rect = target.get_bounding_client_rect();
        at(&target, e.client_x() - rect.x(), e.client_y() - rect.y());
    });

    // Some versions of Safari and Chrome fire no `contextmenu` event on macOS' Control+Enter.
    let on_keydown = EventHandler::new(move |e: KeyboardEvent| {
        if !(is_mac() && e.ctrl_key() && e.typed_key() == KeyboardKey::Enter) {
            return;
        }
        fired.set_value(false);
        e.stop_propagation();
        let Ok(target) = e.expect_current_target().dyn_into::<web_sys::Element>() else {
            return;
        };
        let target = SendWrapper::new(target);
        set_timeout(
            move || {
                // The target may have unmounted in between.
                let Some(was_fired) = fired.try_get_value() else {
                    return;
                };
                if was_fired {
                    fired.set_value(false);
                } else {
                    let rect = target.get_bounding_client_rect();
                    at(&target, rect.width() / 2.0, rect.height() / 2.0);
                }
            },
            Duration::from_millis(10),
        );
    });

    // iOS fires no `contextmenu` event: a long press requests the menu.
    let ios = is_ios();
    UseContextMenuReturn {
        props: UseContextMenuProps {
            on_contextmenu,
            on_keydown,
        },
        on_long_press_start: ios
            .then(|| Callback::new(move |_: LongPressEvent| fired.set_value(false))),
        on_long_press: ios.then(|| {
            Callback::new(move |e: LongPressEvent| {
                if fired.get_value() {
                    fired.set_value(false);
                    return;
                }
                if let Some(target) = e.target.dyn_ref::<web_sys::Element>() {
                    at(target, e.x.unwrap_or_default(), e.y.unwrap_or_default());
                }
            })
        }),
    }
}

// Upstream: react-aria/src/interactions/useContextMenu.ts @ 99e6102368
// Upstream: react-aria/test/interactions/useContextMenu.test.tsx @ 99e6102368
use std::time::Duration;

use leptos::{
    ev::{self},
    prelude::*,
};
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use web_sys::{KeyboardEvent, MouseEvent};

use crate::{
    EventHandler, IntoAttrs, OnEvent, Point,
    hooks::interactions::{LongPress, LongPressEvent},
    utils::{
        dom_ext::EventAccessors,
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
//   as a `LongPress` group for the element's own `use_press` (as `use_button` does with
//   `on_context_menu`), instead of `useLongPress` props: long presses are part of `use_press`,
//   and an element has one press handler.
// - `ContextMenuEvent` has the position as a `Point` (react-aria: `x`, `y`).
//
// =============================================================================

/// A request for a context menu.
#[derive(Debug, Clone)]
pub struct ContextMenuEvent {
    /// The element the context menu is for.
    pub target: SendWrapper<web_sys::Element>,
    /// The position relative to the target's top left corner, in CSS pixels.
    pub point: Point,
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
    pub long_press: Option<LongPress>,
}

/// Props for the target element.
#[derive(Debug, Clone)]
pub struct UseContextMenuProps {
    pub on_contextmenu: EventHandler<MouseEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
}

pub type UseContextMenuAttrs = (OnEvent<ev::contextmenu>, OnEvent<ev::keydown>);

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
            long_press: None,
        };
    };
    // Whether the browser fired `contextmenu` itself (so the fallbacks stay quiet).
    let fired = StoredValue::new(false);

    let at = move |target: &web_sys::Element, point: Point| {
        on_context_menu.run(ContextMenuEvent {
            target: SendWrapper::new(target.clone()),
            point,
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
        at(
            &target,
            Point::new(e.client_x() - rect.x(), e.client_y() - rect.y()),
        );
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
                    at(&target, Point::new(rect.width() / 2.0, rect.height() / 2.0));
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
        long_press: ios.then(|| LongPress {
            on_long_press_start: Some(Callback::new(move |_: LongPressEvent| {
                fired.set_value(false);
            })),
            on_long_press: Some(Callback::new(move |e: LongPressEvent| {
                if fired.get_value() {
                    fired.set_value(false);
                } else {
                    at(&e.target, e.point);
                }
            })),
            ..LongPress::default()
        }),
    }
}

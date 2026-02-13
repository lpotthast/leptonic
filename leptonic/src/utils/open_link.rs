use std::sync::atomic::{AtomicBool, Ordering};

use wasm_bindgen::{JsCast, JsValue};

use crate::utils::{
    focus::focus_element,
    platform::{browser, device},
    Modifiers,
};

/// Whether a link is currently being programmatically opened.
static IS_OPENING_LINK: AtomicBool = AtomicBool::new(false);

/// Returns whether a link is currently being programmatically opened.
/// Used by focus-visible tracking to suppress modality changes during link activation.
pub(crate) fn is_opening_link() -> bool {
    IS_OPENING_LINK.load(Ordering::Acquire)
}

/// Programmatically open a link element by dispatching a synthetic click event.
///
/// We cannot use the native `HTMLElement.click()` method because:
/// 1. `click()` does not carry modifier key state (ctrl, meta, alt, shift),
///    so Ctrl+click / Cmd+click cannot open links in a new tab.
/// 2. Firefox blocks `target="_blank"` links opened from keyboard events
///    via its popup blocker. We work around this by forcing a modifier key,
///    which opens the link in a background tab instead.
/// 3. `WebKit` on macOS does not support firing click events with modifier keys.
///    We dispatch a `KeyboardEvent('keydown')` with `keyIdentifier: 'Enter'`
///    instead, which `WebKit` recognizes for link activation.
///
/// # Deviations from react-aria
///
/// - No global router-provider (`LinkProvider`) or `RouterContext`.
///   React-aria supports a `RouterProvider` that intercepts link clicks for
///   client-side routing. Leptonic relies on the framework's own routing and
///   does not replicate this indirection.
///
/// Otherwise matches the `openLink` function from
/// `packages/@react-aria/utils/src/openLink.tsx`.
pub(crate) fn open_link(element: &web_sys::Element, modifiers: Modifiers, is_keyboard_event: bool) {
    let mut modifiers = modifiers;

    // Firefox blocks target="_blank" links from keyboard events via its popup blocker.
    // Force a modifier key so the link opens in a background tab instead.
    if browser::is_firefox()
        && is_keyboard_event
        && element
            .get_attribute("target")
            .is_some_and(|t| t == "_blank")
    {
        if device::is_mac() {
            modifiers.meta_key = true;
        } else {
            modifiers.ctrl_key = true;
        }
    }

    // WebKit on macOS (not iPad) doesn't support MouseEvent click with modifier keys.
    // Use a KeyboardEvent with the non-standard `keyIdentifier` property instead.
    let event: web_sys::Event = if browser::is_webkit()
        && device::is_mac()
        && !device::is_ipad()
        && (modifiers.meta_key || modifiers.ctrl_key)
    {
        create_webkit_keyboard_event(modifiers)
    } else {
        create_click_event(modifiers)
    };

    // Set the flag so that focus-visible tracking suppresses modality changes
    // during the synthetic click dispatch.
    IS_OPENING_LINK.store(true, Ordering::Release);

    // Focus the element before dispatching, without scrolling the page.
    focus_element(element, true);

    let _ = element.dispatch_event(&event);

    IS_OPENING_LINK.store(false, Ordering::Release);
}

fn create_click_event(modifiers: Modifiers) -> web_sys::Event {
    let init = web_sys::MouseEventInit::new();
    init.set_meta_key(modifiers.meta_key);
    init.set_ctrl_key(modifiers.ctrl_key);
    init.set_alt_key(modifiers.alt_key);
    init.set_shift_key(modifiers.shift_key);
    init.set_detail(1);
    init.set_bubbles(true);
    init.set_cancelable(true);
    web_sys::MouseEvent::new_with_mouse_event_init_dict("click", &init)
        .expect("MouseEvent creation should not fail")
        .into()
}

/// Creates a `WebKit`-compatible keyboard event with the non-standard `keyIdentifier` property.
///
/// `WebKit` on macOS does not properly handle synthetic mouse click events with modifier keys.
/// Instead, it responds to a `keydown` event with `keyIdentifier: 'Enter'`, which triggers
/// the same link activation behavior as a real Enter keypress.
fn create_webkit_keyboard_event(modifiers: Modifiers) -> web_sys::Event {
    // Build the init dict manually using js_sys because web_sys::KeyboardEventInit
    // does not expose the non-standard `keyIdentifier` property used by WebKit.
    let init = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&init, &"keyIdentifier".into(), &"Enter".into());
    let _ = js_sys::Reflect::set(
        &init,
        &"metaKey".into(),
        &JsValue::from_bool(modifiers.meta_key),
    );
    let _ = js_sys::Reflect::set(
        &init,
        &"ctrlKey".into(),
        &JsValue::from_bool(modifiers.ctrl_key),
    );
    let _ = js_sys::Reflect::set(
        &init,
        &"altKey".into(),
        &JsValue::from_bool(modifiers.alt_key),
    );
    let _ = js_sys::Reflect::set(
        &init,
        &"shiftKey".into(),
        &JsValue::from_bool(modifiers.shift_key),
    );
    let _ = js_sys::Reflect::set(&init, &"bubbles".into(), &JsValue::from_bool(true));
    let _ = js_sys::Reflect::set(&init, &"cancelable".into(), &JsValue::from_bool(true));

    web_sys::KeyboardEvent::new_with_keyboard_event_init_dict(
        "keydown",
        init.unchecked_ref::<web_sys::KeyboardEventInit>(),
    )
    .expect("KeyboardEvent creation should not fail")
    .into()
}

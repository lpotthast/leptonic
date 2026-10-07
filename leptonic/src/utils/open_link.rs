// Upstream: react-aria/src/utils/openLink.tsx @ 99e6102368
//! Opening links programmatically (react-aria's `openLink`).

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `openLink(target, modifiers, setOpening)` is two functions: [`open_link`] (marks the dispatch as
//   opening a link) and [`open_link_unmarked`] (react-aria's `setOpening = false`), instead of a
//   bool parameter. The "opening" mark is read through [`is_opening_link`] (react-aria: the
//   `openLink.isOpening` property).
//
// ## OMITTED FEATURES
// - `RouterProvider`, `useRouter`, `shouldClientNavigate`, `useLinkProps`, `useSyntheticLinkProps`:
//   client-side routing is the app's router's job (Leptos' router intercepts link clicks itself).
//   Collections build their synthetic `<a>` in `Node::open` (react-aria's `getSyntheticLink`).
//
// =============================================================================

use std::cell::Cell;

use wasm_bindgen::{JsCast, JsValue};

use crate::utils::{
    Modifiers,
    focus::focus_element,
    platform::{browser, device},
};

thread_local! {
    /// Whether [`open_link`] is dispatching its event (react-aria's `openLink.isOpening`).
    static IS_OPENING_LINK: Cell<bool> = const { Cell::new(false) };
}

/// Whether [`open_link`] is dispatching its synthetic event right now (react-aria's
/// `openLink.isOpening`). Focus-visible tracking and selection ignore that event, and press
/// handlers don't treat its click as a press.
pub(crate) fn is_opening_link() -> bool {
    IS_OPENING_LINK.with(Cell::get)
}

/// Opens the link `element` (an `<a href>`) as if the user clicked it with `modifiers`: focuses it
/// without scrolling and dispatches a synthetic click (react-aria's `openLink`). The dispatch is
/// marked as opening a link ([`is_opening_link`]).
///
/// `HTMLElement.click()` can't carry modifiers (Cmd/Ctrl-click opens a new tab). WebKit on macOS
/// doesn't follow clicks with modifiers at all, but follows a `keydown` with the non-standard
/// `keyIdentifier: "Enter"`; that is dispatched there instead.
pub(crate) fn open_link(element: &web_sys::Element, modifiers: Modifiers) {
    dispatch(element, modifiers, true);
}

/// Like [`open_link`], without marking the dispatch as opening a link (react-aria's
/// `openLink(target, modifiers, false)`): `use_press` opens a link pressed with a key other than
/// Enter this way, and its own click handling then sees the click as usual.
#[allow(dead_code)] // For `use_press` (react-aria's usePress passes `setOpening = false`).
pub(crate) fn open_link_unmarked(element: &web_sys::Element, modifiers: Modifiers) {
    dispatch(element, modifiers, false);
}

fn dispatch(element: &web_sys::Element, modifiers: Modifiers, mark_opening: bool) {
    let event = if browser::is_webkit() && device::is_mac() && !device::is_ipad() {
        create_webkit_keyboard_event(modifiers)
    } else {
        create_click_event(modifiers)
    };
    IS_OPENING_LINK.with(|opening| opening.set(mark_opening));
    focus_element(element, true);
    let _ = element.dispatch_event(&event);
    IS_OPENING_LINK.with(|opening| opening.set(false));
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

/// A `keydown` with `keyIdentifier: "Enter"`, which WebKit's anchor element follows (with
/// modifiers, unlike a synthetic click). `KeyboardEventInit` has no `keyIdentifier`, so the init
/// dictionary is built by hand.
fn create_webkit_keyboard_event(modifiers: Modifiers) -> web_sys::Event {
    let init = js_sys::Object::new();
    let set = |key: &str, value: JsValue| {
        let _ = js_sys::Reflect::set(&init, &JsValue::from_str(key), &value);
    };
    set("keyIdentifier", JsValue::from_str("Enter"));
    set("metaKey", JsValue::from_bool(modifiers.meta_key));
    set("ctrlKey", JsValue::from_bool(modifiers.ctrl_key));
    set("altKey", JsValue::from_bool(modifiers.alt_key));
    set("shiftKey", JsValue::from_bool(modifiers.shift_key));
    web_sys::KeyboardEvent::new_with_keyboard_event_init_dict(
        "keydown",
        init.unchecked_ref::<web_sys::KeyboardEventInit>(),
    )
    .expect("KeyboardEvent creation should not fail")
    .into()
}

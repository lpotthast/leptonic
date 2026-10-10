// Upstream: react-aria/src/overlays/usePreventScroll.ts @ 740c6c5c4a
//! Locking the page's scroll position while an overlay is open (react-aria's `usePreventScroll`).

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `is_disabled` is a `Signal<bool>`: the lock follows it (react-aria re-runs its layout effect
//   when `isDisabled` changes).
//
// ## DIFFERENT BEHAVIOR
// - WebKit on iOS: after a programmatic focus, the focused element is scrolled into view
//   immediately when the on-screen keyboard was already open (the previously focused element
//   opens one), else after the visual viewport's next `resize`. react-aria waits with
//   `runAfterKeyboard`/`runAfterKeyboardTransition`, which build on its on-screen keyboard
//   tracker (`utils/keyboard.tsx`, commit 5a6c32500, shared with `useViewportSize` and overlay
//   placement); leptonic has no port of that tracker yet.
//
// ## OMITTED FEATURES
// - `UNSTABLE_overrideFocus` (the focus override for other browsers than WebKit on iOS): an
//   unstable, private option.
// - The `__webpack_nonce__` fallback for the CSP nonce of the injected `<style>`: webpack-specific;
//   the nonce comes from `<meta name|property="csp-nonce">`.
//
// =============================================================================

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use leptos::prelude::*;
use leptos_use::use_window;
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::HtmlElement;

use crate::utils::{
    platform::{browser, device},
    shadow_dom::{get_active_element, get_event_target},
};

/// A function undoing what a setup step did.
type Restore = Box<dyn FnOnce()>;

thread_local! {
    /// The number of active (enabled) instances, and how to undo the lock when it drops to 0.
    static PREVENT_SCROLL_STATE: RefCell<PreventScrollState> =
        const { RefCell::new(PreventScrollState { count: 0, restore: None }) };
}

struct PreventScrollState {
    count: usize,
    restore: Option<Restore>,
}

/// Input of [`use_prevent_scroll`].
#[derive(Debug, Clone, Copy, Default)]
pub struct UsePreventScrollInput {
    /// Whether the scroll lock is disabled.
    pub is_disabled: Signal<bool>,
}

/// Prevents scrolling of the page while enabled (and mounted), and restores it afterwards; the
/// content doesn't shift when the scrollbar disappears (react-aria's `usePreventScroll`). Nested
/// overlays share one lock.
///
/// Most browsers get `overflow: hidden` on the root element plus a stable scrollbar gutter (or
/// padding of the scrollbar's width). WebKit on iOS scrolls the page anyway in many situations, so
/// there touch moves outside scrollable elements are prevented, nested scroll areas don't chain to
/// the page, and focusing an element scrolls only its scroll parents.
pub fn use_prevent_scroll(input: UsePreventScrollInput) {
    let UsePreventScrollInput { is_disabled } = input;

    // Whether this instance holds one of the shared count's references: only then may it release
    // one (a disabled instance unmounting must not re-enable scrolling under an open overlay). On
    // the server it never does, so the thread-local state is never touched there.
    let holds = StoredValue::new(false);
    let release = move || {
        if holds.try_get_value() == Some(true) {
            holds.set_value(false);
            decrement_and_maybe_restore();
        }
    };
    Effect::new(move |_| {
        let enabled = !is_disabled.get();
        release();
        if enabled {
            increment_and_maybe_setup();
            holds.set_value(true);
        }
    });
    on_cleanup(release);
}

fn increment_and_maybe_setup() {
    PREVENT_SCROLL_STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.count += 1;
        if state.count == 1 {
            state.restore = if device::is_ios() && browser::is_webkit() {
                prevent_scroll_mobile_webkit()
            } else {
                prevent_scroll_standard()
            };
        }
    });
}

fn decrement_and_maybe_restore() {
    // Take the restore function out before running it: it must not run under the borrow.
    let restore = PREVENT_SCROLL_STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.count = state.count.saturating_sub(1);
        if state.count == 0 {
            state.restore.take()
        } else {
            None
        }
    });
    if let Some(restore) = restore {
        restore();
    }
}

/// Runs every restore function, in reverse order of setup.
fn chain(restores: Vec<Restore>) -> Restore {
    Box::new(move || {
        for restore in restores.into_iter().rev() {
            restore();
        }
    })
}

// --- Standard browser path ---

/// For most browsers, all we need to do is set `overflow: hidden` on the root element, and
/// add some padding to prevent the page from shifting when the scrollbar is hidden.
fn prevent_scroll_standard() -> Option<Restore> {
    let window = use_window();
    let window = window.as_ref()?;
    let root: HtmlElement = window.document()?.document_element()?.unchecked_into();

    let scrollbar_width = window
        .inner_width()
        .ok()
        .and_then(|w| w.as_f64())
        .unwrap_or(0.0)
        - f64::from(root.client_width());

    let mut restores: Vec<Restore> = Vec::new();
    if scrollbar_width > 0.0 {
        // Use scrollbar-gutter when supported because it also works for fixed positioned elements.
        if js_sys::Reflect::has(&root.style(), &"scrollbarGutter".into()).unwrap_or(false) {
            restores.push(set_style(&root, "scrollbar-gutter", "stable"));
        } else {
            restores.push(set_style(
                &root,
                "padding-right",
                &format!("{scrollbar_width}px"),
            ));
        }
    }
    restores.push(set_style(&root, "overflow", "hidden"));
    Some(chain(restores))
}

// --- WebKit on iOS ---
//
// Mobile Safari (every browser on iOS runs WebKit) is a whole different beast. Even with
// `overflow: hidden`, it still scrolls the page in many situations:
//
// 1. When the bottom toolbar and address bar are collapsed, page scrolling is always allowed.
// 2. When the keyboard is visible, the viewport does not resize. Instead, the keyboard covers part
//    of it, so it becomes scrollable.
// 3. When tapping on an input, the page always scrolls so that the input is centered in the visual
//    viewport. This may cause even fixed position elements to scroll off the screen.
// 4. When using the next/previous buttons in the keyboard to navigate between inputs, the whole
//    page always scrolls, even if the input is inside a nested scrollable element that could be
//    scrolled instead.
//
// In order to work around these cases, and prevent scrolling without jankiness, we do a few
// things:
//
// 1. Prevent default on `touchmove` events that are not in a scrollable element. This prevents
//    touch scrolling on the window.
// 2. Set `overscroll-behavior: contain` on nested scrollable regions so they do not scroll the
//    page when at the top or bottom. Work around a bug where this does not work when the element
//    does not actually overflow by preventing default in a `touchmove` event. This is best effort:
//    we can't prevent default when pinch zooming or when an element contains text selection, which
//    may allow scrolling in some cases.
// 3. Override `HTMLElement.prototype.focus` to focus without scrolling the page, then scroll only
//    the element's scroll parents; focus moving to an element opening the keyboard is redone
//    through it.
fn prevent_scroll_mobile_webkit() -> Option<Restore> {
    let window = use_window();
    let window = window.as_ref()?;
    let doc = window.document()?;
    let root: HtmlElement = doc.document_element()?.unchecked_into();

    let mut restores: Vec<Restore> = Vec::new();

    // Set overflow hidden so `scroll_into_viewport` (collections) sees that scrolling is prevented
    // and scrolls only scroll parents instead of calling the native `scrollIntoView`, which moves
    // the window.
    restores.push(set_style(&root, "overflow", "hidden"));

    // Shared between the touchstart and touchmove handlers: the nearest scrollable element of the
    // touch, and whether the touch may move natively.
    let scrollable: Rc<RefCell<Option<web_sys::Element>>> = Rc::new(RefCell::new(None));
    let allow_touch_move: Rc<Cell<bool>> = Rc::new(Cell::new(false));

    let on_touch_start = {
        let scrollable = Rc::clone(&scrollable);
        let allow_touch_move = Rc::clone(&allow_touch_move);
        move |e: web_sys::TouchEvent| {
            let Some(target) =
                get_event_target(&e).and_then(|target| target.dyn_into::<web_sys::Element>().ok())
            else {
                return;
            };
            // Store the nearest scrollable parent element from the element that the user touched.
            *scrollable.borrow_mut() =
                Some(if crate::utils::scroll::is_scrollable(&target, true) {
                    target.clone()
                } else {
                    crate::utils::scroll::get_scroll_parent(&target, true)
                });
            allow_touch_move.set(
                is_in_selection(&target)
                    || is_in_range_input(&e)
                    || has_selected_text_while_focused(&target),
            );
        }
    };

    // Prevent scrolling up when at the top and scrolling down when at the bottom of a nested
    // scrollable area, otherwise mobile Safari will start scrolling the window instead. This must
    // be applied before the touchstart event as of iOS 26, so inject it as a <style> element.
    let style: web_sys::HtmlStyleElement = doc.create_element("style").ok()?.unchecked_into();
    if let Some(nonce) = csp_nonce(&doc) {
        let _ = js_sys::Reflect::set(&style, &"nonce".into(), &nonce.into());
    }
    style.set_text_content(Some(
        "@layer {\n  * {\n    overscroll-behavior: contain;\n  }\n}",
    ));
    if let Some(head) = doc.head() {
        let _ = head.prepend_with_node_1(&style);
    }
    restores.push(Box::new(move || style.remove()));

    let on_touch_move = move |e: web_sys::TouchEvent| {
        // Allow pinch-zooming.
        if e.touches().length() == 2 || allow_touch_move.get() {
            return;
        }
        let scrollable = scrollable.borrow();
        // Prevent scrolling the window.
        let Some(scrollable) = scrollable.as_ref() else {
            e.prevent_default();
            return;
        };
        let doc = scrollable.owner_document();
        let is_page = doc.as_ref().is_some_and(|doc| {
            doc.document_element().as_ref() == Some(scrollable)
                || doc.body().map(web_sys::Element::from).as_ref() == Some(scrollable)
        });
        if is_page {
            e.prevent_default();
            return;
        }
        // overscroll-behavior should prevent scroll chaining, but currently does not if the
        // element doesn't actually overflow. https://bugs.webkit.org/show_bug.cgi?id=243452
        // This checks that both the width and height do not overflow, otherwise we might block
        // horizontal scrolling too. In that case, adding `touch-action: pan-x` to the element
        // will prevent vertical page scrolling. We can't add that automatically because it must
        // be set before the touchstart event.
        if scrollable.scroll_height() == scrollable.client_height()
            && scrollable.scroll_width() == scrollable.client_width()
        {
            e.prevent_default();
        }
    };

    let on_blur = move |e: web_sys::FocusEvent| {
        let related_target = e
            .related_target()
            .and_then(|target| target.dyn_into::<HtmlElement>().ok());
        if let Some(related) = related_target {
            if crate::utils::focusability::will_open_keyboard(&related) {
                // Re-focus programmatically to have the focus override perform the scroll.
                let _ = related.focus();
            }
        } else {
            // When tapping the Done button on the keyboard, focus moves to the body. FocusScope
            // will then restore focus back to the input. Later when tapping the same input
            // again, it is already focused, so no blur event will fire, resulting in the flow
            // above never running and Safari's native scrolling occurring. Instead, move focus
            // to the parent focusable element (e.g. the dialog).
            if let Some(target) =
                get_event_target(&e).and_then(|target| target.dyn_into::<web_sys::Element>().ok())
                && let Some(parent) = target.parent_element()
                && let Ok(Some(focusable)) = parent.closest("[tabindex]")
                && let Ok(focusable) = focusable.dyn_into::<HtmlElement>()
            {
                let options = web_sys::FocusOptions::new();
                options.set_prevent_scroll(true);
                let _ = focusable.focus_with_options(&options);
            }
        }
    };

    if let Some(restore_focus) = override_focus(window) {
        restores.push(restore_focus);
    }
    let doc_target: &web_sys::EventTarget = doc.as_ref();
    restores.push(add_event(
        doc_target,
        "touchstart",
        true,
        false,
        on_touch_start,
    ));
    restores.push(add_event(
        doc_target,
        "touchmove",
        true,
        false,
        on_touch_move,
    ));
    restores.push(add_event(doc_target, "blur", true, true, on_blur));

    Some(chain(restores))
}

/// Whether `target` lies in the document's (non-collapsed) text selection: the touch may adjust
/// it.
fn is_in_selection(target: &web_sys::Element) -> bool {
    target
        .owner_document()
        .and_then(|doc| doc.default_view())
        .and_then(|window| window.get_selection().ok().flatten())
        .is_some_and(|selection| {
            !selection.is_collapsed()
                && selection
                    .contains_node_with_allow_partial_containment(target, true)
                    .unwrap_or(false)
        })
}

/// Whether the touch is on a range input: the touch may slide it.
fn is_in_range_input(e: &web_sys::TouchEvent) -> bool {
    e.composed_path().iter().any(|item| {
        item.dyn_ref::<web_sys::HtmlInputElement>()
            .is_some_and(|input| input.type_() == "range")
    })
}

/// Whether `target` is a focused input or textarea with selected text: the touch may drag the
/// selection handles.
fn has_selected_text_while_focused(target: &web_sys::Element) -> bool {
    let selection = if let Some(input) = target.dyn_ref::<web_sys::HtmlInputElement>() {
        (input.selection_start(), input.selection_end())
    } else if let Some(textarea) = target.dyn_ref::<web_sys::HtmlTextAreaElement>() {
        (textarea.selection_start(), textarea.selection_end())
    } else {
        return false;
    };
    matches!(selection, (Ok(Some(start)), Ok(Some(end))) if start < end)
        && target
            .owner_document()
            .is_some_and(|doc| doc.active_element().as_ref() == Some(target))
}

/// Overrides `HTMLElement.prototype.focus` (with `Reflect.defineProperty`, which also works when
/// `focus` is an accessor) to focus without scrolling the page; unless the caller asked for
/// `preventScroll`, the element is then scrolled into view within its scroll parents. Returns the
/// function restoring the original method.
fn override_focus(window: &web_sys::Window) -> Option<Restore> {
    use js_sys::{Function, Reflect};

    let prototype = Reflect::get(window, &"HTMLElement".into())
        .and_then(|ctor| Reflect::get(&ctor, &"prototype".into()))
        .ok()?;
    let original_focus = Reflect::get(&prototype, &"focus".into()).ok()?;
    if !original_focus.is_function() {
        return None;
    }

    // Called after the element was focused: `focused` and whether the keyboard was open before.
    let scroll =
        Closure::<dyn Fn(JsValue, bool)>::new(|element: JsValue, keyboard_was_open: bool| {
            if let Ok(element) = element.dyn_into::<HtmlElement>() {
                scroll_into_view_when_ready(element, keyboard_was_open);
            }
        });
    let make_wrapper = Function::new_with_args(
        "focus, willOpenKeyboard, scroll",
        "return function (opts) {
            let active = document.activeElement;
            while (active && active.shadowRoot && active.shadowRoot.activeElement) {
                active = active.shadowRoot.activeElement;
            }
            let keyboardWasOpen = active != null && willOpenKeyboard(active);
            focus.call(this, Object.assign({}, opts, {preventScroll: true}));
            if (!opts || !opts.preventScroll) {
                scroll(this, keyboardWasOpen);
            }
        };",
    );
    let will_open_keyboard = Closure::<dyn Fn(JsValue) -> bool>::new(|element: JsValue| {
        element
            .dyn_ref::<web_sys::Element>()
            .is_some_and(crate::utils::focusability::will_open_keyboard)
    });
    let wrapper = make_wrapper
        .call3(
            &JsValue::NULL,
            &original_focus,
            will_open_keyboard.as_ref(),
            scroll.as_ref(),
        )
        .ok()?;
    define_focus(&prototype, &wrapper);

    Some(Box::new(move || {
        define_focus(&prototype, &original_focus);
        // The wrapper is gone from the prototype: its callbacks can go.
        drop((scroll, will_open_keyboard));
    }))
}

/// Sets `prototype.focus` to `value` as a configurable, writable data property.
fn define_focus(prototype: &JsValue, value: &JsValue) {
    let descriptor = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&descriptor, &"configurable".into(), &JsValue::TRUE);
    let _ = js_sys::Reflect::set(&descriptor, &"writable".into(), &JsValue::TRUE);
    let _ = js_sys::Reflect::set(&descriptor, &"value".into(), value);
    let _ = js_sys::Reflect::define_property(
        prototype.unchecked_ref::<js_sys::Object>(),
        &"focus".into(),
        &descriptor,
    );
}

/// Scrolls `target` into view once the on-screen keyboard settled: immediately when it was open
/// already, else after the visual viewport's next resize (see the deviation block). Only if
/// `target` still has the focus then.
fn scroll_into_view_when_ready(target: HtmlElement, keyboard_was_open: bool) {
    let Some(window) = use_window().as_ref().cloned() else {
        return;
    };
    let scroll = move || {
        let is_focused = target
            .owner_document()
            .and_then(|doc| get_active_element(&doc))
            .is_some_and(|active| &active == target.unchecked_ref::<web_sys::Element>());
        if is_focused {
            scroll_into_view(&target);
        }
    };
    match window.visual_viewport() {
        Some(visual_viewport) if !keyboard_was_open => {
            let on_resize = Closure::once_into_js(scroll);
            let options = web_sys::AddEventListenerOptions::new();
            options.set_once(true);
            let _ = visual_viewport
                .add_event_listener_with_callback_and_add_event_listener_options(
                    "resize",
                    on_resize.unchecked_ref(),
                    &options,
                );
        }
        _ => scroll(),
    }
}

/// Centers `target` within each of its scroll parents (below the page), as far as it isn't in
/// view, taking the on-screen keyboard (the visual viewport) into account.
fn scroll_into_view(target: &HtmlElement) {
    let Some(window) = use_window().as_ref().cloned() else {
        return;
    };
    let Some(doc) = target.owner_document() else {
        return;
    };
    let root = doc.scrolling_element().or_else(|| doc.document_element());
    let document_element = doc.document_element();
    let body = doc.body().map(web_sys::Element::from);

    let mut next_target: Option<web_sys::Element> = Some(target.clone().into());
    while let Some(current) = next_target
        && root.as_ref() != Some(&current)
        && current.is_connected()
    {
        // Find the parent scrollable element and adjust the scroll position if the target is not
        // already in view.
        let scrollable = crate::utils::scroll::get_scroll_parent(&current, false);
        if document_element.as_ref() != Some(&scrollable)
            && body.as_ref() != Some(&scrollable)
            && scrollable != current
        {
            let scrollable_rect = scrollable.get_bounding_client_rect();
            let target_rect = current.get_bounding_client_rect();
            if target_rect.top() < scrollable_rect.top()
                || target_rect.bottom() > scrollable_rect.top() + f64::from(current.client_height())
            {
                let mut bottom = scrollable_rect.bottom();
                if let Some(visual_viewport) = window.visual_viewport() {
                    bottom = bottom.min(visual_viewport.offset_top() + visual_viewport.height());
                }
                // Center within the viewport.
                let adjustment = (target_rect.top() - scrollable_rect.top())
                    - ((bottom - scrollable_rect.top()) / 2.0 - target_rect.height() / 2.0);
                // Clamp to the valid range to prevent over-scrolling.
                let max = f64::from(scrollable.scroll_height() - scrollable.client_height());
                let options = web_sys::ScrollToOptions::new();
                options.set_top((scrollable.scroll_top() + adjustment).min(max).max(0.0));
                options.set_behavior(web_sys::ScrollBehavior::Smooth);
                scrollable.scroll_to_with_scroll_to_options(&options);
            }
        }
        next_target = scrollable.parent_element();
    }
}

// --- Helpers ---

/// Listens to `event` on `target` with explicit `passive`, returning the function removing it
/// (react-aria's `addEvent`). Touch listeners on the document are passive by default, which would
/// ignore `preventDefault`.
fn add_event<E: JsCast + 'static>(
    target: &web_sys::EventTarget,
    event: &'static str,
    capture: bool,
    passive: bool,
    handler: impl Fn(E) + 'static,
) -> Restore {
    let closure = Closure::<dyn Fn(web_sys::Event)>::new(move |e: web_sys::Event| {
        handler(e.unchecked_into());
    });
    let options = web_sys::AddEventListenerOptions::new();
    options.set_capture(capture);
    options.set_passive(passive);
    let _ = target.add_event_listener_with_callback_and_add_event_listener_options(
        event,
        closure.as_ref().unchecked_ref(),
        &options,
    );
    let target = target.clone();
    Box::new(move || {
        let _ = target.remove_event_listener_with_callback_and_bool(
            event,
            closure.as_ref().unchecked_ref(),
            capture,
        );
    })
}

/// The CSP nonce for injected `<style>` elements, from `<meta name|property="csp-nonce">`
/// (react-aria's `getNonce`): its `nonce` (browsers hide the attribute), else its `content`.
fn csp_nonce(doc: &web_sys::Document) -> Option<String> {
    let meta = doc
        .query_selector(r#"meta[name="csp-nonce"], meta[property="csp-nonce"]"#)
        .ok()
        .flatten()?;
    js_sys::Reflect::get(&meta, &"nonce".into())
        .ok()
        .and_then(|nonce| nonce.as_string())
        .filter(|nonce| !nonce.is_empty())
        .or_else(|| {
            meta.get_attribute("content")
                .filter(|content| !content.is_empty())
        })
}

/// Sets a CSS property on an element, returning the function that restores the previous value
/// and priority (react-aria's `setStyle`).
fn set_style(element: &HtmlElement, property: &'static str, value: &str) -> Restore {
    let style = element.style();
    let initial_value = style.get_property_value(property).unwrap_or_default();
    let initial_priority = style.get_property_priority(property);
    let _ = style.set_property(property, value);
    Box::new(move || {
        if initial_value.is_empty() {
            let _ = style.remove_property(property);
        } else {
            let _ = style.set_property_with_priority(property, &initial_value, &initial_priority);
        }
    })
}

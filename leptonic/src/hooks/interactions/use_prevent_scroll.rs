// Based on: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/overlays/src/usePreventScroll.ts
//
// Intentional deviations from react-aria:
// - DEV-I1: `disabled` is `Signal<bool>` (reactive) instead of a plain `bool`.
//   Idiomatic Leptos; react-aria achieves reactivity via `useLayoutEffect([isDisabled])`.
// - DEV-I2: Returns `UsePreventScrollReturn { props }` instead of `void`.
//   API consistency with other leptonic hooks. `UsePreventScrollAttrs` is `()`.
// - DEV-I3: Uses `thread_local! { Cell<usize> }` instead of a plain `let` variable.
//   Rust requires thread-safe globals. WASM is single-threaded so `Cell` suffices.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use leptos::prelude::*;
use leptos_use::use_window;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::hooks::IntoAttrs;
use crate::utils::platform::device;

thread_local! {
    static PREVENT_SCROLL_STATE: RefCell<PreventScrollState> = const { RefCell::new(PreventScrollState::new()) };
}

struct PreventScrollState {
    count: usize,
    restore: Option<Box<dyn FnOnce()>>,
}

impl PreventScrollState {
    const fn new() -> Self {
        Self {
            count: 0,
            restore: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UsePreventScrollInput {
    pub disabled: Signal<bool>,
}

#[derive(Debug)]
pub struct UsePreventScrollReturn {
    /// Props for the element. Call `.into_attrs()` for view spreading.
    pub props: UsePreventScrollProps,
}

/// Props from `use_prevent_scroll` that can be converted to spreadable attributes.
#[derive(Debug)]
pub struct UsePreventScrollProps;

impl IntoAttrs for UsePreventScrollProps {
    type Attrs = UsePreventScrollAttrs;

    fn into_attrs(self) -> Self::Attrs {}
}

/// These attributes must be spread onto the target element: `<foo {..attrs} />`
pub type UsePreventScrollAttrs = ();

/// Prevents scrolling on the document body on mount, and restores it on unmount.
/// Also ensures that content does not shift due to the scrollbars disappearing.
///
/// On standard browsers, sets `overflow: hidden` on the document element with
/// scrollbar width compensation (preferring `scrollbar-gutter: stable` when supported).
///
/// On iOS Safari, implements comprehensive touch event interception and focus
/// override to prevent Safari's native scrolling behavior.
pub fn use_prevent_scroll(input: UsePreventScrollInput) -> UsePreventScrollReturn {
    let UsePreventScrollInput { disabled } = input;

    let _effect = Effect::new(move |last| {
        if let Some(Some(())) = last {
            decrement_and_maybe_restore();
        }

        if disabled.get() {
            None
        } else {
            increment_and_maybe_setup();
            Some(())
        }
    });

    on_cleanup(move || {
        decrement_and_maybe_restore();
    });

    UsePreventScrollReturn {
        props: UsePreventScrollProps,
    }
}

fn increment_and_maybe_setup() {
    PREVENT_SCROLL_STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.count += 1;
        if state.count == 1 {
            state.restore = setup();
        }
    });
}

fn decrement_and_maybe_restore() {
    PREVENT_SCROLL_STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.count = state.count.saturating_sub(1);
        if state.count == 0 {
            if let Some(restore) = state.restore.take() {
                restore();
            }
        }
    });
}

fn setup() -> Option<Box<dyn FnOnce()>> {
    if device::is_ios() {
        prevent_scroll_mobile_safari()
    } else {
        prevent_scroll_standard()
    }
}

// --- Standard browser path ---

/// For most browsers, all we need to do is set `overflow: hidden` on the root element, and
/// add some padding to prevent the page from shifting when the scrollbar is hidden.
fn prevent_scroll_standard() -> Option<Box<dyn FnOnce()>> {
    let window = use_window();
    let window = window.as_ref()?;
    let doc = window.document()?;
    let root: HtmlElement = doc.document_element()?.unchecked_into();

    let scrollbar_width = window
        .inner_width()
        .ok()
        .and_then(|w| w.as_f64())
        .unwrap_or(0.0)
        - f64::from(root.client_width());

    let mut reverts: Vec<Box<dyn FnOnce()>> = Vec::new();

    if scrollbar_width > 0.0 {
        // Use scrollbar-gutter when supported because it also works for fixed positioned elements.
        if js_sys::Reflect::has(&root.style(), &"scrollbarGutter".into()).unwrap_or(false) {
            reverts.push(set_style(&root, "scrollbar-gutter", "stable"));
        } else {
            reverts.push(set_style(
                &root,
                "padding-right",
                &format!("{scrollbar_width}px"),
            ));
        }
    }
    reverts.push(set_style(&root, "overflow", "hidden"));

    Some(Box::new(move || {
        for f in reverts {
            f();
        }
    }))
}

// --- iOS Safari path ---
//
// Why this is needed:
//
// On standard browsers, `overflow: hidden` on the document root reliably prevents all scrolling.
// iOS Safari ignores `overflow: hidden` in several situations, making a separate code path necessary:
//
// 1. When the bottom toolbar and address bar are collapsed, Safari always allows page scrolling
//    regardless of overflow: hidden.
// 2. When the software keyboard appears, Safari doesn't resize the viewport — it covers part of it,
//    making the page scrollable behind the keyboard.
// 3. Tapping an input causes Safari to forcibly scroll the entire page to center the input in the
//    visual viewport, even moving fixed-position elements off-screen.
// 4. The next/previous buttons in the keyboard toolbar scroll the whole page to the next input,
//    even when that input is inside a nested scrollable container that could handle the scroll.
//
// How we work around it:
//
// 1. Intercept `touchmove` and call `preventDefault()` when the touch target isn't in a scrollable
//    container. This prevents touch scrolling on the window. The handler is careful to still allow
//    pinch-zoom (2+ touches), text selection dragging, range input sliding, and input selection
//    handle adjustment.
// 2. Inject `overscroll-behavior: contain` via a `<style>` element so nested scrollable regions
//    don't chain-scroll to the page when they reach their scroll boundary. Work around a WebKit bug
//    (https://bugs.webkit.org/show_bug.cgi?id=243452) where this doesn't work when an element
//    doesn't actually overflow, by also preventing default in the `touchmove` handler.
// 3. Override `HTMLElement.prototype.focus` to always pass `preventScroll: true`, then manually
//    scroll the focused element into view using `visualViewport`-aware centering logic. This
//    prevents Safari's native page-scroll-on-focus behavior (issues 2, 3, 4 above).
// 4. Intercept `blur` events to catch focus transitions to keyboard-opening elements and apply
//    the same preventScroll + manual scroll-into-view approach. Also handles the iOS "Done" button
//    case where focus moves to the body with no related target.
fn prevent_scroll_mobile_safari() -> Option<Box<dyn FnOnce()>> {
    let window = use_window();
    let window = window.as_ref()?;
    let doc = window.document()?;

    let mut cleanups: Vec<Box<dyn FnOnce()>> = Vec::new();

    // Inject <style> with overscroll-behavior: contain.
    // Prevent scrolling up when at the top and scrolling down when at the bottom
    // of a nested scrollable area, otherwise mobile Safari will start scrolling
    // the window instead.
    // This must be applied before the touchstart event as of iOS 26, so inject it as a <style> element.
    let style_el: web_sys::HtmlStyleElement = doc.create_element("style").ok()?.unchecked_into();
    style_el.set_text_content(Some(
        "@layer {\n  * {\n    overscroll-behavior: contain;\n  }\n}",
    ));
    if let Some(head) = doc.head() {
        let _ = head.prepend_with_node_1(&style_el);
        let style_el_clone = style_el.clone();
        cleanups.push(Box::new(move || {
            style_el_clone.remove();
        }));
    }

    // Shared state between touchstart and touchmove handlers.
    let scrollable: Rc<RefCell<Option<web_sys::Element>>> = Rc::new(RefCell::new(None));
    let allow_touch_move: Rc<Cell<bool>> = Rc::new(Cell::new(false));

    // touchstart handler: find nearest scrollable parent and check for special cases.
    let scrollable_ts = Rc::clone(&scrollable);
    let allow_touch_move_ts = Rc::clone(&allow_touch_move);
    let on_touch_start =
        Closure::<dyn FnMut(web_sys::TouchEvent)>::wrap(Box::new(move |e: web_sys::TouchEvent| {
            // Store the nearest scrollable parent element from the element that the user touched.
            let Some(target) = crate::utils::shadow_dom::get_event_target(&e) else {
                return;
            };
            let Some(target_el) = target.dyn_ref::<web_sys::Element>().cloned() else {
                return;
            };

            let parent = if crate::utils::scroll::is_scrollable(&target_el, false) {
                target_el.clone()
            } else {
                crate::utils::scroll::get_scroll_parent(&target_el, true)
            };
            *scrollable_ts.borrow_mut() = Some(parent);
            allow_touch_move_ts.set(false);

            // If the target is selected, don't preventDefault in touchmove to allow user to adjust selection.
            if let Some(window) = target_el.owner_document().and_then(|d| d.default_view()) {
                if let Ok(Some(selection)) = window.get_selection() {
                    if !selection.is_collapsed()
                        && selection
                            .contains_node_with_allow_partial_containment(&target_el, true)
                            .unwrap_or(false)
                    {
                        allow_touch_move_ts.set(true);
                    }
                }
            }

            // If this is a range input, allow touch move to allow user to adjust the slider value.
            let composed_path = e.composed_path();
            for i in 0..composed_path.length() {
                let item = composed_path.get(i);
                if let Some(input) = item.dyn_ref::<web_sys::HtmlInputElement>() {
                    if input.type_() == "range" {
                        allow_touch_move_ts.set(true);
                        break;
                    }
                }
            }

            // If this is a focused input element with a selected range, allow user to drag the selection handles.
            if let Some(input) = target_el.dyn_ref::<web_sys::HtmlInputElement>() {
                if let (Ok(Some(start)), Ok(Some(end))) =
                    (input.selection_start(), input.selection_end())
                {
                    if start < end {
                        if let Some(doc) = target_el.owner_document() {
                            if doc.active_element().as_ref() == Some(&target_el) {
                                allow_touch_move_ts.set(true);
                            }
                        }
                    }
                }
            } else if let Some(textarea) = target_el.dyn_ref::<web_sys::HtmlTextAreaElement>() {
                if let (Ok(Some(start)), Ok(Some(end))) =
                    (textarea.selection_start(), textarea.selection_end())
                {
                    if start < end {
                        if let Some(doc) = target_el.owner_document() {
                            if doc.active_element().as_ref() == Some(&target_el) {
                                allow_touch_move_ts.set(true);
                            }
                        }
                    }
                }
            }
        }));

    // touchmove handler: prevent scrolling the window.
    let scrollable_tm = Rc::clone(&scrollable);
    let allow_touch_move_tm = Rc::clone(&allow_touch_move);
    let on_touch_move =
        Closure::<dyn FnMut(web_sys::TouchEvent)>::wrap(Box::new(move |e: web_sys::TouchEvent| {
            // Allow pinch-zooming.
            if e.touches().length() >= 2 || allow_touch_move_tm.get() {
                return;
            }

            let scrollable_ref = scrollable_tm.borrow();
            let scrollable = scrollable_ref.as_ref();

            // Prevent scrolling the window.
            let is_root = scrollable.is_none_or(|s| {
                s.owner_document()
                    .and_then(|d| d.document_element())
                    .as_ref()
                    == Some(s)
                    || s.owner_document()
                        .and_then(|d| d.body().map(web_sys::Element::from))
                        .as_ref()
                        == Some(s)
            });

            if is_root {
                e.prevent_default();
                return;
            }

            // overscroll-behavior should prevent scroll chaining, but currently does not
            // if the element doesn't actually overflow. https://bugs.webkit.org/show_bug.cgi?id=243452
            // This checks that both the width and height do not overflow, otherwise we might
            // block horizontal scrolling too. In that case, adding `touch-action: pan-x` to
            // the element will prevent vertical page scrolling. We can't add that automatically
            // because it must be set before the touchstart event.
            if let Some(s) = scrollable {
                if s.scroll_height() == s.client_height() && s.scroll_width() == s.client_width() {
                    e.prevent_default();
                }
            }
        }));

    // blur handler: intercept focus transitions to keyboard-opening elements.
    let on_blur =
        Closure::<dyn FnMut(web_sys::FocusEvent)>::wrap(Box::new(move |e: web_sys::FocusEvent| {
            let Some(target) = e.target() else {
                return;
            };
            let target_el: Option<&web_sys::Element> = target.dyn_ref();
            let related_target: Option<web_sys::HtmlElement> =
                e.related_target().and_then(|rt| rt.dyn_into().ok());

            if let Some(ref related) = related_target {
                let related_el: &web_sys::Element = related.as_ref();
                if crate::utils::focusability::will_open_keyboard(related_el) {
                    // Focus without scrolling the whole page, and then scroll into view manually.
                    let opts = web_sys::FocusOptions::new();
                    opts.set_prevent_scroll(true);
                    related.focus_with_options(&opts).ok();
                    let was_keyboard_visible =
                        target_el.is_some_and(crate::utils::focusability::will_open_keyboard);
                    scroll_into_view_when_ready(related.clone(), was_keyboard_visible);
                }
            } else {
                // When tapping the Done button on the keyboard, focus moves to the body.
                // FocusScope will then restore focus back to the input. Later when tapping
                // the same input again, it is already focused, so no blur event will fire,
                // resulting in the flow above never running and Safari's native scrolling occurring.
                // Instead, move focus to the parent focusable element (e.g. the dialog).
                if let Some(target_el) = target_el {
                    if let Some(parent) = target_el.parent_element() {
                        if let Ok(Some(focusable)) = parent.closest("[tabindex]") {
                            let focusable: HtmlElement = focusable.unchecked_into();
                            let opts = web_sys::FocusOptions::new();
                            opts.set_prevent_scroll(true);
                            focusable.focus_with_options(&opts).ok();
                        }
                    }
                }
            }
        }));

    // Register touch listeners with passive: false (critical for preventDefault to work on iOS).
    let doc_target: &web_sys::EventTarget = doc.as_ref();

    let touch_start_options = web_sys::AddEventListenerOptions::new();
    touch_start_options.set_capture(true);
    touch_start_options.set_passive(false);

    let touch_move_options = web_sys::AddEventListenerOptions::new();
    touch_move_options.set_capture(true);
    touch_move_options.set_passive(false);

    let _ = doc_target.add_event_listener_with_callback_and_add_event_listener_options(
        "touchstart",
        on_touch_start.as_ref().unchecked_ref(),
        &touch_start_options,
    );
    let _ = doc_target.add_event_listener_with_callback_and_add_event_listener_options(
        "touchmove",
        on_touch_move.as_ref().unchecked_ref(),
        &touch_move_options,
    );

    // blur uses capture: true
    let blur_options = web_sys::AddEventListenerOptions::new();
    blur_options.set_capture(true);

    let _ = doc_target.add_event_listener_with_callback_and_add_event_listener_options(
        "blur",
        on_blur.as_ref().unchecked_ref(),
        &blur_options,
    );

    // Store JS function references for cleanup.
    let ts_fn: js_sys::Function = on_touch_start
        .as_ref()
        .unchecked_ref::<js_sys::Function>()
        .clone();
    let tm_fn: js_sys::Function = on_touch_move
        .as_ref()
        .unchecked_ref::<js_sys::Function>()
        .clone();
    let blur_fn: js_sys::Function = on_blur.as_ref().unchecked_ref::<js_sys::Function>().clone();
    let doc_target_clone = doc_target.clone();

    // Keep closures alive by moving them into a Box stored in cleanups.
    let _closures: Vec<Box<dyn std::any::Any>> = vec![
        Box::new(on_touch_start),
        Box::new(on_touch_move),
        Box::new(on_blur),
    ];

    // Override HTMLElement.prototype.focus to prevent Safari's native scrolling.
    let focus_restore = setup_focus_override(window);

    cleanups.push(Box::new(move || {
        let _ = doc_target_clone.remove_event_listener_with_callback_and_bool(
            "touchstart",
            &ts_fn,
            true,
        );
        let _ = doc_target_clone.remove_event_listener_with_callback_and_bool(
            "touchmove",
            &tm_fn,
            true,
        );
        let _ =
            doc_target_clone.remove_event_listener_with_callback_and_bool("blur", &blur_fn, true);
        // Drop the closures so they can be GC'd.
        drop(_closures);
    }));

    if let Some(restore_focus) = focus_restore {
        cleanups.push(restore_focus);
    }

    Some(Box::new(move || {
        for f in cleanups {
            f();
        }
    }))
}

// --- Focus override for iOS ---

/// Override `HTMLElement.prototype.focus` to always use `preventScroll: true` and
/// then manually scroll the focused element into view.
///
/// Returns a cleanup function that restores the original focus method.
fn setup_focus_override(window: &web_sys::Window) -> Option<Box<dyn FnOnce()>> {
    use js_sys::{Function, Object, Reflect};

    let window_js: &JsValue = window.as_ref();

    let html_ctor = Reflect::get(window_js, &"HTMLElement".into()).ok()?;
    let prototype = Reflect::get(&html_ctor, &"prototype".into()).ok()?;

    let original_focus = Reflect::get(&prototype, &"focus".into()).ok()?;
    if !original_focus.is_function() {
        return None;
    }

    // Store the original focus + a Rust-side scroll-into-view callback on a holder object.
    let scroll_closure = Closure::<dyn FnMut(JsValue, bool)>::wrap(Box::new(
        |el: JsValue, was_keyboard_visible: bool| {
            if let Some(html_el) = el.dyn_ref::<HtmlElement>() {
                scroll_into_view_when_ready(html_el.clone(), was_keyboard_visible);
            }
        },
    ));

    let holder = Object::new();
    let _ = Reflect::set(&holder, &"orig".into(), &original_focus);
    let _ = Reflect::set(&holder, &"scrollIntoView".into(), scroll_closure.as_ref());
    scroll_closure.forget();

    let _ = Reflect::set(
        &prototype,
        &"__leptonic_prevent_scroll_focus".into(),
        &holder,
    );

    // The wrapper function:
    // 1. Determines if the keyboard was already visible (by checking if the active element opens keyboard).
    // 2. Calls the original focus with preventScroll: true merged into options.
    // 3. If the caller did NOT request preventScroll, manually scrolls into view.
    let wrapper = Function::new_no_args(
        "\
        var h = HTMLElement.prototype.__leptonic_prevent_scroll_focus; \
        if (!h || !h.orig) { return; } \
        var activeEl = document.activeElement; \
        var wasKbdVisible = false; \
        if (activeEl) { \
            var tag = activeEl.tagName; \
            var type = activeEl.type; \
            wasKbdVisible = (tag === 'TEXTAREA' || activeEl.isContentEditable || \
                (tag === 'INPUT' && type !== 'checkbox' && type !== 'radio' && \
                 type !== 'range' && type !== 'color' && type !== 'file' && \
                 type !== 'image' && type !== 'button' && type !== 'submit' && \
                 type !== 'reset')); \
        } \
        var opts = arguments.length > 0 && typeof arguments[0] === 'object' ? \
            Object.assign({}, arguments[0]) : {}; \
        var callerWantedPreventScroll = opts.preventScroll; \
        opts.preventScroll = true; \
        h.orig.call(this, opts); \
        if (!callerWantedPreventScroll) { \
            h.scrollIntoView(this, wasKbdVisible); \
        }",
    );

    let _ = Reflect::set(&prototype, &"focus".into(), &wrapper);

    let window_clone = window_js.clone();
    Some(Box::new(move || {
        restore_focus_override(&window_clone, &original_focus);
    }))
}

/// Restore `HTMLElement.prototype.focus` for a window.
fn restore_focus_override(window_js: &JsValue, original_focus: &JsValue) {
    use js_sys::Reflect;

    if original_focus.is_undefined() || original_focus.is_null() {
        return;
    }

    let html_ctor = Reflect::get(window_js, &"HTMLElement".into()).ok();
    let prototype = html_ctor.and_then(|c| Reflect::get(&c, &"prototype".into()).ok());
    if let Some(prototype) = prototype {
        let _ = Reflect::set(&prototype, &"focus".into(), original_focus);
        let _ = Reflect::delete_property(
            prototype.unchecked_ref::<js_sys::Object>(),
            &"__leptonic_prevent_scroll_focus".into(),
        );
    }
}

// --- Scroll-into-view helpers for iOS ---

fn scroll_into_view_when_ready(target: HtmlElement, was_keyboard_visible: bool) {
    let Some(window) = use_window().as_ref().cloned() else {
        return;
    };

    if was_keyboard_visible || window.visual_viewport().is_none() {
        // If the keyboard was already visible, scroll the target into view immediately.
        scroll_into_view_centered(&target);
    } else if let Some(vv) = window.visual_viewport() {
        // Otherwise, wait for the visual viewport to resize before scrolling so we can
        // measure the correct position to scroll to.
        let target_clone = target.clone();
        let resize_cb = Closure::<dyn FnMut()>::once(move || {
            scroll_into_view_centered(&target_clone);
        });
        let vv_target: &web_sys::EventTarget = vv.as_ref();
        let opts = web_sys::AddEventListenerOptions::new();
        opts.set_once(true);
        let _ = vv_target.add_event_listener_with_callback_and_add_event_listener_options(
            "resize",
            resize_cb.as_ref().unchecked_ref(),
            &opts,
        );
        resize_cb.forget();
    }
}

/// Walk up the scroll parent chain and center the target within each scrollable ancestor.
fn scroll_into_view_centered(target: &HtmlElement) {
    let Some(window) = use_window().as_ref().cloned() else {
        return;
    };
    let target_el: &web_sys::Element = target.as_ref();

    let root = target_el
        .owner_document()
        .and_then(|d| d.scrolling_element().or_else(|| d.document_element()));

    let mut next_target: Option<web_sys::Element> = Some(target_el.clone());

    while let Some(ref current) = next_target {
        if root.as_ref() == Some(current) {
            break;
        }

        // Find the parent scrollable element.
        let scrollable = crate::utils::scroll::get_scroll_parent(current, false);

        // Skip if scrollable is document root/body or the element itself.
        let is_doc_level = current
            .owner_document()
            .and_then(|d| d.document_element())
            .as_ref()
            == Some(&scrollable)
            || current
                .owner_document()
                .and_then(|d| d.body().map(web_sys::Element::from))
                .as_ref()
                == Some(&scrollable)
            || &scrollable == current;

        if is_doc_level {
            next_target = scrollable.parent_element();
            continue;
        }

        let scrollable_rect = scrollable.get_bounding_client_rect();
        let target_rect = current.get_bounding_client_rect();

        if target_rect.top() < scrollable_rect.top()
            || target_rect.bottom() > scrollable_rect.top() + f64::from(current.client_height())
        {
            let mut bottom = scrollable_rect.bottom();

            // Account for the visual viewport if the keyboard is covering part of the screen.
            if let Some(vv) = window.visual_viewport() {
                bottom = bottom.min(vv.offset_top() + vv.height());
            }

            // Center within the viewport.
            let adjustment = (target_rect.top() - scrollable_rect.top())
                - ((bottom - scrollable_rect.top()) / 2.0 - target_rect.height() / 2.0);

            let scroll_top = scrollable.scroll_top() as f64 + adjustment;
            let max_scroll = f64::from(scrollable.scroll_height() - scrollable.client_height());

            let opts = web_sys::ScrollToOptions::new();
            opts.set_top(scroll_top.max(0.0).min(max_scroll));
            opts.set_behavior(web_sys::ScrollBehavior::Smooth);
            scrollable.scroll_to_with_scroll_to_options(&opts);
        }

        next_target = scrollable.parent_element();
    }
}

// --- Helpers ---

/// Sets a CSS property on an element, returning a closure that restores the original value.
fn set_style(element: &HtmlElement, property: &str, value: &str) -> Box<dyn FnOnce()> {
    let style = element.style();
    let original = style.get_property_value(property).unwrap_or_default();
    let _ = style.set_property(property, value);
    let prop = property.to_owned();
    let element = element.clone();
    Box::new(move || {
        let el_style = element.style();
        if original.is_empty() {
            let _ = el_style.remove_property(&prop);
        } else {
            let _ = el_style.set_property(&prop, &original);
        }
    })
}

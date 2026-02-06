use leptos::ev;
use leptos::prelude::*;
use leptos_use::{use_document, use_event_listener, use_window};
use std::rc::Rc;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Event, KeyboardEvent, MouseEvent, PointerEvent};

use crate::contexts::WasmClosure;
use crate::{
    components::{
        modal::ModalRoot, popover::PopoverRoot, prelude::ToastRoot, theme::Theme,
        theme::ThemeProvider,
    },
    contexts::{
        global_click_event::GlobalClickEvent,
        global_keyboard_event::GlobalKeyboardEvent,
        global_mouseup_event::GlobalMouseupEvent,
        global_pointer_event::{
            GlobalPointerCancelEvent, GlobalPointerDownEvent, GlobalPointerMoveEvent,
            GlobalPointerUpEvent,
        },
        global_resize_event::GlobalResizeEvent,
        global_scroll_event::GlobalScrollEvent,
    },
    signal_ls,
};

/// Leptonic's root context. Always available in components under <Root>.
#[derive(Debug, Clone, Copy)]
pub struct Leptonic {
    /// Whether the users device should be considered 'mobile'.
    /// Please read: <https://developer.mozilla.org/en-US/docs/Web/HTTP/Browser_detection_using_the_user_agent>
    /// and prefer other detection methods for selective functionality or styling.
    pub is_mobile_device: Signal<bool>,

    /// Always provides the inverse of `is_mobile_device`.
    pub is_desktop_device: Signal<bool>,
}

#[component]
#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn Root<T>(
    /// Root directory of JS files used for dynamic script imports. Defaults to "js", as this is commonly used.
    /// Change this if you chose a non-standard location for `[package.metadata.leptonic] > js-dir`.
    #[allow(unused_variables)]
    #[prop(into, default = Oco::Borrowed("js"))]
    runtime_js_dir: Oco<'static, str>,

    default_theme: T,

    children: Children,
) -> impl IntoView
where
    T: Theme + 'static,
{
    if let Some(_root_context) = use_context::<Leptonic>() {
        tracing::warn!("The <Root> component must only be used once! Detected that <Root> was rendered when it was already rendered higher up the stack. Remove this usage.");
    }

    let win = use_window();
    let doc = use_document();

    // KEY DOWN
    let (g_keyboard_event, set_g_keyboard_event) = signal_local::<Option<KeyboardEvent>>(None);
    let mut onkeydown: WasmClosure<KeyboardEvent> = None;
    if let Some(doc) = &*doc {
        let boxed: Box<dyn FnMut(KeyboardEvent)> =
            Box::new(move |e| set_g_keyboard_event.set(Some(e)));
        let closure = Closure::wrap(boxed);
        doc.set_onkeydown(Some(closure.as_ref().unchecked_ref()));
        onkeydown = Some(Rc::new(Box::new(closure)));
    }

    StoredValue::new_local(onkeydown);
    provide_context(GlobalKeyboardEvent::new(
        g_keyboard_event,
        set_g_keyboard_event,
    ));

    // POINTER DOWN
    let (g_pointer_down_event, set_g_pointer_down_event) =
        signal_local::<Option<PointerEvent>>(None);
    let mut on_pointer_down: WasmClosure<PointerEvent> = None;
    if let Some(doc) = &*doc {
        let boxed: Box<dyn FnMut(PointerEvent)> =
            Box::new(move |e| set_g_pointer_down_event.set(Some(e)));
        let closure = Closure::wrap(boxed);
        doc.set_onpointerdown(Some(closure.as_ref().unchecked_ref()));
        on_pointer_down = Some(Rc::new(Box::new(closure)));
    }

    StoredValue::new_local(on_pointer_down);
    provide_context(GlobalPointerDownEvent::new(
        g_pointer_down_event,
        set_g_pointer_down_event,
    ));

    // POINTER UP
    let (g_pointer_up_event, set_g_pointer_up_event) = signal_local::<Option<PointerEvent>>(None);
    let mut on_pointer_up: WasmClosure<PointerEvent> = None;
    if let Some(doc) = &*doc {
        let boxed: Box<dyn FnMut(PointerEvent)> =
            Box::new(move |e| set_g_pointer_up_event.set(Some(e)));
        let closure = Closure::wrap(boxed);
        doc.set_onpointerup(Some(closure.as_ref().unchecked_ref()));
        on_pointer_up = Some(Rc::new(Box::new(closure)));
    }

    StoredValue::new_local(on_pointer_up);
    provide_context(GlobalPointerUpEvent::new(
        g_pointer_up_event,
        set_g_pointer_up_event,
    ));

    // POINTER CANCEL
    let (g_pointer_cancel_event, set_g_pointer_cancel_event) =
        signal_local::<Option<PointerEvent>>(None);
    let mut on_pointer_cancel: WasmClosure<PointerEvent> = None;
    if let Some(doc) = &*doc {
        let boxed: Box<dyn FnMut(PointerEvent)> =
            Box::new(move |e| set_g_pointer_cancel_event.set(Some(e)));
        let closure = Closure::wrap(boxed);
        doc.set_onpointercancel(Some(closure.as_ref().unchecked_ref()));
        on_pointer_cancel = Some(Rc::new(Box::new(closure)));
    }

    StoredValue::new_local(on_pointer_cancel);
    provide_context(GlobalPointerCancelEvent::new(
        g_pointer_cancel_event,
        set_g_pointer_cancel_event,
    ));

    // POINTER MOVE
    let (g_pointer_move_event, set_g_pointer_move_event) =
        signal_local::<Option<PointerEvent>>(None);
    let mut on_pointer_move: WasmClosure<PointerEvent> = None;
    if let Some(doc) = &*doc {
        let boxed: Box<dyn FnMut(PointerEvent)> =
            Box::new(move |e| set_g_pointer_move_event.set(Some(e)));
        let closure = Closure::wrap(boxed);
        doc.set_onpointermove(Some(closure.as_ref().unchecked_ref()));
        on_pointer_move = Some(Rc::new(Box::new(closure)));
    }

    StoredValue::new_local(on_pointer_move);
    provide_context(GlobalPointerMoveEvent::new(
        g_pointer_move_event,
        set_g_pointer_move_event,
    ));

    // CLICK
    let (g_click_event, set_g_click_event) = signal_local::<Option<MouseEvent>>(None);
    let mut onclick: WasmClosure<MouseEvent> = None;
    if let Some(doc) = &*doc {
        let boxed: Box<dyn FnMut(MouseEvent)> = Box::new(move |e| set_g_click_event.set(Some(e)));
        let closure = Closure::wrap(boxed);
        doc.set_onclick(Some(closure.as_ref().unchecked_ref()));
        onclick = Some(Rc::new(Box::new(closure)));
    }

    StoredValue::new_local(onclick);
    provide_context(GlobalClickEvent::new(g_click_event, set_g_click_event));

    // MOUSE UP - data currently not needed
    let (g_mouseup_event, set_g_mouseup_event) = signal_local::<Option<MouseEvent>>(None);
    let mut onmouseup: WasmClosure<MouseEvent> = None;
    if let Some(doc) = &*doc {
        let boxed: Box<dyn FnMut(MouseEvent)> = Box::new(move |e| set_g_mouseup_event.set(Some(e)));
        let closure = Closure::wrap(boxed);
        doc.set_onmouseup(Some(closure.as_ref().unchecked_ref()));
        onmouseup = Some(Rc::new(Box::new(closure)));
    }

    StoredValue::new_local(onmouseup);
    provide_context(GlobalMouseupEvent::new(
        g_mouseup_event,
        set_g_mouseup_event,
    ));

    // RESIZE
    let (g_resize_event, set_g_resize_event) = signal_local::<Option<Event>>(None);
    let mut onresize: WasmClosure<Event> = None;
    if let Some(win) = &*win {
        let boxed: Box<dyn FnMut(Event)> = Box::new(move |e| set_g_resize_event.set(Some(e)));
        let closure = Closure::wrap(boxed);
        win.set_onresize(Some(closure.as_ref().unchecked_ref()));
        onresize = Some(Rc::new(Box::new(closure)));
    }

    StoredValue::new_local(onresize);
    provide_context(GlobalResizeEvent::new(g_resize_event, set_g_resize_event));

    // SCROLL
    let (g_scroll_event, set_g_scroll_event) = signal_local::<Option<Event>>(None);
    let mut onscroll: WasmClosure<Event> = None;
    if let Some(doc) = &*doc {
        let boxed: Box<dyn FnMut(Event)> = Box::new(move |e| set_g_scroll_event.set(Some(e)));
        let closure = Closure::wrap(boxed);
        doc.set_onscroll(Some(closure.as_ref().unchecked_ref()));
        onscroll = Some(Rc::new(Box::new(closure)));
    }

    StoredValue::new_local(onscroll);
    provide_context(GlobalScrollEvent::new(g_scroll_event, set_g_scroll_event));

    let update_vh = move || {
        #[derive(Debug)]
        enum Error {
            InnerHeightIndeterminable,
            InnerHeightNotNumber,
            DocumentIndeterminable,
            SetPropertyFailed,
        }
        if let Some(window) = &*use_window() {
            let inner_height = window
                .inner_height()
                .map_err(|_err| Error::InnerHeightIndeterminable)?;
            let inner_height = inner_height.as_f64().ok_or(Error::InnerHeightNotNumber)?;
            if let Some(document) = &*use_document() {
                document
                    .document_element()
                    .ok_or(Error::DocumentIndeterminable)?
                    .unchecked_into::<web_sys::HtmlElement>()
                    .style()
                    .set_property("--leptonic-vh", format!("{inner_height}px").as_str())
                    .map_err(|_err| Error::SetPropertyFailed)?;
            }
        }
        Result::<(), Error>::Ok(())
    };

    if let Err(err) = update_vh() {
        tracing::warn!(?err, "Could not calculate real viewport height");
    }

    if let Some(win) = &*win {
        let _cleanup = use_event_listener(win.clone(), ev::resize, move |_e| {
            if let Err(err) = update_vh() {
                tracing::warn!(?err, "Could not calculate real viewport height");
            }
        });
    }

    // Reference: https://developer.mozilla.org/en-US/docs/Web/HTTP/Browser_detection_using_the_user_agent
    let is_mobile_device = Signal::derive(move || {
        use_window()
            .as_ref()
            .is_some_and(|window| {
                window
                    .navigator()
                    .user_agent()
                    .map(|agent| agent.to_lowercase().contains("mobi"))
                    .unwrap_or(false)
            })
    });

    provide_context(Leptonic {
        is_mobile_device,
        is_desktop_device: Signal::derive(move || !is_mobile_device.get()),
    });

    cfg_if::cfg_if! { if #[cfg(feature="tiptap")] {
        use leptos_meta::Script;
        let tiptap_js_module_includes = view! {
            <Script type_="module" src=format!("/{}/tiptap-bundle.min.js", runtime_js_dir)/>
            <Script type_="module" src=format!("/{}/tiptap.js", runtime_js_dir)/>
        };
    } else {
        let tiptap_js_module_includes = ();
    }}

    view! {
        { tiptap_js_module_includes }

        <ThemeProvider theme=signal_ls("theme", default_theme)>
            <PopoverRoot>
                <ToastRoot>
                    <ModalRoot>
                        { children() }
                    </ModalRoot>
                </ToastRoot>
            </PopoverRoot>
        </ThemeProvider>
    }
}

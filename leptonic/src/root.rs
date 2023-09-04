use std::rc::Rc;

use leptos::*;
use leptos_use::use_event_listener;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Event, KeyboardEvent, MouseEvent};

use crate::{
    contexts::{
        global_click_event::GlobalClickEvent, global_keyboard_event::GlobalKeyboardEvent,
        global_mouseup_event::GlobalMouseupEvent, global_resize_event::GlobalResizeEvent,
        global_scroll_event::GlobalScrollEvent,
    },
    prelude::*,
};

/// Leptonic's root context. Always available in components under <Root>.
#[derive(Debug, Clone)]
pub struct Leptonic {
    /// Whether or not the users device should be considered 'mobile'.
    /// Please read: https://developer.mozilla.org/en-US/docs/Web/HTTP/Browser_detection_using_the_user_agent
    /// and prefer other detection methods for selective functionality or styling.
    pub is_mobile_device: Signal<bool>,

    /// Always provides the inverse of `is_mobile_device`.
    pub is_desktop_device: Signal<bool>,
}

#[component]
pub fn Root<T>(default_theme: T, children: Children) -> impl IntoView
where
    T: Theme + 'static,
{
    if let Some(_root_context) = use_context::<Leptonic>() {
        tracing::warn!("The <Root> component must only be used once! Detected that <Root> was rendered when it was already rendered higher up the stack. Remove this usage.");
    }

    let win = window();
    let doc = document();

    // KEY DOWN
    let (g_keyboard_event, set_g_keyboard_event) = create_signal::<Option<KeyboardEvent>>(None);
    let onkeydown = Closure::wrap(
        Box::new(move |e| set_g_keyboard_event.set(Some(e))) as Box<dyn FnMut(KeyboardEvent)>
    );
    doc.set_onkeydown(Some(onkeydown.as_ref().unchecked_ref()));
    provide_context(GlobalKeyboardEvent::new(
        Rc::new(Box::new(onkeydown)),
        g_keyboard_event,
        set_g_keyboard_event,
    ));

    // CLICK
    let (g_click_event, set_g_click_event) = create_signal::<Option<MouseEvent>>(None);
    let onclick = Closure::wrap(
        Box::new(move |e| set_g_click_event.set(Some(e))) as Box<dyn FnMut(MouseEvent)>
    );
    doc.set_onclick(Some(onclick.as_ref().unchecked_ref()));
    provide_context(GlobalClickEvent::new(
        Rc::new(Box::new(onclick)),
        g_click_event,
        set_g_click_event,
    ));

    // MOUSE UP
    let (g_mouseup_event, set_g_mouseup_event) = create_signal::<Option<MouseEvent>>(None);
    let onmouseup = Closure::wrap(
        Box::new(move |e| set_g_mouseup_event.set(Some(e))) as Box<dyn FnMut(MouseEvent)>
    );
    doc.set_onmouseup(Some(onmouseup.as_ref().unchecked_ref()));
    provide_context(GlobalMouseupEvent::new(
        Rc::new(Box::new(onmouseup)),
        g_mouseup_event,
        set_g_mouseup_event,
    ));

    // RESIZE
    let (g_resize_event, set_g_resize_event) = create_signal::<Option<Event>>(None);
    let onresize =
        Closure::wrap(Box::new(move |e| set_g_resize_event.set(Some(e))) as Box<dyn FnMut(Event)>);
    win.set_onresize(Some(onresize.as_ref().unchecked_ref()));
    provide_context(GlobalResizeEvent::new(
        Rc::new(Box::new(onresize)),
        g_resize_event,
        set_g_resize_event,
    ));

    // SCROLL
    let (g_scroll_event, set_g_scroll_event) = create_signal::<Option<Event>>(None);
    let onscroll =
        Closure::wrap(Box::new(move |e| set_g_scroll_event.set(Some(e))) as Box<dyn FnMut(Event)>);
    doc.set_onscroll(Some(onscroll.as_ref().unchecked_ref()));
    provide_context(GlobalScrollEvent::new(
        Rc::new(Box::new(onscroll)),
        g_scroll_event,
        set_g_scroll_event,
    ));

    let update_vh = move || {
        #[derive(Debug)]
        enum Error {
            InnerHeightIndeterminable,
            InnerHeightNotNumber,
            DocumentIndeterminable,
            SetPropertyFailed,
        }
        let inner_height = window()
            .inner_height()
            .map_err(|_| Error::InnerHeightIndeterminable)?;
        let inner_height = inner_height.as_f64().ok_or(Error::InnerHeightNotNumber)?;
        document()
            .document_element()
            .ok_or(Error::DocumentIndeterminable)?
            .unchecked_into::<web_sys::HtmlElement>()
            .style()
            .set_property("--leptonic-vh", format!("{inner_height}px").as_str())
            .map_err(|_| Error::SetPropertyFailed)?;
        std::result::Result::<(), Error>::Ok(())
    };

    if let Err(err) = update_vh() {
        tracing::warn!(?err, "Could not calculate real viewport height");
    }

    let _cleanup = use_event_listener(window(), leptos::ev::resize, move |_e| {
        if let Err(err) = update_vh() {
            tracing::warn!(?err, "Could not calculate real viewport height");
        }
    });

    // Reference: https://developer.mozilla.org/en-US/docs/Web/HTTP/Browser_detection_using_the_user_agent
    let (is_mobile_device, _set_is_mobile_device) = create_signal(
        window()
            .navigator()
            .user_agent()
            .unwrap()
            .to_lowercase()
            .contains("mobi"),
    );

    // Adding this context also serves the check at the start of this component!
    provide_context(Leptonic {
        is_mobile_device: is_mobile_device.into(),
        is_desktop_device: Signal::derive(move || !is_mobile_device.get()),
    });

    // NOTE: --leptonic-vh can be used like this in CSS code: height: var(--leptonic-vh, 100vh);

    view! {
        <ThemeProvider theme=create_signal_ls("theme", default_theme)>
            <ToastRoot>
                <ModalRoot>
                    <Box style="min-height: 100%; min-width: 100%; display: flex; flex-direction: column;">
                        { children() }
                    </Box>
                </ModalRoot>
            </ToastRoot>
        </ThemeProvider>
    }
}

use std::rc::Rc;

use leptos::*;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{KeyboardEvent, MouseEvent};

use crate::{
    contexts::{global_keyboard_event::GlobalKeyboardEvent, global_click_event::GlobalClickEvent, global_mouseup_event::GlobalMouseupEvent},
    prelude::*,
};

#[derive(Debug, Clone)]
struct RootContext {}

#[component]
pub fn Root<T>(cx: Scope, default_theme: T, children: Children) -> impl IntoView
where
    T: Theme + 'static,
{
    if let Some(_root_context) = use_context::<RootContext>(cx) {
        tracing::warn!("The <Root> component must only be used once! Detected that <Root> was rendered when it was already rendered higher up the stack. Remove this usage.");
    }
    provide_context(cx, RootContext {});

    let doc = document();

    // KEY DOWN
    let (g_keyboard_event, set_g_keyboard_event) = create_signal::<Option<KeyboardEvent>>(cx, None);
    let onkeydown = Closure::wrap(
        Box::new(move |e| set_g_keyboard_event.set(Some(e))) as Box<dyn FnMut(KeyboardEvent)>
    );
    doc.set_onkeydown(Some(onkeydown.as_ref().unchecked_ref()));
    provide_context(
        cx,
        GlobalKeyboardEvent::new(
            Rc::new(Box::new(onkeydown)),
            g_keyboard_event,
            set_g_keyboard_event,
        ),
    );

    // CLICK
    let (g_click_event, set_g_click_event) = create_signal::<Option<MouseEvent>>(cx, None);
    let onclick = Closure::wrap(
        Box::new(move |e| set_g_click_event.set(Some(e))) as Box<dyn FnMut(MouseEvent)>
    );
    doc.set_onclick(Some(onclick.as_ref().unchecked_ref()));
    provide_context(
        cx,
        GlobalClickEvent::new(Rc::new(Box::new(onclick)), g_click_event, set_g_click_event),
    );

    // MOUSE UP
    let (g_mouseup_event, set_g_mouseup_event) = create_signal::<Option<MouseEvent>>(cx, None);
    let onmouseup = Closure::wrap(
        Box::new(move |e| set_g_mouseup_event.set(Some(e))) as Box<dyn FnMut(MouseEvent)>
    );
    doc.set_onmouseup(Some(onmouseup.as_ref().unchecked_ref()));
    provide_context(
        cx,
        GlobalMouseupEvent::new(Rc::new(Box::new(onmouseup)), g_mouseup_event, set_g_mouseup_event),
    );

    view! {cx,
        <ThemeProvider theme=create_signal_ls(cx, "theme", default_theme)>
            <ToastRoot>
                <ModalRoot>
                    <Box style="min-height: 100vh; min-width: 100vw; display: flex; flex-direction: row;">
                        { children(cx) }
                    </Box>
                </ModalRoot>
            </ToastRoot>
        </ThemeProvider>
    }
}

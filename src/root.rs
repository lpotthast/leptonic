use std::rc::Rc;

use leptos::*;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{KeyboardEvent, MouseEvent};

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GlobalMouseEvent {
    pub read_signal: ReadSignal<Option<MouseEvent>>,
    pub write_signal: WriteSignal<Option<MouseEvent>>,
}

// TODO: Global touch event

#[derive(Debug, Clone)]
pub struct GlobalKeyboardEvent {
    closure: Rc<Box<Closure<dyn FnMut(KeyboardEvent)>>>,
    pub read_signal: ReadSignal<Option<KeyboardEvent>>,
    pub write_signal: WriteSignal<Option<KeyboardEvent>>,
}

#[component]
pub fn Root<T>(cx: Scope, default_theme: T, children: Children) -> impl IntoView
where
    T: Theme + 'static,
{
    let (g_mouse_event, set_g_mouse_event) = create_signal::<Option<MouseEvent>>(cx, None);
    provide_context(
        cx,
        GlobalMouseEvent {
            read_signal: g_mouse_event,
            write_signal: set_g_mouse_event,
        },
    );

    let (g_keyboard_event, set_g_keyboard_event) = create_signal::<Option<KeyboardEvent>>(cx, None);

    let doc = document();
    let f = Closure::wrap(
        Box::new(move |e| set_g_keyboard_event.set(Some(e))) as Box<dyn FnMut(KeyboardEvent)>
    );
    doc.set_onkeydown(Some(f.as_ref().unchecked_ref()));

    provide_context(
        cx,
        GlobalKeyboardEvent {
            closure: Rc::new(Box::new(f)),
            read_signal: g_keyboard_event,
            write_signal: set_g_keyboard_event,
        },
    );

    // TODO: The handler should be defined in the body or html element
    view! {cx,
        <ThemeProvider
            theme=create_signal_ls(cx, "theme", default_theme)
            on:click = move |e| set_g_mouse_event.set(Some(e))
            on:keydown = move |e| {
                tracing::info!("global key event");
                set_g_keyboard_event.set(Some(e));
            }
        >
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

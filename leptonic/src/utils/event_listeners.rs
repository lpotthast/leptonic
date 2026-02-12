use leptos::ev::EventDescriptor;
use leptos::typed_builder::TypedBuilder;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::convert::FromWasmAbi;

#[derive(Debug, Clone, Copy, TypedBuilder)]
pub struct EventListenerOptions {
    #[allow(unused)] // May only be used in non-SSR context.
    once: bool,

    #[allow(unused)] // May only be used in non-SSR context.
    capture: bool,
}

impl Default for EventListenerOptions {
    fn default() -> Self {
        Self {
            once: false,
            capture: false,
        }
    }
}

impl EventListenerOptions {
    pub fn capturing() -> Self {
        EventListenerOptions {
            capture: true,
            ..EventListenerOptions::default()
        }
    }
    pub fn once() -> Self {
        EventListenerOptions {
            once: true,
            ..EventListenerOptions::default()
        }
    }
}

#[allow(unused)] // May only be used in non-SSR context.
pub(crate) trait ListenExt {
    /// Adds an event listener for the given event name.
    #[must_use]
    fn listen<E>(
        &self,
        event: impl EventDescriptor,
        callback: impl FnMut(E) + 'static,
        options: EventListenerOptions,
    ) -> Closure<dyn FnMut(E)>
    where
        E: FromWasmAbi + 'static;
}

impl<T: AsRef<web_sys::EventTarget>> ListenExt for T {
    fn listen<E>(
        &self,
        event: impl EventDescriptor,
        callback: impl FnMut(E) + 'static,
        options: EventListenerOptions,
    ) -> Closure<dyn FnMut(E)>
    where
        E: FromWasmAbi + 'static,
    {
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;

        let target: &web_sys::EventTarget = self.as_ref();

        let closure: Closure<dyn FnMut(E)> = if options.once {
            let boxed: Box<dyn FnOnce(E)> = Box::new(callback);
            Closure::once(boxed)
        } else {
            let boxed: Box<dyn FnMut(E)> = Box::new(callback);
            Closure::wrap(boxed)
        };

        let web_sys_options = web_sys::AddEventListenerOptions::new();
        web_sys_options.set_once(options.once);
        web_sys_options.set_capture(options.capture);

        let _ = target.add_event_listener_with_callback_and_add_event_listener_options(
            event.name().as_ref(),
            closure.as_ref().unchecked_ref(),
            &web_sys_options,
        );

        closure
    }
}

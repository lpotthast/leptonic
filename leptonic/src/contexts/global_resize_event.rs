use std::rc::Rc;

use leptos::*;
use wasm_bindgen::prelude::Closure;
use web_sys::Event;

#[derive(Debug, Clone)]
pub struct GlobalResizeEvent {
    _closure: Rc<Box<Closure<dyn FnMut(Event)>>>,
    pub read_signal: ReadSignal<Option<Event>>,
    pub write_signal: WriteSignal<Option<Event>>,
}

impl GlobalResizeEvent {
    pub fn new(
        _closure: Rc<Box<Closure<dyn FnMut(Event)>>>,
        read_signal: ReadSignal<Option<Event>>,
        write_signal: WriteSignal<Option<Event>>,
    ) -> Self {
        Self {
            _closure,
            read_signal,
            write_signal,
        }
    }
}

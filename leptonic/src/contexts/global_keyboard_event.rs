use std::rc::Rc;

use leptos::*;
use wasm_bindgen::prelude::Closure;
use web_sys::KeyboardEvent;

#[derive(Debug, Clone)]
pub struct GlobalKeyboardEvent {
    _closure: Rc<Box<Closure<dyn FnMut(KeyboardEvent)>>>,
    pub read_signal: ReadSignal<Option<KeyboardEvent>>,
    pub write_signal: WriteSignal<Option<KeyboardEvent>>,
}

impl GlobalKeyboardEvent {
    pub fn new(
        _closure: Rc<Box<Closure<dyn FnMut(KeyboardEvent)>>>,
        read_signal: ReadSignal<Option<KeyboardEvent>>,
        write_signal: WriteSignal<Option<KeyboardEvent>>,
    ) -> Self {
        Self {
            _closure,
            read_signal,
            write_signal,
        }
    }
}

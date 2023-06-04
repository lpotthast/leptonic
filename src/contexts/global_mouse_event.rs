use std::rc::Rc;

use leptos::*;
use wasm_bindgen::prelude::Closure;
use web_sys::MouseEvent;

#[derive(Debug, Clone)]
pub struct GlobalMouseEvent {
    _closure: Rc<Box<Closure<dyn FnMut(MouseEvent)>>>,
    pub read_signal: ReadSignal<Option<MouseEvent>>,
    pub write_signal: WriteSignal<Option<MouseEvent>>,
}

impl GlobalMouseEvent {
    pub fn new(
        _closure: Rc<Box<Closure<dyn FnMut(MouseEvent)>>>,
        read_signal: ReadSignal<Option<MouseEvent>>,
        write_signal: WriteSignal<Option<MouseEvent>>,
    ) -> Self {
        Self {
            _closure,
            read_signal,
            write_signal,
        }
    }
}

use leptos::*;
use web_sys::MouseEvent;

use super::WasmClosure;

#[derive(Debug, Clone)]
pub struct GlobalMouseupEvent {
    _closure: WasmClosure<MouseEvent>,
    pub read_signal: ReadSignal<Option<MouseEvent>>,
    pub write_signal: WriteSignal<Option<MouseEvent>>,
}

impl GlobalMouseupEvent {
    pub fn new(
        _closure: WasmClosure<MouseEvent>,
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

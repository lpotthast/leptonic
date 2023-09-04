use leptos::*;
use web_sys::KeyboardEvent;

use super::WasmClosure;

#[derive(Debug, Clone)]
pub struct GlobalKeyboardEvent {
    _closure: WasmClosure<KeyboardEvent>,
    pub read_signal: ReadSignal<Option<KeyboardEvent>>,
    pub write_signal: WriteSignal<Option<KeyboardEvent>>,
}

impl GlobalKeyboardEvent {
    pub fn new(
        _closure: WasmClosure<KeyboardEvent>,
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

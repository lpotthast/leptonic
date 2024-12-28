use leptos::prelude::*;
use web_sys::KeyboardEvent;

use super::WasmClosure;

#[derive(Debug, Clone)]
pub struct GlobalKeyboardEvent {
    //_closure: WasmClosure<KeyboardEvent>,
    pub read_signal: ReadSignal<Option<KeyboardEvent>, LocalStorage>,
    pub write_signal: WriteSignal<Option<KeyboardEvent>, LocalStorage>,
}

impl GlobalKeyboardEvent {
    #[allow(clippy::used_underscore_binding)]
    pub fn new(
        //_closure: WasmClosure<KeyboardEvent>,
        read_signal: ReadSignal<Option<KeyboardEvent>, LocalStorage>,
        write_signal: WriteSignal<Option<KeyboardEvent>, LocalStorage>,
    ) -> Self {
        Self {
            //_closure,
            read_signal,
            write_signal,
        }
    }
}

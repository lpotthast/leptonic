use leptos::prelude::*;
use web_sys::MouseEvent;

use super::WasmClosure;

#[derive(Debug, Clone)]
pub struct GlobalMouseupEvent {
    _closure: WasmClosure<MouseEvent>,
    pub read_signal: ReadSignal<Option<MouseEvent>, LocalStorage>,
    pub write_signal: WriteSignal<Option<MouseEvent>, LocalStorage>,
}

impl GlobalMouseupEvent {
    #[allow(clippy::used_underscore_binding)]
    pub fn new(
        _closure: WasmClosure<MouseEvent>,
        read_signal: ReadSignal<Option<MouseEvent>, LocalStorage>,
        write_signal: WriteSignal<Option<MouseEvent>, LocalStorage>,
    ) -> Self {
        Self {
            _closure,
            read_signal,
            write_signal,
        }
    }
}

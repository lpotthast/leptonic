use leptos::*;
use web_sys::Event;

use super::WasmClosure;

#[derive(Debug, Clone)]
pub struct GlobalScrollEvent {
    _closure: WasmClosure<Event>,
    pub read_signal: ReadSignal<Option<Event>>,
    pub write_signal: WriteSignal<Option<Event>>,
}

impl GlobalScrollEvent {
    #[allow(clippy::used_underscore_binding)]
    pub fn new(
        _closure: WasmClosure<Event>,
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

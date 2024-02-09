use leptos::*;
use web_sys::PointerEvent;

use super::WasmClosure;

#[derive(Debug, Clone)]
pub struct GlobalPointerUpEvent {
    _closure: WasmClosure<PointerEvent>,
    pub read_signal: ReadSignal<Option<PointerEvent>>,
    pub write_signal: WriteSignal<Option<PointerEvent>>,
}

impl GlobalPointerUpEvent {
    #[allow(clippy::used_underscore_binding)]
    pub fn new(
        _closure: WasmClosure<PointerEvent>,
        read_signal: ReadSignal<Option<PointerEvent>>,
        write_signal: WriteSignal<Option<PointerEvent>>,
    ) -> Self {
        Self {
            _closure,
            read_signal,
            write_signal,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GlobalPointerDownEvent {
    _closure: WasmClosure<PointerEvent>,
    pub read_signal: ReadSignal<Option<PointerEvent>>,
    pub write_signal: WriteSignal<Option<PointerEvent>>,
}

impl GlobalPointerDownEvent {
    #[allow(clippy::used_underscore_binding)]
    pub fn new(
        _closure: WasmClosure<PointerEvent>,
        read_signal: ReadSignal<Option<PointerEvent>>,
        write_signal: WriteSignal<Option<PointerEvent>>,
    ) -> Self {
        Self {
            _closure,
            read_signal,
            write_signal,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GlobalPointerCancelEvent {
    _closure: WasmClosure<PointerEvent>,
    pub read_signal: ReadSignal<Option<PointerEvent>>,
    pub write_signal: WriteSignal<Option<PointerEvent>>,
}

impl GlobalPointerCancelEvent {
    #[allow(clippy::used_underscore_binding)]
    pub fn new(
        _closure: WasmClosure<PointerEvent>,
        read_signal: ReadSignal<Option<PointerEvent>>,
        write_signal: WriteSignal<Option<PointerEvent>>,
    ) -> Self {
        Self {
            _closure,
            read_signal,
            write_signal,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GlobalPointerMoveEvent {
    _closure: WasmClosure<PointerEvent>,
    pub read_signal: ReadSignal<Option<PointerEvent>>,
    pub write_signal: WriteSignal<Option<PointerEvent>>,
}

impl GlobalPointerMoveEvent {
    #[allow(clippy::used_underscore_binding)]
    pub fn new(
        _closure: WasmClosure<PointerEvent>,
        read_signal: ReadSignal<Option<PointerEvent>>,
        write_signal: WriteSignal<Option<PointerEvent>>,
    ) -> Self {
        Self {
            _closure,
            read_signal,
            write_signal,
        }
    }
}

use leptos::prelude::*;
use web_sys::PointerEvent;

#[derive(Debug, Clone)]
pub struct GlobalPointerUpEvent {
    pub read_signal: ReadSignal<Option<PointerEvent>, LocalStorage>,
    pub write_signal: WriteSignal<Option<PointerEvent>, LocalStorage>,
}

impl GlobalPointerUpEvent {
    pub fn new(
        read_signal: ReadSignal<Option<PointerEvent>, LocalStorage>,
        write_signal: WriteSignal<Option<PointerEvent>, LocalStorage>,
    ) -> Self {
        Self {
            read_signal,
            write_signal,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GlobalPointerDownEvent {
    pub read_signal: ReadSignal<Option<PointerEvent>, LocalStorage>,
    pub write_signal: WriteSignal<Option<PointerEvent>, LocalStorage>,
}

impl GlobalPointerDownEvent {
    pub fn new(
        read_signal: ReadSignal<Option<PointerEvent>, LocalStorage>,
        write_signal: WriteSignal<Option<PointerEvent>, LocalStorage>,
    ) -> Self {
        Self {
            read_signal,
            write_signal,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GlobalPointerCancelEvent {
    pub read_signal: ReadSignal<Option<PointerEvent>, LocalStorage>,
    pub write_signal: WriteSignal<Option<PointerEvent>, LocalStorage>,
}

impl GlobalPointerCancelEvent {
    pub fn new(
        read_signal: ReadSignal<Option<PointerEvent>, LocalStorage>,
        write_signal: WriteSignal<Option<PointerEvent>, LocalStorage>,
    ) -> Self {
        Self {
            read_signal,
            write_signal,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GlobalPointerMoveEvent {
    pub read_signal: ReadSignal<Option<PointerEvent>, LocalStorage>,
    pub write_signal: WriteSignal<Option<PointerEvent>, LocalStorage>,
}

impl GlobalPointerMoveEvent {
    #[allow(clippy::used_underscore_binding)]
    pub fn new(
        read_signal: ReadSignal<Option<PointerEvent>, LocalStorage>,
        write_signal: WriteSignal<Option<PointerEvent>, LocalStorage>,
    ) -> Self {
        Self {
            read_signal,
            write_signal,
        }
    }
}

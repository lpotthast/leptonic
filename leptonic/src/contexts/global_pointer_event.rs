use leptos::prelude::*;
use web_sys::PointerEvent;

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub struct GlobalPointerMoveEvent {
    pub read_signal: ReadSignal<Option<PointerEvent>, LocalStorage>,
    pub write_signal: WriteSignal<Option<PointerEvent>, LocalStorage>,
}

impl GlobalPointerMoveEvent {
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

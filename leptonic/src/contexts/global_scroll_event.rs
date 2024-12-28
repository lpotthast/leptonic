use leptos::prelude::*;
use web_sys::Event;

#[derive(Debug, Clone, Copy)]
pub struct GlobalScrollEvent {
    pub read_signal: ReadSignal<Option<Event>, LocalStorage>,
    pub write_signal: WriteSignal<Option<Event>, LocalStorage>,
}

impl GlobalScrollEvent {
    pub fn new(
        read_signal: ReadSignal<Option<Event>, LocalStorage>,
        write_signal: WriteSignal<Option<Event>, LocalStorage>,
    ) -> Self {
        Self {
            read_signal,
            write_signal,
        }
    }
}

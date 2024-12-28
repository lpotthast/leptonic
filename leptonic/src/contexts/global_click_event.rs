use leptos::prelude::*;
use web_sys::MouseEvent;

#[derive(Debug, Clone, Copy)]
pub struct GlobalClickEvent {
    pub read_signal: ReadSignal<Option<MouseEvent>, LocalStorage>,
    pub write_signal: WriteSignal<Option<MouseEvent>, LocalStorage>,
}

impl GlobalClickEvent {
    pub fn new(
        read_signal: ReadSignal<Option<MouseEvent>, LocalStorage>,
        write_signal: WriteSignal<Option<MouseEvent>, LocalStorage>,
    ) -> Self {
        Self {
            read_signal,
            write_signal,
        }
    }
}

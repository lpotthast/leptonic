use leptos::{ReadSignal, WriteSignal};

pub(crate) enum OnOpen {
    First,
    Last,
}

#[derive(Debug, Clone, Copy)]
pub struct MenuTriggerState {
    /// Whether the menu is open.
    pub(crate) open: ReadSignal<bool>,

    /// Setter for the `open` property.
    pub(crate) set_open: WriteSignal<bool>,
}

impl MenuTriggerState {
    pub fn open(&self, on_open: Option<OnOpen>) {
        todo!();
    }

    pub fn close(&self) {
        todo!();
    }

    pub fn toggle(&self, on_open: OnOpen) {
        todo!();
    }
}

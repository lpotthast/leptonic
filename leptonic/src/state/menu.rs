use leptos::{ReadSignal, WriteSignal};

#[derive(Debug, Clone, Copy)]
pub struct MenuTriggerState {
    /// Whether the menu is open.
    open: ReadSignal<bool>,

    /// Setter for the `open` property.
    set_open: WriteSignal<bool>,
}

impl MenuTriggerState {
    pub fn open(&self, elem: &str) {
        todo!();
    }

    pub fn close(&self) {
        todo!();
    }

    pub fn toggle(&self) {
        todo!();
    }
}

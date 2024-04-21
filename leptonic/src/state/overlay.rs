use leptos::{ReadSignal, SignalGetUntracked, SignalSet, WriteSignal};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Copy, TypedBuilder)]
pub struct OverlayTriggerState {
    /// Whether the overlay is currently shown.
    #[builder(setter(into))]
    pub show: ReadSignal<bool>,

    /// Setter for the `show` property.
    #[builder(setter(into))]
    pub set_show: WriteSignal<bool>,
}

impl OverlayTriggerState {
    pub fn show(&self) {
        self.set_show.set(true);
    }

    pub fn hide(&self) {
        self.set_show.set(false);
    }

    pub fn toggle(&self) {
        self.set_show.set(!self.show.get_untracked());
    }
}

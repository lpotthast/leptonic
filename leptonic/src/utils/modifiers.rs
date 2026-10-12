// No upstream: the modifier keys of an event as one value (react-aria reads the four `*Key`
// fields of the event).
//! The modifier keys held during an event ([`Modifiers`]).

/// Keyboard modifiers held during an event.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Modifiers {
    /// Whether the shift keyboard modifier was held during the event.
    pub shift_key: bool,

    /// Whether the ctrl keyboard modifier was held during the event.
    pub ctrl_key: bool,

    /// Whether the meta keyboard modifier was held during the event.
    pub meta_key: bool,

    /// Whether the alt keyboard modifier was held during the event.
    pub alt_key: bool,
}

/// The modifiers held during an event.
pub trait EventModifiers {
    fn modifiers(&self) -> Modifiers;
}

macro_rules! impl_event_modifiers {
    ($($event:ty),*) => {$(
        impl EventModifiers for $event {
            fn modifiers(&self) -> Modifiers {
                Modifiers {
                    shift_key: self.shift_key(),
                    ctrl_key: self.ctrl_key(),
                    meta_key: self.meta_key(),
                    alt_key: self.alt_key(),
                }
            }
        }
    )*};
}

impl_event_modifiers!(
    web_sys::MouseEvent,
    web_sys::KeyboardEvent,
    web_sys::PointerEvent
);

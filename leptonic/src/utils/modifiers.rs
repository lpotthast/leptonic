/// Keyboard modifiers held during an event.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Modifiers {
    /// Whether the shift keyboard modifier was held during the event.
    pub shift_key: bool,

    /// Whether the ctrl keyboard modifier was held during the event.
    pub ctrl_key: bool,

    /// Whether the meta keyboard modifier was held during the event.
    pub meta_key: bool,

    /// Whether the alt keyboard modifier was held during the  event.
    pub alt_key: bool,
}

pub trait EventModifiers {
    fn modifiers(&self) -> Modifiers;
}

impl EventModifiers for web_sys::MouseEvent {
    fn modifiers(&self) -> Modifiers {
        Modifiers {
            shift_key: self.shift_key(),
            ctrl_key: self.ctrl_key(),
            meta_key: self.meta_key(),
            alt_key: self.alt_key(),
        }
    }
}

impl EventModifiers for web_sys::TouchEvent {
    fn modifiers(&self) -> Modifiers {
        Modifiers {
            shift_key: self.shift_key(),
            ctrl_key: self.ctrl_key(),
            meta_key: self.meta_key(),
            alt_key: self.alt_key(),
        }
    }
}

impl EventModifiers for web_sys::KeyboardEvent {
    fn modifiers(&self) -> Modifiers {
        Modifiers {
            shift_key: self.shift_key(),
            ctrl_key: self.ctrl_key(),
            meta_key: self.meta_key(),
            alt_key: self.alt_key(),
        }
    }
}

impl EventModifiers for web_sys::PointerEvent {
    fn modifiers(&self) -> Modifiers {
        Modifiers {
            shift_key: self.shift_key(),
            ctrl_key: self.ctrl_key(),
            meta_key: self.meta_key(),
            alt_key: self.alt_key(),
        }
    }
}

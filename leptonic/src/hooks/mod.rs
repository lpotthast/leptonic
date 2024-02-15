pub mod button;
pub mod calendar;
pub mod focus;
pub mod r#move;
pub mod press;

pub mod prelude {
    pub use super::button::use_button;
    pub use super::button::UseButtonProps;
    pub use super::button::UseButtonReturn;
    pub use super::press::use_press;
    pub use super::press::UsePressInput;
    pub use super::press::UsePressReturn;
    pub use super::r#move::use_move;
    pub use super::r#move::UseMoveInput;
    pub use super::r#move::UseMoveReturn;
}

#[derive(Debug, Clone, Copy)]
pub struct KeyModifiers {
    /// Whether the shift keyboard modifier was held during the event.
    pub shift_key: bool,

    /// Whether the ctrl keyboard modifier was held during the event.
    pub ctrl_key: bool,

    /// Whether the meta keyboard modifier was held during the event.
    pub meta_key: bool,

    /// Whether the alt keyboard modifier was held during the  event.
    pub alt_key: bool,
}

pub trait Modifiers {
    fn modifiers(&self) -> KeyModifiers;
}

impl Modifiers for web_sys::MouseEvent {
    fn modifiers(&self) -> KeyModifiers {
        KeyModifiers {
            shift_key: self.shift_key(),
            ctrl_key: self.ctrl_key(),
            meta_key: self.meta_key(),
            alt_key: self.alt_key(),
        }
    }
}

impl Modifiers for web_sys::TouchEvent {
    fn modifiers(&self) -> KeyModifiers {
        KeyModifiers {
            shift_key: self.shift_key(),
            ctrl_key: self.ctrl_key(),
            meta_key: self.meta_key(),
            alt_key: self.alt_key(),
        }
    }
}

impl Modifiers for web_sys::KeyboardEvent {
    fn modifiers(&self) -> KeyModifiers {
        KeyModifiers {
            shift_key: self.shift_key(),
            ctrl_key: self.ctrl_key(),
            meta_key: self.meta_key(),
            alt_key: self.alt_key(),
        }
    }
}

fn disable_text_selection(element: web_sys::Element) {
    element.set_attribute("data-disable-user-select", "true");
}

fn restore_text_selection(element: web_sys::Element) {
    element.remove_attribute("data-disable-user-select");
}

// No upstream: typed keyboard keys (react-aria compares `KeyboardEvent.key` strings).

use std::{borrow::Cow, convert::Infallible, str::FromStr};

use strum::{EnumIter, IntoEnumIterator, IntoStaticStr};

/// A keyboard key.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter, IntoStaticStr)]
pub enum KeyboardKey {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    N0,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    ArrowUp,
    ArrowRight,
    ArrowDown,
    ArrowLeft,
    Plus,
    Star,
    Dash,
    Underscore,
    Slash,
    Backslash,
    Dot,
    Comma,
    Colon,
    Semicolon,
    Hash,
    Escape,
    Enter,
    Backspace,
    Control,
    Alt,
    Shift,
    CapsLock,
    Command,
    Option,
    /// Control as Apple keyboards label it (⌃), next to [`Command`](Self::Command) and
    /// [`Option`](Self::Option). Events report it as `Control`.
    ControlSymbol,
    Tab,
    Tilde,
    Fn,
    Space,
    Home,
    End,
    PageUp,
    PageDown,
    Delete,
    Meta,
    Insert,
    ContextMenu,
    NumLock,
    PrintScreen,
    ScrollLock,
    Pause,
    /// Any other key value.
    ///
    /// Used both for custom display labels in the `Kbd` atom (as `Cow::Borrowed`)
    /// and as a catch-all for unrecognized `KeyboardEvent.key()` values (as `Cow::Owned`).
    Other(Cow<'static, str>),
}

impl KeyboardKey {
    pub fn known_keys() -> impl Iterator<Item = KeyboardKey> {
        Self::iter().filter(|key| {
            std::mem::discriminant(key)
                != std::mem::discriminant(&KeyboardKey::Other(Cow::Borrowed("")))
        })
    }

    /// The key's name for screen readers, where its display is a glyph or an abbreviation they
    /// would announce wrongly (e.g. "⇧" or "PgUp"); `None` where the display reads well.
    pub fn spoken_name(&self) -> Option<&'static str> {
        Some(match self {
            Self::ArrowUp => "Up Arrow",
            Self::ArrowRight => "Right Arrow",
            Self::ArrowDown => "Down Arrow",
            Self::ArrowLeft => "Left Arrow",
            Self::Shift => "Shift",
            Self::CapsLock => "Caps Lock",
            Self::Command => "Command",
            Self::Option => "Option",
            Self::Tab => "Tab",
            Self::Escape => "Escape",
            Self::Control | Self::ControlSymbol => "Control",
            Self::Fn => "Function",
            Self::PageUp => "Page Up",
            Self::PageDown => "Page Down",
            Self::Delete => "Delete",
            Self::Insert => "Insert",
            Self::NumLock => "Num Lock",
            Self::PrintScreen => "Print Screen",
            Self::ScrollLock => "Scroll Lock",
            _ => return None,
        })
    }

    /// Returns the display string for this key (English).
    pub fn display(&self) -> &str {
        match self {
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
            Self::D => "D",
            Self::E => "E",
            Self::F => "F",
            Self::G => "G",
            Self::H => "H",
            Self::I => "I",
            Self::J => "J",
            Self::K => "K",
            Self::L => "L",
            Self::M => "M",
            Self::N => "N",
            Self::O => "O",
            Self::P => "P",
            Self::Q => "Q",
            Self::R => "R",
            Self::S => "S",
            Self::T => "T",
            Self::U => "U",
            Self::V => "V",
            Self::W => "W",
            Self::X => "X",
            Self::Y => "Y",
            Self::Z => "Z",
            Self::N0 => "0",
            Self::N1 => "1",
            Self::N2 => "2",
            Self::N3 => "3",
            Self::N4 => "4",
            Self::N5 => "5",
            Self::N6 => "6",
            Self::N7 => "7",
            Self::N8 => "8",
            Self::N9 => "9",
            Self::F1 => "F1",
            Self::F2 => "F2",
            Self::F3 => "F3",
            Self::F4 => "F4",
            Self::F5 => "F5",
            Self::F6 => "F6",
            Self::F7 => "F7",
            Self::F8 => "F8",
            Self::F9 => "F9",
            Self::F10 => "F10",
            Self::F11 => "F11",
            Self::F12 => "F12",
            Self::ArrowUp => "↑",
            Self::ArrowRight => "→",
            Self::ArrowDown => "↓",
            Self::ArrowLeft => "←",
            Self::Plus => "+",
            Self::Star => "*",
            Self::Dash => "-",
            Self::Underscore => "_",
            Self::Slash => "/",
            Self::Backslash => "\\",
            Self::Dot => ".",
            Self::Comma => ",",
            Self::Colon => ":",
            Self::Semicolon => ";",
            Self::Hash => "#",
            Self::Escape => "Esc",
            Self::Enter => "Enter",
            Self::Backspace => "Backspace",
            Self::Control => "Ctrl",
            Self::Alt => "Alt",
            Self::Shift => "⇧",
            Self::CapsLock => "⇪",
            Self::Command => "⌘",
            Self::Option => "⌥",
            Self::ControlSymbol => "⌃",
            Self::Tab => "↹",
            Self::Tilde => "~",
            Self::Fn => "fn",
            Self::Space => "Space",
            Self::Home => "Home",
            Self::End => "End",
            Self::PageUp => "PgUp",
            Self::PageDown => "PgDn",
            Self::Delete => "Del",
            Self::Meta => "Meta",
            Self::Insert => "Ins",
            Self::ContextMenu => "Menu",
            Self::NumLock => "NumLk",
            Self::PrintScreen => "PrtSc",
            Self::ScrollLock => "ScrLk",
            Self::Pause => "Pause",
            Self::Other(s) => s.as_ref(),
        }
    }
}

impl FromStr for KeyboardKey {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            // Single letters (lowercase and uppercase map to the same key).
            "a" | "A" => KeyboardKey::A,
            "b" | "B" => KeyboardKey::B,
            "c" | "C" => KeyboardKey::C,
            "d" | "D" => KeyboardKey::D,
            "e" | "E" => KeyboardKey::E,
            "f" | "F" => KeyboardKey::F,
            "g" | "G" => KeyboardKey::G,
            "h" | "H" => KeyboardKey::H,
            "i" | "I" => KeyboardKey::I,
            "j" | "J" => KeyboardKey::J,
            "k" | "K" => KeyboardKey::K,
            "l" | "L" => KeyboardKey::L,
            "m" | "M" => KeyboardKey::M,
            "n" | "N" => KeyboardKey::N,
            "o" | "O" => KeyboardKey::O,
            "p" | "P" => KeyboardKey::P,
            "q" | "Q" => KeyboardKey::Q,
            "r" | "R" => KeyboardKey::R,
            "s" | "S" => KeyboardKey::S,
            "t" | "T" => KeyboardKey::T,
            "u" | "U" => KeyboardKey::U,
            "v" | "V" => KeyboardKey::V,
            "w" | "W" => KeyboardKey::W,
            "x" | "X" => KeyboardKey::X,
            "y" | "Y" => KeyboardKey::Y,
            "z" | "Z" => KeyboardKey::Z,
            // Digits
            "0" => KeyboardKey::N0,
            "1" => KeyboardKey::N1,
            "2" => KeyboardKey::N2,
            "3" => KeyboardKey::N3,
            "4" => KeyboardKey::N4,
            "5" => KeyboardKey::N5,
            "6" => KeyboardKey::N6,
            "7" => KeyboardKey::N7,
            "8" => KeyboardKey::N8,
            "9" => KeyboardKey::N9,
            // Function keys
            "F1" => KeyboardKey::F1,
            "F2" => KeyboardKey::F2,
            "F3" => KeyboardKey::F3,
            "F4" => KeyboardKey::F4,
            "F5" => KeyboardKey::F5,
            "F6" => KeyboardKey::F6,
            "F7" => KeyboardKey::F7,
            "F8" => KeyboardKey::F8,
            "F9" => KeyboardKey::F9,
            "F10" => KeyboardKey::F10,
            "F11" => KeyboardKey::F11,
            "F12" => KeyboardKey::F12,
            // Arrow keys
            "ArrowUp" => KeyboardKey::ArrowUp,
            "ArrowRight" => KeyboardKey::ArrowRight,
            "ArrowDown" => KeyboardKey::ArrowDown,
            "ArrowLeft" => KeyboardKey::ArrowLeft,
            // Special keys
            " " | "Space" => KeyboardKey::Space,
            "Enter" => KeyboardKey::Enter,
            "Escape" => KeyboardKey::Escape,
            "Tab" => KeyboardKey::Tab,
            "Backspace" => KeyboardKey::Backspace,
            "Delete" => KeyboardKey::Delete,
            "Home" => KeyboardKey::Home,
            "End" => KeyboardKey::End,
            "PageUp" => KeyboardKey::PageUp,
            "PageDown" => KeyboardKey::PageDown,
            "Meta" => KeyboardKey::Meta,
            "Control" => KeyboardKey::Control,
            "Alt" => KeyboardKey::Alt,
            "Shift" => KeyboardKey::Shift,
            "CapsLock" => KeyboardKey::CapsLock,
            "Insert" => KeyboardKey::Insert,
            "ContextMenu" => KeyboardKey::ContextMenu,
            "NumLock" => KeyboardKey::NumLock,
            "PrintScreen" => KeyboardKey::PrintScreen,
            "ScrollLock" => KeyboardKey::ScrollLock,
            "Pause" => KeyboardKey::Pause,
            "Fn" => KeyboardKey::Fn,
            // Display names of macOS modifiers (events report these keys as `Meta` and `Alt`).
            "Command" => KeyboardKey::Command,
            "Option" => KeyboardKey::Option,
            // Punctuation
            "+" => KeyboardKey::Plus,
            "*" => KeyboardKey::Star,
            "-" => KeyboardKey::Dash,
            "_" => KeyboardKey::Underscore,
            "/" => KeyboardKey::Slash,
            "\\" => KeyboardKey::Backslash,
            "." => KeyboardKey::Dot,
            "," => KeyboardKey::Comma,
            ":" => KeyboardKey::Colon,
            ";" => KeyboardKey::Semicolon,
            "#" => KeyboardKey::Hash,
            "~" => KeyboardKey::Tilde,
            // Unrecognized
            other => KeyboardKey::Other(Cow::Owned(other.to_owned())),
        })
    }
}

/// The typed [`KeyboardKey`] of a keyboard event, for comparisons without string literals.
pub trait KeyboardEventKey {
    fn typed_key(&self) -> KeyboardKey;
}

impl KeyboardEventKey for web_sys::KeyboardEvent {
    fn typed_key(&self) -> KeyboardKey {
        let Ok(key) = self.key().parse();
        key
    }
}

/// Dispatches a copy of the keyboard event `e` (same type, key, code, location, repeat and
/// modifiers; bubbling and cancelable) on `target`, e.g. to let a collection handle a key that
/// one of its cells or rows received (react-aria: `target.dispatchEvent(new KeyboardEvent(e.type,
/// e))`). Nothing happens if the browser can't create the event.
///
/// Called from a handler of `e`, this is a nested dispatch of the same event type: safe only when
/// no listener still running for `e` is on the copy's path (see leptos-and-dom.md, "No
/// Nested Dispatch of the Same Event Type").
pub(crate) fn redispatch_keyboard_event(e: &web_sys::KeyboardEvent, target: &web_sys::EventTarget) {
    let init = web_sys::KeyboardEventInit::new();
    init.set_key(&e.key());
    init.set_code(&e.code());
    init.set_location(e.location());
    init.set_repeat(e.repeat());
    init.set_shift_key(e.shift_key());
    init.set_ctrl_key(e.ctrl_key());
    init.set_alt_key(e.alt_key());
    init.set_meta_key(e.meta_key());
    init.set_bubbles(true);
    init.set_cancelable(true);
    if let Ok(copy) = web_sys::KeyboardEvent::new_with_keyboard_event_init_dict(&e.type_(), &init) {
        let _ = target.dispatch_event(&copy);
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::KeyboardKey;

    #[test]
    fn parses_mac_modifier_display_names_and_fn() {
        assert_that!("Command".parse::<KeyboardKey>().ok()).is_equal_to(Some(KeyboardKey::Command));
        assert_that!("Option".parse::<KeyboardKey>().ok()).is_equal_to(Some(KeyboardKey::Option));
        assert_that!("Fn".parse::<KeyboardKey>().ok()).is_equal_to(Some(KeyboardKey::Fn));
    }
}

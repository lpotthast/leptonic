// Upstream: @react-types/shared/src/events.d.ts @ 99e6102368
//! The kind of input behind an interaction (react-aria's `PointerType`).

use std::fmt;

/// The kind of input behind an interaction: a pointer (`Mouse`, `Pen`, `Touch`), the keyboard, or
/// assistive technology (`Virtual`).
///
/// API DIFFERENCES: react-aria's `PointerType` is a string union; pointer events whose
/// `pointerType` is none of `"mouse"`, `"pen"` and `"touch"` (an empty string when the browser
/// can't tell, or a vendor-specific type) are `Unknown` here (react-aria passes the string on).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PointerType {
    Mouse,
    Pen,
    Touch,
    Keyboard,
    Virtual,
    /// A pointer of a type the browser can't tell or names otherwise.
    Unknown,
}

impl PointerType {
    /// The pointer type of a DOM pointer event (its `pointerType`).
    pub fn of(event: &web_sys::PointerEvent) -> Self {
        match event.pointer_type().as_str() {
            "mouse" => Self::Mouse,
            "pen" => Self::Pen,
            "touch" => Self::Touch,
            _ => Self::Unknown,
        }
    }
}

impl fmt::Display for PointerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Mouse => "mouse",
            Self::Pen => "pen",
            Self::Touch => "touch",
            Self::Keyboard => "keyboard",
            Self::Virtual => "virtual",
            Self::Unknown => "unknown",
        })
    }
}

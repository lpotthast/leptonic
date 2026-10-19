// Upstream: @react-types/shared/src/orientation.d.ts @ 99e6102368
// No deviations from react-aria beyond the project-wide API conventions.
//! The orientation of components ([`Orientation`], convention C6).

use super::aria::AriaOrientation;

/// The orientation of a component (slider, separator, toolbar, collection, group, ...).
///
/// Deliberately without `Default`: react-aria's default differs per component (horizontal
/// sliders, vertical listboxes), so each hook's input states its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Orientation {
    /// `horizontal` or `vertical` (as in `aria-orientation` and `data-orientation`).
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

impl super::signal_attr::ToAttrText for Orientation {
    fn to_attr_text(&self) -> super::signal_attr::AttrText {
        super::signal_attr::AttrText::Text(leptos::oco::Oco::Borrowed(self.as_str()))
    }
}

impl From<Orientation> for AriaOrientation {
    fn from(value: Orientation) -> Self {
        match value {
            Orientation::Horizontal => Self::Horizontal,
            Orientation::Vertical => Self::Vertical,
        }
    }
}

pub use leptos_styles::css::*;

// Leptonic-specific From impls that bridge leptonic types to CssValue.

impl From<crate::utils::color::RGB8> for leptos_styles::css::CssColor {
    fn from(c: crate::utils::color::RGB8) -> Self {
        Self::Rgb(c.r, c.g, c.b)
    }
}

impl From<crate::utils::color::RGB8> for leptos_styles::css::CssValue {
    fn from(c: crate::utils::color::RGB8) -> Self {
        Self::Color(leptos_styles::css::CssColor::from(c))
    }
}

impl From<crate::utils::color::HSL> for leptos_styles::css::CssColor {
    fn from(c: crate::utils::color::HSL) -> Self {
        Self::Hsl(c.hue, c.saturation * 100.0, c.lightness * 100.0)
    }
}

impl From<crate::utils::color::HSL> for leptos_styles::css::CssValue {
    fn from(c: crate::utils::color::HSL) -> Self {
        Self::Color(leptos_styles::css::CssColor::from(c))
    }
}

// FontWeight, Margin, Padding and their From<X> for CssValue impls
// now live in leptos-styles and are re-exported via `pub use leptos_styles::css::*` above.

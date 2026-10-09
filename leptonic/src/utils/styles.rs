//! Typed inline styles: the `leptos-styles` crate (`Styles`, typed declarations in `property`,
//! CSS values in `css`), with leptonic's additions in `css` (conversions of leptonic's color types
//! and non-panicking helpers for values computed at runtime).

pub use leptos_styles::*;

/// CSS values (`leptos_styles::css`), plus leptonic's conversions and helpers.
pub mod css {
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
            // Hue is undefined (NaN) for achromatic colors; any finite hue renders the same color.
            let finite_or_zero = |v: f64| if v.is_finite() { v } else { 0.0 };
            hsl(
                finite_or_zero(c.hue),
                (finite_or_zero(c.saturation) * 100.0).clamp(0.0, 100.0),
                (finite_or_zero(c.lightness) * 100.0).clamp(0.0, 100.0),
            )
        }
    }

    impl From<crate::utils::color::HSL> for leptos_styles::css::CssValue {
        fn from(c: crate::utils::color::HSL) -> Self {
            Self::Color(leptos_styles::css::CssColor::from(c))
        }
    }

    // Helpers for turning runtime-computed numbers (which may be NaN or out of range, e.g. a slider
    // percentage when `min == max`) into typed CSS values without panicking.

    /// A percentage dimension for a computed value. Non-finite input renders as `0px`.
    pub fn computed_pct(value: f64) -> CssDimension {
        try_pct(value).unwrap_or(CssDimension::Zero)
    }

    /// A pixel dimension for a computed value. Non-finite input renders as `0px`.
    pub fn computed_px(value: f64) -> CssDimension {
        try_px(value).unwrap_or(CssDimension::Zero)
    }

    /// A `width`/`height` value for a computed dimension. Negative input is clamped to `0px`.
    pub fn computed_size(value: CssDimension) -> Size {
        NonNegativeLengthPercentage::try_from(value)
            .unwrap_or_else(|_| NonNegativeLengthPercentage::new(CssDimension::Zero))
            .into()
    }
}

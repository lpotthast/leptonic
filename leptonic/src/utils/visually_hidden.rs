// Upstream: react-aria/src/visually-hidden/VisuallyHidden.tsx @ 99e6102368
use leptos_styles::Styles;

use crate::utils::{
    css::{computed_px, computed_size},
    style::{HeightProperty, WidthProperty},
};

/// The CSS hiding an element visually while keeping it available to assistive technology
/// (react-aria's `useVisuallyHidden` styles), as a string literal for `concat!`.
macro_rules! visually_hidden_css {
    () => {
        "border: 0; clip: rect(0 0 0 0); clip-path: inset(50%); height: 1px; margin: -1px; \
         overflow: hidden; padding: 0; position: absolute; width: 1px; white-space: nowrap;"
    };
}
pub(crate) use visually_hidden_css;

/// The CSS hiding an element visually while keeping it available to assistive technology.
pub const VISUALLY_HIDDEN_STYLE: &str = visually_hidden_css!();

/// [`VISUALLY_HIDDEN_STYLE`] as `Styles`, to merge with other styles of an element.
pub fn visually_hidden_styles() -> Styles {
    Styles::new()
        .add_unchecked("border", "0")
        .add_unchecked("clip", "rect(0 0 0 0)")
        .add_unchecked("clip-path", "inset(50%)")
        .add(HeightProperty.declare(computed_size(computed_px(1.0))))
        .add_unchecked("margin", "-1px")
        .add_unchecked("overflow", "hidden")
        .add_unchecked("padding", "0")
        .add_unchecked("position", "absolute")
        .add(WidthProperty.declare(computed_size(computed_px(1.0))))
        .add_unchecked("white-space", "nowrap")
}

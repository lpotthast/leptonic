// Upstream: react-aria/src/visually-hidden/VisuallyHidden.tsx @ 99e6102368
use leptos_styles::Styles;

use crate::utils::{
    css::{CssDimension, Opacity, computed_pct, computed_px, computed_size},
    style::{HeightProperty, OpacityProperty, WidthProperty},
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
    hidden_with(computed_px(1.0), "absolute")
}

/// [`visually_hidden_styles`] fixed at the top left of the viewport (react-aria-components'
/// `HiddenDateInput`), so that focusing the element doesn't scroll the page.
pub fn visually_hidden_fixed_styles() -> Styles {
    hidden_with(computed_px(1.0), "fixed")
        .add_unchecked("top", "0")
        .add_unchecked("left", "0")
}

/// The styles of an input covering a color thumb, invisible but at its full size (react-aria's
/// color hooks: the visually hidden styles with `opacity: 0.0001`, `width`/`height: 100%` and
/// `pointer-events: none`), so screen readers outline the thumb.
pub fn visually_hidden_full_size_styles() -> Styles {
    hidden_with(computed_pct(100.0), "absolute")
        .add(OpacityProperty.declare(Opacity::new(0.0001)))
        .add_unchecked("pointer-events", "none")
}

fn hidden_with(size: CssDimension, position: &'static str) -> Styles {
    Styles::new()
        .add_unchecked("border", "0")
        .add_unchecked("clip", "rect(0 0 0 0)")
        .add_unchecked("clip-path", "inset(50%)")
        .add(HeightProperty.declare(computed_size(size)))
        .add_unchecked("margin", "-1px")
        .add_unchecked("overflow", "hidden")
        .add_unchecked("padding", "0")
        .add_unchecked("position", position)
        .add(WidthProperty.declare(computed_size(size)))
        .add_unchecked("white-space", "nowrap")
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use assertr::prelude::*;

    use super::*;

    /// Duplicate properties make leptos-styles warn on every render (and the later one wins).
    fn property_names(styles: &Styles) -> Vec<String> {
        styles
            .to_style_string()
            .split(';')
            .filter_map(|declaration| declaration.split_once(':'))
            .map(|(name, _)| name.trim().to_owned())
            .collect()
    }

    #[test]
    fn no_helper_declares_a_property_twice() {
        for styles in [
            visually_hidden_styles(),
            visually_hidden_fixed_styles(),
            visually_hidden_full_size_styles(),
        ] {
            let names = property_names(&styles);
            let unique: HashSet<&String> = names.iter().collect();
            assert_that!(unique.len()).is_equal_to(names.len());
        }
    }

    #[test]
    fn fixed_styles_are_fixed_at_the_top_left() {
        let style = visually_hidden_fixed_styles().to_style_string();
        assert_that!(style.as_str()).contains("position:fixed;");
        assert_that!(style.as_str()).contains("top:0;");
        assert_that!(style.as_str()).contains("left:0;");
        assert_that!(style.as_str()).does_not_contain("absolute");
    }
}

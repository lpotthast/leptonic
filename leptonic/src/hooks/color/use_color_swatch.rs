// Upstream: react-aria/src/color/useColorSwatch.ts @ 99e6102368
use leptos::{attr, attr::Attr, prelude::*};

use crate::{
    hooks::{IntoAttrs, PropsWithStyles},
    utils::{
        aria::AriaRole,
        color::Color,
        css::ForcedColorAdjust,
        i18n::use_locale,
        id::use_id,
        intl_strings::{ColorStrings, use_localized_strings},
        style::ForcedColorAdjustProperty,
        styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The color is any color value or signal of one (`ColorProp`, react-aria: a `Color` or a
//   string); a swatch always has one (react-aria: optional, transparent by default).
//
// =============================================================================

/// Input of [`use_color_swatch`].
#[derive(Debug, Clone)]
pub struct UseColorSwatchInput {
    /// The color to show.
    pub color: Signal<Color>,
    /// Replaces the color's name (e.g. "Ocean" instead of "dark vibrant cyan blue").
    pub color_name: MaybeProp<String>,
    /// Added to the color's name (e.g. "Background" for "vibrant red, Background").
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
    /// The swatch's id. Generated when `None`.
    pub id: Option<String>,
}

/// Return value of [`use_color_swatch`].
#[derive(Debug)]
pub struct UseColorSwatchReturn {
    /// For the swatch element (with its background color).
    pub color_swatch_props: PropsWithStyles<UseColorSwatchProps>,
}

/// Props of a color swatch.
#[derive(Debug, Clone)]
pub struct UseColorSwatchProps {
    pub id: String,
    /// "color swatch" (localized).
    pub aria_roledescription: Signal<String>,
    pub aria_label: Signal<String>,
    pub aria_labelledby: Option<String>,
}

pub type UseColorSwatchAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaRoledescription, Signal<String>>,
    Attr<attr::AriaLabel, Signal<String>>,
    Attr<attr::AriaLabelledby, Option<String>>,
);

impl IntoAttrs for UseColorSwatchProps {
    type Attrs = UseColorSwatchAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, AriaRole::Img),
            Attr(attr::AriaRoledescription, self.aria_roledescription),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
        )
    }
}

/// Accessibility of a swatch showing a color: an image named after the color (and the given
/// label), with the color as its background.
pub fn use_color_swatch(input: UseColorSwatchInput) -> UseColorSwatchReturn {
    let UseColorSwatchInput {
        color,
        color_name,
        aria_label,
        aria_labelledby,
        id,
    } = input;
    let id = id.unwrap_or_else(|| use_id("color-swatch"));
    let locale = use_locale();
    let strings = use_localized_strings::<ColorStrings>();
    let aria_label = Signal::derive(move || {
        // A fully transparent color is "transparent" (react-aria).
        let name = color_name.get().unwrap_or_else(|| {
            let color = color.get();
            if color.alpha <= 0.0 {
                strings.read().transparent()
            } else {
                color.color_name(&locale.get())
            }
        });
        match aria_label.get().filter(|label| !label.is_empty()) {
            Some(label) => format!("{name}, {label}"),
            None => name,
        }
    });
    let aria_labelledby = aria_labelledby.map(|ids| format!("{id} {ids}"));
    let styles = Styles::new()
        // A computed CSS color (any color space): no checked grammar in `leptos-css` yet.
        .add_optional_unchecked("background-color", move || {
            Some(color.get().to_css_string())
        })
        .add(ForcedColorAdjustProperty.declare(ForcedColorAdjust::None));
    UseColorSwatchReturn {
        color_swatch_props: PropsWithStyles::new(
            UseColorSwatchProps {
                id,
                aria_roledescription: Signal::derive(move || strings.read().color_swatch()),
                aria_label,
                aria_labelledby,
            },
            styles,
        ),
    }
}

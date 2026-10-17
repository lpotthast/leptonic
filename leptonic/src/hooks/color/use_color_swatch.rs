// Upstream: react-aria/src/color/useColorSwatch.ts @ 99e6102368
// Upstream: react-aria-components/test/ColorSwatch.test.js @ 99e6102368
use leptos::prelude::*;

use crate::{
    Id, IntoAttrs, PropsWithStyles,
    utils::{
        aria::AriaRole,
        color::Color,
        i18n::use_locale,
        id::use_id,
        intl_strings::{ColorStrings, use_localized_strings},
        part::{Part, Remount},
        signal_attr::{FixedAttr, SignalAttr, fixed_attr, signal_attr},
        styles::{Styles, css::ForcedColorAdjust, property::ForcedColorAdjustProperty},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The color is a `Signal<Color>` (atoms take any color value or signal of one, `ColorProp`;
//   react-aria: a `Color` or a string); a swatch always has one (react-aria: optional,
//   transparent by default).
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
    pub aria_labelledby: Signal<Option<Id>>,
    /// The swatch's id. Generated when `None`.
    pub id: Option<Id>,
}

/// The return value of [`use_color_swatch`].
#[derive(Debug)]
pub struct UseColorSwatchReturn {
    /// The swatch element: `let (attrs, styles) = swatch.attrs();`.
    pub swatch: Part<PropsWithStyles<UseColorSwatchProps>>,
}

/// Props of a color swatch.
#[derive(Debug)]
pub struct UseColorSwatchProps {
    pub id: Id,
    /// "color swatch" (localized).
    pub aria_roledescription: Signal<String>,
    pub aria_label: Signal<String>,
    pub aria_labelledby: Signal<Option<Id>>,
}

impl Remount for UseColorSwatchProps {
    fn remount(&self) -> Self {
        Self {
            id: self.id.clone(),
            aria_roledescription: self.aria_roledescription,
            aria_label: self.aria_label,
            aria_labelledby: self.aria_labelledby,
        }
    }
}

pub type UseColorSwatchAttrs = (
    FixedAttr,
    FixedAttr,
    SignalAttr<String>,
    SignalAttr<String>,
    SignalAttr<Option<Id>>,
);

impl IntoAttrs for UseColorSwatchProps {
    type Attrs = UseColorSwatchAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            fixed_attr("id", self.id),
            fixed_attr("role", AriaRole::Img),
            signal_attr("aria-roledescription", self.aria_roledescription),
            signal_attr("aria-label", self.aria_label),
            signal_attr("aria-labelledby", self.aria_labelledby),
        )
    }
}

/// Accessibility of a swatch showing a color: an image named after the color (and the given
/// label), with the color as its background.
pub fn use_color_swatch(input: UseColorSwatchInput) -> UseColorSwatchReturn {
    crate::metrics::subject!();
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
    // A memo: it allocates, and the labelling reads it too.
    let aria_label: Signal<String> = Memo::new(move |_| {
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
    })
    .into();
    // Next to other labels, the swatch is named by its color name too (react-aria prepends its id).
    let aria_labelledby = {
        let id = id.clone();
        Signal::derive(move || {
            crate::utils::labels::labelledby_ids(
                &id,
                aria_label.with(|label| !label.is_empty()),
                aria_labelledby.get().as_ref(),
            )
        })
    };
    let styles = Styles::new()
        // A computed CSS color (any color space): no checked grammar in `leptos-css` yet.
        .add_optional_unchecked("background-color", move || {
            Some(color.get().to_css_string())
        })
        .add(ForcedColorAdjustProperty.declare(ForcedColorAdjust::None));
    UseColorSwatchReturn {
        swatch: Part::stored(PropsWithStyles::new(
            UseColorSwatchProps {
                id,
                aria_roledescription: Signal::derive(move || strings.read().color_swatch()),
                aria_label,
                aria_labelledby,
            },
            styles,
        )),
    }
}

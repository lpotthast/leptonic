use leptos::{attr, attr::Attr, prelude::*};

use crate::{
    hooks::IntoAttrs,
    utils::{aria::AriaRole, color::ColorValue},
};

// This is based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/color/src/useColorSwatch.ts

// No intentional deviations from the react-aria implementation.

/// Input parameters for `use_color_swatch`.
#[derive(Debug, Clone)]
pub struct UseColorSwatchInput<C: ColorValue> {
    /// The color to display.
    pub color: Signal<C>,

    /// An optional override for the accessible color name.
    /// If not provided, the CSS color string is used.
    pub color_name: Option<Signal<String>>,

    /// An optional aria-label override.
    pub aria_label: Option<String>,
}

/// Return value of `use_color_swatch`.
pub struct UseColorSwatchReturn {
    /// Props for the swatch element. Call `.into_attrs()` for view spreading.
    pub props: UseColorSwatchProps,

    /// The CSS background-color string (e.g. `"rgb(128, 64, 32)"`).
    pub background_color: Signal<String>,
}

/// Props from `use_color_swatch`.
#[derive(Debug)]
pub struct UseColorSwatchProps {
    role: AriaRole,
    aria_roledescription: &'static str,
    aria_label: Signal<String>,
}

impl IntoAttrs for UseColorSwatchProps {
    type Attrs = UseColorSwatchAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            attr::custom::custom_attribute("aria-roledescription", self.aria_roledescription),
            Attr(attr::AriaLabel, self.aria_label),
        )
    }
}

pub type UseColorSwatchAttrs = (
    Attr<attr::Role, AriaRole>,
    attr::custom::CustomAttr<&'static str, &'static str>,
    Attr<attr::AriaLabel, Signal<String>>,
);

/// Creates accessible props for a display-only color swatch.
///
/// The swatch element receives `role="img"` and an `aria-roledescription` of
/// "color swatch". The `aria-label` is auto-generated from the CSS color string
/// unless overridden.
///
/// ## Important: `forced-color-adjust`
///
/// Consumers must apply `forced-color-adjust: none` as a CSS property on the
/// swatch element to prevent Windows high contrast mode from overriding the
/// displayed color. The `ColorSwatch` atom handles this automatically.
pub fn use_color_swatch<C: ColorValue>(input: UseColorSwatchInput<C>) -> UseColorSwatchReturn {
    let UseColorSwatchInput {
        color,
        color_name,
        aria_label,
    } = input;

    let background_color = Signal::derive(move || color.get().to_css_string());

    let effective_label = Signal::derive(move || {
        if let Some(ref label) = aria_label {
            return label.clone();
        }
        if let Some(name_signal) = color_name {
            return name_signal.get();
        }
        // Default to CSS color string representation.
        color.get().to_css_string()
    });

    UseColorSwatchReturn {
        props: UseColorSwatchProps {
            role: AriaRole::Img,
            aria_roledescription: "color swatch",
            aria_label: effective_label,
        },
        background_color,
    }
}

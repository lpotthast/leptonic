//! [`Alpha`]: any color with an alpha channel, and [`Color`], the color of any space.

use std::fmt;

use super::{
    AreaGradient, ColorChannel, ColorChannelRange, ColorFormat, ColorSpaceAxes, ColorValue, HSL,
    HSV, RGB8, to_fixed_percent,
};
use crate::utils::{
    locale::WritingDirection,
    number_formatter::{NumberFormatOptions, NumberStyle},
};

/// A color of type `C` with an alpha channel (0: transparent, 1: opaque): react-aria's colors
/// all have one. Its channels are `C`'s plus [`AlphaChannel::Alpha`], so every color component
/// edits alpha too: `<ColorSlider channel={AlphaChannel::<HsvChannel>::Alpha}>` (braces: `view!`
/// can't parse a turbofish in an attribute value). Inside a `ColorPicker`, components of opaque
/// types keep the picker's alpha.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Alpha<C> {
    pub color: C,
    /// From 0 (transparent) to 1 (opaque).
    pub alpha: f64,
}

impl<C> Alpha<C> {
    /// The opaque `color`.
    pub const fn new(color: C) -> Self {
        Self { color, alpha: 1.0 }
    }

    /// The color with `alpha` (clamped to 0 to 1).
    #[must_use]
    pub fn with_alpha(self, alpha: f64) -> Self {
        Self {
            color: self.color,
            alpha: alpha.clamp(0.0, 1.0),
        }
    }
}

impl<C: Default> Default for Alpha<C> {
    fn default() -> Self {
        Self::new(C::default())
    }
}

/// A channel of an [`Alpha`] color: one of the color's, or the alpha.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlphaChannel<Ch> {
    Color(Ch),
    Alpha,
}

impl<Ch: ColorChannel> ColorChannel for AlphaChannel<Ch> {
    type Color = Alpha<Ch::Color>;
}

impl<C: ColorValue> From<Color> for Alpha<C> {
    fn from(color: Color) -> Self {
        Self {
            color: Alpha::new(color.color).to::<C>(),
            alpha: color.alpha,
        }
    }
}

impl<C: ColorValue> From<Alpha<C>> for Color {
    fn from(color: Alpha<C>) -> Self {
        let opaque: Color = color.color.into();
        opaque.with_alpha(opaque.alpha * color.alpha)
    }
}

impl<C: ColorValue> ColorValue for Alpha<C> {
    type Channel = AlphaChannel<C::Channel>;

    const HAS_ALPHA: bool = true;

    fn channel_value(&self, channel: Self::Channel) -> f64 {
        match channel {
            AlphaChannel::Color(channel) => self.color.channel_value(channel),
            AlphaChannel::Alpha => self.alpha,
        }
    }

    fn with_channel_value(&self, channel: Self::Channel, value: f64) -> Self {
        match channel {
            AlphaChannel::Color(channel) => Self {
                color: self.color.with_channel_value(channel, value),
                alpha: self.alpha,
            },
            AlphaChannel::Alpha => self.with_alpha(value),
        }
    }

    fn channel_range(channel: Self::Channel) -> ColorChannelRange {
        match channel {
            AlphaChannel::Color(channel) => C::channel_range(channel),
            AlphaChannel::Alpha => ColorChannelRange {
                min_value: 0.0,
                max_value: 1.0,
                step: 0.01,
                page_size: 0.1,
                gradient_stops: None,
            },
        }
    }

    fn channels() -> Vec<Self::Channel> {
        C::channels().into_iter().map(AlphaChannel::Color).collect()
    }

    fn color_space_axes(
        x_channel: Option<Self::Channel>,
        y_channel: Option<Self::Channel>,
    ) -> ColorSpaceAxes<Self::Channel> {
        // Alpha is no axis of a color space.
        let color_channel = |channel: Option<Self::Channel>| match channel {
            Some(AlphaChannel::Color(channel)) => Some(channel),
            _ => None,
        };
        let ColorSpaceAxes { x, y, z } =
            C::color_space_axes(color_channel(x_channel), color_channel(y_channel));
        ColorSpaceAxes {
            x: AlphaChannel::Color(x),
            y: AlphaChannel::Color(y),
            z: AlphaChannel::Color(z),
        }
    }

    fn to_css_string(&self) -> String {
        self.color.to_css_string_with_alpha(self.alpha)
    }

    fn is_alpha_channel(channel: Self::Channel) -> bool {
        channel == AlphaChannel::Alpha
    }

    fn channel_name(channel: Self::Channel) -> &'static str {
        match channel {
            AlphaChannel::Color(channel) => C::channel_name(channel),
            AlphaChannel::Alpha => "Alpha",
        }
    }

    fn channel_format_options(channel: Self::Channel) -> NumberFormatOptions {
        match channel {
            AlphaChannel::Color(channel) => C::channel_format_options(channel),
            AlphaChannel::Alpha => NumberFormatOptions {
                style: NumberStyle::Percent,
                ..NumberFormatOptions::default()
            },
        }
    }

    fn display_color(&self, channel: Self::Channel) -> Self {
        match channel {
            AlphaChannel::Color(channel) => Self::new(self.color.display_color(channel)),
            AlphaChannel::Alpha => *self,
        }
    }

    fn opaque(&self) -> Self {
        Self::new(self.color)
    }

    fn to_rgb8(&self) -> RGB8 {
        self.color.to_rgb8()
    }

    fn hue_channel() -> Option<Self::Channel> {
        C::hue_channel().map(AlphaChannel::Color)
    }

    /// The color's name, with its transparency (react-aria: e.g. "vibrant red, 80% transparent").
    fn color_name(&self) -> String {
        let name = self.color.color_name();
        if self.alpha >= 1.0 {
            return name;
        }
        format!("{name}, {:.0}% transparent", (1.0 - self.alpha) * 100.0)
    }

    fn hue_name(&self) -> String {
        self.color.hue_name()
    }

    fn area_gradient(
        &self,
        x_channel: Self::Channel,
        y_channel: Self::Channel,
        direction: WritingDirection,
    ) -> AreaGradient {
        let (AlphaChannel::Color(x), AlphaChannel::Color(y)) = (x_channel, y_channel) else {
            // Alpha is no axis of a color space: the color's default axes.
            let ColorSpaceAxes { x, y, .. } = C::color_space_axes(None, None);
            return self.color.area_gradient(x, y, direction);
        };
        self.color.area_gradient(x, y, direction)
    }
}

/// A color without alpha in any of leptonic's color spaces, kept in the space it was set in.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpaqueColor {
    Rgb(RGB8),
    Hsv(HSV),
    Hsl(HSL),
}

impl OpaqueColor {
    fn to_css_string_with_alpha(self, alpha: f64) -> String {
        match self {
            Self::Rgb(rgb) => rgb.to_css_string_with_alpha(alpha),
            Self::Hsv(hsv) => hsv.to_css_string_with_alpha(alpha),
            Self::Hsl(hsl) => hsl.to_css_string_with_alpha(alpha),
        }
    }
}

/// A color in any of leptonic's color spaces with alpha, kept in the space it was set in
/// (react-aria's `Color`): a gray set as HSV keeps its hue, which RGB would lose. Components of
/// different spaces share one (e.g. a `ColorPicker`'s); read it in a space with [`Color::to`]
/// (an opaque type drops the alpha, an `Alpha<C>` keeps it).
///
/// Parses from CSS-like text as react-aria's `parseColor`: `#rgb`, `#rgba`, `#rrggbb`,
/// `#rrggbbaa`, `rgb(r, g, b)`, `rgba(r, g, b, a)`, `hsb(h, s%, b%)`, `hsba(..)`,
/// `hsl(h, s%, l%)` and `hsla(..)`. Displays as a CSS color.
pub type Color = Alpha<OpaqueColor>;

impl Alpha<OpaqueColor> {
    /// The color in the space `C`.
    #[must_use]
    pub fn to<C: ColorValue>(self) -> C {
        C::from(self)
    }

    /// The color as a CSS color.
    #[must_use]
    pub fn to_css_string(self) -> String {
        self.color.to_css_string_with_alpha(self.alpha)
    }

    /// The color as text in `format` (react-aria's `toString(format)`): converted into the
    /// format's space, percentages with up to two decimals.
    #[must_use]
    pub fn to_string_as(self, format: ColorFormat) -> String {
        let alpha = self.alpha;
        let rgb = || self.to::<RGB8>();
        match format {
            ColorFormat::Css => self.to_css_string(),
            ColorFormat::Hex => rgb().to_string(),
            ColorFormat::Hexa => {
                // `alpha` is within 0 to 1.
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let alpha = (alpha * 255.0).round() as u8;
                format!("{}{alpha:02X}", rgb())
            }
            ColorFormat::Rgb => rgb().to_css_string(),
            ColorFormat::Rgba => {
                let RGB8 { r, g, b } = rgb();
                format!("rgba({r}, {g}, {b}, {alpha})")
            }
            ColorFormat::Hsl | ColorFormat::Hsla => {
                let HSL {
                    hue,
                    saturation,
                    lightness,
                } = self.to::<HSL>();
                let (s, l) = (to_fixed_percent(saturation), to_fixed_percent(lightness));
                if format == ColorFormat::Hsl {
                    format!("hsl({hue}, {s}%, {l}%)")
                } else {
                    format!("hsla({hue}, {s}%, {l}%, {alpha})")
                }
            }
            ColorFormat::Hsb | ColorFormat::Hsba => {
                let HSV {
                    hue,
                    saturation,
                    brightness,
                } = self.to::<HSV>();
                let (s, b) = (to_fixed_percent(saturation), to_fixed_percent(brightness));
                if format == ColorFormat::Hsb {
                    format!("hsb({hue}, {s}%, {b}%)")
                } else {
                    format!("hsba({hue}, {s}%, {b}%, {alpha})")
                }
            }
        }
    }

    /// The color's name, e.g. "dark vibrant blue" or "vibrant red, 80% transparent".
    #[must_use]
    pub fn color_name(self) -> String {
        self.to::<Alpha<RGB8>>().color_name()
    }
}

/// Black (react-aria's default color).
impl Default for Alpha<OpaqueColor> {
    fn default() -> Self {
        Self::new(OpaqueColor::Rgb(RGB8::new()))
    }
}

macro_rules! opaque_color_conversions {
    ($($type:ident => $variant:ident),*) => {$(
        impl From<$type> for Color {
            fn from(color: $type) -> Self {
                Self::new(OpaqueColor::$variant(color))
            }
        }

        /// Drops the alpha.
        impl From<Color> for $type {
            fn from(color: Color) -> Self {
                match color.color {
                    OpaqueColor::Rgb(rgb) => rgb.into(),
                    OpaqueColor::Hsv(hsv) => hsv.into(),
                    OpaqueColor::Hsl(hsl) => hsl.into(),
                }
            }
        }
    )*};
}
opaque_color_conversions!(RGB8 => Rgb, HSV => Hsv, HSL => Hsl);

/// A color shown by a component (e.g. a `ColorSwatch`): any color value or signal of one
/// (`RGB8`, `HSV`, `Color`, `Signal<HSL>`, `RwSignal<RGB8>`, `Memo<Color>`, ...).
#[derive(Debug, Clone, Copy)]
pub struct ColorProp(pub leptos::prelude::Signal<Color>);

impl From<Color> for ColorProp {
    fn from(color: Color) -> Self {
        Self(leptos::prelude::Signal::stored(color))
    }
}

impl<C: ColorValue> From<C> for ColorProp {
    fn from(color: C) -> Self {
        Self(leptos::prelude::Signal::stored(color.into()))
    }
}

impl From<leptos::prelude::Signal<Color>> for ColorProp {
    fn from(color: leptos::prelude::Signal<Color>) -> Self {
        Self(color)
    }
}

macro_rules! color_prop_from_signals {
    ($($signal:ident),*) => {$(
        impl<C: ColorValue> From<leptos::prelude::$signal<C>> for ColorProp {
            fn from(color: leptos::prelude::$signal<C>) -> Self {
                use leptos::prelude::Get;
                Self(leptos::prelude::Signal::derive(move || color.get().into()))
            }
        }
    )*};
}
color_prop_from_signals!(Signal, ReadSignal, RwSignal, Memo);

impl From<leptos::prelude::RwSignal<Color>> for ColorProp {
    fn from(color: leptos::prelude::RwSignal<Color>) -> Self {
        Self(color.into())
    }
}

impl fmt::Display for Alpha<OpaqueColor> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_css_string())
    }
}

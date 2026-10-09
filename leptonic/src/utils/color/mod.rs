// Upstream: react-stately/src/color/Color.ts @ 99e6102368
// Upstream: react-aria/src/color/useColorAreaGradient.ts @ 99e6102368
//! Colors in the RGB, HSV (HSB) and HSL color spaces, with an optional alpha channel.
//
// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - One type per color space (`RGB8`, `HSV`, `HSL`), generic code over the `ColorValue` trait
//   with typed channels per space (react-aria: one `Color` class with string channels and a
//   `colorSpace` prop converting at runtime). Callers convert between spaces with `From`/`Into`.
//   `Color` (an `Alpha<OpaqueColor>`) is the color of any space shared between components.
// - Alpha is opt-in: `Alpha<C>` adds it to any color type (react-aria: every color has one).
// - `toString(format)` is [`Color::to_string_as`] with a [`ColorFormat`]; `parseColor` is
//   `FromStr` for [`Color`], hex parsing `FromStr` for [`RGB8`].
// - `getColorSpaceAxes` returns a [`ColorSpaceAxes`] struct (react-aria: an object of strings).
//
// ## DIFFERENT BEHAVIOR
// - HSV/HSL saturation, brightness and lightness range from 0 to 1 (step 0.01, page size 0.1;
//   react-aria: 0 to 100, step 1, page size 10). This shows wherever the raw channel value does:
//   the hidden range inputs' `value`, `min`, `max` and `step` (so `aria-valuenow` reads e.g.
//   0.5), form values submitted by those inputs, and `on_change` values. Formatted values
//   (`aria-valuetext`, outputs, channel fields) are percentages as react-aria's.
// - Channel values are formatted with leptonic's ICU4X `NumberFormatter` (react-aria:
//   `Intl.NumberFormat`), with the channel's format options.
//
// =============================================================================

mod alpha;
mod hsl;
mod hsv;
mod naming;
mod parse;
mod rgb;
#[cfg(test)]
mod tests;

use std::{cell::RefCell, fmt, hash::Hash};

pub use alpha::{Alpha, AlphaChannel, Color, ColorProp, OpaqueColor};
pub use hsl::{HSL, HslChannel};
pub use hsv::{HSV, HsvChannel};
pub use parse::ParseColorError;
pub use rgb::{RGB8, RgbChannel};

use crate::utils::{
    i18n::{Locale, WritingDirection},
    math::to_fixed_number,
    number_formatter::{NumberFormatOptions, NumberFormatter},
};

/// Describes the numeric range, step, and page size of a color channel.
#[derive(Debug, Clone, Copy)]
pub struct ColorChannelRange {
    pub min_value: f64,
    pub max_value: f64,
    pub step: f64,
    pub page_size: f64,
}

/// The channels of a color space on the axes of a 2D color area (react-aria's
/// `getColorSpaceAxes`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorSpaceAxes<Ch> {
    /// The channel on the horizontal axis.
    pub x: Ch,
    /// The channel on the vertical axis.
    pub y: Ch,
    /// The third channel, fixed in the area.
    pub z: Ch,
}

/// A text format of a color (react-aria's `ColorFormat` plus `'css'`), for
/// [`Color::to_string_as`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorFormat {
    /// `#RRGGBB`.
    Hex,
    /// `#RRGGBBAA`.
    Hexa,
    /// `rgb(r, g, b)`.
    Rgb,
    /// `rgba(r, g, b, a)`.
    Rgba,
    /// `hsl(h, s%, l%)`.
    Hsl,
    /// `hsla(h, s%, l%, a)`.
    Hsla,
    /// `hsb(h, s%, b%)`.
    Hsb,
    /// `hsba(h, s%, b%, a)`.
    Hsba,
    /// A CSS color in the color's own space (an `rgb()` color; `rgba()` when transparent).
    Css,
}

/// A channel of a color space. It names its color type, so components taking a channel infer
/// the color type from it (`<ColorSlider channel=HsvChannel::Hue>` is an HSV slider).
pub trait ColorChannel:
    fmt::Debug + Clone + Copy + PartialEq + Eq + Hash + Send + Sync + 'static
{
    /// The color type whose channel this is.
    type Color: ColorValue<Channel = Self>;
}

impl ColorChannel for HsvChannel {
    type Color = HSV;
}

impl ColorChannel for HslChannel {
    type Color = HSL;
}

impl ColorChannel for RgbChannel {
    type Color = RGB8;
}

/// A color value with typed, color-space-specific channels.
///
/// Each implementor defines its own `Channel` enum so that hooks can be generic
/// over the color type while remaining type-safe (e.g., you cannot request the
/// `Hue` channel from an `RGB8` value).
pub trait ColorValue:
    Clone + Copy + PartialEq + fmt::Debug + Send + Sync + From<Color> + Into<Color> + 'static
{
    /// The channel enum for this color space.
    type Channel: ColorChannel<Color = Self>;

    /// Whether the type has an alpha channel (`Alpha<C>`).
    const HAS_ALPHA: bool = false;

    /// The numeric value of `channel`.
    fn channel_value(&self, channel: Self::Channel) -> f64;

    /// The color with `channel` set to `value`.
    #[must_use]
    fn with_channel_value(&self, channel: Self::Channel, value: f64) -> Self;

    /// The range, step and page size of `channel`.
    fn channel_range(channel: Self::Channel) -> ColorChannelRange;

    /// The channels of this color space (without alpha), in react-aria's order.
    fn channels() -> [Self::Channel; 3];

    /// The axes of a 2D color area for the preferred x and y channels: the space's first
    /// channels by default (react-aria's `getColorSpaceAxes`), and the remaining one as z.
    fn color_space_axes(
        x_channel: Option<Self::Channel>,
        y_channel: Option<Self::Channel>,
    ) -> ColorSpaceAxes<Self::Channel>;

    /// A CSS color (e.g. `"rgb(128, 0, 255)"`).
    fn to_css_string(&self) -> String;

    /// The color with an alpha (0 to 1) as a CSS color (`rgba(..)` when transparent).
    fn to_css_string_with_alpha(&self, alpha: f64) -> String {
        if alpha >= 1.0 {
            return self.to_css_string();
        }
        let RGB8 { r, g, b } = self.to_rgb8();
        format!("rgba({r}, {g}, {b}, {})", round_alpha(alpha))
    }

    /// Whether `channel` is an alpha channel (whose value text names no color, as react-aria's).
    fn is_alpha_channel(_channel: Self::Channel) -> bool {
        false
    }

    /// How `channel`'s value is formatted (react-aria's `getChannelFormatOptions`): degrees for
    /// a hue, percent for saturation, brightness, lightness and alpha, decimal for RGB.
    fn channel_format_options(channel: Self::Channel) -> NumberFormatOptions;

    /// The value of `channel` formatted for `locale` (react-aria's `formatChannelValue`), e.g.
    /// "128", "50%" or "120°".
    fn format_channel_value(&self, channel: Self::Channel, locale: &Locale) -> String {
        channel_formatter(locale, &Self::channel_format_options(channel))
            .format(self.channel_value(channel))
    }

    /// The name of `channel` in `locale` (e.g. "Hue", "Red"; react-aria's `getChannelName`).
    fn channel_name(channel: Self::Channel, locale: &Locale) -> String;

    /// The color to draw a gradient of `channel` with (react-aria's `getDisplayColor`): a hue at
    /// full saturation, other channels as they are (`Alpha` makes them opaque, except for its
    /// alpha channel).
    #[must_use]
    fn display_color(&self, channel: Self::Channel) -> Self;

    /// The color without transparency (only `Alpha` has any).
    #[must_use]
    fn opaque(&self) -> Self {
        *self
    }

    /// The color in RGB (for its name and contrast).
    fn to_rgb8(&self) -> RGB8;

    /// The color space's hue channel, if it has one.
    fn hue_channel() -> Option<Self::Channel> {
        None
    }

    /// The color's name in `locale`, e.g. "vibrant red" or "very dark grayish blue" (react-aria's
    /// `getColorName`).
    fn color_name(&self, locale: &Locale) -> String {
        naming::color_name(self.to_rgb8(), 1.0, locale)
    }

    /// The name of the color's hue in `locale`, e.g. "red orange" (react-aria's `getHueName`).
    fn hue_name(&self, locale: &Locale) -> String {
        naming::hue_name(self.to_rgb8(), locale)
    }

    /// The values of `channel` at which a slider's gradient has its stops (react-aria's
    /// `useColorSlider`): every 60° of a hue, the middle too of a lightness (its color shows only
    /// there), else the channel's ends.
    fn gradient_stops(channel: Self::Channel) -> &'static [f64];

    /// The CSS background of a 2D color area showing `x_channel` × `y_channel`
    /// (react-aria's `useColorAreaGradient`).
    ///
    /// The default is an approximation of two layers (min-Y to transparent, min-X to max-X); the
    /// built-in color types layer the channels as react-aria does.
    fn area_gradient(
        &self,
        x_channel: Self::Channel,
        y_channel: Self::Channel,
        direction: WritingDirection,
    ) -> AreaGradient {
        let end = line_end(direction);
        let x_range = Self::channel_range(x_channel);
        let y_range = Self::channel_range(y_channel);
        let top_right = self
            .with_channel_value(x_channel, x_range.max_value)
            .with_channel_value(y_channel, y_range.max_value);
        let top_left = self
            .with_channel_value(x_channel, x_range.min_value)
            .with_channel_value(y_channel, y_range.max_value);
        let bottom = self.with_channel_value(y_channel, y_range.min_value);
        AreaGradient {
            background: format!(
                "linear-gradient(to top, {} 0%, transparent 100%), \
                 linear-gradient(to {end}, {} 0%, {} 100%)",
                bottom.to_css_string(),
                top_left.to_css_string(),
                top_right.to_css_string(),
            ),
            blend_mode: None,
        }
    }
}

/// CSS background description for a 2D color area.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AreaGradient {
    /// CSS `background` property value (may contain multiple layers).
    pub background: String,
    /// How the background's layers blend, if they do (RGB areas: [`BlendMode::Screen`]).
    pub blend_mode: Option<BlendMode>,
}

/// A CSS `background-blend-mode` an area gradient needs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    /// Lightens: the layers' colors add up (an RGB area's red, green and blue layers).
    Screen,
}

impl BlendMode {
    /// The CSS keyword.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Screen => "screen",
        }
    }
}

thread_local! {
    /// The channel formatters built so far, per locale and format: ICU4X formatters are costly
    /// to build, and a drag formats channel values on every change.
    static CHANNEL_FORMATTERS: RefCell<Vec<(Locale, NumberFormatOptions, NumberFormatter)>> =
        const { RefCell::new(Vec::new()) };
}

/// Formatters kept at most (a few channel formats per locale in use).
const MAX_CHANNEL_FORMATTERS: usize = 16;

/// A formatter for `locale` and `options`, built once (react-aria builds one per value).
pub(crate) fn channel_formatter(locale: &Locale, options: &NumberFormatOptions) -> NumberFormatter {
    CHANNEL_FORMATTERS.with_borrow_mut(|formatters| {
        if let Some((.., formatter)) =
            formatters
                .iter()
                .find(|(cached_locale, cached_options, _)| {
                    cached_locale == locale && cached_options == options
                })
        {
            return formatter.clone();
        }
        let formatter = NumberFormatter::new(locale, options.clone());
        if formatters.len() == MAX_CHANNEL_FORMATTERS {
            formatters.remove(0);
        }
        formatters.push((locale.clone(), options.clone(), formatter.clone()));
        formatter
    })
}

/// A fraction (0 to 1) as a percentage with up to two decimals (react-aria's
/// `toFixedNumber(value, 2)` of its 0 to 100 values).
fn to_fixed_percent(fraction: f64) -> f64 {
    to_fixed_number(fraction * 100.0, 2)
}

/// A fraction (0 to 1) rounded as react-aria rounds the percentage in conversions (two decimals
/// of the percentage).
fn round_fraction(fraction: f64) -> f64 {
    to_fixed_number(fraction, 4)
}

/// Rounds an alpha for CSS (two decimals, as react-aria's percentages).
fn round_alpha(alpha: f64) -> f64 {
    (alpha.clamp(0.0, 1.0) * 100.0).round() / 100.0
}

/// The direction of a gradient along the x axis: towards the end of the line (react-aria:
/// `to left` in right-to-left).
const fn line_end(direction: WritingDirection) -> &'static str {
    match direction {
        WritingDirection::Ltr => "right",
        WritingDirection::Rtl => "left",
    }
}

/// The hues a gradient of hues has its stops at (react-aria: every 60°).
const HUE_STOPS: [f64; 7] = [0.0, 60.0, 120.0, 180.0, 240.0, 300.0, 360.0];

/// The hue stops of a gradient: `color` at the hues 0, 60, ..., 360 (react-aria's `hue`).
fn hue_stops<C: ColorValue>(color: C, hue: C::Channel) -> String {
    HUE_STOPS
        .iter()
        .map(|&degrees| color.with_channel_value(hue, degrees).to_css_string())
        .collect::<Vec<_>>()
        .join(", ")
}

/// The area gradient of a hue-based space (HSV, HSL; react-aria's `useColorAreaGradient`): one
/// gradient per area channel, `stops` giving its colors, layered in the space's channel order
/// with the later channel on top (brightness/lightness above saturation above hue); the vivid
/// `base` color below them when the hue is z.
fn hue_space_area_gradient<C: ColorValue>(
    base: C,
    axes: ColorSpaceAxes<C::Channel>,
    direction: WritingDirection,
    stops: impl Fn(C::Channel) -> String,
) -> AreaGradient {
    let end = line_end(direction);
    let mut layers: Vec<String> = C::channels()
        .into_iter()
        .filter(|channel| *channel != axes.z)
        .map(|channel| {
            let to = if channel == axes.x { end } else { "top" };
            format!("linear-gradient(to {to}, {})", stops(channel))
        })
        .rev()
        .collect();
    if C::hue_channel() == Some(axes.z) {
        layers.push(base.to_css_string());
    }
    AreaGradient {
        background: layers.join(", "),
        blend_mode: None,
    }
}

/// The axes of a 2D area in a space with `channels` (react-aria's `getColorSpaceAxes`): x and y
/// default to the first channels not taken, z is the remaining one.
fn axes_of<Ch: ColorChannel>(
    channels: [Ch; 3],
    x_channel: Option<Ch>,
    y_channel: Option<Ch>,
) -> ColorSpaceAxes<Ch> {
    let x = x_channel
        .or_else(|| channels.into_iter().find(|ch| Some(*ch) != y_channel))
        .unwrap_or(channels[0]);
    let y = y_channel
        .or_else(|| channels.into_iter().find(|ch| *ch != x))
        .unwrap_or(channels[1]);
    let z = channels
        .into_iter()
        .find(|ch| *ch != x && *ch != y)
        .unwrap_or(channels[2]);
    ColorSpaceAxes { x, y, z }
}

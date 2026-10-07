use crate::utils::{
    locale::WritingDirection,
    number_formatter::{NumberFormatOptions, NumberStyle, UnitDisplay},
};
use std::{fmt, hash::Hash, str::FromStr};

// REACT-ARIA DEVIATIONS (color types)
//
// ## Color Space Conversion
// React-aria hooks accept a `colorSpace` prop for runtime conversion.
// Leptonic uses `<C: ColorValue>` generics — the color space is fixed at the
// type level. Callers convert between spaces explicitly before passing to hooks.
// This is idiomatic Rust (compile-time type safety vs. runtime flexibility).
//
// ## Channel Ranges
// HSV/HSL saturation, brightness, and lightness use normalized 0.0–1.0 range
// (step=0.01, page_size=0.1). React-aria uses 0–100 percentage (step=1,
// page_size=10). Display formatting multiplies by 100 for user-facing values.
//
// ## Color Naming
// `color_name()`/`hue_name()` port react-aria's OKLCH-based names (e.g. "dark vibrant blue"),
// in English until leptonic has localized strings.
//
// ## Channel Name Localization
// `get_channel_name()` returns hardcoded English strings. React-aria uses
// localized strings via `LocalizedStringDictionary`. Can be added using the
// ICU4X infrastructure already in place for other modules.
//
// ## Alpha Channel
// Opt-in: `Alpha<C>` adds an alpha channel to any color type (react-aria: every color has one).
// `Color`, the color shared between components, is an `Alpha<OpaqueColor>`.

/// A channel of the HSV color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HsvChannel {
    Hue,
    Saturation,
    Brightness,
}

/// A channel of the HSL color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HslChannel {
    Hue,
    Saturation,
    Lightness,
}

/// A channel of the RGB color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RgbChannel {
    Red,
    Green,
    Blue,
}

/// Describes the numeric range, step, and page size of a color channel.
#[derive(Debug, Clone, Copy)]
pub struct ColorChannelRange {
    pub min_value: f64,
    pub max_value: f64,
    pub step: f64,
    pub page_size: f64,
    /// Fixed gradient stop values for this channel. When `Some`, these specific
    /// values should be used as gradient stops (e.g., hue at 60-degree boundaries).
    /// When `None`, the gradient should use just min and max as stops.
    pub gradient_stops: Option<&'static [f64]>,
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

    /// Returns the numeric value of the given channel.
    fn get_channel_value(&self, channel: Self::Channel) -> f64;

    /// Returns a new color with the given channel set to `value`.
    #[must_use]
    fn with_channel_value(&self, channel: Self::Channel, value: f64) -> Self;

    /// Returns the valid range, step, and page size for a channel.
    fn get_channel_range(channel: Self::Channel) -> ColorChannelRange;

    /// Whether the type has an alpha channel (`Alpha<C>`).
    const HAS_ALPHA: bool = false;

    /// Returns all channels of this color space (excluding alpha).
    fn channels() -> Vec<Self::Channel>;

    /// Given optional x and y channel preferences, returns (x, y, z) axes.
    ///
    /// If both are `None`, a sensible default is chosen.
    /// The z channel is whichever channel is not assigned to x or y.
    fn get_color_space_axes(
        x_channel: Option<Self::Channel>,
        y_channel: Option<Self::Channel>,
    ) -> (Self::Channel, Self::Channel, Self::Channel);

    /// Returns a CSS color string representation (e.g. `"rgb(128, 0, 255)"`).
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

    /// Formats the value of a channel for display (e.g. `"128"`, `"0.50"`).
    fn format_channel_value(&self, channel: Self::Channel) -> String;

    /// Returns the human-readable name of a channel (e.g. `"Hue"`, `"Red"`).
    fn get_channel_name(channel: Self::Channel) -> &'static str;

    /// How a channel's value is formatted in a number field (react-aria's
    /// `getChannelFormatOptions`). Default: decimal.
    fn get_channel_format_options(_channel: Self::Channel) -> NumberFormatOptions {
        NumberFormatOptions::default()
    }

    /// Returns the color to display for gradient rendering of the given channel.
    ///
    /// For hue channels, this returns a fully saturated/bright version so the
    /// gradient shows vivid hues regardless of the current saturation/brightness.
    /// For other channels, returns the color as-is (`Alpha` makes it opaque, except for its
    /// alpha channel, as react-aria).
    #[must_use]
    fn get_display_color(&self, channel: Self::Channel) -> Self;

    /// The color in RGB (for its name and contrast).
    fn to_rgb8(&self) -> RGB8;

    /// The color space's hue channel, if it has one.
    fn hue_channel() -> Option<Self::Channel> {
        None
    }

    /// The color's name in English, e.g. "vibrant red" or "very dark grayish blue" (react-aria's
    /// `getColorName`).
    fn color_name(&self) -> String {
        naming::color_name(self.to_rgb8())
    }

    /// The name of the color's hue in English, e.g. "red orange" (react-aria's `getHueName`).
    fn hue_name(&self) -> String {
        naming::hue_name(self.to_rgb8())
    }

    /// Returns a CSS background for a 2D color area displaying `x_channel` × `y_channel`.
    ///
    /// The default implementation produces a simple two-layer gradient (min-Y to
    /// transparent, min-X to max-X). This is a rough approximation that may not
    /// correctly handle all axis configurations. The built-in color types (HSV, HSL,
    /// RGB8) override this with layered gradients matching react-aria's strategy.
    fn get_area_gradient(
        &self,
        x_channel: Self::Channel,
        y_channel: Self::Channel,
        direction: WritingDirection,
    ) -> AreaGradient {
        // The x axis runs towards the end of the line (react-aria: `to left` in right-to-left).
        let end = match direction {
            WritingDirection::Ltr => "right",
            WritingDirection::Rtl => "left",
        };
        let x_range = Self::get_channel_range(x_channel);
        let y_range = Self::get_channel_range(y_channel);

        // Color at max X, max Y (e.g., pure hue for HSV).
        let top_right = self
            .with_channel_value(x_channel, x_range.max_value)
            .with_channel_value(y_channel, y_range.max_value);
        // Color at min X, max Y (e.g., white for HSV).
        let top_left = self
            .with_channel_value(x_channel, x_range.min_value)
            .with_channel_value(y_channel, y_range.max_value);
        // Color at min Y (e.g., black for HSV).
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

/// Rounds an alpha for CSS (two decimals, as react-aria's percentages).
fn round_alpha(alpha: f64) -> f64 {
    (alpha.clamp(0.0, 1.0) * 100.0).round() / 100.0
}

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

    fn get_channel_value(&self, channel: Self::Channel) -> f64 {
        match channel {
            AlphaChannel::Color(channel) => self.color.get_channel_value(channel),
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

    fn get_channel_range(channel: Self::Channel) -> ColorChannelRange {
        match channel {
            AlphaChannel::Color(channel) => C::get_channel_range(channel),
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

    fn get_color_space_axes(
        x_channel: Option<Self::Channel>,
        y_channel: Option<Self::Channel>,
    ) -> (Self::Channel, Self::Channel, Self::Channel) {
        // Alpha is no axis of a color space.
        let color_channel = |channel: Option<Self::Channel>| match channel {
            Some(AlphaChannel::Color(channel)) => Some(channel),
            _ => None,
        };
        let (x, y, z) = C::get_color_space_axes(color_channel(x_channel), color_channel(y_channel));
        (
            AlphaChannel::Color(x),
            AlphaChannel::Color(y),
            AlphaChannel::Color(z),
        )
    }

    fn to_css_string(&self) -> String {
        self.color.to_css_string_with_alpha(self.alpha)
    }

    fn is_alpha_channel(channel: Self::Channel) -> bool {
        channel == AlphaChannel::Alpha
    }

    fn format_channel_value(&self, channel: Self::Channel) -> String {
        match channel {
            AlphaChannel::Color(channel) => self.color.format_channel_value(channel),
            AlphaChannel::Alpha => format!("{:.0}%", self.alpha * 100.0),
        }
    }

    fn get_channel_name(channel: Self::Channel) -> &'static str {
        match channel {
            AlphaChannel::Color(channel) => C::get_channel_name(channel),
            AlphaChannel::Alpha => "Alpha",
        }
    }

    fn get_channel_format_options(channel: Self::Channel) -> NumberFormatOptions {
        match channel {
            AlphaChannel::Color(channel) => C::get_channel_format_options(channel),
            AlphaChannel::Alpha => NumberFormatOptions {
                style: NumberStyle::Percent,
                ..NumberFormatOptions::default()
            },
        }
    }

    fn get_display_color(&self, channel: Self::Channel) -> Self {
        match channel {
            AlphaChannel::Color(channel) => Self::new(self.color.get_display_color(channel)),
            AlphaChannel::Alpha => *self,
        }
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

    fn get_area_gradient(
        &self,
        x_channel: Self::Channel,
        y_channel: Self::Channel,
        direction: WritingDirection,
    ) -> AreaGradient {
        let (AlphaChannel::Color(x), AlphaChannel::Color(y)) = (x_channel, y_channel) else {
            // Alpha is no axis of a color space: the color's default axes.
            let (x, y, _) = C::get_color_space_axes(None, None);
            return self.color.get_area_gradient(x, y, direction);
        };
        self.color.get_area_gradient(x, y, direction)
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

/// Text that is no color [`Color`] can parse.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseColorError(String);

impl fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid color: {:?}", self.0)
    }
}

impl std::error::Error for ParseColorError {}

impl FromStr for Alpha<OpaqueColor> {
    type Err = ParseColorError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        parse_color(text.trim()).ok_or_else(|| ParseColorError(text.to_owned()))
    }
}

/// The comma-separated arguments of `name(...)`: 3, or 4 for `alpha_name(...)` (the last the
/// alpha).
fn css_arguments<'a>(text: &'a str, name: &str, alpha_name: &str) -> Option<(Vec<&'a str>, bool)> {
    let (inner, with_alpha) = match text.strip_prefix(alpha_name) {
        Some(rest) => (rest, true),
        None => (text.strip_prefix(name)?, false),
    };
    let inner = inner.strip_prefix('(')?.strip_suffix(')')?;
    let arguments: Vec<&str> = inner.split(',').map(str::trim).collect();
    (arguments.len() == if with_alpha { 4 } else { 3 }).then_some((arguments, with_alpha))
}

/// A hex color with alpha: `#rgba` or `#rrggbbaa`.
fn parse_hex_with_alpha(text: &str) -> Option<Color> {
    let digits = text.strip_prefix('#')?;
    // Only hex digits: slicing by byte length needs ASCII, and `from_str_radix` takes a sign.
    if !digits.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    let (color, alpha) = match digits.len() {
        4 => (&digits[..3], digits[3..].repeat(2)),
        8 => (&digits[..6], digits[6..].to_owned()),
        _ => return None,
    };
    let alpha = u8::from_str_radix(&alpha, 16).ok()?;
    Some(Color::from(RGB8::from_hex(color)?).with_alpha(f64::from(alpha) / 255.0))
}

/// react-aria's `parseColor`: RGB (hex or `rgb()`/`rgba()`), then HSB, then HSL.
fn parse_color(text: &str) -> Option<Color> {
    if text.starts_with('#') {
        return RGB8::from_hex(text)
            .map(Color::from)
            .or_else(|| parse_hex_with_alpha(text));
    }
    let number = |text: &str| text.parse::<f64>().ok().filter(|n| n.is_finite());
    let percent = |text: &str| {
        text.strip_suffix('%')
            .and_then(number)
            .map(|n| n.clamp(0.0, 100.0) / 100.0)
    };
    let hue = |text: &str| {
        number(text).map(|hue| {
            #[allow(clippy::float_cmp)]
            if hue == 360.0 {
                hue
            } else {
                hue.rem_euclid(360.0)
            }
        })
    };
    let alpha = |arguments: &[&str], with_alpha: bool| {
        if with_alpha {
            number(arguments[3]).map(|alpha| alpha.clamp(0.0, 1.0))
        } else {
            Some(1.0)
        }
    };
    if let Some((arguments, with_alpha)) = css_arguments(text, "rgb", "rgba") {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let channel = |text: &str| number(text).map(|n| n.clamp(0.0, 255.0).round() as u8);
        let color = RGB8 {
            r: channel(arguments[0])?,
            g: channel(arguments[1])?,
            b: channel(arguments[2])?,
        };
        return Some(Color::from(color).with_alpha(alpha(&arguments, with_alpha)?));
    }
    if let Some((arguments, with_alpha)) = css_arguments(text, "hsb", "hsba") {
        let color = HSV {
            hue: hue(arguments[0])?,
            saturation: percent(arguments[1])?,
            value: percent(arguments[2])?,
        };
        return Some(Color::from(color).with_alpha(alpha(&arguments, with_alpha)?));
    }
    let (arguments, with_alpha) = css_arguments(text, "hsl", "hsla")?;
    let color = HSL {
        hue: hue(arguments[0])?,
        saturation: percent(arguments[1])?,
        lightness: percent(arguments[2])?,
    };
    Some(Color::from(color).with_alpha(alpha(&arguments, with_alpha)?))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HSV {
    pub hue: f64,
    pub saturation: f64,
    pub value: f64,
}

impl HSV {
    /// Hue starting at red (0.0 degrees) with full saturation (1.0) and full value (1.0).
    /// This is a sensitive way to construct a new HSV value.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            hue: 0.0,
            saturation: 1.0,
            value: 1.0,
        }
    }

    #[must_use]
    pub const fn from_hue_fully_saturated(hue: f64) -> Self {
        Self {
            hue,
            saturation: 1.0,
            value: 1.0,
        }
    }

    #[must_use]
    pub const fn with_hue(self, hue: f64) -> Self {
        Self {
            hue,
            saturation: self.saturation,
            value: self.value,
        }
    }

    #[must_use]
    pub const fn with_saturation(self, saturation: f64) -> Self {
        Self {
            hue: self.hue,
            saturation,
            value: self.value,
        }
    }

    #[must_use]
    pub const fn with_value(self, value: f64) -> Self {
        Self {
            hue: self.hue,
            saturation: self.saturation,
            value,
        }
    }

    pub fn into_rgb8(self) -> RGB8 {
        RGB8::from(self)
    }
}

impl Default for HSV {
    fn default() -> Self {
        Self::new()
    }
}

impl ColorValue for HSV {
    fn hue_channel() -> Option<HsvChannel> {
        Some(HsvChannel::Hue)
    }

    fn to_rgb8(&self) -> RGB8 {
        RGB8::from(*self)
    }

    type Channel = HsvChannel;

    fn get_channel_value(&self, channel: HsvChannel) -> f64 {
        match channel {
            HsvChannel::Hue => self.hue,
            HsvChannel::Saturation => self.saturation,
            HsvChannel::Brightness => self.value,
        }
    }

    fn with_channel_value(&self, channel: HsvChannel, value: f64) -> Self {
        match channel {
            HsvChannel::Hue => self.with_hue(value),
            HsvChannel::Saturation => self.with_saturation(value),
            HsvChannel::Brightness => self.with_value(value),
        }
    }

    fn get_channel_range(channel: HsvChannel) -> ColorChannelRange {
        match channel {
            HsvChannel::Hue => ColorChannelRange {
                min_value: 0.0,
                max_value: 360.0,
                step: 1.0,
                page_size: 15.0,
                gradient_stops: Some(&[0.0, 60.0, 120.0, 180.0, 240.0, 300.0, 360.0]),
            },
            // REACT-ARIA DEVIATION: We use normalized 0.0–1.0 range with step=0.01
            // and page_size=0.1. React-aria uses 0–100 (percent) with step=1 and
            // page_size=10. Channel display formatting (`format_channel_value`)
            // multiplies by 100 for percentage display. ARIA `aria-valuenow` reports
            // the raw 0.0–1.0 value, which is unconventional but valid.
            HsvChannel::Saturation | HsvChannel::Brightness => ColorChannelRange {
                min_value: 0.0,
                max_value: 1.0,
                step: 0.01,
                page_size: 0.1,
                gradient_stops: None,
            },
        }
    }

    fn channels() -> Vec<HsvChannel> {
        vec![
            HsvChannel::Hue,
            HsvChannel::Saturation,
            HsvChannel::Brightness,
        ]
    }

    fn get_color_space_axes(
        x_channel: Option<HsvChannel>,
        y_channel: Option<HsvChannel>,
    ) -> (HsvChannel, HsvChannel, HsvChannel) {
        let channels = [
            HsvChannel::Hue,
            HsvChannel::Saturation,
            HsvChannel::Brightness,
        ];

        let x = x_channel.unwrap_or(HsvChannel::Saturation);
        let y = y_channel.unwrap_or_else(|| {
            *channels
                .iter()
                .find(|&&ch| ch != x)
                .unwrap_or(&HsvChannel::Brightness)
        });
        let z = *channels
            .iter()
            .find(|&&ch| ch != x && ch != y)
            .unwrap_or(&HsvChannel::Hue);

        (x, y, z)
    }

    fn to_css_string(&self) -> String {
        let rgb = RGB8::from(*self);
        format!("rgb({}, {}, {})", rgb.r, rgb.g, rgb.b)
    }

    #[allow(clippy::cast_possible_truncation)]
    fn format_channel_value(&self, channel: HsvChannel) -> String {
        match channel {
            HsvChannel::Hue => format!("{}°", self.hue.round() as i32),
            HsvChannel::Saturation => format!("{:.0}%", self.saturation * 100.0),
            HsvChannel::Brightness => format!("{:.0}%", self.value * 100.0),
        }
    }

    fn get_channel_format_options(channel: HsvChannel) -> NumberFormatOptions {
        match channel {
            HsvChannel::Hue => NumberFormatOptions {
                style: NumberStyle::Unit,
                unit: Some("°".to_owned()),
                unit_display: UnitDisplay::Narrow,
                ..NumberFormatOptions::default()
            },
            HsvChannel::Saturation | HsvChannel::Brightness => NumberFormatOptions {
                style: NumberStyle::Percent,
                ..NumberFormatOptions::default()
            },
        }
    }

    fn get_channel_name(channel: HsvChannel) -> &'static str {
        match channel {
            HsvChannel::Hue => "Hue",
            HsvChannel::Saturation => "Saturation",
            HsvChannel::Brightness => "Brightness",
        }
    }

    fn get_display_color(&self, channel: HsvChannel) -> Self {
        match channel {
            HsvChannel::Hue => Self {
                hue: self.hue,
                saturation: 1.0,
                value: 1.0,
            },
            HsvChannel::Saturation | HsvChannel::Brightness => *self,
        }
    }

    fn get_area_gradient(
        &self,
        x_channel: Self::Channel,
        y_channel: Self::Channel,
        direction: WritingDirection,
    ) -> AreaGradient {
        // The x axis runs towards the end of the line (react-aria: `to left` in right-to-left).
        let end = match direction {
            WritingDirection::Ltr => "right",
            WritingDirection::Rtl => "left",
        };
        let (_, _, z_channel) = Self::get_color_space_axes(Some(x_channel), Some(y_channel));
        let z_value = self.get_channel_value(z_channel);

        // Base: vivid HSV with only the z-channel set from current color.
        // Matches react-aria: `parseColor('hsb(0, 100%, 100%)').withChannelValue(zChannel, zValue)`
        let base = Self {
            hue: 0.0,
            saturation: 1.0,
            value: 1.0,
        }
        .with_channel_value(z_channel, z_value);

        let channel_gradient = |ch: HsvChannel| -> String {
            match ch {
                HsvChannel::Hue => [0.0, 60.0, 120.0, 180.0, 240.0, 300.0, 360.0]
                    .iter()
                    .map(|&h| base.with_channel_value(HsvChannel::Hue, h).to_css_string())
                    .collect::<Vec<_>>()
                    .join(", "),
                HsvChannel::Saturation => {
                    format!(
                        "{}, transparent",
                        base.with_channel_value(HsvChannel::Saturation, 0.0)
                            .to_css_string()
                    )
                }
                HsvChannel::Brightness => "black, transparent".to_owned(),
            }
        };

        let x_gradient = format!("linear-gradient(to {end}, {})", channel_gradient(x_channel));
        let y_gradient = format!("linear-gradient(to top, {})", channel_gradient(y_channel));

        // Reversed order: y on top of x (y gradient renders above x gradient).
        let mut layers = vec![y_gradient, x_gradient];

        // When z is hue, the vivid base color forms the bottom layer.
        if z_channel == HsvChannel::Hue {
            layers.push(base.to_css_string());
        }

        AreaGradient {
            background: layers.join(", "),
            blend_mode: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RGB8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB8 {
    pub const fn new() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }

    pub fn into_hsv(self) -> HSV {
        HSV::from(self)
    }

    /// Parses a hex color string (with or without leading `#`): six digits (`"FF00AA"`) or the
    /// three-digit shorthand (`"#f0a"`), as CSS.
    #[must_use]
    pub fn from_hex(s: &str) -> Option<Self> {
        let s = s.strip_prefix('#').unwrap_or(s);
        if !s.chars().all(|c| c.is_ascii_hexdigit()) {
            return None;
        }
        let channel = |digits: &str| u8::from_str_radix(digits, 16).ok();
        match s.len() {
            6 => Some(Self {
                r: channel(&s[0..2])?,
                g: channel(&s[2..4])?,
                b: channel(&s[4..6])?,
            }),
            3 => {
                let doubled = |i: usize| channel(&s[i..=i].repeat(2));
                Some(Self {
                    r: doubled(0)?,
                    g: doubled(1)?,
                    b: doubled(2)?,
                })
            }
            _ => None,
        }
    }

    /// Converts the color to a 24-bit integer (0x000000–0xFFFFFF).
    #[must_use]
    pub const fn to_hex_int(self) -> u32 {
        (self.r as u32) << 16 | (self.g as u32) << 8 | self.b as u32
    }

    /// Creates an `RGB8` from a 24-bit integer.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn from_hex_int(val: u32) -> Self {
        Self {
            r: ((val >> 16) & 0xFF) as u8,
            g: ((val >> 8) & 0xFF) as u8,
            b: (val & 0xFF) as u8,
        }
    }
}

impl Default for RGB8 {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::LowerHex for RGB8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl fmt::UpperHex for RGB8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

impl fmt::Display for RGB8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{self:X}")
    }
}

impl From<(u8, u8, u8)> for RGB8 {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self { r, g, b }
    }
}

impl ColorValue for RGB8 {
    fn to_rgb8(&self) -> RGB8 {
        *self
    }

    type Channel = RgbChannel;

    fn get_channel_value(&self, channel: RgbChannel) -> f64 {
        match channel {
            RgbChannel::Red => f64::from(self.r),
            RgbChannel::Green => f64::from(self.g),
            RgbChannel::Blue => f64::from(self.b),
        }
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn with_channel_value(&self, channel: RgbChannel, value: f64) -> Self {
        let v = value.round().clamp(0.0, 255.0) as u8;
        match channel {
            RgbChannel::Red => Self {
                r: v,
                g: self.g,
                b: self.b,
            },
            RgbChannel::Green => Self {
                r: self.r,
                g: v,
                b: self.b,
            },
            RgbChannel::Blue => Self {
                r: self.r,
                g: self.g,
                b: v,
            },
        }
    }

    fn get_channel_range(_channel: RgbChannel) -> ColorChannelRange {
        ColorChannelRange {
            min_value: 0.0,
            max_value: 255.0,
            step: 1.0,
            page_size: 17.0,
            gradient_stops: None,
        }
    }

    fn channels() -> Vec<RgbChannel> {
        vec![RgbChannel::Red, RgbChannel::Green, RgbChannel::Blue]
    }

    fn get_color_space_axes(
        x_channel: Option<RgbChannel>,
        y_channel: Option<RgbChannel>,
    ) -> (RgbChannel, RgbChannel, RgbChannel) {
        let channels = [RgbChannel::Red, RgbChannel::Green, RgbChannel::Blue];

        let x = x_channel.unwrap_or(RgbChannel::Red);
        let y = y_channel.unwrap_or_else(|| {
            *channels
                .iter()
                .find(|&&ch| ch != x)
                .unwrap_or(&RgbChannel::Green)
        });
        let z = *channels
            .iter()
            .find(|&&ch| ch != x && ch != y)
            .unwrap_or(&RgbChannel::Blue);

        (x, y, z)
    }

    fn to_css_string(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }

    fn format_channel_value(&self, channel: RgbChannel) -> String {
        match channel {
            RgbChannel::Red => self.r.to_string(),
            RgbChannel::Green => self.g.to_string(),
            RgbChannel::Blue => self.b.to_string(),
        }
    }

    fn get_channel_name(channel: RgbChannel) -> &'static str {
        match channel {
            RgbChannel::Red => "Red",
            RgbChannel::Green => "Green",
            RgbChannel::Blue => "Blue",
        }
    }

    fn get_display_color(&self, _channel: RgbChannel) -> Self {
        *self
    }

    fn get_area_gradient(
        &self,
        x_channel: Self::Channel,
        y_channel: Self::Channel,
        direction: WritingDirection,
    ) -> AreaGradient {
        // The x axis runs towards the end of the line (react-aria: `to left` in right-to-left).
        let end = match direction {
            WritingDirection::Ltr => "right",
            WritingDirection::Rtl => "left",
        };
        let (_, _, z_channel) = Self::get_color_space_axes(Some(x_channel), Some(y_channel));
        let z_val = self.get_channel_value(z_channel);

        // The screen blend mode combines the layers channel by channel (1 - (1 - a) * (1 - b)):
        // one layer per channel, the others 0 (react-aria).
        let black = Self { r: 0, g: 0, b: 0 };
        let z_only = black.with_channel_value(z_channel, z_val);
        let x_max = black.with_channel_value(x_channel, 255.0);
        let y_max = black.with_channel_value(y_channel, 255.0);

        AreaGradient {
            background: format!(
                "linear-gradient(to {end}, {} 0%, {} 100%), \
                 linear-gradient(to top, {} 0%, {} 100%), \
                 {}",
                black.to_css_string(),
                x_max.to_css_string(),
                black.to_css_string(),
                y_max.to_css_string(),
                z_only.to_css_string(),
            ),
            blend_mode: Some(BlendMode::Screen),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HSL {
    pub hue: f64,
    pub saturation: f64,
    pub lightness: f64,
}

impl HSL {
    /// Creates a new HSL color with hue=0 (red), full saturation, and mid lightness.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            hue: 0.0,
            saturation: 1.0,
            lightness: 0.5,
        }
    }

    #[must_use]
    pub const fn from_hue_fully_saturated(hue: f64) -> Self {
        Self {
            hue,
            saturation: 1.0,
            lightness: 0.5,
        }
    }

    #[must_use]
    pub const fn with_hue(self, hue: f64) -> Self {
        Self {
            hue,
            saturation: self.saturation,
            lightness: self.lightness,
        }
    }

    #[must_use]
    pub const fn with_saturation(self, saturation: f64) -> Self {
        Self {
            hue: self.hue,
            saturation,
            lightness: self.lightness,
        }
    }

    #[must_use]
    pub const fn with_lightness(self, lightness: f64) -> Self {
        Self {
            hue: self.hue,
            saturation: self.saturation,
            lightness,
        }
    }
}

impl Default for HSL {
    fn default() -> Self {
        Self::new()
    }
}

impl ColorValue for HSL {
    fn hue_channel() -> Option<HslChannel> {
        Some(HslChannel::Hue)
    }

    fn to_rgb8(&self) -> RGB8 {
        RGB8::from(*self)
    }

    type Channel = HslChannel;

    fn get_channel_value(&self, channel: HslChannel) -> f64 {
        match channel {
            HslChannel::Hue => self.hue,
            HslChannel::Saturation => self.saturation,
            HslChannel::Lightness => self.lightness,
        }
    }

    fn with_channel_value(&self, channel: HslChannel, value: f64) -> Self {
        match channel {
            HslChannel::Hue => self.with_hue(value),
            HslChannel::Saturation => self.with_saturation(value),
            HslChannel::Lightness => self.with_lightness(value),
        }
    }

    fn get_channel_range(channel: HslChannel) -> ColorChannelRange {
        match channel {
            HslChannel::Hue => ColorChannelRange {
                min_value: 0.0,
                max_value: 360.0,
                step: 1.0,
                page_size: 15.0,
                gradient_stops: Some(&[0.0, 60.0, 120.0, 180.0, 240.0, 300.0, 360.0]),
            },
            // REACT-ARIA DEVIATION: Normalized 0.0–1.0 range (see HSV comment).
            HslChannel::Saturation => ColorChannelRange {
                min_value: 0.0,
                max_value: 1.0,
                step: 0.01,
                page_size: 0.1,
                gradient_stops: None,
            },
            // REACT-ARIA DEVIATION: Normalized 0.0–1.0 range (see HSV comment).
            HslChannel::Lightness => ColorChannelRange {
                min_value: 0.0,
                max_value: 1.0,
                step: 0.01,
                page_size: 0.1,
                // 3-stop gradient: black (0) → pure color (0.5) → white (1.0).
                gradient_stops: Some(&[0.0, 0.5, 1.0]),
            },
        }
    }

    fn channels() -> Vec<HslChannel> {
        vec![
            HslChannel::Hue,
            HslChannel::Saturation,
            HslChannel::Lightness,
        ]
    }

    fn get_color_space_axes(
        x_channel: Option<HslChannel>,
        y_channel: Option<HslChannel>,
    ) -> (HslChannel, HslChannel, HslChannel) {
        let channels = [
            HslChannel::Hue,
            HslChannel::Saturation,
            HslChannel::Lightness,
        ];

        let x = x_channel.unwrap_or(HslChannel::Saturation);
        let y = y_channel.unwrap_or_else(|| {
            *channels
                .iter()
                .find(|&&ch| ch != x)
                .unwrap_or(&HslChannel::Lightness)
        });
        let z = *channels
            .iter()
            .find(|&&ch| ch != x && ch != y)
            .unwrap_or(&HslChannel::Hue);

        (x, y, z)
    }

    fn to_css_string(&self) -> String {
        let rgb = RGB8::from(*self);
        format!("rgb({}, {}, {})", rgb.r, rgb.g, rgb.b)
    }

    #[allow(clippy::cast_possible_truncation)]
    fn format_channel_value(&self, channel: HslChannel) -> String {
        match channel {
            HslChannel::Hue => format!("{}°", self.hue.round() as i32),
            HslChannel::Saturation => format!("{:.0}%", self.saturation * 100.0),
            HslChannel::Lightness => format!("{:.0}%", self.lightness * 100.0),
        }
    }

    fn get_channel_format_options(channel: HslChannel) -> NumberFormatOptions {
        match channel {
            HslChannel::Hue => NumberFormatOptions {
                style: NumberStyle::Unit,
                unit: Some("°".to_owned()),
                unit_display: UnitDisplay::Narrow,
                ..NumberFormatOptions::default()
            },
            HslChannel::Saturation | HslChannel::Lightness => NumberFormatOptions {
                style: NumberStyle::Percent,
                ..NumberFormatOptions::default()
            },
        }
    }

    fn get_channel_name(channel: HslChannel) -> &'static str {
        match channel {
            HslChannel::Hue => "Hue",
            HslChannel::Saturation => "Saturation",
            HslChannel::Lightness => "Lightness",
        }
    }

    fn get_display_color(&self, channel: HslChannel) -> Self {
        match channel {
            HslChannel::Hue => Self {
                hue: self.hue,
                saturation: 1.0,
                lightness: 0.5,
            },
            HslChannel::Saturation | HslChannel::Lightness => *self,
        }
    }

    fn get_area_gradient(
        &self,
        x_channel: Self::Channel,
        y_channel: Self::Channel,
        direction: WritingDirection,
    ) -> AreaGradient {
        // The x axis runs towards the end of the line (react-aria: `to left` in right-to-left).
        let end = match direction {
            WritingDirection::Ltr => "right",
            WritingDirection::Rtl => "left",
        };
        let (_, _, z_channel) = Self::get_color_space_axes(Some(x_channel), Some(y_channel));
        let z_value = self.get_channel_value(z_channel);

        // Base: vivid HSL with only the z-channel set from current color.
        // Matches react-aria: `parseColor('hsl(0, 100%, 50%)').withChannelValue(zChannel, zValue)`
        let base = Self {
            hue: 0.0,
            saturation: 1.0,
            lightness: 0.5,
        }
        .with_channel_value(z_channel, z_value);

        let channel_gradient = |ch: HslChannel| -> String {
            match ch {
                HslChannel::Hue => [0.0, 60.0, 120.0, 180.0, 240.0, 300.0, 360.0]
                    .iter()
                    .map(|&h| base.with_channel_value(HslChannel::Hue, h).to_css_string())
                    .collect::<Vec<_>>()
                    .join(", "),
                HslChannel::Saturation => {
                    format!(
                        "{}, transparent",
                        base.with_channel_value(HslChannel::Saturation, 0.0)
                            .to_css_string()
                    )
                }
                HslChannel::Lightness => "black, transparent, white".to_owned(),
            }
        };

        let x_gradient = format!("linear-gradient(to {end}, {})", channel_gradient(x_channel));
        let y_gradient = format!("linear-gradient(to top, {})", channel_gradient(y_channel));

        // Reversed order: y on top of x (y gradient renders above x gradient).
        let mut layers = vec![y_gradient, x_gradient];

        // When z is hue, the vivid base color forms the bottom layer.
        if z_channel == HslChannel::Hue {
            layers.push(base.to_css_string());
        }

        AreaGradient {
            background: layers.join(", "),
            blend_mode: None,
        }
    }
}

impl From<HSV> for RGB8 {
    // Expectations: 0 ≤ H < 360, 0 ≤ S ≤ 1 and 0 ≤ V ≤ 1:
    #[allow(
        clippy::many_single_char_names,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    fn from(hsv: HSV) -> Self {
        let (h, s, v) = (hsv.hue, hsv.saturation, hsv.value);

        let c = v * s;
        let x = c * (1.0 - f64::abs(((h / 60.0) % 2.0) - 1.0));
        let m = v - c;

        let (r, g, b) = if (0.0..60.0).contains(&h) {
            (c, x, 0.0)
        } else if (60.0..120.0).contains(&h) {
            (x, c, 0.0)
        } else if (120.0..180.0).contains(&h) {
            (0.0, c, x)
        } else if (180.0..240.0).contains(&h) {
            (0.0, x, c)
        } else if (240.0..300.0).contains(&h) {
            (x, 0.0, c)
        } else if (300.0..360.0).contains(&h) {
            (c, 0.0, x)
        } else {
            (c, x, 0.0) // error! simply using the 0.0..60.0 branch again.
        };

        let (r, g, b) = (
            ((r + m) * 255.0).round() as u8,
            ((g + m) * 255.0).round() as u8,
            ((b + m) * 255.0).round() as u8,
        );

        Self { r, g, b }
    }
}

impl From<RGB8> for HSV {
    #[allow(clippy::many_single_char_names, clippy::float_cmp)]
    fn from(rgb: RGB8) -> Self {
        let RGB8 { r, g, b } = rgb;

        let (r, g, b) = (
            f64::from(r) / 255.0,
            f64::from(g) / 255.0,
            f64::from(b) / 255.0,
        );

        let c_max = f64::max(r, f64::max(g, b));
        let c_min = f64::min(r, f64::min(g, b));
        let delta = c_max - c_min;

        let hue = if delta == 0.0 {
            0.0
        } else if c_max == r {
            60.0 * ((g - b) / delta).rem_euclid(6.0)
        } else if c_max == g {
            60.0 * (((b - r) / delta) + 2.0)
        } else if c_max == b {
            60.0 * (((r - g) / delta) + 4.0)
        } else {
            unreachable!()
        };

        let saturation = if c_max == 0.0 { 0.0 } else { delta / c_max };

        let value = c_max;

        Self {
            hue,
            saturation,
            value,
        }
    }
}

impl From<HSL> for RGB8 {
    // Standard HSL → RGB conversion.
    // Expects: 0 ≤ H < 360, 0 ≤ S ≤ 1, 0 ≤ L ≤ 1.
    #[allow(
        clippy::many_single_char_names,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    fn from(hsl: HSL) -> Self {
        let (h, s, l) = (hsl.hue, hsl.saturation, hsl.lightness);

        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;

        let (r, g, b) = if (0.0..60.0).contains(&h) {
            (c, x, 0.0)
        } else if (60.0..120.0).contains(&h) {
            (x, c, 0.0)
        } else if (120.0..180.0).contains(&h) {
            (0.0, c, x)
        } else if (180.0..240.0).contains(&h) {
            (0.0, x, c)
        } else if (240.0..300.0).contains(&h) {
            (x, 0.0, c)
        } else if (300.0..360.0).contains(&h) {
            (c, 0.0, x)
        } else {
            (c, x, 0.0)
        };

        let (r, g, b) = (
            ((r + m) * 255.0).round() as u8,
            ((g + m) * 255.0).round() as u8,
            ((b + m) * 255.0).round() as u8,
        );

        Self { r, g, b }
    }
}

impl From<RGB8> for HSL {
    #[allow(clippy::many_single_char_names, clippy::float_cmp)]
    fn from(rgb: RGB8) -> Self {
        let RGB8 { r, g, b } = rgb;

        let (r, g, b) = (
            f64::from(r) / 255.0,
            f64::from(g) / 255.0,
            f64::from(b) / 255.0,
        );

        let c_max = f64::max(r, f64::max(g, b));
        let c_min = f64::min(r, f64::min(g, b));
        let delta = c_max - c_min;

        let hue = if delta == 0.0 {
            0.0
        } else if c_max == r {
            60.0 * ((g - b) / delta).rem_euclid(6.0)
        } else if c_max == g {
            60.0 * (((b - r) / delta) + 2.0)
        } else if c_max == b {
            60.0 * (((r - g) / delta) + 4.0)
        } else {
            unreachable!()
        };

        let lightness = f64::midpoint(c_max, c_min);

        let saturation = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * lightness - 1.0).abs())
        };

        Self {
            hue,
            saturation,
            lightness,
        }
    }
}

impl From<HSL> for HSV {
    fn from(hsl: HSL) -> Self {
        let HSL {
            hue,
            saturation: s,
            lightness: l,
        } = hsl;
        let v = l + s * f64::min(l, 1.0 - l);
        let sv = if v == 0.0 { 0.0 } else { 2.0 * (1.0 - l / v) };
        Self {
            hue,
            saturation: sv,
            value: v,
        }
    }
}

impl From<HSV> for HSL {
    fn from(hsv: HSV) -> Self {
        let HSV {
            hue,
            saturation: s,
            value: v,
        } = hsv;
        let l = v * (1.0 - s / 2.0);
        #[allow(clippy::float_cmp)]
        let sl = if l == 0.0 || l == 1.0 {
            0.0
        } else {
            (v - l) / f64::min(l, 1.0 - l)
        };
        Self {
            hue,
            saturation: sl,
            lightness: l,
        }
    }
}

/// Color names (react-aria's `getColorName`/`getHueName`, in English).
mod naming {
    use super::RGB8;

    /// Lightness between orange and brown.
    const ORANGE_LIGHTNESS_THRESHOLD: f64 = 0.68;
    /// Lightness between pure yellow and "yellow green".
    const YELLOW_GREEN_LIGHTNESS_THRESHOLD: f64 = 0.85;
    /// The maximum lightness considered dark.
    const MAX_DARK_LIGHTNESS: f64 = 0.55;
    /// Chroma between gray and a color.
    const GRAY_THRESHOLD: f64 = 0.001;
    /// Where the hues start, in OKLCH degrees.
    const OKLCH_HUES: [(f64, &str); 10] = [
        (0.0, "pink"),
        (15.0, "red"),
        (48.0, "orange"),
        (94.0, "yellow"),
        (135.0, "green"),
        (175.0, "cyan"),
        (264.0, "blue"),
        (284.0, "purple"),
        (320.0, "magenta"),
        (349.0, "pink"),
    ];

    /// The color's name, e.g. "very dark grayish blue".
    pub(super) fn color_name(color: RGB8) -> String {
        let (l, c, h) = to_oklch(color);
        if l > 0.999 {
            return "white".to_owned();
        }
        if l < 0.001 {
            return "black".to_owned();
        }
        let (hue, l) = oklch_hue(l, c, h);
        let chroma = if (GRAY_THRESHOLD..=0.1).contains(&c) {
            if l >= 0.7 { "pale" } else { "grayish" }
        } else if c >= 0.15 {
            "vibrant"
        } else {
            ""
        };
        let lightness = if l < 0.3 {
            "very dark"
        } else if l < MAX_DARK_LIGHTNESS {
            "dark"
        } else if l < 0.7 {
            ""
        } else if l < 0.85 {
            "light"
        } else {
            "very light"
        };
        [lightness, chroma, &hue]
            .into_iter()
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// The name of the color's hue, e.g. "red orange".
    pub(super) fn hue_name(color: RGB8) -> String {
        let (l, c, h) = to_oklch(color);
        oklch_hue(l, c, h).0
    }

    /// The hue's name, and the lightness adjusted for it.
    fn oklch_hue(l: f64, c: f64, h: f64) -> (String, f64) {
        if c < GRAY_THRESHOLD {
            return ("gray".to_owned(), l);
        }
        let mut l = l;
        for (i, &(hue, name)) in OKLCH_HUES.iter().enumerate() {
            let (next_hue, next_name) = OKLCH_HUES.get(i + 1).copied().unwrap_or((360.0, "pink"));
            if h >= hue && h < next_hue {
                let mut name = name.to_owned();
                // Orange splits into brown and orange by lightness.
                if name == "orange" {
                    if l < ORANGE_LIGHTNESS_THRESHOLD {
                        "brown".clone_into(&mut name);
                    } else {
                        l = l - ORANGE_LIGHTNESS_THRESHOLD + MAX_DARK_LIGHTNESS;
                    }
                }
                // At least halfway to the next hue: both names.
                if h > hue + (next_hue - hue) / 2.0 && name != next_name {
                    name = format!("{name} {next_name}");
                } else if name == "yellow" && l < YELLOW_GREEN_LIGHTNESS_THRESHOLD {
                    // Yellow shifts toward green at lower lightnesses.
                    "yellow green".clone_into(&mut name);
                }
                return (name, l);
            }
        }
        ("pink".to_owned(), l)
    }

    /// The color in OKLCH: lightness, chroma, hue in degrees (CSS Color 4's conversion code,
    /// with its constants).
    #[allow(clippy::many_single_char_names, clippy::excessive_precision)]
    fn to_oklch(color: RGB8) -> (f64, f64, f64) {
        let linear = |channel: u8| {
            let v = f64::from(channel) / 255.0;
            if v <= 0.04045 {
                v / 12.92
            } else {
                ((v + 0.055) / 1.055).powf(2.4)
            }
        };
        let (r, g, b) = (linear(color.r), linear(color.g), linear(color.b));
        let xyz = multiply(
            [
                506_752.0 / 1_228_815.0,
                87_881.0 / 245_763.0,
                12_673.0 / 70_218.0,
                87_098.0 / 409_605.0,
                175_762.0 / 245_763.0,
                12_673.0 / 175_545.0,
                7_918.0 / 409_605.0,
                87_881.0 / 737_289.0,
                1_001_167.0 / 1_053_270.0,
            ],
            (r, g, b),
        );
        let lms = multiply(
            [
                0.819_022_437_996_703,
                0.361_906_260_052_890_4,
                -0.128_873_781_520_987_9,
                0.032_983_653_932_388_5,
                0.929_286_861_586_343_4,
                0.036_144_666_350_642_4,
                0.048_177_189_359_624_2,
                0.264_239_531_752_730_8,
                0.633_547_828_469_430_9,
            ],
            xyz,
        );
        let (l, a, b) = multiply(
            [
                0.210_454_268_309_314,
                0.793_617_774_702_305_4,
                -0.004_072_043_011_619_3,
                1.977_998_532_431_168_4,
                -2.428_592_242_048_579_9,
                0.450_593_709_617_411,
                0.025_904_042_465_547_8,
                0.782_771_712_457_529_6,
                -0.808_675_754_923_077_4,
            ],
            (lms.0.cbrt(), lms.1.cbrt(), lms.2.cbrt()),
        );
        let hue = b.atan2(a).to_degrees();
        (l, a.hypot(b), if hue >= 0.0 { hue } else { hue + 360.0 })
    }

    fn multiply(m: [f64; 9], (x, y, z): (f64, f64, f64)) -> (f64, f64, f64) {
        (
            m[0] * x + m[1] * y + m[2] * z,
            m[3] * x + m[4] * y + m[5] * z,
            m[6] * x + m[7] * y + m[8] * z,
        )
    }

    #[cfg(test)]
    mod tests {
        use assertr::prelude::*;

        use super::*;

        /// From react-stately's `Color.test.tsx` ("should return localized color name").
        #[test]
        fn names_colors_as_react_aria() {
            for (hex, name) in [
                ("#FFFFFF", "white"),
                ("#000000", "black"),
                ("#FFFF00", "very light vibrant yellow"),
                ("#800080", "dark vibrant magenta"),
                ("#FF0000", "vibrant red"),
                ("#800000", "dark vibrant red"),
                ("#FF00FF", "light vibrant magenta"),
                ("#008000", "dark vibrant green"),
                ("#808000", "yellow green"),
                ("#000080", "very dark vibrant blue"),
                ("#0000FF", "dark vibrant blue"),
                ("#008080", "dark grayish cyan"),
                ("#faebd7", "light pale orange yellow"),
                ("#7fffd4", "very light green cyan"),
                ("#d2691e", "vibrant brown"),
                ("#ff7f50", "light vibrant red orange"),
                ("#6495ed", "cyan blue"),
                ("#b8860b", "brown yellow"),
                ("#a9a9a9", "light gray"),
                ("#9932cc", "dark vibrant purple magenta"),
                ("#808080", "gray"),
            ] {
                let color = RGB8::from_hex(hex).expect("a hex color");
                assert_that!(color_name(color))
                    .with_detail_message(hex)
                    .is_equal_to(name.to_owned());
            }
            assert_that!(hue_name(RGB8::from_hex("#d2691e").expect("a hex color")))
                .is_equal_to("brown".to_owned());
        }
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    // --- Existing hex formatting tests ---

    #[test]
    fn rgb8_to_lower_hex() {
        let rgb = RGB8 {
            r: 186,
            g: 23,
            b: 241,
        };
        assert_that!(format!("{rgb:x}").as_str()).is_equal_to("ba17f1");
    }

    #[test]
    fn rgb8_to_upper_hex() {
        let rgb = RGB8 {
            r: 186,
            g: 23,
            b: 241,
        };
        assert_that!(format!("{rgb:X}").as_str()).is_equal_to("BA17F1");
    }

    #[test]
    fn rgb8_display() {
        let rgb = RGB8 {
            r: 186,
            g: 23,
            b: 241,
        };
        assert_that!(&rgb.to_string()).is_equal_to("#BA17F1");
    }

    // --- Hex parsing tests ---

    #[test]
    fn rgb8_from_hex_with_hash() {
        assert_that!(RGB8::from_hex("#FF8000")).is_equal_to(Some(RGB8 {
            r: 255,
            g: 128,
            b: 0,
        }));
    }

    #[test]
    fn rgb8_from_hex_without_hash() {
        assert_that!(RGB8::from_hex("ff8000")).is_equal_to(Some(RGB8 {
            r: 255,
            g: 128,
            b: 0,
        }));
    }

    #[test]
    fn rgb8_from_hex_invalid_length() {
        assert_that!(RGB8::from_hex("FFFF")).is_none();
        assert_that!(RGB8::from_hex("FFFFF")).is_none();
    }

    #[test]
    fn rgb8_from_hex_shorthand() {
        assert_that!(RGB8::from_hex("#f0a")).is_equal_to(Some(RGB8 {
            r: 0xFF,
            g: 0x00,
            b: 0xAA,
        }));
    }

    #[test]
    fn rgb8_from_hex_invalid_chars() {
        assert_that!(RGB8::from_hex("ZZZZZZ")).is_none();
    }

    #[test]
    fn rgb8_hex_int_roundtrip() {
        let rgb = RGB8 {
            r: 186,
            g: 23,
            b: 241,
        };
        let int = rgb.to_hex_int();
        assert_that!(RGB8::from_hex_int(int)).is_equal_to(rgb);
    }

    // --- HSV ColorValue trait tests ---

    #[test]
    fn hsv_get_channel_value() {
        let hsv = HSV {
            hue: 120.0,
            saturation: 0.5,
            value: 0.8,
        };
        assert_that!(hsv.get_channel_value(HsvChannel::Hue)).is_close_to(120.0, 0.001);
        assert_that!(hsv.get_channel_value(HsvChannel::Saturation)).is_close_to(0.5, 0.001);
        assert_that!(hsv.get_channel_value(HsvChannel::Brightness)).is_close_to(0.8, 0.001);
    }

    #[test]
    fn hsv_with_channel_value() {
        let hsv = HSV::new();
        let updated = hsv.with_channel_value(HsvChannel::Hue, 180.0);
        assert_that!(updated.hue).is_close_to(180.0, 0.001);
        assert_that!(updated.saturation).is_close_to(hsv.saturation, 0.001);
    }

    #[test]
    fn hsv_get_channel_range() {
        let range = HSV::get_channel_range(HsvChannel::Hue);
        assert_that!(range.min_value).is_close_to(0.0, 0.001);
        assert_that!(range.max_value).is_close_to(360.0, 0.001);
        assert_that!(range.step).is_close_to(1.0, 0.001);
    }

    #[test]
    fn hsv_channels() {
        let channels = HSV::channels();
        assert_that!(channels.len()).is_equal_to(3);
    }

    #[test]
    fn hsv_get_color_space_axes_defaults() {
        let (x, y, z) = HSV::get_color_space_axes(None, None);
        assert_that!(x).is_equal_to(HsvChannel::Saturation);
        assert_that!(y).is_equal_to(HsvChannel::Hue);
        assert_that!(z).is_equal_to(HsvChannel::Brightness);
    }

    #[test]
    fn hsv_get_color_space_axes_custom() {
        let (x, y, z) =
            HSV::get_color_space_axes(Some(HsvChannel::Saturation), Some(HsvChannel::Brightness));
        assert_that!(x).is_equal_to(HsvChannel::Saturation);
        assert_that!(y).is_equal_to(HsvChannel::Brightness);
        assert_that!(z).is_equal_to(HsvChannel::Hue);
    }

    #[test]
    fn hsv_to_css_string() {
        let hsv = HSV {
            hue: 0.0,
            saturation: 1.0,
            value: 1.0,
        };
        assert_that!(hsv.to_css_string()).is_equal_to("rgb(255, 0, 0)".to_owned());
    }

    #[test]
    fn hsv_format_channel_value() {
        let hsv = HSV {
            hue: 120.0,
            saturation: 0.5,
            value: 0.8,
        };
        assert_that!(hsv.format_channel_value(HsvChannel::Hue)).is_equal_to("120°".to_owned());
        assert_that!(hsv.format_channel_value(HsvChannel::Saturation))
            .is_equal_to("50%".to_owned());
        assert_that!(hsv.format_channel_value(HsvChannel::Brightness))
            .is_equal_to("80%".to_owned());
    }

    #[test]
    fn hsv_get_channel_name() {
        assert_that!(HSV::get_channel_name(HsvChannel::Hue)).is_equal_to("Hue");
        assert_that!(HSV::get_channel_name(HsvChannel::Saturation)).is_equal_to("Saturation");
        assert_that!(HSV::get_channel_name(HsvChannel::Brightness)).is_equal_to("Brightness");
    }

    // --- RGB8 ColorValue trait tests ---

    #[test]
    fn rgb8_get_channel_value() {
        let rgb = RGB8 {
            r: 128,
            g: 64,
            b: 32,
        };
        assert_that!(rgb.get_channel_value(RgbChannel::Red)).is_close_to(128.0, 0.001);
        assert_that!(rgb.get_channel_value(RgbChannel::Green)).is_close_to(64.0, 0.001);
        assert_that!(rgb.get_channel_value(RgbChannel::Blue)).is_close_to(32.0, 0.001);
    }

    #[test]
    fn rgb8_with_channel_value() {
        let rgb = RGB8 {
            r: 128,
            g: 64,
            b: 32,
        };
        let updated = rgb.with_channel_value(RgbChannel::Red, 200.0);
        assert_that!(updated).is_equal_to(RGB8 {
            r: 200,
            g: 64,
            b: 32,
        });
    }

    #[test]
    fn rgb8_with_channel_value_clamps() {
        let rgb = RGB8::new();
        let updated = rgb.with_channel_value(RgbChannel::Red, 300.0);
        assert_that!(updated.r).is_equal_to(255);
        let updated = rgb.with_channel_value(RgbChannel::Red, -10.0);
        assert_that!(updated.r).is_equal_to(0);
    }

    #[test]
    fn rgb8_to_css_string() {
        let rgb = RGB8 {
            r: 128,
            g: 64,
            b: 32,
        };
        assert_that!(rgb.to_css_string()).is_equal_to("rgb(128, 64, 32)".to_owned());
    }

    #[test]
    fn rgb8_get_channel_name() {
        assert_that!(RGB8::get_channel_name(RgbChannel::Red)).is_equal_to("Red");
        assert_that!(RGB8::get_channel_name(RgbChannel::Green)).is_equal_to("Green");
        assert_that!(RGB8::get_channel_name(RgbChannel::Blue)).is_equal_to("Blue");
    }

    // --- HSL ColorValue trait tests ---

    #[test]
    fn hsl_get_channel_value() {
        let hsl = HSL {
            hue: 120.0,
            saturation: 0.5,
            lightness: 0.6,
        };
        assert_that!(hsl.get_channel_value(HslChannel::Hue)).is_close_to(120.0, 0.001);
        assert_that!(hsl.get_channel_value(HslChannel::Saturation)).is_close_to(0.5, 0.001);
        assert_that!(hsl.get_channel_value(HslChannel::Lightness)).is_close_to(0.6, 0.001);
    }

    #[test]
    fn hsl_with_channel_value() {
        let hsl = HSL::new();
        let updated = hsl.with_channel_value(HslChannel::Hue, 240.0);
        assert_that!(updated.hue).is_close_to(240.0, 0.001);
        assert_that!(updated.saturation).is_close_to(hsl.saturation, 0.001);
    }

    #[test]
    fn hsl_get_channel_range() {
        let range = HSL::get_channel_range(HslChannel::Hue);
        assert_that!(range.min_value).is_close_to(0.0, 0.001);
        assert_that!(range.max_value).is_close_to(360.0, 0.001);
        assert_that!(range.step).is_close_to(1.0, 0.001);

        let light_range = HSL::get_channel_range(HslChannel::Lightness);
        assert_that!(light_range.gradient_stops.unwrap().len()).is_equal_to(3);
    }

    #[test]
    fn hsl_get_color_space_axes_defaults() {
        let (x, y, z) = HSL::get_color_space_axes(None, None);
        assert_that!(x).is_equal_to(HslChannel::Saturation);
        assert_that!(y).is_equal_to(HslChannel::Hue);
        assert_that!(z).is_equal_to(HslChannel::Lightness);
    }

    #[test]
    fn hsl_format_channel_value() {
        let hsl = HSL {
            hue: 120.0,
            saturation: 0.5,
            lightness: 0.75,
        };
        assert_that!(hsl.format_channel_value(HslChannel::Hue)).is_equal_to("120°".to_owned());
        assert_that!(hsl.format_channel_value(HslChannel::Saturation))
            .is_equal_to("50%".to_owned());
        assert_that!(hsl.format_channel_value(HslChannel::Lightness)).is_equal_to("75%".to_owned());
    }

    #[test]
    fn hsl_get_channel_name() {
        assert_that!(HSL::get_channel_name(HslChannel::Hue)).is_equal_to("Hue");
        assert_that!(HSL::get_channel_name(HslChannel::Saturation)).is_equal_to("Saturation");
        assert_that!(HSL::get_channel_name(HslChannel::Lightness)).is_equal_to("Lightness");
    }

    #[test]
    fn hsl_display_color_hue() {
        let hsl = HSL {
            hue: 120.0,
            saturation: 0.3,
            lightness: 0.2,
        };
        let display = hsl.get_display_color(HslChannel::Hue);
        assert_that!(display.hue).is_close_to(120.0, 0.001);
        assert_that!(display.saturation).is_close_to(1.0, 0.001);
        assert_that!(display.lightness).is_close_to(0.5, 0.001);
    }

    // --- HSL conversion tests ---

    #[test]
    fn hsl_to_rgb8_red() {
        let hsl = HSL {
            hue: 0.0,
            saturation: 1.0,
            lightness: 0.5,
        };
        let rgb = RGB8::from(hsl);
        assert_that!(rgb).is_equal_to(RGB8 { r: 255, g: 0, b: 0 });
    }

    #[test]
    fn hsl_to_rgb8_green() {
        let hsl = HSL {
            hue: 120.0,
            saturation: 1.0,
            lightness: 0.5,
        };
        let rgb = RGB8::from(hsl);
        assert_that!(rgb).is_equal_to(RGB8 { r: 0, g: 255, b: 0 });
    }

    #[test]
    fn hsl_to_rgb8_blue() {
        let hsl = HSL {
            hue: 240.0,
            saturation: 1.0,
            lightness: 0.5,
        };
        let rgb = RGB8::from(hsl);
        assert_that!(rgb).is_equal_to(RGB8 { r: 0, g: 0, b: 255 });
    }

    #[test]
    fn hsl_to_rgb8_white() {
        let hsl = HSL {
            hue: 0.0,
            saturation: 0.0,
            lightness: 1.0,
        };
        let rgb = RGB8::from(hsl);
        assert_that!(rgb).is_equal_to(RGB8 {
            r: 255,
            g: 255,
            b: 255,
        });
    }

    #[test]
    fn hsl_to_rgb8_black() {
        let hsl = HSL {
            hue: 0.0,
            saturation: 0.0,
            lightness: 0.0,
        };
        let rgb = RGB8::from(hsl);
        assert_that!(rgb).is_equal_to(RGB8 { r: 0, g: 0, b: 0 });
    }

    #[test]
    fn hsl_to_rgb8_gray() {
        let hsl = HSL {
            hue: 0.0,
            saturation: 0.0,
            lightness: 0.5,
        };
        let rgb = RGB8::from(hsl);
        assert_that!(rgb).is_equal_to(RGB8 {
            r: 128,
            g: 128,
            b: 128,
        });
    }

    #[test]
    fn rgb8_to_hsl_roundtrip() {
        let colors = [
            RGB8 { r: 255, g: 0, b: 0 },
            RGB8 { r: 0, g: 255, b: 0 },
            RGB8 { r: 0, g: 0, b: 255 },
            RGB8 {
                r: 128,
                g: 64,
                b: 32,
            },
        ];
        for original in colors {
            let hsl = HSL::from(original);
            let back = RGB8::from(hsl);
            // Allow ±1 rounding difference per channel.
            assert_that!(i16::from(back.r) - i16::from(original.r)).is_in_range(-1..=1);
            assert_that!(i16::from(back.g) - i16::from(original.g)).is_in_range(-1..=1);
            assert_that!(i16::from(back.b) - i16::from(original.b)).is_in_range(-1..=1);
        }
    }

    #[test]
    fn hsl_hsv_roundtrip() {
        let hsl = HSL {
            hue: 120.0,
            saturation: 0.8,
            lightness: 0.4,
        };
        let hsv = HSV::from(hsl);
        let back = HSL::from(hsv);
        assert_that!(back.hue).is_close_to(hsl.hue, 0.01);
        assert_that!(back.saturation).is_close_to(hsl.saturation, 0.01);
        assert_that!(back.lightness).is_close_to(hsl.lightness, 0.01);
    }

    #[test]
    fn hsv_hsl_roundtrip() {
        let hsv = HSV {
            hue: 240.0,
            saturation: 0.6,
            value: 0.9,
        };
        let hsl = HSL::from(hsv);
        let back = HSV::from(hsl);
        assert_that!(back.hue).is_close_to(hsv.hue, 0.01);
        assert_that!(back.saturation).is_close_to(hsv.saturation, 0.01);
        assert_that!(back.value).is_close_to(hsv.value, 0.01);
    }

    #[test]
    fn hsl_achromatic_saturation_zero() {
        // Achromatic colors (S=0) should have 0 saturation in HSV too.
        let hsl = HSL {
            hue: 0.0,
            saturation: 0.0,
            lightness: 0.5,
        };
        let hsv = HSV::from(hsl);
        assert_that!(hsv.saturation).is_close_to(0.0, 0.001);
    }

    #[test]
    fn parses_colors_as_react_aria() {
        let parse = |text: &str| text.parse::<Color>().ok();
        assert_that!(parse("#f0a")).is_equal_to(Some(Color::from(RGB8 {
            r: 255,
            g: 0,
            b: 170,
        })));
        assert_that!(parse("rgb(10, 300, -5)")).is_equal_to(Some(Color::from(RGB8 {
            r: 10,
            g: 255,
            b: 0,
        })));
        assert_that!(parse("hsb(-30, 50%, 100%)")).is_equal_to(Some(Color::from(HSV {
            hue: 330.0,
            saturation: 0.5,
            value: 1.0,
        })));
        assert_that!(parse("hsl(360, 100%, 50%)")).is_equal_to(Some(Color::from(HSL {
            hue: 360.0,
            saturation: 1.0,
            lightness: 0.5,
        })));
        assert_that!(parse("rgb(1, , 3)")).is_none();
        assert_that!(parse("hsl(0, 50, 50%)")).is_none();
        assert_that!(parse("red")).is_none();
    }

    #[test]
    fn keeps_the_space_it_was_set_in() {
        let gray = Color::from(HSV {
            hue: 120.0,
            saturation: 0.0,
            value: 0.5,
        });
        assert_that!(gray.to::<HSV>().hue).is_equal_to(120.0);
        assert_that!(gray.to::<HSL>().hue).is_equal_to(120.0);
        assert_that!(gray.to::<RGB8>()).is_equal_to(RGB8 {
            r: 128,
            g: 128,
            b: 128,
        });
    }

    #[test]
    fn hue_of_magenta_is_positive() {
        let magenta = RGB8 {
            r: 255,
            g: 0,
            b: 255,
        };
        assert_that!(HSV::from(magenta).hue).is_equal_to(300.0);
        assert_that!(HSL::from(magenta).hue).is_equal_to(300.0);
    }

    /// From react-stately's `Color.test.tsx` (parsing and formatting with alpha).
    #[test]
    fn parses_and_formats_alpha_as_react_aria() {
        let parse = |text: &str| text.parse::<Color>().expect("a color");
        let hexa = parse("#abcdef99");
        assert_that!(hexa.to::<RGB8>()).is_equal_to(RGB8 {
            r: 171,
            g: 205,
            b: 239,
        });
        assert_that!(hexa.to_css_string()).is_equal_to("rgba(171, 205, 239, 0.6)".to_owned());
        assert_that!(parse("#abc9").to_css_string())
            .is_equal_to("rgba(170, 187, 204, 0.6)".to_owned());
        assert_that!(parse("rgba(128, 128, 0, 0.5)").to_css_string())
            .is_equal_to("rgba(128, 128, 0, 0.5)".to_owned());
        // "normalizes rgba value by clamping"
        assert_that!(parse("rgba(300, -10, 0, 4)").to_css_string())
            .is_equal_to("rgb(255, 0, 0)".to_owned());
        assert_that!("rgba(0, 0, 0, abc)".parse::<Color>()).is_err();
        assert_that!("#aa\u{e9}".parse::<Color>()).is_err();
        assert_that!("#abcdef+f".parse::<Color>()).is_err();
        assert_that!(parse("hsla(0, 100%, 50%, 0.25)").alpha).is_equal_to(0.25);
        assert_that!(parse("hsba(0, 100%, 100%, 0.2)").color_name())
            .is_equal_to("vibrant red, 80% transparent".to_owned());
    }

    #[test]
    fn alpha_survives_conversions_and_opaque_types_drop_it() {
        let color = Color::from(HSV::new()).with_alpha(0.4);
        let hsl: Alpha<HSL> = color.to();
        assert_that!(hsl.alpha).is_equal_to(0.4);
        assert_that!(Color::from(hsl).alpha).is_equal_to(0.4);
        assert_that!(Color::from(color.to::<RGB8>()).alpha).is_equal_to(1.0);
        let channel = AlphaChannel::<HsvChannel>::Alpha;
        let half = Alpha::new(HSV::new()).with_channel_value(channel, 0.5);
        assert_that!(half.format_channel_value(channel)).is_equal_to("50%".to_owned());
        assert_that!(Alpha::<HSV>::channels().len()).is_equal_to(3);
    }
}

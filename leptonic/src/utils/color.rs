use std::fmt;
use std::hash::Hash;

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
// `get_hue_name()` provides 12 basic hue names in 30-degree segments.
// React-aria uses OKLCH conversion for perceptually-uniform descriptive names
// (e.g., "dark vibrant blue"). This can be added when OKLCH support is
// implemented.
//
// ## Channel Name Localization
// `get_channel_name()` returns hardcoded English strings. React-aria uses
// localized strings via `LocalizedStringDictionary`. Can be added using the
// ICU4X infrastructure already in place for other modules.
//
// ## Alpha Channel
// Not yet implemented. RGBA8 is declared as a stub. React-aria has full alpha
// support across all color spaces.

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

/// A color value with typed, color-space-specific channels.
///
/// Each implementor defines its own `Channel` enum so that hooks can be generic
/// over the color type while remaining type-safe (e.g., you cannot request the
/// `Hue` channel from an `RGB8` value).
pub trait ColorValue: Clone + Copy + PartialEq + fmt::Debug + Send + Sync + 'static {
    /// The channel enum for this color space.
    type Channel: fmt::Debug + Clone + Copy + PartialEq + Eq + Hash + Send + Sync + 'static;

    /// Returns the numeric value of the given channel.
    fn get_channel_value(&self, channel: Self::Channel) -> f64;

    /// Returns a new color with the given channel set to `value`.
    #[must_use]
    fn with_channel_value(&self, channel: Self::Channel, value: f64) -> Self;

    /// Returns the valid range, step, and page size for a channel.
    fn get_channel_range(channel: Self::Channel) -> ColorChannelRange;

    /// Returns all channels of this color space (excluding alpha).
    fn channels() -> &'static [Self::Channel];

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

    /// Formats the value of a channel for display (e.g. `"128"`, `"0.50"`).
    fn format_channel_value(&self, channel: Self::Channel) -> String;

    /// Returns the human-readable name of a channel (e.g. `"Hue"`, `"Red"`).
    fn get_channel_name(channel: Self::Channel) -> &'static str;

    /// Returns the color to display for gradient rendering of the given channel.
    ///
    /// For hue channels, this returns a fully saturated/bright version so the
    /// gradient shows vivid hues regardless of the current saturation/brightness.
    /// For other channels, returns the color as-is (alpha stripping will be added
    /// when alpha channel support is implemented).
    #[must_use]
    fn get_display_color(&self, channel: Self::Channel) -> Self;

    /// Returns the hue name if this channel represents a hue (for ARIA valuetext enrichment).
    ///
    /// Override this for color spaces that have a hue channel.
    fn get_hue_name_for_channel(&self, _channel: Self::Channel) -> Option<&'static str> {
        None
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
    ) -> AreaGradient {
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
                 linear-gradient(to right, {} 0%, {} 100%)",
                bottom.to_css_string(),
                top_left.to_css_string(),
                top_right.to_css_string(),
            ),
            blend_mode: None,
        }
    }
}

/// CSS background description for a 2D color area.
#[derive(Debug, Clone)]
pub struct AreaGradient {
    /// CSS `background` property value (may contain multiple layers).
    pub background: String,
    /// CSS `background-blend-mode` value, if needed (e.g., `"screen"` for RGB).
    pub blend_mode: Option<&'static str>,
}

// TODO: Add CMYK, ...
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(variant_size_differences)]
pub enum ColorSpace {
    HSV(HSV),
    HSL(HSL),
    RGB8(RGB8),
    RGBA8(RGBA8),
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

    /// Returns a human-readable name for a hue angle.
    ///
    /// Based on standard 30-degree segments of the hue wheel.
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn get_hue_name(hue: f64) -> &'static str {
        let h = ((hue % 360.0) + 360.0) % 360.0;
        // The hue is always in [0, 360) so the cast to u32 is safe.
        // The 360..=u32::MAX arm covers the theoretical case where h rounds to 360.
        match h as u32 {
            0..30 => "red",
            30..60 => "orange",
            60..90 => "yellow",
            90..120 => "chartreuse",
            120..150 => "green",
            150..180 => "spring green",
            180..210 => "cyan",
            210..240 => "azure",
            240..270 => "blue",
            270..300 => "violet",
            300..330 => "magenta",
            330..=u32::MAX => "rose",
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

    fn channels() -> &'static [HsvChannel] {
        &[
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

    fn get_hue_name_for_channel(&self, channel: HsvChannel) -> Option<&'static str> {
        match channel {
            HsvChannel::Hue => Some(Self::get_hue_name(self.hue)),
            HsvChannel::Saturation | HsvChannel::Brightness => None,
        }
    }

    fn get_area_gradient(
        &self,
        x_channel: Self::Channel,
        y_channel: Self::Channel,
    ) -> AreaGradient {
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

        let x_gradient = format!("linear-gradient(to right, {})", channel_gradient(x_channel));
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

    /// Parses a hex color string (with or without leading `#`).
    ///
    /// Accepts 6-character hex strings like `"FF00AA"` or `"#ff00aa"`.
    #[must_use]
    pub fn from_hex(s: &str) -> Option<Self> {
        let s = s.strip_prefix('#').unwrap_or(s);
        if s.len() != 6 {
            return None;
        }
        let r = u8::from_str_radix(&s[0..2], 16).ok()?;
        let g = u8::from_str_radix(&s[2..4], 16).ok()?;
        let b = u8::from_str_radix(&s[4..6], 16).ok()?;
        Some(Self { r, g, b })
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

    fn channels() -> &'static [RgbChannel] {
        &[RgbChannel::Red, RgbChannel::Green, RgbChannel::Blue]
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
    ) -> AreaGradient {
        let (_, _, z_channel) = Self::get_color_space_axes(Some(x_channel), Some(y_channel));
        let z_val = self.get_channel_value(z_channel);

        // Build a color with only the z-channel value set (others at 0).
        let base = Self { r: 0, g: 0, b: 0 }.with_channel_value(z_channel, z_val);

        // X-channel gradient: base → base + max X.
        let x_max = base.with_channel_value(x_channel, 255.0);
        // Y-channel gradient: base → base + max Y.
        let y_max = base.with_channel_value(y_channel, 255.0);

        AreaGradient {
            background: format!(
                "linear-gradient(to right, {} 0%, {} 100%), \
                 linear-gradient(to top, {} 0%, {} 100%), \
                 {}",
                base.to_css_string(),
                x_max.to_css_string(),
                base.to_css_string(),
                y_max.to_css_string(),
                base.to_css_string(),
            ),
            blend_mode: Some("screen"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RGBA8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
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

    fn channels() -> &'static [HslChannel] {
        &[
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

    fn get_hue_name_for_channel(&self, channel: HslChannel) -> Option<&'static str> {
        match channel {
            HslChannel::Hue => Some(HSV::get_hue_name(self.hue)),
            HslChannel::Saturation | HslChannel::Lightness => None,
        }
    }

    fn get_area_gradient(
        &self,
        x_channel: Self::Channel,
        y_channel: Self::Channel,
    ) -> AreaGradient {
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

        let x_gradient = format!("linear-gradient(to right, {})", channel_gradient(x_channel));
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
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
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
            60.0 * (((g - b) / delta) % 6.0)
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
            60.0 * (((g - b) / delta) % 6.0)
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
        assert_eq!(format!("{rgb:x}").as_str(), "ba17f1");
    }

    #[test]
    fn rgb8_to_upper_hex() {
        let rgb = RGB8 {
            r: 186,
            g: 23,
            b: 241,
        };
        assert_eq!(format!("{rgb:X}").as_str(), "BA17F1");
    }

    #[test]
    fn rgb8_display() {
        let rgb = RGB8 {
            r: 186,
            g: 23,
            b: 241,
        };
        assert_eq!(&rgb.to_string(), "#BA17F1");
    }

    // --- Hex parsing tests ---

    #[test]
    fn rgb8_from_hex_with_hash() {
        assert_that(RGB8::from_hex("#FF8000")).is_equal_to(Some(RGB8 {
            r: 255,
            g: 128,
            b: 0,
        }));
    }

    #[test]
    fn rgb8_from_hex_without_hash() {
        assert_that(RGB8::from_hex("ff8000")).is_equal_to(Some(RGB8 {
            r: 255,
            g: 128,
            b: 0,
        }));
    }

    #[test]
    fn rgb8_from_hex_invalid_length() {
        assert_that(RGB8::from_hex("FFF")).is_none();
    }

    #[test]
    fn rgb8_from_hex_invalid_chars() {
        assert_that(RGB8::from_hex("ZZZZZZ")).is_none();
    }

    #[test]
    fn rgb8_hex_int_roundtrip() {
        let rgb = RGB8 {
            r: 186,
            g: 23,
            b: 241,
        };
        let int = rgb.to_hex_int();
        assert_that(RGB8::from_hex_int(int)).is_equal_to(rgb);
    }

    // --- HSV ColorValue trait tests ---

    #[test]
    fn hsv_get_channel_value() {
        let hsv = HSV {
            hue: 120.0,
            saturation: 0.5,
            value: 0.8,
        };
        assert_that(hsv.get_channel_value(HsvChannel::Hue)).is_close_to(120.0, 0.001);
        assert_that(hsv.get_channel_value(HsvChannel::Saturation)).is_close_to(0.5, 0.001);
        assert_that(hsv.get_channel_value(HsvChannel::Brightness)).is_close_to(0.8, 0.001);
    }

    #[test]
    fn hsv_with_channel_value() {
        let hsv = HSV::new();
        let updated = hsv.with_channel_value(HsvChannel::Hue, 180.0);
        assert_that(updated.hue).is_close_to(180.0, 0.001);
        assert_that(updated.saturation).is_close_to(hsv.saturation, 0.001);
    }

    #[test]
    fn hsv_get_channel_range() {
        let range = HSV::get_channel_range(HsvChannel::Hue);
        assert_that(range.min_value).is_close_to(0.0, 0.001);
        assert_that(range.max_value).is_close_to(360.0, 0.001);
        assert_that(range.step).is_close_to(1.0, 0.001);
    }

    #[test]
    fn hsv_channels() {
        let channels = HSV::channels();
        assert_that(channels.len()).is_equal_to(3);
    }

    #[test]
    fn hsv_get_color_space_axes_defaults() {
        let (x, y, z) = HSV::get_color_space_axes(None, None);
        assert_that(x).is_equal_to(HsvChannel::Saturation);
        assert_that(y).is_equal_to(HsvChannel::Hue);
        assert_that(z).is_equal_to(HsvChannel::Brightness);
    }

    #[test]
    fn hsv_get_color_space_axes_custom() {
        let (x, y, z) =
            HSV::get_color_space_axes(Some(HsvChannel::Saturation), Some(HsvChannel::Brightness));
        assert_that(x).is_equal_to(HsvChannel::Saturation);
        assert_that(y).is_equal_to(HsvChannel::Brightness);
        assert_that(z).is_equal_to(HsvChannel::Hue);
    }

    #[test]
    fn hsv_to_css_string() {
        let hsv = HSV {
            hue: 0.0,
            saturation: 1.0,
            value: 1.0,
        };
        assert_that(hsv.to_css_string()).is_equal_to("rgb(255, 0, 0)".to_owned());
    }

    #[test]
    fn hsv_format_channel_value() {
        let hsv = HSV {
            hue: 120.0,
            saturation: 0.5,
            value: 0.8,
        };
        assert_that(hsv.format_channel_value(HsvChannel::Hue)).is_equal_to("120°".to_owned());
        assert_that(hsv.format_channel_value(HsvChannel::Saturation)).is_equal_to("50%".to_owned());
        assert_that(hsv.format_channel_value(HsvChannel::Brightness)).is_equal_to("80%".to_owned());
    }

    #[test]
    fn hsv_get_channel_name() {
        assert_that(HSV::get_channel_name(HsvChannel::Hue)).is_equal_to("Hue");
        assert_that(HSV::get_channel_name(HsvChannel::Saturation)).is_equal_to("Saturation");
        assert_that(HSV::get_channel_name(HsvChannel::Brightness)).is_equal_to("Brightness");
    }

    #[test]
    fn hsv_get_hue_name() {
        assert_that(HSV::get_hue_name(0.0)).is_equal_to("red");
        assert_that(HSV::get_hue_name(120.0)).is_equal_to("green");
        assert_that(HSV::get_hue_name(240.0)).is_equal_to("blue");
        assert_that(HSV::get_hue_name(60.0)).is_equal_to("yellow");
    }

    // --- RGB8 ColorValue trait tests ---

    #[test]
    fn rgb8_get_channel_value() {
        let rgb = RGB8 {
            r: 128,
            g: 64,
            b: 32,
        };
        assert_that(rgb.get_channel_value(RgbChannel::Red)).is_close_to(128.0, 0.001);
        assert_that(rgb.get_channel_value(RgbChannel::Green)).is_close_to(64.0, 0.001);
        assert_that(rgb.get_channel_value(RgbChannel::Blue)).is_close_to(32.0, 0.001);
    }

    #[test]
    fn rgb8_with_channel_value() {
        let rgb = RGB8 {
            r: 128,
            g: 64,
            b: 32,
        };
        let updated = rgb.with_channel_value(RgbChannel::Red, 200.0);
        assert_that(updated).is_equal_to(RGB8 {
            r: 200,
            g: 64,
            b: 32,
        });
    }

    #[test]
    fn rgb8_with_channel_value_clamps() {
        let rgb = RGB8::new();
        let updated = rgb.with_channel_value(RgbChannel::Red, 300.0);
        assert_that(updated.r).is_equal_to(255);
        let updated = rgb.with_channel_value(RgbChannel::Red, -10.0);
        assert_that(updated.r).is_equal_to(0);
    }

    #[test]
    fn rgb8_to_css_string() {
        let rgb = RGB8 {
            r: 128,
            g: 64,
            b: 32,
        };
        assert_that(rgb.to_css_string()).is_equal_to("rgb(128, 64, 32)".to_owned());
    }

    #[test]
    fn rgb8_get_channel_name() {
        assert_that(RGB8::get_channel_name(RgbChannel::Red)).is_equal_to("Red");
        assert_that(RGB8::get_channel_name(RgbChannel::Green)).is_equal_to("Green");
        assert_that(RGB8::get_channel_name(RgbChannel::Blue)).is_equal_to("Blue");
    }

    // --- HSL ColorValue trait tests ---

    #[test]
    fn hsl_get_channel_value() {
        let hsl = HSL {
            hue: 120.0,
            saturation: 0.5,
            lightness: 0.6,
        };
        assert_that(hsl.get_channel_value(HslChannel::Hue)).is_close_to(120.0, 0.001);
        assert_that(hsl.get_channel_value(HslChannel::Saturation)).is_close_to(0.5, 0.001);
        assert_that(hsl.get_channel_value(HslChannel::Lightness)).is_close_to(0.6, 0.001);
    }

    #[test]
    fn hsl_with_channel_value() {
        let hsl = HSL::new();
        let updated = hsl.with_channel_value(HslChannel::Hue, 240.0);
        assert_that(updated.hue).is_close_to(240.0, 0.001);
        assert_that(updated.saturation).is_close_to(hsl.saturation, 0.001);
    }

    #[test]
    fn hsl_get_channel_range() {
        let range = HSL::get_channel_range(HslChannel::Hue);
        assert_that(range.min_value).is_close_to(0.0, 0.001);
        assert_that(range.max_value).is_close_to(360.0, 0.001);
        assert_that(range.step).is_close_to(1.0, 0.001);

        let light_range = HSL::get_channel_range(HslChannel::Lightness);
        assert_that(light_range.gradient_stops.unwrap().len()).is_equal_to(3);
    }

    #[test]
    fn hsl_get_color_space_axes_defaults() {
        let (x, y, z) = HSL::get_color_space_axes(None, None);
        assert_that(x).is_equal_to(HslChannel::Saturation);
        assert_that(y).is_equal_to(HslChannel::Hue);
        assert_that(z).is_equal_to(HslChannel::Lightness);
    }

    #[test]
    fn hsl_format_channel_value() {
        let hsl = HSL {
            hue: 120.0,
            saturation: 0.5,
            lightness: 0.75,
        };
        assert_that(hsl.format_channel_value(HslChannel::Hue)).is_equal_to("120°".to_owned());
        assert_that(hsl.format_channel_value(HslChannel::Saturation)).is_equal_to("50%".to_owned());
        assert_that(hsl.format_channel_value(HslChannel::Lightness)).is_equal_to("75%".to_owned());
    }

    #[test]
    fn hsl_get_channel_name() {
        assert_that(HSL::get_channel_name(HslChannel::Hue)).is_equal_to("Hue");
        assert_that(HSL::get_channel_name(HslChannel::Saturation)).is_equal_to("Saturation");
        assert_that(HSL::get_channel_name(HslChannel::Lightness)).is_equal_to("Lightness");
    }

    #[test]
    fn hsl_display_color_hue() {
        let hsl = HSL {
            hue: 120.0,
            saturation: 0.3,
            lightness: 0.2,
        };
        let display = hsl.get_display_color(HslChannel::Hue);
        assert_that(display.hue).is_close_to(120.0, 0.001);
        assert_that(display.saturation).is_close_to(1.0, 0.001);
        assert_that(display.lightness).is_close_to(0.5, 0.001);
    }

    #[test]
    fn hsl_hue_name() {
        let hsl = HSL {
            hue: 120.0,
            saturation: 1.0,
            lightness: 0.5,
        };
        assert_that(hsl.get_hue_name_for_channel(HslChannel::Hue)).is_equal_to(Some("green"));
        assert_that(hsl.get_hue_name_for_channel(HslChannel::Saturation)).is_equal_to(None);
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
        assert_that(rgb).is_equal_to(RGB8 { r: 255, g: 0, b: 0 });
    }

    #[test]
    fn hsl_to_rgb8_green() {
        let hsl = HSL {
            hue: 120.0,
            saturation: 1.0,
            lightness: 0.5,
        };
        let rgb = RGB8::from(hsl);
        assert_that(rgb).is_equal_to(RGB8 { r: 0, g: 255, b: 0 });
    }

    #[test]
    fn hsl_to_rgb8_blue() {
        let hsl = HSL {
            hue: 240.0,
            saturation: 1.0,
            lightness: 0.5,
        };
        let rgb = RGB8::from(hsl);
        assert_that(rgb).is_equal_to(RGB8 { r: 0, g: 0, b: 255 });
    }

    #[test]
    fn hsl_to_rgb8_white() {
        let hsl = HSL {
            hue: 0.0,
            saturation: 0.0,
            lightness: 1.0,
        };
        let rgb = RGB8::from(hsl);
        assert_that(rgb).is_equal_to(RGB8 {
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
        assert_that(rgb).is_equal_to(RGB8 { r: 0, g: 0, b: 0 });
    }

    #[test]
    fn hsl_to_rgb8_gray() {
        let hsl = HSL {
            hue: 0.0,
            saturation: 0.0,
            lightness: 0.5,
        };
        let rgb = RGB8::from(hsl);
        assert_that(rgb).is_equal_to(RGB8 {
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
            assert_that(i16::from(back.r) - i16::from(original.r)).is_in_range(-1..=1);
            assert_that(i16::from(back.g) - i16::from(original.g)).is_in_range(-1..=1);
            assert_that(i16::from(back.b) - i16::from(original.b)).is_in_range(-1..=1);
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
        assert_that(back.hue).is_close_to(hsl.hue, 0.01);
        assert_that(back.saturation).is_close_to(hsl.saturation, 0.01);
        assert_that(back.lightness).is_close_to(hsl.lightness, 0.01);
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
        assert_that(back.hue).is_close_to(hsv.hue, 0.01);
        assert_that(back.saturation).is_close_to(hsv.saturation, 0.01);
        assert_that(back.value).is_close_to(hsv.value, 0.01);
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
        assert_that(hsv.saturation).is_close_to(0.0, 0.001);
    }
}

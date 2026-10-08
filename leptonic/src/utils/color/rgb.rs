//! The RGB color space.

use std::{fmt, str::FromStr};

use super::{
    AreaGradient, BlendMode, ColorChannelRange, ColorSpaceAxes, ColorValue, ParseColorError,
    axes_of, line_end,
};
use crate::utils::{i18n::Locale, locale::WritingDirection, number_formatter::NumberFormatOptions};

/// A channel of the RGB color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RgbChannel {
    Red,
    Green,
    Blue,
}

/// A color in the RGB color space, 8 bits per channel.
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

/// Parses a hex color (with or without a leading `#`): six digits (`"FF00AA"`) or the
/// three-digit shorthand (`"#f0a"`), as CSS.
impl FromStr for RGB8 {
    type Err = ParseColorError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let error = || ParseColorError::new(text);
        let digits = text.strip_prefix('#').unwrap_or(text);
        // Only hex digits: slicing by byte length needs ASCII, and `from_str_radix` takes a sign.
        if !digits.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(error());
        }
        let channel = |digits: &str| u8::from_str_radix(digits, 16).map_err(|_| error());
        match digits.len() {
            6 => Ok(Self {
                r: channel(&digits[0..2])?,
                g: channel(&digits[2..4])?,
                b: channel(&digits[4..6])?,
            }),
            3 => {
                let doubled = |i: usize| channel(&digits[i..=i].repeat(2));
                Ok(Self {
                    r: doubled(0)?,
                    g: doubled(1)?,
                    b: doubled(2)?,
                })
            }
            _ => Err(error()),
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

    fn channel_value(&self, channel: RgbChannel) -> f64 {
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

    fn channel_range(_channel: RgbChannel) -> ColorChannelRange {
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

    fn color_space_axes(
        x_channel: Option<RgbChannel>,
        y_channel: Option<RgbChannel>,
    ) -> ColorSpaceAxes<RgbChannel> {
        axes_of(
            [RgbChannel::Red, RgbChannel::Green, RgbChannel::Blue],
            x_channel,
            y_channel,
        )
    }

    fn to_css_string(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }

    fn channel_format_options(_channel: RgbChannel) -> NumberFormatOptions {
        NumberFormatOptions::default()
    }

    fn channel_name(channel: RgbChannel, locale: &Locale) -> String {
        let key = match channel {
            RgbChannel::Red => "red",
            RgbChannel::Green => "green",
            RgbChannel::Blue => "blue",
        };
        super::naming::channel_name(key, locale)
    }

    fn display_color(&self, _channel: RgbChannel) -> Self {
        *self
    }

    fn area_gradient(
        &self,
        x_channel: Self::Channel,
        y_channel: Self::Channel,
        direction: WritingDirection,
    ) -> AreaGradient {
        let end = line_end(direction);
        let z_channel = Self::color_space_axes(Some(x_channel), Some(y_channel)).z;
        let z_val = self.channel_value(z_channel);

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

//! The HSL color space.

use super::{
    AreaGradient, ColorChannelRange, ColorSpaceAxes, ColorValue, HSV, RGB8, axes_of,
    hue_space_area_gradient, hue_stops, round_fraction,
};
use crate::utils::{
    locale::WritingDirection,
    math::to_fixed_number,
    number_formatter::{NumberFormatOptions, NumberStyle, UnitDisplay},
};

/// A channel of the HSL color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HslChannel {
    Hue,
    Saturation,
    Lightness,
}

/// A color in the HSL color space: the hue in degrees (0 to 360), saturation and lightness from
/// 0 to 1.
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

    fn channel_value(&self, channel: HslChannel) -> f64 {
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

    fn channel_range(channel: HslChannel) -> ColorChannelRange {
        match channel {
            HslChannel::Hue => ColorChannelRange {
                min_value: 0.0,
                max_value: 360.0,
                step: 1.0,
                page_size: 15.0,
                gradient_stops: Some(&[0.0, 60.0, 120.0, 180.0, 240.0, 300.0, 360.0]),
            },
            // 0 to 1 (react-aria: 0 to 100; see the module's deviations).
            HslChannel::Saturation => ColorChannelRange {
                min_value: 0.0,
                max_value: 1.0,
                step: 0.01,
                page_size: 0.1,
                gradient_stops: None,
            },
            // 0 to 1 (react-aria: 0 to 100; see the module's deviations).
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

    fn color_space_axes(
        x_channel: Option<HslChannel>,
        y_channel: Option<HslChannel>,
    ) -> ColorSpaceAxes<HslChannel> {
        axes_of(
            [
                HslChannel::Hue,
                HslChannel::Saturation,
                HslChannel::Lightness,
            ],
            x_channel,
            y_channel,
        )
    }

    fn to_css_string(&self) -> String {
        let rgb = RGB8::from(*self);
        format!("rgb({}, {}, {})", rgb.r, rgb.g, rgb.b)
    }

    fn channel_format_options(channel: HslChannel) -> NumberFormatOptions {
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

    fn channel_name(channel: HslChannel) -> &'static str {
        match channel {
            HslChannel::Hue => "Hue",
            HslChannel::Saturation => "Saturation",
            HslChannel::Lightness => "Lightness",
        }
    }

    fn display_color(&self, channel: HslChannel) -> Self {
        match channel {
            HslChannel::Hue => Self {
                hue: self.hue,
                saturation: 1.0,
                lightness: 0.5,
            },
            HslChannel::Saturation | HslChannel::Lightness => *self,
        }
    }

    fn area_gradient(
        &self,
        x_channel: HslChannel,
        y_channel: HslChannel,
        direction: WritingDirection,
    ) -> AreaGradient {
        let axes = Self::color_space_axes(Some(x_channel), Some(y_channel));
        // Vivid HSL with the current z value (react-aria: `parseColor('hsl(0, 100%, 50%)')
        // .withChannelValue(zChannel, zValue)`).
        let base = Self::new().with_channel_value(axes.z, self.channel_value(axes.z));
        hue_space_area_gradient(base, axes, direction, |channel| match channel {
            HslChannel::Hue => hue_stops(base, HslChannel::Hue),
            HslChannel::Saturation => format!(
                "{}, transparent",
                base.with_channel_value(HslChannel::Saturation, 0.0)
                    .to_css_string()
            ),
            HslChannel::Lightness => "black, transparent, white".to_owned(),
        })
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

        // Rounded as react-aria's `toHSL` (two decimals of degrees and percentages).
        Self {
            hue: to_fixed_number(hue, 2),
            saturation: round_fraction(saturation),
            lightness: round_fraction(lightness),
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
        // Rounded as react-aria's `toHSB`.
        HSV {
            hue: to_fixed_number(hue, 2),
            saturation: round_fraction(sv),
            brightness: round_fraction(v),
        }
    }
}

impl From<HSV> for HSL {
    fn from(hsv: HSV) -> Self {
        let HSV {
            hue,
            saturation: s,
            brightness: v,
        } = hsv;
        let l = v * (1.0 - s / 2.0);
        #[allow(clippy::float_cmp)]
        let sl = if l == 0.0 || l == 1.0 {
            0.0
        } else {
            (v - l) / f64::min(l, 1.0 - l)
        };
        // Rounded as react-aria's `toHSL`.
        Self {
            hue: to_fixed_number(hue, 2),
            saturation: round_fraction(sl),
            lightness: round_fraction(l),
        }
    }
}

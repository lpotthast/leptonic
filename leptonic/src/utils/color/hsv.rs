//! The HSV (HSB) color space.

use super::{
    AreaGradient, ColorChannelRange, ColorSpaceAxes, ColorValue, HUE_STOPS, RGB8, axes_of,
    hue_space_area_gradient, hue_stops, round_fraction,
};
use crate::utils::{
    i18n::{Locale, WritingDirection},
    math::to_fixed_number,
    number_formatter::{NumberFormatOptions, NumberStyle, UnitDisplay},
};

/// A channel of the HSV color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HsvChannel {
    Hue,
    Saturation,
    Brightness,
}

/// A color in the HSV (HSB) color space: the hue in degrees (0 to 360), saturation and
/// brightness from 0 to 1.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HSV {
    pub hue: f64,
    pub saturation: f64,
    pub brightness: f64,
}

impl HSV {
    /// Red: hue 0 with full saturation and brightness.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            hue: 0.0,
            saturation: 1.0,
            brightness: 1.0,
        }
    }

    #[must_use]
    pub const fn from_hue_fully_saturated(hue: f64) -> Self {
        Self {
            hue,
            saturation: 1.0,
            brightness: 1.0,
        }
    }

    #[must_use]
    pub const fn with_hue(self, hue: f64) -> Self {
        Self {
            hue,
            saturation: self.saturation,
            brightness: self.brightness,
        }
    }

    #[must_use]
    pub const fn with_saturation(self, saturation: f64) -> Self {
        Self {
            hue: self.hue,
            saturation,
            brightness: self.brightness,
        }
    }

    #[must_use]
    pub const fn with_brightness(self, brightness: f64) -> Self {
        Self {
            hue: self.hue,
            saturation: self.saturation,
            brightness,
        }
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

    fn channel_value(&self, channel: HsvChannel) -> f64 {
        match channel {
            HsvChannel::Hue => self.hue,
            HsvChannel::Saturation => self.saturation,
            HsvChannel::Brightness => self.brightness,
        }
    }

    fn with_channel_value(&self, channel: HsvChannel, value: f64) -> Self {
        match channel {
            HsvChannel::Hue => self.with_hue(value),
            HsvChannel::Saturation => self.with_saturation(value),
            HsvChannel::Brightness => self.with_brightness(value),
        }
    }

    fn channel_range(channel: HsvChannel) -> ColorChannelRange {
        match channel {
            HsvChannel::Hue => ColorChannelRange {
                min_value: 0.0,
                max_value: 360.0,
                step: 1.0,
                page_size: 15.0,
            },
            // 0 to 1 (react-aria: 0 to 100; see the module's deviations).
            HsvChannel::Saturation | HsvChannel::Brightness => ColorChannelRange {
                min_value: 0.0,
                max_value: 1.0,
                step: 0.01,
                page_size: 0.1,
            },
        }
    }

    fn channels() -> [HsvChannel; 3] {
        [
            HsvChannel::Hue,
            HsvChannel::Saturation,
            HsvChannel::Brightness,
        ]
    }

    fn gradient_stops(channel: HsvChannel) -> &'static [f64] {
        match channel {
            HsvChannel::Hue => &HUE_STOPS,
            HsvChannel::Saturation | HsvChannel::Brightness => &[0.0, 1.0],
        }
    }

    fn color_space_axes(
        x_channel: Option<HsvChannel>,
        y_channel: Option<HsvChannel>,
    ) -> ColorSpaceAxes<HsvChannel> {
        axes_of(Self::channels(), x_channel, y_channel)
    }

    fn to_css_string(&self) -> String {
        let rgb = RGB8::from(*self);
        format!("rgb({}, {}, {})", rgb.r, rgb.g, rgb.b)
    }

    fn channel_format_options(channel: HsvChannel) -> NumberFormatOptions {
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

    fn channel_name(channel: HsvChannel, locale: &Locale) -> String {
        let key = match channel {
            HsvChannel::Hue => "hue",
            HsvChannel::Saturation => "saturation",
            HsvChannel::Brightness => "brightness",
        };
        super::naming::channel_name(key, locale)
    }

    fn display_color(&self, channel: HsvChannel) -> Self {
        match channel {
            HsvChannel::Hue => Self {
                hue: self.hue,
                saturation: 1.0,
                brightness: 1.0,
            },
            HsvChannel::Saturation | HsvChannel::Brightness => *self,
        }
    }

    fn area_gradient(
        &self,
        x_channel: HsvChannel,
        y_channel: HsvChannel,
        direction: WritingDirection,
    ) -> AreaGradient {
        let axes = Self::color_space_axes(Some(x_channel), Some(y_channel));
        // Vivid HSV with the current z value (react-aria: `parseColor('hsb(0, 100%, 100%)')
        // .withChannelValue(zChannel, zValue)`).
        let base = Self::new().with_channel_value(axes.z, self.channel_value(axes.z));
        hue_space_area_gradient(base, axes, direction, |channel| match channel {
            HsvChannel::Hue => hue_stops(base, HsvChannel::Hue),
            HsvChannel::Saturation => format!(
                "{}, transparent",
                base.with_channel_value(HsvChannel::Saturation, 0.0)
                    .to_css_string()
            ),
            HsvChannel::Brightness => "black, transparent".to_owned(),
        })
    }
}

impl From<HSV> for RGB8 {
    // Expectations: 0 ≤ S ≤ 1 and 0 ≤ V ≤ 1; hues wrap around (react-aria: `% 6` of sixths).
    #[allow(
        clippy::many_single_char_names,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    fn from(hsv: HSV) -> Self {
        let (h, s, v) = (hsv.hue.rem_euclid(360.0), hsv.saturation, hsv.brightness);

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
            // NaN, or 360° (`rem_euclid` rounds tiny negative hues up to it): as 0°.
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

        // Rounded as react-aria's `toHSB` (two decimals of degrees and percentages).
        Self {
            hue: to_fixed_number(hue, 2),
            saturation: round_fraction(saturation),
            brightness: round_fraction(c_max),
        }
    }
}

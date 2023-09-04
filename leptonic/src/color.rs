// TODO: Add HSL, CMYK, ...
#[derive(Debug, Clone, Copy)]
pub enum ColorSpace {
    HSV(HSV),
    RGB8(RGB8),
    RGBA8(RGBA8),
}

#[derive(Debug, Clone, Copy)]
pub struct HSV {
    pub hue: f64,
    pub saturation: f64,
    pub value: f64,
}

impl HSV {
    /// Hue starting at red (0.0 degrees) with full saturation (1.0) and full value (1.0).
    /// This is a sensitive way to construct a new HSV value.
    pub fn new() -> Self {
        Self {
            hue: 0.0,
            saturation: 1.0,
            value: 1.0,
        }
    }

    pub fn from_hue_fully_saturated(hue: f64) -> Self {
        Self {
            hue,
            saturation: 1.0,
            value: 1.0,
        }
    }

    pub fn with_hue(self, hue: f64) -> Self {
        Self {
            hue,
            saturation: self.saturation,
            value: self.value,
        }
    }

    pub fn with_saturation(self, saturation: f64) -> Self {
        Self {
            hue: self.hue,
            saturation,
            value: self.value,
        }
    }

    pub fn with_value(self, value: f64) -> Self {
        Self {
            hue: self.hue,
            saturation: self.saturation,
            value,
        }
    }
}

impl Default for HSV {
    fn default() -> Self {
        Self::new()
    }
}

impl HSV {
    pub fn into_rgb8(self) -> RGB8 {
        RGB8::from(self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RGB8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB8 {
    pub fn new() -> RGB8 {
        Self { r: 0, g: 0, b: 0 }
    }

    pub fn into_hsv(self) -> HSV {
        HSV::from(self)
    }
}

impl Default for RGB8 {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::LowerHex for RGB8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:02x}", self.r))?;
        f.write_fmt(format_args!("{:02x}", self.g))?;
        f.write_fmt(format_args!("{:02x}", self.b))
    }
}

impl std::fmt::UpperHex for RGB8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:02X}", self.r))?;
        f.write_fmt(format_args!("{:02X}", self.g))?;
        f.write_fmt(format_args!("{:02X}", self.b))
    }
}

impl std::fmt::Display for RGB8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("#{:X}", self))
    }
}

impl From<(u8, u8, u8)> for RGB8 {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self { r, g, b }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RGBA8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<HSV> for RGB8 {
    // Expectations: 0 ≤ H < 360, 0 ≤ S ≤ 1 and 0 ≤ V ≤ 1:
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

        RGB8 { r, g, b }
    }
}

impl From<RGB8> for HSV {
    fn from(rgb: RGB8) -> Self {
        let RGB8 { r, g, b } = rgb;

        let (r, g, b) = (r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0);

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

        let saturation = match c_max == 0.0 {
            true => 0.0,
            false => delta / c_max,
        };

        let value = c_max;

        HSV {
            hue,
            saturation,
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RGB8;

    #[test]
    fn rgb8_to_lower_hex() {
        let rgb = RGB8 {
            r: 186,
            g: 23,
            b: 241,
        };
        assert_eq!(format!("{:x}", rgb).as_str(), "ba17f1");
    }

    #[test]
    fn rgb8_to_upper_hex() {
        let rgb = RGB8 {
            r: 186,
            g: 23,
            b: 241,
        };
        assert_eq!(format!("{:X}", rgb).as_str(), "BA17F1");
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
}

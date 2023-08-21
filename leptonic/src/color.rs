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
    pub fn into_hsv(self) -> HSV {
        HSV::from(self)
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

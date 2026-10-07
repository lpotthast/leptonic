//! Color names (react-aria's `getColorName`/`getHueName`, in English).
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
            let color = hex.parse::<RGB8>().expect("a hex color");
            assert_that!(color_name(color))
                .with_detail_message(hex)
                .is_equal_to(name.to_owned());
        }
        assert_that!(hue_name("#d2691e".parse::<RGB8>().expect("a hex color")))
            .is_equal_to("brown".to_owned());
    }
}

//! Parsing colors (react-aria's `parseColor`).

use std::{fmt, str::FromStr};

use super::{Alpha, Color, HSL, HSV, OpaqueColor, RGB8};

/// Text that is no color [`Color`] can parse.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseColorError(String);

impl fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid color: {:?}", self.0)
    }
}

impl std::error::Error for ParseColorError {}

impl ParseColorError {
    pub(super) fn new(text: &str) -> Self {
        Self(text.to_owned())
    }
}

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
    let arguments = arguments_of(inner)?;
    (arguments.len() == if with_alpha { 4 } else { 3 }).then_some((arguments, with_alpha))
}

/// The comma-separated arguments of `(...)`.
fn arguments_of(text: &str) -> Option<Vec<&str>> {
    let inner = text.strip_prefix('(')?.strip_suffix(')')?;
    Some(inner.split(',').map(str::trim).collect())
}

/// The arguments of `rgb(...)` or `rgba(...)`: either name with 3 or 4 arguments, the fourth the
/// alpha (react-aria's `/^rgba?\((.*)\)$/`).
fn rgb_arguments(text: &str) -> Option<Vec<&str>> {
    let inner = text
        .strip_prefix("rgba")
        .or_else(|| text.strip_prefix("rgb"))?;
    let arguments = arguments_of(inner)?;
    matches!(arguments.len(), 3 | 4).then_some(arguments)
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
    Some(Color::from(color.parse::<RGB8>().ok()?).with_alpha(f64::from(alpha) / 255.0))
}

/// react-aria's `parseColor`: RGB (hex or `rgb()`/`rgba()`), then HSB, then HSL.
fn parse_color(text: &str) -> Option<Color> {
    if text.starts_with('#') {
        return text
            .parse::<RGB8>()
            .ok()
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
    if let Some(arguments) = rgb_arguments(text) {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let channel = |text: &str| number(text).map(|n| n.clamp(0.0, 255.0).round() as u8);
        let color = RGB8 {
            r: channel(arguments[0])?,
            g: channel(arguments[1])?,
            b: channel(arguments[2])?,
        };
        return Some(Color::from(color).with_alpha(alpha(&arguments, arguments.len() == 4)?));
    }
    if let Some((arguments, with_alpha)) = css_arguments(text, "hsb", "hsba") {
        let color = HSV {
            hue: hue(arguments[0])?,
            saturation: percent(arguments[1])?,
            brightness: percent(arguments[2])?,
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

use assertr::prelude::*;

use super::*;
use crate::utils::i18n::Locale;

fn en() -> Locale {
    "en-US".parse().expect("a valid locale")
}

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
    assert_that!("#FF8000".parse::<RGB8>().ok()).is_equal_to(Some(RGB8 {
        r: 255,
        g: 128,
        b: 0,
    }));
}

#[test]
fn rgb8_from_hex_without_hash() {
    assert_that!("ff8000".parse::<RGB8>().ok()).is_equal_to(Some(RGB8 {
        r: 255,
        g: 128,
        b: 0,
    }));
}

#[test]
fn rgb8_from_hex_invalid_length() {
    assert_that!("FFFF".parse::<RGB8>().ok()).is_none();
    assert_that!("FFFFF".parse::<RGB8>().ok()).is_none();
}

#[test]
fn rgb8_from_hex_shorthand() {
    assert_that!("#f0a".parse::<RGB8>().ok()).is_equal_to(Some(RGB8 {
        r: 0xFF,
        g: 0x00,
        b: 0xAA,
    }));
}

#[test]
fn rgb8_from_hex_invalid_chars() {
    assert_that!("ZZZZZZ".parse::<RGB8>().ok()).is_none();
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
fn hsv_channel_value() {
    let hsv = HSV {
        hue: 120.0,
        saturation: 0.5,
        brightness: 0.8,
    };
    assert_that!(hsv.channel_value(HsvChannel::Hue)).is_close_to(120.0, 0.001);
    assert_that!(hsv.channel_value(HsvChannel::Saturation)).is_close_to(0.5, 0.001);
    assert_that!(hsv.channel_value(HsvChannel::Brightness)).is_close_to(0.8, 0.001);
}

#[test]
fn hsv_with_channel_value() {
    let hsv = HSV::new();
    let updated = hsv.with_channel_value(HsvChannel::Hue, 180.0);
    assert_that!(updated.hue).is_close_to(180.0, 0.001);
    assert_that!(updated.saturation).is_close_to(hsv.saturation, 0.001);
}

#[test]
fn hsv_channel_range() {
    let range = HSV::channel_range(HsvChannel::Hue);
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
    // react-aria's `getColorSpaceAxes`: the space's channels in order.
    let ColorSpaceAxes { x, y, z } = HSV::color_space_axes(None, None);
    assert_that!(x).is_equal_to(HsvChannel::Hue);
    assert_that!(y).is_equal_to(HsvChannel::Saturation);
    assert_that!(z).is_equal_to(HsvChannel::Brightness);
    // Only y given: x is the first other channel.
    let axes = HSV::color_space_axes(None, Some(HsvChannel::Hue));
    assert_that!(axes.x).is_equal_to(HsvChannel::Saturation);
    assert_that!(axes.z).is_equal_to(HsvChannel::Brightness);
}

#[test]
fn hsv_get_color_space_axes_custom() {
    let ColorSpaceAxes { x, y, z } =
        HSV::color_space_axes(Some(HsvChannel::Saturation), Some(HsvChannel::Brightness));
    assert_that!(x).is_equal_to(HsvChannel::Saturation);
    assert_that!(y).is_equal_to(HsvChannel::Brightness);
    assert_that!(z).is_equal_to(HsvChannel::Hue);
}

#[test]
fn hsv_to_css_string() {
    let hsv = HSV {
        hue: 0.0,
        saturation: 1.0,
        brightness: 1.0,
    };
    assert_that!(hsv.to_css_string()).is_equal_to("rgb(255, 0, 0)".to_owned());
}

#[test]
fn hsv_format_channel_value() {
    let hsv = HSV {
        hue: 120.0,
        saturation: 0.5,
        brightness: 0.8,
    };
    assert_that!(hsv.format_channel_value(HsvChannel::Hue, &en())).is_equal_to("120°".to_owned());
    assert_that!(hsv.format_channel_value(HsvChannel::Saturation, &en()))
        .is_equal_to("50%".to_owned());
    assert_that!(hsv.format_channel_value(HsvChannel::Brightness, &en()))
        .is_equal_to("80%".to_owned());
}

#[test]
fn hsv_channel_name() {
    assert_that!(HSV::channel_name(HsvChannel::Hue)).is_equal_to("Hue");
    assert_that!(HSV::channel_name(HsvChannel::Saturation)).is_equal_to("Saturation");
    assert_that!(HSV::channel_name(HsvChannel::Brightness)).is_equal_to("Brightness");
}

// --- RGB8 ColorValue trait tests ---

#[test]
fn rgb8_channel_value() {
    let rgb = RGB8 {
        r: 128,
        g: 64,
        b: 32,
    };
    assert_that!(rgb.channel_value(RgbChannel::Red)).is_close_to(128.0, 0.001);
    assert_that!(rgb.channel_value(RgbChannel::Green)).is_close_to(64.0, 0.001);
    assert_that!(rgb.channel_value(RgbChannel::Blue)).is_close_to(32.0, 0.001);
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
fn rgb8_channel_name() {
    assert_that!(RGB8::channel_name(RgbChannel::Red)).is_equal_to("Red");
    assert_that!(RGB8::channel_name(RgbChannel::Green)).is_equal_to("Green");
    assert_that!(RGB8::channel_name(RgbChannel::Blue)).is_equal_to("Blue");
}

// --- HSL ColorValue trait tests ---

#[test]
fn hsl_channel_value() {
    let hsl = HSL {
        hue: 120.0,
        saturation: 0.5,
        lightness: 0.6,
    };
    assert_that!(hsl.channel_value(HslChannel::Hue)).is_close_to(120.0, 0.001);
    assert_that!(hsl.channel_value(HslChannel::Saturation)).is_close_to(0.5, 0.001);
    assert_that!(hsl.channel_value(HslChannel::Lightness)).is_close_to(0.6, 0.001);
}

#[test]
fn hsl_with_channel_value() {
    let hsl = HSL::new();
    let updated = hsl.with_channel_value(HslChannel::Hue, 240.0);
    assert_that!(updated.hue).is_close_to(240.0, 0.001);
    assert_that!(updated.saturation).is_close_to(hsl.saturation, 0.001);
}

#[test]
fn hsl_channel_range() {
    let range = HSL::channel_range(HslChannel::Hue);
    assert_that!(range.min_value).is_close_to(0.0, 0.001);
    assert_that!(range.max_value).is_close_to(360.0, 0.001);
    assert_that!(range.step).is_close_to(1.0, 0.001);

    let light_range = HSL::channel_range(HslChannel::Lightness);
    assert_that!(light_range.gradient_stops.unwrap().len()).is_equal_to(3);
}

#[test]
fn hsl_get_color_space_axes_defaults() {
    let ColorSpaceAxes { x, y, z } = HSL::color_space_axes(None, None);
    assert_that!(x).is_equal_to(HslChannel::Hue);
    assert_that!(y).is_equal_to(HslChannel::Saturation);
    assert_that!(z).is_equal_to(HslChannel::Lightness);
}

#[test]
fn hsl_format_channel_value() {
    let hsl = HSL {
        hue: 120.0,
        saturation: 0.5,
        lightness: 0.75,
    };
    assert_that!(hsl.format_channel_value(HslChannel::Hue, &en())).is_equal_to("120°".to_owned());
    assert_that!(hsl.format_channel_value(HslChannel::Saturation, &en()))
        .is_equal_to("50%".to_owned());
    assert_that!(hsl.format_channel_value(HslChannel::Lightness, &en()))
        .is_equal_to("75%".to_owned());
}

#[test]
fn hsl_channel_name() {
    assert_that!(HSL::channel_name(HslChannel::Hue)).is_equal_to("Hue");
    assert_that!(HSL::channel_name(HslChannel::Saturation)).is_equal_to("Saturation");
    assert_that!(HSL::channel_name(HslChannel::Lightness)).is_equal_to("Lightness");
}

#[test]
fn hsl_display_color_hue() {
    let hsl = HSL {
        hue: 120.0,
        saturation: 0.3,
        lightness: 0.2,
    };
    let display = hsl.display_color(HslChannel::Hue);
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
        brightness: 0.9,
    };
    let hsl = HSL::from(hsv);
    let back = HSV::from(hsl);
    assert_that!(back.hue).is_close_to(hsv.hue, 0.01);
    assert_that!(back.saturation).is_close_to(hsv.saturation, 0.01);
    assert_that!(back.brightness).is_close_to(hsv.brightness, 0.01);
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
        brightness: 1.0,
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
        brightness: 0.5,
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
    assert_that!(parse("#abc9").to_css_string()).is_equal_to("rgba(170, 187, 204, 0.6)".to_owned());
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
    assert_that!(half.format_channel_value(channel, &en())).is_equal_to("50%".to_owned());
    assert_that!(Alpha::<HSV>::channels().len()).is_equal_to(3);
}

#[test]
fn channel_values_are_formatted_for_the_locale() {
    let german: Locale = "de-DE".parse().expect("a valid locale");
    let hsv = HSV {
        hue: 45.5,
        saturation: 0.5,
        brightness: 1.0,
    };
    assert_that!(hsv.format_channel_value(HsvChannel::Hue, &german))
        .is_equal_to("45,5°".to_owned());
    assert_that!(hsv.format_channel_value(HsvChannel::Hue, &en())).is_equal_to("45.5°".to_owned());
    let rgb = RGB8 { r: 7, g: 0, b: 0 };
    assert_that!(rgb.format_channel_value(RgbChannel::Red, &german)).is_equal_to("7".to_owned());
}

#[test]
fn area_gradients_layer_the_later_channel_on_top() {
    let hsv = HSV {
        hue: 0.0,
        saturation: 1.0,
        brightness: 1.0,
    };
    let ltr = WritingDirection::Ltr;
    // x = saturation, y = brightness (z = hue): brightness on top, the hue color below.
    let background = hsv
        .area_gradient(HsvChannel::Saturation, HsvChannel::Brightness, ltr)
        .background;
    assert_that!(background.as_str()).is_equal_to(
        "linear-gradient(to top, black, transparent), \
         linear-gradient(to right, rgb(255, 255, 255), transparent), rgb(255, 0, 0)",
    );
    // Swapped axes: brightness still on top, now running to the right.
    let background = hsv
        .area_gradient(HsvChannel::Brightness, HsvChannel::Saturation, ltr)
        .background;
    assert_that!(background.as_str()).starts_with("linear-gradient(to right, black, transparent), linear-gradient(to top, rgb(255, 255, 255), transparent)");
    // y = hue: the saturation layer (x) is above the hue layer.
    let background = hsv
        .area_gradient(HsvChannel::Saturation, HsvChannel::Hue, ltr)
        .background;
    assert_that!(background.as_str()).starts_with("linear-gradient(to right, ");
    assert_that!(background.as_str()).contains("), linear-gradient(to top, rgb(255, 0, 0), ");
    // HSL: lightness on top.
    let hsl = HSL::new();
    let background = hsl
        .area_gradient(
            HslChannel::Lightness,
            HslChannel::Saturation,
            WritingDirection::Rtl,
        )
        .background;
    assert_that!(background.as_str()).starts_with(
        "linear-gradient(to left, black, transparent, white), linear-gradient(to top, ",
    );
}

#[test]
fn parses_rgb_with_either_name_and_three_or_four_arguments() {
    let parse = |text: &str| text.parse::<Color>().expect("a color");
    assert_that!(parse("rgb(10, 20, 30, 0.5)").alpha).is_equal_to(0.5);
    assert_that!(parse("rgba(10, 20, 30)").to::<RGB8>()).is_equal_to(RGB8 {
        r: 10,
        g: 20,
        b: 30,
    });
    assert_that!(parse("rgba(10, 20, 30)").alpha).is_equal_to(1.0);
    assert_that!("rgb(10, 20)".parse::<Color>()).is_err();
    assert_that!("rgb(10, 20, 30, 0.5, 1)".parse::<Color>()).is_err();
}

#[test]
fn formats_colors_as_react_aria() {
    let color: Color = "hsba(120, 50%, 80%, 0.5)".parse().expect("a color");
    assert_that!(color.to_string_as(ColorFormat::Hsb)).is_equal_to("hsb(120, 50%, 80%)".to_owned());
    assert_that!(color.to_string_as(ColorFormat::Hsba))
        .is_equal_to("hsba(120, 50%, 80%, 0.5)".to_owned());
    assert_that!(color.to_string_as(ColorFormat::Hex)).is_equal_to("#66CC66".to_owned());
    assert_that!(color.to_string_as(ColorFormat::Hexa)).is_equal_to("#66CC6680".to_owned());
    assert_that!(color.to_string_as(ColorFormat::Rgb)).is_equal_to("rgb(102, 204, 102)".to_owned());
    assert_that!(color.to_string_as(ColorFormat::Rgba))
        .is_equal_to("rgba(102, 204, 102, 0.5)".to_owned());
    assert_that!(color.to_string_as(ColorFormat::Hsl)).is_equal_to("hsl(120, 50%, 60%)".to_owned());
    assert_that!(color.to_string_as(ColorFormat::Hsla))
        .is_equal_to("hsla(120, 50%, 60%, 0.5)".to_owned());
}

#[test]
fn conversions_round_to_two_decimals_of_degrees_and_percent() {
    let hsv = HSV::from(RGB8 {
        r: 10,
        g: 20,
        b: 30,
    });
    assert_that!(hsv.hue).is_equal_to(210.0);
    assert_that!(hsv.saturation).is_equal_to(0.6667);
    assert_that!(hsv.brightness).is_equal_to(0.1176);
    let hsl = HSL::from(RGB8 {
        r: 10,
        g: 20,
        b: 30,
    });
    assert_that!(hsl.saturation).is_equal_to(0.5);
    assert_that!(hsl.lightness).is_equal_to(0.0784);
    let hsv = HSV::from(HSL {
        hue: 33.333_333,
        saturation: 1.0 / 3.0,
        lightness: 0.5,
    });
    assert_that!(hsv.hue).is_equal_to(33.33);
    assert_that!(hsv.saturation).is_equal_to(0.5);
}

#[test]
fn the_opaque_color_drops_the_alpha() {
    let color = Alpha::new(RGB8 { r: 1, g: 2, b: 3 }).with_alpha(0.3);
    assert_that!(color.opaque().alpha).is_equal_to(1.0);
    assert_that!(RGB8 { r: 1, g: 2, b: 3 }.opaque()).is_equal_to(RGB8 { r: 1, g: 2, b: 3 });
}

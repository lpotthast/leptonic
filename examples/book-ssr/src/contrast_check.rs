//! Checks that the text colors of the book's design tokens (`style/book/_theme.scss`) meet WCAG AA in both themes:
//! 4.5:1 for text, 3:1 for large text and focus rings. The browser tests check the rendered chrome as well
//! (`tests/ui_tests/test_contrast.rs`).

use std::{collections::HashMap, fs, path::Path};

use assertr::prelude::*;

/// The custom properties declared in the blocks of `scss` whose selector is `selector`.
fn declarations(scss: &str, selector: &str) -> HashMap<String, String> {
    let mut found = HashMap::new();
    for (start, _) in scss.match_indices(&format!("{selector} {{")) {
        // Only blocks starting a line: `[data-theme]` must not match inside `[data-theme="light"]`.
        if start > 0 && !scss[..start].ends_with('\n') {
            continue;
        }
        let block = &scss[start + selector.len() + 2..];
        let block = &block[..block.find("\n}").unwrap_or(block.len())];
        for line in block.lines() {
            let line = line.trim();
            if let Some((name, value)) = line.strip_prefix("--").and_then(|line| line.split_once(':')) {
                found.insert(
                    format!("--{name}"),
                    value.trim().trim_end_matches(';').trim().to_owned(),
                );
            }
        }
    }
    found
}

/// The variables of one theme: leptonic's theme, then the book's overrides and tokens.
struct Theme(HashMap<String, String>);

impl Theme {
    fn read(name: &str) -> Self {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("style");
        let leptonic = fs::read_to_string(dir.join(format!("leptonic/themes/{name}.scss")))
            .expect("leptonic's theme is copied into style/leptonic by its build script");
        let book = fs::read_to_string(dir.join("book/_theme.scss")).expect("the book's theme is readable");
        let mut variables = declarations(&leptonic, &format!("[data-theme=\"{name}\"]"));
        variables.extend(declarations(&book, ":root"));
        variables.extend(declarations(&book, "[data-theme]"));
        variables.extend(declarations(&book, &format!("[data-theme=\"{name}\"]")));
        Self(variables)
    }

    /// The value of `variable`, with `var(..)` references resolved.
    fn value(&self, variable: &str) -> String {
        let mut value = self
            .0
            .get(variable)
            .unwrap_or_else(|| panic!("{variable} is defined"))
            .clone();
        while let Some(reference) = value
            .strip_prefix("var(")
            .and_then(|rest| rest.strip_suffix(')'))
        {
            value = self.value(reference);
        }
        value
    }

    fn color(&self, variable: &str) -> [u8; 3] {
        parse_hex(&self.value(variable)).unwrap_or_else(|| panic!("{variable} is a hex color"))
    }

    /// The hex colors (stops) of a gradient.
    fn stops(&self, variable: &str) -> Vec<[u8; 3]> {
        self.value(variable)
            .split(|c: char| !(c.is_ascii_hexdigit() || c == '#'))
            .filter(|part| part.starts_with('#'))
            .filter_map(parse_hex)
            .collect()
    }
}

fn parse_hex(value: &str) -> Option<[u8; 3]> {
    let hex = value.strip_prefix('#')?;
    let hex: String = match hex.len() {
        3 => hex.chars().flat_map(|c| [c, c]).collect(),
        6 => hex.to_owned(),
        _ => return None,
    };
    let channel = |i: usize| u8::from_str_radix(&hex[i..i + 2], 16).ok();
    Some([channel(0)?, channel(2)?, channel(4)?])
}

fn luminance(color: [u8; 3]) -> f64 {
    let channel = |value: u8| {
        let value = f64::from(value) / 255.0;
        if value <= 0.040_45 {
            value / 12.92
        } else {
            ((value + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * channel(color[0]) + 0.7152 * channel(color[1]) + 0.0722 * channel(color[2])
}

fn contrast(a: [u8; 3], b: [u8; 3]) -> f64 {
    let (a, b) = (luminance(a), luminance(b));
    (a.max(b) + 0.05) / (a.min(b) + 0.05)
}

#[test]
fn token_colors_meet_wcag_aa_in_both_themes() {
    const TEXT: f64 = 4.5;
    const LARGE: f64 = 3.0;

    let backgrounds = [
        "--main-background-color",
        "--book-surface-color",
        "--book-surface-color-raised",
        "--book-hover-background-color",
    ];
    let mut problems = Vec::new();
    for name in ["light", "dark"] {
        let theme = Theme::read(name);
        let mut check = |text: [u8; 3], text_name: &str, background: &str, minimum: f64| {
            let ratio = contrast(text, theme.color(background));
            if ratio < minimum {
                problems.push(format!("{name}: {text_name} on {background}: {ratio:.2} < {minimum}"));
            }
        };
        for text in ["--main-color", "--book-brand-text-color", "--book-muted-color", "--link-color"] {
            for background in backgrounds {
                check(theme.color(text), text, background, TEXT);
            }
        }
        for background in [
            "--brand-color",
            "--book-badge-hook-color",
            "--book-badge-atom-color",
            "--book-badge-comp-color",
            "--book-badge-util-color",
        ] {
            check(theme.color("--book-on-brand-color"), "--book-on-brand-color", background, TEXT);
        }
        check(theme.color("--book-focus-color"), "--book-focus-color", "--main-background-color", LARGE);
        let stops = theme.stops("--book-brand-gradient");
        assert_that!(stops.len()).is_greater_or_equal_to(2);
        for stop in stops {
            check(stop, "a --book-brand-gradient stop", "--main-background-color", LARGE);
        }
    }
    assert_that!(problems).is_empty();
}

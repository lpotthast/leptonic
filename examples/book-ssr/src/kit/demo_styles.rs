//! The demo styles shown under a demo: the rules of the book's demo stylesheets that select a class the demo's source
//! uses. Readers copying a demo see exactly the CSS it needs.

use std::sync::LazyLock;

/// The book's demo stylesheets (`style/demos/*.scss`), in the order `main.scss` includes them. Every demo
/// stylesheet `main.scss` includes must be listed (checked by a test).
const STYLESHEETS: &[&str] = &[
    include_str!("../../style/demos/_shared.scss"),
    include_str!("../../style/demos/_getting-started.scss"),
    include_str!("../../style/demos/_interactions.scss"),
    include_str!("../../style/demos/_dnd.scss"),
    include_str!("../../style/demos/_focus.scss"),
    include_str!("../../style/demos/_overlays.scss"),
    include_str!("../../style/demos/_transitions.scss"),
    include_str!("../../style/demos/_input.scss"),
    include_str!("../../style/demos/_toggles.scss"),
    include_str!("../../style/demos/_calendar.scss"),
    include_str!("../../style/demos/_date-field.scss"),
    include_str!("../../style/demos/_combobox.scss"),
    include_str!("../../style/demos/_listbox-select.scss"),
    include_str!("../../style/demos/_color-atoms.scss"),
    include_str!("../../style/demos/_data-display.scss"),
    include_str!("../../style/demos/_table.scss"),
    include_str!("../../style/demos/_layout.scss"),
    include_str!("../../style/demos/_tabs.scss"),
    include_str!("../../style/demos/_feedback.scss"),
    include_str!("../../style/demos/_modal-atoms.scss"),
    include_str!("../../style/demos/_navigation.scss"),
    include_str!("../../style/demos/_general.scss"),
];

/// Top-level rules (with the comments directly above them) of all demo stylesheets.
static RULES: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    STYLESHEETS
        .iter()
        .flat_map(|scss| top_level_rules(scss))
        .collect()
});

/// The demo style rules for elements with a class used in `source`, in stylesheet order. Empty if there are none.
///
/// A rule belongs to an element class when one of its selectors starts with it: `.demo-item.selected` belongs to
/// `demo-item`, not to `selected`.
pub(super) fn styles_for(source: &str) -> String {
    let classes = kebab_case_words(source);
    let used = |rule: &str| {
        primary_classes(rule)
            .iter()
            .any(|class| classes.contains(class))
    };
    let selected: Vec<&str> = RULES.iter().copied().filter(|rule| used(rule)).collect();
    // Keyframes animating the selected rules.
    let keyframes = |rule: &str| {
        keyframes_name(rule).is_some_and(|name| {
            selected
                .iter()
                .any(|selected| contains_word(selected, name))
        })
    };
    RULES
        .iter()
        .copied()
        .filter(|rule| used(rule) || keyframes(rule))
        .collect::<Vec<_>>()
        .join("\n\n")
}

/// The name of a `@keyframes` rule.
fn keyframes_name(rule: &str) -> Option<&str> {
    let at = rule.find("@keyframes ")?;
    rule[at + "@keyframes ".len()..].split_whitespace().next()
}

/// Whether `text` contains `word`, not as part of a longer identifier.
fn contains_word(text: &str, word: &str) -> bool {
    let is_identifier = |c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_';
    text.match_indices(word).any(|(at, _)| {
        !text[..at].ends_with(is_identifier) && !text[at + word.len()..].starts_with(is_identifier)
    })
}

/// Splits a stylesheet into its top-level rules (`selector { .. }`, `@media .. { .. }`), each including the comments
/// directly above it. Braces in comments and strings are ignored.
fn top_level_rules(scss: &str) -> Vec<&str> {
    let bytes = scss.as_bytes();
    let mut rules = Vec::new();
    let mut depth = 0usize;
    let mut start = 0;
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'/' if bytes.get(i + 1) == Some(&b'/') => {
                i = scss[i..].find('\n').map_or(bytes.len(), |end| i + end);
            }
            b'/' if bytes.get(i + 1) == Some(&b'*') => {
                i = scss[i + 2..]
                    .find("*/")
                    .map_or(bytes.len(), |end| i + 2 + end + 1);
            }
            quote @ (b'"' | b'\'') => {
                i = scss[i + 1..]
                    .find(quote as char)
                    .map_or(bytes.len(), |end| i + 1 + end);
            }
            b'{' => depth += 1,
            b'}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    rules.push(scss[start..=i].trim());
                    start = i + 1;
                }
            }
            b';' if depth == 0 => start = i + 1, // `@use` and other top-level statements.
            _ => {}
        }
        i += 1;
    }
    rules
        .into_iter()
        .map(without_leading_unrelated_comments)
        .collect()
}

/// Drops comments separated from the rule by a blank line (they describe a group of rules, not this one).
fn without_leading_unrelated_comments(rule: &str) -> &str {
    let body_start = rule.find('{').unwrap_or(0);
    rule[..body_start]
        .rfind("\n\n")
        .map_or(rule, |blank| rule[blank..].trim_start())
}

/// Words like `demo-btn` or `demo-field`: lowercase, with at least one dash.
fn kebab_case_words(source: &str) -> Vec<&str> {
    let mut words: Vec<&str> = source
        .split(|c: char| !(c.is_ascii_alphanumeric() || c == '-'))
        .map(|word| word.trim_matches('-'))
        .filter(|word| {
            word.contains('-')
                && word.starts_with(|c: char| c.is_ascii_lowercase())
                && word
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        })
        .collect();
    words.sort_unstable();
    words.dedup();
    words
}

/// The first class of each of the rule's selectors: `.a.b, .c:hover` gives `a` and `c`.
fn primary_classes(rule: &str) -> Vec<&str> {
    let selectors = rule[..rule.find('{').unwrap_or(rule.len())]
        .lines()
        .filter(|line| !line.trim_start().starts_with("//") && !line.trim_start().starts_with("/*"))
        .collect::<Vec<_>>()
        .join(" ");
    selectors
        .split(',')
        .filter_map(|selector| {
            let class = &selector[selector.find('.')? + 1..];
            let end = class
                .find(|c: char| !(c.is_ascii_alphanumeric() || c == '-' || c == '_'))
                .unwrap_or(class.len());
            Some(&rule[rule.find(&class[..end])?..][..end])
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    const SCSS: &str = r#"@use "x";

// Group comment, not about the next rule.

/* Buttons */
.demo-btn { padding: 1em; content: "}"; }
.demo-btn-primary { color: red; }

// The meter.
.demo-meter {
    height: 12px;

    &.complete { background: none; }
}
"#;

    #[test]
    fn splits_top_level_rules_with_their_comments() {
        assert_that!(top_level_rules(SCSS)).is_equal_to(vec![
            "/* Buttons */\n.demo-btn { padding: 1em; content: \"}\"; }",
            ".demo-btn-primary { color: red; }",
            "// The meter.\n.demo-meter {\n    height: 12px;\n\n    &.complete { background: none; }\n}",
        ]);
    }

    #[test]
    fn rules_belong_to_the_first_class_of_their_selectors() {
        assert_that!(primary_classes(
            ".demo-item.selected, .demo-row:hover { x: y; }"
        ))
        .is_equal_to(vec!["demo-item", "demo-row"]);
        assert_that!(primary_classes(
            "// A comment.\n.demo-meter {\n  &.complete { x: y; }\n}"
        ))
        .is_equal_to(vec!["demo-meter"]);
        assert_that!(primary_classes(
            "@media (max-width: 800px) { .a { x: y; } }"
        ))
        .is_equal_to(Vec::<&str>::new());
    }

    #[test]
    fn includes_keyframes_of_selected_rules() {
        assert_that!(keyframes_name(
            "@keyframes demo-fade { from { opacity: 0; } }"
        ))
        .is_equal_to(Some("demo-fade"));
        assert_that!(contains_word(
            ".a { animation: demo-fade 1s; }",
            "demo-fade"
        ))
        .is_true();
        assert_that!(contains_word(
            ".a { animation: demo-fade-out 1s; }",
            "demo-fade"
        ))
        .is_false();
    }

    #[test]
    fn lists_every_demo_stylesheet_of_main_scss() {
        let main = include_str!("../../style/main.scss");
        let demo_stylesheets = main
            .lines()
            .filter(|line| line.starts_with("@use \"./demo"))
            .count();
        assert_that!(STYLESHEETS.len()).is_equal_to(demo_stylesheets);
    }

    #[test]
    fn collects_kebab_case_words() {
        assert_that!(kebab_case_words(
            r#"<div class="demo-btn demo-pressed" on:click=x>"Hi-there"</div>"#
        ))
        .is_equal_to(vec!["demo-btn", "demo-pressed"]);
    }
}

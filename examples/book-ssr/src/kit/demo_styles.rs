//! The demo styles shown under a demo: the rules of the book's demo stylesheets that select a class the demo's source
//! uses. Readers copying a demo see exactly the CSS it needs.

use std::sync::LazyLock;

/// `(name, contents)` of the demo stylesheets `style/demos/_<name>.scss`.
macro_rules! stylesheets {
    ($($name:literal),* $(,)?) => {
        &[$(($name, include_str!(concat!("../../style/demos/_", $name, ".scss")))),*]
    };
}

/// The book's demo stylesheets, one per sidebar group (see "CSS conventions" in `STYLE_GUIDE.md`), in the order
/// `main.scss` includes them (checked by a test).
const STYLESHEETS: &[(&str, &str)] = stylesheets![
    "shared",
    "guides",
    "buttons",
    "fields",
    "pickers",
    "collections",
    "table",
    "date-time",
    "color",
    "overlays",
    "navigation",
    "status",
    "layout",
    "interactions",
    "focus",
    "overlay-behavior",
    "collection-state",
    "drag-and-drop",
    "animation",
    "screen-readers",
    "utilities",
];

/// Top-level rules (with the comments directly above them) of all demo stylesheets.
static RULES: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    STYLESHEETS
        .iter()
        .flat_map(|(_, scss)| top_level_rules(scss))
        .collect()
});

/// The demo style rules for elements with a class used in `source`, in stylesheet order. Empty if there are none.
pub(super) fn styles_for(source: &str) -> String {
    rules_for(&RULES, source).join("\n\n")
}

/// The rules of `rules` that belong to a class used in `source`, and the keyframes they animate with.
///
/// A rule belongs to an element class when one of its selectors starts with it: `.demo-item.selected` belongs to
/// `demo-item`, not to `selected`. An `@media` rule belongs to the classes of the rules inside it, and an `@media`
/// rule of keyframes (e.g. their reduced-motion variants) is shown with the keyframes.
fn rules_for<'a>(rules: &[&'a str], source: &str) -> Vec<&'a str> {
    let classes = kebab_case_words(source);
    let used = |rule: &str| {
        primary_classes(rule)
            .iter()
            .any(|class| classes.binary_search(class).is_ok())
    };
    let selected: Vec<&str> = rules.iter().copied().filter(|rule| used(rule)).collect();
    let animates_selected = |rule: &str| {
        keyframes_names(rule)
            .iter()
            .any(|name| selected.iter().any(|selected| contains_word(selected, name)))
    };
    rules
        .iter()
        .copied()
        .filter(|rule| used(rule) || animates_selected(rule))
        .collect()
}

/// The names of the `@keyframes` rules in `rule`, also inside an `@media` rule.
fn keyframes_names(rule: &str) -> Vec<&str> {
    rule.match_indices("@keyframes")
        .map(|(at, keyword)| {
            let name = rule[at + keyword.len()..].trim_start();
            &name[..name.find(|c| !is_identifier(c)).unwrap_or(name.len())]
        })
        .filter(|name| !name.is_empty())
        .collect()
}

fn is_identifier(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '-' || c == '_'
}

/// Whether `text` contains `word`, not as part of a longer identifier.
fn contains_word(text: &str, word: &str) -> bool {
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

/// `rule` without the comments above it.
fn without_leading_comments(mut rule: &str) -> &str {
    loop {
        rule = rule.trim_start();
        let end = if rule.starts_with("//") {
            rule.find('\n')
        } else if rule.starts_with("/*") {
            rule.find("*/").map(|end| end + 2)
        } else {
            return rule;
        };
        rule = end.map_or("", |end| &rule[end..]);
    }
}

/// Words like `demo-btn` or `demo-field`: lowercase, with at least one dash. Sorted and deduplicated.
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

/// The first class of each of the rule's selectors: `.a.b, .c:hover` gives `a` and `c`. For an `@media` (or other
/// at-rule wrapping rules), the primary classes of the rules inside; keyframes have none.
fn primary_classes(rule: &str) -> Vec<&str> {
    let rule = without_leading_comments(rule);
    let Some(body_start) = rule.find('{') else {
        return Vec::new();
    };
    let prelude = rule[..body_start].trim();
    if prelude.starts_with("@keyframes") {
        return Vec::new();
    }
    if prelude.starts_with('@') {
        let body = &rule[body_start + 1..rule.rfind('}').unwrap_or(rule.len())];
        return top_level_rules(body)
            .into_iter()
            .flat_map(primary_classes)
            .collect();
    }
    prelude
        .split(',')
        .filter_map(|selector| {
            let class = &selector[selector.find('.')? + 1..];
            Some(&class[..class.find(|c| !is_identifier(c)).unwrap_or(class.len())])
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

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
            "// A comment, with. dots, and commas.\n/* More. */\n.demo-meter {\n  &.complete { x: y; }\n}"
        ))
        .is_equal_to(vec!["demo-meter"]);
        assert_that!(primary_classes(
            "[aria-selected=\"true\"] > .demo-day, input.demo-item { x: y; }"
        ))
        .is_equal_to(vec!["demo-day", "demo-item"]);
    }

    #[test]
    fn media_rules_belong_to_the_classes_inside() {
        assert_that!(primary_classes(
            "@media (max-width: 800px) {\n    .demo-a { x: y; }\n\n    // B.\n    .demo-b.c { x: y; }\n}"
        ))
        .is_equal_to(vec!["demo-a", "demo-b"]);
        assert_that!(primary_classes(
            "@media (prefers-reduced-motion: reduce) { @keyframes demo-fade { from { opacity: 0; } } }"
        ))
        .is_equal_to(Vec::<&str>::new());
        assert_that!(primary_classes("@keyframes demo-fade { from { opacity: 0; } }"))
            .is_equal_to(Vec::<&str>::new());
    }

    #[test]
    fn finds_every_keyframes_rule() {
        assert_that!(keyframes_names(
            "@keyframes demo-fade{ from { opacity: 0; } }"
        ))
        .is_equal_to(vec!["demo-fade"]);
        assert_that!(keyframes_names(
            "@media (prefers-reduced-motion: reduce) {\n    @keyframes demo-in { from { opacity: 0; } }\n\n    @keyframes demo-out { to { opacity: 0; } }\n}"
        ))
        .is_equal_to(vec!["demo-in", "demo-out"]);
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
    fn selects_the_rules_and_keyframes_of_the_used_classes() {
        let scss = r"
.demo-panel { padding: 1em; }

.demo-panel[data-entering] { animation: demo-in 1s; }

.demo-other { animation: demo-spin 1s; }

@keyframes demo-in { from { opacity: 0; } }

@keyframes demo-spin { to { rotate: 1turn; } }

// Without motion, only fade.
@media (prefers-reduced-motion: reduce) {
    @keyframes demo-in { from { opacity: 0; } }

    @keyframes demo-spin { to { rotate: none; } }
}

@media (max-width: 800px) {
    .demo-panel { padding: 0; }
}

@media (max-width: 800px) {
    .demo-other { padding: 0; }
}
";
        let rules = top_level_rules(scss);
        assert_that!(rules_for(&rules, r#"<div class="demo-panel">"#)).is_equal_to(vec![
            ".demo-panel { padding: 1em; }",
            ".demo-panel[data-entering] { animation: demo-in 1s; }",
            "@keyframes demo-in { from { opacity: 0; } }",
            "// Without motion, only fade.\n@media (prefers-reduced-motion: reduce) {\n    @keyframes demo-in { from { opacity: 0; } }\n\n    @keyframes demo-spin { to { rotate: none; } }\n}",
            "@media (max-width: 800px) {\n    .demo-panel { padding: 0; }\n}",
        ]);
        assert_that!(rules_for(&rules, r#"<div class="unrelated">"#)).is_empty();
    }

    #[test]
    fn lists_the_demo_stylesheets_of_main_scss_in_order() {
        let main = include_str!("../../style/main.scss");
        let included: Vec<&str> = main
            .lines()
            .filter_map(|line| line.strip_prefix("@use \"./demos/")?.strip_suffix("\";"))
            .collect();
        let listed: Vec<&str> = STYLESHEETS.iter().map(|(name, _)| *name).collect();
        assert_that!(listed).is_equal_to(included);
    }

    #[test]
    fn every_stylesheet_starts_with_a_comment_naming_its_group() {
        for (name, scss) in STYLESHEETS {
            assert_that!(scss.starts_with("// "))
                .with_detail_message(*name)
                .is_true();
        }
    }

    /// Every demo style rule belongs to a class that a page or demo uses: rules of removed classes are removed too.
    #[test]
    fn every_rule_belongs_to_a_used_class() {
        fn sources(dir: &Path, into: &mut String) {
            for entry in std::fs::read_dir(dir).expect("readable source directory") {
                let path = entry.expect("readable directory entry").path();
                if path.is_dir() {
                    sources(&path, into);
                } else if path.extension().is_some_and(|extension| extension == "rs") {
                    into.push_str(&std::fs::read_to_string(&path).expect("readable source file"));
                }
            }
        }
        let mut source = String::new();
        sources(
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("src/pages"),
            &mut source,
        );
        let used = kebab_case_words(&source);
        for rule in RULES.iter() {
            let classes = primary_classes(rule);
            assert_that!(
                classes.is_empty() || classes.iter().any(|class| used.binary_search(class).is_ok())
            )
            .with_detail_message(format!("unused demo style rule:\n{rule}"))
            .is_true();
        }
    }

    #[test]
    fn collects_kebab_case_words() {
        assert_that!(kebab_case_words(
            r#"<div class="demo-btn demo-pressed" on:click=x>"Hi-there"</div>"#
        ))
        .is_equal_to(vec!["demo-btn", "demo-pressed"]);
    }
}

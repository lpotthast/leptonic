// Upstream: @internationalized/string/src/LocalizedStringDictionary.ts @ 99e6102368
// Upstream: @internationalized/string/src/LocalizedStringFormatter.ts @ 99e6102368
// Upstream: @internationalized/string-compiler/src/stringCompiler.js @ 99e6102368
//! Localized messages of the hooks (labels, descriptions, announcements), in react-aria's 34
//! locales.
//!
//! Each family's messages are a typed struct ([`TableStrings`], [`GridStrings`], ...) with one
//! method per message, taking its arguments: `strings.ascending_sort(&column)`. Get one for the
//! current locale with [`use_localized_strings`]. Without the `intl-strings` feature, every
//! locale gets the English (en-US) messages.

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - One struct per message bundle with a method per message and typed arguments (`usize` for a
//   plural, `bool` for a select on `true`), generated from the bundles by
//   `scripts/port-intl-strings.py`. React-aria: `useLocalizedStringFormatter(intlMessages)` and
//   `format(key, args)` with an untyped `args` object.
//
// ## DIFFERENT BEHAVIOR
// - Messages are ICU MessageFormat strings formatted at runtime (`{arg}`, `plural` with `=N` and
//   CLDR categories, `#`, `select`, apostrophe quoting). React-aria compiles them to JavaScript
//   functions at build time; the result is the same, except that a count in a plain `{count}`
//   (not `#`) is formatted for the locale ("1,234") as ICU does, where the compiled template
//   prints the raw number. No bundle message does this at the moment.
//
// ## OMITTED FEATURES
// - Apps' own bundles (`LocalizedStringDictionary` with custom strings): override a hook's text
//   through its input (`aria_label`, ...) instead.
//
// =============================================================================

mod bundles;

use std::borrow::Cow;

pub use bundles::*;
use icu_plurals::{PluralCategory, PluralOperands, PluralRules, PluralRulesPreferences};
use leptos::prelude::*;

use super::{
    i18n::{Locale, use_locale},
    number_formatter::{NumberFormatOptions, NumberFormatter},
};

/// The messages of one family in every locale (generated; see the module docs).
#[derive(Debug)]
pub struct Bundle {
    /// The message keys, in upstream's (en-US) order; `en_us` and every locale follow it.
    pub(crate) keys: &'static [&'static str],
    pub(crate) en_us: &'static [&'static str],
    /// Every other locale (none without the `intl-strings` feature).
    pub(crate) other: &'static [(&'static str, &'static [&'static str])],
}

impl Bundle {
    /// The messages for `locale`: its own, else those of another locale with its language, else
    /// en-US (react-aria's `LocalizedStringDictionary.getStringsForLocale`).
    fn messages_for(&self, locale: &Locale) -> &'static [&'static str] {
        let tag = locale.locale_str();
        if tag == "en-US" {
            return self.en_us;
        }
        if let Some((_, messages)) = self.other.iter().find(|(other, _)| *other == tag) {
            return messages;
        }
        let language = locale.language();
        let same_language = |other: &str| other.split('-').next() == Some(language.as_str());
        if language == "en" {
            return self.en_us;
        }
        self.other
            .iter()
            .find(|(other, _)| same_language(other))
            .map_or(self.en_us, |(_, messages)| messages)
    }
}

/// The messages of one bundle, in one locale. Wrapped by the generated per-family structs.
#[derive(Debug, Clone)]
pub struct Strings {
    bundle: &'static Bundle,
    messages: &'static [&'static str],
    locale: Locale,
}

impl PartialEq for Strings {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.bundle, other.bundle) && self.locale == other.locale
    }
}

impl Strings {
    fn new(bundle: &'static Bundle, locale: Locale) -> Self {
        Self {
            bundle,
            messages: bundle.messages_for(&locale),
            locale,
        }
    }

    /// The message with upstream's `key` (without arguments), for code choosing messages by
    /// computed keys (color names). `None` for an unknown key.
    #[must_use]
    pub fn by_key(&self, key: &str) -> Option<String> {
        let index = self.bundle.keys.iter().position(|k| *k == key)?;
        Some(self.format(index, &[]))
    }

    /// Formats message `index` with its arguments.
    pub(crate) fn format(&self, index: usize, args: &[(&str, Arg<'_>)]) -> String {
        let message = self.messages[index];
        match parse(message) {
            Ok(parts) => {
                let mut out = String::new();
                Formatter {
                    locale: &self.locale,
                    args,
                }
                .write(&parts, None, &mut out);
                out
            }
            // The tests parse every message, so this is unreachable.
            Err(error) => {
                let key = self.bundle.keys[index];
                crate::utils::dev_warn!("intl_strings: can't parse {key:?} ({message:?}): {error}");
                message.to_owned()
            }
        }
    }
}

/// A family's typed messages ([`TableStrings`], ...).
pub trait LocalizedStrings: Clone + PartialEq + Send + Sync + 'static {
    /// The family's messages in every locale.
    fn bundle() -> &'static Bundle;
    /// Wraps the messages of one locale.
    fn new(strings: Strings) -> Self;
    /// The messages, e.g. to look one up by its upstream key ([`Strings::by_key`]).
    fn strings(&self) -> &Strings;

    /// The messages for `locale`.
    #[must_use]
    fn for_locale(locale: Locale) -> Self {
        Self::new(Strings::new(Self::bundle(), locale))
    }
}

/// A family's messages in the current locale (from the i18n context), following locale changes
/// (react-aria's `useLocalizedStringFormatter`).
///
/// ```ignore
/// let strings = use_localized_strings::<TableStrings>();
/// let label = Signal::derive(move || strings.read().select_all());
/// ```
pub fn use_localized_strings<S: LocalizedStrings>() -> Memo<S> {
    let locale = use_locale();
    Memo::new(move |_| S::for_locale(locale.get()))
}

/// An argument of a message.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Arg<'a> {
    /// Inserted as is (`{name}`), or selecting an option (`{name, select, ..}`).
    Str(&'a str),
    /// A plural's value (`{count, plural, ..}`); `#` formats it as a number.
    Count(usize),
    /// Selects `true` or `other` (`{flag, select, true {..} other {..}}`).
    Bool(bool),
}

/// A parsed message.
#[derive(Debug, PartialEq)]
enum Part<'m> {
    Text(Cow<'m, str>),
    Arg(&'m str),
    /// The value of the innermost plural, as a number.
    Pound,
    Plural {
        arg: &'m str,
        options: Vec<(Selector<'m>, Vec<Part<'m>>)>,
    },
    Select {
        arg: &'m str,
        options: Vec<(&'m str, Vec<Part<'m>>)>,
    },
}

#[derive(Debug, PartialEq)]
enum Selector<'m> {
    /// `=N`.
    Exact(usize),
    /// A plural category (`zero`, `one`, `two`, `few`, `many`, `other`).
    Category(&'m str),
}

/// Parses an ICU MessageFormat message (the subset react-aria's bundles use).
fn parse(message: &str) -> Result<Vec<Part<'_>>, String> {
    let mut parser = Parser {
        message,
        position: 0,
    };
    let parts = parser.parts(false)?;
    if parser.position < message.len() {
        return Err(format!("unexpected `}}` at {}", parser.position));
    }
    Ok(parts)
}

struct Parser<'m> {
    message: &'m str,
    position: usize,
}

impl<'m> Parser<'m> {
    fn rest(&self) -> &'m str {
        &self.message[self.position..]
    }

    fn peek(&self) -> Option<char> {
        self.rest().chars().next()
    }

    fn skip_whitespace(&mut self) {
        let rest = self.rest();
        self.position += rest.len() - rest.trim_start().len();
    }

    fn expect(&mut self, expected: char) -> Result<(), String> {
        self.skip_whitespace();
        if self.peek() == Some(expected) {
            self.position += expected.len_utf8();
            Ok(())
        } else {
            Err(format!("expected `{expected}` at {}", self.position))
        }
    }

    /// A name or selector: everything up to whitespace or a delimiter.
    fn word(&mut self) -> Result<&'m str, String> {
        self.skip_whitespace();
        let rest = self.rest();
        let length = rest
            .find(|c: char| c.is_whitespace() || matches!(c, '{' | '}' | ','))
            .unwrap_or(rest.len());
        if length == 0 {
            return Err(format!("expected a name at {}", self.position));
        }
        self.position += length;
        Ok(&rest[..length])
    }

    /// Message parts up to the end or a `}` closing an option (`in_plural`: `#` is the value).
    fn parts(&mut self, in_plural: bool) -> Result<Vec<Part<'m>>, String> {
        let mut parts = Vec::new();
        let mut text = String::new();
        while let Some(c) = self.peek() {
            match c {
                '}' => break,
                '{' => {
                    if !text.is_empty() {
                        parts.push(Part::Text(Cow::Owned(std::mem::take(&mut text))));
                    }
                    self.position += 1;
                    parts.push(self.argument()?);
                }
                '#' if in_plural => {
                    if !text.is_empty() {
                        parts.push(Part::Text(Cow::Owned(std::mem::take(&mut text))));
                    }
                    self.position += 1;
                    parts.push(Part::Pound);
                }
                '\'' => self.apostrophe(in_plural, &mut text),
                c => {
                    text.push(c);
                    self.position += c.len_utf8();
                }
            }
        }
        if !text.is_empty() {
            parts.push(Part::Text(Cow::Owned(text)));
        }
        Ok(parts)
    }

    /// ICU's apostrophe rules: `''` is one apostrophe; an apostrophe before a special character
    /// (`{`, `}`, `#` in a plural) quotes text up to the next single apostrophe; any other one is
    /// literal.
    fn apostrophe(&mut self, in_plural: bool, text: &mut String) {
        let next = self.rest()[1..].chars().next();
        let quotes = matches!(next, Some('{' | '}')) || (in_plural && next == Some('#'));
        match next {
            Some('\'') => {
                text.push('\'');
                self.position += 2;
            }
            _ if quotes => {
                self.position += 1;
                while let Some(c) = self.peek() {
                    self.position += c.len_utf8();
                    if c == '\'' {
                        if self.peek() == Some('\'') {
                            text.push('\'');
                            self.position += 1;
                        } else {
                            break;
                        }
                    } else {
                        text.push(c);
                    }
                }
            }
            _ => {
                text.push('\'');
                self.position += 1;
            }
        }
    }

    /// After a `{`: `name}`, `name, plural, ..}` or `name, select, ..}`.
    fn argument(&mut self) -> Result<Part<'m>, String> {
        let arg = self.word()?;
        self.skip_whitespace();
        if self.peek() == Some('}') {
            self.position += 1;
            return Ok(Part::Arg(arg));
        }
        self.expect(',')?;
        let kind = self.word()?;
        self.expect(',')?;
        let part = match kind {
            "plural" => {
                let mut options = Vec::new();
                while let Some((selector, parts)) = self.option(true)? {
                    let selector = match selector.strip_prefix('=') {
                        Some(number) => Selector::Exact(
                            number
                                .parse()
                                .map_err(|_| format!("invalid plural selector {selector:?}"))?,
                        ),
                        None => Selector::Category(selector),
                    };
                    options.push((selector, parts));
                }
                Part::Plural { arg, options }
            }
            "select" => {
                // `#` is the count only in a plural's own options, not in a select nested in them.
                let mut options = Vec::new();
                while let Some(option) = self.option(false)? {
                    options.push(option);
                }
                Part::Select { arg, options }
            }
            kind => return Err(format!("unsupported argument type {kind:?}")),
        };
        self.expect('}')?;
        Ok(part)
    }

    /// `selector {parts}`, or `None` at the argument's closing `}`.
    #[allow(clippy::type_complexity)]
    fn option(&mut self, in_plural: bool) -> Result<Option<(&'m str, Vec<Part<'m>>)>, String> {
        self.skip_whitespace();
        if self.peek() == Some('}') {
            return Ok(None);
        }
        let selector = self.word()?;
        self.expect('{')?;
        let parts = self.parts(in_plural)?;
        self.expect('}')?;
        Ok(Some((selector, parts)))
    }
}

struct Formatter<'a> {
    locale: &'a Locale,
    args: &'a [(&'a str, Arg<'a>)],
}

impl Formatter<'_> {
    fn arg(&self, name: &str) -> Option<Arg<'_>> {
        let arg = self.args.iter().find(|(arg, _)| *arg == name).map(|(_, arg)| *arg);
        if arg.is_none() {
            crate::utils::dev_warn!("intl_strings: no argument {name:?}");
        }
        arg
    }

    /// Appends `parts` to `out`; `count` is the value of the innermost plural.
    fn write(&self, parts: &[Part<'_>], count: Option<usize>, out: &mut String) {
        for part in parts {
            match part {
                Part::Text(text) => out.push_str(text),
                Part::Arg(name) => match self.arg(name) {
                    Some(Arg::Str(value)) => out.push_str(value),
                    Some(Arg::Count(value)) => out.push_str(&self.number(value)),
                    Some(Arg::Bool(value)) => out.push_str(if value { "true" } else { "false" }),
                    None => {}
                },
                Part::Pound => {
                    if let Some(count) = count {
                        out.push_str(&self.number(count));
                    }
                }
                Part::Plural { arg, options } => {
                    let value = match self.arg(arg) {
                        Some(Arg::Count(value)) => value,
                        _ => 0,
                    };
                    let category = self.category(value);
                    let chosen = options
                        .iter()
                        .find(|(selector, _)| *selector == Selector::Exact(value))
                        .or_else(|| {
                            options
                                .iter()
                                .find(|(selector, _)| *selector == Selector::Category(category))
                        })
                        .or_else(|| {
                            options
                                .iter()
                                .find(|(selector, _)| *selector == Selector::Category("other"))
                        });
                    if let Some((_, parts)) = chosen {
                        self.write(parts, Some(value), out);
                    }
                }
                Part::Select { arg, options } => {
                    let value = match self.arg(arg) {
                        Some(Arg::Str(value)) => value,
                        Some(Arg::Bool(true)) => "true",
                        Some(Arg::Bool(false)) => "false",
                        Some(Arg::Count(_)) | None => "",
                    };
                    let chosen = options
                        .iter()
                        .find(|(option, _)| *option == value)
                        .or_else(|| options.iter().find(|(option, _)| *option == "other"));
                    if let Some((_, parts)) = chosen {
                        self.write(parts, count, out);
                    }
                }
            }
        }
    }

    fn number(&self, value: usize) -> String {
        NumberFormatter::new(self.locale, NumberFormatOptions::default()).format(value)
    }

    /// The CLDR plural category of `value` in the locale.
    fn category(&self, value: usize) -> &'static str {
        let rules =
            PluralRules::try_new_cardinal(PluralRulesPreferences::from(self.locale.icu_locale()));
        match rules.map(|rules| rules.category_for(PluralOperands::from(value))) {
            Ok(PluralCategory::Zero) => "zero",
            Ok(PluralCategory::One) => "one",
            Ok(PluralCategory::Two) => "two",
            Ok(PluralCategory::Few) => "few",
            Ok(PluralCategory::Many) => "many",
            Ok(PluralCategory::Other) | Err(_) => "other",
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use assertr::prelude::*;

    use super::*;

    fn format(message: &str, locale: &str, args: &[(&str, Arg<'_>)]) -> String {
        let parts = parse(message).expect("parses");
        let mut out = String::new();
        Formatter {
            locale: &locale.parse().expect("a locale"),
            args,
        }
        .write(&parts, None, &mut out);
        out
    }

    /// The argument names a message uses (recursively).
    fn arg_names<'m>(parts: &[Part<'m>], names: &mut BTreeSet<&'m str>) {
        for part in parts {
            match part {
                Part::Arg(name) => {
                    names.insert(name);
                }
                Part::Plural { arg, options } => {
                    names.insert(arg);
                    for (_, parts) in options {
                        arg_names(parts, names);
                    }
                }
                Part::Select { arg, options } => {
                    names.insert(arg);
                    for (_, parts) in options {
                        arg_names(parts, names);
                    }
                }
                Part::Text(_) | Part::Pound => {}
            }
        }
    }

    #[test]
    fn every_message_parses_and_uses_the_arguments_of_en_us() {
        for bundle in bundles::all() {
            for (index, key) in bundle.keys.iter().enumerate() {
                let mut expected = BTreeSet::new();
                arg_names(&parse(bundle.en_us[index]).expect("en-US parses"), &mut expected);
                for (locale, messages) in bundle.other {
                    let message = messages[index];
                    let parts = parse(message)
                        .unwrap_or_else(|e| panic!("{locale} {key}: {e} in {message:?}"));
                    let mut names = BTreeSet::new();
                    arg_names(&parts, &mut names);
                    assert_that!(names)
                        .with_detail_message(format!("{locale} {key}"))
                        .is_equal_to(expected.clone());
                }
            }
        }
    }

    #[test]
    fn plurals_select_exact_values_then_categories() {
        let message = "{count, plural, =0 {No items selected} one {# item selected} other {# items selected}}.";
        assert_that!(format(message, "en-US", &[("count", Arg::Count(0))]))
            .is_equal_to("No items selected.".to_owned());
        assert_that!(format(message, "en-US", &[("count", Arg::Count(1))]))
            .is_equal_to("1 item selected.".to_owned());
        assert_that!(format(message, "en-US", &[("count", Arg::Count(1234))]))
            .is_equal_to("1,234 items selected.".to_owned());
    }

    #[test]
    fn selects_nest_plurals_and_arguments() {
        let message = "{isGroupChange, select, true {Entered group {groupTitle}, with {groupCount, plural, one {# option} other {# options}}. } other {}}{optionText}{isSelected, select, true {, selected} other {}}";
        let args = |group: bool, selected: bool| {
            format(
                message,
                "en-US",
                &[
                    ("isGroupChange", Arg::Bool(group)),
                    ("groupTitle", Arg::Str("Fruit")),
                    ("groupCount", Arg::Count(3)),
                    ("optionText", Arg::Str("Apple")),
                    ("isSelected", Arg::Bool(selected)),
                ],
            )
        };
        assert_that!(args(true, true))
            .is_equal_to("Entered group Fruit, with 3 options. Apple, selected".to_owned());
        assert_that!(args(false, false)).is_equal_to("Apple".to_owned());
    }

    #[test]
    fn pound_is_the_number_only_directly_in_a_plural() {
        // ICU (`MessagePattern`) and @formatjs: `#` is the count in a plural's own options; in a
        // select nested in them it is text, and an apostrophe before it is no quote.
        let message = "{n, plural, other {# {kind, select, a {#a} other {'#}}}}";
        let format_kind = |kind| {
            format(
                message,
                "en-US",
                &[("n", Arg::Count(2)), ("kind", Arg::Str(kind))],
            )
        };
        assert_that!(format_kind("a")).is_equal_to("2 #a".to_owned());
        assert_that!(format_kind("b")).is_equal_to("2 '#".to_owned());
    }

    #[test]
    fn apostrophes_follow_icu() {
        assert_that!(format("l'élément", "fr-FR", &[])).is_equal_to("l'élément".to_owned());
        assert_that!(format("it''s", "en-US", &[])).is_equal_to("it's".to_owned());
        assert_that!(format("'{literal}' {x}", "en-US", &[("x", Arg::Str("arg"))]))
            .is_equal_to("{literal} arg".to_owned());
    }

    #[cfg(feature = "intl-strings")]
    #[test]
    fn locales_fall_back_to_their_language_then_english() {
        let strings = |locale: &str| TableStrings::for_locale(locale.parse().expect("a locale"));
        assert_that!(strings("en-US").select_all()).is_equal_to("Select All".to_owned());
        assert_that!(strings("en-GB").select_all()).is_equal_to("Select All".to_owned());
        assert_that!(strings("xx").select_all()).is_equal_to("Select All".to_owned());
        assert_that!(strings("de-DE").select_all()).is_equal_to("Alles auswählen".to_owned());
        assert_that!(strings("de-AT").select_all()).is_equal_to("Alles auswählen".to_owned());
    }

    #[cfg(feature = "intl-strings")]
    #[test]
    fn plural_categories_follow_the_locale() {
        // Polish: 2 is "few", 5 is "many".
        let strings = GridStrings::for_locale("pl-PL".parse().expect("a locale"));
        assert_that!(strings.selected_count(2)).is_not_equal_to(strings.selected_count(5));
        assert_that!(strings.selected_count(2).as_str()).contains("2");
    }
}

// Upstream: react-stately/src/datepicker/placeholders.ts @ 99e6102368
//! The placeholders of empty segments: the locale's letters for year, month and day (as
//! browsers' `<input type="date">`), the formatted value for era and day period, dashes for
//! times.

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The placeholder of a segment is looked up by `DateSegmentType` and our `Locale` (react-aria:
//   by type string and locale string).
//
// =============================================================================

use super::types::DateSegmentType;
use crate::utils::i18n::Locale;

/// Year, month and day placeholders by locale (Chrome's and Firefox's).
const PLACEHOLDERS: &[(&str, [&str; 3])] = &[
    ("ach", ["mwaka", "dwe", "nino"]),
    ("af", ["jjjj", "mm", "dd"]),
    ("am", ["ዓዓዓዓ", "ሚሜ", "ቀቀ"]),
    ("an", ["aaaa", "mm", "dd"]),
    ("ar", ["سنة", "شهر", "يوم"]),
    ("ast", ["aaaa", "mm", "dd"]),
    ("az", ["iiii", "aa", "gg"]),
    ("be", ["гггг", "мм", "дд"]),
    ("bg", ["гггг", "мм", "дд"]),
    ("bn", ["yyyy", "মিমি", "dd"]),
    ("br", ["bbbb", "mm", "dd"]),
    ("bs", ["gggg", "mm", "dd"]),
    ("ca", ["aaaa", "mm", "dd"]),
    ("cak", ["jjjj", "ii", "q'q'"]),
    ("ckb", ["ساڵ", "مانگ", "ڕۆژ"]),
    ("cs", ["rrrr", "mm", "dd"]),
    ("cy", ["bbbb", "mm", "dd"]),
    ("da", ["åååå", "mm", "dd"]),
    ("de", ["jjjj", "mm", "tt"]),
    ("dsb", ["llll", "mm", "źź"]),
    ("el", ["εεεε", "μμ", "ηη"]),
    ("en", ["yyyy", "mm", "dd"]),
    ("eo", ["jjjj", "mm", "tt"]),
    ("es", ["aaaa", "mm", "dd"]),
    ("et", ["aaaa", "kk", "pp"]),
    ("eu", ["uuuu", "hh", "ee"]),
    ("fa", ["سال", "ماه", "روز"]),
    ("ff", ["hhhh", "ll", "ññ"]),
    ("fi", ["vvvv", "kk", "pp"]),
    ("fr", ["aaaa", "mm", "jj"]),
    ("fy", ["jjjj", "mm", "dd"]),
    ("ga", ["bbbb", "mm", "ll"]),
    ("gd", ["bbbb", "mm", "ll"]),
    ("gl", ["aaaa", "mm", "dd"]),
    ("he", ["שנה", "חודש", "יום"]),
    ("hr", ["gggg", "mm", "dd"]),
    ("hsb", ["llll", "mm", "dd"]),
    ("hu", ["éééé", "hh", "nn"]),
    ("ia", ["aaaa", "mm", "dd"]),
    ("id", ["tttt", "bb", "hh"]),
    ("is", ["áááá", "mm", "dd"]),
    ("it", ["aaaa", "mm", "gg"]),
    ("ja", ["年", "月", "日"]),
    ("ka", ["წწწწ", "თთ", "რრ"]),
    ("kk", ["жжжж", "аа", "кк"]),
    ("kn", ["ವವವವ", "ಮಿಮೀ", "ದಿದಿ"]),
    ("ko", ["연도", "월", "일"]),
    ("lb", ["jjjj", "mm", "dd"]),
    ("lo", ["ປປປປ", "ດດ", "ວວ"]),
    ("lt", ["mmmm", "mm", "dd"]),
    ("lv", ["gggg", "mm", "dd"]),
    ("meh", ["aaaa", "mm", "dd"]),
    ("ml", ["വർഷം", "മാസം", "തീയതി"]),
    ("ms", ["tttt", "mm", "hh"]),
    ("nb", ["åååå", "mm", "dd"]),
    ("nl", ["jjjj", "mm", "dd"]),
    ("nn", ["åååå", "mm", "dd"]),
    ("no", ["åååå", "mm", "dd"]),
    ("oc", ["aaaa", "mm", "jj"]),
    ("pl", ["rrrr", "mm", "dd"]),
    ("pt", ["aaaa", "mm", "dd"]),
    ("rm", ["oooo", "mm", "dd"]),
    ("ro", ["aaaa", "ll", "zz"]),
    ("ru", ["гггг", "мм", "дд"]),
    ("sc", ["aaaa", "mm", "dd"]),
    ("scn", ["aaaa", "mm", "jj"]),
    ("sk", ["rrrr", "mm", "dd"]),
    ("sl", ["llll", "mm", "dd"]),
    ("sr", ["гггг", "мм", "дд"]),
    ("sr-Latn", ["gggg", "mm", "dd"]),
    ("sv", ["åååå", "mm", "dd"]),
    ("szl", ["rrrr", "mm", "dd"]),
    ("tg", ["сссс", "мм", "рр"]),
    ("th", ["ปปปป", "ดด", "วว"]),
    ("tr", ["yyyy", "aa", "gg"]),
    ("uk", ["рррр", "мм", "дд"]),
    ("zh-CN", ["年", "月", "日"]),
    ("zh-TW", ["年", "月", "日"]),
];

fn lookup(locale: &str) -> Option<[&'static str; 3]> {
    PLACEHOLDERS
        .iter()
        .find(|(id, _)| *id == locale)
        .map(|(_, placeholders)| *placeholders)
}

/// The placeholder of a segment of type `kind` whose formatted value is `value`.
pub(crate) fn placeholder(kind: DateSegmentType, value: &str, locale: &Locale) -> String {
    let index = match kind {
        DateSegmentType::Era | DateSegmentType::DayPeriod => return value.to_owned(),
        DateSegmentType::Year => 0,
        DateSegmentType::Month => 1,
        DateSegmentType::Day => 2,
        _ => return "––".to_owned(),
    };
    let language = locale.language();
    let script = locale
        .icu_locale()
        .id
        .script
        .map(|script| format!("{language}-{script}"));
    let region = locale.region().map(|region| format!("{language}-{region}"));
    let placeholders = [script, region, Some(language)]
        .into_iter()
        .flatten()
        .find_map(|id| lookup(&id))
        .or_else(|| lookup("en"))
        .unwrap_or(["yyyy", "mm", "dd"]);
    placeholders[index].to_owned()
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    fn locale(id: &str) -> Locale {
        id.parse().expect("a locale")
    }

    #[test]
    fn looks_placeholders_up_by_locale() {
        assert_that!(placeholder(DateSegmentType::Year, "2024", &locale("en-US")))
            .is_equal_to("yyyy".to_owned());
        assert_that!(placeholder(DateSegmentType::Day, "5", &locale("de-DE")))
            .is_equal_to("tt".to_owned());
        assert_that!(placeholder(DateSegmentType::Month, "5", &locale("zh-CN")))
            .is_equal_to("月".to_owned());
        assert_that!(placeholder(
            DateSegmentType::Year,
            "2024",
            &locale("sr-Latn")
        ))
        .is_equal_to("gggg".to_owned());
        assert_that!(placeholder(DateSegmentType::Year, "2024", &locale("xx")))
            .is_equal_to("yyyy".to_owned());
        assert_that!(placeholder(
            DateSegmentType::DayPeriod,
            "PM",
            &locale("en-US")
        ))
        .is_equal_to("PM".to_owned());
        assert_that!(placeholder(DateSegmentType::Minute, "05", &locale("en-US")))
            .is_equal_to("––".to_owned());
    }
}

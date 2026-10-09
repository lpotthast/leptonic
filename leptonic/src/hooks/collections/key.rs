// Upstream: react-aria/src/collections/BaseCollection.ts @ 99e6102368
use std::{fmt, sync::Arc};

/// Identifies an item (or section, header, ...) in a [`Collection`](super::Collection).
///
/// A `Key` is an opaque, cheap-to-clone identifier. Create it from your own ids: integers and
/// strings convert with `Key::from` / `.into()`; for your own types, implement `From<T> for Key`
/// (or [`SelectionValue`], e.g. with [`selection_value!`](crate::selection_value)). Integer keys
/// of different integer types are equal when their values are (`Key::from(1_u8)` is
/// `Key::from(1_i64)`); keys created from different kinds of values never are (`Key::from(1)` is
/// not `Key::from("1")`).
///
/// Collection hooks use this one concrete key type instead of being generic over your item type:
/// they only need identity and order, and a concrete type keeps the large hook bodies from being
/// compiled once per item type. Atoms and components stay typed: they take your values and hand
/// your values back in callbacks.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Key(Repr);

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Repr {
    /// Every integer type's values (`u64` and `i64` included).
    Int(i128),
    Str(Arc<str>),
    /// A generated cell key: the row's key and the column index.
    Cell(Arc<(Key, usize)>),
    /// A key generated for structural nodes (table header rows, placeholders, ...); never equal to
    /// a key created from user values.
    Generated(&'static str, usize),
}

impl Key {
    /// The key of the cell in column `column` of the row `row` (keys of grid cells are generated
    /// from their position).
    pub fn cell(row: &Key, column: usize) -> Self {
        Self(Repr::Cell(Arc::new((row.clone(), column))))
    }

    /// A key for a generated structural node: `kind` names the kind of node, `index` tells nodes
    /// of the same kind apart.
    pub(crate) fn generated(kind: &'static str, index: usize) -> Self {
        Self(Repr::Generated(kind, index))
    }

    /// The key as a string slice, if it is a string key.
    pub fn as_str(&self) -> Option<&str> {
        match &self.0 {
            Repr::Str(s) => Some(s),
            Repr::Int(_) | Repr::Cell(_) | Repr::Generated(..) => None,
        }
    }

    /// The key as an integer, if it is an integer key in the range of `i64`.
    pub fn as_i64(&self) -> Option<i64> {
        self.as_integer().and_then(|i| i64::try_from(i).ok())
    }

    /// The key as an integer, if it is an integer key.
    fn as_integer(&self) -> Option<i128> {
        match self.0 {
            Repr::Int(i) => Some(i),
            Repr::Str(_) | Repr::Cell(_) | Repr::Generated(..) => None,
        }
    }
}

impl Key {
    /// The key as part of an element id: different keys always give different fragments
    /// (`Key::from(1)` and `Key::from("1")` too, which [`Display`](fmt::Display) prints alike),
    /// and fragments contain no whitespace. String keys keep their text, with whitespace and `%`
    /// escaped (`"Ice cream"` → `Ice%20;cream`); other kinds start with `%` and a letter
    /// (`Key::from(1)` → `%i1`).
    pub fn id_fragment(&self) -> String {
        let mut fragment = String::new();
        self.write_id_fragment(&mut fragment);
        fragment
    }

    fn write_id_fragment(&self, out: &mut String) {
        use std::fmt::Write as _;
        match &self.0 {
            Repr::Str(s) => {
                for c in s.chars() {
                    if c.is_whitespace() || c == '%' {
                        let _ = write!(out, "%{:X};", u32::from(c));
                    } else {
                        out.push(c);
                    }
                }
            }
            Repr::Int(i) => {
                let _ = write!(out, "%i{i}");
            }
            Repr::Cell(cell) => {
                out.push_str("%c");
                cell.0.write_id_fragment(out);
                let _ = write!(out, "%{}", cell.1);
            }
            Repr::Generated(kind, index) => {
                let _ = write!(out, "%g{kind}%{index}");
            }
        }
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Repr::Int(i) => write!(f, "{i}"),
            Repr::Str(s) => f.write_str(s),
            Repr::Cell(cell) => write!(f, "{}-{}", cell.0, cell.1),
            Repr::Generated(kind, index) => write!(f, "{kind}-{index}"),
        }
    }
}

impl fmt::Debug for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Repr::Int(i) => write!(f, "Key({i})"),
            Repr::Str(s) => write!(f, "Key({s:?})"),
            Repr::Cell(cell) => write!(f, "Key::cell({:?}, {})", cell.0, cell.1),
            Repr::Generated(kind, index) => write!(f, "Key::generated({kind:?}, {index})"),
        }
    }
}

impl From<&str> for Key {
    fn from(s: &str) -> Self {
        Self(Repr::Str(Arc::from(s)))
    }
}

impl From<String> for Key {
    fn from(s: String) -> Self {
        Self(Repr::Str(Arc::from(s)))
    }
}

impl From<&String> for Key {
    fn from(s: &String) -> Self {
        Self(Repr::Str(Arc::from(s.as_str())))
    }
}

impl From<Arc<str>> for Key {
    fn from(s: Arc<str>) -> Self {
        Self(Repr::Str(s))
    }
}

impl From<i8> for Key {
    fn from(i: i8) -> Self {
        Self(Repr::Int(i128::from(i)))
    }
}

impl From<i16> for Key {
    fn from(i: i16) -> Self {
        Self(Repr::Int(i128::from(i)))
    }
}

impl From<i32> for Key {
    fn from(i: i32) -> Self {
        Self(Repr::Int(i128::from(i)))
    }
}

impl From<i64> for Key {
    fn from(i: i64) -> Self {
        Self(Repr::Int(i128::from(i)))
    }
}

impl From<isize> for Key {
    fn from(i: isize) -> Self {
        // `isize` has at most 64 bits on every Rust target.
        Self(Repr::Int(i as i128))
    }
}

impl From<u8> for Key {
    fn from(i: u8) -> Self {
        Self(Repr::Int(i128::from(i)))
    }
}

impl From<u16> for Key {
    fn from(i: u16) -> Self {
        Self(Repr::Int(i128::from(i)))
    }
}

impl From<u32> for Key {
    fn from(i: u32) -> Self {
        Self(Repr::Int(i128::from(i)))
    }
}

impl From<u64> for Key {
    fn from(i: u64) -> Self {
        Self(Repr::Int(i128::from(i)))
    }
}

impl From<usize> for Key {
    fn from(i: usize) -> Self {
        // `usize` has at most 64 bits on every Rust target.
        Self(Repr::Int(i as i128))
    }
}

/// A typed value that atoms select (a radio group's or a select's value, ...): the [`Key`] that
/// identifies it in the collection and the key-based hooks, and back. Implemented for `Key`,
/// `String` and the integers; implement it for an enum with [`selection_value!`](crate::selection_value).
pub trait SelectionValue: Clone + Eq + std::hash::Hash + Send + Sync + 'static {
    /// The key identifying this value.
    fn to_key(&self) -> Key;
    /// The value `key` identifies; `None` for keys of other values.
    fn from_key(key: &Key) -> Option<Self>;
}

impl SelectionValue for Key {
    fn to_key(&self) -> Key {
        self.clone()
    }

    fn from_key(key: &Key) -> Option<Self> {
        Some(key.clone())
    }
}

impl SelectionValue for String {
    fn to_key(&self) -> Key {
        Key::from(self.as_str())
    }

    fn from_key(key: &Key) -> Option<Self> {
        key.as_str().map(str::to_owned)
    }
}

macro_rules! selection_value_int {
    ($($t:ty),*) => {$(
        impl SelectionValue for $t {
            fn to_key(&self) -> Key {
                Key::from(*self)
            }

            fn from_key(key: &Key) -> Option<Self> {
                key.as_integer().and_then(|i| <$t>::try_from(i).ok())
            }
        }
    )*};
}
selection_value_int!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

/// Implements [`SelectionValue`] (and `From<T> for Key`, so that items take the values as their
/// `value`/`key`) for a fieldless enum, each variant identified by a string key. The keys are
/// what forms submit; they must be distinct (a compile error otherwise).
///
/// ```ignore
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// enum Size { Small, Large }
///
/// leptonic::selection_value!(Size { Small = "s", Large = "l" });
///
/// view! {
///     <RadioGroup value=size set_value=size>
///         <RadioField value=Size::Small><RadioButton>"Small"</RadioButton></RadioField>
///         <RadioField value=Size::Large><RadioButton>"Large"</RadioButton></RadioField>
///     </RadioGroup>
/// }
/// ```
///
/// ```compile_fail
/// #[derive(Clone, PartialEq, Eq, Hash)]
/// enum Size { Small, Large }
///
/// leptonic::selection_value!(Size { Small = "s", Large = "s" });
/// ```
#[macro_export]
macro_rules! selection_value {
    ($ty:ty { $($variant:ident = $key:literal),+ $(,)? }) => {
        impl $crate::hooks::collections::SelectionValue for $ty {
            fn to_key(&self) -> $crate::hooks::collections::Key {
                $crate::hooks::collections::Key::from(match self {
                    $(Self::$variant => $key,)+
                })
            }

            // Two variants with one key: an error, as the second would never be read back.
            #[deny(unreachable_patterns)]
            fn from_key(key: &$crate::hooks::collections::Key) -> ::core::option::Option<Self> {
                match key.as_str()? {
                    $($key => ::core::option::Option::Some(Self::$variant),)+
                    _ => ::core::option::Option::None,
                }
            }
        }

        impl ::core::convert::From<$ty> for $crate::hooks::collections::Key {
            fn from(value: $ty) -> Self {
                <$ty as $crate::hooks::collections::SelectionValue>::to_key(&value)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use assertr::prelude::*;

    use super::*;

    #[test]
    fn string_and_integer_keys_differ() {
        assert_that!(Key::from(1)).is_not_equal_to(Key::from("1"));
        assert_that!(Key::from("a")).is_equal_to(Key::from(String::from("a")));
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum Size {
        Small,
        Large,
    }

    crate::selection_value!(Size { Small = "s", Large = "l" });

    #[test]
    fn selection_values_round_trip() {
        assert_that!(Size::from_key(&Size::Large.to_key())).is_equal_to(Some(Size::Large));
        assert_that!(Key::from(Size::Small)).is_equal_to(Key::from("s"));
        assert_that!(Size::from_key(&Key::from("m"))).is_none();
        assert_that!(u8::from_key(&Key::from(300))).is_none();
        assert_that!(String::from_key(&Key::from(3))).is_none();
        assert_that!(i32::from_key(&7_i32.to_key())).is_equal_to(Some(7));
        assert_that!(u64::from_key(&u64::MAX.to_key())).is_equal_to(Some(u64::MAX));
        assert_that!(i64::from_key(&u64::MAX.to_key())).is_none();
        assert_that!(Key::from(1_u64)).is_equal_to(Key::from(1_i8));
    }

    #[test]
    fn id_fragments_are_unambiguous_and_without_whitespace() {
        let keys = [
            Key::from(1),
            Key::from("1"),
            Key::from("%i1"),
            Key::from("Ice cream"),
            Key::from("Icecream"),
            Key::from("Ice\tcream"),
            Key::from("Ice%20;cream"),
            Key::cell(&Key::from(1), 2),
            Key::cell(&Key::from("1"), 2),
            Key::generated("row", 1),
        ];
        let fragments: Vec<String> = keys.iter().map(Key::id_fragment).collect();
        let distinct: HashSet<&String> = fragments.iter().collect();
        assert_that!(distinct).has_length(keys.len());
        assert_that!(fragments.iter().any(|f| f.chars().any(char::is_whitespace))).is_false();
        assert_that!(Key::from("Apple").id_fragment()).is_equal_to("Apple".to_owned());
        assert_that!(Key::from("Ice cream").id_fragment()).is_equal_to("Ice%20;cream".to_owned());
        assert_that!(Key::from(7).id_fragment()).is_equal_to("%i7".to_owned());
    }

    #[test]
    fn accessors_and_display() {
        assert_that!(Key::from(7).as_i64()).is_equal_to(Some(7));
        assert_that!(Key::from(7).as_str()).is_none();
        assert_that!(Key::from("apple").as_str()).is_equal_to(Some("apple"));
        assert_that!(Key::from("apple").to_string()).is_equal_to("apple".to_owned());
        assert_that!(Key::from(42).to_string()).is_equal_to("42".to_owned());
    }
}

// Upstream: react-aria/src/collections/BaseCollection.ts @ 99e6102368
use std::{fmt, sync::Arc};

/// Identifies an item (or section, header, ...) in a [`Collection`](super::Collection).
///
/// A `Key` is an opaque, cheap-to-clone identifier. Create it from your own ids: integers and
/// strings convert with `Key::from` / `.into()`; for your own types, implement [`ToKey`].
/// Keys created from different kinds of values never compare equal (`Key::from(1)` is not
/// `Key::from("1")`).
///
/// Collection hooks use this one concrete key type instead of being generic over your item type:
/// they only need identity and order, and a concrete type keeps the large hook bodies from being
/// compiled once per item type. Atoms and components stay typed: they take your values and hand
/// your values back in callbacks.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Key(Repr);

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Repr {
    Int(i64),
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

    /// The key as an integer, if it is an integer key.
    pub fn as_i64(&self) -> Option<i64> {
        match self.0 {
            Repr::Int(i) => Some(i),
            Repr::Str(_) | Repr::Cell(_) | Repr::Generated(..) => None,
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

macro_rules! key_from_int {
    ($($t:ty),*) => {$(
        impl From<$t> for Key {
            fn from(i: $t) -> Self {
                Self(Repr::Int(i64::from(i)))
            }
        }
    )*};
}
key_from_int!(i8, i16, i32, i64, u8, u16, u32);

impl From<usize> for Key {
    /// # Panics
    ///
    /// Panics if `i` does not fit into an `i64`.
    fn from(i: usize) -> Self {
        Self(Repr::Int(
            i64::try_from(i).expect("collection key out of i64 range"),
        ))
    }
}

/// Conversion between a typed value (an enum, an id, ...) and a [`Key`], used by atoms and
/// components that offer typed APIs on top of the key-based collection hooks.
pub trait ToKey: Clone + Send + Sync + 'static {
    /// The key identifying this value.
    fn to_key(&self) -> Key;
}

impl ToKey for String {
    fn to_key(&self) -> Key {
        Key::from(self.as_str())
    }
}

impl ToKey for &'static str {
    fn to_key(&self) -> Key {
        Key::from(*self)
    }
}

macro_rules! to_key_int {
    ($($t:ty),*) => {$(
        impl ToKey for $t {
            fn to_key(&self) -> Key {
                Key::from(*self)
            }
        }
    )*};
}
to_key_int!(i8, i16, i32, i64, u8, u16, u32, usize);

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn string_and_integer_keys_differ() {
        assert_that!(Key::from(1)).is_not_equal_to(Key::from("1"));
        assert_that!(Key::from("a")).is_equal_to(Key::from(String::from("a")));
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

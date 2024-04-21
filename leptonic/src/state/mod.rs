use leptos::Oco;

pub mod menu;
pub mod overlay;
pub mod selection;
pub mod tree;
pub mod collection;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Key {
    String(Oco<'static, str>),
    U64(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionFocusStrategy {
    FocusCollection,
    FocusItem(FocusStrategy),
}

/// TODO: document
/// TODO: move to selection module?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusStrategy {
    First,
    Last,
}

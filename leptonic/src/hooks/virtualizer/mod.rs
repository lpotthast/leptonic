//! Virtualized collections: only the visible items are rendered (react-stately's `virtualizer`
//! and `layout` packages, react-aria's `virtualizer`).

/// The `Layout` trait: where the elements of a virtualized collection go.
pub(crate) mod layout;

/// Where one element goes, and how it is shown.
pub(crate) mod layout_info;

/// Items stacked along an orientation, with fixed or measured sizes.
pub(crate) mod list_layout;

/// The visible rectangle, extended towards where the user scrolls.
pub(crate) mod overscan;

/// Keeping the viewport anchored to an item or an edge while the content changes.
pub(crate) mod scroll_anchor;

/// The scroll view: reports the visible rectangle as it scrolls and resizes.
pub(crate) mod use_scroll_view;

/// An item: positioned at its layout info, measured once rendered.
pub(crate) mod use_virtualizer_item;

/// The virtualizer as reactive state.
pub(crate) mod use_virtualizer_state;

/// The virtualizer: lays out a collection and finds what is visible.
pub(crate) mod virtualizer_state;

pub use layout::*;
pub use layout_info::*;
pub use list_layout::*;
pub use overscan::*;
pub use scroll_anchor::*;
pub use use_scroll_view::*;
pub use use_virtualizer_item::*;
pub use use_virtualizer_state::*;
pub use virtualizer_state::*;

//
// The following deviations apply to all hooks in this module:
//
// ## DIFFERENT BEHAVIOR
//
// - Attribute merging (`mergeProps` equivalent)
//   Rationale: React-aria provides a generic `mergeProps` function that merges
//   two props objects at runtime by dynamically iterating over object keys.
//   In Rust, this is not possible: *Attrs types are defined as tuples with
//   statically known sizes. There is no way to combine two arbitrarily sized
//   tuples at runtime and return a potentially differently sized tuple.
//
//   Leptonic uses explicit `merge_with_*` methods on *Return types instead of
//   a generic merge function. These methods have full compile-time knowledge
//   of both hook returns and can produce a correctly-typed merged result.
//
//   Example:
//   ```rust
//   let button = use_button(...);
//   let menu_trigger = use_menu_trigger(...);
//   let merged = menu_trigger.merge_with_button(button);
//   view! { <button {..merged.attrs}>"Actions"</button> }
//   ```
//
//   React-aria: `mergeProps(buttonProps, menuTriggerProps)` iterates over
//   object keys at runtime, chaining handlers and merging attributes.
//
// ## LEPTOS-SPECIFIC ADAPTATIONS
//
// - Element reference handling
//   Rationale: All hooks use `IntoElementMaybeSignal` pattern instead of React
//   refs. This integrates with leptos_use and handles Send+Sync requirements.
//   React-aria: Uses React's useRef with RefObject<HTMLElement>.
//
// - Callback types
//   Rationale: Uses `Callback<T>` from Leptos instead of React event handlers.
//   React-aria: Uses React's (event: E) => void function signatures.
//

use crate::utils::{merge::MergeWith, styles::Styles};
use std::fmt::Debug;

mod animation;
mod breadcrumbs;
mod button;
mod calendar;
mod color;
mod combobox;
mod datepicker;
mod dialog;
mod disclosure;
mod dnd;
mod focus;
mod form;
mod grid;
mod interactions;
mod link;
mod listbox;
mod menu;
mod merged;
mod meter;
mod modal;
mod overlay;
mod progress;
mod select;
mod selection;
mod separator;
mod slider;
mod spinbutton;
mod table;
mod tabs;
mod tag;
mod toolbar;
mod tooltip;
mod tree;

pub use animation::*;
pub use breadcrumbs::*;
pub use button::*;
pub use calendar::*;
pub use color::*;
pub use combobox::*;
pub use datepicker::*;
pub use dialog::*;
pub use disclosure::*;
pub use dnd::*;
pub use focus::*;
pub use form::*;
pub use grid::*;
pub use interactions::*;
pub use link::*;
pub use listbox::*;
pub use menu::*;
pub use merged::*;
pub use meter::*;
pub use modal::*;
pub use overlay::*;
pub use progress::*;
pub use select::*;
pub use selection::*;
pub use separator::*;
pub use slider::*;
pub use spinbutton::*;
pub use table::*;
pub use tabs::*;
pub use tag::*;
pub use toolbar::*;
pub use tooltip::*;
pub use tree::*;

/// Trait for converting hook `*Props` types into spreadable attribute tuples.
///
/// All `*Props` types returned by hooks implement this trait. Call `.into_attrs()`
/// to convert props into an attribute tuple that can be spread onto elements
/// using Leptos's spreading syntax (`<div {..props.into_attrs()}/>`).
pub trait IntoAttrs: Debug {
    /// The concrete attributes tuple type produced by this conversion.
    type Attrs;

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    fn into_attrs(self) -> Self::Attrs;
}

/// Wrapper for hook props that include styles.
///
/// Does **not** implement [`IntoAttrs`] or any Leptos `Attribute` trait,
/// so it cannot be spread directly. Callers must call [`.into_parts()`](Self::into_parts)
/// to obtain both the spreadable attributes and the [`Styles`] that must be
/// merged with any user-provided styles before being applied via `style=`.
#[derive(Debug)]
pub struct PropsWithStyles<P: IntoAttrs> {
    props: P,
    styles: Styles,
}

impl<P: IntoAttrs> PropsWithStyles<P> {
    pub fn new(props: P, styles: Styles) -> Self {
        Self { props, styles }
    }

    /// Consume self, converting the inner props to spreadable attributes
    /// and returning them alongside the hook's styles.
    // TODO: Deprecate this? Remove and rename into_inner to into_parts.
    pub fn into_parts(self) -> (P::Attrs, Styles) {
        (self.props.into_attrs(), self.styles)
    }

    /// Extract the inner props and styles.
    pub fn into_inner(self) -> (P, Styles) {
        (self.props, self.styles)
    }
}

/// Blanket impl: if `P` can merge with `Other`, then `PropsWithStyles<P>` can too,
/// preserving styles through the merge chain.
impl<P, Other> MergeWith<Other> for PropsWithStyles<P>
where
    P: IntoAttrs + MergeWith<Other>,
    P::Output: IntoAttrs,
{
    type Output = PropsWithStyles<P::Output>;

    fn merge_with(self, other: Other) -> Self::Output {
        let (props, styles) = self.into_inner();
        PropsWithStyles::new(props.merge_with(other), styles)
    }
}

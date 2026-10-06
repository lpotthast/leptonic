//
// The following deviations apply to all hooks in this module:
//
// ## API DIFFERENCES
//
// - API conventions C1–C13 (documentation/hooks-implementation.md, "API Conventions"): typed
//   enums/newtypes instead of strings and unions, `is_*` state flags as signals, `MaybeProp<String>`
//   for user-visible text, state structs with methods, `new(required)` + struct update, typed
//   ARIA values, `Duration`/`Fraction`/`Point` units.
//   Rationale: react-aria's props objects are JavaScript idioms (stringly-typed values,
//   `string | number` unions, optional everything); Rust expresses the same with types.
//
// - Hook-owned state
//   Rationale: hooks create and own their state signals and expose read-only signals plus
//   mutation methods, so callers can't bypass invariants and change callbacks always fire.
//   React-aria: `useControlledState` accepts controlled (`value`) or uncontrolled
//   (`defaultValue`) state.
//
// - Locale and writing direction from the i18n context
//   Rationale: one source of truth; hooks read `use_locale()`/`use_direction()` as react-aria's
//   `useLocale()` does, instead of taking `is_rtl`/`locale` inputs.
//
// ## DIFFERENT BEHAVIOR
//
// - Combining hooks (`mergeProps` equivalent)
//   React-aria merges arbitrary props objects at runtime with `mergeProps`. Spreadable attribute
//   sets are statically typed tuples in Leptos, so there is no generic equivalent. Leptonic
//   combines hooks in two ways:
//
//   1. Input composition (preferred). A hook that configures an element rendered by another hook
//      returns that hook's *input*. Callers add their own settings with struct update syntax:
//      ```rust
//      let menu_trigger = use_menu_trigger(...);
//      let button = use_button(UseButtonInput { on_hover_start: .., ..menu_trigger.button });
//      view! { <button {..button.props.into_parts().0}>"Actions"</button> }
//      ```
//      This mirrors react-aria, where e.g. `menuTriggerProps` are `AriaButtonProps`, and keeps
//      one press/focus state machine per element.
//   2. `MergeWith` for independent hooks spread onto the same element (e.g. press + hover),
//      producing an explicitly typed merged props struct.
//
// ## LEPTOS-SPECIFIC ADAPTATIONS
//
// - Element references
//   Rationale: hooks capture their elements with `CapturedElement` (leptos-element-capture),
//   spread as an attribute, instead of React refs.
//   React-aria: `useRef` with `RefObject<HTMLElement>`.
//
// - Callback types
//   Rationale: Uses `Callback<T>` from Leptos instead of React event handlers.
//   React-aria: Uses React's (event: E) => void function signatures.
//
// - Element ids come from `use_id` (hydration-stable), never from random ids.
//

use std::fmt::Debug;

use crate::utils::{merge::MergeWith, styles::Styles};

mod animation;
mod breadcrumbs;
mod button;
mod calendar;
pub mod collections;
mod color;
mod combobox;
mod datepicker;
mod dialog;
mod disclosure;
mod dnd;
mod focus;
mod form;
mod grid;
mod gridlist;
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
mod separator;
mod slider;
mod spinbutton;
mod table;
mod tabs;
mod tag;
mod toolbar;
mod tooltip;
mod tree;
mod visually_hidden;

pub use animation::*;
pub use breadcrumbs::*;
pub use button::*;
pub use calendar::*;
// The collections' most used names; everything else is used via `hooks::collections`.
pub use collections::{
    Collection, CollectionBuilder, CollectionMemo, DisabledBehavior, FocusStrategy, ItemBuilder,
    ItemElements, ItemLink, Key, ListState, Node, NodeKind, SectionBuilder, SelectionBehavior,
    SelectionMode, SingleSelectListState, ToKey, use_collection, use_list_collection,
    use_list_state, use_single_select_list_state,
};
pub use color::*;
pub use combobox::*;
pub use datepicker::*;
pub use dialog::*;
pub use disclosure::*;
pub use dnd::*;
pub(crate) use focus::use_focus_visible::track_interaction_modality;
pub use focus::*;
pub use form::*;
pub use grid::*;
pub use gridlist::*;
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
pub use separator::*;
pub use slider::*;
pub use spinbutton::*;
pub use table::*;
pub use tabs::*;
pub use tag::*;
pub use toolbar::*;
pub use tooltip::*;
pub use tree::*;
pub use visually_hidden::*;

pub use crate::utils::orientation::Orientation;

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
    /// and returning them alongside the hook's styles. For spreading onto an element; use
    /// [`into_inner`](Self::into_inner) to compose the props further first (e.g. `merge_with`).
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

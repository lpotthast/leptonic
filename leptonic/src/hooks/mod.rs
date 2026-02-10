// =============================================================================
// GLOBAL REACT-ARIA DEVIATIONS
// =============================================================================
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
// =============================================================================

mod breadcrumbs;
mod button;
mod calendar;
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
mod table;
mod tabs;
mod tag;
mod toolbar;
mod tooltip;
mod tree;

pub use breadcrumbs::*;
pub use button::*;
pub use calendar::*;
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
pub use table::*;
pub use tabs::*;
pub use tag::*;
pub use toolbar::*;
pub use tooltip::*;
pub use tree::*;

//
// The following deviations apply to all hooks in this module:
//
// ## API DIFFERENCES
//
// - API conventions C1–C16 (documentation/conventions.md, "API conventions"): typed enums/newtypes instead of
//   strings and unions, `is_*` state flags as signals, `MaybeProp<String>` for user-visible text,
//   `Copy` state structs with methods, struct literals (no constructors on `*Input`s; `Default` +
//   struct update where every field has a meaningful default), typed ARIA values,
//   `Duration`/`Fraction`/`Point` units, generic number values.
//   Rationale: react-aria's props objects are JavaScript idioms (stringly-typed values,
//   `string | number` unions, optional everything); Rust expresses the same with types.
//
// - Hook-owned state (C4)
//   Rationale: a state hook takes `default_*` + `on_*_change`, or a `ValueBinding` to app state
//   (a read signal plus a setter); callers read signals and change the state only through its
//   methods, so they can't bypass invariants and change callbacks always fire.
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
//   2. Independent hooks on the same element (e.g. press + hover) are each spread onto it:
//      `<div {..press_attrs} {..hover_attrs}>`. Leptos attaches every spread's listeners (in
//      spread order). Where two hooks contribute to one id list attribute (`aria-describedby`),
//      the combining hook joins them with `crate::IdRefs`.
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

pub mod animation;
pub mod breadcrumbs;
pub mod button;
pub mod calendar;
pub mod clipboard;
pub mod collections;
pub mod color;
pub mod combobox;
pub mod datepicker;
pub mod dialog;
pub mod disclosure;
pub mod dnd;
pub mod focus;
pub mod form;
pub mod grid;
pub mod gridlist;
pub mod interactions;
pub mod landmark;
pub mod link;
pub mod listbox;
pub mod menu;
pub mod meter;
pub mod modal;
pub mod overlay;
pub mod progress;
pub mod select;
pub mod separator;
pub mod slider;
pub mod spinbutton;
pub mod table;
pub mod tabs;
pub mod tag;
pub mod toast;
pub mod toolbar;
pub mod tooltip;
pub mod tree;
pub mod virtualizer;
pub mod visually_hidden;

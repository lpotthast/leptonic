# Atom Implementation Patterns

Atoms live in `leptonic/src/atoms/` and are leptonic's port of react-aria-components: Leptos components that wrap
hooks into elements, unstyled but carrying a default class and data attributes to style them by. Port them from
`react-aria-components/src/<Name>.tsx`, with an `// Upstream:` header and a deviation block (`porting.md`).

## Single-Element Rule

Every atom renders exactly **one** HTML element: atoms stay composable and consumers control the surrounding DOM.
Anything visible or stylable is its own atom, structural wrappers too: a calendar cell is `CalendarCell` (the
`<td role="gridcell">`) and `CalendarCellButton` (the focusable `<div role="button">` in it), where
react-aria-components renders both from one component (the user).

**Exception: hidden inputs** (the user). An atom may render native inputs that are invisible and never styled, inside
its one element: the visually hidden range inputs of a thumb (`SliderThumb`, `ColorThumb`: screen readers draw their
focus outline on the thumb only when the inputs sit on it), the visually hidden `<input>` of `CheckboxButton`,
`RadioButton` and `SwitchButton`, and `type="hidden"` inputs carrying a field's value for forms (`ColorField`, ...).

## Wrapping Hooks

An atom starts with `crate::metrics::subject!();` (after any `use` items; `hooks-implementation.md`, "Hook
Naming"), calls its hooks, splits their props into attributes and styles (`into_parts()`, "Props and Styles" in
`hooks-implementation.md`) and spreads them onto its element with the default class and the data attributes.
Shortened from `atoms/button.rs`:

```rust
/// Data attributes: `data-pressed`, `data-hovered`, `data-focused`, `data-focus-visible`,
/// `data-disabled`, `data-pending`.
///
/// Default class: `leptonic-Button`.
#[component]
pub fn Button(
    #[prop(into, optional)] on_press: Option<Callback<PressEvent>>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    crate::metrics::subject!();
    let classes = with_default_class("leptonic-Button", classes);
    // The public hook, used as a user would: its part spread in the atom's body.
    let UseButtonReturn { button, is_pressed, .. } =
        use_button(UseButtonInput { is_disabled, on_press, ..UseButtonInput::default() });
    let (button_attrs, button_styles) = button.attrs();

    view! {
        <button {..button_attrs} class=classes style=button_styles.merge(styles) data-pressed=flag(is_pressed)>
            {children()}
        </button>
    }
}
```

## Props Design

Atoms expose react-aria-components' props, adapted to the conventions (`conventions.md`):

- State flags `is_*: Signal<bool>` (C1), user-visible text and id references `MaybeProp<String>` (C2; hand id
  references to hooks with `id_ref_prop`), an element's own `id: Option<Id>`, typed ARIA values (C12).
- **State props (C4):** controlled state is two props, a readable `<x>` (`#[prop(into, optional)]
  Option<Signal<T>>`: a value, any signal or a closure) and a writable `set_<x>: Option<Out<T>>` (`is_<x>` pairs with
  `set_<x>`); uncontrolled `default_<x>` + `on_<x>_change`. The atom builds the hook's `ValueBinding`
  (`atoms/checkbox.rs`):

  ```rust
  let (value, on_change) = ValueBinding::from_state_props(is_selected, set_selected, on_change);
  let state = use_toggle_state(UseToggleStateInput { default_selected, value, on_change, is_read_only });
  ```
- `classes: Classes` and `styles: Styles` for the consumer's styling (merged with the hook's styles).
- Event callbacks as `Option<Callback<Event>>`; settings with no meaning for an atom keep the hook's defaults.
- **Generic atoms:** let a typed prop carry the type parameter, so callers rarely write it. Leptos components can't
  default type parameters (a group without a typed prop is written `<RadioGroup<Key>>`, and leptosfmt mangles such
  tags), and `#[prop(into)] Signal<T>` can't infer `T` (Leptos converts any value into `Signal<Option<_>>`): number
  props are `NumberSignal<T>`/`OptionalNumberSignal<T>`, which convert from `NumberValue`s only
  (`<NumberField value=Some(200)>` infers `i32`).

## Default Classes and Data Attributes

Atoms are unstyled, not class-less. Every atom rendering its own element starts with
`let classes = with_default_class("leptonic-<AtomName>", classes);` (`utils::default_class`) and documents it
("Default class: `leptonic-<AtomName>`."); the caller's classes add to it (react-aria-components: a `className`
replaces it). Atoms without an element of their own (providers, triggers, iterators) have none.

State is exposed as data attributes, as react-aria-components' render props are (`data-hovered`, `data-pressed`,
`data-focus-visible`, `data-selected`, `data-disabled`, ...): boolean ones through `utils::data_attributes::flag`
(`"true"` or absent), listed in the doc comment ("Data attributes: ..."). Inline styles only where the behavior needs
them (thumb positions, visually hidden inputs). Atoms add no design classes or variant tokens of their own: the atom
theme (`atom-theme.md`) styles exactly these classes and attributes.

**One writer per attribute** (the user): `use_focus_ring` renders `data-focus-visible` (also for a within ring), and
`use_button`/`use_link` through it, so elements using them are styleable without wiring (react-aria's hooks render
none). An atom spreading such a hook's attributes doesn't render the attribute again; it renders it only where it
spreads just the hook's handlers, or for another meaning.

## Context Pattern (Complex Atoms)

When an atom comprises cooperating elements (a Slider's track, thumb and output):

1. The root atom creates the state through its hooks and provides a context with what its parts need
   (`SliderContext`, plus `LabelContext`/`FieldContext` for the field parts).
2. The parts read it with `use_context`. A part outside its root is a usage error: warn with `dev_warn!` (debug
   builds) and render nothing, never panic (`atoms/slider.rs`). Parts react-aria-components renders standalone when
   no context provides their props are no error: `Input` and `TextArea` outside a field render a plain input
   (`atoms/input.rs`).

For the missing-parent guard keep the signature `-> impl IntoView` and return `None` or `Some(view)` (`Either` for two
branches) from the body (`atoms/select.rs`): no `AnyView` erasure, while an explicit `-> Option<impl IntoView>`
conflicts with the component macro's `erase_components` transformation in debug builds.

Provide contexts with `<Provider value=..>` around the children (or `scoped_view`), never with `provide_context` in
the component body: a component has no owner of its own, so a context provided there reaches its later siblings too.

An overlay hides contexts of its surroundings from its content (react-aria-components' `clearContexts`): a combo
box's popover passes `ClearContexts(&[clear_context::<LabelContext>, ..])`, so a `Label` inside it isn't the combo
box's. Parts that may sit in such content read their context with `use_clearable_context`.

An atom calls the public hooks and uses their parts exactly as a user's component would (one way, `conventions.md`;
`hooks-implementation.md`, "One way: leptonic composes its public hooks"): an element it renders in its own body
spreads the part (`{..button.attrs()}`, `let (attrs, styles) = button.attrs();`); to change the props first it takes
them with `into_props()`. A part the root hands to a child component travels in the context as the hook returned it
(`label`, `description`, the calendar cell's button, a date field's group, `TagList`, `ColorThumb`'s thumb and inputs,
`LabelContext::label(field.label)`), or `part.map(..)` adding the atom's own attributes (`SelectTrigger`'s element
capture, `InputContext`, a color slider thumb's input styles). The part's body spreads it (`{..ctx.x.attrs()}`): one
element at a time, fresh props per mount, no stored attributes cloned per render. Everything an atom builds its
output with is public (`with_default_class`, `id_ref_prop`, `data_flags`, `scoped_view`, `clear_context`,
`LabelPresence`, `Part`, `Remount`, `signal_attr`/`fixed_attr`/`maybe_attr`, `Spread`), so an atom can be copied as a
starting point for one's own. A part's presence (`Part::observed`,
`is_held()`; `Presence` for an element without a part, the select's and combo box's listbox) replaces watching its
element with an effect (`DisclosureTrigger` naming the panel).

Data attributes: `{..data_flags([("data-x", signal), ..])}` (`utils/data_attributes.rs`), at most 3 signals per
group, the interaction state (`pressed`, `hovered`, `focused`, `focus-visible`) apart from the props; never one
`data-x=flag(..)` per attribute (`performance.md`, R4). A spread after an attribute whose value is a bare path needs
that value braced (`style={styles} {..data_flags(..)}`: `styles {..}` parses as a struct literal).

## Shared Atom Infrastructure

- `Out<T>` (`lib.rs`): the writable half of state props: an `RwSignal`, `WriteSignal`, `StoredValue`, closure or
  `Callback`; `Out::set` writes.
- `ValueBinding::from_state_props` (`utils/value_binding.rs`): `<x>` + `set_<x>` + `on_<x>_change` → the hook's
  binding and change callback.
- `with_default_class`, `flag` (`utils/default_class.rs`, `utils/data_attributes.rs`).
- `LabelContext`, `LabelPresence` (`atoms/field.rs`): whether the `Label` part is rendered decides the element's
  `aria-labelledby`, so don't guess it from the ARIA props: create `LabelPresence::new(aria_label, aria_labelledby)`,
  pass its `has_label` to the hook and attach it with `LabelContext::with_presence`. Until mounted it guesses (the
  server HTML references a likely label), then follows the rendered `Label` (react-aria-components' `useSlot`).
- `FieldContext` (`atoms/field.rs`): one context for a field's `Description` and `FieldError` (C14): the hook's
  `description_props`/`error_message_props` (`SlotProps`) and the validation state. Every field atom provides it.
- `Slot`, `SlotProps`, `use_slot` (`utils/slot_id.rs`): optional elements referenced by id (`part: Part<SlotProps>`);
  `referenced_id: Signal<Option<Id>>` is `Some` only while an element holds the part, so references never dangle (react-aria's `useSlotId`).
- `IdRefs`, `labels` (`utils/id_refs.rs`, `utils/labels.rs`): the one way to join id lists
  (`IdRefs::derive([..])` over `Signal<Option<Id>>`s: absent ones skipped, each id once, constants folded); `labels` is react-aria's `useLabels`.
- `use_description` (`utils/use_description.rs`): a shared, visually hidden description element (client-only).
- `scoped_view`, `ClearContexts`, `clear_context`, `use_clearable_context` (`utils/scoped_context.rs`): a view whose
  contexts reach only its descendants (and which forwards attributes set on the component to its root); hiding a
  context from part of the tree.
- Virtual focus (`utils/virtual_focus.rs`): DOM focus stays on one element (a combo box input) while another (an
  option) is focused through `aria-activedescendant`, with synthetic focus/blur events.
- `OwnerAlive` (`utils/owner_alive.rs`): turns `false` when the owner is cleaned up; check it in deferred callbacks.

## Reference Implementations

| Pattern                      | File                                 |
|------------------------------|--------------------------------------|
| Simple hook wrapper          | `leptonic/src/atoms/button.rs`       |
| State props (`x` + `set_x`)  | `leptonic/src/atoms/checkbox.rs`     |
| Context-based composition    | `leptonic/src/atoms/slider.rs`       |
| Field parts (label, errors)  | `leptonic/src/atoms/field.rs`        |
| Router integration           | `leptonic/src/atoms/link.rs`         |
| Overlay with portal          | `leptonic/src/atoms/popover.rs`      |
| Label detection, value parts | `leptonic/src/atoms/progress_bar.rs` |
| `node_ref`                   | `leptonic/src/atoms/input.rs`        |

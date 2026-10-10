# Atom Implementation Patterns

Atoms live in `leptonic/src/atoms/` and are leptonic's port of react-aria-components: Leptos components that wrap
hooks into elements, unstyled but carrying a default class and data attributes to style them by. Port them from
`react-aria-components/src/<Name>.tsx` (an `// Upstream:` header and a deviation block, as for hooks).

## Single-Element Rule

Every atom renders exactly **one** HTML element. This keeps them composable and gives consumers
full control over the surrounding DOM structure.

**Exception: hidden inputs** (decided by the user, 2026-10-06). An atom may render native inputs that are
invisible and that consumers never style, inside its one element:
- the visually hidden range inputs of a thumb (`SliderThumb`, `ColorThumb`): screen readers draw their
  focus outline on the thumb only when the inputs sit on it, as in react-aria-components;
- the visually hidden `<input>` of `Checkbox`, `Radio` and `Switch`;
- `type="hidden"` inputs carrying a field's value for forms (`ColorField`, `ColorChannelField`, ...).

Anything visible or stylable is its own atom. Structural wrappers too: a calendar cell is two atoms, `CalendarCell` (the `<td role="gridcell">`) and
`CalendarCellButton` (the focusable `<div role="button">` inside it), where react-aria-components renders both from
one component (decided by the user, 2026-10-06).

## Wrapping Hooks

An atom calls its hooks, splits their props into attributes and styles (`into_parts()`, see "Props and Styles" in
`hooks-implementation.md`), and spreads them onto its element with the default class and the data attributes. A
shortened `atoms/button.rs`:

```rust
/// Data attributes: `data-pressed`, `data-hovered`, `data-focused`, `data-focus-visible`,
/// `data-disabled`, `data-pending`.
///
/// Default class: `leptonic-Button`.
#[component]
pub fn Button(
    #[prop(into, optional)] on_press: Option<Callback<PressEvent>>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] is_pending: Signal<bool>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-Button", classes);
    let UseButtonReturn { props, is_disabled, is_pressed, is_hovered, is_focused, .. } =
        use_button(UseButtonInput {
            is_disabled,
            is_pending,
            aria_label,
            on_press,
            ..UseButtonInput::default()
        });
    let (button_attrs, button_styles) = props.into_parts();

    view! {
        <button
            {..button_attrs}
            class=classes
            style=button_styles.merge(styles)
            data-pressed=flag(is_pressed)
            data-hovered=flag(is_hovered)
            data-focused=flag(is_focused)
            data-disabled=flag(is_disabled)
            data-pending=flag(is_pending)
        >
            {children()}
        </button>
    }
}
```

## Props Design

Atoms expose react-aria-components' props, adapted to the conventions (`documentation/conventions.md`):

- State flags `is_*: Signal<bool>` (C1), user-visible text `MaybeProp<String>` (C2), typed ARIA values (C12).
- **State props (C4):** controlled state is two props, a readable `<x>` (`#[prop(into, optional)] Option<Signal<T>>`:
  a value, any signal or a closure) and a writable `set_<x>: Option<Out<T>>` (for `is_<x>`, the setter is
  `set_<x>`: `is_selected` + `set_selected`); uncontrolled `default_<x>` + `on_<x>_change`. The atom turns them into
  the hook's `ValueBinding` with `ValueBinding::from_state_props` (`atoms/checkbox.rs`):

  ```rust
  let (value, on_change) = ValueBinding::from_state_props(is_selected, set_selected, on_change);
  let state = use_toggle_state(UseToggleStateInput { default_selected, value, on_change, is_read_only });
  ```
- `classes: Classes` and `styles: Styles` for the consumer's styling (merged with the hook's styles).
- `node_ref` for atoms whose element callers need (react-aria-components forwards `ref` everywhere; done for
  `Input`/`TextArea`).
- Event callbacks as `Option<Callback<Event>>`; settings with no meaning for an atom keep the hook's defaults.
- **Generic atoms:** let a typed prop carry the type parameter, so callers rarely write it. Leptos components can't
  default type parameters (a group without any typed prop is written `<RadioGroup<Key>>`, and leptosfmt mangles such
  tags), and `#[prop(into)] Signal<T>` can't infer `T` (Leptos converts any value into `Signal<Option<_>>`): number
  props are `NumberSignal<T>`/`OptionalNumberSignal<T>`, which convert from `NumberValue`s only
  (`<NumberField value=Some(200)>` infers `i32`).

## Default Classes, Data Attributes and the Atom Theme

Atoms are unstyled, not class-less. Every atom rendering its own element starts with
`let classes = with_default_class("leptonic-<AtomName>", classes);` (`utils::default_class`) and documents it
("Default class: `leptonic-<AtomName>`."); the caller's classes add to it (react-aria-components: a `className`
replaces the default). Atoms without an element of their own (providers, triggers, iterators) have none.

State is exposed as data attributes, as react-aria-components' render props are: `data-hovered`, `data-pressed`,
`data-focus-visible`, `data-selected`, `data-disabled`, ... Render boolean ones with `utils::data_attributes::flag`
(`"true"` or absent, as react-aria-components) and list them in the doc comment ("Data attributes: ..."). Inline styles only where
the behavior needs them (thumb positions, visually hidden inputs).

The optional atom theme (`leptonic-theme/scss/atoms/`, `documentation/atom-theme.md`) styles exactly these classes
and attributes. Atoms add no design classes or `data-variant`-like tokens of their own.

## Context Pattern (Complex Atoms)

When an atom comprises multiple cooperating elements (e.g., Slider with track, thumb, output):

1. The root atom creates the state through its hooks (`use_slider_state`, `use_slider`) and provides a context with
   what its parts need (`SliderContext`, plus a `LabelContext`/`FieldContext` for the field parts).
2. The parts (`SliderTrack`, `SliderThumb`, `SliderOutput`) read it with `use_context`. A part outside its root is a
   usage error: warn with `dev_warn!` (debug builds) and render nothing, rather than panicking
   (`expect_slider()` in `atoms/slider.rs`). Parts that react-aria-components renders standalone when no context
   provides their props are no usage error: `Input` and `TextArea` outside a field render a plain input with their
   own hover and focus ring (`atoms/input.rs`).

For a missing-parent guard, keep the component signature `-> impl IntoView` and return `None` or `Some(view)`
from its body (`atoms/select.rs`). This avoids adding `AnyView` erasure. An explicit `-> Option<impl IntoView>`
signature conflicts with the component macro's own `erase_components` transformation in debug builds.

Provide contexts with `<Provider value=..>` around the children (or `scoped_view`), never with
`provide_context` in the component body: a component has no owner of its own, so a context provided there
reaches the atom's later siblings too.

An overlay hides contexts of its surroundings from its content (react-aria-components' `clearContexts`): a combo
box's popover passes `ClearContexts(&[clear_context::<LabelContext>, ..])`, so a `Label` or `Input` inside it doesn't
render as the combo box's. Parts that may sit in such content read their context with `use_clearable_context`
(`utils/scoped_context.rs`).

## Shared Atom Infrastructure

| Piece                                            | Where                                                | What it is for                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
|--------------------------------------------------|------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `Out<T>`                                         | `lib.rs`                                             | The writable half of state props (`set_<x>`): an `RwSignal`, `WriteSignal`, `StoredValue`, closure or `Callback`; `Out::set` writes.                                                                                                                                                                                                                                                                                                                                                                     |
| `ValueBinding::from_state_props`                 | `utils/value_binding.rs`                             | Turns `<x>` + `set_<x>` + `on_<x>_change` into the hook's binding and change callback.                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `with_default_class`, `flag`                     | `utils/default_class.rs`, `utils/data_attributes.rs` | The default class; boolean data attributes.                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `LabelContext`, `LabelPresence`                  | `atoms/field.rs`                                     | The `Label` part: the root provides `LabelContext` (`label`/`span`, `with_on_click`, `with_default_text`). Whether the `Label` is rendered decides the element's `aria-labelledby`, so don't guess it from the ARIA props: create a `LabelPresence::new(aria_label, aria_labelledby.as_ref())`, pass its `has_label` to the hook and attach it with `LabelContext::with_presence`. Until mounted it guesses (so server HTML references a likely label), then follows the rendered `Label` (react-aria-components' `useSlot`). |
| `FieldContext`                                   | `atoms/field.rs`                                     | One context for a field's `Description` and `FieldError` parts (C14): the hook's `description_props`/`error_message_props` (`SlotProps`) and the validation state. Every field atom provides it.                                                                                                                                                                                                                                                                                                         |
| `Slot`, `SlotProps`, `use_slot`                   | `utils/slot_id.rs`                                   | Optional elements referenced by id (description, error message): `use_slot` captures the element and its `referenced_id` is `Some` only while it is rendered, so ARIA references never dangle (react-aria's `useSlotId`).                                                                                                                                                                                                                                                                                |
| `IdRefs`, `labels`                               | `utils/id_refs.rs`, `utils/labels.rs`                | The one way to join id reference lists (`aria-describedby`, `aria-labelledby`, ...): `IdRefs::derive([..])` joins signals (absent ones skipped, each id once); `labels` is react-aria's `useLabels` (an element named by an `aria-label` and other elements lists itself first). |
| `use_description`                                | `utils/use_description.rs`                           | A shared, visually hidden description element for `aria-describedby` (react-aria's `useDescription`), client-only.                                                                                                                                                                                                                                                                                                                                                                                       |
| `scoped_view`                                    | `utils/scoped_context.rs`                            | Builds a view in a child owner whose contexts reach only its descendants, and forwards attributes set on the component to the view's root (where `<Provider>` would wrap the children instead).                                                                                                                                                                                                                                                                                                          |
| `ClearContexts`, `clear_context`, `use_clearable_context` | `utils/scoped_context.rs`                   | Hide a context from part of the tree (an overlay's content); `use_clearable_context` finds no `T` there unless a descendant provides one again (react-aria-components' `clearContexts`). See "Context Pattern".                                                                                                                                                                                                                                                       |
| Virtual focus                                    | `utils/virtual_focus.rs`                             | DOM focus stays on one element (a combo box input) while another (an option) is focused through `aria-activedescendant`; synthetic focus/blur events (`move_virtual_focus`). The collection hooks take `should_use_virtual_focus`.                                                                                                                                                                                                                                                                       |
| `OwnerAlive`                                     | `utils/owner_alive.rs`                               | A flag outside the reactive arena that turns `false` when the owner is cleaned up: check it in deferred callbacks (timeouts, animation frames, global listeners) before touching the atom's signals (see also "Blur After Disposal" in `leptos-and-dom.md`).                                                                                                                                                                                                                                       |

## Reference Implementations

| Pattern                       | File                                 |
|-------------------------------|--------------------------------------|
| Simple hook wrapper           | `leptonic/src/atoms/button.rs`       |
| State props (`x` + `set_x`)   | `leptonic/src/atoms/checkbox.rs`     |
| Context-based composition     | `leptonic/src/atoms/slider.rs`       |
| Field parts (label, errors)   | `leptonic/src/atoms/field.rs`        |
| Router integration            | `leptonic/src/atoms/link.rs`         |
| Overlay with portal           | `leptonic/src/atoms/popover.rs`      |
| Label detection, value parts  | `leptonic/src/atoms/progress_bar.rs` |
| `node_ref`                    | `leptonic/src/atoms/input.rs`        |

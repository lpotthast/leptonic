# Hook Implementation Patterns

Implementation patterns for Leptonic's hooks.

## Main Goal And Usage

The main goal for hooks is to give users simple, easy to use access to complex logic and accessibility handling.

Hooks generally add "capabilities" to DOM elements. Or the "make an element BE / BEHAVE LIKE {something}".

Hooks are just functions.

Connecting a hook to an element happens via Leptos's attribute spreading:

```rust
#[component]
fn Button() -> impl IntoView {
    // Making a <div> announce, show and act like a button.
    let UseButtonReturn { props, .. } = use_button(UseButtonInput {
        element_type: ButtonElementType::Other,
        ..UseButtonInput::default()
    });
    // `use_button` also returns styles (see "Props and Styles"), so its props are split first.
    let (attrs, styles) = props.into_parts();
    view! { <div {..attrs} style=styles>"Press me"</div> }
}
```

## Hook Naming

Hooks follow the `use_*` naming convention.

- A hook adding "press" functionality is named `use_press`.
- A hook adding "move" functionality is named `use_move`.
- ...

## Module Organization

Each hook must be implemented in its own module (file). Do not combine multiple hooks in a single file,
even when they are closely related (e.g., `use_grid_list` and `use_grid_list_item` belong in separate files).

Related hooks are grouped under a common directory with a `mod.rs` that re-exports all public items via `pub use`.

## Hook `*Input`, `*Return`, `*Props` and `*Attrs` Types

Any hook, like `use_press` for example, at least declares the following additional types (prefixed with their hook
name):

- UsePressInput
- UsePressReturn
- UsePressProps
- UsePressAttrs

`*Input` types should derive `Debug` and `Clone`. They may implement `Copy` but do not have to. When the hook needs
something without a sensible default (its state, an element, a key), callers write a struct literal naming every field:
`UseFooInput { state, on_bar: Some(cb), is_disabled: Signal::stored(false) }`. Never add `new(..)` (or similar)
constructors to `*Input` types (the user's rule, 2026-10-07): creating an input stays explicit and its field names
visible.
Implement `Default` only when every field has a meaningful default ("everything off": optional callbacks as
`Option<Callback<_>>`, flags as `Signal<bool>` defaulting to `false`). Never add placeholder defaults that build an
invalid configuration (an empty state, a never-attached element).
`*Props` and `*Return` types should derive `Debug` but **not** `Clone`. Props are designed to bind a hook to exactly
one DOM element; making them non-Clone enforces single-use at compile time. Only `into_attrs(self)` (consuming) is
provided for conversion.

(Use `Signal<T>` for reactive values (accepts constants via `.into()`, derived signals, or existing signals for `*Input`
fields.)

Hooks receive their input as a single parameter of `*Input`.

Hooks return data as a single `*Return` return value.

A `*Return` value has:

- At least one `props: *Props` member (`<part>_props` for further elements, C9), or `PropsWithStyles<*Props>` when the
  hook also sets inline styles (see "Props and Styles").
- Additional fields of type `Signal` that provide reactive access to data (`is_pressed`, `is_focus_visible`, ...).
- No `Callback` fields for programmatic control: state and its mutations live on the state struct of the matching
  `use_*_state` hook, as methods (C3).

Hooks expose (through `*Props`):

- their attributes as `Signal`s
- their event handlers as `EventHandler<E>`s
- special attributes like `ElementCaptureAttr` from the standalone `leptos-element-capture` crate

Every `*Props` type support conversion to the `*Attrs` type.

The `*Attrs` type is a type alias for a tuple declaring all `Attr` and `On` members (Leptos types). This tuple type
represents the type spreadable via Leptos's spread syntax: `{..props.into_attrs()}`.

Returning `props: *Props` (convertible to *Attrs), instead of directly returning `attrs: *Attrs`, gives us the
possibility to merge props returned by different hooks, as the values used in *Attrs are not necessarily destructable
after construction. Having raw access (owned, when possible) to signals and event handlers "making up the heart of a
hook" allows for easy programmatic merges. Merges of different hook *Return types must be implemented explicitly though.
We do not support a react-aria like generic `mergeProps` function.

### Props and Styles (`IntoAttrs`, `PropsWithStyles`)

Every `*Props` type implements `hooks::IntoAttrs` (`fn into_attrs(self) -> Self::Attrs`). Hooks that also set inline
styles (positions, `touch-action`, `user-select`, ...; e.g. `use_press`, `use_button`, `use_link`, `use_slider_thumb`,
the grid/table/listbox items) return `PropsWithStyles<*Props>` instead (`hooks/mod.rs`). It deliberately has no
`into_attrs` and isn't an attribute itself, because a `style` attribute spread onto an element would be replaced by
the caller's `style=`, or replace it. Split it and merge the styles with the caller's:

```rust
let (attrs, hook_styles) = button.props.into_parts();
view! { <button {..attrs} style=hook_styles.merge(styles)>"Save"</button> }
```

- `into_parts()` → `(Attrs, Styles)`: for spreading.
- `into_inner()` → `(Props, Styles)`: to compose the props further first (e.g. `merge_with`, then `into_attrs`).
- `PropsWithStyles<P>` implements `MergeWith<Other>` whenever `P` does, keeping its styles through the merge.

## API Conventions

react-aria defines the behavior; the API shape is ours (see "Based On React-Aria"). These conventions apply to every
hook input/return and atom prop. They are project-wide deviations from react-aria, recorded once as global entries
in `leptonic/src/hooks/mod.rs`; a hook's own deviation block only lists what goes beyond them.

| #   | Convention                                                                                                                                                                                                                                                                                 | Why                                                                                                                                                                      |
|-----|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| C1  | State flags are `is_disabled`, `is_read_only`, `is_required`, `is_invalid: Signal<bool>` (default `false`), in hook inputs and atom props alike. DOM-level `*Props` keep DOM attribute names (`disabled`, `aria_disabled`).                                                                | react-aria's names (`isDisabled`); one name per concept across hooks and atoms.                                                                                          |
| C2  | Ids, `name`, `form`: `Option<String>`. User-visible text (`aria_label`, placeholders, value labels): `MaybeProp<String>`.                                                                                                                                                                  | Text must be able to change at runtime (e.g. with the locale); `MaybeProp` accepts constants, `String`s and signals via `into`. `&'static str` rules out dynamic values. |
| C3  | `use_foo_state(..) -> FooState`: a `Copy` struct with read-only `Signal`s and methods (`set_value`, `toggle`, ...).                                                                                                                                                                        | Methods are discoverable and typed; a struct of `Callback` fields with tuple arguments is a JavaScript props-bag shape.                                                  |
| C4  | Hook-owned state: `default_*` + `on_*_change`, or a `ValueBinding` to app state; changes go through the state's methods (see "Hook-Owned State"). Atoms: `<x>` + `set_<x>: Out<T>`, or `default_<x>` + `on_<x>_change`. `is_invalid: Signal<bool>` is OR-ed into validation results. | Callers can't bypass the hook's invariants; change callbacks always fire.                                                                                               |
| C5  | Hooks read locale and writing direction from the i18n context (`use_locale()`, `use_direction()`); they never take `is_rtl`, `writing_direction` or `locale` inputs. Locale-derived defaults are `Option<_>` meaning "from the locale".                                                    | react-aria's `useLocale()` does the same; per-hook flags drift apart from the actual locale.                                                                             |
| C6  | One `Orientation` enum (no `Default`; callers name it, the docs give react-aria's default for each hook).                                                                                                                                                                                  | No near-identical per-module copies.                                                                                                                                     |
| C7  | No constructors on `*Input` types: struct literals naming every field; `Default` (and struct update) only when everything has a meaningful default.                                                                                                                                        | Creation stays explicit, field names visible (the user's rule).                                                                                                          |
| C8  | A hook takes one `*Input`; the state it works on goes into it (`UseFooInput { state, .. }`). Settings that live on the state are read from it, never repeated on the hook input.                                                                                                           | One place per setting; they can't disagree.                                                                                                                              |
| C9  | The element the hook is named after gets `props`, other elements `<part>_props`. Label, description and error message come from `use_field` (`SlotProps`).                                                                                                                                 | Uniform shapes; no per-hook label/error variants.                                                                                                                        |
| C10 | Callbacks: `Option<Callback<NamedEvent>>`. `Arc<dyn Fn(&T) -> R>` aliases only for predicates over borrowed data. No tuple arguments, no bools meaning modes, no `Callback<()>` for configuration. Delays are `Duration`; keys are `utils::key::KeyboardKey`, pointer types `PointerType`. | Typed, self-describing call sites; no string comparisons.                                                                                                                |
| C11 | Anything a user could change at runtime is a `Signal<T>` with a default. `Option<Signal<T>>` only for "inherit vs. override", documented on the field.                                                                                                                                     | Reactivity is the Leptos way to change configuration.                                                                                                                    |
| C12 | ARIA attributes use the typed enums from `utils/aria.rs`; `tabindex` is `i32`.                                                                                                                                                                                                             | See "ARIA Attribute Types".                                                                                                                                              |
| C13 | Units: `Fraction` (0..=1) for percentages, `Point { x, y }` for coordinates, `Duration` for time.                                                                                                                                                                                          | Units in the type, not in naming conventions.                                                                                                                            |
| C14 | One generic `Label`, `Description` and `FieldError` atom reading the `LabelContext`/`FieldContext` every field atom provides.                                                                                                                                                             | react-aria-components' `LabelContext`/`TextContext`/`FieldErrorContext`; no per-family parts.                                                                           |
| C15 | Number values are generic over `NumberValue` (all primitive integers and floats), with ICU4X decimals for parsing and formatting.                                                                                                                                                         | Exact integer stepping and clamping, min/max from the type (react-aria: JS numbers).                                                                                    |

## Input Destructuring

Hooks must destructure their `*Input` parameter at the very top of the function body.
This makes all available inputs visible at a glance and avoids `input.field` access scattered throughout the body.
It gives us compile-time safety while preserving our "single parameter input" concept.

```rust
pub fn use_foo(input: UseFooInput) -> UseFooReturn {
    let UseFooInput { is_disabled, on_change, value } = input;
    // ... rest of hook body uses `is_disabled`, `on_change`, `value` directly
}
```

## Event Handler Naming Conventions

Hooks use different naming conventions depending on where an event handler lives:

| Location                                       | Convention  | Example                                      |
|------------------------------------------------|-------------|----------------------------------------------|
| `*Input` struct fields (user callbacks)        | `on_*`      | `on_press`, `on_change`                      |
| Internal event handler closures (hook body)    | `handle_*`  | `handle_click`, `handle_keydown`             |
| `*Props` struct fields (`EventHandler<E>`)     | `on_*`      | `on_keydown`, `on_click`                     |
| Internal helper functions (not event handlers) | descriptive | `trigger_press_start`, `toggle`, `increment` |

**Rationale:** User callbacks in `*Input` and `EventHandler` fields in `*Props` both use `on_*` because they represent
the public API surface. Internal closures in the hook body use `handle_*` to avoid naming conflicts — a `handle_click`
closure often captures an `on_click` user callback, and distinct prefixes prevent shadowing and improve readability.

Internal helpers that invoke user callbacks but aren't direct DOM event handlers can use descriptive names like
`trigger_press_start` or `toggle`.

## Shared State

Many concepts, e.g. "sliders" with their respective `use_slider_*` hooks, need to share state between hooks.
Instead of adding the same fields to each hook's *Input type, create a `use_{concept_name}_state` hook returning
a shared state struct. This can then be passed explicitly to dependent hooks.

### Global State and SSR

Page-wide state (the visible overlays stack, the focus scope tree, description elements, the prevent-scroll count,
...) lives in `thread_local!`s. That is only sound in the browser, where one page runs on one thread. On the server,
axum's work-stealing runtime interleaves many requests on one thread and moves a request's render between threads at
every `.await` (Suspense, streaming), so a thread-local reached while rendering mixes requests and loses state.

- Touch thread-locals (and `static` atomics or locks) only from client-only code: `Effect`s, event handlers,
  `request_animation_frame`/timeouts, or code behind `#[cfg(not(feature = "ssr"))]`. Component and hook bodies
  and `on_cleanup` run on the server: from there, only release what an effect acquired (track it in a
  `StoredValue`, which stays empty on the server).
- Per-request state goes into the reactive owner: `provide_context` at a root, or Leptos' shared context (as
  `use_id` does for ids).
- Immutable caches whose content doesn't depend on the request (syntax sets, ICU data) may be global.

## EventHandler Abstraction

`EventHandler<E>` is a chainable and clonable wrapper for event handler functions.

```rust
use leptonic::utils::EventHandler;

fn main() {
    // Create single handler (no Vec allocation).
    let handler = EventHandler::new(move |e: KeyboardEvent| { /* ... */ });

    // Chain handlers (all handler-functions from both will be run in sequence).
    let handler = handler.chain(EventHandler::new(move |e: KeyboardEvent| { /* ... */ }));

    // Add one additional handler using a convenience fn.
    let handler = handler.then(move |e: KeyboardEvent| { /* ... */ });

    // Convert to `On<>` attribute for view spreading.
    let on_keydown = handler.into_on(ev::keydown);
}
```

`*Props` types store event handlers as `EventHandler` instances. This allows easy merges of different hook return
values.

## Event Propagation Control

**Rule** (the user's decision, 2026-10-07): an event type implements the sealed `Propagation` trait exactly when its
react-aria counterpart has `continuePropagation()`. Those events stop propagation by default; a user handler calls
`continue_propagation()` to let the native event bubble. Every other event type keeps upstream's fixed propagation
behavior and has no `Propagation`.

### Types That Implement It

| Event type | Hook(s) | react-aria counterpart |
|------------|---------|------------------------|
| `PressEvent` | `use_press` (and everything built on it: buttons, links, toggles, items, ...) | `PressEvent.continuePropagation()` (usePress) |
| `KeyboardEventWrapper` | `use_keyboard` (and type select, toggles, the autocomplete, keyboard shortcuts) | `KeyboardEvent = BaseEvent<KeyboardEvent>` (`createEventHandler`) |
| `EventWrapper<E>` | raw DOM events handed through such APIs | — (generic wrapper, `utils/event_wrapper.rs`) |

### Types That Deliberately Don't

| Event type | Upstream behavior (kept) |
|------------|--------------------------|
| Hover (`HoverStartEvent`, `HoverEndEvent`) | `useHover` never stops propagation. |
| Focus, focus within (`FocusEvent`, focus-within callbacks) | `useFocus`/`useFocusWithin` never stop propagation. Stopping `focusin`/`focusout` would break nested focus-within containers and collection listeners above the element. |
| Long press (`LongPressEvent`) | `LongPressEvent` is `Omit<PressEvent, 'type' \| 'continuePropagation'>`; `useLongPress` continues its press events' propagation. |
| Move (`MoveStartEvent`, `MoveEvent`, `MoveEndEvent`) | `useMove` stops the native events it handles unconditionally (no opt-out). |
| Scroll wheel | `useScrollWheel` stops the wheel event unconditionally (except Ctrl+wheel zoom). |
| DnD (drag, drop, drop-target events) | `useDrag`/`useDrop`/`DragManager` stop the native events they handle unconditionally. |

### Deciding for a New Event Type

Look up its type in `@react-types/shared/src/events.d.ts` (and the hook's own types): with `continuePropagation()`,
implement `Propagation` via `PropagationControl`; without it, port the hook's fixed behavior (never stop, or stop
unconditionally where upstream calls `stopPropagation()`) and add no `Propagation`.

### The Propagation Trait (Sealed)

`Propagation` (`utils/propagation_control.rs`) provides:

- `continue_propagation()` — opt in to letting the native DOM event bubble.
- `stop_propagation()` — explicit no-op (propagation is already stopped by default). Emits a compile-time
  deprecation warning to educate callers.
- `is_propagation_stopped() -> bool` — check current state.

### How It Works

1. Hook creates a `PropagationControl` (wraps `Arc<AtomicBool>` + callback).
2. Hook constructs the event (e.g., `PressEvent`) with a shared reference to the `PropagationControl`.
3. Hook runs the user's `on_press` handler, passing the event.
4. After the handler returns, the hook checks `is_propagation_stopped()` on the shared control.
5. If propagation was not continued, the hook calls `stop_propagation()` on the native DOM event.

**Reference**: `PressEvent` in `hooks/interactions/use_press.rs`

---

## Combining Hooks

There are two ways to combine hooks. Prefer the first one.

### Input Composition (preferred)

When a hook *configures* an element that another hook renders, it returns that hook's **input**, not DOM props.
A menu trigger configures a button, so `use_menu_trigger` returns a `UseButtonInput`; a spin button configures two
stepper buttons, so `use_spin_button` returns two `UseButtonInput`s. The caller hands them to `use_button`, adding
its own settings with struct update syntax:

```rust
let menu_trigger = use_menu_trigger(UseMenuTriggerInput { .. });
let button = use_button(UseButtonInput {
    on_hover_start: Some(Callback::new(|_| { /* ... */ })),
    ..menu_trigger.button
});
let (attrs, styles) = button.props.into_parts();
view! { <button {..attrs} style=styles>"Actions"</button> }
```

This is how react-aria passes `AriaButtonProps` between hooks. It keeps exactly one press, focus and hover state
machine per element. (Merging the DOM props of a menu trigger and a button used to attach two independent press
handlers to the same element.) When a configuring hook needs to *add* to a callback the caller may also set, chain
them, e.g. with `chain_optional_callbacks` from `use_press`.

For this to be pleasant, input structs that are meant to be composed implement `Default` (all options off, no
callbacks), so callers only name what they set.

### Merging Props (`MergeWith`)

When independent hooks add unrelated behavior to the same element (e.g., `use_press` + `use_hover` +
`use_focus_ring`), their Props need to be merged. The `MergeWith` trait provides a type-safe way to combine Props from
different hooks.

### The `MergeWith` Trait

```rust
use leptonic::utils::MergeWith;

// Trait definition
pub trait MergeWith<Other>: Sized {
    type Output;
    fn merge_with(self, other: Other) -> Self::Output;
}
```

### Merge Semantics

Different field types are merged differently:

| Field Type           | Merge Behavior                        |
|----------------------|---------------------------------------|
| `EventHandler<E>`    | Chained (both run in sequence)        |
| `ElementCaptureAttr` | Both kept (both capture element refs) |
| Other attributes     | "Last wins" (second overrides first)  |
| Distinct fields      | Included in output as-is              |

### Basic Usage

```rust
use leptonic::utils::MergeWith;
use leptonic::hooks::{use_press, use_hover, UsePressInput, UseHoverInput};

let press = use_press(press_input);
let hover = use_hover(hover_input);

// Merge press and hover props. `use_press` returns `PropsWithStyles`, which keeps its styles through the merge.
let (attrs, styles) = press.props.merge_with(hover.props).into_parts();

view! {
    <button {..attrs} style=styles>
        "Hover and click me"
    </button>
}
```

### Chained Merging

Merged types also implement `MergeWith`, enabling chains:

```rust
use leptonic::utils::MergeWith;

let press = use_press(press_input);
let hover = use_hover(hover_input);
let focus_ring = use_focus_ring(focus_ring_input);

let (attrs, styles) = press
    .props
    .merge_with(hover.props)
    .merge_with(focus_ring.props)
    .into_parts();

view! {
    <button {..attrs} style=styles>
        "Interactive button"
    </button>
}
```

### Available Merged Types

Pre-defined merged types in `leptonic::hooks::merged`:

| Merged Type                          | Source Hooks                                           |
|--------------------------------------|--------------------------------------------------------|
| `MergedPressHoverProps`              | `UsePressProps` + `UseHoverProps`                      |
| `MergedPressFocusRingProps`          | `UsePressProps` + `UseFocusRingProps`                  |
| `MergedHoverFocusRingProps`          | `UseHoverProps` + `UseFocusRingProps`                  |
| `MergedPressHoverFocusRingProps`     | `MergedPressHoverProps` + `UseFocusRingProps`          |
| `MergedFocusablePressProps`          | `UseFocusableProps` + `UsePressProps`                  |
| `MergedFocusablePressFocusRingProps` | `MergedFocusablePressProps` + `UseFocusRingProps`      |
| `MergedOverlayOverlayPositionProps`  | `UseOverlayProps` + `UseOverlayPositionProps`          |

Each merge is implemented in both orders (`a.merge_with(b)` and `b.merge_with(a)`).

### Why Not a Generic `mergeProps`?

React-aria provides a generic `mergeProps` function that dynamically merges props objects at runtime.
In Rust, this isn't possible because:

1. `*Attrs` types are statically-sized tuples
2. Merging two tuples produces a new tuple with a different size
3. Rust requires compile-time knowledge of tuple sizes

Instead, Leptonic uses explicit `MergeWith` implementations that define exactly which output type
results from merging two input types. This provides full type safety at the cost of requiring
explicit definitions for each useful combination.

### Implementing Custom Merges

For hook authors, implementing `MergeWith` for custom combinations:

```rust
use leptonic::utils::MergeWith;

// Define the merged output type
pub struct MergedFooBarProps {
    // Fields from both hooks
    pub from_foo: EventHandler<KeyboardEvent>,
    pub from_bar: EventHandler<MouseEvent>,
}

// Implement the merge
impl MergeWith<UseBarProps> for UseFooProps {
    type Output = MergedFooBarProps;

    fn merge_with(self, other: UseBarProps) -> Self::Output {
        MergedFooBarProps {
            from_foo: self.on_keydown,
            from_bar: other.on_click,
        }
    }
}
```

For overlapping event handlers, use `EventHandler::chain()`:

```rust
impl MergeWith<UseBarProps> for UseFooProps {
    type Output = MergedFooBarProps;

    fn merge_with(self, other: UseBarProps) -> Self::Output {
        MergedFooBarProps {
            // Chain overlapping handlers (both run in sequence)
            on_keydown: self.on_keydown.chain(other.on_keydown),
            // Include distinct fields directly
            on_click: other.on_click,
        }
    }
}
```

## Example

```rust
pub type UseFooAttrs = (
    Attr<attr::Disabled, Signal<bool>>,
    Attr<attr::Tabindex, Signal<i32>>,
    ElementCaptureAttr,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
);

pub struct UseFooProps {
    pub disabled: Signal<bool>,
    pub tabindex: Signal<i32>,
    pub element_capture: ElementCaptureAttr,

    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_click: EventHandler<MouseEvent>,
    pub on_pointerdown: EventHandler<PointerEvent>,
}

impl IntoAttrs for UseFooProps {
    type Attrs = UseFooAttrs;

    /// Convert to spreadable attributes for Leptos views, consuming self.
    fn into_attrs(self) -> UseFooAttrs {
        (
            // attributes
            Attr(attr::Disabled, self.disabled),
            Attr(attr::Tabindex, self.tabindex),
            self.element_capture,
            // event-handlers
            self.on_keydown.into_on(ev::keydown),
            self.on_click.into_on(ev::click),
            self.on_pointerdown.into_on(ev::pointerdown),
        )
    }
}

pub struct UseFooReturn {
    pub props: UseFooProps,
    pub is_pressed: Signal<bool>,
}

pub fn use_foo(input: UseFooInput) -> UseFooReturn {
    let UseFooInput { is_disabled, .. } = input;

    // hook logic...

    let handle_keydown = move |e: KeyboardEvent| {
        // ...
    };

    UseFooReturn {
        props: UseFooProps {
            disabled: is_disabled,
            on_keydown: EventHandler::new(handle_keydown),
            // ...
        },
        is_pressed: unimplemented!(),
    }
}
```

### Props are Single-Use (non-Clone)

Props types are intentionally **not Clone**. Only `into_attrs(self)` is provided, which consumes the Props.
This enforces that each hook's props are spread onto exactly one DOM element — preventing bugs from:

1. **Element capture conflicts** — `ElementCaptureAttr` writes to a single `CapturedElement`; cloning would overwrite.
2. **Shared event handler state** — Both elements would share the same `Arc<dyn Fn>` handlers mutating the same state.
3. **Duplicate ARIA IDs** — Both elements get identical IDs, violating HTML uniqueness requirements.

When you need attrs inside a reactive closure (e.g., `<Show>`), convert to attrs **before** the closure
and clone the attrs (which are Clone):

```rust
let press = use_press(input);
let attrs = press.props.into_attrs();  // Convert once

view! {
    <Show when=move || is_open.get()>
        <button {..attrs.clone()}>"Inside Show"</button>
    </Show>
}
```

## Implementation Details

### Custom Data Attributes

For `data-*` or unlisted ARIA attributes, use the following pattern (other types can be used in `Signal<...>`!):

```rust
use leptos::attr::custom::{custom_attribute, CustomAttr};

type UseFooAttrs = (
    CustomAttr<&'static str, Signal<Option<&'static str>>>,
);

struct UseFooProps {
    data_focus_visible: Signal<Option<&'static str>>,
}

impl IntoAttrs for UseFooProps {
    type Attrs = UseFooAttrs;

    fn into_attrs(self) -> UseFooAttrs {
        (custom_attribute("data-focus-visible", self.data_focus_visible),)
    }
}

fn use_foo() -> UseFooReturn {
    // Present and empty while true, absent otherwise (as `utils::data_attributes::flag` renders it).
    let data_focus_visible = Signal::derive(move || is_focus_visible.get().then_some(""));

    UseFooReturn {
        props: UseFooProps {
            data_focus_visible
        }
    }
}
```

### ARIA Attribute Types

ARIA attributes use the typed enums from `utils/aria.rs` (`AriaDisabled`, `AriaExpanded`, `AriaCheckedTristate`,
`AriaRole`, ...), **never `bool`** (and no string literals).

**Why:** Leptos renders `bool` values with standard HTML boolean-attribute semantics (attribute present when `true`,
absent when `false`). ARIA attributes require explicit string values like `"true"` or `"false"` per the
[WAI-ARIA spec](https://www.w3.org/TR/wai-aria-1.2/). Using `bool` causes the attribute to silently not render in
the DOM — the code compiles, no runtime warning is logged, and the accessibility attribute is simply missing.

**Pattern for reactive boolean ARIA attributes:**

```rust
// WRONG: aria-disabled will silently not render
pub aria_disabled: Signal<bool>,
// CORRECT: renders as aria-disabled="true"
pub aria_disabled: Signal<Option<AriaDisabled>>,

// Derive from a bool signal:
let aria_disabled = Signal::derive(move || is_disabled.get().then_some(AriaDisabled::True));
```

**Pattern for optional ARIA attributes** (attribute absent when not applicable):

```rust
// Use Option to suppress the attribute entirely when None
pub aria_selected: Signal<Option<AriaSelected>>,
let aria_selected = Signal::derive(move || {
    (selection_mode != SelectionMode::None).then(|| AriaSelected::from(is_selected.get()))
});
```

### Typed ARIA Roles

`utils/aria.rs` provides an `AriaRole` enum covering all WAI-ARIA roles, plus typed enums for other ARIA
attributes (`AriaDisabled`, `AriaExpanded`, `AriaCheckedTristate`, etc.).

**Integration with Leptos:** The `impl_attribute_value_via_str!` macro implements `AttributeValue` for each
type, enabling direct use in attribute tuples:

```rust
// In Props
pub role: Signal<AriaRole>,

// In Attrs type
Attr<attr::Role, Signal<AriaRole> >,
```

**Other typed ARIA types:**

- Boolean: `AriaDisabled`, `AriaExpanded`, `AriaHidden`, `AriaPressed`, `AriaRequired`, `AriaSelected`, etc.
- Tristate: `AriaCheckedTristate` (`True`, `False`, `Mixed`)
- Multi-value enums: `AriaOrientation`, `AriaSort`, `AriaAutoComplete`, etc.
- Reference IDs: `String` or `Signal<String>` for `aria-labelledby`, `aria-describedby`, etc.

**Design principle:** No `Undefined` variants — optionality is expressed via `Option<T>`. An `Option<AriaRole>`
that is `None` produces no `role` attribute in the DOM.

### Element Ids

Hooks that link elements (`aria-labelledby`, `aria-controls`, `aria-activedescendant`, `for`, ...) create ids with
`crate::utils::id::use_id("prefix")`, never with random values like `Uuid::new_v4()`. `use_id` draws from Leptos'
hydration counter, so the server and the hydrating client produce the same ids.

Call it in the hook body, unconditionally, in the same order on server and client: never in an effect or event
handler, and never in a code path that exists on only one side (e.g. after an `#[cfg(feature = "ssr")]` early
return). Otherwise every id created afterwards differs between server and client. The browser test
`test_hydration_ids.rs` compares the server's HTML with the hydrated DOM on every fixture the test app's index
lists (new fixtures are covered automatically).

### Element Capture Pattern

Many hooks cannot solely rely on returning spreadable props. They often need direct programmatic access to DOM
element, e.g. for focus management or dimension retrieval.

Hooks should NOT require users to manually declare and bind `NodeRef`s to the element to which props are spread as well.

Hooks should use `CapturedElement` from the standalone `leptos-element-capture` crate. It bundles a `StoredValue` for
the DOM element with a `Trigger` for reactive tracking. This ensures that Effects reading the element via
`CapturedElement::get()` automatically re-run when the element is captured — which is critical when the element lives
inside a reactive boundary like `<Show>` or is rendered during client-side navigation:

```rust
use leptos_element_capture::{CapturedElement, ElementCaptureAttr};

fn use_foo() -> UseFooReturn {
    let element = CapturedElement::new();

    // Reactive read — Effect re-runs when the element is captured.
    Effect::new(move |_| {
        let Some(el) = element.get() else { return };
        if let Some(html_el) = el.dyn_ref::<web_sys::HtmlElement>() {
            let _ = html_el.focus();
        }
    });

    // Include in attrs tuple
    UseFooReturn {
        props: UseFooProps {
            element_capture: element.attr(),
        }
    }
}
```

**Timing**: `Attribute::build()` runs synchronously during view construction. During SSR hydration the element is
captured before Effects run. However, inside reactive boundaries (`<Show>`, `<Suspense>`, etc.) during client-side
navigation, the element may be captured *after* Effects have already run once. `CapturedElement` handles this by
notifying a `Trigger`, causing dependent Effects to re-run.

**Reference**: `use_menu_item`

### CapturedElement vs IntoElementMaybeSignal

**Always strongly prefer `CapturedElement`.** It gives hooks a shared, crate-level element-capture abstraction and
frees users from having to create and bind a `NodeRef` manually — the hook captures the element automatically via
attribute spreading.

`IntoElementMaybeSignal` (generic element input) is a **last resort**, only required when ALL of these are true:

1. The hook does NOT spread props onto the element (so it cannot capture the element itself).
2. No other hook being used alongside could provide the element via its own `CapturedElement`.

Even when a hook doesn't spread props, if it's typically used alongside another hook that does spread (and
thus captures the element), the non-spreading hook should accept a `CapturedElement` from the spreading hook
rather than forcing the user to create a `NodeRef`.

**Reference**: `use_overlay_position` accepts a `CapturedElement` from the trigger hook rather than requiring
a separate `NodeRef`.

### Element Reference Pattern

When hooks really need an element reference, and cannot use the `Element Capture Pattern`,
use `IntoElementMaybeSignal<web_sys::Element, M>` from `leptos_use::core`:

```rust
use leptos_use::core::IntoElementMaybeSignal;

pub struct UseHookInput<El, M>
where
    El: IntoElementMaybeSignal<web_sys::Element, M> + Clone
{
    pub element: El,
    pub phantom_data: PhantomData<M>,
}

#[component]
fn SomeComponent() {
    // Consumer passes NodeRef directly
    let el = NodeRef::<html::Div>::new();
    let UseHookReturn { props, .. } = use_hook(UseHookInput { element: el, phantom_data: PhantomData });

    view! {
        <div {..props.into_attrs()} node_ref=el>"div"</div>
    }
}
```

### Dynamic Event Listeners

When event listeners must be attached dynamically, use `leptos_use::use_event_listener` (can be called from within other
event handlers):

```rust
use leptos_use::use_event_listener;

fn use_foo() {
    type CleanupFn = Box<dyn Fn() + Send + Sync + 'static>;
    type CleanupFns = (CleanupFn, CleanupFn, CleanupFn);
    let cleanup_listeners: StoredValue<Option<CleanupFns>, LocalStorage> =
        StoredValue::new_local(None);

    let on_move = move |e: MouseEvent| {};
    let on_up = move |e: PointerEvent| {};
    let on_cancel = move |e: PointerEvent| {};

    let on_pointer_down = move |e: PointerEvent| {
        // Prefer using an event-target based "owner document" instead of a global document.
        // This supports iframes and shadow-doms.
        let doc = e.current_target().expect("has target").get_owner_document();

        // Clone already existing handlers to enable repeated usage.
        let move_cleanup = use_event_listener(doc.clone(), ev::pointermove, on_move.clone());
        let up_cleanup = use_event_listener(doc.clone(), ev::pointerup, on_up.clone());
        let cancel_cleanup = use_event_listener(doc.clone(), ev::pointercancel, on_cancel.clone());

        // Store cleanup for later removal.
        cleanup_listeners.set_value(Some((
            Box::new(move_cleanup),
            Box::new(up_cleanup),
            Box::new(cancel_cleanup),
        )));
    };

    let on_pointer_cancel = move |e: PointerEvent| {
        if tracking_pointer.get_value().is_some_and(|id| id == e.pointer_id()) {
            // other logic...

            // Potentially clean up listeners on other events.
            cleanup_listeners.with_value(|cleanup| {
                if let Some((move_cleanup, up_cleanup, cancel_cleanup)) = cleanup {
                    move_cleanup();
                    up_cleanup();
                    cancel_cleanup();
                }
            });
            cleanup_listeners.set_value(None);
        }
    };
}
```

Always prefer using `get_owner_document()` from `crate::utils::EventTargetExt`.

Document "why", when deviating from this recommendation.

**Reference**: `use_press`, `use_move`, `use_slider_thumb`

### No Nested Dispatch of the Same Event Type

Every `on:` handler (Leptos' event delegation feature is not enabled, so each handler is its own listener) and every
`use_event_listener` handler is a wasm-bindgen `FnMut` closure, which can't be re-entered: when a handler synchronously causes another event of the same type, the nested dispatch throws
"closure invoked recursively or after being dropped" (and the nested handler doesn't run). The classic case is
moving focus inside a `focusin` handler: `element.focus()` dispatches a nested `focusin`.

react-aria often does this synchronously (React's event system tolerates it). Port it by moving the action out of the
dispatch with `queue_microtask` (wrap DOM values in `SendWrapper`), and say why in a comment. Browser tests fail on
uncaught page errors, so the nested-dispatch error is caught by any test that triggers it.

**Reference**: `use_selectable_collection` (`on_focusin`), `FocusScope` (focus containment)

### Blur After Disposal

Removing a focused element (a dismiss button removing its chip, a clear button hiding itself) makes the browser
fire `blur`/`focusout` on it while Leptos unmounts it, after its owner and the callbacks and signals it owned were
disposed. Blur and focus-out paths therefore run callbacks with `try_run` and read signals with `try_get_untracked`:
nothing is left to notify then. (Other events can't reach a removed element, so they keep `run`.)

**Reference**: `use_focus`, `use_focus_within`, `use_focus_ring`; `utils::owner_alive::OwnerAlive` for deferred
callbacks (timeouts, animation frames, global listeners) that may run after disposal.

### Effect Read Order

An effect reading several memos must read upstream ones before memos derived from them (a collection before its
filtered memo). reactive_graph 0.2 checks an effect's sources in read order, with the effect as observer: a memo
that recomputes while a downstream memo is being checked doesn't mark the effect dirty (it assumes the observer
triggered it), and when the effect checks that memo itself, it is already clean. If the downstream memo came out
unchanged, the effect doesn't run, although an upstream value it reads changed. App values often reach a hook
through memos (a value and a collection derived from one app memo), so this applies to every reconciling effect.

So: read the app's inputs (bindings, collections, signals passed in) first, then the hook's own memos; read every
source up front rather than only in some branches. No order helps when one input is an app memo and another a
derived signal of the same upstream memo: there, read the inputs through hook-local memos, so that the effect isn't
a direct subscriber of an app memo (the skip only spares direct subscribers).

**Reference**: `use_combobox_state`'s reconciliation effect; browser test `combobox_tests` (agnite dev-ui's log
source picker: value and items derived from one memo, with a filter).

## Hook-Owned State (React Aria Deviation)

React Aria's state hooks (e.g., `useOverlayTriggerState`) use `useControlledState` to support both
controlled (`isOpen` prop from parent) and uncontrolled (`defaultOpen`) patterns. This exists because
React components cannot share mutable state — parent-child communication requires explicit props.

In Leptos, `Signal<T>` is `Copy` and inherently shared. However, accepting a writable signal
(e.g., `RwSignal<bool>`) from the caller would allow them to mutate state directly, bypassing the
hook's mutation path. This breaks invariants and prevents the hook from intercepting changes
(e.g., firing `on_open_change`, resetting related state).

**Convention (C4):** the hook owns the state's mutation path. A state hook (`use_*_state`) takes one of:

- `default_<x>: T` (the initial value) and `on_<x>_change: Option<Callback<T>>`: the hook stores the value itself;
- `<x>: Option<ValueBinding<T>>`: the value lives in app state (a read `Signal` plus a setter; from an `RwSignal`, a
  signal pair or `ValueBinding::new`), and every change still goes through the hook, which calls the setter
  (`default_<x>` is then ignored).

It returns a `Copy` state struct (C3) exposing:

- read-only `Signal`s for observation;
- semantic mutation methods (`open`, `close`, `toggle`, `set_value`, ...), which keep the invariants and call
  `on_<x>_change` on every change.

Examples: `UseToggleStateInput::value`, `SelectionOptions::selection`; `UseTextFieldStateInput` is a small complete
one.

**Atoms** (the user's rule, 2026-10-06) take controlled state as two props, never as one binding:
a readable `<x>` (`#[prop(into)] Signal<T>` or `MaybeProp<T>`: a plain value, any signal, a closure) and a writable
`set_<x>: Out<T>` (an `RwSignal`, `WriteSignal`, `StoredValue`, closure or `Callback`; for `is_<x>` the setter is
`set_<x>`). This keeps every usage pattern open instead of forcing an `RwSignal`. Uncontrolled: `default_<x>`, plus
`on_<x>_change` to observe. The atom builds the hook's `ValueBinding` from the two props
(`ValueBinding::from_state_props`, see `atoms-implementation.md`).

When porting a React Aria hook whose state uses `useControlledState`, map `value`/`defaultValue`/`onChange` onto
this pattern. It is a **project-wide deviation** (the global block in `hooks/mod.rs`), not a per-hook one.

## Animation Lifecycle Hooks

The animation hooks (`hooks/animation/`) manage CSS animation lifecycles using the Web Animations API.

### `use_enter_animation`

- **Input**: `CapturedElement` + `is_ready: Signal<bool>`
- **Output**: `is_entering: Signal<bool>`
- Watches for CSS animations on the element when `is_ready` becomes true. Reports `true` while animations
  are running, `false` when complete.

### `use_exit_animation`

- **Input**: `CapturedElement` + `is_open: Signal<bool>`
- **Output**: `is_exiting: Signal<bool>`, `exit_state: Signal<ExitState>`
- `ExitState` transitions: `Open` → `Exiting` → `Closed`.
- When `is_open` becomes `false`, transitions to `Exiting` and watches animations. Transitions to `Closed`
  when animations complete.

### Internal: `watch_animations()`

Uses `Element.getAnimations()` + `Promise.all(animation.finished)` to detect animation completion. This is
a Web Animations API feature — during SSR, no API calls are made; enter reports `false`, exit follows
`is_open` directly.

### Usage Pattern (Overlays)

Exit animations require the element to stay mounted during the animation:

```rust
let exit = use_exit_animation(UseExitAnimationInput {
element: captured_element.clone(),
is_open: is_open.into(),
});

// Keep element mounted until exit animation completes
let is_mounted = Signal::derive(move | | {
is_open.get() | | exit.exit_state.get() == ExitState::Exiting
});
```

**Reference**: `hooks/animation/use_enter_animation.rs`, `hooks/animation/use_exit_animation.rs`

---

## Form Validation Hooks

Three cooperating hooks handle form validation, composed internally by the field hooks (text field, number field,
checkbox, radio group, select, combo box, date field, ...).

### `use_form_validation_state`

State management for multiple validation sources:

- `is_invalid` (OR-ed with the rest), server errors (via `FormValidationContext`, the `Form` atom's
  `validation_errors`), client-side `validate` functions, and native HTML5 validity (`builtin_validation`, read
  back by `use_form_validation`).
- `ValidationBehavior::Aria`: errors show in realtime through ARIA attributes, the form submits anyway.
- `ValidationBehavior::Native`: errors show on commit (form submission, `change`), and the field's custom validity
  (`setCustomValidity()`) blocks submission.
- Returns `UseFormValidationStateReturn`: `realtime_validation` and `display_validation` (`ValidationResult`:
  `is_invalid`, `validation_errors`, `validation_details` as a `ValidityStateSnapshot`) plus `update`/`reset`/
  `commit_validation` (still callbacks: moving them onto a state struct with methods is a C3 item in `PLAN.md`).

### `use_form_validation`

DOM connection hook (no return value), `UseFormValidationInput { element, state, validation_behavior, focus }`:

- In `Native` mode, calls `setCustomValidity()` on the field's own `<input>`/`<textarea>`/`<select>` (`element`)
  from the realtime validation, and reads the native validity back.
- Commits the validation on `invalid` (form submission) and `change`, focusing the form's first invalid field
  (`focus` for fields whose focusable element isn't the validated one).
- Resets the validation when the form is reset.

### `use_form_reset`

Restores a field's initial value when its `<form>` is reset:

- `UseFormResetInput { element, initial_value, on_reset }`: listens for `reset` on the form of `element` and calls
  `on_reset(initial_value)`.

### Integration Pattern

Field hooks compose all three internally:

```rust
let validation = use_form_validation_state(UseFormValidationStateInput { value, validate, .. });
use_form_validation(UseFormValidationInput {
    element,
    state: validation,
    validation_behavior,
    focus: None,
});
use_form_reset(UseFormResetInput { element, initial_value, on_reset: state_reset });
```

**Reference**: `hooks/form/use_form_validation_state.rs`, `hooks/form/use_text_field.rs`

---

## Native Tests

Pure logic and `*_state` hooks get native unit tests (`cargo test -p leptonic --features full --lib`), with
`assertr`. A hook needs a reactive owner; run it through `crate::testing::with_owner` (`leptonic/src/testing.rs`,
test builds only), which also makes Effects run:

```rust
use crate::testing::{flush_effects, with_owner};

#[test]
fn focus_moves_to_the_row_that_took_the_removed_rows_place() {
    with_owner(|| {
        let rows = RwSignal::new(vec!["alice", "bob", "carol"]);
        let state = grid(rows, &[], GridFocusMode::Row); // calls `use_grid_state`
        flush_effects(); // the Effects' first runs
        state.list.selection.set_focused_key(Some(Key::from("bob")), None);
        rows.update(|rows| rows.retain(|row| *row != "bob"));
        flush_effects(); // the re-runs the change caused
        assert_that!(state.list.selection.focused_key()).is_equal_to(Some(Key::from("carol")));
    });
}
```

- `with_owner(f)` runs `f` in a fresh `Owner` (as a component body runs), without Leptos' "read outside a tracking
  context" warnings, and disposes the owner afterwards.
- `flush_effects()` runs every pending Effect of the test's thread until none can make progress: the first runs of
  new Effects, and the re-runs caused by signal changes. Effects never run on their own, so a test decides when (like
  React's `act`); call it after creating the state and after each change whose Effects matter.
- How: leptonic's dev-dependencies enable `reactive_graph`'s `effects` feature (Leptos enables it only for
  `csr`/`hydrate`), and `with_owner` installs an `any_spawner` executor that queues every task on the spawning
  thread, polled by `flush_effects`. Tests under a plain `Owner::new().with(..)` keep working; their Effects don't run
  (`any_spawner`'s `tracing` feature drops tasks spawned before an executor exists instead of panicking).
- Signals and memos work without `flush_effects`; only Effects need it. Code touching the DOM can't run natively:
  that is the browser tests' job.

**Reference**: `hooks/grid/use_grid_state.rs` (tests of the refocus Effect).

## Based On React-Aria

Most hooks are based on hooks from Adobe's `react-aria` library (part of `react-spectrum`), checked out at
`{leptonic_root_dir}/../react-spectrum`. Since react-spectrum's package consolidation (2026-03), the sources live in
`packages/react-aria/src/<package>/` and `packages/react-stately/src/<package>/`. The old `packages/@react-aria/*`
packages only re-export them.

### Tracking Upstream Changes

react-aria keeps evolving, so every file that ports upstream code declares where it came from, at the very top:

```rust
// Upstream: react-aria/src/interactions/usePress.ts @ 6f664fe911
// Upstream: react-stately/src/toggle/useToggleState.ts @ 6f664fe911
```

The path is relative to react-spectrum's `packages/` directory. The hash is the react-spectrum commit this file was
last synced against: everything upstream up to that commit is either ported or consciously skipped (and then listed
in the deviations block below).

`scripts/upstream-drift.sh` turns these lines into a work list:

```bash
scripts/upstream-drift.sh                 # Files with unabsorbed upstream commits, most drifted first.
scripts/upstream-drift.sh -v use_press    # The commits themselves, for files matching the filter.
scripts/upstream-drift.sh --mark-synced leptonic/src/hooks/interactions/use_press.rs
```

Browser tests use the same header for the react-aria tests they mirror (e.g.
`// Upstream: react-aria-components/test/ListBox.test.js @ <sha>` in `leptonic/tests/ui_tests/test_listbox.rs`), so
new or changed upstream tests show up in the drift report too. react-aria's tests are the best specification of
expected behavior: derive our test cases from them (and from the implementation), instead of inventing them.

To re-sync a hook: read the listed upstream commits (`git -C ../react-spectrum show <hash>`), port what applies, add
deviations for what doesn't, cover the behavior with tests, then run `--mark-synced` on the file. When you add a new
hook, add its `// Upstream:` lines with react-spectrum's current HEAD (`git -C ../react-spectrum rev-parse
--short=10 HEAD`).

### React-Aria Deviations

Wherever we decided to deviate from the react-aria implementation of a hook, document these intentional differences
at the top of hook files:

```rust
// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - [What differs]: [why]. React-aria: [what it does].
//
// ## OMITTED FEATURES
// - `featureName`: [why]. React-aria: [what it does].
//
// =============================================================================
```

Use exactly these categories, in this order, and only the ones that apply. **Every entry states its reason.**

| Category                      | Use When                                                              |
|-------------------------------|-----------------------------------------------------------------------|
| `API DIFFERENCES`             | Naming or structural changes (Rust-native API shapes)                 |
| `DIFFERENT BEHAVIOR`          | Same feature, different approach                                      |
| `LEPTOS-SPECIFIC ADAPTATIONS` | Changes required by Rust/Leptos (event model, SSR, ownership)         |
| `ADDITIONS`                   | Functionality react-aria doesn't have                                 |
| `OMITTED FEATURES`            | Not implemented: intentionally (say why) or not yet (say what blocks) |

A hook without deviations says so in one line: `// No deviations from react-aria beyond the project-wide API
conventions.` Leptonic-only hooks (no upstream counterpart) start with `// No upstream: <what it is for>.` instead of
an `// Upstream:` header. Project-wide deviations (the API conventions above, hook-owned state, `CapturedElement`
instead of refs) are recorded once in `leptonic/src/hooks/mod.rs`; don't repeat them per hook.

## Book-SSR Documentation Pages

Every hook has a page in the book. Page structure, the page kit (`DocPage`, `Section`, `ApiTable`, `Demo`, ...) and
how to register a page (route in `src/routes.rs`, entry in `src/nav.rs`) are described in
`documentation/documentation-strategy.md`.

## Reference Implementations

| Pattern                           | File                                                                                                         |
|-----------------------------------|--------------------------------------------------------------------------------------------------------------|
| API conventions (state, input)    | `leptonic/src/hooks/tabs/use_tab_list_state.rs`, `leptonic/src/hooks/table/use_table_column_resize_state.rs` |
| Hook composition, attrs           | `leptonic/src/hooks/button/use_button.rs`                                                                    |
| Props pattern, mergeable handlers | `leptonic/src/hooks/interactions/use_press.rs`                                                               |
| Props merging (`MergeWith`)       | `leptonic/src/hooks/merged/mod.rs`                                                                           |
| CustomAttr                        | `leptonic/src/hooks/focus/use_focus_ring.rs`                                                                 |
| Element reference                 | `leptonic/src/hooks/overlay/use_overlay_position.rs`                                                         |
| Element capture                   | `leptonic/src/hooks/menu/use_menu_item.rs`                                                                   |
| Dynamic listeners, drag           | `leptonic/src/hooks/interactions/use_press.rs`                                                               |
| Slider drag                       | `leptonic/src/hooks/slider/use_slider.rs`                                                                    |
| Animation lifecycle (enter)       | `leptonic/src/hooks/animation/use_enter_animation.rs`                                                        |
| Animation lifecycle (exit)        | `leptonic/src/hooks/animation/use_exit_animation.rs`                                                         |
| Form validation state             | `leptonic/src/hooks/form/use_form_validation_state.rs`                                                       |
| DOM validation binding            | `leptonic/src/hooks/form/use_form_validation.rs`                                                             |
| Form reset detection              | `leptonic/src/hooks/form/use_form_reset.rs`                                                                  |
| Event propagation control         | `leptonic/src/hooks/interactions/use_press.rs` (PressEvent)                                                  |
| Keyboard DnD                      | `leptonic/src/hooks/dnd/drag_manager.rs`                                                                     |

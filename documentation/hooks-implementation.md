# Hook Implementation Patterns

How Leptonic's hooks are shaped: types, props, events, state, ids and elements. Rules for any code running
in Leptos and the browser (hooks and atoms): `leptos-and-dom.md`; porting from react-aria (upstream headers,
deviation blocks): `porting.md`; tests: `testing.md`; atoms: `atoms-implementation.md`.

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

Related hooks are grouped under a common directory with a `mod.rs` that re-exports all public items via `pub use`
(the hook files themselves are `pub(crate) mod`). That family module is the hook's only public path
(`leptonic::hooks::button::use_button`): `hooks/mod.rs` re-exports the families only crate-internally, and there is no
prelude (`conventions.md`, "No prelude; one path per public item").

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

Every `*Props` type implements `IntoAttrs` (`fn into_attrs(self) -> Self::Attrs`; `leptonic::IntoAttrs`, defined in
`attrs.rs`). Hooks that also set inline styles (positions, `touch-action`, `user-select`, ...; e.g. `use_press`,
`use_button`, `use_link`, `use_slider_thumb`, the grid/table/listbox items) return `PropsWithStyles<*Props>` instead
(`attrs.rs`, `leptonic::PropsWithStyles`). It deliberately has no
`into_attrs` and isn't an attribute itself, because a `style` attribute spread onto an element would be replaced by
the caller's `style=`, or replace it. Split it and merge the styles with the caller's:

```rust
let (attrs, hook_styles) = button.props.into_parts();
view! { <button {..attrs} style=hook_styles.merge(styles)>"Save"</button> }
```

- `into_parts()` → `(Attrs, Styles)`: for spreading.
- `into_inner()` → `(Props, Styles)`: to change the props further first (then `into_attrs`).

## API Conventions

The conventions C1–C16 for every hook input/return and atom prop (state flags, strings, state shape, hook-owned
state, locale, callbacks, reactivity, ARIA typing, units, field parts, number and selection values) are defined in
`conventions.md`, "API conventions". They are project-wide deviations from react-aria, recorded once as global
entries in `leptonic/src/hooks/mod.rs`; a hook's own deviation block only lists what goes beyond them.

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

## EventHandler Abstraction

`EventHandler<E>` is a chainable and clonable wrapper for event handler functions.

```rust
use leptonic::EventHandler;

fn main() {
    // Create single handler (no Vec allocation).
    let handler = EventHandler::new(move |e: KeyboardEvent| { /* ... */ });

    // Chain handlers (all handler-functions from both will be run in sequence).
    let handler = handler.chain(EventHandler::new(move |e: KeyboardEvent| { /* ... */ }));

    // Add one additional handler using a convenience fn.
    let handler = handler.then(move |e: KeyboardEvent| { /* ... */ });

    // The listener attribute for view spreading (`OnEvent<ev::keydown>`).
    let on_keydown = handler.into_on(ev::keydown);
}
```

`*Props` types store event handlers as `EventHandler` instances. This allows easy merges of different hook return
values. `into_on` gives an `OnEvent<D>`: a DOM listener for a handler, nothing for an `EventHandler::empty()`, so a
hook leaves handlers it doesn't need empty instead of attaching no-op listeners (also on the server).

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
- `is_propagation_stopped() -> bool` — check current state.

There is no `stop_propagation()`: propagation is already stopped by default.

### How It Works

1. Hook creates a `PropagationControl` (wraps an `Arc<AtomicBool>`; clones share it).
2. Hook constructs the event (e.g., `PressEvent`) with a clone of the `PropagationControl`.
3. Hook runs the user's `on_press` handler, passing the event.
4. After the handler returns, the hook checks `is_propagation_stopped()` on the shared control.
5. If propagation was not continued, the hook calls `stop_propagation()` on the native DOM event.

**Reference**: `PressEvent` in `hooks/interactions/use_press.rs`

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
them (inside leptonic: `chain_optional_callbacks` from `use_press`; long press groups: `merge_long_press`).

For this to be pleasant, input structs that are meant to be composed implement `Default` (all options off, no
callbacks), so callers only name what they set.

### Independent Hooks on One Element

When independent hooks add unrelated behavior to the same element (e.g. `use_press` + `use_hover`), spread each
hook's attributes onto it; Leptos attaches every spread's listeners, in spread order:

```rust
let press = use_press(press_input);
let hover = use_hover(hover_input);
let (press_attrs, styles) = press.props.into_parts();

view! { <button {..press_attrs} {..hover.props.into_attrs()} style=styles>"Hover and click me"</button> }
```

There is no merge mechanism (react-aria's runtime `mergeProps`; leptonic's typed `MergeWith` structs were removed
2026-10-09: one struct per pair of hooks, and each spread works as well). Two hooks never set the same plain
attribute on one element; where several contribute to one id reference list (`aria-describedby`,
`aria-labelledby`), the hook that combines them joins the contributions with `IdRefs`
(`IdRefs::derive([..])` for signals, `collect::<IdRefs>()` for values), never by hand. An element labelled by both
an `aria-label` and other elements gets its labelling from `utils::labels` (react-aria's `useLabels`).

## Example

```rust
pub type UseFooAttrs = (
    Attr<attr::Disabled, Signal<bool>>,
    Attr<attr::Tabindex, Signal<i32>>,
    ElementCaptureAttr,
    OnEvent<ev::keydown>,
    OnEvent<ev::click>,
    OnEvent<ev::pointerdown>,
);

#[derive(Debug)] // `IntoAttrs` requires `Debug`.
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

#[derive(Debug)]
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
let hover = use_hover(input);
let attrs = hover.props.into_attrs();  // Convert once

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

#[derive(Debug)]
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
    // `"true"` while true, absent otherwise (as `utils::data_attributes::flag` renders it).
    let data_focus_visible = Signal::derive(move || is_focus_visible.get().then_some("true"));

    UseFooReturn {
        props: UseFooProps {
            data_focus_visible
        }
    }
}
```

### ARIA Attribute Types

ARIA attributes use the typed enums from `utils/aria.rs` (`AriaDisabled`, `AriaExpanded`, `AriaChecked`,
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
attributes (`AriaDisabled`, `AriaExpanded`, `AriaChecked`, etc.).

**Integration with Leptos:** The `impl_attribute_value_via_str!` macro implements `AttributeValue` for each
type, enabling direct use in attribute tuples:

```rust
// In Props
pub role: Signal<AriaRole>,

// In Attrs type
Attr<attr::Role, Signal<AriaRole> >,
```

**Other typed ARIA types:**

- Boolean: `AriaDisabled`, `AriaExpanded`, `AriaHidden`, `AriaModal`, `AriaReadonly`, `AriaRequired`,
  `AriaSelected`, etc.
- Tristate: `AriaChecked`, `AriaPressed` (`True`, `False`, `Mixed`)
- Multi-value enums: `AriaOrientation`, `AriaSort`, `AriaAutocomplete`, `AriaHasPopup`, `AriaInvalid`, etc.
- Reference IDs: `Option<String>` (`Signal<Option<String>>`) for `aria-labelledby`, `aria-describedby`, etc.; lists
  joined with `IdRefs` (see "Independent Hooks on One Element").

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
use `IntoElementMaybeSignal<web_sys::Element, M>` from `leptos_use::core` (no hook needs it today):

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
fn SomeComponent() -> impl IntoView {
    // Consumer passes NodeRef directly
    let el = NodeRef::<html::Div>::new();
    let UseHookReturn { props, .. } = use_hook(UseHookInput { element: el, phantom_data: PhantomData });

    view! {
        <div {..props.into_attrs()} node_ref=el>"div"</div>
    }
}
```

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

- **Input**: `element: CapturedElement` + `is_ready: Signal<bool>` + `on_enter: Option<Callback<_>>` (called with
  the element when the entry starts, e.g. to start a Web Animation)
- **Output**: `is_entering: Signal<bool>`, `styles: Styles` (hide the element, without affecting layout, until it is
  ready for the first time, so that a closing popover stays visible during its exit animation; merge them into the
  element's `style`)
- Watches the element's animations once `is_ready` is true. Reports `true` while they are running, `false` when
  complete.

### `use_exit_animation`

- **Input**: `element: CapturedElement` + `is_open: Signal<bool>` + `on_exit: Option<Callback<_>>` (called with the
  element when the exit starts)
- **Output**: `is_exiting: Signal<bool>`, `exit_state: Signal<ExitState>`
- `ExitState` transitions: `Open` → `Exiting` → `Closed`.
- When `is_open` becomes `false`, transitions to `Exiting` and watches animations. Transitions to `Closed`
  when animations complete.

### Internal: `watch_animations()`

In `hooks/animation/use_animation.rs` (react-aria's `useAnimation`). Calls `on_enter`/`on_exit`, then uses
`Element.getAnimations()` + `Promise.all(animation.finished)` over the running animations on the document timeline
(CSS animations and transitions, Web Animations) to detect completion; it returns a cancel function. This is a Web
Animations API feature — during SSR, no API calls are made; enter reports `false`, exit follows `is_open` directly.

### Usage Pattern (Overlays)

Exit animations require the element to stay mounted during the animation:

```rust
let exit = use_exit_animation(UseExitAnimationInput {
    element,
    is_open: is_open.into(),
    on_exit: None,
});

// Keep element mounted until exit animation completes
let is_mounted = Signal::derive(move || {
    is_open.get() || exit.exit_state.get() == ExitState::Exiting
});
```

**Reference**: `hooks/animation/use_enter_animation.rs`, `hooks/animation/use_exit_animation.rs`

## Localized Strings

Hooks get their texts (labels, descriptions, live announcements) from react-aria's message bundles, in its 34
locales (`utils::intl_strings`). This replaces `@internationalized/string` and `useLocalizedStringFormatter`.

- **Generated tables:** `scripts/port-intl-strings.py` converts upstream's `intl/<locale>.json` files into
  `utils/intl_strings/bundles/<family>.rs`. Each file holds the messages (en-US always, the other locales behind
  the `intl-strings` feature, on by default) and a typed struct with one method per message (`TableStrings`,
  `DndStrings`, ...). Never edit the generated files; rerun the script after upstream changes the bundles. Its
  `FIXES` table repairs upstream translation errors. A new family is a line in `FAMILIES`.
- **Typed methods:** a message's arguments become parameters: `&str` for `{name}`, `usize` for a plural, `bool` for
  a `select` on `true`/`other`. A message with two arguments of one type takes a generated args struct
  (`strings.insert_between(InsertBetweenArgs { before_item_text, after_item_text })`). Positional parameters of one
  type would swap silently if upstream reordered the message.
- **Use in a hook:** `let strings = use_localized_strings::<TableStrings>();` is a `Memo` that follows the locale
  (`I18nProvider`). Read it where the text is built (`strings.read().ascending_sort(&column)`), so the text follows
  locale changes. A hook input that overrides a text (`aria_label`, ...) wins over the bundle.
- **Formatting:** at runtime, by a small ICU MessageFormat formatter (`{arg}`, `plural` with `=N` and CLDR categories
  from `icu_plurals`, `#` as the locale-formatted count, `select`, apostrophe quoting). Upstream compiles the
  messages to JavaScript at build time; the results are the same. A locale missing from a bundle falls back to its
  language (`fr-CA` → `fr-FR`), then to en-US. A test parses every message of every locale and checks that it uses
  the en-US message's arguments.

## Form Validation Hooks

Three cooperating hooks handle form validation, composed internally by the field hooks (text field, number field,
checkbox, radio group, select, combo box, date field, ...).

### `use_form_validation_state`

State management for multiple validation sources:

- `is_invalid` (OR-ed with the rest), server errors (via `FormValidationContext`, the `Form` atom's
  `validation_errors`, under the field's `names`), client-side `validate` functions, the field's own validation
  (`builtin_validation`, e.g. a date outside its range), and native HTML5 validity (read back by
  `use_form_validation`).
- `ValidationBehavior::Aria`: errors show in realtime through ARIA attributes, the form submits anyway.
- `ValidationBehavior::Native`: errors show on commit (form submission, `change`), and the field's custom validity
  (`setCustomValidity()`) blocks submission.
- Returns `FormValidationState`, a `Copy` state struct (C3): the signals `realtime_validation` and
  `display_validation` (`ValidationResult`: `is_invalid`, `validation_errors`, `validation_details` as a
  `ValidityStateSnapshot`), `is_invalid` and `validation_errors` (of the displayed result), and the methods
  `update_validation`, `reset_validation` and `commit_validation`.

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
let validation = use_form_validation_state(UseFormValidationStateInput {
    is_invalid,
    value: state.value,
    validate,
    builtin_validation: Signal::default(),
    validation_behavior,
    names: name.clone().into_iter().collect(),
});
use_form_reset(UseFormResetInput {
    element,
    initial_value: state.value.get_untracked(),
    on_reset: Callback::new(move |value| state.set_value(value)),
});
use_form_validation(UseFormValidationInput {
    element,
    state: validation,
    validation_behavior,
    focus: None,
});
```

**Reference**: `hooks/form/use_form_validation_state.rs`, `hooks/form/use_text_field.rs`

## Reference Implementations

| Pattern                           | File                                                                                                         |
|-----------------------------------|--------------------------------------------------------------------------------------------------------------|
| API conventions (state, input)    | `leptonic/src/hooks/tabs/use_tab_list_state.rs`, `leptonic/src/hooks/table/use_table_column_resize_state.rs` |
| Hook composition, attrs           | `leptonic/src/hooks/button/use_button.rs`                                                                    |
| Props pattern, mergeable handlers | `leptonic/src/hooks/interactions/use_press.rs`                                                               |
| Id reference lists (`IdRefs`)     | `leptonic/src/utils/id_refs.rs`                                                                              |
| CustomAttr                        | `leptonic/src/hooks/focus/use_focus_ring.rs`                                                                 |
| Element from another hook         | `leptonic/src/hooks/overlay/use_overlay_position.rs`                                                         |
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

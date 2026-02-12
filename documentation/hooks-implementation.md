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
    let UseButtonReturn { props, .. } = use_button(UseButtonInput { .. });
    view! { <div {..props.into_attrs()}>"Press me"</div> }
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

`*Input` types should derive `Debug` and `Clone`. They may implement `Copy` but do not have to.
`*Props` and `*Return` types should derive `Debug` but **not** `Clone`. Props are designed to bind a hook to exactly
one DOM element; making them non-Clone enforces single-use at compile time. Only `into_attrs(self)` (consuming) is
provided for conversion.

(Use `Signal<T>` for reactive values (accepts constants via `.into()`, derived signals, or existing signals for `*Input`
fields.)

Hooks receive their input as a single parameter of `*Input`.

Hooks return data as a single `*Return` return value.

A `*Return` value has:

- At least one `props: *Props` member. (more when returning props for multiple elements.)
- Additional fields of type `Signal` that provide reactive access to data.
- Additional fields of type `Callback` that give users additional programmatic control.

Hooks expose (through `*Props`):

- their attributes as `Signal`s
- their event handlers as `EventHandler<E>`s
- special attributes like `ElementCaptureAttr`

Every `*Props` type support conversion to the `*Attrs` type.

The `*Attrs` type is a type alias for a tuple declaring all `Attr` and `On` members (Leptos types). This tuple type
represents the type spreadable via Leptos's spread syntax: `{..props.into_attrs()}`.

Returning `props: *Props` (convertible to *Attrs), instead of directly returning `attrs: *Attrs`, gives us the
possibility to merge props returned by different hooks, as the values used in *Attrs are not necessarily destructable
after construction. Having raw access (owned, when possible) to signals and event handlers "making up the heart of a
hook" allows for easy programmatic merges. Merges of different hook *Return types must be implemented explicitly though.
We do not support a react-aria like generic `mergeProps` function.

Most hooks support a `disabled` input. This should always be a `Signal<bool>` for reactive enabling/disabling.

## Input Destructuring

Hooks must destructure their `*Input` parameter at the very top of the function body.
This makes all available inputs visible at a glance and avoids `input.field` access scattered throughout the body.
It gives us compile-time safety while preserving our "single parameter input" concept.

```rust
pub fn use_foo(input: UseFooInput) -> UseFooReturn {
    let UseFooInput { disabled, on_change, value } = input;
    // ... rest of hook body uses `disabled`, `on_change`, `value` directly
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

## Merging Props from Multiple Hooks

When composing multiple hooks (e.g., `use_press` + `use_hover` + `use_focus_ring`), their Props need to be merged.
The `MergeWith` trait provides a type-safe way to combine Props from different hooks.

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

// Merge press and hover props
let combined = press.props.merge_with(hover.props);

view! {
    <button {..combined.into_attrs()}>
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

let combined = press.props
.merge_with(hover.props)
.merge_with(focus_ring.props);

view! {
    <button {..combined.into_attrs()}>
        "Interactive button"
    </button>
}
```

### Available Merged Types

Pre-defined merged types in `leptonic::hooks::merged`:

| Merged Type                      | Source Hooks                          |
|----------------------------------|---------------------------------------|
| `MergedPressHoverProps`          | `UsePressProps` + `UseHoverProps`     |
| `MergedPressFocusRingProps`      | `UsePressProps` + `UseFocusRingProps` |
| `MergedHoverFocusRingProps`      | `UseHoverProps` + `UseFocusRingProps` |
| `MergedPressHoverFocusRingProps` | All three combined                    |

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

impl UseFooProps {
    /// Convert to spreadable attributes for Leptos views, consuming self.
    pub fn into_attrs(self) -> UseFooAttrs {
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
    let UseFooInput { disabled, .. } = input;

    // hook logic...

    let handle_keydown = move |e: KeyboardEvent| {
        // ...
    };

    UseFooReturn {
        props: UseFooProps {
            is_disabled: disabled,
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
    data_focus_visible: Signal<&'static str>,
}

impl UseFooProps {
    pub fn into_attrs(self) -> UseFooAttrs {
        (
            custom_attribute("data-focus-visible", self.data_focus_visible)
        )
    }
}

fn use_foo() -> UseFooReturn {
    let data_focus_visible = Signal::derive(move || {
        if is_focus_visible.get() { Some("true") } else { None }
    });

    UseFooReturn {
        props: UseFooProps {
            data_focus_visible
        }
    }
}
```

### ARIA Attribute Types

ARIA attributes must use string types (`&'static str`, `Option<&'static str>`, or custom enums from `utils/aria.rs`),
**never `bool`**.

**Why:** Leptos renders `bool` values with standard HTML boolean-attribute semantics (attribute present when `true`,
absent when `false`). ARIA attributes require explicit string values like `"true"` or `"false"` per the
[WAI-ARIA spec](https://www.w3.org/TR/wai-aria-1.2/). Using `bool` causes the attribute to silently not render in
the DOM — the code compiles, no runtime warning is logged, and the accessibility attribute is simply missing.

**Pattern for reactive boolean ARIA attributes:**

```rust
// WRONG: aria-disabled will silently not render
pub aria_disabled: Signal<bool>,
// CORRECT: renders as aria-disabled="true" or aria-disabled="false"
pub aria_disabled: Signal< & 'static str>,

// Derive from a bool signal:
let aria_disabled = Signal::derive( move | | if is_disabled.get() { "true" } else { "false" });
```

**Pattern for optional ARIA attributes** (attribute absent when not applicable):

```rust
// Use Option to suppress the attribute entirely when None
pub aria_selected: Signal<Option< & 'static str> >,
let aria_selected = Signal::derive(move | | {
if selection_mode == SelectionMode::None {
None                // attribute absent from DOM
} else if is_selected.get() {
Some("true")        // aria-selected="true"
} else {
Some("false")       // aria-selected="false"
}
});
```

### Element Capture Pattern

Many hooks cannot solely rely on returning spreadable props. They often need direct programmatic access to DOM
element, e.g. for focus management or dimension retrieval.

Hooks should NOT require users to manually declare and bind `NodeRef`s to the element to which props are spread as well.

Hooks should use `CapturedElement`, which bundles a `StoredValue` for the DOM element with a `Trigger` for reactive
tracking. This ensures that Effects reading the element via `CapturedElement::get()` automatically re-run when the
element is captured — which is critical when the element lives inside a reactive boundary like `<Show>` or is rendered
during client-side navigation:

```rust
use leptonic::utils::element_capture::{CapturedElement, ElementCaptureAttr};

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

## Based On React-Aria

Most hooks are loosely based on hooks from Adobe's `react-aria` library (part of `react-spectrum`), checked out at
`{leptonic_root_dir}/../react-spectrum`.

### React-Aria Deviations

Wherever we decided to deviate from the react-aria implementation of a hook, document these intentional differences
at the top of hook files:

```rust
// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## OMITTED FEATURES
// - `featureName`: [Rationale]. React-aria: [comparison].
//
// ## LEPTOS-SPECIFIC ADAPTATIONS
// - [Change]: [Rationale]. React-aria: [comparison].
//
// =============================================================================
```

| Category                      | Use When                              |
|-------------------------------|---------------------------------------|
| `OMITTED FEATURES`            | Feature intentionally not implemented |
| `DIFFERENT BEHAVIOR`          | Same feature, different approach      |
| `LEPTOS-SPECIFIC ADAPTATIONS` | Changes required by Rust/Leptos       |
| `API DIFFERENCES`             | Naming or structural changes          |

Global deviations go in `leptonic/src/hooks/mod.rs`.

## Book-SSR Documentation Pages

Each hook needs a page in `examples/book-ssr/src/pages/documentation/hooks/`:

```rust
#[component]
pub fn PageUseHookName() -> impl IntoView {
    let UseHookReturn { attrs, .. } = use_hook(UseHookInput { .. });

    view! {
        <Article>
            <h1 id="use_hook" class="anchor">
                "use_hook"
                <AnchorLink href="#use_hook" description="Direct link"/>
            </h1>
            <p>"Description"</p>
            <Code>{indoc!(r#"..."#)}</Code>
            <div {..attrs}>"Demo"</div>
        </Article>
        <Toc toc=Toc::List { inner: vec![Toc::Leaf { title: "use_hook", link: "#use_hook" }] }/>
    }
}
```

Register in `hooks/mod.rs`, add route definition in `lib.rs`, routing in `app.rs` and menu item in
`pages/documentation/doc_layout.rs`.

**Reference**: `use_press` in `leptonic/src/hooks/interactions/use_press.rs`

## Reference Implementations

| Pattern                           | File                                                 |
|-----------------------------------|------------------------------------------------------|
| Hook composition, attrs           | `leptonic/src/hooks/button.rs`                       |
| Props pattern, mergeable handlers | `leptonic/src/hooks/interactions/use_press.rs`       |
| Props merging (`MergeWith`)       | `leptonic/src/hooks/merged/mod.rs`                   |
| CustomAttr                        | `leptonic/src/hooks/focus/use_focus_ring.rs`         |
| Element reference                 | `leptonic/src/hooks/overlay/use_overlay_position.rs` |
| Element capture                   | `leptonic/src/hooks/menu/use_menu_item.rs`           |
| Dynamic listeners, drag           | `leptonic/src/hooks/interactions/use_press.rs`       |
| Slider drag                       | `leptonic/src/hooks/form/use_slider.rs`              |

# Leptos and DOM Rules

Rules for any code that runs in Leptos and the browser: hooks, atoms and their helpers. Each comes from a bug we
had; the pitfalls behind them are in `lessons.md`.

## Global State and SSR

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
- `StoredValue::new_local(None)` is still thread-bound: its storage wraps even the empty value in a
  `SendWrapper`, whose drop on another SSR worker panics. Gate local storage to client-only code, or use
  `StoredValue<Option<SendWrapper<DomType>>>` with ordinary storage and wrap only DOM values acquired in the
  browser. The empty SSR value can then be disposed on any worker.
- Immutable caches whose content doesn't depend on the request (syntax sets, ICU data) may be global.

## Hydration and Captured Elements

The first client attribute value must match the server's value: hydration may cache it without writing to the DOM.
Element capture runs during hydration, so a preceding slot can already be captured when its control hydrates.
Publish optional slot references through an `Effect` after hydration (`utils::slot_id::use_slot`), keeping their
initial value `None` on both sides. A derived signal over capture alone makes references depend on element order.

## Effect Read Order

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

**Reference**: `use_combobox_state`'s reconciliation effect; the browser test case
`value_and_items_derived_from_one_signal` in `leptonic/tests/ui_tests/test_combobox.rs` (agnite dev-ui's log source
picker: value and items derived from one memo, with a filter).

## Derived Values

`Signal::derive` recomputes on every read and notifies on every upstream change. A derived value that several
readers read, that allocates (clones a set, builds a formatter, parses), or that feeds effects is a `Memo`: it
computes once per change and notifies only when its value changed. Objects that are expensive to build (ICU4X
formatters, collators, plural rules) are built once per locale, not per read. (Review 2026-10-09: form validation,
number field values, date validation and tab keys were `Signal::derive` read many times per keystroke.)

## Blur After Disposal

Removing a focused element (a dismiss button removing its chip, a clear button hiding itself) makes the browser
fire `blur`/`focusout` on it while Leptos unmounts it, after its owner and the callbacks and signals it owned were
disposed. Blur and focus-out paths therefore run callbacks with `try_run` and read signals with `try_get_untracked`:
nothing is left to notify then. Native drag events are the other exception: Chromium keeps sending `drag`/`dragend`
to a drag source that was removed (an item moved to another list, a virtualized row scrolled away), so `use_drag`'s
handlers use `try_*` accessors too and guard against ending twice. (Other events can't reach a removed element, so
they keep `run`.)

## Bubbling Focus Events and Untracked Reads in Handlers

React's `onFocus`/`onBlur` bubble: they are `focusin`/`focusout`. A ported handler that must see focus moving to or
within descendants (React's `onFocus` on a container, e.g. a grid list row whose child is clicked) listens on
`focusin`/`focusout`; `ev::focus`/`ev::blur` fit only where upstream acts on `target == currentTarget` anyway (most
focusable elements' own handlers). Event
handlers read signals with `get_untracked`/`with_untracked` (a tracked read outside a reactive context makes Leptos
warn, which fails the browser tests' page health), and app callbacks called from event paths (`get_drop_operation`)
run in `untrack`, as react-aria's never subscribe.

## Shadow DOM, Re-Entrancy and Optional Features

- DOM queries go through the shadow-aware helpers of `utils::shadow_dom` (`get_event_target`, `get_active_element`,
  `node_contains`), as react-aria's `getEventTarget`/`getActiveElement`/`nodeContains`, never
  `document.active_element()`, `Node::contains` or a raw `e.target()`.
- User callbacks never run while the hook's own state is borrowed (`StoredValue::update_value`, a held lock): a
  callback may call back into the hook and would panic. Take what the callback needs, release, then call it.
- A feature group the caller didn't ask for costs nothing: `long_press: Option<LongPress>` set to `None` sets up no
  listeners, Effects or `use_description`; empty event handlers attach no DOM listener.

**Reference**: `use_focus`, `use_focus_within`, `use_focus_ring`; `utils::owner_alive::OwnerAlive` for deferred
callbacks (timeouts, animation frames, global listeners) that may run after disposal.

## Nested Event Dispatch

Leptos's standard `on:` attributes and `leptos_use::use_event_listener` wrap handlers in wasm-bindgen `FnMut`
closures. They cannot be re-entered: a handler that synchronously dispatches an event back to itself throws
"closure invoked recursively or after being dropped" before the nested Rust handler runs. For example,
`element.focus()` inside a `focusin` handler can dispatch another `focusin`.

Hook props use `EventHandler::into_on` and its `OnEvent` attribute. It installs a native `Fn` listener through
`utils::event_listeners`, allowing synchronous re-entry so that the hook's own guards can apply. This matters for
`use_press`: a press callback may call `element.click()`, and its re-entry guard must run before any second press.
Wrapping the handler in a standard Leptos `on:` closure loses this property. The attribute runs callbacks in the
captured reactive owner, removes its listener when its mounted state is dropped, and attaches nothing for empty
handlers.

When a path still uses a standard `FnMut` listener, move recursive dispatch out of the handler with `queue_microtask`
(wrap DOM values in `SendWrapper`) and explain why in a comment. Browser tests check uncaught page errors as well as
the resulting state, so an apparently correct callback log does not conceal a failed nested dispatch.

**Reference**: `use_selectable_collection` (`on_focusin`), `FocusScope` (focus containment)

## Dynamic Event Listeners

Listeners that come and go with an interaction (on the document between pointer down and pointer up; react-aria's
`useGlobalListeners`) are `utils::event_listeners::Listener`s: `listen_to(&target, ev::pointerup, capture, handler)`
adds one, dropping the `Listener` removes it. Keep them in the interaction's state, so that ending the interaction
(dropping the state) removes them. Unlike `leptos_use::use_event_listener`, a `Listener` registers nothing with the
reactive owner (one per interaction doesn't pile up cleanups until the component unmounts), and its handler is an `Fn`
closure, which can be re-entered. Listeners that live as long as the hook (set up once in its body, e.g.
`use_interact_outside`'s document listeners) use `leptos_use::use_event_listener`.

```rust
use crate::utils::event_listeners::{Listener, listen_to};

fn use_foo() {
    /// The drag in progress.
    struct DragState {
        pointer_id: i32,
        /// Removed when the state is dropped.
        _listeners: Vec<Listener>,
    }
    let state = StoredValue::new_local(None::<DragState>);

    let on_pointer_move = move |e: PointerEvent| { /* ... */ };
    let on_pointer_up = move |e: PointerEvent| {
        if state.with_value(|s| s.as_ref().is_some_and(|s| s.pointer_id == e.pointer_id())) {
            // Dropping the state removes the document listeners.
            state.set_value(None);
            // other logic...
        }
    };

    let handle_pointer_down = move |e: PointerEvent| {
        // Prefer an event-target based "owner document" over the global document: it supports
        // iframes and shadow DOMs.
        let Some(doc) = e.expect_current_target().get_owner_document() else {
            return;
        };
        state.set_value(Some(DragState {
            pointer_id: e.pointer_id(),
            _listeners: vec![
                listen_to(&doc, ev::pointermove, false, on_pointer_move),
                listen_to(&doc, ev::pointerup, false, on_pointer_up),
                listen_to(&doc, ev::pointercancel, false, on_pointer_up),
            ],
        }));
    };
}
```

Always prefer using `get_owner_document()` from `crate::utils::EventTargetExt`.

Document "why", when deviating from this recommendation.

**Reference**: `use_press`, `use_move`, `use_slider_thumb`

# Leptos and DOM Rules

Rules for any code that runs in Leptos and the browser: hooks, atoms and their helpers. Each comes from a bug we
had; more pitfalls in `lessons.md`.

## Global State and SSR

Page-wide state (the visible overlays stack, the focus scope tree, description elements, the prevent-scroll count,
...) lives in `thread_local!`s. That is only sound in the browser, where one page runs on one thread. On the server,
axum's work-stealing runtime interleaves requests on a thread and moves a request's render between threads at every
`.await` (Suspense, streaming), so a thread-local reached while rendering mixes requests and loses state.

- Touch thread-locals (and `static` atomics or locks) only from client-only code: `Effect`s, event handlers,
  `request_animation_frame`/timeouts, or code behind `#[cfg(not(feature = "ssr"))]`. Component and hook bodies and
  `on_cleanup` run on the server: from there, only release what an effect acquired (tracked in a `StoredValue`, which
  stays empty on the server).
- Per-request state goes into the reactive owner: `provide_context` at a root, or Leptos' shared context (as `use_id`
  does for ids).
- Local storage is thread-bound: `StoredValue::new_local(..)` wraps even an empty value in a `SendWrapper`, whose drop
  on another SSR worker panics, and so does an `Effect` whose owner is dropped on another thread. Gate local storage
  to client-only code, or use `StoredValue<Option<SendWrapper<DomType>>>` with ordinary storage and wrap only DOM
  values acquired in the browser.
- Immutable caches whose content doesn't depend on the request (syntax sets, ICU4X data) may be global.
- JavaScript calls (`js_sys::Date::now()`, ...) panic outside WebAssembly, also in native unit tests: gate them on
  `target_arch = "wasm32"`, not on `not(feature = "ssr")`.

## Hydration

Server and client must derive the same initial state and attributes while rendering:

- The first client attribute value must match the server's: hydration may cache it without writing to the DOM.
  Element capture runs during hydration, so a preceding slot can already be captured when its control hydrates:
  publish optional slot references through an `Effect` after hydration (`utils::slot_id::use_slot`), `None` initially
  on both sides.
- Ids come from `use_id` (`hooks-implementation.md`, "Element Ids"), never from random values.
- Compute defaults while rendering, deterministically: `HashSet` iteration order differs between the server and the
  client (std's seeded hasher), so "the first default key" is the first given or the smallest; a default decided in a
  client `Effect` (a disabled first tab skipped) renders different server HTML.
- Platform checks (`is_ios()`, ...) are `false` on the server, and hydration keeps the server's attributes: decide
  platform-dependent attributes reactively after hydration (`use_platform_check`), never once at hook creation.
- "The last item to register" is not hydration-safe (`For` hydrates each row before the next registers; the server
  registers all first): pass such facts as props (`Breadcrumb`'s `is_current`).

## Effect Read Order

An effect reading several memos must read upstream ones before memos derived from them (a collection before its
filtered memo). reactive_graph 0.2 checks an effect's sources in read order, with the effect as observer: a memo that
recomputes while a downstream memo is being checked doesn't mark the effect dirty, and when the effect checks it
itself, it is already clean. If the downstream memo came out unchanged, the effect doesn't run, although an upstream
value it reads changed (Leptos issue draft `a-effect-skips-run`, `PLAN.md`). App values often reach a hook through
memos (a value and a collection derived from one app memo), so this applies to every reconciling effect.

So: read the app's inputs (bindings, collections, signals passed in) first, then the hook's own memos; read every
source up front rather than only in some branches. When one input is an app memo and another a derived signal of the
same upstream memo, no order helps: read the inputs through hook-local memos, so the effect isn't a direct subscriber
of an app memo (the skip only spares direct subscribers).

**Reference**: `use_combobox_state`'s reconciliation effect; browser case `value_and_items_derived_from_one_signal`
(`tests/ui_tests/test_combobox.rs`).

## Effect Timing

Leptos `Effect`s run after a `MutationObserver`'s callback (React's layout effects run before it), and render effects
run before `Effect::new` (React commits attributes before `useEffect`), so a ported effect may observe a different DOM
than upstream's. Check the order when porting layout effects (a modal's `aria_hide_outside` made overlays opened from
inside it inert before they registered: `keep_visible` reveals them again).

Synchronous guards don't port to effects: react-aria's `ScrollView` ignores the content size changes its own size
update causes with a flag (`isUpdatingSize`), which works because `flushSync` lays out while the flag holds. In
Leptos those passes run later (microtasks, the next frame's resize observer callbacks), when the flag is cleared; a
feedback loop between them (a scrollbar appearing and disappearing) then never yields to the browser and the page
freezes. Bound such an update by its number of passes instead (`use_scroll_view`).

## Derived Values

`Signal::derive` recomputes on every read and notifies on every upstream change. A derived value that several readers
read, that allocates (clones a set, builds a formatter, parses), or that feeds effects is a `Memo`: it computes once
per change and notifies only when its value changed. Objects expensive to build (ICU4X formatters, collators, plural
rules) are built once per locale (`utils::recent_cache::RecentCache`), not per read.

## Blur After Disposal

Removing a focused element (a dismiss button removing its chip, a clear button hiding itself) makes the browser fire
`blur`/`focusout` on it while Leptos unmounts it, after its owner and the callbacks and signals it owned were
disposed. Blur and focus-out paths therefore run callbacks with `try_run` and read signals with `try_get_untracked`.
Native drag events are the other exception: Chromium keeps sending `drag`/`dragend` to a removed drag source, so
`use_drag`'s handlers use `try_*` accessors too and guard against ending twice. Other events can't reach a removed
element and keep `run`.

Deferred callbacks (timeouts, animation frames, global listeners) check `utils::owner_alive::OwnerAlive` before
touching the owner's signals, and may hold another owner's reactive values only while that owner provably lives (the
item registry keeps every live registration per key, removed by its own cleanup). Getters of a disposed `NodeRef`
treat it as having no element.

## Focus Events, Untracked Reads and Callbacks in Handlers

- React's `onFocus`/`onBlur` bubble: they are `focusin`/`focusout`. A ported handler that must see focus moving to or
  within descendants (a grid list row whose child is clicked) listens on `focusin`/`focusout`; `ev::focus`/`ev::blur`
  fit only where upstream acts on `target == currentTarget` anyway.
- Event handlers read signals with `get_untracked`/`with_untracked` (a tracked read outside a reactive context makes
  Leptos warn, which fails the browser tests' page health); app callbacks called from event paths
  (`get_drop_operation`) run in `untrack`, as react-aria's never subscribe.
- User callbacks never run while the hook's own state is borrowed (`StoredValue::update_value`, a held lock): a
  callback may call back into the hook and would panic. Take what it needs, release, then call it.
- DOM queries go through the shadow-aware helpers of `utils::shadow_dom` (`get_event_target`, `get_active_element`,
  `node_contains`), as react-aria's `getEventTarget`/`getActiveElement`/`nodeContains`, never
  `document.active_element()`, `Node::contains` or a raw `e.target()`.
- A feature group the caller didn't ask for costs nothing: `long_press: None` sets up no listeners, Effects or
  `use_description`; empty event handlers attach no DOM listener.

## Nested Event Dispatch

Leptos' `on:` attributes and `leptos_use::use_event_listener` wrap handlers in wasm-bindgen `FnMut` closures, which
can't be re-entered: a handler that synchronously dispatches an event back to itself (`element.focus()` inside a
`focusin` handler) throws "closure invoked recursively or after being dropped". Leptos event delegation is off, so
every `on:` handler is such a closure.

Hook props therefore attach through `EventHandler::into_on` (`OnEvent`), a native `Fn` listener installed through
`utils::event_listeners`: re-entrant, so the hook's own guards apply (a press callback calling `element.click()` must
reach `use_press`'s re-entry guard). It runs callbacks in the captured owner, removes its listener when unmounted and
attaches nothing for empty handlers. Where a path still uses an `FnMut` listener, move recursive dispatch out of the
handler with `queue_microtask` and say why in a comment.

**Reference**: `use_selectable_collection` (`on_focusin`), `FocusScope` (focus containment)

## Dynamic Event Listeners

Listeners that come and go with an interaction (on the document between pointer down and up; react-aria's
`useGlobalListeners`) are `utils::event_listeners::Listener`s: `listen_to(&target, ev::pointerup, capture, handler)`
adds one, dropping it removes it. Keep them in the interaction's state, so that ending the interaction removes them.
A `Listener` registers nothing with the reactive owner (cleanups don't pile up until unmount) and its handler is
re-entrant. Attach them to the event target's owner document (`EventTargetExt::get_owner_document`, for iframes and
shadow DOM), not the global one. Listeners living as long as the hook use `leptos_use::use_event_listener`.

```rust
struct DragState {
    pointer_id: i32,
    _listeners: Vec<Listener>, // removed when the state is dropped
}
let state = StoredValue::new_local(None::<DragState>);
let handle_pointer_down = move |e: PointerEvent| {
    let Some(doc) = e.expect_current_target().get_owner_document() else { return };
    state.set_value(Some(DragState {
        pointer_id: e.pointer_id(),
        _listeners: vec![
            listen_to(&doc, ev::pointermove, false, on_pointer_move),
            listen_to(&doc, ev::pointerup, false, on_pointer_up), // sets the state to `None`
        ],
    }));
};
```

**Reference**: `use_press`, `use_move`, `use_slider_thumb`

## Resize Observers

The browser delivers `ResizeObserver` observations in passes, each deeper in the DOM than the last. Work in a callback
that resizes an element not deeper than an observed one (repositioning a portaled overlay; a measured size the
overlay is styled with) can't be delivered in that pass: the browser fires "ResizeObserver loop completed with
undelivered notifications" as an uncaught `error` (failing page health, and visible in apps' error monitoring). An
Effect notified from the callback runs in a microtask, still inside the delivery. Such work goes to the next
animation frame, coalesced, with `utils::next_frame::NextFrame` (cancelled on cleanup); starting to observe an element
from inside a delivery is deferred the same way. Work that follows from opening or a changed input stays immediate.

**Reference**: `use_overlay_position`, `Popover` (`--trigger-width`), `use_virtualizer_item` (observing new rows)

## Classes and Styles

Styling props use the `leptos-classes` (`Classes`) and `leptos-styles` (`Styles`) crates. `Styles` takes typed
declarations only (`WidthProperty.declare(..)`, typed custom properties through `css_custom_property!`); properties
leptos-css doesn't cover yet (`position`, `transform`, `display`, ...) go through the explicit `*_unchecked` methods,
values computed at runtime through the non-panicking helpers in `utils/styles.rs` (`css::computed_pct`,
`computed_px`, `computed_size`).

- **One writer of `class`:** never `class=classes` together with `class:foo=` on one element; use
  `Classes::add_reactive`.
- **One writer of `style`:** a reactive `Styles` rewrites the whole `style` attribute whenever one of its reactive
  parts changes, so a property set on the element directly (`style.set_property`) is gone after the next change of
  any other part (a popover's hide-until-placed styles were wiped as soon as its `--trigger-width` arrived). Hooks
  return such styles (`PropsWithStyles`, `UseEnterAnimationReturn::styles`) for the atom to merge into its `Styles`;
  a hook writes the style directly only for values its returned `Styles` hold as well (`use_overlay_position`).
- **`style=styles` owns the `style` attribute:** the server merges `style:prop=` into the same attribute, but
  hydration (and every reactive change) writes the `Styles` string whole, so a `style:` directive (or `style=`,
  `attr:style`) on the same element is lost, even with static `Styles`. On atoms, pass `styles=`, never `style:`
  (verified in the browser, 2026-10-16).

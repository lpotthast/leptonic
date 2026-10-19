# Performance: what Leptos costs, and how to write cheap hooks and atoms

Allocation count first, then allocated bytes; wasm size before speed (`conventions.md`). The numbers come from the
sources of the Leptos the test-app runs (the user's fork at `~/dev/leptos`, branch `leptonic-metrics` = Leptos
0.8.22: `reactive_graph` 0.2.15, `tachys` 0.2.19, `leptos_macro` 0.8.19; the registry crates are identical except
for the counters), from a native probe with a counting allocator against the fork (2026-10-16; counts are the same
on wasm, bytes below are x86_64, roughly half on wasm32), from wasm32 size probes at `opt-level = "z"`, and from the
budgets (`leptonic/tests/budgets/`). Source references are relative to `~/dev/leptos/reactive_graph/src/` in the
reactive tables and to `~/dev/leptos/tachys/src/` in the view table, else prefixed with their crate.

Every build of this repository uses `--cfg=erase_components` (`.cargo/config.toml`, and the advice to users in
`build-performance.md`), so the budgets measure the erased view layer: the "erased" columns are the production ones.

## 1. Cost model

### Reactive primitives

| Construct | Create (allocations, bytes) | Read | Notes and source |
|---|---|---|---|
| any arena item (`RwSignal`, `Memo`, `Signal`, `StoredValue`, `Callback`, `Effect`, ...) | +1 `Box` in the global slot map, and a push to the owner's node `Vec` | slot-map lookup + downcast | freed only when the owner is cleaned up (rerun or dispose), not when the handle is dropped: `owner/arena_item.rs:47` |
| `Owner::new()` / `owner.child()` | 1 (~150 B) + push to the parent's `children` | | every memo and every effect has its own: `owner.rs:148`, `owner.rs:240` |
| `RwSignal::new` (`_local`) | 5 (~260 B): value `Arc`, subscriber-set `Arc`, `IndexSet::with_capacity(2)` = 2, arena box | `.get()` clones, `.with()`/`.read()` 0 | `signal/arc_rw.rs:76`, `graph/sets.rs:67` |
| `ArcRwSignal::new` | 4 | | |
| `signal()` | 6 (two arena items) | | `signal.rs:115`; `read_only()`, `write_only()`, `.into()` to `Signal`: +1 each |
| `Signal::stored(v)`, `Signal::default()`, `Signal::from(v)` | 2 | 0, never tracks | an absent `#[prop(optional)] Signal<T>` and every `Signal` field in a `Default` cost this: `wrappers.rs:654`, `wrappers.rs:705` |
| `Signal::from(ReadSignal/Memo/ArcRwSignal)` / `Signal::from(RwSignal)` | 1 / 2 (`RwSignal` goes through an extra `read_only()` arena item) | delegates | `wrappers.rs:853`; passing an `RwSignal` to a `#[prop(into)] Signal<T>` costs 2 |
| `Signal::derive(f)` | 2 (closure `Arc`, arena box) | calls `f` (0 allocations), tracks `f`'s sources directly, recomputes per read | `wrappers.rs:631` |
| `MaybeProp::default()` / from a value or signal | 0 / 2 | `Option` clone | `wrappers.rs:1730` |
| leptonic `MaybeSignal`: `default()`, from a value / from a signal | 0 / as `Signal::from` | fixed: 0, never tracks, `with` doesn't clone | `leptonic/src/utils/maybe_signal.rs` |
| `StoredValue::new` (`_local`) / `ArcStoredValue` | 2 / 1 | 0 | `owner/stored_value.rs:85` |
| `Trigger::new` / `ArcTrigger::new` | 2 / 1 (empty subscriber set) | `track()` | `signal/arc_trigger.rs:24` |
| `Memo::new` / `ArcMemo::new` | 7-8 (~600 B) / 6, lazy | **every read +2** (it clones its source set to check staleness, even when clean: `computed/inner.rs:108`); first computation +1-3 | value `Arc`, fun `Arc`, inner `Arc`, `Owner`, subscriber set 2: `computed/arc_memo.rs:154` |
| `Memo::new_owning` | as `Memo::new` | | gets the previous value by value: reuse its buffer (`Vec`, `String`) instead of allocating a new one |
| `Callback::new` / `UnsyncCallback::new` | 3, also for non-capturing closures (`Arc<dyn Fn>` in a `StoredValue`) | `run`: 0 | `callback.rs:212`; a plain `Arc<dyn Fn>` (leptonic's `EventHandler::new`) is 1 |
| `on_cleanup(f)` | 1 box (0 for a zero-sized closure) + push | | `owner.rs:312`; registered again on every rerun |
| `provide_context(v)` | 1 box (0 for ZSTs); the owner's first context +1 (map), map regrows at 4, 8, ... types | | `owner/context.rs:9` |
| `use_context::<T>()` | 0 | walks owners up to the provider (or the root when absent), one lock per level; clones `T` | `owner/context.rs:46` |
| `RenderEffect::new` | 7 without sources, ~10 with one: boxed fn, channel `Arc`, `Owner`, inner `Arc`, value `Arc`, 3 for the task (`any_spawner` `Box::pin`, js-sys `Rc<Task>` + `Box::pin`), source set 2 on first track | first run at creation | `effect/render_effect.rs:62`, `:147`, `:223`; `any_spawner/src/lib.rs:129`; js-sys 0.3.106 `futures/task/singlethread.rs:60` |
| `Effect::new` | 8 + first run (a microtask later) ~10 | | `effect/effect.rs:95`, `:169` |
| `Effect::watch` | ~11 | | two value `Arc`s |
| `ImmediateEffect::new` | 6 including its first run, no task; runs synchronously inside the `set` | | `effect/immediate.rs:74`; `new_scoped` +1 (`on_cleanup`) |
| `Selector::new` | 4 `Arc`s + a `RenderEffect` (isomorphic, async); per key: an `ArcRwSignal` (4) on its first `selected(&key)` | `selected`: map lookup + track | keys are never removed unless you call `remove`; a change compares every key (no allocation) and notifies the changed ones: `computed/selector.rs:86-135` |

### Reactive work

| Work | Allocations | Source |
|---|---|---|
| notify a signal or trigger (`set`, `update`, `notify`) | 1 without subscribers, 2 with (it clones its subscriber `IndexSet`) | `signal/subscriber_traits.rs:59` |
| each memo on the notification path | +2 to mark (clones its subscribers), +2 per read (clones its sources), recompute: its source set is dropped and re-tracked (+2, more above 3 sources), +2 if changed | `computed/inner.rs:69-160` |
| each effect rerun | +2 (sources dropped and re-tracked; +1-3 above 3 sources), plus everything the run creates: each `on_cleanup` +1, each signal +5, memo +8, effect +10, all disposed and recreated per run | `effect/render_effect.rs:223-247` |
| measured: signal → 1 render effect | 4 | probe |
| signal → 10 render effects | 22 (2 + 2 per effect) | probe |
| signal → memo (changed) → render effect | 16; memo unchanged: 10 | probe |
| signal → 3 memos (all unchanged) → render effect | 22 | probe |
| `.get()` of a `String`/`Vec`/`HashSet` signal | +1 per read (clone); `.with`, `.read`, `.with_untracked`, `untrack(..)`: 0 | |
| disposing an owner | 0 | `owner.rs:519` |

Per-item state of N items reading one shared value, one change of the value (probe; leptonic's equivalents:
`RadioGroup`, `CheckboxGroup`, `Tabs`, collections):

| Pattern | N = 10 | N = 1,000 |
|---|---|---|
| per item `Memo::new(sel == i)` + a render effect | 94 allocations, 10 memo runs (8 unchanged) | 8,014 allocations, 1,000 memo runs |
| per item `Signal::derive(sel == i)` + a render effect | 22 allocations, 10 effect runs | 2,002 allocations, 1,000 effect runs |
| `Selector` | 12 | 12 |
| per-key triggers (leptonic's `KeyedTriggers`) | 15 | 15 |

### View layer (tachys)

| Construct | Not erased | Erased (`erase_components`, the budgets) | Source |
|---|---|---|---|
| static attribute (`id="x"`, `Attr(K, plain value)`) | 0 (one DOM call) | ~4 per attribute: `&str` value → `Arc<str>`, `Erased` box, `keys()` `Vec` (+1 `String` for custom/`data-*` attributes), state box; plus the element's `Vec<AnyAttribute>` (grows by push) and its states `Vec` | `html/attribute/any_attribute.rs:67-162`, `value.rs:144`, `custom.rs:118`, `erased.rs` |
| reactive attribute (closure, `Signal`, `Memo`, `MaybeProp`, `flag(..)`) | one `RenderEffect` (~10) + the attribute name as `String`; every run clones the value (`move \|\| self.get()`) | + the static attribute's ~4; `view!` closures +1 (`Arc<Mutex<dyn FnMut>>`, `leptos/src/lib.rs:269`) | `reactive_graph/mod.rs:427-447`, `:789` |
| a constant as reactive attribute (`Signal::stored`, `MaybeProp` `None`, a derive over stored signals) | the full `RenderEffect` (7-8), which never reruns | same | |
| leptonic `MaybeSignal` attribute | fixed: as the static value; from a signal: as the `Signal` | same | |
| `class:name=closure` | `RenderEffect`, no name allocation | + ~4 | `reactive_graph/class.rs:120` |
| `style:name=closure` | `RenderEffect` + `Arc<str>` name | + ~4 | `reactive_graph/style.rs:30` |
| `prop:value=` | every write queues a microtask: box + JS closure | | `renderer/dom.rs:253` |
| `on:event=` (tachys) | 3: boxed handler, event name `String`, remover box | + `Rc<RefCell>` handler + ~3 | `html/event.rs:195`, `renderer/dom.rs:279-311` |
| leptonic `OnEvent` (`EventHandler::into_on`) | 2: handler `Arc` + wasm-bindgen `Closure` box; empty handlers attach nothing | inside a spread: shared | `utils/event_handler.rs`, `utils/event_listeners.rs` |
| spread `{..attrs}` of a tuple | like the same attributes inline | one `AnyAttribute` for the whole tuple (2 boxes), but `keys()` still allocates one `Vec` per `Attr` and regrows the tuple's buffer | `html/attribute/mod.rs:387` |
| spread onto a component / `AnyView` | | `AnyViewWithAttrs`: `Vec` + `elements()` `Vec` + a clone per attribute per root element | `view/any_view.rs:416`, `:687` |
| element with several children | tuple, 0 | `StaticVec<AnyView>` (`Vec`) + 2 per child | `html/element/mod.rs:148`; macro `view/mod.rs:565` |
| `into_any()` / `AnyView` | 1 box + 1 state box on build (+ a placeholder comment if it may be empty) | same | `view/any_view.rs:196` |
| `#[component]` | no owner, no allocation; props builder none | output becomes an `AnyView`: 2 | macro `component.rs:362` |
| `Children` / `ChildrenFn` | `Box<dyn FnOnce>` (1, 0 if zero-sized) + `AnyView` 2 / `Arc` | | `leptos/src/children.rs:17` |
| `<Provider>` | child owner + context (2-3) + children box + `AnyView` ≈ 6-7 | | `leptos/src/provider.rs` |
| `<Show>` | `ArcMemo` (6) + `RenderEffect` (10) + children `Arc`; a toggle rebuilds the branch | | `leptos/src/show.rs:23` |
| `<For>` | per row: an `Owner` + the row; per update: a key `IndexSet` (2), item `Vec`, diff `Vec`s, `key_fn` and a hash for every row | | `leptos/src/for_loop.rs:140`, `tachys/src/view/keyed.rs:130-160` |
| static text / dynamic text | 0 / `RenderEffect` + the formatted `String` per run | `AnyView` per child among several | |

Hydration runs the same effects and attribute states (static attributes aren't written, nodes are found instead of
created); the client wasm contains both the build and the hydrate path of every view.

**Price list per mount**, a regression over the 252 mount measurements under 3,000 allocations (R² = 0.97): about
**22 allocations per render effect run, 18 per memo run, 25 per effect run, 6 per added DOM node**. These include
what usually comes with them (the attribute boxing, the signals feeding them).

### Debug assertions

The test-app measures with debug assertions; they add 0.05% of allocations (`testing.md`). In the sources: `defined_at`
locations (bytes only; the probe's counts are identical with and without debug assertions), warnings that allocate
only when emitted (`reactive_graph/traits.rs:121`), `SpecialNonReactiveZone` (a thread-local `Cell`), tachys' element
locations and hydration error texts (formatted only on errors, `tachys/src/hydration.rs:141-230`). Timing only:
js-sys' executor calls `console.createTask` for every spawned effect task and polls through `console.run` in debug
builds (js-sys `futures/task/singlethread.rs:21-63`), so debug timings of effect-heavy pages are inflated.

### Wasm size per call site (wasm32, `opt-level = "z"`, 40 sites with distinct closures)

| Construct | Bytes per site |
|---|---|
| `Effect::watch` | 1,267 |
| `Effect::new` | 1,084 |
| `Memo::new` | 602 |
| `ImmediateEffect::new` | 446 |
| `Signal::derive` | 361 |
| `Callback::new` | 334 |
| `RenderEffect::new` | 114 (boxes the fn, generic only over the value) |
| `RwSignal::new`, `StoredValue::new`, `Signal::stored` (same `T`) | ~52 |

| Element attribute (tachys builder) | Not erased | Erased |
|---|---|---|
| inline closure | 568 | 696 |
| `on:click` closure | 721 | 842 |
| `class:x` closure | 811 | 508 |
| `flag(sig)` (one opaque type for all sites) | 57 | 30 |
| `Signal<T>` | 57 | 38 |
| static value | 61 | 33 |
| `MaybeSignal` fixed / from a signal (another probe: its plain value 63 / 80, `Signal::stored` 74 / 141; ±30) | 66 / 60 | 91 / 68 |

A `Part` (`utils/part.rs`, every props value a public hook returns) costs per props type, not per site, when built
with `Part::stored` (the vtable, `remount`, drop glue): measured 2026-10-17 on a probe of 9 hooks (`opt-level = "z"`,
no LTO) 1.3 KB raw / 275 bytes brotli per part, a quarter of it the `remount` copies; the test-app (every public
hook, ~140 parts) grew 0.7% brotli (3,119,199 → 3,141,570 bytes).

**Part costs since atoms and hooks compose the public hooks** (2026-10-17, no crate-private `use_x_raw` bodies,
`hooks-implementation.md`, "One way"): every hook's part is 1 allocation (the `Arc`) when the hook runs, also an inner
hook's part a composing hook consumes with `into_props()` (no lease cleanup; a stored part hands over its props
without a copy); a mount through `attrs()`/`props()` is 1 more (the lease's `on_cleanup`). Copying a chained
`EventHandler` for a mount is a refcount bump (a chain is one shared `Arc<[..]>`, allocated once when it is built).
So an atom pays 1 per hook in its stack plus 1-2 per element it renders: Button mount 289 → 300 allocations
(`use_button` on `use_press`, `use_context_menu`, `use_focusable` with `use_focus` and `use_keyboard`, `use_hover`,
`use_focus_ring`: 8 parts consumed, the button's own part and its lease 3), `use_button` alone 271 → 279; per element
of the large fixtures: list box option +7 (516 → 523), menu item +10, grid list item +12, tag (row, cell, remove
button) +23, checkbox +8, radio +5, calendar cell ~+10; all budgets 87,221,089 → 88,324,086 allocations (+1.3%),
effect, memo, render effect and DOM counts unchanged. Wasm: Button probe (`pilot/btnprobe`, `opt-level = "z"`, LTO)
278,772 → 281,096 bytes brotli (+0.8%; raw 1,594,573 → 1,604,171), test-app 3,140,895 → 3,143,255 (+0.08%).

`Effect::new` monomorphizes its whole async loop per closure (`effect/effect.rs:169-200`). A closure as attribute value
monomorphizes the render effect and attribute machinery per closure type; a value of a shared type (a `Signal`, the
opaque type of one helper function) is instantiated once. The probes use the tachys builder API; `view!` under
`erase_components` wraps closures in `__as_shared_reactive_fn` (one type per output type, +1 allocation per closure),
which these numbers don't show.

## 2. Rules for cheap hooks and atoms

Ordered by expected savings in leptonic.

### R1. Per-item state subscribes per key, never to the whole value

An item reading the group's value or the collection (`selected.with(|s| s == key)`, `collection.with(|c| c.get(key))`,
`set.contains(key)`) reruns on every change of it: O(n) per change, 8 allocations per item with a memo, 2 + an effect
run with a derive (table above). Subscribe the item to its key instead (`KeyedTriggers`, section 3) and notify the
keys that changed. RadioGroup of 1,000: a selection 38,161 → 22,196 allocations, 4,008 → 16 render effects.

```rust
// Before (use_radio.rs): 1,000 radios × 4 render effects rerun on each selection.
let is_selected = Signal::derive(move || state.selected_value.with(|s| s.as_ref() == Some(&value)));
// After: only the old and the new selected radio rerun.
let changed = state.subscribe_radio(value.clone()); // once per item: KeyedTriggers::subscribe
let is_selected = Signal::derive(move || {
    changed.track();
    state.selected_value.with_untracked(|s| s.as_ref() == Some(&value))
});
```

A memo shared by every item is no shield either, when it recomputes with each change of the collection: marking is
eager, so every reader below it (each item's memo, each render effect) is marked and checked again, even when the
memo's value stays. Measured (native, 1,000 rows; adding one): per reader ~4 allocations for a render effect, ~14 for
a memo with its render effect, with no memo or effect run at all: the table's "unexplained" 67 allocations per row
on a row change (`is_empty` in every row's and cell's flags memo, the columns in every cell's `data-column-index`,
the row header columns in every row's `aria-labelledby` and cell's `role`), the grid's ~22. A collection-wide value
every item reads is written only when it changes, so its readers are never marked otherwise: a trigger the node
diff notifies (`SelectionManager::is_collection_empty`, read by `GridState::is_empty`), or a signal an
`ImmediateEffect` sets only on a change (`TableState::columns` and `row_header_columns`; native: 38k → 16k). Find
it: budgets where a change of the collection costs ≈ items × k allocations with few memo and render effect runs.

### R2. Constants are not reactive attributes

A `Signal` (or `MaybeProp`, or closure) in an attribute always gets a `RenderEffect` (~9 allocations measured, with
the name `String`), even when it is `Signal::stored` and never changes: a computation that tracked nothing can't be
notified, so its effect never reruns. Render an attribute bound to a signal with `signal_attr(key, signal)`
(`maybe_attr` for a `MaybeProp`, `fixed_attr` for a plain value; `utils/signal_attr.rs`): it evaluates the value
once under a probe observer (`utils/probe.rs`: `probe_sources`, `is_constant`; `signal_prop` for boolean DOM
properties such as a checkbox's `indeterminate`), writes it and keeps no effect when
it read no reactive source, else one effect that writes only changed texts. Measured on the Button pilot: mount 586 →
507 allocations, 23 → 14 render effects (9 constants in the fixture).

The attribute code is shared: every value type only converts to an `AttrText` (absent, present, text, number), so
`SignalAttr` is one small instantiation per value type instead of tachys' attribute and effect code per value type. A
first design wrapping the tachys value (`Attr(K, wrapper(signal))`, static and reactive paths per value type) cut the
same allocations but grew a Button-only app by 20 KB raw / 3.3 KB brotli; `SignalAttr` shrank it by 9.6 KB raw /
1.0 KB brotli. Costs: the value is evaluated twice at mount when it is reactive (a `String` value: one more clone, a
memo-backed value: +2 for the second memo read); an absent `MaybeProp` and an absent `fixed_attr` have no key (no
removal list entry, below R11).

```rust
// Before (use_button.rs): one RenderEffect per field, constant or not; a key `Vec` per attribute.
Attr(attr::AriaHaspopup, self.aria_haspopup),            // Signal<Option<AriaHasPopup>>
Attr(attr::Form, form),                                   // Option<String>, mostly None
// After: no effect for a constant, the key list only for what renders.
signal_attr("aria-haspopup", self.aria_haspopup),
fixed_attr("form", form),
```

The same probe gates reactions: an `Effect` reacting to a flag (`if disabled.get() { reset() }`) does nothing for a
constant flag, so don't create it (`utils/reactions.rs`, `when_true`, R6), and fold `a || b` of constants
(`probe::either`) instead of deriving.

### R3. Absent and fixed inputs cost nothing

`Signal::default()` is `Signal::stored(T::default())`, and `Signal::from(value)` the same: 2 allocations that live
until the owner is disposed, a reactive read, and a render effect wherever it is an attribute. An optional `Signal`
prop that isn't passed costs them, and `..UseXInput::default()` creates a stored signal for every `Signal` field,
including the ones the literal sets (Button: `UseButtonInput::default()` 16 signals, `UsePressInput` 5,
`UseFocusRingInput` 1, its absent optional `Signal` props, `prop_signal` of absent `MaybeProp`s: ~50-60 allocations
of 586). 89 stored signals sit in 30 `*Input` defaults, 187 optional `Signal` props in atoms. Inputs and props that
are mostly constant are leptonic's `MaybeSignal<T>` (`utils/maybe_signal.rs`; why not Leptos' deprecated one:
`conventions.md`): a fixed value or a `Signal`, `Default` fixed. Fixed: 0 allocations, `with` neither tracks nor
clones, and as attribute value it renders as the plain value, without an effect. Fold fixed values with `fixed()`
instead of deriving.

```rust
// Before
#[prop(into, optional)] is_disabled: Signal<bool>,          // absent or `true`: 2 allocations
pub struct UseHoverInput { pub is_disabled: Signal<bool>, .. } // Default: Signal::stored(false)
// After
#[prop(into, optional)] is_disabled: MaybeSignal<bool>,     // absent or `true`: 0
fn either(a: MaybeSignal<bool>, b: MaybeSignal<bool>) -> MaybeSignal<bool> {
    match (a.fixed().copied(), b.fixed().copied()) {
        (Some(true), _) | (_, Some(true)) => true.into(),
        (Some(false), _) => b,
        (_, Some(false)) => a,
        (None, None) => Signal::derive(move || a.get() || b.get()).into(),
    }
}
```

### R4. One effect for an element's data attributes

Every `data-x=flag(sig)` is a render effect (with its own copy of the name) plus, erased, ~5 allocations of
attribute boxing (`keys()` allocates a `Vec` and another copy of the name, `tachys/src/html/attribute/custom.rs:116`).
RadioButton's `<label>` has 9, Button 5 (+ `data-focus-visible`), and 331 `flag(` sites exist. One attribute value
setting all of an element's flags in one render effect costs one effect at mount; a change reruns it once and writes
only the flags that changed; flags whose signals are all constant get no effect (R2). `utils/data_attributes.rs`,
`data_flags` (feature `atoms`). Measured (Button, 5 flags → 1 effect): mount −40 allocations. Keep at most **3
signals per effect**: a rerun re-tracks its sources into a fresh set that holds 3 without growing, so a 4th costs +1
allocation and +1 reallocation on every change (measured: a hover 7 → 8, a click 57 → 61 with 5 flags in one
effect). Group by what changes together: Button has its interaction state (`pressed`, `hovered`, `focused`) and its
props (`disabled`, `pending`, usually constants, then no effect).

```rust
// Before (radio.rs:358-366): 9 render effects.
data-selected=flag(radio.is_selected) data-pressed=flag(radio.is_pressed) ...
// After: one.
{..data_flags([("data-selected", radio.is_selected), ("data-pressed", radio.is_pressed), ...])}
```

### R5. A memo only where it cuts off a fan-out

A memo costs 8 allocations to create, 2 on every read (also when clean) and ~7 per rerun that ends unchanged (38% of
all memo runs). `Signal::derive` costs 2 and nothing per read; a plain `move ||` closure costs nothing. Keep a memo
where its value feeds many subscribers, is expensive (collections, formatting), or usually stays equal while its
inputs change *and* the readers are expensive. Trivial combinations (`a || b`, `!x`, `x == k`) are derives or closures.
`Memo::new(..).into()` adds another arena item. Measured (`use_focus_ring`'s `focus_visible`, read by one
attribute): Button keyboard focus 44 → 20 and blur 38 → 18 allocations (with R6, R8 below), mount −7; render effect
runs +1 per focus change (the attribute reruns where the memo stopped), memo runs 0.

```rust
// Before (use_selectable_item.rs:261-262): 2 × (8 + 2 per read).
let has_action = Memo::new(move |_| has_primary_action.get() || has_secondary_action.get());
let long_press_enabled = Memo::new(move |_| has_action.get() && allows_selection.get());
// After
let has_action = move || has_primary_action.get() || has_secondary_action.get();
let long_press_enabled = Signal::derive(move || has_action() && allows_selection.get());
```

Flags that change together and rarely (an item's disabled state, links, the action model) go into one memo of a `Copy`
struct, with a derive per output: `use_selectable_item` 6 memos → 1, `use_calendar_cell` 12 → 2 (one for the focus, so
an arrow key reruns the readers of 2 cells). The price: a change of one flag reruns the readers of all of them (a Grid's
selection mode change +3 render effects per row, disabling a Calendar 120 → 330 render effects); mount is the priority.
Keep the short-circuits of the reads inside the memo (only items with an action read the global "selection is empty"). A
derive over a notification source (a per-key trigger notified on both sides of a change, `notify_all`, a mode change)
reruns its readers on every notification even when its value stays: keep the memo there (`use_selectable_item`'s
`is_selected` and `is_focused`).

### R6. No `Effect` to derive or to sync state

`Effect::new` is ~10 allocations per instance, 4 per run and the most expensive call site in wasm (1.1 KB; 128
sites). Derive the value where it is read, update it where the change is caused (the event handler, the state
method), clean up with `on_cleanup`. When it must be a reaction to a signal, `ImmediateEffect::new_scoped` (7, 0.45
KB) runs synchronously inside the `set` (mind re-entrancy); keep `Effect` for DOM work after render.

Reactions of composed hooks to the same flag share one effect: `use_press`, `use_hover` and `use_focus_ring` each
reset their state while disabled; they register with `when_true(flag, reset)` (no effect for a constant flag), and
`use_button` calls them inside `sharing_one_effect(|| ..)` (one effect for all, created after them; the resets are
idempotent, so running all of them when one flag changes changes nothing). An effect reads what it doesn't react to
untracked: Button's pending announcement acts only on a pending change, but tracked `is_focused` and reran on every
focus change (react-aria-components has it in the deps and compares the previous pending state). Measured (Button,
disabled and pending reactive): effects at mount 4 → 2, disable 28 → 16 and enable 15 → 10 allocations, focus and blur
1 → 0 effect runs.

`sharing_one_effect` is public, and a nested call joins the outermost one: `use_option` shares one effect with the
`use_selectable_item` it composes, and atoms wrap an item hook and their own `use_hover` in it (Grid, GridList and
Table rows). Share wherever a disabled flag is a memo: `when_true` can't skip a memo (it is a reactive source), so
every collection item (disabled keys) creates the effect.

A reset that acts only on state the hook holds between events (`use_press`'s press, `use_hover`'s hover,
`use_focus_ring`'s and `use_focus_within`'s focus) registers with `when_true_once_armed` and arms it when that state
begins: the (shared) effect is created then, under the hook's owner (kept weakly), and stays. Equivalent because the
reset is a no-op without state (the focus ring's reset notifies only a change) and the state never begins while the
flag is `true` (the handlers check it), so the effect's first run, a tick after arming, resets exactly when the flag
became `true` since. An item that is never pressed, hovered or focused has no effect (~16 allocations at mount saved
per item; the first interaction pays ~18 once); a group with a `when_true` reaction (`use_selectable_item`'s focused
key, which exists without an event) creates it at once, as before. One difference: an effect created later
subscribes to the flag later, so within one tick it may run after other effects reacting to the same change.

```rust
// Before (use_focus_ring.rs, use_hover.rs, use_press.rs): an effect each, also for a constant flag.
Effect::new(move || if disabled.get() { set_focused.set(false) });
// After
when_true(disabled, move || set_focused.set(false));          // no effect for a constant
sharing_one_effect(|| button(input))                          // use_button: one effect for its hooks
```

Per-item effects that act only on the one focused or current item become one effect per collection reading the item's
registration: `ItemElements::register` (one effect per collection focuses the focused key's item, created lazily under
the collection's owner, held weakly (`lessons.md`); it remembers the registration and child focus strategy it focused
for, so other items mounting don't refocus) and the calendar's `CalendarCells`: −1 effect per item (ListBox option,
every grid row and cell, calendar cell). A registry that readers track needs per-key triggers (`KeyedTriggers`), or
every item's mount and unmount reruns them (+1 allocation per item on unmount with one global `Trigger`). Find it:
`Effect::new` in a hook that runs per item (`use_*_item`, `use_option`, `use_*_cell`, `use_*_row`) whose body returns
early unless the item is the focused, current or selected one; budgets whose mount effect runs are a multiple of the
item count.

### R7. One `StoredValue` per hook state

Each `StoredValue` is 2 allocations. `use_press` has 9 (`use_press.rs:781-817`; 18 per press, a radio has two
presses), `use_combobox_state` 11, `use_scroll_view` and `Tooltip` 9. Put a hook's non-reactive state in one struct.

`StoredField` (`utils/stored_field.rs`) projects one field of the state struct with the `StoredValue` methods, so the
call sites stay as they were. Measured: `use_press` 9 → 1 (−16 per instance), `use_focus` 2 → 1.

```rust
// Before
let state = StoredValue::new_local(None::<PressState>);
let is_triggering_event = StoredValue::new_local(false);
let saw_virtual_pointer_event = StoredValue::new_local(false); // ... 9 in all
// After
#[derive(Default)]
struct PressHook { press: Option<PressState>, is_triggering_event: bool, .. }
let hook = StoredValue::new_local(PressHook::default());
let state = StoredField::new(hook, |h| &h.press, |h| &mut h.press);   // used as before
```

### R8. No `Callback` for composition

`Callback::new` is 3 allocations (an `Arc<dyn Fn>` in an arena `StoredValue`); a plain `Arc<dyn Fn>` (leptonic's
`EventHandler::new`) is 1. Hook inputs are `Option<Callback<_>>` (C10), so a hook that wraps the user's callback to
pass it to an inner hook pays 3 per wrapper per instance (`use_radio.rs:237`, `:290`, `:341`;
`use_selectable_item.rs` 8 per item). Pass the user's callback through unchanged and add the hook's own reaction to the
inner hook's returned `EventHandler` (`chain`), keeping the order and the conditions (disabled, pending) the inner hook
applies; private helpers take `impl Fn` or `Arc<dyn Fn>`. Where the composing hook needs its reaction under the
inner hook's conditions, give the inner hook's public input a field for the extra reaction (no crate-private entry
points, `hooks-implementation.md`): `use_focus_ring` called `use_focus` with two wrapping `Callback`s (6
allocations); now it passes `UseFocusInput { mark: Some(FocusMark::new(set_focused)), .. }` (1), which marks the
state before the callbacks under the same conditions (measured: focus ring mount −19 with R5).

```rust
// Before (use_radio.rs:237): a new arena callback per radio.
on_focus: Some(Callback::new(move |e: FocusEvent| {
    state.set_last_focused_value(Some(focused_value.clone()));
    if let Some(on_focus) = on_focus { on_focus.run(e); }
})),
// After: the user's callback passes through; the radio's own part is an `EventHandler` (1 allocation). Valid only if
// use_focusable calls `on_focus` for every `focus` of the element (check its conditions first).
on_focus,
// ...
focusable_props.on_focus = EventHandler::new(move |_: FocusEvent| state.set_last_focused_value(Some(v.clone())))
    .chain(focusable_props.on_focus);
```

### R9. Expose what exists; don't wrap outputs

Every wrapper around an output is a new arena item: `Signal::derive(move || a.get() || b.get())` 2,
`ReadSignal.into()` 1, `Memo::new(..).into()` 1, `signal()` = `RwSignal::new` + 1 (+1 per `read_only()`). Return the
underlying signal when there is nothing to combine (`is_pressed` without `force_is_pressed` is `is_pressed`), use
`RwSignal::new` and hand out the `RwSignal` or one `read_only()`.

### R10. Read without cloning; cheap-clone values

`.get()` clones; reactive attributes clone their value on every run. Read with `.with`/`.read` in derives and
closures; give attribute values a type that clones without allocating (`Oco<'static, str>`, `Arc<str>`, `Copy`
enums). Ids are `Id` everywhere (`utils/id.rs`, conventions C2): a static `&str` or a shared `Arc<str>`, so a clone,
a context lookup or a reactive attribute run copies nothing, and `IdRefs::derive` passes a single contribution on
without joining. Form texts a part renders on every mount (`name`, `value` of a toggle or radio) are converted once
into a shared `Oco`. A memo producing a collection or string reuses the previous allocation with
`Memo::new_owning`. Don't clone a whole set to change one key: `CheckboxGroupState` changes a value it owns in place
and clones it only for an `on_change` handler (a bound value is the app's: a new set).

`Arc::<str>::from("")` allocates (an `ArcInner` even for an empty slice): the collection builders take texts as
`impl Into<Arc<str>> + AsRef<str>` and give every empty one the same shared `Arc` (`collections::empty_text`,
crate-private). VirtualList rebuilt its collection with `""` per line: 1 allocation per line per append (654k of
654k for 30 appends to 21k lines).

Keep `Copy` handles small: a closure captures them by value and is boxed with them. `SelectionManager` was 336
bytes of signals (captured by every item's handlers and derives); one arena `StoredValue` of them makes it 16
(+2 allocations per manager, made once per collection). Find it: clippy's `large_types_passed_by_value` on a `Copy`
type, or `size_of` of a handle that items capture.

### R11. Erased views: fewer attributes, children and wrappers per element

Erased, every inline attribute is ~4 allocations before it does anything, every child among several 2, every
component boundary 2, every `<Provider>` ~7. Put a hook's attributes into its one spread tuple (already the pattern),
provide an atom's contexts in one scope (`scoped_view`: one owner, no component or children box per context; the
field and group atoms of checkboxes, radios, switches and disclosures nested 2-3 `<Provider>`s), and use plain functions instead of
internal `#[component]`s where no props builder is needed. A spread is one `AnyAttribute`, which asks its tuple for
the keys of every attribute at build (to remove them on a type change): tachys allocates a `Vec` per `Attr` and grows
the first. `Spread((..))` (`utils/spread.rs`) collects them into one `Vec` of the right size; its elements implement
`AttrKeys` (leptonic's attributes, listeners, element captures). Button (~30 attributes): ~15 allocations and ~5
reallocations at mount (measured together with R6's shared effect: mount 334 → 301, reallocations 20 → 15), wasm
−2 KB raw.

### R12. Create nothing reactive in code that reruns

Everything created in an effect, memo or render closure is disposed and recreated on its next run (signal +5, memo
+8, effect +10, `on_cleanup` +1 per run). Create once outside and only read inside; register per-item cleanups once
per item (R1), not per read.

### R13. Wasm size

- Prefer values of a shared type in attributes: `flag(sig)`, a `Signal`, a named helper function returning `impl Fn`
  (30-60 bytes per site) over inline closures (570-700 bytes per site).
- `Effect::new` (1.1 KB per site) > `Memo::new` (0.6 KB) > `ImmediateEffect` (0.45 KB) > `Signal::derive` (0.36 KB) >
  `Callback::new` (0.33 KB) > `RenderEffect::new` (0.11 KB): fewer of the expensive ones (R5, R6).
- In generic hooks (`T` value types), move everything that doesn't need `T` into non-generic functions; monomorphized
  code is per `T` and per closure type.
- Never trade size for speed without measuring both (`build-performance.md` has the method). For a hook-level change,
  a cdylib that renders only the subject (a `Button` app: `hydrate_body`, `opt-level = "z"`, LTO) built against the
  tree with and without the change shows its size effect in 2 minutes, without other agents' changes in between.
- Thin generic wrappers around non-generic cores: `probe_sources<T>` only moves `T` around a `&mut dyn FnMut` core.

### R14. Nothing for what isn't there

Attach no listener for an absent callback (react-aria attaches none): `use_keyboard` without `on_key_down` and
shortcuts, and `use_focus` without callbacks, return empty handlers (each listener: an `Arc` handler and a closure
box; chained into another hook's, a `Vec`). Don't wrap handlers in conditions that can't apply (`use_button`'s pending
check for a button that is never pending). Name every field of a hook input at hot internal call sites:
`..UseButtonInput::default()` created 16 stored signals (32 allocations) of which 11 were overwritten at once
(Button: `UseButtonInput`, `UsePressInput`, `UseFocusRingInput`: −28, counted). Measured together with R5-R8 (Button): mount 431
→ 334, a click 57 → 45, Enter 44 → 32.

Optional inputs that can't be there cost nothing: `use_form_validation_state` builds no memo, effect or signal for
an error source that can't report one (a constant `is_invalid` of `false`, no `validate`, no name or no server
errors, constant builtin validation; most fields), and `use_description` returns `Signal::stored(None)` without an
effect or signal when its description reads no reactive source and is absent (collection items passed a long-press
description that is always `None`: ~25 allocations and 1 effect per item). Per-read subscriptions under an attribute
probe linger until unmount (`leptos-and-dom.md`, "Ownership and Cleanup").

### R15. Constants stay visible

`signal_attr`, `data_flags`, `when_true`, `use_description` and the validation sources skip their effect only for a
value that reads no reactive source (`utils/probe.rs`). A `Memo` is always a source, even over constants, and so is
any signal read before a static check. Decide on static configuration when the hook runs (a table without a tree
column: stored constants, not memos over `Option<TableTree>`, ~350 per row), put static checks before signal reads
(`use_tag`: `has_remove` before the modality), and keep the short-circuits of a combined memo in the same order (only
items with an action read whether the selection is empty). Find it: memos or derives whose body starts with a signal
read and only then tests a captured `bool` or `Option`; `let x = config?` inside a memo; `when_true`/`signal_attr`
over a memo of something the hook could know at creation.

### R16. Key items by identity, not position

A key derived from a position changes for every item after an insertion (a selection column, a sorted or filtered
row), so every keyed reader of a shifted item reruns, its view is rebuilt, and focus and other element state is lost.
Key by what identifies the item (a table cell by row and column key, as react-stately; toggling the selection
column of a 1,000-row table: 2.3M → 0.9M allocations, focus kept). Positions stay attributes (`aria-colindex`,
`data-column-index`), read where rendered. Find it: keys built from an index (`enumerate()`, `index` in a `<For>`'s
`key`, `Key::generated` for user content), and keyed views whose key memo reads a position.

### R17. Rebuilt structures: shared empties, known sizes, reused nodes

A structure rebuilt on every change (a collection per items change, a layout per resize) pays per element per
rebuild. Give empty and default values one shared allocation (an empty `Arc<str>` allocates: `collections::empty_text`),
size buffers up front where the count is known (`CollectionBuilder::reserve`: 14 reallocations per append of 21k
items before), and overwrite cached nodes in place when nothing else holds them (`Arc::get_mut` in
`virtualizer/list_layout.rs`) instead of allocating a new one per node. Find it: in the budgets, allocations of about
one per element on a change with no reactive work and no DOM change; in code, `Arc::from`, `String::new()`/`format!`,
`Vec::new()` + `push` inside builder and layout loops, and `Arc::new` of a node that replaces an equal one.

## 3. Keyed selection for collections and groups

**`KeyedTriggers<K>`** (`utils/keyed_triggers.rs`, crate-private): a `StoredValue<HashMap<K, (ArcTrigger,
subscribers)>>`.

- `subscribe(key) -> ArcTrigger`: once per item, in the item's hook (not per read). The key's trigger is shared by
  the items of that key; the subscription ends with an `on_cleanup` of the item's owner, so unmounted items leave
  nothing behind.
- `notify(keys)`: looks each key's trigger up and notifies it outside the map's borrow (a synchronous reader may
  unmount items, which unsubscribe); no allocation besides the notification. `notify_all()`: one `Vec`.
- Readers: `trigger.track()` and an untracked read of the shared value, in a `Signal::derive` or a render closure,
  never a memo (R5).
- Writers: the state's methods notify the keys whose state changed, after changing the value (single value: old and
  new; a set: the symmetric difference; a superset is fine). A value the app may change itself (a bound value) gets
  one `ImmediateEffect::new_scoped` per group that diffs it against the value last notified: synchronous (readers
  never see a stale value, unlike with an `Effect`), and only when the value is bound.
- A superset is fine, a subset is a bug: a key no writer notifies leaves its readers stale, silently. A diff of a
  tree walks every node, nested ones included (the node diff once walked only the top level: the cells of a removed
  row and the items of a removed section were never told), and an entry whose content another node shows notifies
  that node too (a section's heading: its header node). Find it: diffs iterating `iter()`/top-level children of a
  nested structure; readers whose value depends on another key than the one they track.
- A change every item shows (every radio becoming a tab stop) goes through one group-level `Trigger` that only the
  readers of that aspect track, so the other readers of each item stay put.

Chosen over `reactive_graph::computed::Selector` (an `ArcRwSignal` per key, 4 allocations; updated in an async
effect, so readers right after a change see the old value, which breaks `use_radio`'s rejected-change check; compares
every key per change; never prunes keys; single values only). The selection manager's former private
`KeySubscribers` (subscribing per *read*) is `KeyedTriggers::track` now: one implementation; a reader that isn't an
item (`SelectionManager::is_selected`, a group's `is_selected`) subscribes for its current run (a map update and an
`on_cleanup` per run), an item once (`subscribe`). `following(set)` / `following_one(value)` keep triggers in step
with a set or a single key (one `ImmediateEffect` diffing against the value last notified). Costs (native probe, 1,000 items with 4 readers each; and the budgets): per item mount +4-5
allocations (trigger 1, its subscriber set 2, +1 past 3 readers, `on_cleanup` 1); per change only the changed keys'
readers rerun; wasm ~7 KB (pre-`wasm-opt`, `opt-level = "z"`) for the first `K` (`Key`), shared by every group
using `KeyedTriggers<Key>`; a second key type (calendar `Date`s) adds its own copy.

**RadioGroup** (the pilot): `use_radio_group_state` notifies the radios whose selection or tab stop changed (at most
the old and new selected and last-focused radios); `use_radio`'s `is_selected` and `tabindex` track the radio's
trigger (and `tabindex` the group's "every radio is a tab stop" trigger). Budgets before → after:

| Measurement | Allocations | Render effects |
|---|---|---|
| RadioGroup 1,000: select another with the mouse | 38,161 → 22,196 | 4,008 → 16 |
| RadioGroup 1,000: arrow key | 38,158 → 22,197 | 4,004 → 12 |
| RadioGroup 1,000: the app selects another | 16,014 → 56 | 4,000 → 8 |
| RadioGroup 1,000: first selection (every radio stops being a tab stop: inherent) | 39,129 → 27,143 | 4,004 → 1,007 |
| RadioGroup 10: select another with the mouse | 524 → 406 | 46 → 14 |
| `use_radio_group` 1,000: click / arrow key | 12,066 → 8,066 | 2,000 → 4 |
| mount, RadioGroup 1,000 | 785,723 → 800,738 (+15 per radio; `use_radio` alone +4, as the probe; the atoms' rest unexplained, `atoms/radio.rs` changed meanwhile) | 21,015 (same) |

What still grows with the group: a selection *by the user* costs ~8 allocations per radio (`use_radio_group`
fixture) and ~22 (RadioGroup, native validation) with no reactive work, while the app selecting costs 56 at any size:
it is not in the reactive path (a native probe of `set_selected_value` with 1,000 radios: 9 allocations), but in the
event or validation path (the atom's extra ~14 per radio: probably every radio's native validity being read on each
commit, the readers of `register_native_validity_reader`; open in `PLAN.md`). Disabling the group reruns 4 `Effect`s per radio (R6).

**Batch-apply list** (applied 2026-10-17, measured results in `history.md`; the rows' "Expected" were the estimates):

| Where | Per-item reader it replaces | Writer that notifies | Expected |
|---|---|---|---|
| CheckboxGroup | `use_checkbox_group.rs:250` derive over `state.is_selected(&value)` (the whole `HashSet`) | `use_checkbox_group_state.rs` `add_value`/`remove_value`/`set_value` (+ the bound-value watcher); also R10 (no set clone per toggle) | click (1,000): 3,004 → ~6 render effects, ~−15k of 52k; the rest is the per-item commit validation (as the radio's residual) |
| ToggleButtonGroup | `use_toggle_button_group.rs:110` `Memo` over `group.is_selected` and `atoms/toggle_button.rs:149` derive (`selected_keys`) | `use_toggle_group_state.rs` `set_selected_keys`/toggle | per change ~8 per button → O(1); mount −6 per button (derive instead of memo) |
| Tabs | `atoms/tabs.rs:371` panel `Memo` over `state.selected_key()` (clones the selection) | none needed: use the selection manager's per-key `is_selected(&key)` | Tabs (100) 3,500 → ~900 |
| DisclosureGroup | `atoms/disclosure.rs:172` `Memo` over `group.is_expanded(key)` (`expanded_keys`) | `use_disclosure_group_state.rs` `set_expanded_keys` (symmetric difference) + watcher | toggle (1,000 memos) 20k → <0.5k |
| Tree and Table expansion | `use_tree_item.rs:97`, `use_table_row.rs:180-182` `expansion.is_expanded(key)` | the expansion state's setters | expanding one row: O(rows) memo runs → O(1) |
| ListBox, GridList, Table, Tag, Menu, ColorSwatchPicker: selection and focus | already per key (`KeySubscribers`, per read) | move `SelectionManager` to `KeyedTriggers`, subscribing once per item in `use_selectable_item` | ~2 per item per rerun (the per-read `on_cleanup` and map update); one implementation instead of two |
| the same collections: collection changes | `use_selectable_item.rs:223` `is_disabled` and `:229` `is_link` memos, `use_option.rs:167` link memo, `use_node_aria_label` (`use_node.rs`), `use_grid_list_item.rs:214` label, `use_table_row.rs:146` labelledby: each reads the whole collection (and `disabled_keys`) | the collection memo diffs old and new nodes per key and notifies the changed keys; `disabled_keys` by symmetric difference | ListBox append (1,000) 92k → ~15k; grid row add 260k → ~20k; swatch add 16k → ~2k; positions (`aria-posinset`) stay O(n) |
| Drag and drop | `use_draggable_item.rs:173` `is_dragging` derive (`dragging_keys`), `:102` `drag_count` memo; `use_droppable_item.rs:130` `is_drop_target` memo and `use_drop_indicator.rs:167/180` memos (the current target) | `set_dragging_keys`; the drop target setter notifies the old and the new target's item key | drop target change 2,002 memos (22-32k) → ~10 memos; drag start 169k → ~15k (the session-wide part, `is_valid_drop_target`, stays per item) |
| Calendar cells | `use_calendar_cell.rs` `is_focused` (focused date), `is_selected` (value, range highlight) memos | `KeyedTriggers<Date>`: focus moves notify the old and new date, selection the old and new value, a range highlight the dates entering or leaving it | arrow key (12 months) 14k → ~1k; selecting a date 38k → ~2k; min/max/unavailable changes stay per cell |

## 4. Hit list for leptonic

Estimates are per measurement of the budgets (allocations unless noted); "risk" is the risk of losing behavior.

| # | Pattern | Where (files, counts) | Estimated savings | Risk |
|---|---|---|---|---|
| 1 | Per-item reads of whole values (R1, section 3; RadioGroup done, the rest: section 3's batch-apply list) | `use_checkbox_group_state.rs:107`, tabs, disclosure group, swatch picker, `use_selectable_item.rs:222-229`, `use_option.rs:167`, dnd `dragging_keys`, `use_calendar_cell.rs` | RadioGroup selection (1,000) measured 38k → 22k, 4,008 → 16 render effects; CheckboxGroup click 52k → <1k (if its commit doesn't validate every item, `PLAN.md`); Tabs (100) 3,500 → ~900; DisclosureGroup toggle 20k → <0.5k; ListBox append 92k → ~15k; drag start 169k → ~15k; calendar arrow key (12 months) 14k → ~1k | medium: notification timing (synchronous, app-side changes through a diff effect), keys of removed items |
| 2 | Constant values as reactive attributes (R2; Button stack done) | 460 `Attr(attr::` in hooks, most `Signal` fields of `*Props`; every atom on `use_link`, field hooks | measured ~9 per constant attribute (Button −79 of 586); every hook with `aria-*` props a few effects each | low: `signal_attr` (semantics identical) |
| 3 | Stored defaults for absent inputs (R3) | 89 stored signals in 30 `*Input` defaults (`UseButtonInput` 16, calendar states 10, `UseLinkInput` 6, `UsePressInput` 5); 187 optional `Signal` props in atoms (DatePicker 32, Calendar 14, ComboBox 10) | Button ~60 per mount (10%); DatePicker ~100; each `..Default` literal 2 per `Signal` field | low (mechanical, but changes input field types: grep the literals in the book and the test-app) |
| 4 | One effect per element for `data-*` flags (R4; Button done) | 331 `flag(` sites; RadioButton label 9, RadioField 5 | measured Button −40 per mount (5 flags); radio ~70 of ~800; disabling a group (1,000) 4,000 → ~1,000 render effects | low (≤3 signals per effect) |
| 5 | Memos for trivial derivations (R5; `use_selectable_item` 10 → 3 and `use_calendar_cell` 12 → 2 per item done, 2026-10-17) | 243 `Memo::new` sites: `use_date_field_state.rs` 12, `use_number_field_state.rs` 10, `use_form_validation_state.rs` 10 (built only for sources that exist) | ~8 per memo; fewer "unchanged" memo runs | low-medium: keep memos that cut off fan-outs |
| 6 | Effects for derived or synced state (R6; collection items 5 → 1 and calendar cells done, 2026-10-17) | ~140 `Effect::new` sites (`FocusScope` 8, `use_combobox` 5, selection manager 3) | ~10 per avoided instance (Button ~30-40); wasm ~1.1 KB per removed site | medium: effect timing |
| 7 | Many `StoredValue`s per hook (R7; `use_press`, `use_focus`, `use_selectable_item` done) | `use_combobox_state.rs` 11, `use_scroll_view.rs` 9, `tooltip.rs` 9 (per tooltip), `use_form_validation_state.rs` 6 | 2 per merged `StoredValue` (`use_press` measured −16, in every pressable) | low |
| 8 | `Callback::new` for composition (R8; `use_selectable_item`'s wrappers and the per-item `on_action` done) | ~300 sites in hooks, 34 in atoms (`use_radio.rs` 3, `use_toggle.rs` 4 for the label's press) | 2-3 per wrapper: radio ~6-9 | low-medium: handler order and conditions |
| 9 | Output wrappers and `signal()` (R9) | `use_press.rs:1636`, `use_focus_ring.rs:209/230/236`, `Signal::derive` 470 sites, `signal(` 133 | ~2 per wrapper; Button ~10 | low |
| 10 | Clones (R10) | ids in atoms (`button.rs` 4 `String` clones), `Option<String>` attribute values, `use_checkbox_group_state.rs:143-158`, text fields (a keystroke copies the text ~5×, `PLAN.md`) | Button ~5; checkbox toggle 1 set clone per change (1,000 keys: one `HashSet` of 1,000); typing ~4 copies per keystroke | low |
| 11 | Erased element structure (R11) | nested `<Provider>`s (Radio 3, atoms 57 `provide_context`), internal components | ~7 per removed `Provider`; radio ~14 | low-medium (context lookups move) |

**Batch-apply from the Button pilot** (2026-10-16; Button mount 586 → 301, a click 57 → 45, keyboard focus 44 → 20,
a hover unchanged at 7; a Button-only app −15 KB raw / −1.7 KB brotli). Mechanical, in this order, each measured by
the subject's performance test:

1. `Attr(attr::X, signal)` and `custom_attribute(key, signal)` in hook attrs → `signal_attr(key, signal)` /
   `maybe_attr` / `fixed_attr` (R2; ~430 sites; ~9 per constant attribute at mount, wasm smaller).
2. Hook attr tuples → `Spread((..))` (R11; ~1 per attribute per spread at mount).
3. `data-x=flag(sig)` runs → `{..data_flags([..])}`, ≤3 signals per group (R4; ~8 per flag after the first).
4. Effects resetting state on a flag (`if disabled.get() { .. }`) → `when_true`; hooks composing such hooks wrap
   their body in `sharing_one_effect` (R6; ~10 per effect; for constant flags all of them).
5. Several `StoredValue`s in a hook → one struct with `StoredField`s (R7; 2 each; `use_combobox_state` 11,
   `use_scroll_view` 9, `Tooltip` 9, `use_form_validation_state` 7).
6. Wrapping `Callback`s for inner hooks → a crate-private entry point taking the reaction (R8; 2-3 each).
7. Memos read by one attribute or trivial → `Signal::derive`/closures (R5); effects read untracked what they don't
   react to (R6).
8. Listeners for absent callbacks, conditions that can't apply, and `..Input::default()` at hot internal call sites
   (235 literals, `UseHoverInput` 20, `UseButtonInput` 17, `UseKeyboardInput` 16, `UsePressInput` 13) → none, none,
   every field named (R14; 2 per defaulted `Signal` field).

Upstream (Leptos) costs (the user, 2026-10-16: patch the fork only with fixes that are safe and benefit every user;
document the rest):
- **Done** (fork branch `leptonic-metrics`; upstream branch `reactive-graph-snapshot-notify`, based on upstream, not
  yet sent): notifications, memo marks and source checks iterate an exactly sized `Vec` snapshot instead of a clone of
  the address-hashed `IndexSet` (a set clone allocates the hash table plus an entries buffer sized to the table's
  capacity: 2 allocations of layout-dependent size, which made `allocated_bytes` vary between builds; now 1, none for
  an empty set); a clean or dirty memo no longer copies its sources on read (only `Check` visits them).
- **Prototype** (branch `into-reactive-value-from-signal`, upstream-based, with an integration test): a
  `FromReactiveClosure<T>` trait, so that closures convert into a downstream type such as leptonic's `MaybeSignal`
  through `#[prop(into)]` (a downstream crate can't implement `IntoReactiveValue` for closures: orphan rule).
- **Not patched** (API changes or design choices, not safe drop-in fixes): every reactive attribute copies its name
  into a `String` (`tachys/src/reactive_graph/mod.rs`, `key.to_owned()`: the `&str` key of `AttributeValue` can't
  tell a static name; needs a trait change); erased attributes return their keys as a `Vec` per attribute
  (`Attribute::keys`, `any_attribute.rs`; upstream's own TODO: "fix this whole approach" for 0.9); `Callback` costs 3
  allocations (an arena-stored `Arc<dyn Fn>`, the price of being `Copy`); an effect rerun rebuilds its source set
  (dynamic dependency tracking).
- **Found, not patched** (2026-10-18, native probe against reactive_graph 0.2.15; the fork's `computed/inner.rs` has
  the same logic): a memo chain under an `ImmediateEffect` recomputes twice per change. The effect runs while the
  source's notification is still marking (`mark_check` runs it at once); its check recomputes the top memo, whose
  change marks the next memo dirty and runs the effect again, re-entrantly, which recomputes the rest of the chain;
  back in the outer check, `needs_update` already returned `true` and recomputes them once more. Probe: source →
  memo A → memo B → memo C → `ImmediateEffect`: A runs once, B and C twice. The table pays its selection column
  rebuild (`with_selection_column`, ~2 allocations per row) twice per change of the rows. A fix would re-check the
  memo's state (clean again: skip) before recomputing; a candidate for the fork (`PLAN.md`).
- reactive_graph's immediate-effect tests are flaky when run in parallel threads (also unpatched); run them with
  `--test-threads=1`.

## 5. Finding these patterns

The budgets (`leptonic/tests/budgets/`) show most of them as ratios; read a large fixture's measurements per item
(the fixture's count: `testing/test-app/src/pages/perf/`). A short script ranking every measurement by a metric (all
budgets flattened to subject, scenario, step and totals) finds the worst first; the signs:

| Sign in the budgets | Likely pattern | Rule |
|---|---|---|
| a change of one item reruns render effects or memos of every item (`render_effect_runs` or `memo_runs` ≈ items) | per-item read of a whole value | R1 |
| `memo_runs_unchanged` ≈ k × items on one change | memos over whole values, or memos for trivial derivations | R1, R5 |
| mount `effect_runs` a multiple of the items | per-item `Effect` (a reset, focus following, a description) | R6, R14 |
| mount allocations per item well above the cost of what renders (compare a hook's budget with its atom's) | constants hidden in memos, stored defaults, wrapper callbacks, absent inputs | R2, R3, R8, R14, R15 |
| allocations ≈ items on a change with no reactive work and no DOM change | rebuilt plain data (texts, buffers, nodes) | R17 |
| allocations ≈ k × items on a change of the collection, few memo and render effect runs | per-item readers of a collection-wide memo that recomputes with every change (marked and checked again) | R1 |
| unmount allocations per item above 0 | per-item notifications or work on cleanup (a registry trigger every reader tracks) | R6 |
| an insertion or a column change re-renders every item (`dom_added_nodes` ≈ items) | keys from positions | R16 |
| a step costs a whole extra pass (paging once more after mounting, a step's numbers ≈ the next step's) | an asynchronous correction | `leptos-and-dom.md`, "State Corrections" |
| effect runs double on repeated events (focus, press) | a setter notifying an equal value | `leptos-and-dom.md`, "State Corrections" |
| a fixture's measurement ≈ items × a small constant though the library does little | the fixture measures itself (`format!` keys) | `testing.md`, "Performance Measurements" |

In code (`rg` over `leptonic/src`): `Memo::new` in per-item hooks and what reads each one; `Effect::new` in per-item
hooks; `Signal::stored(` and `..Default::default()` at internal call sites; `Callback::new(move |e| { ..; inner.run(e)
})`; `with(` of a collection, selection or group value inside per-item closures; `Arc::from("")` and buffers grown by
`push` in rebuild loops; keys from indexes; `Owner::current()` kept in a stored value (`leptos-and-dom.md`, "Ownership
and Cleanup").

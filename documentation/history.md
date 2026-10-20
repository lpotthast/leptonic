# Leptonic: history

Finished work, one short entry each (what, when, where). Rules and knowledge that came out of it live in the
documents `README.md` lists; git history has the details. Most recent first.

## Library

### 2026-10-20

- A combo box opened with a highlighted option (button, ArrowDown/ArrowUp) now closes and loses `data-focused` when a
  click outside moves focus away. `use_focus` reports every blur of the element, as react-aria does, and drops only
  Chrome's native blur after the synthetic one `SyntheticBlurObserver` dispatched for an element disabled while
  focused. Virtual focus (a `focusin` on an element without DOM focus) no longer activates a focus scope that only
  restores focus (react-aria's `useRestoreFocus` checks the active element), so the closing popover no longer
  refocuses the input. Tests: `test_combobox::click_outside_*`, `test_focus::synthetic_blur_then_native_blur`.

### 2026-10-18

- Interactions and groups, behavior-keeping except one ordering detail (`PLAN.md`): the disabled resets of `use_press`,
  `use_hover`, `use_focus_ring` and `use_focus_within` create their effect on the first press, hover or focus
  (`when_true_once_armed`), not at mount; `signal_prop` (`checked`, `indeterminate` without an effect when constant);
  `scoped_view` instead of nested `<Provider>`s in Checkbox, Switch, Radio, Disclosure, Button and ToggleButton; one
  shared `aria-describedby` per checkbox group; every `ToggleOptions` field named; `data-orientation` through
  `signal_attr`. Both rounds together: allocations 56.67M → 53.47M (−39.6% since 2026-10-17), effect runs −16% (−74%),
  DOM operations unchanged; CheckboxGroup (1,000) 601k → 540k, Table row add 86k → 18k, TagGroup 737k → 698k. Test-app
  wasm raw 15,831,193, brotli 3,152,242.
- Collection items, behavior-keeping (anatomy of table, grid, list box, tag and drag-and-drop items): the 23-70
  allocations per row of a row change in Table and Grid were per-item readers of collection-wide memos that recompute
  with every change (marked and checked again, `performance.md` R1): `GridState::is_empty` follows a trigger the node
  diff notifies only when emptiness changes (`SelectionManager::is_collection_empty`; also `TableBody`),
  `TableState::columns`/`row_header_columns` are signals written only on a change (isomorphic effects: see
  `leptos-and-dom.md`, "State Corrections", for the two bugs this first had).
  Per-item constants made once per collection (`GridData` actions, link behavior and cell role, `ListBoxData::on_action`,
  a table's flat-row tree values), one `StoredValue` per grid cell, `data-selection-mode` without a render effect for a
  constant mode (`SelectionManager::selection_mode_signal`), per-item contexts through `scoped_view` instead of
  `<Provider>` (list box, grid list, table row, menu item, tag), single-flag `data_flags` groups folded (tag, grid row,
  list box and menu items), `has_focused_key` a signal written only when it changes (every tag's `tabindex` reads it),
  `use_drag_session` hands out the session signal (no copy of the session per read).
- Sweep against the patterns to avoid (`performance.md` R1-R17 and section 5, `leptos-and-dom.md`), behavior-keeping
  fixes: per-node tracking for item labels, child rows and section headings (grid list, tree, sections, draggable
  items); flat grid list rows without tree machinery; one description per collection or group (drop targets, tags); one
  selection column memo per table; selection checkboxes reuse the row's `is_selected`; inline item registrations;
  `use_radio` and the checkbox/switch/radio atoms share one reset effect (hover created by the field);
  `use_form_validation_state` builds the native commit only for `ValidationBehavior::Native`, `use_form_validation` one
  effect per element; checkbox group items reuse the group's validation; `use_overlay_position` one effect for its
  listeners and observers; Tooltip, FocusScope, color swatches, disclosure (`on_beforematch`), number and text fields,
  range calendar ends per date. Bugs found on the way: removed nested nodes (cells of a removed row, items of a removed
  section) never notified their readers (`node_triggers` diffed only the top level); NumberField's synchronous text
  update deadlocked on the first commit of an empty field (an `ImmediateEffect` that only `track()`ed a memo,
  `leptos-and-dom.md`); a RadioGroup budget varied with the browser's post-scroll hover events (`testing.md`). All
  budgets: allocations 59.52M → 56.67M (−36% since 2026-10-17), effect runs −69%, memo runs −68%. Three changes that
  would alter behavior wait for the user (`PLAN.md`).
- Timing runs (the user): `just browser-test-timed` runs the suite sequentially and records every passed test's duration
  in `leptonic/tests/timings/<machine>/tests.json` (checked in, per machine: OS, CPU, logical CPUs, memory;
  `tests/timing.rs`), compared with the previous timed run; `just browser-perf`'s scenario reports moved to the same
  directory. First baseline on the 7950X: 21m 18s sequential. The parallel suite (8 at a time, same machine and mode):
  3m 44.1s at 11922064 → 3m 24.0s (test bodies 28m 37s → 26m 03s; `measure` steps −13%, page loads −9%).
- Table cells keyed by row and column key (`Key::cell(row, column)`, the selection cell by
  `TableCollection::SELECTION_COLUMN`; react-stately's `${rowKey}${column.key}`), grid cells by position: toggling the
  selection column no longer re-renders every cell or loses the focused cell; `TableCell` renders once and reads
  only the columns; `use_table_cell` takes row and column from the key (`Key::as_cell`). Direction and strings per
  table/grid (`GridData::direction`, `GridData::strings`, `TableData::strings`) instead of per row, cell or column.
- Optimizations that keep react-aria's behavior (the user: done right away, `AGENTS.md`): `SelectionManager` a 16-byte
  handle (was 336 bytes, copied into every item closure); items subscribe once to their selection and focus
  (`SelectionSubscription`); `UseSelectableItemInput::on_action` takes the collection's `Callback<Key>` (no per-item
  wrapper); range calendar ends notified per date (`UseCalendarCellReturn::is_selection_start`/`_end`);
  `use_form_validation_state` builds no memo, effect or signal for error sources that can't report one (most fields);
  collection texts of `""` share one empty text (`empty_text()` crate-private again); `sharing_one_effect` public, used
  by the Grid, GridList and Table row atoms; a tag without `on_remove` has no description effect. All budgets:
  allocations 67.45M → 59.53M (88.48M before 2026-10-17: −33%), memo runs → 0.73M (−62%), effect runs → 0.32M (−60%);
  Table selection column shown/hidden (1,000 rows) 2.24M/1.31M → 0.88M/0.06M. Test-app wasm (`wasm-release`) raw
  15,838,256 (−1.1% since 2026-10-17), brotli 3,148,206 (−0.07%).
- Review of both passes: the collection's focus registry held its own owner strongly (a cycle: a collection in a
  `<For>` row was never freed and its focus effect kept running), now a `WeakOwner` (test
  `the_focus_effect_does_not_keep_the_collection_alive`, `lessons.md`); `ItemElements::register` is the one,
  crate-private registration (the public element-only one had no callers); deviation blocks, docs and names brought in
  line with the new code (`use_selectable_item`, `use_calendar_cell`, `use_form_validation_state`,
  `sharing_one_effect`); native tests for `sharing_one_effect`'s panic safety, constant validation sources and
  `use_description`; a calendar cell shares the calendar's constant `false` (−2 per cell).

### 2026-10-17

- Per-item costs (from an analysis of the budgets; what is left: `PLAN.md`, "Per-item work left"): `use_description`
  creates nothing for a description that can't be there (every collection item's long-press description, the single
  `Calendar`'s range prompt); nested `sharing_one_effect` joins the outermost, `use_selectable_item`, `use_option`,
  `use_toggle`, `use_menu_item` share their resets; `use_selectable_item`'s six action-model memos are one `Copy` flags
  memo, its four `StoredValue`s one, no virtual-focus wrapper callbacks; one focus effect per collection
  (`ItemElements::register`, per-key registry triggers) instead of one per item; Table rows without a tree
  column have constant tree attributes (no memos), `GridState::is_empty`, `TableState::row_header_columns`, cell
  positions and row headers tracked per node, the row's selection column from `columns`; VirtualList: the shared
  `empty_text()`, `CollectionBuilder::reserve`, `LayoutNode`s reused on relayout, fixtures without `format!` keys;
  Calendar: 12 memos per cell → 2, the range prompt only in range calendars, synchronous (silent) realignment on a
  duration change, `tabindex` per focused date, one focus effect per calendar (`CalendarCells`), `CalendarWeek` without
  per-position memos. DnD screen reader tests wait for the drag session (`DndActions::wait_for_drag_session`; their
  drops raced the session's first frame). All budgets: allocations 88.48M → 67.45M (−24%), memo runs 1.93M → 0.94M
  (−51%), effect runs 820k → 363k (−56%), render effects −11%; ListBox option mount (1,000) 489k → 358k, Table mount
  (1,000 rows) 3.37M → 2.60M, Table row add 277k → 164k, Menu mount 633k → 511k, 12-month Calendar 333k → 193k,
  VirtualList append at the start 654k → 1.2k. Up: a selection mode change in Grid/Menu (+7-11 per item: the flags memo
  reruns all its readers), disabling a Calendar (render effects 120 → 330). Test-app wasm (`wasm-release`): raw
  16,016,315 → 15,861,534 (−1.0%), brotli 3,150,568 → 3,153,395 (+0.09%).
- Shared crate-private helpers public and documented (the user: "Make these hooks public and add documentation for
  them in the book"): `use_hide_outside(UseHideOutsideInput { is_open, overlay, behavior: HideOutsideBehavior })`
  (overlay; was positional with a `keep_visible_only` bool), `use_wheel_stepping` (interactions; returns `group` and
  `input` parts), `use_tree_expansion(UseTreeExpansionInput)` (tree; `TreeExpansion::set_expanded_keys` replaces
  `TreeState::set_expanded_keys`, `toggle_key` a method), `use_node(UseNodeInput)` and
  `use_node_aria_label(UseNodeAriaLabelInput)` (collections, own file `use_node.rs`),
  `FormValidationState::register_native_validity_reader` (`NativeValidityReaders` private), `merge_long_press`. Five
  new perf tests and fixtures, behavior tests `test_hide_outside.rs`, `test_wheel_stepping.rs` (+ `delta_x` on wheel
  events), native tests; budgets of 82 files re-recorded (`ran` gains the new hooks). Book: pages `use_hide_outside`,
  `use_wheel_stepping`; sections on Tree Hooks, Collection State, `use_press`, Form Hooks.
- No crate-private hook variants (the user, option 1): `focus_props`, `use_toggle_with`, `use_checkbox_with` and
  `use_progress_bar::progress` are gone; their extra inputs are public fields: `UseFocusInput::mark` (`FocusMark`),
  `UseToggleInput::role` (`ToggleRole`) and `::validation`, `UseCheckboxInput::validation` (`FormValidationState`,
  `from_parts` public), `UseProgressBarInput::role` (`ProgressRole`); `UseSwitchInput` is its own struct; a checkbox
  group item adds the group's ids to `aria_describedby` (the former `ToggleGroup`). Budgets: counts unchanged, `ran`
  lists name the inner hooks (56 files).
- Leptonic's code is user code (the user: "I want users to be able to look at how we implement atoms and be able to
  copy most code for their own spin"): the crate-private `use_x_raw` bodies and `UseXRaw` structs are gone (95); atoms
  and composing hooks call the public hooks and use their parts as users do (`attrs()` to render, `into_props()` to
  build on a part, new; parts passed through unchanged), `metrics.rs` no longer strips `_raw`. Public now: `Part::new`,
  `Part::stored`, `Part::observed`, `Remount`, `Presence::new`/`hold`, `fixed_attr`, `signal_attr`, `maybe_attr`,
  `FixedAttr`, `SignalAttr`, `ToAttrText`, `AttrText`, `Spread`, `AttrKeys`, `EventAccessors`, `join_ids`,
  `IdListBuilder`, `PreventFocusAttr`, `scoped_view`, and (atoms) `data_flags`, `DataFlags`, `selection_mode`,
  `with_default_class`, `id_ref_prop`, `clear_context`, `use_clearable_context`, `ClearContexts`, `LabelPresence`,
  `LabelContext::with_presence`. `EventHandler` chains are one shared `Arc<[..]>` (a copy per mount is a refcount
  bump). `ColorThumb` and `ColorSliderTrack` take the hooks' parts directly (no combined stored structs). Budgets
  +1.3% allocations (87.22M → 88.32M; effects, memos, DOM unchanged; Button mount 289 → 300), wasm: Button probe
  +0.8% brotli (278,772 → 281,096), test-app +0.08% (3,140,895 → 3,143,255). Docs: `hooks-implementation.md`
  ("One way: leptonic composes its public hooks"), `atoms-implementation.md`, `conventions.md`, `performance.md`
  ("Part costs"), the book's Hooks & Atoms "Parts" (a hook of your own).
- `Part` everywhere (the user): every props value a public hook returns is a `Part` named after its element (C9;
  `target` for behavior hooks), built with `Part::stored` (`Remount`, one implementation per props type); hooks and
  atoms compose through crate-private `use_x_raw` bodies (they report as `use_x`); `Part::map` public; a
  `PropsWithStyles` part's `attrs()` gives attributes and styles; `Presence` (a lease without props) replaces
  `use_rendered` (select and combo box listboxes; `UseListBoxInput::presence`); `TagList`, `SelectTrigger`, the
  hidden select, `CalendarCellButton`, `DateInput`, `DatePickerGroup`, `InputContext` (takes a `Part`),
  `LabelContext` (takes a `Part`), `ColorThumb` and the color slider track take fresh props per mount. `Id` lost
  `Default` (the tab's fallback collection id is the tab list's id; the combo box takes its button id from the menu
  trigger). Budgets: atoms unchanged (50.80M), public-hook fixtures +0.7% (36.17M → 36.42M: a part costs 2-3
  allocations per mount). Wasm: Button probe unchanged, test-app +0.7% brotli (3,119,199 → 3,141,570).
- Consistency sweep over every hook and atom (the user: "everything we decided applied to all hooks and atoms"):
  ids are `Id` everywhere (`use_id -> Id`, element ids `Option<Id>`/`Id`, references `Signal<Option<Id>>` joined by
  `IdRefs::derive`, slots' `referenced_id`, `labels()`; atoms convert their `MaybeProp<String>` references with
  `id_ref_prop`; the `String` bridges and `IdRefs`' string API are gone); every hook's attributes through
  `signal_attr`/`fixed_attr`/`maybe_attr`, every atom's data attributes through `data_flags` (≤3 per group);
  `KeyedTriggers` for CheckboxGroup (in-place updates of an owned value), ToggleButtonGroup, Tabs, DisclosureGroup,
  tree/table expansion, drag and drop, calendar cells, collection node changes (one diff per change,
  `SelectionManager::track_node`) and the selection manager (its `KeySubscribers` merged into `KeyedTriggers::track`);
  `Part`s for the overlay arrow, popover and tooltip elements, `NumberFieldGroup`, `ColorWheelTrack` and
  `DisclosureTrigger` (its presence names the panel: no `use_rendered`); merged stores in `use_combobox_state`,
  `use_scroll_view`, `use_move`, `use_focus_within`; named input fields where a default's stored signal was
  overwritten. Budgets: all allocations 123.5M → 87.0M (−30%; collections −31%, dates −42%, forms −19%, button/link/
  toggle −38%); e.g. ListBox append (1,000) 91,714 → 2,556, calendar arrow key (12 months) 14,393 → 1,216,
  CheckboxGroup click (1,000) 54,120 → 13,106 and 3,005 → 7 render effects, DisclosureGroup toggle (1,000) 20,078 →
  88, Tabs (100) 3,480 → 564, grid row add (1,000) 352k → 98k, CheckboxField mount 1,035 → 753. Test-app wasm
  (brotli) 3,113,820 → 3,117,200 bytes (+0.1%). Left: `PLAN.md`.

### 2026-10-16

- Fixed, found by the performance tests: the RangeCalendar's `focus_nearest_available_date` read the visible
  duration tracked from a keyup listener (warning; `untrack`); `ColorChannelField` never stepped by wheel (its root
  takes the number field's focus-within handlers). Not bugs: the stepper/clear buttons' `data-focus-visible`,
  `NumberFieldState::increment` without `is_disabled` (as upstream).
- Fixed, found by the performance tests: dangling ARIA references (Select value/listbox, ComboBox listbox,
  Disclosure panel/trigger, Tooltip; `use_rendered`); atoms rendering their hook's id ignored `attr:id` (an `id` prop
  on eight atoms, `test_own_ids.rs`); virtualizer layout options `None`→`Some` and estimate changes now relayout
  (deviations: upstream ignores both). Not a bug: `use_tag` tab stops (as upstream, `test_tag_group::tab_stops`).
- Leptonic's own `MaybeSignal<T>` (the user): a fixed value without allocation, tracking or render effect, or a
  `Signal<T>` (`utils/maybe_signal.rs`; why: `conventions.md`); first use: `Separator`'s `aria-orientation` (−11
  allocations, −1 render effect). `testing::count_allocations` for native tests.
- Per-key subscription pilot (R1) on the RadioGroup family: `utils/keyed_triggers.rs` (`KeyedTriggers<K>`);
  selecting in a group of 1,000: 4,008 → 16 render effects, the app selecting 16,014 → 56 allocations, the first
  selection 4,004 → 1,007 render effects; +4 allocations per radio at mount (`performance.md` R1, section 3).
- Button stack pilot: Button mount 586 → 301 allocations, 23 → 10 render effects; click 57 → 45; keyboard focus
  44 → 20; wasm −0.6% brotli (`utils/{probe,signal_attr,spread,data_attributes,stored_field,reactions}.rs`;
  `performance.md` R2, R4–R8, R11, R13, R14).
- Flaky tests fixed: `virtual_list::resize_grown` (the browser's scroll clamp on shrinking content counted as user
  scrolling: `use_scroll_view.rs`; `test_virtual_list::shrinking_content_keeps_following`), DnD TalkBack click before
  the drag session's frame (test race), slow role/text lookups (one in-page query, `tests/pages/lookup.rs`),
  `use_safely_mouse_to_submenu` (single-step pointer moves).
- `erase_components` measured (`build-performance.md`): kept for every build. reactive_graph (fork): notifications
  snapshot into a `Vec` instead of cloning subscriber sets.
- `Part<P>` (`utils/part.rs`: props built per mount, an owner-scoped lease: one element at a time) and `Id`
  (`utils/id.rs`) introduced; slots, field/label contexts, dialog, modal, menu, slider, listbox, grid list, select
  value, toast, tooltip, disclosure, text field, checkbox/switch/radio and calendar error converted;
  `test_remounted_parts.rs` (27 tests); `PartProps` removed; slot presence from the lease (no Effect).
- `use_hidden_date_input`'s container is a `Part<PropsWithStyles<..>>` (its styles were a field `attrs()` dropped);
  the `ssr` dead-code warning on `ScrollState::view_scroll` is gone.
- `chain_optional_callbacks` public (documented on the `use_press` page next to `merge_long_press`).
- Performance fixtures: one per subject, no shared helpers between fixtures or test modules.
- Documentation trimmed to long-term knowledge; `AGENTS.md` cut to the hard rules (the user). New rules: run only
  the subject's tests while working, fix bugs and flaky tests when found, performance priorities (wasm size,
  allocation count, allocated bytes), one performance fixture per subject.
- Documentation contradictions resolved on the documentation side: `design-collections.md` states C4's bindings
  and the kept `data-collection`; `hooks-implementation.md` no longer describes `CapturedElement`'s internals.
- Opt-level comparison of the test-app (leptonic-e5): counts identical at `opt-level = 3` and `"z"`, `3` makes the
  wasm 70% larger; decided: measure at `"z"` only (`testing.md`, `build-performance.md`).
- Performance tests for all 356 hooks and atoms (~2,330 tests, written by 9 parallel agents), by kind:
  `tests/perf_tests/{atoms,hooks}/<subject>.rs`, fixtures `/perf/{atoms,hooks}/<subject>`, budgets
  `tests/budgets/{atoms,hooks}/<subject>.json`. The library reports what ran (`metrics::subject!()`, feature
  `metrics`); a coverage check keeps `unmeasured.txt` shrinking. Findings in `PLAN.md`, "Performance".
- Performance counts with budgets in the regular suite: allocations, reactive work (`reactive_graph` `metrics`
  feature on the user's fork, branch `leptonic-metrics`), DOM mutations, layouts (`testing/perf-metrics`).
  `allocated_bytes` unbudgeted (16-48 byte jitter).
- Test runs centralized: one agent runs batched requests for all (`AGENTS.md`), after unbatched runs rebuilt the
  test-app per agent and a tree-wide fix script broke 40 files.

### 2026-10-13

- Both browser suites on the published crates (assertr 0.8, browser-test 0.7; `TestSettings`, 10-minute default time
  limit).

### 2026-10-12

- Virtualizer froze on a scrollbar loop (wrapping rows overflowing once measured): `ListLayout` keeps measured lengths
  as estimates when only the breadth changes, `use_scroll_view` bounds a size update to two passes
  (`leptos-and-dom.md`, "Effect Timing"); browser-test got per-test time limits and closes frozen pages.
- Performance suite (`just browser-perf`): in-page timings of `/perf` scenarios, first `perf/virtual-list`.
- Collection arena (`Collection` in document-order `Vec`s with a position map; `Node`'s sibling/child keys replaced
  by `Collection` methods); `ListLayout` continues after unchanged nodes: 20,000 rows build 4.9 → ~2 ms, an append
  following the end 4.75 → 0.6 ms of layout.
- Caches: `utils::recent_cache::RecentCache` for formatters, parsers, plural rules and localized messages (a plural
  message 6.4 → 0.9 µs); calendar cells share `use_today` and their strings.
- Quality pass over `leptonic/src` against the written rules: upstream headers and deviation blocks completed,
  `// No upstream:` for leptonic-only files, one helper per job (`aria_true`, `shadow_dom::node_contains`, merged
  duplicates), shadow-aware DOM queries, `Signal::derive` → `Memo`, `FocusScope` default class. Open findings:
  `PLAN.md`, "Quality pass".
- Fixed: `MenuItems`/`TagItems` re-render renamed items; `TabPanels` keeps its size properties in its `Styles`; a
  `CheckboxField` without `value` in a group warns instead of panicking.

### 2026-10-10 – 10-11

- Parts outside their parent warn and render nothing (`orphan_parts::*`, `/atoms/orphan-parts`); item hooks warn
  about keys missing from the collection.
- Id references reactive (C2: `MaybeProp<String>` in 83 atom props, `Signal<Option<String>>` in 35 hook inputs),
  id lists through `IdRefs`.
- `*Props`/`*Return` no longer `Clone` (the user); `TextField` tracks which element renders it.
- `data-focus-visible` has one writer per element (the user; `atoms-implementation.md`).
- Tooltip stays open while the pointer is on it after the trigger blurs (selectable tooltip text).
- `VirtualList` scrolls horizontally for wide rows (`ListLayoutOptions::allows_overflow_across`).
- Review of commit 07f628b1 (1,254 files) against react-spectrum `740c6c5c4a`: defects fixed in interactions, focus,
  collections, tables, forms and pickers; resize-triggered repositioning in the next frame (`leptos-and-dom.md`,
  "Resize Observers"); `TimeBound`; tests type through CDP with US key definitions and take the page's modifiers, so
  the suite runs on macOS hosts.

### 2026-10-09

- Review of every hook, atom and test against upstream 99e610236 (13 agents); per family fixed the same day:
  - Interactions: `use_press` click prevention after a long press, coordinates relative to the element, typed
    events, `EventHandler::into_on` (nothing attached for empty handlers).
  - Focus: one global modality state, FocusScope active-scope tracking as react-aria, `FocusManager` `Copy`.
  - Collections: per-key notification in `SelectionManager`, `FilterQuery`, `Key::id_fragment`, keyboard delegates as
    memos.
  - Grid/table/tree: GridList rows on `focusin`, `js_sign` column sizing, constant-time cell lookups, no doubled
    accessible names.
  - Overlays: hide-until-placed styles through `Styles`, `OverlayPositionOptions`, `use_modal` removed, `ToastKey`.
  - Buttons/links/tabs/disclosure: `IdRefs` and `labels` as the one way to join ids, `hooks/merged` deleted, ids
    known while rendering (`Button` `id`), `TabListState` as the tabs' context.
  - Color and slider: hue wrapping, states step from the current value, C13 types.
  - Calendar and dates: Gregorian formatting forced, times kept on date selection, `Time` bounds, picker state
    contexts, memoized formatters.
  - DnD and clipboard: untracked app callbacks, drag generation guard, 46 native DnD cases.
  - Virtualizer: binary-search layout, batched item measuring, `ItemSize`/`EndAnchor`/`VirtualListOptions`.
  - Cross-cutting: no prelude and one path per public item (the user), dead API removed, RTL from ICU4X.
- Refactor integration: the reworked APIs reconciled with the test-app, tests and book; final run 1,538 browser cases.
- Build warnings fixed, canonical imports, `just doc` warning-free.
- Slider percentage rounding as upstream; `ThemeProvider` restores the document's `data-theme` on cleanup.

### 2026-10-08 – 10-09: browser test suite rebuilt

From 139 tests sharing state to ~875 independent cases (`testing.md`): one case per test loading its own page
(68 had depended on earlier ones), explicit `TestGroup` registration with `#[browser_test]`, `Locator`s on
browser-computed roles, assertr's eventual assertions instead of polling macros, page health after every test,
session reuse with per-test resets, the test-app built `--release` with ICU4X data of its locales (821 tests 2m 40s →
871 tests ~35 s).

### 2026-10-07 – 10-08

- Tree tables (`role="treegrid"`, `TableExpandButton`), typed selection values (C16), localized strings from
  react-aria's 21 bundles in 34 locales (`utils::intl_strings`), the single `Checkbox`/`Radio`/`Switch` atoms removed
  for `*Field` + `*Button` pairs (the user), 12-hour times in the locale's clock.
- Fidelity review against 99e6102368, applied by nine agents: modals focus the dialog and set no `aria-modal`,
  `LinkButton` removed, `Propagation` only on press and keyboard events (the user); most hooks on one input (C8);
  `NumberParser` re-ported.

### 2026-10-07: components layer removed (the user)

Atoms with default classes `leptonic-<AtomName>`, the atom theme (`atom-theme.md`), then the deletion of the
`components`, `tiptap`, `sanitize` and `themes` features. Replacements: atoms (`Button`, `Link`, fields, `Select`,
`Tabs`, `Table`, overlays, date pickers, `ToastRegion`, `TagGroup`, ...), plain markup + CSS recipes in the book
(`Card`, `Stack`, `Drawer`, `Alert`, ...), `leptos-tiptap`/`ammonia` directly. All dependencies upgraded and declared
with `default-features = false`.

### 2026-10-05 – 10-07: roadmap R1–R5

The 2026-10-05 audit (18 bug and crash risks, 13 conventions to settle, ~78 untested hooks, 101 files on an old
upstream commit) worked off: R1 crash risks (`Href`, `OwnerAlive`, drop-guard listeners, `use_id`, SSR global
state, Effect Read Order); R2 conventions C1–C16 library-wide; R3 every family re-ported to 99e6102368 with deviation
blocks, tests from upstream and atoms (dates on jiff, the `time` crate gone; collections per `design-collections.md`);
R4/R5 modality tracking, assertr, parallel browser tests.

### 2026-10-04 – 10-07: found by the book and apps

Closing a focused dialog panicked ("Blur After Disposal"), random ids broke hydration (`use_id`), listbox ARIA and
focus fixes, `use_press` stopped every click (now upstream's propagation), Cmd + Home/End on macOS, the live announcer
as react-aria's singleton.

## Book

- 2026-10-17: adopted the library's `Part`s and `Id`s everywhere (demos, code samples, API tables, prose): parts
  named after their element, `attrs()` at the element that renders it (no stored or cloned attributes; a
  `StoredValue` of the part in nested children closures), `props()` only where a demo reads or changes props; new
  "Parts" and "Ids" sections in Hooks & Atoms, changelog entries, `id` rows for the eight atoms, `presence`,
  `collection_id` and the grid list section's `header` part.
- 2026-10-09: refactor integration across API tables, demos and prose; the browser harness honors `BOOK_TARGET_DIR`;
  wasm precompressed at server startup in every profile (`src/assets.rs`).
- 2026-10-08: guide "Optimizing Compile Times & Binary Sizes"; browser tests on browser-test 0.6, one test per check
  (240 tests in ~32 s); `wasm-dev` at `opt-level = "z"`; tree tables and typed selects documented.
- 2026-10-07: off the components layer (atoms + book CSS, recipes); re-ported library documented; dependencies
  trimmed (TLS on `ring`); theme kept in a cookie and rendered by the server; dev wasm no longer compressed per request.
- 2026-10-04 – 10-06: all ~130 pages on the page kit, the book built with leptonic, `kit::api_check`; navigation as
  Guides, Concepts and Building blocks (`nav.rs`).

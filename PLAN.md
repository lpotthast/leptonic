# Leptonic: working plan

The only todo list of the repository: open work of the library (`leptonic/`, `leptonic-theme/`, `testing/`) and
the book (`examples/book-ssr/`, section "Book"). Open items only: finished work goes to `documentation/history.md`,
long-term rules and knowledge elsewhere:
- guiding decisions and API conventions (C1–C16): `documentation/conventions.md`;
- implementation patterns: `documentation/{hooks,atoms}-implementation.md`;
- Leptos/browser/tooling pitfalls: `documentation/lessons.md`;

## Waiting on the user

- **Leptos issues to file** (repros and issue drafts in `~/dev/leptos-issue-repros/`; file them?):
  - `a-effect-skips-run` (reactive_graph 0.2.15, also 0.9.0-beta2): an effect run is skipped when a memo recomputes
    while the effect checks a downstream memo that comes out unchanged. Leptonic reads upstream first ("Effect Read
    Order", `leptos-and-dom.md`).
  - `b-spread-attrs-lost` (passes on 0.9.0-beta2: ask for a 0.8 fix): attributes spread onto a component are lost
    when its root element is replaced in `erase_components` builds (`lessons.md`).
  - `c-spread-listener-leak` (tachys 0.2.19; passes on 0.9.0-beta2): `AnyViewWithAttrs::rebuild` keeps the old
    spread attributes' handlers (disposed owner: they panic) and drops the new ones. Known-issue case
    `component_spread_rebuild_known_issues`; atoms avoid needing such spreads meanwhile.
  - `d-keyed-for-moves` (also 0.9.0-beta2): the keyed `For` diff moves kept rows whose relative order didn't change,
    collapsing text selections (why `render_visible_items` mounts rows itself).
- **The user's crates:** leptos-styles: `add_reactive_unchecked` (today `add_optional_unchecked(prop, move ||
  Some(..))`), leptos-css lacks `Margin::all`-style helpers. leptos-element-capture: identity `PartialEq`/`Eq`/`Hash`
  for `CapturedElement`, `try_get_untracked` for deferred callbacks. leptos-classes: `classes="a b"` compiles but
  panics at runtime (`From<&'static str>` for `ClassName`): split whitespace-separated names, or reject them at
  compile time?
- **Decisions:**
  - `icu_experimental` as a dependency, for a CLDR-aware number formatter ("Number formatting" under Families).
  - The shape of `UseFormValidationInput.focus` (today `Callback<()>`, C10; "Forms" under Families).
  - Untrack the no longer generated `testing/test-app/style/leptonic` (106 tracked files; `git rm -r --cached`).
  - Remove `examples/book-ssr/style/leptonic/` (55 tracked files: an old copy of leptonic's styles incl. the removed
    components and themes, referenced nowhere)?

## Library roadmap

Order: open bugs first, then families (each a complete package: API onto the conventions, missing react-aria
features, re-sync to the current upstream commit, deviation block, fixture + browser test from upstream's tests,
atoms, book heads-up), then cross-cutting work.

### The atom theme
- [ ] The atom theme (`leptonic-theme/scss/atoms/`, `documentation/atom-theme.md`): every family with atoms is
  ported, none checked in a browser yet. Open:
  - A showcase to check it visually (light and dark, every family), e.g. an `examples/atom-theme` app with the
    starter's markup per family; then fix what's off.
  - Atom gaps the stylesheets need: `Table` root `data-focus-visible`, `TableBody` `data-empty`, column resizer
    `data-focus-visible`/`data-focused`/`data-hovered`; custom column header content (a flex wrapper and a sort
    indicator; the resizer rules assume it); a `SelectionIndicator` atom (sliding tab indicator, OMITTED in
    `atoms/tabs.rs`: needs a shared-element transition, snapshot-before-unmount ordering in Leptos unclear);
    `DateInput` with `part`
    rendering `slot="start"`/`"end"`; `CalendarCellButton` children as a function of the cell state (range
    start/end, hovered, pressed, the formatted day); a `ToastQueue` `wrap_update` hook (view transitions); the
    table's selection checkbox (a raw `<input>` in `TableCell`) and `GridListItem`'s as `Checkbox` atoms; links
    (`href`) in list box/menu/grid list/tag items.
  - Decide: `ColorPicker` renders no element (the theme's `.color-picker` trigger is the app's class, and a
    `leptonic-Button` trigger brings button looks it doesn't undo); calendar buttons and the toast close button are
    quiet only with `attr:data-variant="quiet"`, maybe quiet by default in the theme; the `Disclosure` trigger is a
    `Button` (the theme undoes button looks).
  - Not ported (no atoms yet): `CommandPalette`, `DropZone`, `NavigationTree`, `SegmentedControl`, `Sheet`,
    `TokenField`, `Tree`.

### Virtualization
- [ ] Virtualizer, rest: virtualized `GridList`, `Table`, `ComboBox`/`Select` popovers and menus (RAC's virtualized
  `GridList`/`Table`/`ComboBox`/`VirtualizedMenu` tests), sections/headers in virtualized lists,
  `GridLayout`/`TableLayout`/`WaterfallLayout`, drop targets. `VirtualList` showing a growing log (20,000
  lines, 10–100 appended per second): an append rebuilds the collection and lays out again. In the browser
  (`just browser-perf`, `perf/virtual-list`, 2026-10-12; headless Chrome, the user's Mac, wasm `opt-level = "z"`
  without debug assertions): 50 lines appended to ~21,000 cost ~3.0 ms until settled following the end and
  3.4–3.9 ms at the start, plus 0.1–0.3 ms of the next frame's work (native: build ~2 ms, layout 0.2–0.6 ms; why
  the start costs more in the browser is open); 100 appends per second would take ~30–40% of the main thread, so
  apps appending that often should batch appends (one per animation frame). Where the time goes and the route to
  O(k log n) appends: "Virtualizer" below.
- [ ] `VirtualList` counts (budgets in `leptonic/tests/budgets/atoms/virtual_list.json`, 2026-10-16), to bring
  down: mounting 20,000 lines makes 67,418 allocations (27.4 MB allocated, 15.6 MB still
  held: ~780 bytes per line, though ~20 rows render) and takes 8 layouts and style recalculations over 4 frames;
  appending 50 lines while showing the start (nothing visible changes) makes ~21,830 allocations (8.9 MB, 1
  layout), following the end ~24,170 (9.1 MB, 2 layouts over 3 frames). Changing a grown log's width: ~4,900
  allocations (1.9 MB), 3 layouts. Look for
  forced layouts (more layouts than frames) and what the per-line allocations are (collection nodes, keys, maps).

### Performance: findings of the performance tests (2026-10-16)
The performance tests of all 356 hooks and atoms (`leptonic/tests/perf_tests/`, budgets in `leptonic/tests/budgets/`)
show where leptonic is slow. Fix the causes, then lower the budgets (`BROWSER_TEST_PERF_BUDGETS=update`).
- [ ] Per-item work on every change of a collection or group: per item, memos that read the whole value or
  collection rerun on every change, so a change costs O(n) (and mostly reruns to "unchanged"). A keyed selector
  (per-key subscription; the controlled single selection already has one: 42 allocations) or memos that read only
  the item's own data would make these constant. Evidence (1,000 items unless noted):
  - RadioGroup: one selection 4,004 render effects, ~39k allocations; `use_radio_group` an arrow key 2,000 render
    effects. CheckboxGroup: one click 3,004 render effects, 52k allocations; `use_checkbox_group` checking one 26k
    (every item calls `state.is_selected(&value)` on the whole `HashSet`; `add_value`/`remove_value` clone it).
    Disabling a group ~30k, 4,000 effects. A user's click costs 3-4x the app setting the same value (suspected:
    the commit validates every item's native validity, `use_checkbox_group.rs`; unconfirmed).
  - Tabs (100): a selection change reruns every `TabPanel`'s `is_selected` memo (3,500 allocations instead of 900).
    ColorSwatchPicker (100): adding a color 16k, 614 memo runs. DisclosureGroup: each toggle 1,000 memos, 20k.
  - Collections: appending one option to a ListBox 92k (4,010 memos; `is_disabled`/`is_link` in
    `use_selectable_item`, `use_node_aria_label`, the link memo in `use_option`); adding a menu item 95k; removing a
    tag 142k; adding a grid row 260k (11k memos; the rebuild itself ~12k), a hook table row 683k; renaming one
    section header (200 sections) 104k.
  - Drag and drop: a keyboard drag start over 1,000 droppable rows 169k allocations, 9,014 memos, 4,023 attribute
    writes; every drop target change 2,002 memos (22-32k); native drag start of one row 1,000 render effects (each
    row's `is_dragging` reads the shared `dragging_keys`); Tab into a draggable list 2,003 memos (the drag modality
    turning keyboard reruns every row's description). `register_drop_item`/`register_drop_target` `retain`-scan
    every registration on each registration: O(n²) mounting (read in the code, not measured).
  - Calendars: one arrow key in 12 months reruns every cell's memos (14k, 639 memo runs; 1,479 in one month);
    selecting a date 38k; paging 10k (one month) and 123k (twelve).
- [ ] Mount costs per item: CheckboxField ~1,140 allocations (~62 KB), RadioButton ~800, Button-based atoms
  450-610 (every Button runs `use_context_menu`, `use_keyboard`, `use_focusable`, `use_focus_ring`, `use_hover`,
  `use_press`; `use_press` alone 144), ToggleButtonGroup ~600 per button, Tag ~1,370 (twice a ListBox option, ~720),
  Table row ~4,500 (Grid row with 3 cells ~2,100, GridList item ~800), calendar cell ~950 (25 render effects; a
  12-month Calendar ~400k), color swatch ~1,500, menu item ~890, disclosure in a group ~1,060, toast ~800, tab
  ~730, NumberField 2,840 (TextField 1,142). Look for what each item allocates that could be shared or lazy.
- [ ] Specific costs: toggling the Table's selection column re-renders every cell (4.1M allocations, 85k render
  effects at 1,000 rows: cell keys and indexes shift); Select and ComboBox popovers render every option (opening
  with 1,000: 720-738k; not virtualized); type-ahead without a match costs ~18 allocations per item (collation);
  a rejected character in a NumberField 440 allocations and 105 reallocations (`beforeinput` →
  `is_valid_partial_number`); every keystroke copies a text value about five times (50 KB for 10,000 characters);
  opening a DatePicker ~42k (mounting the calendar); disabling a Link rebuilds it (`<a>`/`<span>`, the router's
  `<A>`); the first key press on a page makes ~40 allocations once (probably platform detection,
  `utils/platform.rs`); opening a 10-item menu with Enter 10,669 allocations and 5 layouts; SliderMarks re-render
  every mark on a range or step change (keyed by value); changing a collection re-lays out a virtualized list
  (appending 10 to 10,000: ~40k).
- [ ] Bugs the performance tests found:
  - `AvailableRange::around` reads a signal with `.get()` outside a tracking context
    (`use_range_calendar_state.rs:148`, from `focus_nearest_available_date` when a range starts with Enter in a
    RangeCalendar with `is_date_unavailable`): a Leptos warning. Then restore the unavailable date and its test in
    the RangeCalendar performance fixture.
  - Parts that take their props once (`Option::take`) can't mount a second time inside their parent:
    `CalendarCellButton`, `DateInput`, `DatePickerGroup`, `ToastDescription`, `ListBoxSectionHeading` (which never
    gives them back on cleanup, unlike `slot()` in `listbox.rs`).
  - `ColorChannelField`: the scroll wheel never steps it (`use_number_field` puts the handlers on `group_props`,
    which the atom drops).
  - `use_tag.rs`: every tag's `tabindex` depends on `!selection.has_focused_key()`, so all tags are tab stops until
    one is focused, and the first focus rewrites every tag (1,009 attribute writes at 1,000); the comment says "The
    first tag is the tab stop".
  - Virtualizer layout options from `None` to `Some` are ignored (`virtualizer_state.rs` ~331: `(Some(new), None)`
    falls into `_ => false`; upstream invalidates). Changing an estimated row size keeps the rows laid out before
    at the old estimate (maybe a deviation).
  - `data-focus-visible` documented but not rendered: `NumberFieldGroup` (`number_field.rs:248`, though
    `ring.is_focus_visible` is there), `NumberFieldIncrementButton`, `NumberFieldDecrementButton` (`stepper_button`),
    `SearchFieldClearButton` (`search_field.rs:204`).
  - `attr:id` on `ListBox` doesn't reach its element (its view is wrapped in providers).
  - Dangling ARIA references when a part unmounts: `SelectValue` (the trigger's `aria-labelledby`), an open
    `SelectPopover`/`ComboBoxPopover` (`aria-controls`), `DisclosurePanel`, `DisclosureTrigger`, `Tooltip`.
  - Check against upstream: `NumberFieldState::increment` ignores `is_disabled`.
  - Verify and document (`leptos-and-dom.md`): `style=styles` plus `style:width=` on one element dropped the width
    (like the `class=` rule).

### Bugs and review leftovers
- [ ] Flaky (2026-10-16): `test_dnd::talkback_click_on_the_drag_source_cancels`: after the TalkBack click that should
  cancel the drag, the source keeps `data-dragging="true"` (5 s); 1 of 3 runs alone, once in the full suite. Find
  whether a click right after the drag start is lost (library race) or the test races.
- [ ] Flaky under the full suite's load (2026-10-16): `perf_tests::atoms::virtual_list::resize_grown` once counted
  more than its budget (+2,025 allocations, +1 memo run, +3 effect runs over 30 resizes); deterministic alone (3 of
  3) and in 8 earlier runs. Find which resize does extra work (log the per-window counts) and why.
- [ ] Flaky: `overlay_position` cases fail under load in the full suite (no `[data-placement=bottom]`/`top]` within
  10 s; `placed_above`, `reopened_with_arrow`), pass alone. No root cause yet; `placed_above` waits in steps
  (`aria-expanded`, presence, placement) so the next failure shows which. The hide-until-placed styles no longer get
  wiped by `--trigger-width` (fixed 2026-10-09). Not reproduced in the 1,538-case integration run or the subsequent
  1,543-case Just run; investigate if it returns under load.

### Quality pass 2026-10-12: open findings
A rule and consistency pass over all of `leptonic/src` (what it fixed: `history.md`). Every bug fix starts with a
failing test.
- [ ] Bugs:
  - Two more writers of `style`: `use_safely_mouse_to_submenu` sets `pointer-events` on the menu element, and
    `UseVisuallyHiddenProps` writes the whole `style` attribute as a custom attribute (a caller's `style=` clashes;
    return `PropsWithStyles` with `Styles`). Both public API.
- [ ] Microtask workarounds that may be obsolete: `use_selectable_collection`'s `on_focusin` and `use_toolbar`'s
  `on_focus_capture` defer focus to a microtask because "Leptos handlers can't be re-entered", but both attach through
  `EventHandler::into_on` (re-entrant; `leptos-and-dom.md`, "Nested Event Dispatch", even names `on_focusin` as its
  reference). Try focusing synchronously in a browser run; drop the comments and the toolbar's deviation entry.
- [ ] Public API inconsistencies:
  - Missing props other field atoms or RAC have: `ComboBox` `id`/`aria_describedby`; `ColorChannelField`
    `auto_focus`/`is_wheel_disabled`/`on_focus_change` (hard-coded); `ColorSwatch` `id`; `DatePicker`/
    `DateRangePicker` roots lack `data-focus-within`/`data-focus-visible` (only `DatePickerGroup` has them).
  - Default classes undecided for hidden helpers: `DismissButton` (own `<button>`, no `classes`/`styles`),
    `HiddenSelect` (own `<div>`); table column headers and resizers have a hard-coded class and no `classes`.
  - `MenuSection` renders its own `<header>` (`heading_classes`), ListBox has a `ListBoxSectionHeading` part: a
    `MenuSectionHeading` part (single-element rule).
  - Parts used twice: a second `CalendarCellButton` renders nothing, a second `GridListItemDescription`/
    `GridListHeader` renders without its props, a second `ToastDescription` renders nothing silently: one rule
    (warn). `dev_warn!` wording varies ("must be inside", "belongs in", "X: not inside a Y"): normalize.
  - C9: `UseClipboardReturn::clipboard_props` (the hook's own element: `props`). C11: plain `bool`s
    `UseTextFieldInput::exclude_from_tab_order`, `is_wheel_disabled` (number, color, color channel field inputs).
  - `Debug`/`Clone` missing on `*Input`s (`hooks-implementation.md`): text, number, search, color and color channel
    field inputs, `NumberFieldState`; `UseToastInput`/`UseToastRegionInput` need `Debug` on `ToastQueue`;
    `ListFormatter` has none. `CheckboxGroupContext` (and its `data` field) is `pub`, looks internal.
  - `live_announcer`: `Assertiveness::default()` is `Polite`, react-aria's `announce` defaults to assertive
    (recorded as a deviation): keep or change?
  - `radio.rs`/`switch.rs`/`checkbox.rs`: one-caller `use_*_atom(XSetup { .. })` helpers: inline all three.
- [ ] Deviation blocks: entries without a reason (couldn't be inferred): `use_grid` `getRowText`, `list_layout` AI
  layout bottom alignment, `use_table_row`/`use_table_state` synthetic link props, `expandedKeys: 'all'`,
  `UNSTABLE_useFilteredTableState`, `use_droppable_collection` "drop handlers run synchronously", several in
  `dnd/types.rs`, `use_breadcrumb_item` `autoFocus` (portable: `UseLinkInput` has `auto_focus`); atoms
  `separator`, `toolbar` (render props), `slider` (values a `Vec`), `menu` (`MenuSection` keeping disabled keys),
  `popover` (`isEntering`/`isExiting`, `shouldSkipAnimation`, `PreviewTrigger`), `table`. State hooks repeating C3/C4
  (`porting.md`: record once): trimmed in calendar/datepicker; most others carry hook-specific detail next to it
  (selection manager, select/combo box/menu trigger/tab list/tree states, toggle, text field, radio/checkbox group,
  number field, color states): decide a uniform trim.
- [ ] Upstream tracking: the intl bundles' header (`// Generated by ... from react-aria/intl/<x>/*.json @ <sha>`)
  isn't read by `upstream-drift.sh`; have `scripts/port-intl-strings.py` emit `// Upstream: react-aria/intl/<family>
  @ <sha>` too (regenerating at react-spectrum's current HEAD changes translations: review).
- [ ] Module docs: 141 of the 390 source files have a `//!` doc. Decide whether every module gets one.
- [ ] `use_press.rs`: the middle (about lines 700-1700) was only scanned in this pass (raw targets, unwraps, derives),
  not read in full.

### Rules the code contradicts (decide: fix the code, or change the rule)
Found 2026-10-09 while checking the documentation against the code; the documents were left unchanged.
- [ ] `design-collections.md` vs the code: §4.11 "no `selectedKeys`/`expandedKeys` controlled props, only `default_*`
  + manager methods", but selection, expanded keys, sort descriptor, select/combo box values and tabs bind app state
  through `ValueBinding` (C4, the later rule): reword §4.11. §4.11 also says the collection deviations are recorded
  globally in `hooks/mod.rs`; only C4 is, the rest per file. §4.3 said `data-collection` goes away; items still
  render it (a `use_id` id telling nested collections apart): confirm and update the design. Decision 1 and §4.10
  promise typed item collections (only values are typed; "Typed collections" under "Collections core").
- [ ] Book: "every hook has a page" (`hooks-implementation.md` used to say so): ~166 hook files, 74 hook pages in the
  book (pages cover several hooks; unverified which hooks have none).
- [ ] Test rules (`testing.md`):
  - "`with_subject_name` only for a runtime value the expression can't show, in helpers": helpers use fixed names
    ("inner text", "focus"), and `test_tabs`, `test_label_slots`, `test_forms`, `test_slider`, `test_virtual_list`
    call it directly.
  - "Fixture-specific actions only for a fixture with an id convention": `ClipboardActions` (CDP clipboard access)
    and `VirtualListActions` (selection, scrolling) aren't.
- [ ] `hooks-implementation.md`'s "Element Capture Pattern" describes `CapturedElement` as "StoredValue + Trigger" (crates.io 0.1.0); the local
  `leptos-element-capture` checkout is `RwSignal`-based: update the text when leptonic moves to it.

### Review 2026-10-09: findings by family
A read-only review of every hook, atom and browser test against upstream 99e610236 (13 agents). Line numbers are as
of 2026-10-09 and drift; find code by name. Every bug fix starts with a failing test.

#### Cross-cutting (patterns several reviewers found)
- [ ] `Signal::derive` where a `Memo` is needed (rule: `leptos-and-dom.md`, "Derived Values"), the cases left after
  the 2026-10-12 pass, each blocked by "Effect Read Order" (convert together with reading the collection or value
  first in the readers, plus a browser case like `value_and_items_derived_from_one_signal`): the bound selection of
  `use_select_state` and `use_combobox_state` (the hidden select's options memo and the combo box reconciliation
  read it before the collection); the calendar states' `is_value_invalid` (every cell's `is_invalid` memo reads it
  before the value) and `visible_range` (`weeks_in_month`). `use_grid_list_item`'s `on_action` stays a derive
  (`Callback` has no `PartialEq`).
- [ ] `utils/intl_strings`: messages are parsed once per thread and number formatters, plural rules and the
  locale's messages are cached (2026-10-12); left: one `Memo` per `use_localized_strings` call in per-item hooks
  (`use_table_row`, `use_tag`, `use_tree_item`, DnD items, date segments; the calendar cells share theirs through
  `CalendarData`), and `scripts/port-intl-strings.py` could emit pre-parsed messages (or one fn per message, as
  upstream's string compiler) so nothing is parsed at runtime.
- [ ] `ThemeProvider` (`atoms/theme.rs`): replace `signal_ls` with leptos-use's SSR-aware, cross-tab
  `use_local_storage`, then drop `Theme`'s serde bounds/derives (`Display`/`FromStr` on `LeptonicTheme` instead).
  Update the book's themes/installation pages.
- [ ] Better than react-aria, ranked by value/effort:
  1. `Validator<T>` with `From<F>` for closures returning `Result<(), E: Into<ValidationErrors>>` (today
     `validate=Arc::new(|v: &String| ..)`, `ValidateFn<T>`); pairs with the `Validation` enum (Forms).
  2. Typed item collections: `<ListBox items=fruits key=|f| f.id text=|f| f.name.clone()>{|f: &Fruit| ..}`, the
     collection and the views from one source; only the atom wrapper generic, hooks stay `Key`-based; with C16
     `S: SelectedValues` replacing `selection_mode` + `disallow_empty_selection`.
  3. `Tabs<V: SelectionValue>` (an enum of pages; exhaustive panel `match`), `DisclosureGroup<V>`, typed
     `on_action` on Breadcrumbs/GridList/Menu (`typed_values.rs` exists).
  4. Pre-compiled localized strings (above).
  5. `#[slot]` for fixed two-part structures (`TooltipTrigger`, `DialogTrigger`, `SubmenuTrigger` take the overlay
     as a required slot: a missing/duplicate overlay fails to compile instead of a runtime `dev_warn!`).
  6. Ids known at render time (a `PressResponderTrigger` fallback id, `Button` `id` defaulting to a generated id):
     trigger/panel relations in the SSR HTML instead of `ensure_element_id` effects.
  7. Typed `*StateContext`s (`DisclosureState`, `DatePickerState<V>`, `SliderState<T>`, tab/toggle render state)
     as the typed replacement for RAC's render props.

#### Focus
- [ ] Open from the review (f-focus, 2026-10-09; the rest is in history):
  - FocusScope in another document (FocusScopeOwnerDocument.test.js): listeners use `use_document()`, not the
    scope's owner document.
  - RAC `ShadowDOMFocus.browser.test` (ComboBox/NumberField/Menu with `prevent_focus_on_press` in a shadow root):
    not ported (needs shadow-root lookups in `tests/pages`).
  - A public focusable-elements `Iterator` (upstream exports `getFocusableTreeWalker`).

#### Overlays
- [ ] API: `ToastQueue<T: Clone + Send + Sync>` (`use_toast_state.rs`) can't hold non-`Send` content: make it
  storage-generic (`LocalStorage`).
- [ ] Tests still missing (inventory 2026-10-09):
  - useOverlayPosition.test.tsx: "should close the overlay when the trigger scrolls" for a popover (the tooltip
    covers the same path) and the "positioned container" cases (need a hook fixture: the atoms portal to `body`).
  - Popover.test.js "should not leak PopoverContext to nested popovers", "supports overriding styles", "should
    support custom Pressable trigger"; Tooltip.test.js "shows on hover when button has isPending", "supports
    overriding styles", "should support custom Focusable trigger on hover", the two tab-order cases.
  - Dialog.test.js onEnter/onExit variants (entering until an `on_enter` animation ends, both exits awaited,
    standalone modal), "should not close Modal when DateRangePicker is dismissed by outside click" (with a date range
    picker); Modal.browser "does not scroll the modal into view when the backdrop is pressed".
  - useOverlay/useModalOverlay touch variants; useOverlay.shadow (outside presses in a shadow root).
  - useTooltipTriggerState.test.js (close delay longer than default / zero, warm-up called twice, controlled
    tooltips); useToastState "three toasts, remove the middle via timeout" (browser); useLandmark: one landmark,
    nested last, added as parent, Alt+F6 without main / main only, mouse focus, focus after blur, controller
    listeners, components handling focus; runAfterTransition.test.ts; useViewportSize/useLandmark/Dialog `.ssr`.

#### Forms and fields
- [ ] API:
  - `ValueBinding::from_state_props` returns a tuple, and every state input keeps a `default_x` "ignored when bound":
    `enum StateSource<T> { Owned(T), Bound(ValueBinding<T>) }` + a named `StateProps<T> { source, on_change }`.
  - `ValidationResult` can be `is_invalid: false` with errors: `enum Validation { Valid, Invalid { errors, details:
    ValidityFlags } }` (bitflags for the 11-bool `ValidityStateSnapshot`). Typed validators `impl Fn(&T) ->
    Result<(), E>` with `E: Into<FieldErrors>` (today `Result<(), Vec<String>>`), rendered by `FieldError`; server
    errors as a `ServerErrors` newtype (optionally keyed by a field-name enum), which also answers "several names per
    validation state" (Families, Forms).
- [ ] Tests:
  - A custom `validate` with `ValidationBehavior::Native` is never browser-tested (fixtures with `validate` use
    `Aria`), nor "invalid event defaultPrevented → no autofocus" (`use_form_validation.rs`; RS TextField.test.js:599,
    788).
  - NumberField validation (RS NumberField.test.js:3142-3462: steppers commit, validate, server via submit, custom
    message, commit on blur only if changed, aria, form reset): the atom fixture has no `validate`, no `Aria` field,
    no reset button. `number_field_atoms::validate_commit_behavior` lacks the announcement checks and upstream's final
    "30" after the empty commit (RAC NumberField.test.js:508). Missing: currency onChange/announcements, paste in
    another numbering system, commit-on-blur clamp/snap, read-only can't step (`read_only_state` checks only
    `data-readonly`), compositionend revert; useNumberFieldState.test.ts:18/48 (formatOptions stability).
  - `useFormReset.test.tsx:36` (reset listener stops propagation) and `:97` (reset cancelled in capture).
  - Groups: `test_radio_group::validation` lacks focus after `checkValidity` and all-inputs-valid (RAC
    RadioGroup.test.js:617, 659); RS Radio.test.js:898 (keyboard; the fixture has one enabled radio); group
    server-error/`validate` fixtures (RS CheckboxGroup 566-836); the group description/error in `test_forms.rs`.
  - `data-pressed` (mouse hold, Space) never tested for Checkbox/Switch/RadioGroup; focus event logs; `form` prop.
  - No disabled TextField/SearchField fixture; TextArea variants mostly untested.
  - `test_label_slots.rs` names upstream files (useLabel/useField/FieldError tests) it mirrors no case from.

#### Collections core, ListBox, Select
- [ ] `Select`: add the `auto_focus` prop.
- [ ] API:
  - `use_select_state`: generic over `S: SelectedKeys` (`Option<Key>` | `Vec<Key>`), drop `selection_mode`.
  - `ListBox` requires no `collection` (warns without one outside Select/ComboBox): a required `collection`; Select
    and ComboBox render an internal `ParentListBox`.
  - `ListBox` props static (C11): `layout`, `should_focus_wrap`, `escape_key_behavior`, `disabled_behavior`,
    `aria_labelledby`; missing `disallow_type_ahead`, `should_select_on_press_up`, `should_focus_on_hover`, focus
    callbacks, `id`, the `FocusScope`.
  - Hidden select form values stay `Key::to_string()` (`Key::from(1)` and `Key::from("1")` submit alike; mapped back
    to the first item): decide whether to prefix by kind (servers would see `%i1`). No Uuid keys.
  - Typed collections (`ListCollection<T>`, `ListBoxItems<T>`, `on_action: Callback<T>`, a `ListBoxCollection`
    renderer); `SelectValue` render function (RAC "supports custom select value").
  - `KeyboardDelegate`/`LayoutDelegate` don't require `Debug` (inputs holding them have manual impls).
- [ ] Tests still missing (gap list 2026-10-09): ListBox focus ring/press states (`data-focus-visible`,
  `data-pressed`, not on non-interactive options), Meta+Arrow and Ctrl+Shift+Home/End, Escape in a modal clears
  first, missing-label warning (feature), `aria_labelledby` prop; Select: opening on touch up, clicking outside,
  `activeElement.blur()` keeps it open, extra children reading `SelectContext`, customized native messages, server
  HTML of the default value, links in a Select, in-app router links.
#### ComboBox, Menu, TagGroup
- [ ] API: `Menu` takes `collection: Option` + `state: Option` and warns at runtime (one required `MenuItemsSource`
  enum). Better than react-aria: typed menu actions `Menu<A: SelectionValue>` with `on_action: Callback<A>`
  (exhaustive `match`); `SubmenuTrigger` rendering its own trigger item (no key-match `dev_warn`); a typed combo box
  entry `enum ComboBoxEntry<V> { Selected(V), Custom(String), Empty }`; `MenuSection<S: SelectedValues>`; generic
  `TagGroup<V>` with `on_remove: Callback<Vec<V>>` in collection order.
- [ ] Tests: no hooks-level combo box fixture (`use_combobox` + `use_combobox_state` without the atoms).
  `ComboBoxValue` has no `valueProps` id (react-aria's `useValueId` describing the input), see `use_combobox.rs`.

#### Grid, Table, GridList, Tree
Done 2026-10-09 (f-grid, see history); open:
- [ ] A column header inside a `TooltipTrigger`: `TableHeader` renders the headers itself; needs custom column header
  content (a `TableColumn` render function) and a tooltip outside the `<tr>`: design first.
- [ ] Low: `KeyboardNavigationBehavior` lives in `hooks::gridlist` but grid and table use it.
- [ ] Tests still missing: GridList dynamic collections / moving items between sections (GridList.test.js:526, 687),
  links in trees (Tree.test.tsx:1549), Tree `focusMode`/`allowsArrowNavigation` cases (2944-3100), Table "does not
  hang restoring focus" (1167); resizing `max_width`, `ColumnBound::Percent`, changing container width, smaller after
  `minWidth` shrinks (tableResizingTests.tsx:149-169, 292-340, 511, 598-660).
- [ ] API (better than react-aria): typed tables `Table<T, C: ColumnId>` (columns an enum, rows `Signal<Vec<T>>`,
  cells `Fn(&T, C) -> AnyView`); `SortDescriptor<C>` + a `sort_rows` helper with the locale `Collator`; persistable
  column widths (`column_widths` + `set_column_widths: Out<..>` on `ResizableTableContainer`); data-driven
  `GridList<T>`/`Tree<T>`; `Table` accepting any signal/closure and memoizing inside.

#### Drag and drop, clipboard
- [ ] API: typed drag payloads (`DragFormat` trait: `const MIME`, `type Data: Serialize + DeserializeOwned`;
  `DragItem::typed::<F>(&data)`, `TextDropItem::get::<F>()`); a public `DropOperations` bitflags type with
  `preferred` and `DropResult::{Dropped(op), Canceled}` replacing `allowed_drop_operations:
  Option<Signal<Vec<DropOperation>>>` and its order-dependent defaults (a `pub(crate) DropOperations(u8)` exists in
  `hooks/dnd/types.rs`); the drag preview as a `ViewFn` rendered off-screen for `setDragImage`;
  `use_copy_to_clipboard() -> (Callback<String>, Signal<CopyState>)` with a self-resetting "copied" state.
- [ ] Tests: touch descriptions ("Double tap to ...", "Long press to ...") are checked natively only
  (`messages.rs`): a browser case needs a coarse primary pointer (`(pointer: coarse)`), i.e. CDP touch emulation
  that session resets undo. DnD auto-scroll (WebKit-only) stays untested (`test_dnd_collection.rs` header).

#### Virtualizer
- [ ] Long term (not done, bigger redesigns): a flat-list layout for `VirtualList` (no `Collection`: `Vec<Key>` +
  key → index + a Fenwick tree of row lengths; append k rows O(k log n), visible range O(log n + v)), with an
  append-aware source (`VirtualListItems<T>` with `push`/`extend`/`drain_front` and a change generation) instead of
  `items: Signal<Vec<T>>`. Appends cost an O(n) comparison of the top-level nodes (no lookups; the layout continues
  after the unchanged ones; ~0.6 ms at 20,000 rows, end-anchored) plus `Collection::build` (~2 ms); trimming the
  start moves every row (~5 ms).
- [ ] `layout_info.rs:24` `transform: Option<Arc<str>>` stringly CSS.
- [ ] Sectioned lists: continuation of partial builds is disabled for them (sections may be laid out partially
  anywhere); virtualized sections/headers aren't rendered by the atoms anyway.

#### Calendar and date pickers
- [ ] Fidelity:
  - Leading zeros: `format.rs` pads months/days/hours as ICU4X's short pattern does (de-DE "05.06.2024"); `Intl`'s
    numeric fields follow CLDR's `yMd` skeleton, padded per locale and field (de-DE "5.6.2024", bg "5.06.2024",
    en-GB "05/06/2024"). ICU4X ships no `availableFormats` (its baked patterns are per length, and its data is a
    trie, not readable per skeleton). Options: a per-locale table of month/day/hour padding generated from `Intl`
    (Node's ICU equals Chrome's) for the locales ICU4X supports, or wait for ICU4X skeleton data. Documented in
    `format.rs`'s deviation block (2026-10-09).
- [ ] API:
  - Calendar systems through ICU4X `AnyCalendar` (Buddhist, Japanese, Persian, Hijri, ...; `format.rs`
    `preferences()` forces Gregorian today): keep jiff values, convert for display and editing; `IncompleteDate`
    holds era, month code and era year (upstream's `toCalendar` model). Decide together with "ICU4X calendars" under
    Build performance.
  - Granularity tied to the value type: `trait DateValue { type Granularity }` (`Date`: zero-sized `DayOnly`;
    `DateTime`/`Zoned`: `Hour | Minute | Second`): `Granularity::Hour` on a `Date` (silently Day today) stops
    compiling, `resolve_granularity` goes.
  - Typed segment edits: `enum SegmentValue { Era(Era), DayPeriod(DayPeriod), Number(i32) }` instead of
    `DateFieldState::set_segment(kind, i32)` with era-as-index and day-period 0/1.
  - One range type: `utils::date::DateRange` and `datepicker::RangeValue<V>` duplicate each other (`RangeValue<T>`
    everywhere); a generic `Calendar<V: DateValue = Date>`/`RangeCalendar<V>` keeping times (upstream
    `normalizeValue`) drops the picker glue (`CalendarPickerContext`/`RangeCalendarPickerContext`).

#### Color and slider
- [ ] Fidelity left: `use_color_field.rs` no `name`/`form` on the visible input (the atom's hidden `#RRGGBB`
  input submits; `form` only with `name`): document in hook and atom deviation blocks or expose;
  `ColorField` lacks `data-channel` (RAC "should support the channel prop"); color field input lacks
  `aria-readonly`/`aria-disabled`; `Css` format differs from upstream
  (`rgb()` vs `rgba(..., 1)`/`hsla`), parser differences (5+ `rgb()` args, fractional channels): document.
- [ ] Tests still missing (gap lists 2026-10-09): `test_color_field.rs` (Enter commits/restores, `on_key_down`
  once, invalid/disabled/aria-required, controlled read-only and bound fields, typing the same value twice,
  wheel decrement, End/Home twice, native validation (required, validate fn, server errors, custom messages,
  commit on blur only if changed), channel field form reset and named hidden input); swatch/picker
  (`aria-label(ledby)`, custom classes, click selects with one `on_change`, alpha-0 "transparent",
  area-driven propagation, gray keeping its HSV hue, default black selection); native
  `Color.test.tsx` cases (exact rgb→hsl/hsb strings, short/long hex(a), clamping `rgba(300,-10,0,4)`,
  `hsla(-400,120%,-4%,-1)`, `hsb(360,..)`, the round-trip property test, missing `getColorName` names);
  area: Shift+Tab full text, "no events when disabled" via dispatched keys.
- [ ] API (left by decision): `Hue` newtype (normalizing on construction; conversions wrap since 2026-10-09);
  generic `ColorField<C: ColorValue = RGB8>` with `#RRGGBBAA` for `Alpha<_>`; typed `SliderState<T>` context
  (`use_slider_state_context::<T>()`, RAC render props); generic marks `SliderMarkValue<T>`; a per-locale
  `ColorFormatting` struct replacing the thread-local formatter/strings caches in `utils/color`
  (`channel_formatter`, `naming::strings_for`); reactive `ColorArea` x/y steps (plain `Option<f64>` now).

#### Buttons, links, disclosure, tabs, toolbar, small widgets
- [ ] Open from the review:
  - Tab press callbacks (`on_press*` on `Tab`): needs a field for an item's own press handlers in
    `UseSelectableItemInput` (collections family).
  - `Kbd`/`KeyboardKey::spoken_name` localization: upstream has no translations of key names; needs a source.
  - `TabPanel`s rendered before their `TabList` don't know `Tab::is_disabled` (documented difference; needs the
    collection built from the children).
  - Tabs `keyboard_activation` as a signal: still blocked (`CollectionOptions::select_on_focus` is fixed).
  - Upstream headers: list the upstream TEST files at the top of every module of this family (not done).
  - Missing upstream cases (survey 2026-10-09; not yet written): Button press/key callback counts, Space submits /
    non-submit types don't submit, `auto_focus` (Button, Link), `aria_labelledby`/`aria_describedby` on Button,
    use_button `aria_disabled` passthrough case; Link default class, enabled `use_link` span (`role=link`,
    `tabindex=0`), tooltip around a Link; ToggleButton default class, hover, press, `default_selected`, controlled
    `is_selected`; ToggleButtonGroup default class, no hover when disabled, controlled single/multiple;
    Disclosure/DisclosureGroup/DisclosurePanel default classes, standalone collapsed disabled, panel stays hidden
    while toggling `is_disabled`, panel `aria-hidden`; native `default_expanded`, group `default_expanded_keys`
    (single, multiple), group `is_disabled`; Tabs touch selection, `--tab-panel-width` (inline-size), TabPanel
    `data-entering`/`data-exiting` (new), Tab hover/focus callbacks; Breadcrumbs: current item doesn't fire
    `on_action`, custom `aria_label` (atom, hook), enabled span item, disabled items not pressable.
  - Leave as items: `Tabs<V>`/`DisclosureGroup<V>` generics, `ButtonElement` enum, async pending via `Action`,
    `AriaLabelling` enum.

#### Browser-test infrastructure (strength, speed, coverage)
- [ ] Page panics: element lookups, `wait_for_*` and `DndActions::wait_for_log` give up once the page panicked;
  eventual assertions written in cases still run out their timeout first (the harness then reports the panic).
  Decide whether they need an early give-up.
- [ ] Hygiene:
  - Case-level scripts left (API-first rule): `test_combobox_forms.rs` `blur_in_the_next_frame` (`eval_async`),
    `test_virtual_list.rs` (`low_level().eval`, `eval`), `test_table_selection.rs` (`eval` before a press);
    `test_submenu::safe_triangle` builds its own `ActionChain` (a `PageActions` gesture helper once more cases need
    one). Text formatted into CSS attribute selectors (`[aria-label='{name}']` in test_tabs, test_tree,
    test_table(_selection), test_menu_atoms, test_spin_button, test_tag_group_atoms, test_dnd_collection): the
    locator has only `css`/`role`/`.text`/`.has`.
  - Weak cases: `test_button::pending` (doc cites "displays a spinner", unchecked); `test_color_area::disabled` (no
    "no change events" stays check); `test_color_field::defaults` (expects no `role`, upstream `textbox`:
    undocumented); `test_calendar::range_committed_by_an_outside_press` overlaps `commit_select_on_release`;
    `test_move::doesnt_bubble_to_a_movable_parent` (no stays check); `test_menu_trigger::aria_attributes` (no
    `aria-labelledby`/`aria-controls`, upstream "should return default props"); `test_focus_scope::auto_focus` (check
    hidden in `goto`); `test_focus_ring::atom_text_input` (two behaviors); attributes only, no interaction:
    `test_focusable_atoms::disabled_and_excluded`, `test_disclosure::disabled_group`,
    `test_date_picker::twelve_hour_clocks`.
  - Typed helpers: fixture paths and ids as shared consts (today a `const PATH` per test file).
  - For the user's crates: browser-test's summary shows only steps (count WebDriver commands per test; a page-health
    give-up hook). assertr: an eventual assertion whose deadline hits while an observation is in flight fails with
    "could not be observed" and lists values only when they changed: always show the last completed observation.
  - 15 non-empty `/tmp/org.chromium.Chromium.scoped_dir.*` from 2026-10-09: find which session lets chromedriver
    create them (profiles belong in `<target>/tmp/browser-test-profiles`; browser-test's runner tests kill child
    runs on purpose).
- [ ] Coverage gaps:
  - Atoms no fixture uses: `FocusManagerProvider`, `ClearPressResponder`, `SliderMarks`/`SliderMark`.
  - Behaviors without any test: DnD auto-scroll (`use_auto_scroll`, WebKit-only), the long-press-to-select
    description (`use_highlight_selection_description`).
  - Upstream test files no `// Upstream:` header names: RAC `Tabs.browser`, `Tree.browser`, `TokenField*.browser`,
    `RangeCalendar.shadow`; react-aria `useTable.test`, `ariaTableResizing.test`, `useModal.test`,
    `utils/keyboard.test`, `useInteractOutside.shadow`; the `*.ssr` tests (Breadcrumbs, Dialog, Disclosure, GridList,
    Table, Tabs, Tree, `useModal`, `SSRProvider`: name them in the hydration test's header); react-stately
    `useTreeState.test`, `useColor.test`, `useDisclosureState.test`, `useDisclosureGroupState.test`,
    `TableUtils.test`. Name them where the behavior is tested; mirror the relevant cases.

### Families
- [ ] **Number formatting:** a formatter that knows CLDR currency/unit/percent patterns (`format_percent` appends
  `%`; German expects `50 %`), with typed currency/unit values instead of `NumberFormatOptions`' strings. Needs
  `icu_experimental` (the user decides). Then mirror the rest of `NumberParser.test.js` (localized unit names and
  plurals, SAR `ر.س.‏`, de "50 %") and the ar-AE number field browser tests.
- [ ] **Forms:** C10 `UseFormValidationInput.focus: Option<Callback<()>>` has no clear Rust-native shape yet (the date
  field focuses its first segment): decide.
- [ ] **Button, link, tabs:** link tabs (`href` on `Tab`, omitted in `atoms/tabs.rs`); Tabs `keyboard_activation` as a
  signal (blocked: `CollectionOptions::select_on_focus` is fixed); building the tab collection from the children
  (RAC; documented as a difference).
- [ ] **Calendar and date picker:** range formatting with shared fields ("June 1 – 15, 2024"; ICU4X has none yet).
- [ ] **Collections:**
  - Tree expansion and tree input (keyboard delegate, Tab navigation); focus-mode enum names; tag group options;
    per-item `on_action`; menu item links; combo box item actions/links; tab links; grid list selection checkbox (labelled by its row); `SelectionManager` cell
    selection/layout ranges; load more; DnD on the collection atoms, drag preview, tree drops; Tree atoms.
  - Autocomplete (with `get_pointer_type()`), then searchable Select/Multiselect (search keeps focus, collection
    unfiltered for the trigger text).
  - Table resizing leftovers: cursor overlay while mouse-resizing, scrollable ancestor, empty tables.
  - `ListBoxItem`/`GridListItem` rendering `<a href>` for link items (RAC).
  - `KeyboardDelegate`/`LayoutDelegate` don't require `Debug` (inputs holding them have manual impls).
- [ ] **Grid, table, DnD, virtualizer:** a column header inside a `TooltipTrigger` (no fixture: `TableHeader` renders
  the headers); RAC's TagGroup-in-cell case; virtualizer section headers are measured only once the virtualized
  ListBox renders them.
- [ ] **Interactions and focus:** the on-screen keyboard tracker (upstream `utils/keyboard.tsx`/`runAfterKeyboard`,
  5a6c32500), used by `use_prevent_scroll`, `use_viewport_size`, Popover and ModalOverlay (listed as omitted).
- [ ] **Overlays:** no test for "closed overlays observe nothing" (`atoms/popover.rs`); the landmark "toggle browser
  tabs" test was dropped (headless Chrome leaves `<body>` focused; unverified).
- [ ] Missing modules and atoms (2026-10-05 audit): `useSafeArea`, `useActionGroup`; `DropZone` and `FileTrigger`
  atoms; `Group`, `Heading`, `Header` and a generic `Text` atom.
- [ ] Deferred by the user (2026-10-07: "skip for now"): TOML in `utils::syntax_highlight` (syntect's default syntaxes
  have none): a small own `.sublime-syntax` in a separate `SyntaxSet` (no merge, which would deserialize every
  default syntax), loaded from a build-time dump (no `yaml-load` at runtime); apps highlighting in the browser pay
  for it in their wasm.

### Conventions still to apply
- [ ] Generics sweep, rest: the hooks below the typed atoms (`use_radio_group_state`, `use_select_state`, ... take
  `Key`s; worth it?), `ListBox`/`GridList`/`Table` selections (`Selection` of keys). Color channel values stay `f64`
  (decided 2026-10-07).
- [ ] Optional `Callback` props with generic arguments (`CheckboxGroup`/`ToggleButtonGroup` `on_change:
  Option<Callback<HashSet<V>>>`, `Slider` `Option<Callback<Vec<T>>>`, color atoms `Option<Callback<C>>`) can't infer
  an untyped closure in `view!` (E0282).

### Cross-cutting
- [ ] Localized strings, rest: `useDefaultLocale` (browser language + `languagechange`) and a reactive
  `I18nProvider` `locale` prop (today initial only); the autocomplete's `collectionLabel` once `use_autocomplete`
  exists (add its bundle to `scripts/port-intl-strings.py`); a non-English browser check for calendar, DnD, color and
  toast.
- [ ] Atom hygiene: a context for render state (selected/pressed) in Tab/GridRow/TableRow/GridListItem; one rule for
  `Children` vs `ChildrenFn`; atoms dropping `attr:` attributes; a `node_ref` prop on every atom rendering an element
  (react-aria-components forwards `ref` everywhere; only `Input`/`TextArea`, and `use_toolbar` creates its own
  `CapturedElement`).
- [ ] Port hooks onto `use_keyboard` shortcuts (upstream uses them in ~20 hooks; here only `use_submenu_trigger`).
- [ ] Hygiene: split the largest functions (`use_press`, 933 lines; `use_move`, `use_selectable_collection`,
  `FocusScope`); document public items (DnD states' pub fields, `SelectionOptions.disabled_behavior`,
  `UseListBoxInput`/`Return`, `CollectionOptions`, `SelectState`, `Column`, `TableColumnResizeState.table_state`,
  `TabListState.list`, `TreeState.expansion`, `UseGridCellReturn`; most atom props).
- [ ] Deviation blocks: decide one category for fixed upstream bugs and document it in `porting.md`.
  `virtualizer/list_layout.rs` records its fix under `DIFFERENT BEHAVIOR` (since 2026-10-12);
  `DismissButton`'s fix is planned as an ADDITION.

### Build performance
Numbers, methods and findings: `documentation/build-performance.md` (measure with `scripts/build-bench.sh`).
- [ ] Book profile vs advice: the book's `[profile.wasm-release]` has `lto = true`, `codegen-units = 1`, `panic =
  "abort"`, while `build-performance.md`'s advice and the book's guide say to leave LTO out of the profile and enable
  it for production builds only (`CARGO_PROFILE_WASM_RELEASE_*`): follow the advice or change it.
- [ ] `scripts/build-bench.sh` edits files with macOS `sed -i ''`: its `book-edit`/`lib-edit` scenarios fail on Linux.
  The 2026-10-08/09 measurements don't all say which machine (M4 Max or Linux) they come from.
- [ ] ICU4X calendars: `utils/date_time_formatter.rs` formats Gregorian weekday/month names with the any-calendar
  `DateTimeFormatter`; `FixedCalendarDateTimeFormatter<Gregorian, _>` would let the linker drop every other
  calendar's data and code. Measure; the date field (`hooks/datepicker/format.rs`) needs the locale's calendar.
- [ ] typed-builder `PropsBuilder::build` is instantiated per combination of props a call site sets (5.5% of the
  book's IR): measure what grouping rarely used props of the biggest atoms (`TextField`, `DatePicker`,
  `SearchField`, `Calendar`, `NumberField`) into `Default` structs saves.
- [ ] Decide (user): route splitting with `#[lazy_route]` + `--split` (main module −38%, but ~90 files per first page
  visit and an unstable Leptos feature: try a coarser grouping first).

### Testing infrastructure
- [ ] Clean up the performance tests' shared fixtures: some agents shared one fixture page between the subjects of a
  family, against "the fixture renders the subject alone" (each test still measures its own subject): drag and drop
  (`DraggableListPage` in `perf/hooks/use_draggable_item.rs`, `DroppableListPage` in `use_droppable_collection.rs`),
  grids (helpers in `perf/hooks/use_grid.rs`, `use_grid_list.rs`, `use_table.rs`, `perf/atoms/table.rs`; five
  fixtures re-export a sibling's `PagePerf`), collections (one fixture component per family, in `atoms/select.rs`,
  `combo_box.rs`, `tag_group.rs`, `hooks/use_listbox.rs`, `use_selectable_list.rs`, `use_collection.rs`,
  `use_select.rs`, `use_combobox.rs`, `use_tag_group.rs`); `pub(crate)` test helpers shared between sibling test
  modules. Decide (user): allow a family fixture (and say so in `testing.md`), or one fixture per subject.
- [ ] `Page` helper for desktop Safari emulation: `use_auto_scroll` scrolls only there, and
  `perf_tests/hooks/use_auto_scroll.rs` sends its own CDP user-agent override (`send_raw`).
- [ ] `allocated_bytes` became unbudgeted (2026-10-16): some measurements differ by 16-48 bytes between runs with
  equal counts (closing overlays, reopening a popover, collection changes). Find the cause (allocator tracing per
  call site; suspected: generated ids crossing a digit) and budget it again if it can be made deterministic.

- [ ] Performance framework, next steps (the plan of 2026-10-16; done: performance tests with budgets in the
  regular suite, what ran reported by the library, coverage):
  - Native counts: a test binary of its own with a thread-local counting allocator and `reactive_graph`'s
    `metrics` (patch the workspace too): allocations and reactive work per hook setup and per update, with counting
    inputs ("changing `is_disabled` runs the press state once"); allocations and time of rendering each atom on the
    server. Same metrics and budget files.
  - Leak checks: mount, use and unmount a fixture N times, collect garbage (`HeapProfiler.collectGarbage`), then
    live wasm bytes, DOM nodes and JS event listeners (`Memory.getDOMCounters`) must be back at their baseline
    (what `retained_bytes` per measurement can't tell).
  - Timings as A/B: `just perf-ab <git-ref>` builds the baseline in a worktree and the current tree, alternates
    their runs on one machine and reports the median difference with a confidence interval (bootstrap).
  - Explaining: `just perf-trace <scenario>`: a CDP trace (DevTools, Perfetto), a CPU profile summarized as text
    (hottest wasm functions by self time; needs a profile keeping the wasm's names section) and the scenario's steps
    as `performance.measure` marks. Forced layouts per measurement (layouts during the action's effects, before its
    frame) also need the trace.
  - The book: hydration of every page (wasm compile and instantiation, hydration time), layout shifts during
    hydration (budget 0), a few interactions on large pages.
  - Decide (user): measure at `opt-level = "z"` only (the test-app's), or also `3`.
  - Upstream `reactive_graph::metrics` (the fork's branch `leptonic-metrics`, ~30 lines plus a test) so that the
    patch can go.
- [ ] Performance suite scenarios: only `perf/virtual-list` so far. Candidates, by expected value: a `Table` of 1,000 rows (mount, sorting, selecting
  all), `ComboBox` filtering a long list per keystroke, `Select` with 320 options (opening), `ListBox` of 5,000 options
  (mount, range selection), `Calendar` mount and month changes, `DatePicker` opening, a `NumberField`/`DateField`
  keystroke, a `Tree` expanding 1,000 rows, page hydration of a large fixture; a `VirtualList` with single-line
  appends at 100 per second (the main thread's share).
- [ ] Session start failures (2026-10-10): the first wave of parallel sessions after chromedriver starts fails with
  `StartWebDriverSession` ("error sending request for url .../session", chrome-for-testing-manager 0.14): in the
  evening runs 2 and 4 of the first 8 sessions, every run (the first group's cases fail; they pass with
  `BROWSER_TEST_PARALLELISM=1`). A transport error, not a timeout: look into chromedriver's handling of concurrent
  `POST /session` and retrying session creation in browser-test (its checkout has uncommitted work: the user's).
- [ ] Fixture instances that existed only for sequential cases (audit 2026-10-09; every case now loads its page
  fresh). Merge the REDUNDANT ones, switch the listed cases to the kept instance; each family agent owns its fixtures:
  - `atoms/date_picker.rs`: decide whether `required` vs `form-required` (plain `<form>` vs `Form`) and
    `empty` vs the picker in `focus` are redundant. `atoms/date_field.rs` (optional): `bc`, `help` → `edit-date`.
  - `atoms/number_field.rs`: `#nf-validate-submit` → `#nf-validate` (one form; re-check `validate_commit_behavior`'s
    Tab focus); `#nf-currency` → `#nf-form-value` (`pasting_into_a_format`).
  - `hooks/focus_within.rs` (low value):
    `#test-fw-change-*` → the basic container with `on_focus_within_change`.
  - Test side: `table_resizing::on_resize_start_and_end_without_moving` reloads halfway. Unreferenced elements:
    listbox_selection `#replace-after`, select_behavior `#empty-after`, table_selection's 5 "Before" buttons.
- [ ] Fixtures without `Section`s render everything for every case: listbox_features, tabs, number_field,
  combobox_forms, table_navigation, combobox, select_forms (one select with 320 options), grid_list_features, popover,
  grid, table_selection, table_tree; hooks focus_manager, press, aria_hide_outside; focus_scope's `classic` section
  bundles 13 parts (and `goto` waits for its auto-focus scope in all 17 cases). Split into sections (keep together what
  a case tabs into or past) where page loads show it pays.
- [ ] Fail-first: most 2026-10-07 regression tests were written together with their fix. Spot-check the important
  ones against the old code (revert the fix in a scratch copy; `git stash` is off limits).

## Book

Page and writing rules: `documentation/documentation-strategy.md` (incl. "Verification"); look, design tokens and
which leptonic piece to use: `examples/book-ssr/STYLE_GUIDE.md`. Library gaps the book hits go into the library's
sections above and, while the book waits for them, under "Waiting on the library". Finished book work:
`documentation/history.md` ("Book").

### Next
- [ ] Resolve the dependency future-incompatibility warning in `just serve`: `book-ssr` →
  `leptos-routes 0.4.2` → `leptos-routes-macro 0.4.1` → `proc-macro-error2 2.0.1` re-exports the private
  `proc_macro` extern crate ([upstream issue #13](https://github.com/GnomedDev/proc-macro-error-2/issues/13)).
  All three are their latest published versions as of 2026-10-09; no compatible upgrade fixes it yet. Update
  the routes dependency when it removes or replaces this unmaintained dependency; do not suppress the warning
  or patch the local registry cache.
- [ ] Rules the book contradicts (`documentation-strategy.md`, found 2026-10-09; fix the pages or change the rule):
  group overviews have a "Recipes" section with a subsection per part, but `groups/status.rs` uses a top-level
  `Section title="Alert"` (only `groups/layout.rs` follows it); "every entry has a marker or a badge" doesn't hold
  for guides (`doc_layout.rs`: markers only in Concepts, badges only in Building blocks); a building block placed in
  a concept group isn't caught by the placement tests (it looks like a single-layer concept); the single-layer Tree
  Hooks page has a Keyboard section, a case the rules don't cover; Verification says `cargo clippy --features ssr
  --tests`, `just clippy` omits the book's `ssr` feature.
- [ ] The browser suite (240 tests, ~32 s) waits 15-30 s for `markdown::warm_markdown_cache`, which converts the ~210
  pages one after another at server start (search and the LLM index wait for it; the other tests finish after
  ~18 s). Convert several pages at a time (also shortens the production server's warm-up).
- [ ] Decide (user): self-host the fonts instead of Google Fonts. Every test's fresh browser context fetches the
  render-blocking stylesheet and fonts again (blocking them made the page tests 25% faster, 18.97 s → 14.28 s); the
  suite depends on the internet for them; embedding Google Fonts is a known GDPR issue in Germany.
- [ ] The Dockerfile builds from the repository root (path dependency on `../../leptonic`); not yet test-built (the
  user's call: images, a downloaded install script, a full release build).

### Waiting on the library
- TOML code blocks stay unhighlighted (TOML highlighting deferred by the user).
- Search results → Autocomplete with arrow keys through results.

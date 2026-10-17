# Leptonic: working plan

The only todo list of the repository: open work of the library (`leptonic/`, `leptonic-theme/`, `testing/`) and the
book (`examples/book-ssr/`, section "Book"). Open items only, each short: finished work goes to
`documentation/history.md`, long-term rules and knowledge into the document `documentation/README.md` names for them.
Bugs and flaky tests are fixed when found; they are listed here only while they can't be fixed now. Every bug fix
starts with a failing test.

## Waiting on the user

- **Leptos issues to file** (repros and drafts in `~/dev/leptos-issue-repros/`; file them?):
  - `a-effect-skips-run` (reactive_graph 0.2.15, also 0.9.0-beta2): an effect run is skipped when a memo recomputes
    while the effect checks a downstream memo that comes out unchanged ("Effect Read Order", `leptos-and-dom.md`).
  - `b-spread-attrs-lost` (passes on 0.9.0-beta2: ask for a 0.8 fix): attributes spread onto a component are lost
    when its root element is replaced in `erase_components` builds (`lessons.md`).
  - `c-spread-listener-leak` (tachys 0.2.19; passes on 0.9.0-beta2): `AnyViewWithAttrs::rebuild` keeps the old spread
    attributes' handlers (disposed owner: they panic) and drops the new ones. Known-issue case
    `component_spread_rebuild_known_issues`; atoms avoid such spreads meanwhile.
  - `d-keyed-for-moves` (also 0.9.0-beta2): the keyed `For` diff moves kept rows whose relative order didn't change,
    collapsing text selections (why `render_visible_items` mounts rows itself).
- **The user's crates:** leptos-styles: `add_reactive_unchecked` (today `add_optional_unchecked(prop, move ||
  Some(..))`); leptos-css lacks `Margin::all`-style helpers. leptos-element-capture: identity `PartialEq`/`Eq`/`Hash`
  for `CapturedElement`, `try_get_untracked` for deferred callbacks. leptos-classes: `classes="a b"` compiles but
  panics at runtime (`From<&'static str>` for `ClassName`): split whitespace-separated names, or reject them at
  compile time? browser-test: count WebDriver commands per test in the summary, a page-health give-up hook. assertr:
  an eventual assertion whose deadline hits mid-observation fails with "could not be observed" and lists values only
  when they changed: always show the last completed observation.
- **Decisions:**
  - `icu_experimental` as a dependency, for a CLDR-aware number formatter ("Number formatting" under Families).
  - The shape of `UseFormValidationInput.focus` (today `Callback<()>`, C10).
  - Untrack the no longer generated `testing/test-app/style/leptonic` (106 tracked files; `git rm -r --cached`).
  - Remove `examples/book-ssr/style/leptonic/` (55 tracked files: an old copy of leptonic's styles incl. the removed
    components and themes, referenced nowhere)?
  - Route splitting with `#[lazy_route]` + `--split` ("Build performance").
  - `live_announcer`: `Assertiveness::default()` is `Polite`, react-aria's `announce` defaults to assertive (recorded
    as a deviation): keep or change?

## Library roadmap

Order: open bugs first, then families (each a complete package: API on the conventions, missing react-aria
features, re-sync to the current upstream commit, deviation block, fixture + browser test from upstream's tests,
atoms, book heads-up), then cross-cutting work.

### Bugs

- [ ] Flaky perf counts (full suite, 2026-10-17; fixed: `virtual_list::resize_grown`, the DnD TalkBack click):
  `perf_tests::atoms::virtualizer::keyboard_navigation_large` ±1 allocation between its `home`/`end` steps (seen
  twice); `perf_tests::hooks::use_exit_animation::mount_and_unmount` 1 style recalc more (timing of the animation);
  `test_dnd_collection::screen_reader_basic_drag_and_drop` failed once. Reproduce with CPU throttling (`lessons.md`)
  and fix the cause.
- [ ] Flaky under load: `overlay_position` cases (`placed_above`, `reopened_with_arrow`: no `[data-placement=...]`
  within 10 s), pass alone; `placed_above` waits in steps (`aria-expanded`, presence, placement) so the next failure
  shows which. Not seen in the last two full runs.
- [ ] Found by the performance tests:
  - `Part` sweep leftovers (every public hook hands out `Part`s since 2026-10-17, `history.md`):
    `create_focus_manager` (not a `use_` hook) still returns plain `props`; `UseMenuTriggerReturn::menu_props`
    (`UseMenuTriggerMenuProps`) is `use_menu`'s input bundle, not element props: make it a `UseMenuInput`-like input
    named `menu` (as `use_select`'s `listbox: UseListBoxInput`); `use_hidden_select`'s `input` part is a template for
    one input per value (a part is one element): hand out the template's fields instead.
  - A cheaper `Part` for composition (the user decides; `performance.md`, "Part costs"): since hooks and atoms
    compose the public hooks, every inner hook's part costs its `Arc` only to be consumed by `into_props()` right
    away (Button mount 289 → 300, 8 of the 11 such). A `Part` holding its stored props inline until it is shared
    (cloned, mapped or rendered; interior `OnceLock`) would make `into_props()` free, with the same API.
  - Atom reference props `MaybeSignal<Option<Id>>`, texts `MaybeSignal<Option<Oco>>` (closures:
    `FromReactiveClosure` prototype, the user decides); until then atoms convert `MaybeProp<String>` with
    `id_ref_prop` (public).
  - Dangling ARIA references left: an unrendered `ToastTitle` (upstream too); `MenuTrigger`/`DialogTrigger`
    `aria-controls` (needs the overlay element in `use_overlay_trigger`). `Menu` has no `id` prop (its id comes from
    the trigger context).
- [ ] More writers of `style` (public API): `use_disclosure` sets `--disclosure-panel-*` with `set_property` (lost
  when the user's `styles` are reactive; the tabs atom passes a `variables` signal), `text_selection` on the press
  target (iOS), `use_safely_mouse_to_submenu` sets `pointer-events` on the menu element; `UseVisuallyHiddenProps` writes the whole `style` attribute as a custom attribute (return
  `PropsWithStyles`).
- [ ] Possibly obsolete microtask deferrals: `use_selectable_collection`'s `on_focusin` and `use_toolbar`'s
  `on_focus_capture` defer focus because "Leptos handlers can't be re-entered", but both attach through the re-entrant
  `EventHandler::into_on`. Try focusing synchronously; drop the comments and the toolbar's deviation entry.

### Performance (findings of the performance tests, 2026-10-16)

Fix the causes, then lower the budgets (`BROWSER_TEST_PERF_BUDGETS=update`). Priorities: wasm size, allocation count,
allocated bytes (`documentation/conventions.md`).
- [ ] Per-item work left after `KeyedTriggers` (applied 2026-10-17 to CheckboxGroup, ToggleButtonGroup, Tabs,
  DisclosureGroup, tree/table expansion, the selection manager (one implementation), collection changes (node
  diffs), drag and drop, calendar cells; results in `history.md`):
  - RadioGroup/CheckboxGroup: a user's selection still costs ~6-13 allocations per item with no reactive work
    (probably every item's native validity read per commit: the readers of `register_native_validity_reader`, from
    `use_form_validation.rs`); disabling a group of 1,000 reruns 4-6 effects per item (R6: the items' `Effect`s on
    the disabled flag).
  - Readers that subscribe per read (`KeyedTriggers::track`: the selection manager's queries, group `is_selected`)
    pay a map update and an `on_cleanup` on each rerun: "check all" in a CheckboxGroup of 1,000 12k → 14k. Items
    could subscribe once (`KeyedTriggers::subscribe`) and read untracked (`use_selectable_item`, group items).
  - A set-wide change notifies key by key (`KeyedTriggers::following`): dragging a selection of 1,000 rows start
    12k → 17k, end 2k → 7k (inherent per-key notifications; a threshold to `notify_all` is cheaper for large diffs).
  - Collections: positions (`aria-posinset`/`setsize`) stay O(n) per change; a keyboard drag start over 1,000
    droppable rows 108k (the session-wide `is_valid_drop_target` per row); paging a 12-month calendar 56-60k (cells
    re-render with their dates). `register_drop_item`/`register_drop_target` `retain`-scan every registration on
    each registration: O(n²) mounting (read in the code, not measured).
- [ ] Rules not yet applied everywhere (the 2026-10-17 sweep applied R2/R11 `signal_attr`/`fixed_attr` to every
  hook's attributes, R4 `data_flags` to every atom, R14 named fields where a literal overwrote a `Signal` default):
  `Spread` on the remaining hook attribute tuples (only the Button/Link/field/toggle/radio/overlay ones have it);
  R6 `when_true` for the remaining effects reacting to a flag (`use_slider_thumb`, `use_calendar`'s button focus,
  `TabList`); R7 merged stores in `Tooltip` (9 `StoredValue`s) and `use_form_validation_state` (7); R8 wrapper
  `Callback`s (300 sites; e.g. `use_toggle`'s four label press callbacks, `use_radio`'s `on_focus`); R5 memos that
  feed one reader (`use_calendar_cell`'s 12 per cell). Decision 5 (leptonic's `MaybeSignal` for usually fixed hook
  inputs) is applied to `UseSeparatorInput` only: hook inputs that the atoms fill from their `Signal` props save
  nothing until the atom props change too (`FromReactiveClosure`), and the hook-to-hook inputs (`UsePressInput`,
  `UseHoverInput`, ...) would need the interaction stack to take `MaybeSignal` throughout (`when_true`, `either`).
- [ ] Mount costs per item (allocations): CheckboxField ~1,140 (~62 KB), RadioButton ~800, Button-based atoms 450-610
  (every Button runs `use_context_menu`, `use_keyboard`, `use_focusable`, `use_focus_ring`, `use_hover`, `use_press`;
  `use_press` alone 144), ToggleButtonGroup ~600 per button, Tag ~1,370 (twice a ListBox option, ~720), Table row
  ~4,500 (Grid row with 3 cells ~2,100, GridList item ~800), calendar cell ~950 (25 render effects; a 12-month
  Calendar ~400k), color swatch ~1,500, menu item ~890, disclosure in a group ~1,060, toast ~800, tab ~730,
  NumberField 2,840 (TextField 1,142). Look for what each item allocates that could be shared or lazy.
- [ ] Specific costs: toggling the Table's selection column re-renders every cell (4.1M allocations, 85k render
  effects at 1,000 rows: cell keys and indexes shift); Select and ComboBox popovers render every option (opening with
  1,000: 720-738k; not virtualized); type-ahead without a match ~18 allocations per item (collation); a rejected
  character in a NumberField 440 allocations and 105 reallocations (`beforeinput` → `is_valid_partial_number`); every
  keystroke copies a text value about five times (50 KB for 10,000 characters); opening a DatePicker ~42k (mounting
  the calendar); disabling a Link rebuilds it (`<a>`/`<span>`, the router's `<A>`); the first key press on a page ~40
  allocations once (probably platform detection, `utils/platform.rs`); opening a 10-item menu with Enter 10,669
  allocations and 5 layouts; SliderMarks re-render every mark on a range or step change (keyed by value); changing a
  collection re-lays out a virtualized list (appending 10 to 10,000: ~40k).
- [ ] `VirtualList` (budgets `leptonic/tests/budgets/atoms/virtual_list.json`): mounting 20,000 lines 67,418
  allocations (27.4 MB allocated, 15.6 MB held: ~780 bytes per line though ~20 rows render), 8 layouts and style
  recalculations over 4 frames; appending 50 lines while showing the start ~21,830 allocations (8.9 MB, 1 layout),
  following the end ~24,170 (9.1 MB, 2 layouts over 3 frames); changing a grown log's width ~4,900 (1.9 MB), 3
  layouts. Look for forced layouts (more layouts than frames) and the per-line allocations (collection nodes, keys,
  maps). Timings (`just browser-perf`, 2026-10-12, headless Chrome on the user's Mac): 50 lines appended to ~21,000
  take ~3.0 ms until settled following the end, 3.4–3.9 ms at the start (why the start costs more is open), plus
  0.1–0.3 ms of the next frame; 100 appends per second would take ~30–40% of the main thread, so apps appending that
  often should batch appends per animation frame. Route to O(k log n) appends: "Virtualizer" below.

### The atom theme
- [ ] Every family with atoms is ported (`leptonic-theme/scss/atoms/`), none checked in a browser yet:
  - A showcase to check it visually (light and dark, every family), e.g. an `examples/atom-theme` app with the
    starter's markup per family; then fix what's off.
  - Atom gaps the stylesheets need: `Table` root `data-focus-visible`, `TableBody` `data-empty`, column resizer
    `data-focus-visible`/`data-focused`/`data-hovered`; custom column header content (a flex wrapper and a sort
    indicator; the resizer rules assume it); a `SelectionIndicator` atom (sliding tab indicator, OMITTED in
    `atoms/tabs.rs`: needs a shared-element transition, snapshot-before-unmount ordering in Leptos unclear);
    `DateInput` with `part` rendering `slot="start"`/`"end"`; `CalendarCellButton` children as a function of the cell
    state (range start/end, hovered, pressed, the formatted day); a `ToastQueue` `wrap_update` hook (view
    transitions); the table's selection checkbox (a raw `<input>` in `TableCell`) and `GridListItem`'s as `Checkbox`
    atoms; links (`href`) in list box/menu/grid list/tag items.
  - Decide: `ColorPicker` renders no element (the theme's `.color-picker` trigger is the app's class, and a
    `leptonic-Button` trigger brings button looks it doesn't undo); calendar buttons and the toast close button are
    quiet only with `attr:data-variant="quiet"` (maybe quiet by default); the `Disclosure` trigger is a `Button` (the
    theme undoes button looks).
  - Not ported (no atoms yet): `CommandPalette`, `DropZone`, `NavigationTree`, `SegmentedControl`, `Sheet`,
    `TokenField`, `Tree`.

### Quality pass 2026-10-12: open findings
- [ ] Public API inconsistencies:
  - Missing props other field atoms or RAC have: `ComboBox` `id`/`aria_describedby`; `ColorChannelField`
    `auto_focus`/`is_wheel_disabled`/`on_focus_change` (hard-coded); `ColorSwatch` `id`; `DatePicker`/
    `DateRangePicker` roots lack `data-focus-within`/`data-focus-visible` (only `DatePickerGroup` has them).
  - Default classes undecided for hidden helpers: `DismissButton` (own `<button>`, no `classes`/`styles`),
    `HiddenSelect` (own `<div>`); table column headers and resizers have a hard-coded class and no `classes`.
  - `MenuSection` renders its own `<header>` (`heading_classes`), ListBox has a `ListBoxSectionHeading` part: a
    `MenuSectionHeading` part (single-element rule).
  - A second `CalendarCellButton` renders nothing (see "Bugs"); `dev_warn!` wording varies ("must be inside",
    "belongs in", "X: not inside a Y"): normalize.
  - C9: `UseClipboardReturn::clipboard_props` (the hook's own element: `props`). C11: plain `bool`s
    `UseTextFieldInput::exclude_from_tab_order`, `is_wheel_disabled` (number, color, color channel field inputs).
  - `Debug`/`Clone` missing on `*Input`s: text, number, search, color and color channel field inputs,
    `NumberFieldState`; `UseToastInput`/`UseToastRegionInput` need `Debug` on `ToastQueue`; `ListFormatter` has none.
    `KeyboardDelegate`/`LayoutDelegate` don't require `Debug` (inputs holding them have manual impls).
    `CheckboxGroupContext` (and its `data` field) is `pub`, looks internal.
  - `radio.rs`/`switch.rs`/`checkbox.rs`: one-caller `use_*_atom(XSetup { .. })` helpers: inline all three.
- [ ] Deviation blocks: entries without a reason (couldn't be inferred): `use_grid` `getRowText`, `list_layout` AI
  layout bottom alignment, `use_table_row`/`use_table_state` synthetic link props, `expandedKeys: 'all'`,
  `UNSTABLE_useFilteredTableState`, `use_droppable_collection` "drop handlers run synchronously", several in
  `dnd/types.rs`, `use_breadcrumb_item` `autoFocus` (portable: `UseLinkInput` has `auto_focus`); atoms `separator`,
  `toolbar` (render props), `slider` (values a `Vec`), `menu` (`MenuSection` keeping disabled keys), `popover`
  (`isEntering`/`isExiting`, `shouldSkipAnimation`, `PreviewTrigger`), `table`. State hooks repeating C3/C4 (record
  once, `porting.md`): trimmed in calendar/datepicker; most others carry hook-specific detail next to it: decide a
  uniform trim. Decide one category for fixed upstream bugs and document it in `porting.md`
  (`virtualizer/list_layout.rs` uses `DIFFERENT BEHAVIOR`; `DismissButton`'s fix is planned as an ADDITION).
- [ ] Upstream tracking: the intl bundles' header (`// Generated by ... from react-aria/intl/<x>/*.json @ <sha>`)
  isn't read by `upstream-drift.sh`; have `scripts/port-intl-strings.py` emit `// Upstream: react-aria/intl/<family>
  @ <sha>` too (regenerating at react-spectrum's current HEAD changes translations: review).
- [ ] Module docs: 141 of the 390 source files have a `//!` doc. Decide whether every module gets one.
- [ ] `use_press.rs`: the middle (about lines 700-1700) was only scanned (raw targets, unwraps, derives), not read.

### Rules the code contradicts (fix the code, or change the rule)
- [ ] Book: "every hook has a page": ~166 hook files, 74 hook pages (pages cover several hooks; unverified which
  hooks have none).
- [ ] `testing.md`: "`with_subject_name` only for runtime values, in helpers": helpers use fixed names ("inner text",
  "focus"), and `test_tabs`, `test_label_slots`, `test_forms`, `test_slider`, `test_virtual_list` call it directly.
  "Fixture-specific actions only for a fixture with an id convention": `ClipboardActions` (CDP clipboard access) and
  `VirtualListActions` (selection, scrolling) aren't.
- [ ] `Signal::derive` where a `Memo` is needed ("Derived Values", `leptos-and-dom.md`), each blocked by "Effect
  Read Order" (convert together with reading the collection or value first in the readers, plus a browser case like
  `value_and_items_derived_from_one_signal`): the bound selection of `use_select_state` and `use_combobox_state` (the
  hidden select's options memo and the combo box reconciliation read it before the collection); the calendar states'
  `is_value_invalid` (every cell's `is_invalid` memo reads it before the value) and `visible_range`
  (`weeks_in_month`). `use_grid_list_item`'s `on_action` stays a derive (`Callback` has no `PartialEq`).
- [ ] `utils/intl_strings`: one `Memo` per `use_localized_strings` call in per-item hooks (`use_table_row`,
  `use_tag`, `use_tree_item`, DnD items, date segments; calendar cells share theirs through `CalendarData`), and
  `scripts/port-intl-strings.py` could emit pre-parsed messages (or one fn per message, as upstream's string
  compiler) so nothing is parsed at runtime.
- [ ] `ThemeProvider` (`atoms/theme.rs`): replace `signal_ls` with leptos-use's SSR-aware, cross-tab
  `use_local_storage`, then drop `Theme`'s serde bounds (`Display`/`FromStr` on `LeptonicTheme`). Update the book's
  themes/installation pages.
- [ ] Better than react-aria, ranked by value/effort:
  1. `Validator<T>` with `From<F>` for closures returning `Result<(), E: Into<ValidationErrors>>` (today
     `validate=Arc::new(|v: &String| ..)`, `ValidateFn<T>`); pairs with the `Validation` enum (Forms).
  2. Typed item collections: `<ListBox items=fruits key=|f| f.id text=|f| f.name.clone()>{|f: &Fruit| ..}`, the
     collection and the views from one source (`ListCollection<T>`, `ListBoxItems<T>`, `on_action: Callback<T>`);
     only the atom wrapper generic, hooks stay `Key`-based (`design-collections.md`); with C16 `S: SelectedValues`
     replacing `selection_mode` + `disallow_empty_selection`; a `SelectValue` render function (RAC "supports custom
     select value").
  3. `Tabs<V: SelectionValue>` (an enum of pages; exhaustive panel `match`), `DisclosureGroup<V>`, typed `on_action`
     on Breadcrumbs/GridList/Menu (`typed_values.rs` exists).
  4. Pre-compiled localized strings (above).
  5. `#[slot]` for fixed two-part structures (`TooltipTrigger`, `DialogTrigger`, `SubmenuTrigger` take the overlay as
     a required slot: a missing/duplicate overlay fails to compile instead of a runtime `dev_warn!`).
  6. Ids known at render time (a `PressResponderTrigger` fallback id, `Button` `id` defaulting to a generated id):
     trigger/panel relations in the SSR HTML instead of `ensure_element_id` effects.
  7. Typed `*StateContext`s (`DisclosureState`, `DatePickerState<V>`, `SliderState<T>`, tab/toggle render state) as
     the typed replacement for RAC's render props.

#### Focus
- [ ] FocusScope in another document (FocusScopeOwnerDocument.test.js): listeners use `use_document()`, not the
  scope's owner document.
- [ ] RAC `ShadowDOMFocus.browser.test` (ComboBox/NumberField/Menu with `prevent_focus_on_press` in a shadow root):
  not ported (needs shadow-root lookups in `tests/pages`).
- [ ] A public focusable-elements `Iterator` (upstream exports `getFocusableTreeWalker`).

#### Overlays
- [ ] `ToastQueue<T: Clone + Send + Sync>` (`use_toast_state.rs`) can't hold non-`Send` content: make it
  storage-generic (`LocalStorage`).
- [ ] Tests missing:
  - useOverlayPosition.test.tsx: "should close the overlay when the trigger scrolls" for a popover (the tooltip covers
    the path) and the "positioned container" cases (need a hook fixture: the atoms portal to `body`).
  - Popover.test.js "should not leak PopoverContext to nested popovers", "supports overriding styles", "should support
    custom Pressable trigger"; Tooltip.test.js "shows on hover when button has isPending", "supports overriding
    styles", "should support custom Focusable trigger on hover", the two tab-order cases.
  - Dialog.test.js onEnter/onExit variants (entering until an `on_enter` animation ends, both exits awaited,
    standalone modal), "should not close Modal when DateRangePicker is dismissed by outside click"; Modal.browser
    "does not scroll the modal into view when the backdrop is pressed".
  - useOverlay/useModalOverlay touch variants; useOverlay.shadow (outside presses in a shadow root).
  - useTooltipTriggerState.test.js (close delay longer than default / zero, warm-up called twice, controlled
    tooltips); useToastState "three toasts, remove the middle via timeout" (browser); useLandmark: one landmark,
    nested last, added as parent, Alt+F6 without main / main only, mouse focus, focus after blur, controller
    listeners, components handling focus; runAfterTransition.test.ts; useViewportSize/useLandmark/Dialog `.ssr`.
- [ ] No test for "closed overlays observe nothing" (`atoms/popover.rs`); the landmark "toggle browser tabs" test was
  dropped (headless Chrome leaves `<body>` focused; unverified).

#### Forms and fields
- [ ] API:
  - `ValueBinding::from_state_props` returns a tuple, and every state input keeps a `default_x` "ignored when bound":
    `enum StateSource<T> { Owned(T), Bound(ValueBinding<T>) }` + a named `StateProps<T> { source, on_change }`.
  - `ValidationResult` can be `is_invalid: false` with errors: `enum Validation { Valid, Invalid { errors, details:
    ValidityFlags } }` (bitflags for the 11-bool `ValidityStateSnapshot`). Typed validators `impl Fn(&T) -> Result<(),
    E>` with `E: Into<FieldErrors>` (today `Result<(), Vec<String>>`), rendered by `FieldError`; server errors as a
    `ServerErrors` newtype (optionally keyed by a field-name enum), which also answers "several names per validation
    state".
- [ ] Tests:
  - A custom `validate` with `ValidationBehavior::Native` is never browser-tested (fixtures with `validate` use
    `Aria`), nor "invalid event defaultPrevented → no autofocus" (`use_form_validation.rs`; RS TextField.test.js:599,
    788).
  - NumberField validation (RS NumberField.test.js:3142-3462: steppers commit, validate, server via submit, custom
    message, commit on blur only if changed, aria, form reset): the atom fixture has no `validate`, no `Aria` field, no
    reset button. `number_field_atoms::validate_commit_behavior` lacks the announcement checks and upstream's final
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
- [ ] Tests missing: ListBox focus ring/press states (`data-focus-visible`, `data-pressed`, not on non-interactive
  options), Meta+Arrow and Ctrl+Shift+Home/End, Escape in a modal clears first, missing-label warning (feature),
  `aria_labelledby` prop; Select: opening on touch up, clicking outside, `activeElement.blur()` keeps it open, extra
  children reading `SelectContext`, customized native messages, server HTML of the default value, links in a Select,
  in-app router links.

#### ComboBox, Menu, TagGroup
- [ ] API: `Menu` takes `collection: Option` + `state: Option` and warns at runtime (one required `MenuItemsSource`
  enum). Better than react-aria: typed menu actions `Menu<A: SelectionValue>` with `on_action: Callback<A>`;
  `SubmenuTrigger` rendering its own trigger item (no key-match `dev_warn`); a typed combo box entry `enum
  ComboBoxEntry<V> { Selected(V), Custom(String), Empty }`; `MenuSection<S: SelectedValues>`; generic `TagGroup<V>`
  with `on_remove: Callback<Vec<V>>` in collection order.
- [ ] Tests: no hooks-level combo box fixture (`use_combobox` + `use_combobox_state` without the atoms).
  `ComboBoxValue` has no `valueProps` id (react-aria's `useValueId` describing the input), see `use_combobox.rs`.

#### Grid, Table, GridList, Tree
- [ ] A column header inside a `TooltipTrigger`: `TableHeader` renders the headers itself; needs custom column header
  content (a `TableColumn` render function) and a tooltip outside the `<tr>`: design first. RAC's TagGroup-in-cell
  case.
- [ ] Low: `KeyboardNavigationBehavior` lives in `hooks::gridlist` but grid and table use it.
- [ ] Tests missing: GridList dynamic collections / moving items between sections (GridList.test.js:526, 687), links
  in trees (Tree.test.tsx:1549), Tree `focusMode`/`allowsArrowNavigation` cases (2944-3100), Table "does not hang
  restoring focus" (1167); resizing `max_width`, `ColumnBound::Percent`, changing container width, smaller after
  `minWidth` shrinks (tableResizingTests.tsx:149-169, 292-340, 511, 598-660).
- [ ] API (better than react-aria): typed tables `Table<T, C: ColumnId>` (columns an enum, rows `Signal<Vec<T>>`, cells
  `Fn(&T, C) -> AnyView`); `SortDescriptor<C>` + a `sort_rows` helper with the locale `Collator`; persistable column
  widths (`column_widths` + `set_column_widths: Out<..>` on `ResizableTableContainer`); data-driven
  `GridList<T>`/`Tree<T>`; `Table` accepting any signal/closure and memoizing inside.

#### Drag and drop, clipboard
- [ ] API: typed drag payloads (`DragFormat` trait: `const MIME`, `type Data: Serialize + DeserializeOwned`;
  `DragItem::typed::<F>(&data)`, `TextDropItem::get::<F>()`); a public `DropOperations` bitflags type with `preferred`
  and `DropResult::{Dropped(op), Canceled}` replacing `allowed_drop_operations: Option<Signal<Vec<DropOperation>>>`
  and its order-dependent defaults (a `pub(crate) DropOperations(u8)` exists in `hooks/dnd/types.rs`); the drag
  preview as a `ViewFn` rendered off-screen for `setDragImage`; `use_copy_to_clipboard() -> (Callback<String>,
  Signal<CopyState>)` with a self-resetting "copied" state.
- [ ] Tests: touch descriptions ("Double tap to ...", "Long press to ...") are checked natively only (`messages.rs`):
  a browser case needs a coarse primary pointer, i.e. CDP touch emulation that session resets undo. DnD auto-scroll
  (`use_auto_scroll`, WebKit-only) stays untested; a `Page` helper for desktop Safari emulation
  (`perf_tests/hooks/use_auto_scroll.rs` sends its own CDP user-agent override).

#### Virtualizer
- [ ] Rest: virtualized `GridList`, `Table`, `ComboBox`/`Select` popovers and menus (RAC's virtualized
  `GridList`/`Table`/`ComboBox`/`VirtualizedMenu` tests), sections/headers in virtualized lists (headers are measured
  only once the virtualized ListBox renders them), `GridLayout`/`TableLayout`/`WaterfallLayout`, drop targets.
- [ ] Long term: a flat-list layout for `VirtualList` (no `Collection`: `Vec<Key>` + key → index + a Fenwick tree of
  row lengths; append k rows O(k log n), visible range O(log n + v)), with an append-aware source
  (`VirtualListItems<T>` with `push`/`extend`/`drain_front` and a change generation) instead of
  `items: Signal<Vec<T>>`. Today an append costs an O(n) comparison of the top-level nodes (~0.6 ms at 20,000 rows,
  end-anchored) plus `Collection::build` (~2 ms); trimming the start moves every row (~5 ms).
- [ ] `layout_info.rs` `transform: Option<Arc<str>>` stringly CSS.
- [ ] Sectioned lists: continuation of partial builds is disabled for them (sections may be laid out partially
  anywhere).

#### Calendar and date pickers
- [ ] Leading zeros: `format.rs` pads months/days/hours as ICU4X's short pattern does (de-DE "05.06.2024"); `Intl`'s
  numeric fields follow CLDR's `yMd` skeleton, padded per locale and field (de-DE "5.6.2024", bg "5.06.2024", en-GB
  "05/06/2024"). ICU4X ships no `availableFormats`. Options: a per-locale padding table generated from `Intl` (Node's
  ICU equals Chrome's), or wait for ICU4X skeleton data (`format.rs`'s deviation block).
- [ ] API:
  - Calendar systems through ICU4X `AnyCalendar` (Buddhist, Japanese, Persian, Hijri, ...; `format.rs`
    `preferences()` forces Gregorian today): keep jiff values, convert for display and editing; `IncompleteDate`
    holds era, month code and era year (upstream's `toCalendar` model). Decide with "ICU4X calendars" (Build
    performance).
  - Granularity tied to the value type: `trait DateValue { type Granularity }` (`Date`: zero-sized `DayOnly`;
    `DateTime`/`Zoned`: `Hour | Minute | Second`): `Granularity::Hour` on a `Date` (silently Day today) stops
    compiling, `resolve_granularity` goes.
  - Typed segment edits: `enum SegmentValue { Era(Era), DayPeriod(DayPeriod), Number(i32) }` instead of
    `DateFieldState::set_segment(kind, i32)` with era-as-index and day-period 0/1.
  - One range type: `utils::date::DateRange` and `datepicker::RangeValue<V>` duplicate each other; a generic
    `Calendar<V: DateValue = Date>`/`RangeCalendar<V>` keeping times (upstream `normalizeValue`) drops the picker glue
    (`CalendarPickerContext`/`RangeCalendarPickerContext`).
- [ ] Range formatting with shared fields ("June 1 – 15, 2024"; ICU4X has none yet).

#### Color and slider
- [ ] Fidelity: `use_color_field.rs` no `name`/`form` on the visible input (the atom's hidden `#RRGGBB` input
  submits): document in hook and atom deviation blocks or expose; `ColorField` lacks `data-channel` (RAC "should
  support the channel prop"); the color field input lacks `aria-readonly`/`aria-disabled`; the `Css` format differs
  from upstream (`rgb()` vs `rgba(..., 1)`/`hsla`), parser differences (5+ `rgb()` args, fractional channels):
  document.
- [ ] Tests missing: `test_color_field.rs` (Enter commits/restores, `on_key_down` once, invalid/disabled/aria-required,
  controlled read-only and bound fields, typing the same value twice, wheel decrement, End/Home twice, native
  validation (required, validate fn, server errors, custom messages, commit on blur only if changed), channel field
  form reset and named hidden input); swatch/picker (`aria-label(ledby)`, custom classes, click selects with one
  `on_change`, alpha-0 "transparent", area-driven propagation, gray keeping its HSV hue, default black selection);
  native `Color.test.tsx` cases (exact rgb→hsl/hsb strings, short/long hex(a), clamping `rgba(300,-10,0,4)`,
  `hsla(-400,120%,-4%,-1)`, `hsb(360,..)`, the round-trip property test, missing `getColorName` names); area:
  Shift+Tab full text, "no events when disabled" via dispatched keys.
- [ ] API (left by decision): `Hue` newtype (normalizing on construction); generic `ColorField<C: ColorValue = RGB8>`
  with `#RRGGBBAA` for `Alpha<_>`; typed `SliderState<T>` context (`use_slider_state_context::<T>()`, RAC render
  props); generic marks `SliderMarkValue<T>`; a per-locale `ColorFormatting` struct replacing the thread-local
  formatter/strings caches in `utils/color`; reactive `ColorArea` x/y steps (plain `Option<f64>` now).

#### Buttons, links, disclosure, tabs, toolbar, small widgets
- [ ] Tab press callbacks (`on_press*` on `Tab`): needs a field for an item's own press handlers in
  `UseSelectableItemInput`.
- [ ] Link tabs (`href` on `Tab`, omitted in `atoms/tabs.rs`); building the tab collection from the children (RAC;
  documented as a difference; also lets `TabPanel`s rendered before their `TabList` know `Tab::is_disabled`); Tabs
  `keyboard_activation` as a signal (blocked: `CollectionOptions::select_on_focus` is fixed).
- [ ] `Kbd`/`KeyboardKey::spoken_name` localization: upstream has no translations of key names; needs a source.
- [ ] Upstream headers: list the upstream test files at the top of every module of this family.
- [ ] Missing upstream cases: Button press/key callback counts, Space submits / non-submit types don't submit,
  `auto_focus` (Button, Link), `aria_labelledby`/`aria_describedby` on Button, use_button `aria_disabled`
  passthrough; Link default class, enabled `use_link` span (`role=link`, `tabindex=0`), tooltip around a Link;
  ToggleButton default class, hover, press, `default_selected`, controlled `is_selected`; ToggleButtonGroup default
  class, no hover when disabled, controlled single/multiple; Disclosure/DisclosureGroup/DisclosurePanel default
  classes, standalone collapsed disabled, panel stays hidden while toggling `is_disabled`, panel `aria-hidden`; native
  `default_expanded`, group `default_expanded_keys` (single, multiple), group `is_disabled`; Tabs touch selection,
  `--tab-panel-width` (inline-size), TabPanel `data-entering`/`data-exiting`, Tab hover/focus callbacks; Breadcrumbs:
  current item doesn't fire `on_action`, custom `aria_label` (atom, hook), enabled span item, disabled items not
  pressable.
- [ ] Leave as items: `Tabs<V>`/`DisclosureGroup<V>` generics, `ButtonElement` enum, async pending via `Action`,
  `AriaLabelling` enum.

#### Browser-test infrastructure
- [ ] Page panics: element lookups, `wait_for_*` and `DndActions::wait_for_log` give up once the page panicked;
  eventual assertions written in cases still run out their timeout first (the harness then reports the panic).
  Decide whether they need an early give-up.
- [ ] Hygiene:
  - Case-level scripts left (API-first rule): `test_combobox_forms.rs` `blur_in_the_next_frame` (`eval_async`),
    `test_virtual_list.rs` (`low_level().eval`, `eval`), `test_table_selection.rs` (`eval` before a press);
    `test_submenu::safe_triangle` builds its own `ActionChain` (a `PageActions` gesture helper once more cases need
    one). Text formatted into CSS attribute selectors (`[aria-label='{name}']` in test_tabs, test_tree,
    test_table(_selection), test_menu_atoms, test_spin_button, test_tag_group_atoms, test_dnd_collection): the locator
    has only `css`/`role`/`.text`/`.has`.
  - Weak cases: `test_button::pending` (doc cites "displays a spinner", unchecked); `test_color_area::disabled` (no "no
    change events" stays check); `test_color_field::defaults` (expects no `role`, upstream `textbox`: undocumented);
    `test_calendar::range_committed_by_an_outside_press` overlaps `commit_select_on_release`;
    `test_move::doesnt_bubble_to_a_movable_parent` (no stays check); `test_menu_trigger::aria_attributes` (no
    `aria-labelledby`/`aria-controls`, upstream "should return default props"); `test_focus_scope::auto_focus` (check
    hidden in `goto`); `test_focus_ring::atom_text_input` (two behaviors); attributes only, no interaction:
    `test_focusable_atoms::disabled_and_excluded`, `test_disclosure::disabled_group`,
    `test_date_picker::twelve_hour_clocks`.
  - Typed helpers: fixture paths and ids as shared consts (today a `const PATH` per test file).
  - 15 non-empty `/tmp/org.chromium.Chromium.scoped_dir.*` from 2026-10-09: find which session lets chromedriver
    create them (profiles belong in `<target>/tmp/browser-test-profiles`).
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
- [ ] **Collections:**
  - Tree expansion and tree input (keyboard delegate, Tab navigation); focus-mode enum names; tag group options;
    per-item `on_action`; menu item links; combo box item actions/links; grid list selection checkbox (labelled by
    its row); `SelectionManager` cell selection/layout ranges; load more; DnD on the collection atoms, drag preview,
    tree drops; Tree atoms.
  - Autocomplete (with `get_pointer_type()`), then searchable Select/Multiselect (search keeps focus, collection
    unfiltered for the trigger text).
  - Table resizing leftovers: cursor overlay while mouse-resizing, scrollable ancestor, empty tables.
  - `ListBoxItem`/`GridListItem` rendering `<a href>` for link items (RAC).
- [ ] **Interactions and focus:** the on-screen keyboard tracker (upstream `utils/keyboard.tsx`/`runAfterKeyboard`,
  5a6c32500), used by `use_prevent_scroll`, `use_viewport_size`, Popover and ModalOverlay (listed as omitted).
- [ ] Missing modules and atoms (2026-10-05 audit): `useSafeArea`, `useActionGroup`; `DropZone` and `FileTrigger`
  atoms; `Group`, `Heading`, `Header` and a generic `Text` atom.
- [ ] Deferred by the user ("skip for now"): TOML in `utils::syntax_highlight` (syntect's default syntaxes have none):
  a small own `.sublime-syntax` in a separate `SyntaxSet` (no merge, which would deserialize every default syntax),
  loaded from a build-time dump (no `yaml-load` at runtime); apps highlighting in the browser pay for it in their wasm.

### Conventions still to apply
- [ ] Generics sweep, rest: the hooks below the typed atoms (`use_radio_group_state`, `use_select_state`, ... take
  `Key`s; worth it?), `ListBox`/`GridList`/`Table` selections (`Selection` of keys). Color channel values stay `f64`
  (decided).
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

### Build performance
Numbers, methods and decisions: `documentation/build-performance.md` (measure with `scripts/build-bench.sh`).
- [ ] Book profile vs advice: the book's `[profile.wasm-release]` has `lto = true`, `codegen-units = 1`, `panic =
  "abort"`, while the advice (and the book's guide) says LTO for production builds only
  (`CARGO_PROFILE_WASM_RELEASE_*`): follow the advice or change it.
- [ ] Without `erase_components` (cargo-leptos' release default): 23 browser tests fail: focus lost on in-place
  rebuilds (`use_landmark::navigate`, `test_table_selection::{changing_columns, removing_the_focused_column_header}`:
  leptos-element-capture 0.1's `rebuild`, fixed in its unreleased 0.2) and hydration panics ("expected `<button>`,
  found Comment") on `/hooks/focus`, `/hooks/focus-safely`, `/atoms/focus-scope?only=shadow` (`<Show>` +
  `<Portal use_shadow=true>`; not investigated). Run the suite without erasure before releases (test-app needs
  `#![recursion_limit = "256"]`, its server `CARGO_PROFILE_SERVER_RELEASE_DEBUG=false`, ~21 GB RAM). Also measure the
  book's LTO production build in both modes.
- [ ] ICU4X calendars: `utils/date_time_formatter.rs` formats Gregorian names with the any-calendar
  `DateTimeFormatter`; `FixedCalendarDateTimeFormatter<Gregorian, _>` would let the linker drop every other calendar's
  data and code. Measure; the date field (`hooks/datepicker/format.rs`) needs the locale's calendar.
- [ ] typed-builder `PropsBuilder::build` is instantiated per combination of props a call site sets (5.5% of the
  book's IR): measure what grouping rarely used props of the biggest atoms (`TextField`, `DatePicker`, `SearchField`,
  `Calendar`, `NumberField`) into `Default` structs saves.
- [ ] Route splitting with `#[lazy_route]` + `--split` (main module −38%, but ~90 files per first page visit and an
  unstable Leptos feature): try a coarser grouping first; the user decides.

### Testing infrastructure
- [ ] Split the performance tests' shared fixtures (decided by the user, 2026-10-16: one fixture per subject; an agent
  is splitting them now): drag and drop (`DraggableListPage` in `perf/hooks/use_draggable_item.rs`,
  `DroppableListPage` in `use_droppable_collection.rs`), grids (helpers in `perf/hooks/use_grid.rs`,
  `use_grid_list.rs`, `use_table.rs`, `perf/atoms/table.rs`; five fixtures re-export a sibling's `PagePerf`),
  collections (one fixture component per family, in `atoms/select.rs`, `combo_box.rs`, `tag_group.rs`,
  `hooks/use_listbox.rs`, `use_selectable_list.rs`, `use_collection.rs`, `use_select.rs`, `use_combobox.rs`,
  `use_tag_group.rs`); `pub(crate)` test helpers shared between sibling test modules.
- [ ] Budget `allocated_bytes` and `largest_allocation` (new, unbudgeted). Cause of the 16-byte-step variation
  (proven, 2026-10-16): reactive_graph's `mark_subscribers_check` clones the address-hashed `SubscriberSet`
  (indexmap sizes the clone's entries to the table's capacity, which tombstones reduce), so bytes follow the heap
  layout: deterministic within one build, different across builds. Fix at the source (decide, user): copy the
  subscribers into a `Vec` instead of cloning the set (also saves an allocation per notification) or a deterministic
  growth policy in `SubscriberSet::subscribe`: fork patch and upstream PR. Then: windows also wait for frees (count
  deallocations in `Counters::activity`; late frees seen in 3 of 18 pages), `is_budgeted` for both, aggregate
  `largest_allocation` by max, one update, two checks. Later: a size-class histogram, `reallocated_bytes`.
- [ ] Performance framework, next steps:
  - Native counts: a test binary with a thread-local counting allocator and `reactive_graph`'s `metrics` (patch the
    workspace too): allocations and reactive work per hook setup and update, with counting inputs ("changing
    `is_disabled` runs the press state once"); allocations and time of rendering each atom on the server. Same metrics
    and budget files.
  - Leak checks: mount, use and unmount a fixture N times, collect garbage (`HeapProfiler.collectGarbage`), then live
    wasm bytes, DOM nodes and JS event listeners (`Memory.getDOMCounters`) must be back at their baseline.
  - Timings as A/B: `just perf-ab <git-ref>` builds the baseline in a worktree, alternates runs with the current tree
    on one machine and reports the median difference with a confidence interval (bootstrap).
  - Explaining: `just perf-trace <scenario>`: a CDP trace, a CPU profile summarized as text (hottest wasm functions by
    self time; needs the wasm's names section) and the steps as `performance.measure` marks; forced layouts per
    measurement (layouts before the action's frame) need the trace too.
  - The book: hydration of every page (wasm compile and instantiation, hydration time), layout shifts during
    hydration (budget 0), a few interactions on large pages.
  - Upstream `reactive_graph::metrics` (the fork's branch `leptonic-metrics`, ~30 lines plus a test) so that the patch
    can go.
- [ ] Performance suite scenarios: only `perf/virtual-list` so far. Candidates, by expected value: a `Table` of 1,000
  rows (mount, sorting, selecting all), `ComboBox` filtering a long list per keystroke, `Select` with 320 options
  (opening), `ListBox` of 5,000 options (mount, range selection), `Calendar` mount and month changes, `DatePicker`
  opening, a `NumberField`/`DateField` keystroke, a `Tree` expanding 1,000 rows, page hydration of a large fixture; a
  `VirtualList` with single-line appends at 100 per second.
- [ ] Session start failures: the first wave of parallel sessions after chromedriver starts fails with
  `StartWebDriverSession` ("error sending request for url .../session", chrome-for-testing-manager 0.14): sessions 2
  and 4 of the first 8 in every evening run of 2026-10-10 (they pass with `BROWSER_TEST_PARALLELISM=1`). A transport error, not a timeout: look
  into chromedriver's handling of concurrent `POST /session` and retrying session creation in browser-test (its
  checkout has the user's uncommitted work).
- [ ] Fixture instances that existed only for sequential cases (every case now loads its page fresh): merge the
  redundant ones and switch their cases to the kept instance:
  - `atoms/date_picker.rs`: decide whether `required` vs `form-required` (plain `<form>` vs `Form`) and `empty` vs the
    picker in `focus` are redundant. `atoms/date_field.rs` (optional): `bc`, `help` → `edit-date`.
  - `atoms/number_field.rs`: `#nf-validate-submit` → `#nf-validate` (re-check `validate_commit_behavior`'s Tab focus);
    `#nf-currency` → `#nf-form-value` (`pasting_into_a_format`).
  - `hooks/focus_within.rs` (low value): `#test-fw-change-*` → the basic container with `on_focus_within_change`.
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

Page and writing rules: `documentation/documentation-strategy.md` (incl. "Verification"); look, tokens and which
leptonic piece to use: `examples/book-ssr/STYLE_GUIDE.md`. Library gaps the book hits go into the library's sections
above and, while the book waits, under "Waiting on the library".

### Next
- [ ] Once the user decides on `MaybeSignal` (atom reference props as `MaybeSignal<Option<Id>>`, texts as
  `MaybeSignal<Option<Oco>>`; library "Bugs"): document it (Hooks & Atoms "Ids", the atoms' Props tables). Until
  then the book doesn't present it.
- [ ] The dependency future-incompatibility warning in `just serve`: `leptos-routes 0.4.2` → `leptos-routes-macro
  0.4.1` → `proc-macro-error2 2.0.1` re-exports the private `proc_macro` extern crate
  ([upstream issue #13](https://github.com/GnomedDev/proc-macro-error-2/issues/13)); all latest as of 2026-10-09.
  Update the routes dependency once it replaces this unmaintained dependency; don't suppress the warning or patch the
  registry cache.
- [ ] Rules the book contradicts (`documentation-strategy.md`; fix the pages or change the rule): group overviews
  have a "Recipes" section with a subsection per part, but `groups/status.rs` uses a top-level `Section title="Alert"`
  (only `groups/layout.rs` follows it); "every entry has a marker or a badge" doesn't hold for guides
  (`doc_layout.rs`: markers only in Concepts, badges only in Building blocks); a building block placed in a concept
  group isn't caught by the placement tests (it looks like a single-layer concept); the single-layer Tree Hooks page
  has a Keyboard section, a case the rules don't cover; Verification says `cargo clippy --features ssr --tests`,
  `just clippy` omits the book's `ssr` feature.
- [ ] The browser suite (240 tests, ~32 s) waits 15-30 s for `markdown::warm_markdown_cache`, which converts the ~210
  pages one after another at server start (search and the LLM index wait for it). Convert several pages at a time
  (also shortens the production server's warm-up).
- [ ] Decide (user): self-host the fonts instead of Google Fonts. Every test's fresh browser context fetches the
  render-blocking stylesheet and fonts again (blocking them made the page tests 25% faster, 18.97 s → 14.28 s); the
  suite depends on the internet for them; embedding Google Fonts is a known GDPR issue in Germany.
- [ ] The Dockerfile builds from the repository root (path dependency on `../../leptonic`); not yet test-built (the
  user's call: images, a downloaded install script, a full release build).

### Waiting on the library
- TOML code blocks stay unhighlighted (TOML highlighting deferred by the user).
- Search results → Autocomplete with arrow keys through results.

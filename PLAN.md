# Leptonic: working plan

The only todo list of the repository: open work of the library (`leptonic/`, `leptonic-theme/`, `testing/`) and
the book (`examples/book-ssr/`, section "Book"). Keep it to open items: finished work goes to
`documentation/history.md`, long-term rules and knowledge elsewhere:
- guiding decisions and API conventions (C1–C15): `documentation/conventions.md`;
- implementation patterns: `documentation/{hooks,atoms}-implementation.md`;
- Leptos/browser/tooling pitfalls: `documentation/lessons.md`;
- consumers (crudkit, agnite dev-ui) and what they depend on: `documentation/consumers.md`.

## Waiting on the user

- **leptos-styles suggestion:** an `add_reactive_unchecked` method; today an always-present reactive unchecked value
  needs `add_optional_unchecked(prop, move || Some(..))`. leptos-css lacks `Margin::all`-style helpers.
- **leptos-element-capture suggestions:** `PartialEq`/`Eq`/`Hash` (identity) for `CapturedElement` (the collection
  item registry tracks registration ids instead); `try_get_untracked` for deferred callbacks.
- **Leptos bug report** (2026-10-06): reactive_graph 0.2.15 can skip an effect run. A memo recomputed while the
  effect checks a downstream memo doesn't mark the effect dirty (`Observer::is` skip in `MemoInner::update_if_
  necessary`), and is clean when the effect checks it next; if the downstream memo came out unchanged, the effect
  doesn't run. Leptonic reads upstream first ("Effect Read Order" in `documentation/hooks-implementation.md`). Repro
  and issue draft ready: `~/dev/leptos-issue-repros/a-effect-skips-run` (also fails on 0.9.0-beta2): file it?
- **Leptos `erase_components` report** (2026-10-06): attributes spread onto a component (`attr:id`) are lost when
  its root element is replaced in erased builds (details in `documentation/lessons.md`). Repro and issue draft:
  `~/dev/leptos-issue-repros/b-spread-attrs-lost` (passes on 0.9.0-beta2: ask for a 0.8 fix): file it?
- **Leptos bug, listeners leak on rebuild** (2026-10-07, from agnite dev-ui; tachys 0.2.19 and Leptos main of
  2026-03-14): with `erase_components`, attributes spread onto a component (`<Comp {..hook.props.into_attrs()} />`)
  go through `AnyViewWithAttrs`, whose `rebuild` removes nothing of the previous set (it rebuilds against an empty
  state) and drops the new attributes' states: a view rebuilt in place keeps the disposed owner's handlers (which
  panic when they run) and loses the new ones. Repros: `component_spread_rebuild_known_issues`
  (`BROWSER_TEST_KNOWN_ISSUES=1`) and `~/dev/leptos-issue-repros/c-spread-listener-leak` with an issue draft
  (passes on 0.9.0-beta2: ask for a 0.8 fix): file it? Meanwhile atoms avoid needing such spreads (`VirtualList`'s
  `is_focusable`).
- **Leptos bug, keyed `For` moves kept rows** (2026-10-07): the keyed diff moves a kept item whose relative order
  didn't change (items removed before it, added after it), which collapses a text selection in it (why
  `render_visible_items` mounts rows itself). Repro and issue draft: `~/dev/leptos-issue-repros/d-keyed-for-moves`
  (also fails on 0.9.0-beta2): file it?
- **leptos-classes:** `classes="a b"` compiles but panics during SSR (`ClassName::from(&'static str)`): split
  whitespace-separated names, or reject them at compile time? (The user's crate, published.)
- **Commits:** the latest is 842585b (2026-10-07); the working tree holds the changes since.
- **Open decisions from the fidelity wave (2026-10-07):**
  - `icu_experimental` as a dependency, for a CLDR-aware number formatter (currencies, units, percent patterns)?
  - The shape of `UseFormValidationInput.focus` (today `Callback<()>`, C10).
  - Untrack `testing/test-app/style/leptonic` (no longer generated) from git?
  - Remove `examples/book-ssr/style/leptonic/` (55 tracked files: an old copy of leptonic's styles incl. the removed
    components and themes, referenced nowhere)?

## Library roadmap

Order: open bugs first, then families (each a complete package: API onto the conventions, missing react-aria
features, re-sync to the current upstream commit, deviation block, fixture + browser test from upstream's tests,
atoms, book heads-up), then cross-cutting work. Port react-aria's behavior and structure (cross-check
what it provides before designing); adapt only the API shape (`documentation/conventions.md`).

### The atom theme
The components layer is gone (2026-10-07, see history); leptonic is hooks + atoms + this optional theme. Note:
`ColorSwatchPicker`/`ColorSwatchPickerItem` render through `ListBox`/`ListBoxItem` and carry their classes too (style
`.leptonic-ListBox:not(.leptonic-ColorSwatchPicker)` or scope the listbox rules).
- [ ] The atom theme in `leptonic-theme` (`documentation/atom-theme.md`): a port of react-aria-components'
  starter styles (`starters/docs/src/*.css`) to `leptonic-theme/scss/atoms/`, entry `leptonic-atoms.scss`. Ported
  2026-10-07: every family with atoms (51 stylesheets, compiled with dart-sass); not yet checked in a browser. The
  user confirmed (2026-10-07) to keep and maintain it; the book won't use it (its own styling). Open:
  - A showcase to check it visually (light and dark, every family), e.g. an `examples/atom-theme` app with the
    starter's markup per family; then fix what's off. Not the test-app (the user, 2026-10-07: it needs no theme).
  - Atom gaps the stylesheets need (else those rules don't apply; done 2026-10-07: `ListBox`/`GridList` root
    `data-empty`/`data-focused`/`data-focus-visible`/`data-layout` (+ `data-orientation` on `ListBox`), section
    heading default classes, `Menu` `data-empty`, `MenuItem` `data-selection-mode` + public `MenuItemCtx` state,
    `GridListItem`'s gridcell `display: contents`, `TabList` `data-orientation`, `TabPanel` focus ring,
    `ComboBoxButton` `data-hovered`/`data-disabled`; `ModalBackdrop`'s `--page-*`/`--visual-viewport-*`):
    `Table` `data-focus-visible`, `TableBody` `data-empty`,
    rows/cells/column headers/resizer `data-focus-visible`; custom column header content (a flex wrapper, sort
    indicator: the resizer rules assume it); a `SelectionIndicator` atom (sliding tab indicator), `TabPanels`;
    `ProgressBarFill`/`MeterFill` exposing `--percent`; `DateInput` with
    `part` rendering `slot="start"`/`"end"`; `CalendarCellButton` children as a function of the cell state (range
    calendar: selection start/end, hovered, pressed, the formatted day); a `ToastQueue` `wrap_update` hook (view
    transitions); the table's selection checkbox and `GridListItem`'s as `Checkbox` atoms; links in list
    box/menu/grid list/tag items (`data-href` rules dropped).
  - Decide: `ColorPicker` renders no element (the theme's `.color-picker` trigger is the app's class, and a
    `leptonic-Button` trigger brings button looks it doesn't undo); calendar buttons and the toast close button are
    quiet only with `attr:data-variant="quiet"` (the starter's components set it), maybe quiet by default in the
    theme; the `Disclosure` trigger is a `Button` (the theme undoes button looks).
  - Not ported (no atoms yet): `CommandPalette`, `DropZone`, `NavigationTree`, `SegmentedControl`, `Sheet`,
    `TokenField`, `Tree`.

### Consumer requests
- [ ] Virtualizer, rest (see history: `ListBox` and `VirtualList` done 2026-10-07): virtualized `GridList`, `Table`,
  `ComboBox`/`Select` popovers and menus (RAC's virtualized `GridList`/`Table`/`ComboBox`/`VirtualizedMenu` tests;
  no consumer asks yet; dev-ui's collections are all short), sections/headers in virtualized lists,
  `GridLayout`/`TableLayout`/`WaterfallLayout`, drop targets. `VirtualList` under dev-ui's load (20,000 lines, 10–100
  appended per second): an append rebuilds the collection and lays out again, 11.6 ms native at 20,000 rows
  (2026-10-08; wasm likely 2–3×). At 100 separate appends per second that saturates the main thread: dev-ui
  should batch appends (one per animation frame); measure in the browser before going further (incremental
  appends would need a persistent node map).
### Bugs and review leftovers
- [ ] Grid list with sections (`#glf-sections` in the test-app): the Banana row's `aria-labelledby` resolves to
  "BananaYellow Yellow": its first referenced element contains the description too, so the accessible name repeats
  it. Compare with react-aria's `GridListItem` labelling (found by the browser test consolidation, 2026-10-08).
- [ ] Reorderable collection rows set `data-dragging` as a presence flag (`""`), the DnD hooks and other atoms write
  `"true"`/`"false"`: make it consistent (2026-10-08).
The 2026-10-07 react-aria fidelity review was applied the same day by nine agents (history: "Fidelity review
2026-10-07"). What is open from it is below, by family.

- [ ] Flaky: `overlay_position_tests` fails under load (the full suite, 2026-10-07 twice: no
  `.test-op-flip-popover[data-placement=bottom]` within 10 s; passes alone).
- [ ] `TabPanel`s rendered before their `TabList` don't know about tabs disabled through `Tab::is_disabled` (the
  default selection skips them only once the tabs have rendered).
- [ ] Tree: on a page where the tree never had focus, clicking a row's expand button leaves the focus on the button
  instead of the row (upstream: `preventFocusOnPress` + `setFocusedKey`). Found 2026-10-08 when the browser test cases
  became independent (an earlier case had focused the tree); known-issue case `tree::expand_button`. Suspects:
  `utils/prevent_focus.rs` restoring focus to `<body>`, or the row-focus effect when the tree had no focus before.

### Families
- [ ] **Number formatting:** a formatter that knows currencies/units/percent patterns from CLDR (`format_percent`
  appends `%`; German expects `50 %`) with typed currency/unit values instead of `NumberFormatOptions`' strings. Needs
  `icu_experimental` (not yet a dependency: the user decides). Then mirror the remaining `NumberParser.test.js`
  cases (localized unit names and plurals, SAR `ر.س.‏`, de "50 %") and ar-AE number field browser tests.
- [ ] **Forms:**
  - C10 `UseFormValidationInput.focus: Callback<()>` (`use_form_validation.rs:41`): no clear Rust-native shape yet
    (the date field focuses its first segment): decide.
  - Several names per validation state (react-aria `name: string | string[]`: server errors under the range end's
    name): `UseFormValidationStateInput.name` as a list (15 literals).
  - Single `Checkbox`/`Radio`/`Switch` atoms removed (2026-10-07): tell crudkit and dev-ui, whose code still uses
    them and builds from this tree (sites in `documentation/consumers.md`).
- [ ] **Button, link, tabs:** a `SelectionIndicator` atom (needs a shared-element transition; snapshot-before-unmount
  ordering in Leptos unclear) and link tabs (`href` on `Tab`), both listed as omitted in `atoms/tabs.rs`; Tabs
  `keyboard_activation` as a signal (blocked: `CollectionOptions::select_on_focus` is fixed); building the tab
  collection from the children (RAC; documented as a difference); `ProgressBarFill`/`MeterFill` `--percent`.
- [ ] **Calendar and date picker:** the picker's contexts reach its popover (RAC `clearContexts`,
  DatePicker.test.js:340; needs a context-clearing mechanism in the popover/field atoms); range formatting with shared
  fields ("June 1 – 15, 2024": ICU4X has none yet).
- [ ] **Collections:**
  - tree expansion and tree input (keyboard delegate, Tab navigation, select on press up); focus-mode enum names;
    tag group options; per-item `on_action`; menu item links; combo box item actions/links, `aria-labelledby`
    fallback, `should_focus_wrap`; tab links; grid list selection checkbox (labelled by its row); `SelectionManager`
    cell selection/layout ranges; load more; DnD on the collection atoms + drag preview + tree drops; Tree atoms;
    Autocomplete (with `get_pointer_type()`, ported 2026-10-07), then searchable Select/Multiselect (search keeps
    focus, collection unfiltered for the trigger text); combo box label click shows no focus ring (check upstream
    first); `atoms::prelude` with `Grid*`/`Table*` (legacy names gone); table resizing leftovers (cursor
    overlay while mouse-resizing, resizer `data-focused`/`data-focus-visible`/`data-hovered`, scrollable ancestor,
    empty tables).
  - `ListBoxItem`/`GridListItem` rendering `<a href>` for link items (RAC).
  - `KeyboardDelegate`/`LayoutDelegate` don't require `Debug` (inputs holding them have manual impls).
  
- [ ] **Grid, table, DnD, virtualizer:** a column header inside a `TooltipTrigger` (no fixture: `TableHeader` renders
  the headers); RAC's TagGroup-in-cell case; virtualizer section headers are measured only once the virtualized
  ListBox renders them; layout node pointer comparison needs a collections change; DnD: re-run
  `dnd_screen_reader_tests` (timing fix, `test_dnd.rs` ~556) and `clipboard_tests` (never run), mirror
  useDraggableCollection's multi-select and action keyboard cases.
- [ ] **Interactions and focus:** the on-screen keyboard tracker (upstream `utils/keyboard.tsx`/`runAfterKeyboard`,
  5a6c32500), used by `use_prevent_scroll`, `use_viewport_size`, Popover and ModalOverlay (listed as omitted);
  `focusable_tree_walker.rs` still differs from upstream (doesn't reject nodes inside `from`; dedupes radio groups by
  the `from` group, not the current node).
- [ ] **Overlays:** no tests yet for the `use_dialog` shadow-DOM focus check and "closed overlays observe nothing";
  the landmark "toggle browser tabs" test was dropped (headless Chrome leaves `<body>` focused; unverified).
- [ ] **Color:** `ColorArea` x/y step props (`atoms/color_area.rs:94`).
- [ ] Missing modules and atoms (from the 2026-10-05 audit, still open): `useSafeArea`, `useActionGroup`; `DropZone` and `FileTrigger` atoms; `Group`, `Heading`,
  `Header` and a generic `Text` atom.
- [ ] Deferred by the user (2026-10-07: "skip for now"): `utils::syntax_highlight` lacks TOML (syntect's default
  syntaxes have none; the book's TOML blocks stay plain): a small own `.sublime-syntax` in a separate `SyntaxSet` (no
  merge, which would deserialize every default syntax), loaded from a build-time dump (no `yaml-load` at runtime;
  apps that highlight in the browser pay for the syntax in their wasm).

### Conventions still to apply
- [ ] Generics sweep, rest: the atoms' value-like selections are typed (`SelectionValue`, 2026-10-08); open: the
  hooks below them (`use_radio_group_state`, `use_select_state`, ... still take `Key`s; worth it?), `ListBox`/
  `GridList`/`Table` selections (`Selection` of keys), spin button values. (Color channel values stay `f64`, decided
  2026-10-07.)
- [ ] Optional `Callback` props with generic arguments (`on_selection_change: Option<Callback<HashSet<Key>>>`) can't
  infer an untyped closure in `view!` (E0282).
- [ ] Recount C1/C2/C3/C8/C10/C12 after the 2026-10-07 wave (every listed violation was addressed by its family;
  grep for stragglers: `pub fn use_\w+\([^)]*,`, `Callback<\(`, `: &'static str` in inputs, string `aria_*`).

### Cross-cutting
- [ ] Localized strings, rest: `useDefaultLocale` (browser language + `languagechange`) and a reactive
  `I18nProvider` `locale` prop; the autocomplete's `collectionLabel` once `use_autocomplete` exists (add its bundle
  to `scripts/port-intl-strings.py`). A non-English browser check per migrated family where none exists yet
  (calendar, DnD, color, toast).
- [ ] Atom hygiene: `data-hovered` on `MenuItem` (RAC; book request); context for render state (selected/pressed) in
  Tab/GridRow/TableRow/GridListItem; `FocusScope` has no default class; `atoms::prelude` with `Table*`; one rule for
  `Children` vs `ChildrenFn`; atoms dropping `attr:` attributes.
- [ ] Atoms forward their element (react-aria-components forwards `ref` everywhere): a `node_ref` prop on every atom
  rendering an element (done for `Input`/`TextArea`, 2026-10-07).
- [ ] Port hooks onto `use_keyboard` shortcuts (upstream uses them in ~20 hooks).
- [ ] Input composition: which `MergeWith` types become unnecessary (e.g. link + press, tooltip trigger)?
- [ ] Theme: the global `[data-focus-visible]` outline also hits virtually focused listbox options.
- [ ] `use_clipboard` lives in `hooks::dnd`: move it to its own module?
- [ ] Later: step list, token field, preview trigger, `S/data` list helpers.
- [ ] Hygiene: split the largest functions (`use_press`, `use_move`, `use_selectable_collection`, `FocusScope`);
  docs for public items (undocumented pub fields of `UseFocusInput`, DnD states,
  `SelectionOptions.disabled_behavior`, `UseListBoxInput`/`Return`, `CollectionOptions`, `SelectState`, `Column`,
  `TableColumnResizeState.table_state`, `TabListState.list`, `TreeState.expansion`, `UseGridCellReturn`) and most
  atom props; recount the files without deviation block or `// Upstream:`/`// No upstream:` header, the legacy "based
  on work in" lines and `cfg_attr(ssr, allow(dead_code))` (the families fixed theirs 2026-10-07). Run the book's
  `api_check` after API changes.
- [ ] `documentation/lessons.md`: `StoredValue::new_local` is not the only SSR cross-thread problem; Leptos Effects
  also panic when their owner is dropped on another thread (found with `use_toast_region`).

### Build performance
Numbers, methods and findings: `documentation/build-performance.md` (measure with `scripts/build-bench.sh`).
- [ ] ICU4X calendars: `utils/date_time_formatter.rs` formats Gregorian weekday/month names with the any-calendar
  `DateTimeFormatter`; `FixedCalendarDateTimeFormatter<Gregorian, _>` would let the linker drop every other
  calendar's data and code. Measure; the date field (`hooks/datepicker/format.rs`) needs the locale's calendar.
- [ ] typed-builder `PropsBuilder::build` is instantiated per combination of props a call site sets (5.5% of the
  book's IR): measure what grouping rarely used props of the biggest atoms (`TextField`, `DatePicker`,
  `SearchField`, `Calendar`, `NumberField`) into `Default` structs saves.
- [ ] Users' guide in the book ("Build times and bundle size"), from the document's "Advice for users".
- [ ] Decide (user): route splitting with `#[lazy_route]` + `--split` (main module −38%, but ~90 files per first
  page visit and an unstable Leptos feature: try a coarser grouping first).

### Testing infrastructure
- [ ] `use_toast_state.rs`'s `now()` calls `js_sys::Date::now()` in every non-`ssr` build, which panics in native
  tests (found 2026-10-08; the virtualizer's twin is fixed: `cfg(target_arch = "wasm32")`). Native toast timer
  tests need the same fix.
- [ ] Fail-first: most 2026-10-07 regression tests were written together with their fix (the test-app was often
  broken by parallel edits). Spot-check the important ones against the old code (`git stash` is off limits: revert
  the fix in a scratch copy).
- [ ] `testing/test-app/style/leptonic` (107 generated files) is no longer written (2026-10-07) but still tracked in
  git: untrack it (`git rm -r --cached`, the user's call).
- [ ] `browser-test` is a path dependency on the user's checkout (`../../browser-test`, 0.6.0 unreleased: per-session
  Chrome profiles in `<target>/tmp/browser-test-profiles`, cancellation, `rustls-no-provider` + `ring`, focused
  session pages, failure reports, session reuse). Switch to the crates.io release once it is published (the user's call).
- [ ] browser-test's own runner tests (they kill child runs on purpose) left one empty
  `/tmp/org.chromium.Chromium.scoped_dir.*` (2026-10-08): find which session still lets chromedriver create one.
- [ ] The test-app's `cargo check` needs `LEPTOS_OUTPUT_NAME=...` set (note it in CLAUDE.md's commands).

## Book

Page and writing rules: `documentation/documentation-strategy.md` (incl. "Verification"); look, design tokens and
which leptonic piece to use: `examples/book-ssr/STYLE_GUIDE.md`. Library gaps the book hits go into the roadmap above
and, while the book waits for them, under "Waiting on the library" below. Finished book work:
`documentation/history.md` ("Book").

### Next
- [ ] The book's browser tests still use browser-test 0.5, which leaks a Chrome profile into `/tmp` (a RAM disk) for
  every session not quit cleanly. Switch as the library did (2026-10-08, `leptonic/tests/browser_test.rs`): browser-test
  0.6 with `Cancellation::on_shutdown_signals()`, `ChromeProfilesDir` in `CARGO_TARGET_TMPDIR`, features
  `rustls-no-provider` + `rustls` with `ring` (installed as the default provider), and a direct `thirtyfour` with
  `cdp` if the book's tests use `driver.cdp()`. Failure reports (test-code frames, last steps) then come with it;
  `documentation/browser-tests.md` describes the library suite's helpers and checks, which the book's may follow.
- [ ] The Dockerfile builds from the repository root (path dependency on `../../leptonic`); not yet test-built. A test
  build (images, a downloaded install script, a full release build) is the user's call (main, 2026-10-07).

### Waiting on the library
- TOML code blocks stay unhighlighted: TOML highlighting is deferred (the user: "skip for now").
- Search results → Autocomplete with arrow keys through results (R3j).

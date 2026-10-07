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
- **Commits:** the latest is 842585b (2026-10-07); the working tree holds the changes since.
- **Open decisions from the fidelity wave (2026-10-07):**
  - `icu_experimental` as a dependency, for a CLDR-aware number formatter (currencies, units, percent patterns)?
  - Keep the single `Checkbox`/`Radio`/`Switch` atoms next to the new `*Field`/`*Button` pairs (RAC deprecates them)?
  - The shape of `UseFormValidationInput.focus` (today `Callback<()>`, C10).
  - Untrack `testing/test-app/style/leptonic` (no longer generated) from git?
  - Clippy in release builds (cost).

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
  `GridLayout`/`TableLayout`/`WaterfallLayout`, drop targets. Check `VirtualList` with dev-ui's load (20,000 lines,
  10–100 appended per second): the collection is rebuilt per append, the rebuild is O(n²) and every layout pass
  clones the whole collection (see the fidelity review's bugs below).
- [ ] Tree tables (agnite dev-ui, 2026-10-07): react-aria-components' expandable `Table` rows (`treeColumn`, nested
  rows, `expandedKeys` + `onExpandedChange`, ←/→ collapse/expand, `role="treegrid"`, `aria-level`/`aria-expanded`/
  `aria-setsize`/`aria-posinset`): `TableCollection` rows with child rows, `expanded_keys` + `set_expanded_keys` on
  `Table` (C4), an expander button atom for the tree column's cells.

### Bugs and review leftovers
The 2026-10-07 react-aria fidelity review was applied the same day by nine agents (history: "Fidelity review
2026-10-07"). What is open from it is below, by family.

- [ ] From crudkit (2026-10-05, not reproduced): in a `use_table_cell` cell with `CellFocusMode::Child`, Enter on a
  `Button` atom (not the cell's first child) first moves focus to the cell's first focusable child, then the
  button's press fires. Repro: a fixture row with two buttons in one cell.
- [ ] `use_table` replaces instead of merging `aria-describedby` (latent until the grid has a description).
- [ ] `classes="a b"` compiles but panics during SSR (`ClassName::from(&'static str)` in leptos-classes, the user's
  crate): accept/split it or reject it at compile time.
- [ ] Nested modals: closing the inner modal leaves focus on `<body>` instead of the opener inside the outer modal
  (likely `FocusScope` restores before `aria_hide_outside` un-inerts the outer one; note at
  `leptonic/tests/ui_tests/test_overlay.rs:147`).
- [ ] A `MenuTrigger` inside an RTL `I18nProvider` doesn't open (click or ArrowDown); found writing the RTL submenu
  keys test (removed with its fixture). Then: RTL submenu keys test.
- [ ] RTL date picker: in he-IL, ArrowLeft from the leftmost segment doesn't reach the picker button, even after
  `tabbable_segments` walks all tabbables (note at `test_date_picker.rs`, `right_to_left`).
- [ ] Flaky or unexplained (2026-10-07, after the six failures of the fidelity wave were fixed):
  `combobox_multiple_tests` failed once with the required multiple `ComboBox` keeping `data-invalid` after a
  selection and a blur (7 later runs pass; if it comes back, log `required` and the hidden value against the blur);
  `overlay_position_tests` failed once under load (passes alone).
- [ ] `TabPanel`s rendered before their `TabList` don't know about tabs disabled through `Tab::is_disabled` (the
  default selection skips them only once the tabs have rendered).
- [ ] Grid list: mirror the last steps of upstream's "ArrowLeft/Right cycles through children and row element"
  (ArrowLeft from the first child to the row, ArrowRight back in) and its RTL variant.
- [ ] Tree: an item that gets children doesn't become expandable (`aria-expanded`): `use_grid_list_item.rs` ~200-220
  reads `has_child_nodes` once (assertion commented in `tree_cases_tests`).
- [ ] Virtualizer: `VirtualListFollowToggleTests` passes without the follow-mode fix (doesn't reproduce; the unit
  test `anchoring_changes_keep_measured_sizes` does): rework or drop it.
- [ ] Safe triangle: the `use_safely_mouse_to_submenu` browser test never saw `pointer-events: none` after diagonal
  moves (removed): test problem or hook bug.

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
  - Decide: RAC deprecates the single `Checkbox`/`Radio`/`Switch` for the new `*Field`/`*Button` pairs (ported
    2026-10-07); keep both or remove the single ones (no legacy).
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
  - Grid, table and date segment code still build an ICU `Collator` per read: use `utils::filter::use_collator`.
  - `KeyboardDelegate`/`LayoutDelegate` don't require `Debug` (inputs holding them have manual impls).
  - `use_grid_selection_announcement`/`use_highlight_selection_description` (omitted until localized strings;
    `use_grid_list.rs:40`, `use_grid.rs:56`).
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
- [ ] **Color:** channel, color and hue names stay English (localized strings); `ColorArea` x/y step props
  (`atoms/color_area.rs:94`).
- [ ] Missing modules and atoms (from the 2026-10-05 audit, still open): `useDisplayNames` (English segment names,
  `use_date_field.rs:52`), `useSafeArea`, `useActionGroup`; `DropZone` and `FileTrigger` atoms; `Group`, `Heading`,
  `Header` and a generic `Text` atom.
- [ ] Deferred by the user (2026-10-07: "skip for now"): `utils::syntax_highlight` lacks TOML (syntect's default
  syntaxes have none; the book's TOML blocks stay plain). Highlighting also runs in the browser (`kit/code.rs` after
  navigation), so wasm size counts: a small own `.sublime-syntax` in a separate `SyntaxSet` (no merge, which would
  deserialize every default syntax), loaded from a build-time dump (no `yaml-load` in the wasm).

### Conventions still to apply
- [ ] Generics sweep (user, 2026-10-05): typed values instead of the dynamic collection `Key` for value-like
  selections (RadioGroup, Select, ComboBox, ToggleButtonGroup, CheckboxGroup values), spin button values. (Color
  channel values stay `f64`, decided 2026-10-07: channels mix fractional hue and 0..1 ranges in one trait.)
- [ ] Optional `Callback` props with generic arguments (`on_selection_change: Option<Callback<HashSet<Key>>>`) can't
  infer an untyped closure in `view!` (E0282).
- [ ] Recount C1/C2/C3/C8/C10/C12 after the 2026-10-07 wave (every listed violation was addressed by its family;
  grep for stragglers: `pub fn use_\w+\([^)]*,`, `Callback<\(`, `: &'static str` in inputs, string `aria_*`).

### Cross-cutting
- [ ] Localized strings: a `use_localized_string_formatter` equivalent with upstream's message bundles (~36 hooks
  hard-code English, incl. the toast region's and close button's names), `useDefaultLocale` (browser language +
  `languagechange`), reactive `I18nProvider`. crudkit (German) overrides labels through the inputs meanwhile; keep
  that working.
- [ ] Theme flash: the server renders `ThemeProvider`'s default theme, the stored choice applies one frame after hydration
  (`signal_ls`). dev-ui solved it with a cookie (leptos-use `use_cookie_with_options`, its `axum` feature on the
  server) feeding `ThemeProvider`'s `theme`/`set_theme`, and `<html data-theme>` rendered on the server through
  leptos_meta's `<Html>` (2026-10-07). For the docs: recommend a cookie in `signal_ls`'s and
  `ThemeProvider`'s docs; let `ThemeProvider` render `<html data-theme>` on the server itself (optionally via
  leptos_meta), so apps need no own `<Html>`.
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
- [ ] CLAUDE.md's `[package.metadata.leptonic]` example is wrong: the build script writes to
  `<style-dir>/leptonic`, so it should read `style-dir = "style"`.

### Testing infrastructure
- [ ] Switch tests onto the polling helpers (`BaseActions::wait_for_value`/`wait_until`/`wait_for_prop`/
  `assert_stays`, and the `wait_for!`/`stays!` macros of `tests/ui_tests/polling.rs`, which work inside
  `#[async_trait]` bodies where the closure helpers hit a `Send` error: unify the two):
  - hand-rolled 50 ms poll loops: test_combobox.rs:66, :81, :158, :243; test_calendar.rs:786; test_date_field.rs:66,
    :462; test_dnd.rs:67, :176; test_select.rs:207; test_table.rs:103; test_tabs.rs:81; test_tree.rs:59; test_toast.rs;
  - fixed sleeps (positive check → `wait_*`, negative → `assert_stays`): test_checkbox.rs:148, 162, 228, 283, 311;
    test_switch.rs:89, 100, 123; test_radio_group.rs:211, 224, 238; test_button.rs:148–256; test_listbox.rs:103–128;
    test_menu.rs:61, 259; test_calendar.rs:685, 807, 847; test_toast.rs; test_long_press.rs.
- [ ] Native `*_state` tests (helper `crate::testing::{with_owner, flush_effects}`, 2026-10-07) for
  `use_color_channel_field_state`, `use_color_picker_state`, `use_color_slider_state`, `use_form_validation_state`,
  `use_text_field_state`, `use_virtualizer_state`.
- [ ] Fail-first: most 2026-10-07 regression tests were written together with their fix (the test-app was often
  broken by parallel edits). Spot-check the important ones against the old code (`git stash` is off limits: revert
  the fix in a scratch copy).
- [ ] `testing/test-app/style/leptonic` (107 generated files) is no longer written (2026-10-07) but still tracked in
  git: untrack it (`git rm -r --cached`, the user's call).
- [ ] Clippy in release too: some lints depend on type sizes that differ in release (`MaybeProp`, `StoredValue`:
  `trivially_copy_pass_by_ref` fires only there). `just clippy` checks debug builds only; add a release run once the
  user decides on the cost.
- [ ] Chrome profiles leak (browser-test crate, the user's): chromedriver's `/tmp/org.chromium.Chromium.scoped_dir.*`
  profiles stay behind when a session isn't quit cleanly; 2026-10-06 they filled the /tmp quota (14 GB), again
  2026-10-07 (245 dirs, ~17 GB; Chrome sessions then fail to start: "Devtools port number file"). Fix in
  browser-test: an own `--user-data-dir` per session, removed on drop, or a sweep at startup.
- [ ] The test-app's `cargo check` needs `LEPTOS_OUTPUT_NAME=...` set (note it in CLAUDE.md's commands).

## Book

Page and writing rules: `documentation/documentation-strategy.md` (incl. "Verification"); look, design tokens and
which leptonic piece to use: `examples/book-ssr/STYLE_GUIDE.md`. Library gaps the book hits go into the roadmap above
and, while the book waits for them, under "Waiting on the library" below. Finished book work:
`documentation/history.md` ("Book").

### Next
- [ ] The fidelity wave's API changes (2026-10-07): page texts, code samples and API tables. The old→new list is in
  `documentation/history.md` ("API changes of the fidelity review"). Also: a tab disabled via `Tab::is_disabled` is
  never the default selection (server too); `TabPanels` measures in the next animation frame.
- [ ] Off the components layer: phases A, B and C done (2026-10-07; see history). Left: the full browser suite on the
  final state (incl. the two shell search tests `search_lists_results_clears_closes_and_opens_the_first` and
  `search_opens_with_ctrl_k`, which failed in partial runs) and a visual pass (light/dark, 390px) over one page per group
  now that no library styles are loaded; then the follow-up pass for leptonic-e9's consolidated API list.
- [ ] The kit's `Keys` draws group descriptions inside combinations ("Shift + Arrow keys") as key caps: compose them
  as key caps for the keys and plain text for the description (the library's advice; `KeyboardKey::Other` is a key).
- [ ] Browser checks for the book's own code blocks and key caps (styles, copy button).
- [ ] Browser test speed: implicit wait 0 with explicit waits only?; book page loads hydrate debug wasm (~900 ms
  each), a release/wasm-opt build for the browser tests would cut that. Step timing: `BROWSER_TEST_LOG_STEPS=1`.
- [ ] The Dockerfile builds from the repository root (path dependency on `../../leptonic`); not yet test-built.

### Waiting on the library
- TOML code blocks stay unhighlighted: TOML highlighting is deferred (the user: "skip for now").
- Search results → Autocomplete with arrow keys through results (R3j).
- The calendar atoms' English strings wait for localized strings (R4).
- Document once implemented: tree tables; the `Label` atom (renders a `for` pointing at a generated id nothing has,
  R3c).

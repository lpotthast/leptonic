# Leptonic: working plan

The only todo list of the repository: open work of the library (`leptonic/`, `leptonic-theme/`, `testing/`) and
the book (`examples/book-ssr/`, section "Book"). Keep it to open items: finished work goes to
`documentation/history.md`, long-term rules and knowledge elsewhere:
- guiding decisions and API conventions (C1–C15): `documentation/conventions.md`;
- implementation patterns: `documentation/{hooks,atoms,components}-implementation.md`;
- Leptos/browser/tooling pitfalls: `documentation/lessons.md`;
- consumers (crudkit, agnite dev-ui) and what they depend on: `documentation/consumers.md`;
- the 2026-10-05 audit's raw findings ("audit §n"): `documentation/audit-2026-10-05.md`.

## Waiting on the user

- **leptos-styles suggestion:** an `add_reactive_unchecked` method; today an always-present reactive unchecked value
  needs `add_optional_unchecked(prop, move || Some(..))`. leptos-css lacks `Margin::all`-style helpers.
- **leptos-tiptap suggestion** (2026-10-06): attributes for the editable element (tiptap's `editorProps.attributes`;
  0.10's `UseTiptapEditorInput` has none), so it can be labelled (`aria-label`/`aria-labelledby`; leptonic's
  `TiptapEditor` labels only its group).
- **leptos-element-capture suggestions:** `PartialEq`/`Eq`/`Hash` (identity) for `CapturedElement` (the collection
  item registry tracks registration ids instead); `try_get_untracked` for deferred callbacks (audit §2 B5).
- **Leptos bug report** (2026-10-06): reactive_graph 0.2.15 can skip an effect run. A memo recomputed while the
  effect checks a downstream memo doesn't mark the effect dirty (`Observer::is` skip in `MemoInner::update_if_
  necessary`), and is clean when the effect checks it next; if the downstream memo came out unchanged, the effect
  doesn't run. Leptonic reads upstream first ("Effect Read Order" in `documentation/hooks-implementation.md`). File
  an issue upstream (with the combo box repro)?
- **Name of the third layer:** "component" is the styled layer (`leptonic::components`, feature `components`), but
  readers also know it as a Leptos `#[component]` and as a UI element. The book now says "concept" for UI elements
  and "styled component" where needed (2026-10-06). Renaming the layer itself (e.g. `styled`) would remove the last
  ambiguity but changes the public module and feature names: decide.
- **Leptos `erase_components` report** (2026-10-06): attributes spread onto a component (`attr:id`) are lost when
  its root element is replaced in erased builds (details in `documentation/lessons.md`): report upstream?
- **Leptos bug, listeners leak on rebuild** (2026-10-07, from agnite dev-ui; tachys 0.2.19 and Leptos main of
  2026-03-14): with `erase_components`, attributes spread onto a component (`<Comp {..hook.props.into_attrs()} />`)
  are a `Vec<AnyAttribute>`, whose `rebuild` removes the old attributes by key but not their event listeners. A view
  rebuilt in place (same type, new owner) keeps the disposed owner's handlers, which panic when they run. Repro:
  `component_spread_rebuild_known_issues` (`BROWSER_TEST_KNOWN_ISSUES=1`). Fix upstream (rebuild pairwise like
  `AnyAttribute::rebuild`): file an issue / a PR? Meanwhile atoms avoid needing such spreads (`VirtualList`'s
  `is_focusable`).
- **Commits:** nothing has been committed; the working tree holds all changes.

## Library roadmap

Order: open bugs first, then families (each a complete package: API onto the conventions, missing react-aria
features, re-sync to the current upstream commit, deviation block, fixture + browser test from upstream's tests,
atoms, components, book heads-up), then cross-cutting work. Port react-aria's behavior and structure (cross-check
what it provides before designing); adapt only the API shape (`documentation/conventions.md`).

### Remove the components layer (the user's decision, 2026-10-07)
leptonic becomes hooks + atoms + an optional CSS theme for the atoms (`documentation/conventions.md`). Order, so the
tree keeps compiling:
- Step 1 (default classes `leptonic-<AtomName>`) done 2026-10-07. Note for the theme: `ColorSwatchPicker`/
  `ColorSwatchPickerItem` render through `ListBox`/`ListBoxItem` and carry their classes too (style
  `.leptonic-ListBox:not(.leptonic-ColorSwatchPicker)` or scope the listbox rules).
- [ ] 2. The atom theme in `leptonic-theme` (`documentation/atom-theme.md`): a port of react-aria-components'
  starter styles (`starters/docs/src/*.css`) to `leptonic-theme/scss/atoms/`, entry `leptonic-atoms.scss`. Ported
  2026-10-07: every family with atoms (51 stylesheets, compiled with dart-sass); not yet checked in a browser. The
  user confirmed (2026-10-07) to keep and maintain it; the book won't use it (its own styling). Open:
  - A showcase to check it visually (light and dark, every family): e.g. an `examples/atom-theme` app (or a test-app
    build with the atom theme instead of the component themes), with the starter's markup per family; then fix
    what's off. Possibly screenshot tests.
  - Atom gaps the stylesheets need (else those rules don't apply; done 2026-10-07: `ListBox`/`GridList` root
    `data-empty`/`data-focused`/`data-focus-visible`/`data-layout` (+ `data-orientation` on `ListBox`), section
    heading default classes, `Menu` `data-empty`, `MenuItem` `data-selection-mode` + public `MenuItemCtx` state,
    `GridListItem`'s gridcell `display: contents`, `TabList` `data-orientation`, `TabPanel` focus ring,
    `ComboBoxButton` `data-hovered`/`data-disabled`): `Table` `data-focus-visible`, `TableBody` `data-empty`,
    rows/cells/column headers/resizer `data-focus-visible`; custom column header content (a flex wrapper, sort
    indicator: the resizer rules assume it); a `SelectionIndicator` atom (sliding tab indicator), `TabPanels`;
    `ModalBackdrop`'s `--page-width`, `--page-height`, `--visual-viewport-height` (RAC's `ModalOverlay` sets them;
    the content's max height needs them); `ProgressBarFill`/`MeterFill` exposing `--percent`; `DateInput` with
    `part` rendering `slot="start"`/`"end"`; `CalendarCellButton` children as a function of the cell state (range
    calendar: selection start/end, hovered, pressed, the formatted day); a `ToastQueue` `wrap_update` hook (view
    transitions); the table's selection checkbox and `GridListItem`'s as `Checkbox` atoms; links in list
    box/menu/grid list/tag items (`data-href` rules dropped).
  - Decide: `ColorPicker` renders no element (the theme's `.color-picker` trigger is the app's class, and a
    `leptonic-Button` trigger brings button looks it doesn't undo); calendar buttons and the toast close button are
    quiet only with `attr:data-variant="quiet"` (the starter's components set it), maybe quiet by default in the
    theme; the `Disclosure` trigger is a `Button` (the theme undoes button looks).
  - Not ported (no atoms yet): `CommandPalette`, `DropZone`, `NavigationTree`, `SegmentedControl`, `Sheet`,
    `TokenField`, `Tree`. The component stylesheets go with step 4.
  - What becomes of each component (decided 2026-10-07):
  - covered by atoms already, the component goes: `Button`, `Link`/`LinkButton` (atoms `Link`, `AnchorLink`,
    `LinkButton`), `Checkbox`, `Radio`, `Switch`, `TextField`, `NumberField`, `Slider`, `Select` (incl. multiple
    selection: `SelectMode`), `Tabs`/`Tab`, `Table`, `Meter`, `ProgressBar`, `Separator`, `Modal`, `Popover`,
    `DateSelector`/`DatePicker` (Calendar/DatePicker atoms), `ColorPicker`, `Collapsible` (`Disclosure`),
    `Toasts`/`ToastRoot` (`ToastRegion` + `Toast` atoms on a `ToastQueue`), `Kbd`/`KbdShortcut` (`ShortcutKeys`
    for a `Shortcut` in the platform's form, `Keys` for literal keys);
  - layout and decoration only, gone (plain markup + CSS; the book shows recipes): `Card`, `Stack`, `Grid`, `Tile`,
    `Skeleton`, `Typography`, `AppBar` (a `<header>`), `Drawer` (modal atoms + CSS), `Alert` (`role="alert"`
    markup), `Icon` (use `leptos_icons`), the transitions (atoms expose `data-entering`/`data-exiting` like RAC;
    animate with CSS);
  - `Root`: gone; `ThemeProvider` + `ToastRegion` cover it, `--leptonic-vh` is replaced by `dvh` units;
  - `Code`: gone; `utils::syntax_highlight` (feature `syntax-highlight`) and the clipboard util stay;
  - `Chip`: gone; the `TagGroup`/`Tag` atoms replace it (done 2026-10-07);
  - `TiptapEditor` (feature `tiptap`) and `SanitizedHtml` (feature `sanitize`): gone with their features (use
    `leptos-tiptap` / `ammonia` directly).
- [ ] 3. The book moves off `leptonic::components` (509 files: its shell, kit and every demo) onto atoms + the
  atom theme or its own styles; component tabs become styling sections of the atom pages (book session).
- [ ] 4. Delete `leptonic/src/components/`, the feature `components` (and features only it needed), the component
  stylesheets, the component fixtures/browser tests in `testing/` and `leptonic/tests/`; drop the component items in
  this plan (themed `Breadcrumbs`/`ToggleButton`/`DateRangePicker`, the legacy `TableHeaderCell`, `Select`
  search styling, ...).

### Consumer requests
- [ ] Virtualizer, rest (see history: `ListBox` and `VirtualList` done 2026-10-07): virtualized `GridList`, `Table`,
  `ComboBox`/`Select` popovers and menus (RAC's virtualized `GridList`/`Table`/`ComboBox`/`VirtualizedMenu` tests;
  no consumer asks yet; dev-ui's collections are all short), sections/headers in virtualized lists,
  `GridLayout`/`TableLayout`/`WaterfallLayout`, drop targets. Check `VirtualList` with dev-ui's load (20,000 lines,
  10–100 appended per second): the collection is rebuilt per append.
- [ ] Tree tables (agnite dev-ui, 2026-10-07): react-aria-components' expandable `Table` rows (`treeColumn`, nested
  rows, `expandedKeys` + `onExpandedChange`, ←/→ collapse/expand, `role="treegrid"`, `aria-level`/`aria-expanded`/
  `aria-setsize`/`aria-posinset`): `TableCollection` rows with child rows, `expanded_keys` + `set_expanded_keys` on
  `Table` (C4), an expander button atom for the tree column's cells.

### Bugs and review leftovers
- [ ] From crudkit (2026-10-05, not reproduced): in a `use_table_cell` cell with `CellFocusMode::Child`, Enter on a
  `Button` atom (not the cell's first child) first moves focus to the cell's first focusable child, then the
  button's press fires. Repro: a fixture row with two buttons in one cell.
- [ ] Review of 2026-10-07, open: toast focus tracking by key instead of index (a new toast above the focused one
  shifts it; as upstream); `use_table` replaces instead of merging `aria-describedby` (latent until the grid has a
  description); `use_landmark` tests for nested landmark order, the duplicate-role warnings, `LandmarkController`,
  browser tab/window toggling.
- [ ] Test gaps from the 2026-10-06 overlay/link review: `aria_hide_outside` MutationObserver cases (elements added
  outside, into hidden containers, inside a target, reparented, top-layer) and "unhide after reorder"; popover
  reopened without its dialog (containment reset); link hover/focus/press data attributes and Enter; `AnchorLink`
  (scroll, hash without history entry); disabled `LinkButton`; disclosure group `on_expanded_change`, nested groups,
  focus ring.
- [ ] `use_breadcrumbs`' doc says its props and `aria_label` belong to the navigation landmark, while the attrs type
  and the atom put them on the `<ol>` (upstream: `navProps` on the `<nav>`): check and align.
- [ ] Legacy `TableHeaderCell` always attaches `use_press`, so static table headers (the book's documentation tables)
  are press targets.
- [ ] `classes="a b"` compiles but panics during SSR (`ClassName::from(&'static str)` in leptos-classes, the user's
  crate): accept/split it or reject it at compile time.

- [ ] Atoms forward their element (react-aria-components forwards `ref` everywhere): a `node_ref` prop on every atom
  rendering an element (done for `Input`/`TextArea`, 2026-10-07).

### Families
- [ ] **Text inputs and fields:** re-port `NumberParser` fully from `@internationalized/number` (other numbering
  systems when pasting, literal stripping from formatted parts, unit plurals, accounting sign, fr-FR/Swiss group
  characters, percent rounding) with a formatter that knows currencies/units/percent patterns from CLDR
  (`format_percent` appends `%`; German expects `50 %`); `NumberFormatOptions`' currency and unit are strings (typed
  values instead); re-sync `number_formatter.rs` (`@ 6f664fe911`); the `Select` search still styles through the
  legacy `leptonic-input` classes (`input.scss`).
- [ ] **Link, button, breadcrumbs, toolbar:** a themed `Breadcrumbs` and a themed `ToggleButton` component; the
  breadcrumbs' localized default label (localized strings).
- [ ] **Calendar and date picker:**
  - calendar test gaps: `pageBehavior: single` and the 2-week view, held arrow keys, announcements, `weeks_in_month`,
    commit behaviors `Clear`/`Reset` and on focus leaving;
  - the format options as signals (C11: `hour_cycle`, `granularity`, `hide_time_zone`, `should_force_leading_zeros`,
    `placeholder_value`, `max_granularity`); several names per validation state (react-aria `name: string |
    string[]`: server errors under the range's end name);
  - date picker test gaps: close on select `false`, the range's placeholder time on closing, a disabled picker,
    required/FieldError for pickers and time fields, segment hover/focus-visible;
  - range formatting with shared fields ("June 1 – 15, 2024": ICU4X has none yet); a themed `DateRangePicker`;
  - `DateTimeFormatter`'s English fallback narrow names still cut the first character.
- [ ] **Collections:** tree expansion and tree input (keyboard delegate, Tab navigation, select on press up);
  focus-mode enum names; tag group options; menu item links; per-item `on_action`; combobox item actions/links,
  `aria-labelledby` fallback, missing props (`should_focus_wrap`, `on_open_change`, ...); tab links; grid list
  selection checkbox (labelled by its row); `SelectionManager` cell selection/layout ranges; load more; DnD on the
  collection atoms + drag preview + tree drops; Tree atoms; Autocomplete (with upstream's
  `getPointerType` from `useFocusVisible`, which only `useAutocomplete` reads), then searchable Select/Multiselect
  (search keeps focus, collection unfiltered for the trigger text); legacy `components::select` issues; combo box
  label click shows no focus ring (check upstream first); `atoms::prelude` with `Grid*`/`Table*` once the legacy
  names are gone; table resizing leftovers (cursor overlay while mouse-resizing, resizer `data-focused`/
  `data-focus-visible`/`data-hovered` via focus ring + hover, scrollable ancestor, empty tables).

- [ ] `utils::syntax_highlight`: syntect's default syntaxes lack TOML (the book's TOML blocks stay plain, 2026-10-07):
  bundle a TOML `.sublime-syntax` (license check) or use `two-face`'s extra syntaxes (heavier); decide by binary size.

### Conventions still to apply
- [ ] C2 text types within the families (the `&'static str`/`Oco`/`Cow` text fields in the form, slider and color
  hooks; breadcrumbs' `label`, `use_anchor_link` and the `Icon` component's `Option<Oco>` label; `Dialog`'s
  `aria_label`).
- [ ] C12: the remaining string ARIA props and string tabindex (date field/picker, spin button, number field, date
  segment, breadcrumb item).
- [ ] Generics sweep (user, 2026-10-05): typed values instead of the dynamic collection `Key` for value-like
  selections (RadioGroup, Select, ComboBox, ToggleButtonGroup, CheckboxGroup values), slider/spin button/meter/
  progress values (generic numeric like C15), color channel values. Decide per family.
- [ ] Optional `Callback` props with generic arguments (`on_selection_change: Option<Callback<HashSet<Key>>>`) can't
  infer an untyped closure in `view!` (E0282).

### Cross-cutting
- [ ] Localized strings: a `use_localized_string_formatter` equivalent with upstream's message bundles (~36 hooks
  hard-code English), `useDefaultLocale` (browser language + `languagechange`), reactive `I18nProvider`. crudkit
  (German) overrides labels through the inputs meanwhile; keep that working.
- [ ] Theme flash: the server renders `Root`'s default theme, the stored choice applies one frame after hydration
  (`signal_ls`). dev-ui solved it with a cookie (leptos-use `use_cookie_with_options`, its `axum` feature on the
  server) feeding `ThemeProvider`'s `theme`/`set_theme`, and `<html data-theme>` rendered on the server through
  leptos_meta's `<Html>` (2026-10-07). For `Root`/the docs: recommend a cookie in `signal_ls`'s and
  `ThemeProvider`'s docs; let `ThemeProvider` render `<html data-theme>` on the server itself (optionally via
  leptos_meta), so apps need no own `<Html>`.
- [ ] Atom hygiene: `data-hovered` everywhere (`use_option` must return `is_hovered`), `data-focus-visible` on
  Tab/GridRow/TableRow/ComboBoxButton, context for render state (selected/pressed) in Tab/GridRow/TableRow/
  GridListItem, no `leptos-*` classes in atoms, complete `atoms::prelude`, one rule for `Children` vs
  `ChildrenFn`, atoms dropping `attr:` attributes.
- [ ] Port hooks onto `use_keyboard` shortcuts (upstream uses them in ~20 hooks).
- [ ] Input composition: which `MergeWith` types become unnecessary (e.g. link + press, tooltip trigger)?
- [ ] Theme: the global `[data-focus-visible]` outline also hits virtually focused listbox options.
- [ ] `use_clipboard` lives in `hooks::dnd`: move it to its own module?
- [ ] Later: step list, token field, preview trigger, `S/data` list helpers.
- [ ] Hygiene: 101 files on upstream commit 6f664fe911 and 3 others to re-sync (`scripts/upstream-drift.sh`);
  standard deviation blocks (ad-hoc formats, files without any, wrong "no deviations" claims, reasons for every API
  DIFFERENCES entry, "no upstream" headers for leptonic-only hooks), missing `// Upstream:` headers on ported utils,
  the legacy "mostly based on work in" lines; split the largest functions (`use_press`, `use_move`,
  `use_selectable_collection`, `FocusScope`); `cfg_attr(ssr, allow(dead_code))` → precise `cfg`s; docs for public
  items (undocumented pub fields of `UseFocusInput`, DnD states, `SelectionOptions.disabled_behavior`,
  `UseListBoxInput`/`Return`, `CollectionOptions`, `SelectState`, `Column`, `TableColumnResizeState.table_state`,
  `TabListState.list`, `TreeState.expansion`, `UseSpinButtonInput`, `UseGridCellReturn`, `UseNumberFieldLabelProps`)
  and most atom/component props. Run the book's `api_check` after API changes.

### Testing infrastructure
- [ ] Every hook/atom family gets a fixture + browser test from upstream's tests as part of its package (~78 hooks
  and 14 atom families had none in the audit; §2.6). Browser tests without `// Upstream:` headers get one.
- [ ] Native test helper for hooks (`with_owner`) and a way to run Effects natively, for `*_state` hooks.
- [ ] Extend the hydration id test to every fixture that generates ids.
- [ ] Clippy in release too: some lints depend on type sizes that differ in release (`MaybeProp`, `StoredValue`:
  `trivially_copy_pass_by_ref` fires only there). `just clippy` checks debug builds only; add a release run once the
  user decides on the cost.
- [ ] Chrome profiles leak (browser-test crate, the user's): chromedriver's `/tmp/org.chromium.Chromium.scoped_dir.*`
  profiles stay behind when a session isn't quit cleanly; 2026-10-06 they filled the /tmp quota (14 GB). Fix in
  browser-test: an own `--user-data-dir` per session, removed on drop, or a sweep at startup.

## Book

Page and writing rules: `documentation/documentation-strategy.md` (incl. "Verification"); look, design tokens and
which leptonic piece to use: `examples/book-ssr/STYLE_GUIDE.md`. Library gaps the book hits go into the roadmap above
and, while the book waits for them, under "Waiting on the library" below. Finished book work:
`documentation/history.md` ("Book").

### Next
- [ ] Off the components layer (the user's decision 2026-10-07, step 3 of "Remove the components layer"):
  - A: done 2026-10-07 (shell, `Root` → `ThemeProvider` + the book's toast queue and `ToastRegion`, kit `Icon`/`Link`/
    `Keys`/`DocTable`, pages import the kit). Left: the kit's `Code` once `utils::syntax_highlight` is public and the
    `syntax-highlight` feature no longer needs `components` (then drop the `Code`/`Language` re-export in
    `kit/mod.rs`); remove `ComponentDemoContexts` in `app.rs` (provides `ToastRoot`/`Leptonic` for the component
    demos) and the welcome `Showcase`'s components with phase B.
  - B (the user, 2026-10-07: "The book should use its own unique styling. I think that even maintaining a
    stylesheet is too much for leptonic."): the book never uses a library theme; it styles every atom itself with
    its tokens, selecting on the atoms' default classes `leptonic-<AtomName>`, its own classes and the data
    attributes. Stages: B0 one agent — Component tabs out of nav/routes (redirects to the atom pages' styling
    sections), component pages and demos deleted, the standard demo-control markup (atoms + book CSS) in STYLE_GUIDE
    §5; B1 four agents by sidebar groups — every atom page's styling section (default class, data attributes, inner
    markup, the book's CSS as the example), all demos on atoms + book CSS, Quick Starts, recipes for component-only
    concepts (Card, Stack, Grid, Skeleton, Typography, AppBar, Drawer, Alert, Icon, transitions; Chip → TagGroup
    atoms), group stylesheets; at the end `leptonic-themes` leaves `style/main.scss` and `ComponentDemoContexts`
    leaves `app.rs`. Replacements decided by the library: Button, Link/LinkButton, Checkbox, Radio, Switch,
    TextField, NumberField, Slider, Select (`SelectMode` multiple), Tabs, Table, Meter, ProgressBar, Separator, Modal,
    Popover, Calendar/DatePicker, ColorPicker, Collapsible → Disclosure, toasts → `ToastRegion`/`Toast`, Kbd →
    `ShortcutKeys`/`Keys`; gone with their features: Root, Code, TiptapEditor, SanitizedHtml. The library keeps its
    optional atom theme (`@use "leptonic/leptonic-atoms";`, the user's decision): the styling sections mention it in
    one sentence for apps that don't want to style from scratch; the Themes guide explains it (stage C).
  - C: `documentation-strategy.md`, `STYLE_GUIDE.md` (also its new tokens `--book-z-app-bar`, `--book-z-modal`,
    `--book-app-bar-*`, `--book-switch-*` and the `key-cap` mixin), the guides (Overview, Architecture, Installation, Themes,
    Classes & Styles) and the sidebar markers (H/A/C → H/A) drop the component layer; changelog.
- [ ] The kit's `Keys` draws group descriptions inside combinations ("Shift + Arrow keys") as key caps: compose them
  as key caps for the keys and plain text for the description (the library's advice; `KeyboardKey::Other` is a key).
- [ ] Browser checks for the book's own code blocks and key caps (styles, copy button).
- [ ] Browser test speed: implicit wait 0 with explicit waits only?; book page loads hydrate debug wasm (~900 ms
  each), a release/wasm-opt build for the browser tests would cut that. Step timing: `BROWSER_TEST_LOG_STEPS=1`.
- [ ] The Dockerfile builds from the repository root (path dependency on `../../leptonic`); not yet test-built.

### Waiting on the library
- TOML code blocks stay unhighlighted until the library bundles a TOML syntax.
- The search trigger (`doc_search.rs`) works out the platform itself for its `KbdShortcut` and `aria-keyshortcuts`:
  once the themed `KbdShortcut` takes a `Shortcut` (or `ShortcutKeys` gets the theme's look) and `Shortcut` gives an
  `aria-keyshortcuts` value, use them.
- Search results → Autocomplete with arrow keys through results (R3j).
- The `DateSelector`'s English labels ("choose a year", "Previous years") and the calendar strings wait for
  localized strings (R4).
- Document once implemented: tree tables; the `Label` atom (renders a `for` pointing at a generated id nothing has,
  R3c).

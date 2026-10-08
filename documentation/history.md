# Leptonic: history

Finished work, moved out of `PLAN.md` (which holds open work only). Most recent first within each part; git
history has the details.

## Library: observations read as sentences (2026-10-08, evening)

- The polling macros (`wait_for!`, `wait_until!`, `stays!`, `stays_for!`) are replaced by a builder in
  `tests/polling/mod.rs`: `wait_for("the red value").observing(|| number(&red)).to_be("64 (±1)", |red| ..)`,
  `.to_be_equal_to(v)`; `expect("the press log").observing(|| log.inner_text()).to_stay_equal_to("")`,
  `.for_at_least(d)` for real timers. Observations are `Fn() -> Future` closures (`async ||` closures still hit the
  compiler's higher-ranked `Send` limitation in `#[async_trait]` bodies).
- The ~50 former `wait_until!` sites observe the value their condition is about (distances, rectangles, the focused
  option's text, warnings, the rendered lines), so a failure shows the last value instead of "false". JavaScript
  predicates return the values they compare.

## Library: consistent browser test API (2026-10-08, evening)

- Lookups take a `Locator` (`"css"`, `role("option")`, `.text("Apple")` matching text content, `xpath(..)` for
  relations) and exist on pages and elements alike: `element` (waits for its element), `elements`/`count`/
  `inner_texts` (read now), `wait_for_count`, `count_stays`. Replaced `css`, `by_role_and_text`, `all`, `texts` and
  every thirtyfour `query`/`find` in tests.
- Every state has read / `wait_for_*` / `*_stays`: attribute, property (new `prop_stays`), inner text (`inner_text`,
  `wait_for_inner_text`, `inner_text_stays`), count (new `count_stays`), focus (`focused_element`, `wait_for_focus`,
  `focus_stays`). Renamed: `PageActions`/`ElementActions` (were `BaseActions`/`ElementExt`), `virtual_input`,
  `referenced_text`, `blur_focused`, form `reset`; `parse_text` removed. `Diagnostics`: `panics`,
  `uncaught_errors`, `console_errors`, `console_warnings` (the test-app's lists named alike).
- A lookup's text filter skips candidates the page replaces while it reads them (a re-rendered virtualized list),
  instead of failing. The calendar's `enter` helper waits for the focus on the tabbable date instead of returning
  silently. The failure probe (`tests/failure_probe.rs`) is removed: browser-test's own tests cover the reports.

## Library: one way per check, failure reports (2026-10-08, afternoon)

- Browser test helpers consolidated to one way per check: find an element (`page.css`, `page.by_role_and_text`,
  `element.query`), then its one method: `wait_for_attr`/`attr_stays`, `wait_for_text`/`text_stays` (text is
  `innerText`, trimmed, everywhere), `wait_for_prop`, focus by identity (`page.wait_for_focus_on`, `focus_stays`),
  `page.wait_for_count`. Removed: id shortcuts (`element`, `click_element_with_id`, `read_text_of`, ...), four other
  focus waits, three other text waits, `press_tab`. New: `SyntheticEvent` + `element.dispatch` (replaces ~20 event
  scripts), `page.eval` (the one way to run a script, naming it on failure), `client_rect`, `scroll_to_top`, form
  actions, `page.diagnostics()`. `ElementExt` moved to `pages/element.rs`; eight trivial page objects removed.
- Every `run` lists its cases with `cases!` (each a named step); inline logic in ~25 `run` bodies became documented
  cases. Checks strengthened throughout: full values over `contains`/`is_some`, single reads after interactions became
  waits or stays checks, exact validation messages and accessible names.
- The test-app records Rust panics, uncaught errors, `console.error` and `console.warn` separately (before, other
  console errors were dropped); a test fails on the first three.
- Failure reports, in browser-test (the user's checkout): every error carries the test-code frames that led to it
  (also through helpers and steps), panics their location and frames, a failing test its last steps with timing;
  WebDriver errors show their message only, messages print unquoted, browser-test's internal wrappers and locations
  are gone. README section "Failure Reports", regression tests there. Full suite: 126/126.

## Library: browser test style and browser-test 0.6 (2026-10-08)

- The browser tests use thirtyfour's own waiting: zero implicit wait, lookups through `driver.query(..)` /
  `element.query(..)` and element waits through `element.wait_until()`, both polling with `polling::
  element_query_wait()` (10 s, 50 ms). The hand-rolled poll loops of `pages/mod.rs` and the tests are gone; the
  `wait_for!`/`wait_until!`/`stays!` macros remain for observations thirtyfour can't wait on, and timeouts report
  the last value seen and the element (`ElementExt::describe`).
- Shared helpers instead of per-file copies (22 copies of `attr`, 7 of `wait_for_value`, 7 virtual clicks, 9 of
  `hover`, ...): `ElementExt` on `WebElement` (`wait_for_attr`/`prop`/`value`, `attr_stays`, `virtual_click`,
  `hover`, `texts_of`, `is_valid`, `set_value_from_script`, `rendered_text`, `describe`) and new `BaseActions`
  helpers (`text_stays`, `focus_stays`, `texts_of_all`, `referenced_texts`, `check_validity`, `form_values`,
  `hold_key`, `blur_active`, `read_parsed::<T>`). Scripts duplicating thirtyfour (`focus`, `scrollIntoView`,
  `value`, `activeElement`, `location`) replaced; script values passed as `arguments[n]`.
- assertr used for what it offers: `get_some().is_equal_to(..)` instead of `Some("x".to_owned())`, no assertions on
  `bool`-wrapped comparisons (`contains`, `contains_exactly`, `is_close_to`, `is_not_blank`, ...).
- Every file in one layout (imports, `run`, one documented function per case ending with `Ok(())`); long `run`
  bodies split into cases. Racy reads right after interactions became waits, single-read negative checks became
  `stays!` checks, and the fixed sleeps went down from ~75 to 7 real timers (type-ahead reset, long press, toast,
  pointer-move gaps), each commented. Style guide: `documentation/browser-tests.md`.
- browser-test 0.6 (the user's checkout): per-session Chrome profiles in `<target>/tmp/browser-test-profiles`
  instead of chromedriver's leaking `/tmp/org.chromium.Chromium.scoped_dir.*` (`/tmp` is a RAM disk here),
  cancellation on Ctrl-C, `rustls-no-provider` with `ring` instead of `aws-lc-rs`. Fixed in browser-test on the
  way: on a profile it doesn't create itself, chromedriver starts the page without focus (`document.hasFocus()`
  false, so script focus fires no `focus` events; 2 tests failed); the runner now brings every session's page to
  the front (`Page.bringToFront`), with a fail-first regression test there. Full suite: 126/126.

## Book: tree tables, typed selects, snippet review (2026-10-08)

- Table atoms: "Tree Tables" section (collection with child rows, `tree_column`, flat rendering with `hidden`, expanded
  state, keys; collapsing moves focus from a child row to its row, type-ahead searches shown rows, leaf rows are never
  expanded), `TableExpandButton` section, file-browser demo (`table_tree.rs`), and the styling pitfall: collapsed rows
  are `<tr hidden>`, so positional selectors need `:nth-child(2n of :not([hidden]))`, and row `display` or an
  `all: unset` button must not undo `[hidden]`. The book's own table CSS uses no positional row selectors; demo tables
  align with `start`/`end` instead of `left`/`right`.
- Select/ComboBox take their value's shape as their type (`Option<V>`/`Vec<V>`, no `selection_mode`): prop tables,
  intros, `ComboBoxValue<S>`, changelog, Collection State; new multiple-select demo (`select_multiple.rs`, an enum
  via `selection_value!`). Collection State: duplicate `selection_value!` keys are a compile error, keys of no value
  and `ListBoxItem` keys missing from the collection warn in debug builds.
- `use_color_picker_state` page: the untested sample became a live demo (`color_picker_state.rs`: area and hue slider
  bound to one picker color, a swatch reading it, Reset setting it from code).
- Static snippet review of 30 pages against the library (all guides included): fixed `use_menu`'s
  `id: Some(menu_props.id)` (a `String`, not a signal) and the number parser (integers saturate, `None` only for
  invalid text, fractions in integer types, or beyond a float's range).

## Book dependencies (2026-10-07, evening)

- Every dependency of the book declared with `default-features = false` and only the features its code uses
  (`leptos-use`: 7 functions instead of ~80; `tower-http`: the compression features instead of `full`, no zstd;
  `syn`: `full`, `parsing`, `clone-impls`; `leptos-use/ssr` in the book's `ssr` feature). Breaking upgrades one at a
  time: `scraper` 0.27, `tower-http` 0.7 (`leptos_axum` still pulls 0.6), `syn` 3 (`Type::FnPtr`, `NamedArg` in
  `api_check`). `markup5ever_rcdom` stays at `htmd`'s 0.38 (its handlers get rcdom nodes; `htmd` re-exports `Node`
  but not `NodeData`, which the demo and key handlers match on).
- Removed: `itertools`, `leptos-styles`, `strum`, `rootcause` (unused); `ringbuf` (the event logs are a `VecDeque`,
  newest first, so readers can copy the demos with std only); `ordered-float` (the slider marks demo keys by
  `f64::to_bits`). `reqwest` stays for the client's downloads ("Copy as Markdown"; the user: Rust only, no `fetch`
  through web-sys).
- TLS through `ring` instead of `aws-lc-rs` (`axum-server`'s `tls-rustls-no-provider`, `rustls` with `ring`,
  installed in `main`): about 23 s less CPU per fresh build of the server.
- ICU4X data regenerated for 2.3 (the 2.2 data broke every book build: "cannot find `provider` in `locale`").
  `scripts/icu-datagen.sh` is a version check plus one `icu4x-datagen --markers all --segmenter-models none` call
  instead of collecting markers from the data crates (6.2 MB of data instead of 5.8 MB, no `unstable` retry loop).
- `copy_as_markdown_downloads_only_on_press_and_once` grants clipboard access and checks the copied export (it
  accepted "Copy failed" before), and waits for the navigation's smooth scroll to the top before pressing (a press
  during it hit the button above the viewport).

## Book: fidelity follow-up (2026-10-07, evening)

- `kit::api_check` also compares each row's `ty` with the field's or prop's type (`api_types_match_the_library`;
  last path segments, a type parameter of the item may be described freely). It found 23 "see DateField"-style
  placeholder types on the date picker pages (now real types, one row per prop) and `UseSelectableItemInput::
  on_action` becoming `Signal<Option<Callback<()>>>`. The page texts, samples, tables and defaults of the fidelity
  wave's API list were otherwise already current.
- `TabPanels` got a demo (panels of different heights, height transition on `--tab-panel-height`); a tab disabled
  through `Tab::is_disabled` is documented as never the default selection.
- Collator page: the example uses `use_collator` (a collator per locale, not per read); `use_collator` and
  `use_filter` documented.
- The kit's `Keys` renders key group descriptions in combinations as text ("Shift + Arrow keys": a Shift key cap,
  then "+ Arrow keys").
- Browser tests: the Ctrl+K tests use the platform's primary modifier (Meta+K on Apple devices, as leptonic's
  `is_apple_device`); new `code_block_copy_button_copies_the_code` and `keys_are_key_caps_and_descriptions_are_text`.
  The full suite passes (31 tests), incl. both search tests.
- Visual pass (light, dark, 390px; one page per concept group and building-block area, headless screenshots through
  chromedriver): prose in articles gets `line-height: 1.6` (inline code chips covered the descenders of the line
  above: "sort_descriptor" lost its underscore); code has no ligatures and no automatic hyphenation; default values
  wrap only between words; a code block's text ends left of its copy button (it covered code on phones); the table of
  contents breaks identifiers between words (`<wbr>` after `_` and inside camel case); the event logs and the
  virtualized log used `--typography-code-*` tokens left over from the removed library theme (no background):
  now book tokens. Page descriptions and search text read keys by name ("Escape", not "EscEscape").
- Not a bug after all: `TabPanels` seemed to paint one frame at the new panel's full height, but the sampling `rAF`
  ran before TabPanels' own; read after frame 0's rendering, the box has the old height and then animates.
- `ContextMenuTrigger` and `use_context_menu_target` documented (Menu Atoms: section, props, a demo with a context
  menu per row of a file list, checked in a browser: labelled by the row, the action names the file, focus returns),
  linked from the Menu overview and the Grid List Atoms.
- The Markdown index and the search also list `###` sections naming an item that no `##` names (the DnD collection
  hooks and the keyboard delegates were missing from `llm-index.md`).
- Link texts of layer pages: "Button Atom" (not "Atoms") in See Also lists; prose about an atom links its
  identifier ("the `ColorPicker` atom").
- Markdown export: code blocks are converted from their raw text; converting the highlighter's spans dropped the
  line breaks at their ends (comments and `}` lines were joined with the next line in every exported sample).
- Markdown export: links that already point to Markdown keep their path (the overview's link to the index was
  `/doc/llm-index.md.md`).
- Event Propagation guide: the table names every kind of event, as the library decided them: `use_move`'s events
  and drag and drop stop always, hover and focus events never.
- Book Dockerfile: `apt-get upgrade -y` (without `-y` it aborts in a non-interactive build), `set -eu` in its `RUN`
  scripts.
- Changelog ("Changed during development"): the fidelity wave's input structs, renames and removals, for apps that
  followed the development branch.
- Welcome page: the install command wraps on phones instead of hiding its end behind the copy button.
- SSR guide: the `ssr` feature no longer mentions the removed rich text editor.
- The single `Checkbox`, `Radio` and `Switch` atoms are gone from the book (the user's decision): 125 files use the
  `*Field` + `*Button` pairs (state props on the field, `classes` and children on the button), the atom pages document
  only the pairs (the Switch tab is now "Switch Atoms"), the changelog says so. Demos spell the pair out instead of a
  kit helper, as demo sources must compile standalone.
- Keyboard tables live on concept overviews only: the six concept hook pages that repeated them (button, search field,
  tag group, popover, tooltip, toolbar) link the overview instead; their extra facts moved there (Button: excluded from
  the tab order, context menu keys, shortcuts; Popover: when focus stays inside). Rule in documentation-strategy.md.
- New guide "Build Times & Bundle Size" (from build-performance.md's advice for users).
- `kit::demo_styles` test `every_custom_property_used_is_defined`: a `var(--x)` without fallback must be declared in
  the book's stylesheets or set by the book's or leptonic's code.
- The page checks visit each page once: the dark-theme checks at desktop width, then the phone-width check after
  resizing (`pages_fit_a_phone_screen` merged into `pages_load_cleanly_in_the_dark_theme_and_fit_a_phone`).
- The book enables leptonic's `intl-strings` feature (localized messages). New utility page `use_localized_strings`
  (the feature, fallback, a demo switching four families' messages with the locale, `for_locale`, the families); the
  pages no longer call the hooks' texts English-only; the color demos pass the locale to `color_name`/`hue_name`/
  `channel_name`, and the Color group says names follow the locale.
- Grid Hooks: `use_grid_selection_announcement` and `use_highlight_selection_description`.
- New demos for features without one: Button `is_pending` (a Save button with a spinner in an indeterminate
  `ProgressBar`), `CalendarMonthPicker`/`CalendarYearPicker` (as `Select` atoms in a birthday calendar's header),
  `GridListSection`/`GridListHeader`/`GridListItemDescription`.
- Headings break long identifiers between their words, like the table of contents (the merged page check caught
  `use_highlight_selection_description` overflowing a phone screen).
- The book's server no longer compresses the WebAssembly bundle on the fly: brotli on the 43 MB development bundle
  took 3.4 s of CPU per request, which delayed every reload and, with all test shards loading pages at once, made a
  shard time out waiting for hydration. Release builds serve it precompressed (`precompress.sh`). The browser suite
  went from 2m31s to 31 s (slowest hydration 0.7 s).
- Pages no longer describe fixed library gaps: merged press + hover props fire `on_double_press`, the Collator's
  `ignore_punctuation` works and `Case` ignores accents, shortcuts ignore Shift for characters without case.
- Tree tables (2026-10-08): Table Atoms has a "Tree Tables" section (collection with child rows, `tree_column`,
  flat rendering with hidden rows, expanded state, `--table-row-level` indentation) with a file-browser demo checked in
  a browser (light, dark, 390px; the button expands and relabels), `TableExpandButton` with its props and data
  attributes, and the rows' and cells' tree data attributes; the Table overview lists the treegrid semantics and the
  ArrowRight/ArrowLeft keys; Table Hooks explains rendering a tree table with the hooks.
- Typed selection values (the user's design, 2026-10-08): the tables of `RadioGroup`, `CheckboxGroup`, `Select`,
  `ComboBox` and `ToggleButtonGroup` (renamed props) show `V`; Collection State documents `SelectionValue` and
  `selection_value!` (replacing `ToKey`); the Radio and Select atom demos use enums (`Plan`, `Size`, whose keys the
  form submits); the other group atoms point to `SelectionValue`; `TableBody`'s optional children; changelog entry.
- No theme flash: the book keeps the reader's theme in a cookie (leptos-use `use_cookie_with_options`, its `axum`
  feature on the server) instead of local storage, and renders it on `<html>` with leptos_meta's `Html`, so pages
  arrive in the reader's theme. The Themes guide shows the pattern ("Remembering the Theme on the Server"), the SSR
  guide links it.
- Accuracy sweeps against the source: all 36 `#[prop(default = ..)]` and all hand-written `Default` field values match
  the Default columns; every `leptonic-<Atom>` class the book names exists; every data attribute the atoms' docs
  list is documented (added `data-focused`/`data-focus-visible` of the search field's clear button and the number
  field's stepper buttons, which aren't tab stops and keep the focus in the input when pressed).

## Continuous improvement (2026-10-07, evening)

- `use_table` merges the sort description into the grid's `aria-describedby` instead of replacing it (upstream
  `mergeProps`).
- RTL menus: a `MenuTrigger` inside an RTL `I18nProvider` opens by click and ArrowDown, ArrowLeft opens submenus and
  ArrowRight closes them (`submenu_tests`, `right_to_left`; the earlier report no longer reproduces).
- Nested modals restore focus to their opener inside the outer modal, for sibling modals (`overlay_tests`) and a
  modal nested in the outer one's markup (`dialog_tests`); the earlier report no longer reproduces.
- crudkit's report (Enter on a `Button` atom that isn't its `CellFocusMode::Child` cell's first child moving focus
  to the first child) doesn't reproduce: `table_navigation_tests` (`enter_on_a_button_that_is_not_the_first_child`:
  arrows, Enter, Space, click, click + Enter in a selectable table) guards it.
- Tree rows follow an item getting children (`aria-expanded`, toggling on press): `use_grid_list_item` reads
  `has_child_nodes` reactively, and `UseSelectableItemInput::on_action` is a `Signal<Option<Callback<()>>>` (an item
  can gain or lose its action, as upstream's per-render `onAction`).
- Item labels follow the collection (an item relabelled in place): the `aria-label` of options, menu items, grid
  list rows and list box/menu sections, and a menu item's role (follows the selection mode), were read once at
  creation (`use_node_aria_label`; `listbox_disabled_and_empty_tests`).
- `just clippy` also checks the library in release (the user, 2026-10-07); clean.
- Tests: fixed sleeps and hand-rolled poll loops in the date picker, calendar, button and toast tests replaced by
  `wait_for!`/`stays!`/the new `stays_for!` (a negative check over a window, e.g. a toast's timeout).
- The single `Checkbox`, `Radio` and `Switch` atoms are removed (the user, 2026-10-07; react-aria-components
  deprecates them): a checkbox is a `CheckboxField` with its `CheckboxButton` (same for radios and switches). The
  test-app and the book (book agent) migrated; the `*Button` parts warn and render nothing outside their field
  instead of panicking; the atom theme styles `.leptonic-CheckboxButton`/`SwitchButton`/`RadioButton` and the
  fields (upstream's starter styles had moved there).
- Localized strings (`utils::intl_strings`, feature `intl-strings`, default on): react-aria's 21 message bundles in
  34 locales, converted by `scripts/port-intl-strings.py` into typed per-family structs (`TableStrings::
  ascending_sort(&column)`), formatted at runtime (ICU plural/select, apostrophes; a test parses every message of
  every locale and checks its arguments, which found upstream's sr-SP `{veza}`). Every family with hooks uses
  them: table, grid, tree, DnD (the drag manager gets the strings when a drag starts, as upstream), calendar, date
  field/picker (segment names too: upstream's `useDisplayNames` fallback), date validation, color (channel and
  color names: `ColorValue::color_name`/`hue_name`/`channel_name` take a `&Locale`, as upstream's
  `getColorName(locale)`), combobox (labels, announcements), menu, number field, search field, spin button, tag,
  toast, breadcrumbs, dismiss button, select placeholder, color swatch picker. Browser checks: `table_tests`
  (de-DE → fr-FR), `localized_atom_tests` (de-DE), `date_picker_tests` (German segment names).
- Grid and grid list: selection announcements (`use_grid_selection_announcement`: "Drafts selected. 2 items
  selected.", "All items selected.") and the touch "highlight selection" description
  (`use_highlight_selection_description`), ported now that messages are localized (`grid_list_tests`, from
  `ListView.test.js`).
- Tree tables (agnite dev-ui's request, 2026-10-08; react-aria-components' `Treeble`): child rows via
  `ItemBuilder::children` (after the row's cells; `TableBuilder::rows` for recursive builders; the selection cell
  in every row), `UseTableStateInput.tree` (`TableTreeInput`: tree column + C4 expansion state, shared with trees
  via `use_tree_expansion`), the grid on the visible rows (`Collection::with_expanded` keeps collapsed rows'
  cells), `role="treegrid"`, rows with `aria-level`/`aria-posinset`/`aria-setsize`/`aria-expanded`, ←/→ expand,
  collapse, to the parent (mirrored in RTL), the `expand_button` input. Atoms: `Table`'s `tree_column`,
  `default_expanded_keys`, `expanded_keys` + `set_expanded_keys`, `on_expanded_change`; `TableExpandButton`; rows
  and cells with `data-expanded`/`data-has-child-items`/`data-level`, rows `--table-row-level`, tree column cells
  `data-tree-column`; rows under a collapsed row stay rendered but `hidden`. `Collection::cells` (a row's cells
  without its child rows) for the grid delegates. `table_tree_tests` mirror `Treeble.test.js` (structure, mouse,
  keyboard LTR/RTL, default and controlled expanded keys, flattened rows, cells, selection).
- The selection's shape as the type (the user's decision, 2026-10-08): `Select<S>`/`ComboBox<S>` with
  `S: SelectedValues` (sealed; `Option<V>` selects one value, `Vec<V>` several, in the order selected), replacing
  `selection_mode` + `Vec<V>` (where `vec![a, b]` with single selection compiled). `ComboBoxValue<T = Vec<Key>>`
  holds the typed value. Items keep taking a `Key` (the user's decision: no generic items; a key of no value of the
  group's type warns in debug builds), and a `ListBoxItem` whose key isn't in its collection warns too (the
  test-app's bound page size select had string item keys over integer collection keys after porting).
- Review of the 2026-10-08 work (one review agent), fixed the same day:
  - Tree tables: type-ahead walks the rows shown (it only searched top-level rows); a focused row collapsed out of
    view from outside (bound expanded keys) moves focus to its closest shown ancestor row (the grid's index-based
    refocus picked an unrelated row); leaf rows are never expanded (`data-expanded`, "Collapse"); the theme counts
    shown rows only (`:nth-child(2n of :not([hidden]))`, last-row rounding), hides `[hidden]` rows and expand
    buttons, and has upstream's tree indentation and chevron styles. Tests: type-ahead, ArrowLeft to the parent,
    outside collapse, leaf rows (browser), refocus to the ancestor (native).
  - `Collection::last_key` enters items' children only in a tree view, as `key_after`/`key_before`.
  - Localized strings: `#` inside a `select` nested in a `plural` is text (ICU, @formatjs); a plain `{count}` is
    formatted for the locale (ICU), documented as a deviation from upstream's compiled templates.
  - `scripts/port-intl-strings.py` finds a message's arguments with a parse mirroring the Rust one (the regex took a
    one-word option body for an argument), and a message with two arguments of one type takes a generated args
    struct with named fields (`InsertBetweenArgs { before_item_text, after_item_text }`, `DateRangeArgs`,
    `FocusAnnouncementArgs`, the color names: eight messages), so that an upstream reordering can't swap arguments
    at call sites that still compile.
  - Long collections (dev-ui's log): a node carries its document position (`compare_order`/`sorted_keys` read it),
    replacing a second key map and a full traversal per build. An append to a 20,000-row `VirtualList` (rebuild
    plus layout pass, native release) went from 14.3 ms to 11.6 ms (`timing_appends_to_a_long_list`, ignored test
    in `list_layout.rs`). The stale PLAN note that every layout pass clones the collection was wrong (it holds an
    `Arc`).
  - The grid and table keyboard delegates built an ICU `Collator` on every read (every key press): they use the
    per-locale `use_collator` now; the date segment uses `use_filter`.
  - Typed values: a key of no value of the group's type warns in debug builds; `selection_value!` rejects duplicate
    keys at compile time (`compile_fail` doctest).
  - Tests: one polling API (`wait_for!`, `wait_until!`, `stays!`, `stays_for!` in `tests/polling/mod.rs`, timed as
    steps); the closure helpers `wait_for_value`/`wait_until`/`assert_stays(_for)` are gone; ~25 fixed sleeps and
    eight hand-rolled poll loops replaced.
- Typed selection values (the user's choice, 2026-10-08): `RadioGroup`, `CheckboxGroup`, `Select`, `ComboBox` and
  `ToggleButtonGroup` are generic over `V: SelectionValue` (`to_key`/`from_key`; for `Key`, `String`, integers, and
  enums via `selection_value!`, which also implements `From<T> for Key` so items take the enum). The hooks stay
  key-based; `atoms/typed_values.rs` converts at the boundary. `ToKey` (unused) is replaced. `ToggleButtonGroup`'s
  state props are `value`/`set_value`/`default_value`/`on_change` like every value-like selection. A group without
  any typed prop names its type (`<RadioGroup<Key>>`; Leptos components can't default type parameters).
  `radio_group_tests` checks an enum group and its submitted key. Also: `TableBody` children optional; `Select`
  and `ComboBox` atoms got their `// Upstream:` headers and deviation blocks.
- `MergedPressHoverProps` spread no `dblclick` handler, so `on_double_press` never fired through merged press and
  hover props (the only merged type with a field missing from its attributes; `press_tests`). `CollatorSensitivity`
  maps as ECMA-402 does: `Case` is primary strength with the case level (was tertiary, telling accents apart too),
  `Variant` tertiary (was quaternary); unit test over a/á/A. Both found by the book agent.
- Safe triangle (`use_safely_mouse_to_submenu`): no hook bug. The earlier test's moves were too coarse (its second
  move already hovered the next item, which closes the submenu before the hook has two processed moves to judge the
  direction). `submenu_tests` (`safe_triangle`) moves in small steps 60 ms apart, opens by click (pointer modality)
  and fails with the hook disabled.
- Grid list child navigation mirrors all of upstream's "ArrowLeft/Right cycles through children and row element"
  (from the first child to the row and back in) and its RTL variant (`grid_list_child_navigation_tests`).
- RTL date picker arrow keys: not a library bug. The he-IL fixture had no `dir="rtl"` (`I18nProvider`, like
  react-aria's, renders none), so the button sat below the segments at the same `left`. Fixture laid out right to
  left; `date_picker_tests` now walks ArrowLeft by position to the button and back (upstream
  `DatePickerBase.test.js`).
- `combobox_multiple_tests` flaked (2 of 3 runs): the test blurred from JavaScript in the frame of the option press,
  while `prevent_focus` swallows blurs (as upstream); its `blur` waits a frame now (10/10; `lessons.md`).
- `virtual_list_tests` flaked (2 of 4 runs): the test scrolled before `selectionchange` was dispatched; it now waits
  for the event (5/5).
- Rustdoc is warning-free (`just doc`, `-D warnings`).
- Browser tests: `TEST_APP_TARGET_DIR` builds the test-app in its own target dir; CLAUDE.md's
  `[package.metadata.leptonic]` example corrected (`style-dir = "style"`).
- Native `*_state` tests (2026-10-08, 42 tests): `use_color_channel_field_state` (7, from `ColorField.test.js`'s
  "channel" cases), `use_color_picker_state` (5, `ColorPicker.test.js`), `use_color_slider_state` (7,
  `ColorSlider.test.tsx`: keyboard steps with `on_change_end`, disabled, drag, form reset, `getDisplayColor`),
  `use_form_validation_state` (10, upstream's implementation: source priority, native commit, reset, server
  errors, `merge_validation`), `use_text_field_state` (4, `useControlledState.test.tsx`), `use_virtualizer_state`
  (9: renders on every input, invalidations, viewport moves, layout delegate). Fix: the virtualizer's clock called
  `js_sys::Date::now()` in every non-`ssr` build, which panics natively; it is WebAssembly-only now.

## Fidelity review 2026-10-07

A read-only review of the whole library against react-spectrum @ 99e6102368 (nine passes: one per family plus
architecture/docs/tests), applied the same day by nine agents, one per family, failing test first where possible (the
user: "Improving our integration-test suite is of utmost importance"). Open leftovers are in `PLAN.md`. User
decisions: modals focus the dialog on open (`ModalContent` `auto_focus` defaults to false) and set no `aria-modal`
(as upstream); the `LinkButton` atom is removed (use `Link`); the `Propagation` rule covers only press and keyboard
events (as upstream's `continuePropagation`); the 2026-10-05 audit file is deleted (its open rest is in `PLAN.md`).

- Interactions and focus: `use_press` keyup in the capture phase and dragstart without a panic (leptonic-dd), a child
  stopping the click cancels the press, the macOS Meta keyup synthesis re-ported (Mac emulated in the test),
  `on_press_up` on keyup only for unrepeated keys inside the element, the long-press contextmenu blocker removed after
  release, drag out/in via `pointerenter`/`pointerleave` (`interaction_rect` gone), the `openLink.isOpening` guard;
  `use_move` re-ported (leptonic-dd: no axis filter, which fixed the sliders' cross-axis arrows), typed keys;
  `text_selection` on a weak map; `use_focus_visible` synced (window refocus, `invalid`, `get_pointer_type()`); the
  prevent-focus attribute (tree expand button, hidden select); `use_focus_within` capture listener; the
  `synthetic_blur` leak; SVG focus; FocusScope select-on-Tab and Tab outside the scope; scroll, platform, open_link,
  prevent_scroll synced. `upstream-drift.sh` reports no drift.
- Collections: `Filter` by characters (umlauts, accents), memoized collator/filter per locale, the combo box's custom
  value/revert and validation, `Collection::build` O(n), separators in filtered collections, `TagGroup` contexts,
  `HiddenSelect` beyond 300 options, link items under `LinkBehavior::Override`, combo box announcements and hidden
  form inputs, Select `is_open`/`set_open`, one-element `ListBoxSection` + `ListBoxSectionHeading`, `GridListSection`/
  `GridListHeader`/`GridListItemDescription`, per-item `disabled_behavior`, reactive `selection_behavior`,
  `data-hovered` on list box/grid list items and the select trigger.
- Grid, table, tree, virtualizer, DnD: nested column groups as upstream's `buildHeaderRows`, reactive table atoms,
  empty tables without keyboard navigation, PageUp/PageDown reaching the headers, `data-hovered`/`data-focus-visible`
  on grid and table atoms, tree `expanded_keys` binding, ArrowRight on expanded tree rows; `VirtualList` follow mode
  keeps measured sizes, RTL `scroll_to`, shared layout nodes; keyboard DnD hides with `inert`, registration effects
  untracked, disabled drags; DnD/clipboard browser tests with synthesized native drags.
- Overlays: modal decisions above, a `DismissButton` in dismissable modals, reset positions on reopen, popover `dir`,
  per-opening containment and measurements (closed overlays observe nothing), tooltip `should_skip_animation`, toast
  focus by key, one-element `MenuSection` with its own selection, `use_overlay`/landmark/`aria_hide_outside` fixtures
  and tests, DismissButton's upstream tests.
- Forms: `NumberParser` re-ported from `@internationalized/number` (numbering systems, literals, accounting, Swiss and
  Arabic separators; ~40 upstream tests + a round-trip matrix), `format_to_parts`, saturating integer values, controlled
  text field re-sync, server errors after an edit, native custom validity, `use_form_reset` re-synced, `checked` as the
  default for native form reset, `FormValidationState` (C3), `CheckboxField`/`RadioField`/`SwitchField` (+ `*Button`).
- Calendar and dates: the range calendar's touch tap, the server's today for SSR, reactive layout and format options,
  memoized label formatters, BC eras, `CalendarMonthPicker`/`CalendarYearPicker`, autofill (`use_hidden_date_input`),
  `DateTimeFormatter::format()` on ICU4X field sets, single-input hooks; calendar/date picker browser tests.
- Slider and color: color area/wheel inputs follow assistive technology, gradient layer order, slider index panic,
  restricted bound values, per-thumb labels, `ColorWheel` without a channel, localized channel values, `utils::color`
  split; slider/color browser tests.
- Button, link, tabs: `Button` pending state (RAC's tests), `ThemeProvider` contexts, modified clicks on anchor links,
  disabled hook anchors keep their role, `Tab` `is_disabled`, `TabPanels`, force-mounted panels as RAC, `Fraction` for
  meter/progress, reactive orientations; VisuallyHidden/Separator/Toolbar/Breadcrumbs/Theme tests.
- Docs and test infrastructure: `crate::testing::{with_owner, flush_effects}` runs Effects in native tests; polling
  helpers; the hydration id test discovers every fixture (4 shards); `just clippy` also checks the atoms-only set; the
  test-app no longer generates a theme; atoms/architecture/hooks docs rewritten to the code; `design-collections.md`
  marked implemented; the narrowed `Propagation` rule documented.
- The six browser tests still failing after the wave, fixed at their root causes (full suite before: 124 tests, 6
  failed): `table_selection_tests` (the item registry's duplicate-key check read a disposed `CapturedElement` after
  two cells swapped keys; the registry now keeps every live registration per key, latest wins), `tabs_tests` (the
  test expected no event for pressing the selected tab, but upstream's `useSingleSelectListState` reports reselection:
  expectation corrected) plus `TabPanels` measuring in the next frame, `hydration_id_tests` (a disabled first tab was
  skipped only in a client Effect; now the default skips disabled tabs while rendering, on the server too),
  `grid_list_child_navigation_tests` (`SelectionManager::set_focused_key`/`set_focused` notified on unchanged values;
  the deferred row focus re-checks focus; `test_grid`'s restore step now leaves the grid first, as upstream's browser
  behavior keeps focus on a cell focused from its own child), `forms_tests` (group items now keep their own
  validation behavior, as upstream; native validity is re-read when constraint attributes change, via a
  MutationObserver) and `combobox_multiple_tests` (not reproduced since).

### API changes of the fidelity review (old → new, as the agents reported them)

#### interactions
- UseTreeItemReturn +expand_button_attrs: PreventFocusAttr (book demos/tree.rs fixed)
- utils::focusability::{PREVENT_FOCUS_ATTRIBUTE, PreventFocusAttr, prevent_focus_attr}
- hooks::get_pointer_type()
- removed utils::{InteractionRect, RectPrecise, is_over}, hooks::PressEvents, focus_scope_tree::{get_active_scope, innermost_containing_ancestor}
- use_press touch-action on server
- ShortcutKeys/Keys dir=ltr
- platform::browser::{is_firefox,is_safari}.
#### collections
- use_list_keyboard_delegate(..4..)/_with -> (UseListKeyboardDelegateInput{state,element,orientation,layout,layout_delegate})
- UseComboBoxInput -is_read_only -name, is_required Signal, placeholder MaybeProp, +form_value ComboBoxFormValue, +form
- UseComboBoxReturn -label_on_click +form_values
- UseComboBoxStateInput.on_open_change Callback<ComboBoxOpenChange>
- ComboBox atom is_required Signal, placeholder MaybeProp, +on_open_change +form_value +form, hidden inputs
- UseSelectInput -name -validation_behavior, is_required Signal
- UseSelectStateInput +is_open Option<ValueBinding<bool>>
- UseHiddenSelectInput -name -validation_behavior, is_required Signal, +label
- UseHiddenSelectReturn +label +first_input_capture, required Signal
- Select atom is_required Signal, +is_open +set_open
- SelectValue placeholder MaybeProp default "Select an item", list join
- HiddenSelect +label
- ListBox +empty_state, panics w/o collection
- ListBoxSection -heading_classes -> <ListBoxSectionHeading>
- ListBoxItemCtx +is_hovered
- UseOptionReturn +is_hovered
- SelectionOptions.selection_behavior Signal
- atoms selection_behavior into Signal
- UseSelectableItemInput.focus Option<FocusItem>
- ItemBuilder::disabled_behavior, Node.disabled_behavior, SelectionManager::is_item_disabled
- use_filtered_list_state removed (book groups/collection_state.rs documents it)
- new atoms GridListSection, GridListHeader, GridListItemDescription
- GridListItem +focus_mode +allows_arrow_navigation
- utils::filter::{use_collator,use_filter}
- ComboBoxState name()/validation_behavior()/is_read_only_signal()
- SelectState name()/validation_behavior(). Book compile fixes: hooks/demos/{select,combobox,grid_list,dnd_reorder}.rs, atoms/demos/{listbox,select}.rs
- page text code samples still old.
#### grid
- use_table_header_placeholder(&data,&key) -> (UseTableHeaderPlaceholderInput{table,key})
- use_table_selection_checkbox(&t,k) -> (UseTableSelectionCheckboxInput{table,key})
- use_table_select_all_checkbox(&t) -> (UseTableSelectAllCheckboxInput{table})
- UseTableColumnResizeInput.aria_label MaybeProp
- GridState.is_keyboard_navigation_disabled RwSignal -> is_keyboard_navigation_disabled() Signal + set_keyboard_navigation_disabled(bool)
- UseGridCellProps role/aria_colspan/aria_colindex/colspan Signals + on_pointerdown
- UseTableRowProps.aria_labelledby Signal<String>
- UseTreeStateInput +expanded_keys Option<ValueBinding<HashSet<Key>>>
- UseTreeItemReturn -expand_button_label, has_child_items Signal<bool>
- Table disabled_behavior DisabledBehavior default Selection
- ColumnSizes public
- TableCell children ChildrenFn
- Table.should_select_on_press_up
- TableCell/GridCell allows_arrow_navigation
- data-hovered/focus-visible on grid/table atoms. Virtualizer: use_scroll_view(input,element) -> (UseScrollViewInput{element,..}) Signal options
- use_virtualizer_item(input,element) -> (UseVirtualizerItemInput{element,..}) update_item_size Callback<ItemSizeChange>
- state.visible/content_size -> visible()/content_size()
- ListLayoutOptions.drop_indicator_thickness removed. DnD: get_items Callback<()> -> items Signal<Vec<DragItem>>
- get_allowed_drop_operations -> allowed_drop_operations Option<Signal<..>>
- use_draggable_collection(state,element) -> (UseDraggableCollectionInput{..})
- drop item/indicator target Signal<DropTarget>
- on_key_down Callback<DropTargetKeyDownEvent>
- DragPreview.offset Option<Point>
- ListDropTargetDelegate::with_direction removed. Book compile fixes hooks/demos/{table,table_resizing,tree,virtualizer,dnd_*}.rs
- texts/API tables in hooks/{table,tree,grid,virtualizer,dnd}.rs + atoms/virtualizer.rs:172 stale.
#### overlays
- use_tooltip_trigger(input,state) -> (UseTooltipTriggerInput{state,is_disabled,trigger,should_close_on_press}) no Default
- return -is_open -trigger_id -tooltip_id (state.overlay.is_open, tooltip_props.id)
- trigger props -id
- UseTooltipTriggerTooltipProps -role
- UseTooltipInput -on_open -on_close, props +role
- UseOverlayTriggerInput{show, overlay_id:Oco} -> {is_open, overlay_id:String}
- UseOverlayReturn.id/UsePopoverReturn.id/UseModalBackdropReturn.id Oco->String
- UseMenuTriggerMenuProps.id Signal<String>->String (Clone not Copy)
- UseMenuItemProps.aria_expanded Signal<Option<AriaExpanded>>
- MenuSection props selection_mode, default_selected_keys, selection/set_selection, on_selection_change, disallow_empty_selection, should_close_on_select
- Menu.should_close_on_select
- MenuSection DOM <section>+<header>
- UseModalInput.is_disabled Signal
- UseModalProps.aria_modal Signal
- ModalContent contain_focus/restore_focus/auto_focus Signal, auto_focus default false, no aria-modal, DismissButton when dismissable
- ModalBackdrop no leptonic-modal-backdrop class
- UseDialogProps.tabindex i32
- DismissButton on_dismiss required + id
- use_landmark(input, element) -> UseLandmarkInput{element,..}
- use_toast(input,queue) -> UseToastInput{queue,..}
- use_toast_region(input,queue,element) -> UseToastRegionInput{queue,element,..} no Default
- UseToastContentProps.aria_hidden Signal<Option<AriaHidden>>
- Popover dir attr. Book compile fixes: hooks/demos/{tooltip_basic,tooltip_positioning,toast}.rs, five use_overlay_trigger demos, use_landmark calls in app.rs, hooks/demos/landmark.rs, groups/demos/layout_app_bar.rs, hooks/landmark.rs
- texts/indoc still old.
#### forms
- UseFormValidationStateReturn -> FormValidationState (.commit_validation()/reset_validation()/update_validation(result)
- native_validity_readers pub(crate))
- book API tables in hooks/{form,select,radio,text_field,combobox,date_field,date_picker,number_field,checkbox,color_field}.rs still name old type
- UseCheckboxGroupItemInput.validate removed -> options.validate
- use_formatted_text_field(element, validate, set) -> (UseFormattedTextFieldInput{element,state}) + trait FormattedTextState
- use_slot_id removed
- NumberFormatOptions + currency_sign, numbering_system
- NumberFormatter::{format_to_parts, numbering_system}
- NumberPart/NumberPartKind
- NumberParser::numbering_system(text)
- parser + and % rules
- UseToggleInputProps/UseRadioInputProps + default_checked
- spin button aria_disabled/readonly/required + number field group aria_disabled typed (AriaDisabled/AriaReadonly/AriaRequired)
- NumberValue::from_decimal saturates
- new atoms CheckboxField/CheckboxButton, RadioField/RadioButton, SwitchField/SwitchButton (prelude).
#### calendar
- use_calendar(input,state) -> UseCalendarInput{state,id,aria_label,aria_labelledby,aria_describedby,aria_details} no Default
- use_range_calendar -> UseRangeCalendarInput{state,commit_behavior,..}
- use_calendar_heading -> UseCalendarHeadingInput{state,offset,format}
- use_calendar_month_picker -> UseCalendarMonthPickerInput{state,format}
- use_calendar_year_picker -> UseCalendarYearPickerInput{state,visible_years:u8,format:CalendarYearPickerFormat{year,era}}
- picker aria_label Month/Year -> month/year
- Use(Range)CalendarStateInput visible_duration/page_behavior/selection_alignment/first_day_of_week(Signal<Option<Weekday>>)/weeks_in_month Signals
- CalendarState.visible_duration Signal
- is_date_unavailable Callback<DateAvailabilityQuery{date,anchor_date},bool>
- Calendar/RangeCalendar props reactive, first_day_of_week + weeks_in_month MaybeProp
- use_date_field(input,state,el,input_el) -> UseDateFieldInput{state,element,input_element,options:DateFieldOptions}
- use_time_field -> UseTimeFieldInput
- is_in_picker/focus_manager/open -> DateFieldOptions.picker: Option<DateFieldPicker{overlay,focus_manager}>
- use_date_picker_group(el,bool,open) -> UseDatePickerGroupInput{element,arrow_keys:GroupArrowKeys,overlay}
- use_date_picker(input,state,group) -> UseDatePickerInput{state,group,options:DatePickerOptions}
- use_date_range_picker -> UseDateRangePickerInput
- use_date_segment(seg,data,el) -> UseDateSegmentInput{segment,data,element}
- state format options Signals
- DatePickerState/DateRangePickerState granularity + has_time Signals
- DateField/TimeField/DatePicker/DateRangePicker placeholder_value/granularity/hour_cycle MaybeProp, hide_time_zone/should_force_leading_zeros Signal<bool>, +auto_complete
- DateTimeFormatOptions hour12 -> hour_cycle Option<HourCycle>, -time_zone (format_zoned), PartialEq
- HourCycle in utils::date_time_formatter (re-exported by datepicker)
- new exports CalendarMonthPicker, CalendarYearPicker, use_hidden_date_input. Book demo fixes: hooks/demos/{calendar_single,calendar_range,calendar_unavailable,date_field,time_field,date_picker}.rs.
#### slider-color
- get_channel_value/range/name/format_options, get_display_color, get_area_gradient -> without get_
- get_color_space_axes tuple -> color_space_axes -> ColorSpaceAxes
- format_channel_value(ch) -> (ch, &Locale)
- HSV.value/with_value -> brightness/with_brightness
- RGB8::from_hex -> parse::<RGB8>()
- into_rgb8/into_hsv removed (to_rgb8()/From)
- ColorWheel + UseColorWheelStateInput lose channel
- ColorSlider group on ColorSliderTrack
- SliderMark.name MaybeProp<String>
- ComputedSliderMark{percentage,value,name:Option<String>} + is_in_range()
- UseSliderInput.aria_details
- UseSliderThumbInput.aria_errormessage/aria_details
- Slider props is_required, aria_describedby, aria_details
- SliderThumb props is_required, is_invalid, aria_describedby, aria_errormessage, aria_details
- UseColorAreaInputProps/UseColorWheelInputProps on_change -> on_input
- color field state/hook generic.
#### widgets
- LinkButton atom removed (book uses Link in pages/welcome.rs AtomLink, err404.rs, atoms/demos/link_button.rs
- link/button pages, changelog, ApiTable need rewrite)
- UseTabReturn.tab_props->props
- UseTabPanelReturn.tab_panel_props->props
- UseTabPanelInput +aria_describedby +aria_details
- UseTabListInput/UseToolbarInput/UseSeparatorInput.orientation Signal<Orientation>
- atoms Tabs/Toolbar/Separator orientation into Signal
- Separator +id +aria_labelledby
- UseVisuallyHiddenInput.is_focusable Signal<bool> (atom too)
- UseProgressBarReturn.percentage Signal<Option<Fraction>> (.as_percent())
- Button +is_pending + callbacks + auto_focus + prevent_focus_on_press, data-pending
- use_button +is_pending, id/form attrs Option<String>
- ThemeProvider +classes, class leptonic-ThemeProvider
- Tab +is_disabled
- TabPanels atom
- TabPanel +aria_describedby/details
- utils::fraction::Fraction.
#### misc (coordinator)
- utils::visually_hidden::visually_hidden_fixed_styles() (new; used by use_hidden_date_input)
- use_move/use_press: see leptonic-dd's earlier message (use_move re-ported: no axis/constrained mode, UseMoveReturn { props })
#### later (the six failing tests)
- ToggleOptions.validation_behavior: ValidationBehavior (default Aria) -> Option<ValidationBehavior> (default None)


## Finished roadmap items

### R1. Bugs and crash risks (audit §2.1, §3, §1.2 "Bugs")
- [x] Done 2026-10-05: `AnchorLink` panic (typed, infallible `Href`); calendar `focus_month` (typed
  `time::Month`)/`focus_year`/year navigation unwraps; `handle_decimal_operation` (`DecimalOperation` enum); timers and
  deferred callbacks outliving their owner (tooltip warmup, drop activation, dialog iOS refocus, grid/grid-list
  frames via the new `utils::owner_alive::OwnerAlive`, DnD end/auto scroll); outside interactions no longer
  `preventDefault` the dismissing click; tooltip trigger as upstream (focus opens immediately, Escape stops
  propagation, any key down is a press start, touch never hovers); breadcrumb links activate natively; transient
  global listeners as drop guards (`utils::event_listeners::{listen, listen_to}`) in press, move, hover, focus
  within, slider, color wheel and spin button (no more effects per interaction); `run_after_transition` re-ported
  (shared, removable `transitioncancel` listener); `use_hover` global touch handling re-ported (ref-counted
  listener, 500 ms); `-webkit-user-select`; one `is_ctrl_key_pressed`; focus-visible notifies outside its lock;
  merged press props keep `on_dblclick`; open states report only changes; hydration-stable overlay ids, calendar
  weeks/days keyed by date; MutationObserver/`listen_once` closure leaks; dialog blur handler on `focusout`.
- [x] In their families (verified 2026-10-06): the form items (radio values/roving tabindex, no `Box::leak`, number
  field hidden input and `required`, switch on a real input, search field value, checkbox `checked`/`required`), slider
  input and thumb focus, controlled tooltip, macOS Meta keyup synthesis came with R3b/R3c/R3d/R3e/R3g; `preventFocus`
  re-ported 2026-10-06 (`utils::prevent_focus`: the focus moves back to the active element with its focus/blur events
  stopped, and focus-visible ignores them, instead of `preventDefault` on mouse down; browser check in `press_tests`);
  the legacy random ids went with their rebuilds.
- [x] Review 2026-10-06 (overlays, positioning, menu, separator), fixed: `OverlayArrow` on every open; `Show` around
  `Portal` (no empty portal container while closed); SSR `use_press` merges the responder's `force_is_pressed`;
  `use_overlay` releases the element it pushed; `use_overlay_position` (`max_height: Some(0.0)` is 0, initial scale,
  synchronous `PositionUpdater::update`, tracks the scroll element, boundary check across shadow roots via
  `utils::shadow_dom::node_contains`); a disabled responder disables the button itself (no keyboard opening, no
  long-press description); overlay content clears both trigger contexts (`ClearTriggerContexts`: a button in a modal
  opened by a `DialogTrigger` no longer toggles it); non-modal popovers with a dialog contain focus
  (`OverlayFocusContain`, reactive `FocusScope::contain`); `aria_hide_outside` walks like upstream's TreeWalker
  (cells of hidden rows, the root itself); `Tooltip` without `TooltipTrigger` warns; `atoms/dialog.rs` deviation
  block; overlay flags are signals and the outside filter an `InteractOutsideFilter` (C10/C11); menu keys typed.
  Browser tests: `test_aria_hide_outside.rs` (new), dialog, popover, overlay position.
- [x] Review 2026-10-06 (links, breadcrumbs, toolbar, disclosure, button, key sweep), fixed: a `Region` disclosure
  panel rendered two `role` attributes (the role is now a signal in the hook's panel props) and panel attrs were
  single-use; `Button`/`LinkButton` `data-disabled` from the merged disabled state (`UseButtonReturn::is_disabled`);
  a disabled `LinkButton` renders an `<a>` without `href`; a single-expansion disclosure group reduces keys
  deterministically (first default, else the smallest key: `HashSet` order differed between server and client);
  a popover's focus containment is per opening, modals provide their own; `AnchorLink` keeps `history.state`;
  no description element for an empty description; `use_button` takes `Vec<LinkRel>`/`LinkTarget`/`Signal<Option<
  String>>` href like links; `target` is a plain `LinkTarget` (default `Same`); `AnchorLink`'s `None` scroll
  behavior means "don't scroll"; toolbar labelling reactive. Tests added: toolbar vertical and RTL, disclosure
  region role (server HTML) and repeated Enter.
- [x] From agnite dev-ui (2026-10-06, at cf92a26), fixed: a modal's hide-outside left overlays opened from inside
  it inert when its observer ran before their registration (Leptos effects run after the observer's callback,
  React's layout effects before): `keep_visible` and a nested `aria_hide_outside` reveal the element again
  (browser check "late" in `aria_hide_outside_tests`, plus a ComboBox in a modal in `combobox_tests`); table column
  resizers added a duplicate `touch-action`.
- [x] `ComboBox` didn't follow a controlled value changed together with its collection when both derive from one app
  memo and a filter is set (agnite dev-ui, fixed 2026-10-06): its reconciliation effect read `filtered` before the
  value, and reactive_graph then skipped the run (see "Leptos bug report" under Waiting on the user). Upstream reads
  first; rule "Effect Read Order" in `documentation/hooks-implementation.md`; `combobox_tests` covers dev-ui's shape.
  Also fixed: `I18nProvider` leaked its locale to its siblings (now scoped to its children).
- [x] Audit 2026-10-07 of the effects that read memos for the read-order rule: fixed the themed `Select`'s option
  list (one memo source; check for narrowing searches in `select_components_tests`), `use_number_field`'s validity
  sync, `use_tab_list_state` (disabled keys read up front), the combo box's input text. The rest is safe.
- [x] SSR global state (audit 2026-10-06; rule in `documentation/hooks-implementation.md`, "Global State and SSR"):
  `use_description` is reactive and client-only (one API; the tag's description no longer touches the registry on
  the server); themed `Tab` and `TiptapEditor` ids from `use_id` (were random `Uuid`s: hydration mismatch, the
  editor could not attach); `use_prevent_scroll` releases only a count it holds (a disabled instance unmounting
  re-enabled scrolling under an open overlay).
### R2. Library-wide conventions (mechanical, before the families)
- [x] No constructors on `*Input` types (the user's rule, 2026-10-07, in CLAUDE.md): removed all 66 `new(..)`
  (the 5 added that day and 61 older ones); every call site in the library, test-app and book is an explicit struct
  literal. Defaults the constructors carried are now written by the callers (e.g. `UseSearchFieldInput`'s
  `input_type: Search`, `UseTableStateInput`'s `DisabledBehavior::Selection`).
- [x] C4 state props (the user's rule, 2026-10-06): every atom and component takes `<x>` (`Signal<T>`) +
  `set_<x>: Out<T>` (`is_<x>` → `set_<x>`: `is_open`/`set_open`, `is_selected`/`set_selected`,
  `is_expanded`/`set_expanded`) beside `default_<x>` + `on_<x>_change`; `ValueBinding::from_state_props`
  turns the props into the hook's binding (no `set_<x>`: read-only; no `<x>`: `set_<x>` receives every
  change). Converted: tooltip, menu (open, selection), listbox, grid list, grid, table (selection,
  sorting), select, combobox (value, input value), tabs, number field, disclosure (+ group), collapsible,
  slider, checkbox, switch, toggle button, text and search field, dialog trigger, modal backdrop, popover,
  themed modal/popover/number field/text fields/checkbox/switch/theme toggle. New controlled values:
  radio, checkbox and toggle button groups (`UseRadioGroupStateInput::value`, ...). `ListState` props
  stay (whole shared collection states for composition, not a value). Test app and book usages
  converted; unit tests for `from_state_props` and a bound radio group, browser test of a controlled
  and a fixed radio group.
- [x] C10 key comparisons (2026-10-06): all `e.key()` string comparisons are `KeyboardEventKey::typed_key()`
  (`use_press`'s keyboard helpers take `&KeyboardKey`; the macOS Meta map is keyed by it). `use_date_field`'s
  keydown match is still empty: upstream's `useDatePickerGroup` arrow navigation between segments (R3h).
- [x] C1–C13 documented in `documentation/hooks-implementation.md` ("API Conventions", standard deviation block
  format with mandatory reasons) and as global entries in `hooks/mod.rs`.
- [x] C1: `is_disabled`/`is_read_only`/`is_required` on all hook inputs, states, contexts and atom props
  (`Option<Signal<bool>>` atom props became `Signal<bool>`; `PressResponder` keeps its inherit/override
  `Option`); focus-visible's inverted `enabled` became `is_disabled`. Legacy components keep their props until
  rebuilt; the calendar view model (`utils::time`) moves with R3h.
- [x] C5: `use_locale()` (reactive signal), `use_direction()`, `use_i18n()` (the context, to change the locale);
  the panicking and snapshot accessors are gone. `is_rtl`/`writing_direction`/`locale` inputs removed (use_move's
  constrained mode, slider, slider thumb, color area, color slider, overlay position, popover, number field state
  (re-formats on locale change), atoms). `Locale: FromStr` (error `InvalidLocale`) and
  `From<icu_locale::Locale>` with the re-exported compile-time `locale!` macro replace the silently falling back
  `Locale::new`.
- [x] C6: one `utils::orientation::Orientation` (re-exported from `hooks`, no `Default`; atoms state react-aria's
  default per component).
- [x] C10 public API: `utils::key::Key` renamed `KeyboardKey` (no clash with the collection `Key`);
  `KeyboardEventWrapper::key() -> KeyboardKey` (+ `key_value()` for display), `PressEvent.key: Option<KeyboardKey>`,
  `KeyboardEventKey::typed_key()` for events. Internal string comparisons (~140) convert within the family
  packages; delays as `Duration` likewise.
- [x] `SelectionMode` defaults to `None`; collection atoms take `Signal<SelectionMode>`.
- [x] `PropsWithStyles::into_parts` vs `into_inner`: both stay (spread vs. compose further), documented.
- [x] Rule sweep: no direct `web_sys::window()/document()` left in hooks/atoms/utils (`EventTargetExt::
  get_owner_document` returns `Option`; unused `get_owner_document`/`get_owner_window` free functions removed);
  `target().and_then(dyn_into)` → `expect_target()` (the `if let` forms and stored events stay); untracked reads
  in the press/slider handlers; developer warnings via `utils::dev_warn!` (debug builds only, as react-aria).
### R3. Families (each: API to C3/C4/C7/C8/C9, missing features, re-sync, deviation block, tests, atoms, components)
- [x] **a. Quick wins:** done: dead `components/calendar.rs`/`time.rs`, the old `utils/event_listeners.rs`
  content, unused helpers (`AriaDescribedby::elements_with_ids`, `Key::as_cell`, `IncompleteDate::get_field`,
  `get_owner_document`/`get_owner_window`), commented-out code, `Mount::WhenShown`'s stray doc, the atom
  `ButtonWrapper` (only a theme class; the component stays), `ModalTitle` on the `DialogTitle` atom. Tabs rebuilt on `atoms::tabs` 2026-10-06 (see Done).
- [x] **b. Toggles:** one `use_toggle_state` → `ToggleState` (`ToggleState::new` for delegated state,
  `From<RwSignal<bool>>`/`From<(ReadSignal, WriteSignal)>` to bind app state like `bind:checked`); `use_toggle`
  (input inside a label: label press, `checked` as DOM property, form reset/validation) composed by `use_checkbox`
  (indeterminate) and `use_switch` (`role="switch"` on the input); checkbox group (state + group/item hooks, item
  validation merged into the group's); radio group (Key values, state into input, roving tabindex, arrow keys via
  `FocusManager` with RTL, read-only, form/reset/validation); `use_toggle_button`, `use_toggle_group_state`,
  `use_toggle_button_group(_item)` (`radiogroup` of `radio`s for single selection); `use_toolbar` re-ported (focus
  moves itself, Tab leaves, re-entry restores the last focused child, nested toolbar → group); `UseButtonInput`
  `role`/`aria_checked`, `UseButtonReturn::is_focused`. Atoms Checkbox(Group+Label/Description/ErrorMessage),
  RadioGroup(+parts)/Radio, Switch, ToggleButton(Group) with RAC's `data-*` render state. Components rebuilt:
  `Checkbox`/`CheckboxGroup`, `RadioGroup`/`Radio`, `Switch` (was `Toggle`; `size` now styled, `active` dropped),
  `ThemeToggle` (an accessible switch) + `use_theme`. Fixed on the way: native validation never cleared after a
  fix (the state now re-reads registered inputs' native validity before each commit). Browser tests from RAC
  `Checkbox`/`CheckboxGroup`/`RadioGroup`/`Switch`/`ToggleButton`/`ToggleButtonGroup` and `useToolbar` tests.
  Open: the styled ToggleButton component (moved to f).
- [x] **d. Overlays:** done 2026-10-06 (submenus, subdialogs, context menus, close on scroll, entry/exit
  animations; see Done).
- [x] **e. Interactions and focus:** done 2026-10-06 (see Done): `use_press`/`use_move`/focus APIs, `Pressable` and
  `Focusable` on their child, FocusScope re-sync (re-parenting, restore fallback, `focus_safely`, blur and Tab
  handling), browser tests from all of upstream's interaction tests (macOS/iOS-only paths of `useContextMenu` need
  those platforms).
- [x] **g. Meter, progress, spin button:** done 2026-10-06 (see Done), with a themed `Meter` component.
- [x] **Label slots:** done 2026-10-06 (see Done): the select's trigger and listbox, the combo box's button and
  listbox, the number field's steppers and the slider's thumbs follow whether the label is rendered.
- [x] Foundation: `utils::date` (`DateDuration`, `DateRange`, `DateExt` with upstream's month arithmetic and
  clamping, ICU4X first day of week, `today`), `DateTimeFormatter::format_date`; `jiff` re-exported at the root.
- [x] Calendar re-port (2026-10-06, switched over 2026-10-07: `hooks::calendar`, `atoms::calendar`; the old
  `time`-based hooks and the calendar view model in `utils/time.rs` are gone):
  `use_calendar_state`/`use_range_calendar_state` (visible durations of days/weeks/months, page behavior, selection
  alignment, unavailable dates, non-contiguous ranges), `use_calendar`/`use_range_calendar` (labels, announcements,
  prev/next buttons, commit behavior on outside presses), `use_calendar_grid`, `use_calendar_cell` (press, drag,
  labels, range prompt, DOM focus), `use_calendar_heading`, month/year picker hooks. Atoms `Calendar`,
  `RangeCalendar`, `CalendarHeading`, `CalendarPreviousButton`/`CalendarNextButton`, `CalendarErrorMessage`,
  `CalendarGrid` + header/body/week parts, `CalendarCell` (`td`) + `CalendarCellButton` (user's decision: two
  atoms). Cells are keyed by position (as RAC), so the focus stays when paging. Browser test `calendar_tests`
  (RAC Calendar/RangeCalendar tests, useCalendar keyboard tests). Resolves the book's calendar findings below.
- [x] Review 2026-10-06, fixed: the grid renders the id its `aria-labelledby` lists; single-field formats
  (weekday, month, day alone) via CLDR standalone patterns (narrow names instead of a cut first character, day
  numbers without "日"/"일"); today marks set in the browser (`utils::date::use_today`); paging shows the actual
  focused date (controlled focus); atoms take `selection_alignment`, `weeks_in_month`, `id`,
  `aria_describedby`, `aria_details`; the range calendar's default alignment uses the locale's first weekday;
  disabling the calendar unfocuses it; shadow-DOM-aware targets/containment, typed pointer type; the touch drag
  timer is cleared on cleanup. Checks added: labelled by another element, a range committed by an outside press,
  RTL arrows, the focused date set from outside, a controlled range cleared, anchor-dependent unavailable dates.
- [x] `DateSelector` rebuilt on the atoms (2026-10-07): jiff, C4 state props, days/months/years views
  (`initial_view` replaces `guide_mode`; the focus follows into each view; arrows move between months and
  years), own theme file `date_selector.scss`; `DateTimeInput` converts at its boundary until its rebuild.
  Browser test `date_selector_component_tests`. From the book (2026-10-07): the calendar buttons do set
  `data-focus-visible` (from `use_button`; now checked); the views' English labels ("choose a year", "Previous
  years") wait for localized strings (R4).
- [x] Date and time fields, date picker, date range picker on jiff (2026-10-07; `hooks::datepicker`,
  `atoms::datepicker`): `DateValue` (`civil::Date`/`DateTime`/`Zoned`) and `TimeValue` (`civil::Time`/...)
  traits, everything generic over them; `IncompleteDate` (upstream's new partial-value model: invalid dates such
  as February 30 shown until the field is left, DST-aware zoned hours), segments from ICU4X parts in the locale's
  order (placeholders per locale, the locale's or a forced hour cycle, leading zeros, eras only for BC),
  validation against min/max/unavailable dates (new `builtin_validation` of `use_form_validation_state`),
  `use_date_field_state`, `use_time_field_state`, `use_date_picker_state` (commit on close through the overlay
  state), `use_date_range_picker_state` (incomplete ranges, "Start date must be before end date"),
  `use_date_field` (+ `use_date_picker_group`), `use_date_segment` (spin button, typed digits moving on when full,
  AM/PM and era letters, Backspace, `beforeinput`/composition, collapsed selection, focus handover from a removed
  segment), `use_date_picker`/`use_date_range_picker` (shared `picker_aria`; `FocusManager` got default
  options: `with_default_accept`). Atoms `DateField`, `TimeField`, `DateInput` (`part` for range pickers),
  `DateSegment`, `DatePicker`, `DateRangePicker`, `DatePickerGroup`, `DatePickerButton`; inside the pickers the
  generic `Popover`/`Dialog` and `Calendar`/`RangeCalendar` (picker contexts). Browser test `date_field_tests`.
  Resolves the book's segment and picker findings (2026-10-05). Blur paths of the new hooks check that their
  state still exists (`is_alive`, "Blur After Disposal").
- [x] 2026-10-07: `hooks/datepicker_old` deleted; `toast.rs` and `DateTimeFormatter::format` on jiff
  (`civil::DateTime`); `uuid` dependency dropped. New themed `components::date_picker::DatePicker<V>` (label,
  segments + calendar button, description, errors; a `DateSelector` in a popover; theme `date_picker.scss`),
  browser test `date_picker_component_tests`.
- [x] 2026-10-07: deleted `components/datetime_input.rs` (`DateTimeInput`), `utils/time.rs` (`GuideMode`, `Type`),
  the `.leptonic-datetime` styles and the `time` dependency (the book moved to `DatePicker` first).
- [x] Review 2026-10-07, fixed: the picker's segments are labelled by its label (reactive); a time field submits
  the time ("08:30:00"); an edit belongs to a revision of the value (an A→B→A change resets it); values are
  reported only when they change (the range picker reported `None` on every edit of one end); a zoned time field's
  placeholder takes the value's zone; the picker's value wins over a pending selection, and its zone outlives a
  cleared value; the range validation uses the start's name; the dialog is named by the picker's button and label
  (`DialogTriggerContext::with_dialog_labelledby`) and its id tells focus moving into it; H11 zoned hours
  documented. Checks added for the label, the time's form value and the A→B→A reset.
- [x] **i. Color:** done 2026-10-06, see Done.
- [x] **k. Toast and landmark:** done 2026-10-07: `use_landmark` (F6 navigation), `ToastQueue`/`use_toast_state`
  (pausable timers), `use_toast`, `use_toast_region` (hover/focus pause, focus handover to the next toast, focus
  restore; a removed focused toast's blur, which React suppresses, keeps the state), atoms `ToastRegion`/`Toast`/
  `ToastContent`/`ToastTitle`/`ToastDescription`/`ToastCloseButton`, browser test `toast_tests` (from RAC's
  `Toast.test.js`); `components/toast.rs` rebuilt on the atoms (every toast closable, 5 s default, timeouts pause;
  `toast_component_tests`). `Drawer`: a modal side sheet on the modal atoms (slides via `data-entering`/
  `data-exiting`, reduced motion, `drawer_component_tests`). Transitions: CSS on `data-shown`, `inert` and invisible
  while hidden, `is_shown` (was `inn`/`show`), theme styles for all five, Collapse on a grid track (follows its
  content), reduced motion (`transitions_component_tests`; the exit hook isn't needed: a CSS transition starts after
  the attribute change, the delayed `visibility` covers it). `Alert`'s `assertiveness` (`Polite`:
  `role="status"`), `Icon`'s typed `width`/`height` (custom properties; they had no effect), `alert_component_tests`; the table's sort is described (`aria-describedby`) and announced
  ("sorted by column Type in ascending order", English until R4; `table_tests`).
### R4. Cross-cutting
- [x] From the book (2026-10-07), done: `Drawer`'s `width` (typed, `--drawer-width`, checked in
  `drawer_component_tests`); `AppBar` is a banner landmark (`use_landmark`, optional `aria_label`).
- [x] `utils::scroll` is public (dev-ui, 2026-10-07): `scroll_into_view`, `scroll_into_viewport`, `is_scrollable`,
  `get_scroll_parent(s)`, as react-aria exports them.
- [x] One value for boolean state attributes (2026-10-05): `"true"` everywhere (as react-aria-components), through
  `utils::data_attributes::flag`; `SelectTrigger` no longer repeats `use_button`'s `data-focus-visible`.
- [x] From crudkit (2026-10-05), grid focus jumping from a cell child to the old focused item: root cause was the
  interaction modality staying `Unknown` (= keyboard) until a focus-visible hook started tracking it (react-aria
  tracks from module load). Hooks reading the modality now start tracking when created
  (`track_interaction_modality`); grid regression test (refocus a non-first child, pointer and keyboard). crudkit's
  earlier focus-restore report was synthetic key events. Still open from crudkit: Enter on a cell's `Button` doing
  nothing (low confidence, not reproduced).
- [x] From crudkit (2026-10-05): the `Button` atom lacks `aria_label: MaybeProp<String>` and
  `aria_current: Signal<Option<AriaCurrent>>` (crudkit keeps own `use_button` atoms for icon and pagination buttons).
  Done: `aria_label`, `aria_labelledby`, `aria_describedby`, `aria_controls`, `aria_current` (tested).
  Also (crudkit): `Button`/`LinkButton` atoms expose `data-pressed`, `data-hovered`, `data-focused`,
  `data-disabled` (as react-aria-components; browser test extended, not run yet).
  leptosfmt breaks generic component tags (`<NumberField<T>>`); inference from `state`/`default_value` avoids them.
- [x] `use_visually_hidden` (`is_focusable`) + `VisuallyHidden` atom; one CSS source (`VISUALLY_HIDDEN_STYLE`) for
  the live announcer and hidden select; `DismissButton` re-ported on it (`aria_labelledby`, `MaybeProp` label).
- [x] Theme without the components layer (agnite dev-ui, 2026-10-06; done 2026-10-07): `Theme`, `LeptonicTheme`,
  `ThemeContext`, `ThemeProvider`, `use_theme` moved to `atoms::theme` (also in the components prelude);
  `ThemeToggle` stays a component (styled `Switch`); `signal_ls` was already at the crate root.
### R5. Testing infrastructure
- [x] Std `assert!`/`assert_eq!` in unit tests → assertr (doc examples keep std asserts).
- [x] Fixed `sleep`s in focus_scope / has_tabbable_child tests → polling (`BaseActions::wait_for_active_id`).
  The remaining fixed sleeps check that something does *not* happen (disabled controls, non-submit buttons) or
  outlast a timeout (type-ahead, announcer); they stay.
- [x] Page-object helpers duplicated by `BaseActions` removed.
- [x] Parallel browser tests: done (see the Book section's "Parallel browser tests").
- [x] Stale theme in browser runs (2026-10-06): with an outer `CARGO_TARGET_DIR`, leptonic's build script found no
  app and the test-app never got theme changes; and cargo-leptos may compile the styles before the build script
  writes the theme. The harness now writes the theme itself before starting the app (`leptonic_theme::generate`) and
  passes `LEPTONIC_APP_DIR`.

## Done (summary until 2026-10-07)

Interactions (R3e, 2026-10-06): a `PressResponder` without a pressable child warns (its `registered` flag); `use_press`
takes `propagation: PressPropagation` (`Stop`, `Continue`) instead of `force_propagation`, the never-used
`force_prevent_default` is gone, `on_press` is an `Option`, the config flags are signals and the long-press
description a `MaybeProp` (also on `UseButtonInput` and `PressResponder`); `use_focus_visible` keeps `auto_focus` (it
re-syncs only after being disabled); `use_interact_outside` passes `MouseEvent`s to both callbacks; the `FocusManager`
atom is `FocusManagerProvider` (no clash with the hook's type); `Pressable` puts its handlers onto its child
(`AddAnyAttr`, no wrapper), makes it focusable where it has no `tabindex`, and warns without a role (browser test from
`Pressable.test.js`); the responder's OR-ed flags are documented. `Pressable` also takes a surrounding
`PressResponder`'s trigger props (`aria-haspopup`/`-expanded`/`-controls`, the element to position at, as
`usePress` merges them). Two items rendered with one collection key warn in development (they made focus bounce
between them and froze the page). `use_move`: the constraint-only fields are grouped in `MoveConstraintOptions` (`mode`,
`allow_container_click`, `initial_position`, `on_position_change`; `MoveConstraintOptions::new(mode)`),
`UseMoveInput: Default`, `axis: Signal<MoveAxis>` (no `None` next to `MoveAxis::Both`) and `pixel_position:
Signal<Point>` (C13). The `FocusRing` atom takes `is_text_input` and sets `data-focused` (upstream's `focusClass`); test from
`useFocusVisible.test.js` ("emits on modality change (text input)"). New `Focusable` atom (upstream's `Focusable`: handlers onto its
child, `tabindex` where the child has none and kept in sync, upstream's role checks); `Pressable` shares its child
handling and now also takes a `FocusableContext`'s description and attributes, so both work as custom tooltip
triggers (tests from `Focusable.test.js` and `Tooltip.test.js`). FocusScope (audit "Interactions / focus"): scopes register in their
component body (as upstream's layout effect: all scopes mounting together register before any auto-focuses), a scope
mounting outside the active scope gets it as parent, an unmounting scope's children move to its parent and the active
scope passes to the parent; restoring falls back to the first tabbable element of the nearest ancestor scope still
mounted (and skips the body), and restores and recaptures with `focus_safely`; `focus_safely` re-synced (returns
for disconnected elements). Focus escaping a containing scope comes back without scrolling (upstream's
`focusFirstInScope`, not the scrolling focus manager), and a blur goes back to the element that lost focus. Tests: "does not throw when there is no focusable element to restore focus to" (with
and without another element) and "tracks node to restore if the node to restore was removed in another part of the
tree", "should restore focus to the last focused element in the scope on focus out". Of the
re-sync leftovers, press pointer capture release already matched upstream; the focus manager scrolls like upstream's.
Interaction tests (R3e, 2026-10-06) from upstream's `useHover`, `useMove`, `useKeyboard`, `useLongPress` and
`PressResponder` tests (fixtures `hooks/{hover,move,keyboard,long-press}`, synthetic pointer/key events where WebDriver
has no equivalent). They found: a long press never cancelled the press (its synthetic `pointercancel` didn't bubble to
the document listener, so `on_long_press_end` never fired and the press stayed active); long press callbacks now
precede the press callbacks, as upstream's `useLongPress` + `usePress`.
Review of R3e (2026-10-06), fixed: blur recapture refocuses the element that lost focus (re-checking containment in
the frame); the Tab handling of non-containing restoring scopes is upstream's (only when the next element is outside
the scope, `should_restore_focus`, gone nodes forgotten, blur when leaving the top-level scope; capture phase);
focus outside all scopes ends a plain scope's activity (tree knows `restore`); restore fallback also takes focusable
elements, dispatches the restore event on any element, and the event stops at each scope's boundary; deviations
documented (registration in the body, body skipped, cleanup order); `Focusable` auto-focuses after setting the
`tabindex`; `manage_child` per element, no warning on unmount, SSR-safe storage (a local `StoredValue` dropped on
another server thread panicked the server); `Pressable` merges the long-press description into `aria-describedby`;
`use_move` starts only on real movement and starts the pixel position at `initial_position`; the duplicate-key
warning compares with the replaced entry; release-only clippy lints (`trivially_copy_pass_by_ref` on `MaybeProp` and
`StoredValue`, 8 bytes in release) fixed. Further tests: `useInteractOutside` (inside/outside, other buttons, pointer up
alone, disabled), `useContextMenu` (right click position, prevented and stopped, no handler, no Ctrl+Enter off macOS),
`focusSafely` (deferred focus with virtual modality, not onto a removed element). Left as is: stale focusout frames aren't cancelled (each re-checks
containment, later corrected: a stale frame from an earlier blur could win, so only the latest one runs, as
upstream); a `FocusScope` created without an owner stays in the scope tree.
Label slots (2026-10-06): `UseButtonInput::aria_labelledby` and `UseListBoxInput::aria_labelledby` are
`Signal<Option<String>>`; the select, combo box, number field steppers and slider thumbs derive their labelling from
the label's presence (thumb ids now come from the group's stable id, the stepper ids are always rendered: deviations
documented); `use_label` puts the field's own id first and dedupes ids, as `useLabels` (our order made the computed
name "label aria-label" instead of "aria-label label"; unit test from `useLabel.test.js`); browser test
`label_slots_tests` (labels toggled after mount).
Color area (R3i, 2026-10-06): `use_color_area_state` → `ColorAreaState<C>` (C3 methods, C4 `value` binding, default
axes, steps snapped to the channel step: page steps used to snap to themselves); `use_color_area` re-ported (keyboard
shortcuts, arrow steps with Shift for page steps through `use_move`'s keyboard deltas instead of pixel moves, thumb and
area presses with global pointer up, focus handling of the two inputs with `tabindex`/`aria-hidden`, value text,
labels as `useLabels`, visually hidden inputs, form reset, gradient and thumb styles in the returned props);
`get_area_gradient` takes the writing direction, the RGB gradient's layers are upstream's; atoms `ColorArea` (C4,
`String` props, `data-disabled`) and `ColorThumb` (one element each; the thumb renders the two hidden inputs, as
RAC's and our `SliderThumb`); the `ColorPalette` component is built on them. Browser test from react-spectrum's
`ColorArea.test.tsx` (`color_area_tests`). Color slider: `ColorSliderState<C>` (C3/C4, input by value, disabled/orientation only on the
state), `use_color_slider` returns typed `track_styles`/`input_styles` and labels as upstream (the channel's name on
the group without labels); atoms `ColorSlider`, `ColorSliderTrack`, `ColorSliderOutput`. Color wheel: re-ported
(0° at 3 o'clock as upstream, was 12 o'clock; steps wrap like upstream's; track/thumb/input props with upstream's
styles: conic gradient, clip path); atoms `ColorWheel`, `ColorWheelTrack`. One `ColorThumb` for area, slider and
wheel (`ColorThumbContext`, as RAC's `InternalColorThumbContext`). Browser tests `color_slider_tests`,
`color_wheel_tests`.
Color fields, names, swatches, picker (R3i, 2026-10-06): `use_color_field(_state)` (hex text, C3/C4, validation,
spin keys, scroll wheel, form reset) and `use_color_channel_field(_state)` on the number field (channel format
options, name/form, validation); atoms `ColorField`, `ColorChannelField<C>`; `use_formatted_text_field` shared with the
number field. Color names as upstream (`color_name()`/`hue_name()`, OKLCH; in the value texts of area, slider and
wheel; unit test from `Color.test.tsx`). `use_color_swatch` re-ported (name joined with `aria_label`, `color_name`,
labelledby with its own id, styles in the props); atoms `ColorSwatch` (color optional in a picker),
`ColorSwatchPicker`/`ColorSwatchPickerItem`/`ColorSwatchPickerItems` (a single-selection grid `ListBox` keyed by hex). Then
(found by the book): `ColorSwatch` shows any color (`ColorProp`: any color value or signal of one, no type
parameter); a `ColorSwatchPickerItem` provides its color to the swatch inside (`ColorSwatchPickerItemContext`, before
a `ColorPicker`'s) and takes `is_disabled` (registered as the listbox's disabled keys); `ColorSwatchPickerItems` lost
its type parameters and renders a swatch by default. The button theme's disabled colors are variables
(`--button-disabled-*`, dark values) instead of light-only literals. Alpha: `Alpha<C>` adds an alpha channel to any color type
(`AlphaChannel::{Color(..), Alpha}`, percent formatting, opaque display colors except for the alpha slider, names
"…, 80% transparent", `rgba(..)` CSS); `Color` is `Alpha<OpaqueColor>` (parses `#rgba`, `#rrggbbaa`, `rgba()`, `hsba()`,
`hsla()` as `parseColor`); opaque components in a `ColorPicker` keep its alpha; an alpha slider's value text names no
color, a fully transparent swatch is "transparent"; `channels()` returns a `Vec`; the `RGBA8` stub is gone. Unit tests
from `Color.test.tsx`, alpha checks in `color_picker_tests`.
`Color` enum (replaces the unused `ColorSpace`; keeps the space it was set in, `to::<C>()`, `FromStr` as `parseColor`
without alpha, `Display`); `ColorValue: From<Color> + Into<Color>`, `to_rgb8`, `hue_channel`; typed `BlendMode`;
`ColorChannel { type Color }`: `ColorSlider`, `ColorWheel` and `ColorChannelField` are generic over the channel and
infer the color type from it.
`use_color_picker_state` re-ported (`Color`, C3/C4); atom `ColorPicker` (no element; `ColorPickerContext`, which every
color atom without its own value binds to). Styled `ColorPicker` rebuilt on the atoms (C4 `Color`; preview, palette,
hue slider, HSB and RGB channel fields, hex field), `ColorPreview`/`ColorPalette`/`HueSlider` wrap atoms. Fixed on the
way: RGB → HSV/HSL gave negative hues (`%` instead of `rem_euclid`), HSV → RGB8 truncated instead of rounding, the
hidden inputs' styles added `width`/`height` twice (`visually_hidden_full_size_styles`). Browser tests
`color_field_tests`, `color_swatch_tests`, `color_picker_tests` (RAC `ColorPicker.test.js`),
`color_picker_component_tests`; color pages in `hydration_ids_tests`.

Collections leftovers (R3j, 2026-10-06): the `use_listbox` doc example compiles against today's API (`MaybeProp`
label, `PropsWithStyles`); the legacy `Table` takes `bordered`/`hoverable` and `TableHeaderCell` `min_width` (default
`true`) as plain signals, and the bordered theme draws its cell borders again (check in `table_components_tests`); the
`ListBox`/`GridList` atoms lost the unused `state` alternative (one source: `collection`, or a `Select`/`ComboBox`
parent; the `GridList` requires it); the tri-state flags are enums `CloseOnSelect`/`SelectOnFocus`/`SelectOnPressUp`
(`Auto`/`Always`/`Never`, `resolve(auto)`, `From<bool>`; react-aria: optional booleans). Already done: `trigger_id`
(now the captured trigger), the ComboBox/Tabs/single-select unit tests run.

Tabs (R3a, 2026-10-06): the styled `Tabs`/`Tab` keep their declarative API (`<Tabs><Tab name label>..</Tab></Tabs>`)
on the tabs atoms: `Tab`s register while `Tabs` creates its children (synchronously, so the server renders every
tab), `Tabs` builds the collection and renders `TabList`/`Tab`/`TabPanel`; C4 selection by name
(`selected_key`/`set_selected_key`/`default_selected_key`/`on_selection_change`), `Tab::is_disabled`, orientation,
keyboard activation, `aria_label`; `Mount::Once` keeps hidden panels mounted through the new `TabPanel`
`should_force_mount` (inert, `data-inert`, as RAC); `on_show`/`on_hide` dropped (`on_selection_change`). Theme on
`data-selected`/`data-disabled`/`data-inert`, a focus ring and a vertical layout. Browser test
`tabs_components_tests` (from RAC `Tabs.test.js`), the page in `hydration_ids_tests`.

Review 2026-10-06 (use_press propagation, selects and chips, tiptap 0.10, swatch picker, alpha), fixed: `#rgba` parsing
sliced non-ASCII text (panic) and took signs; `use_press` disabled text selection on the event target instead of the
pressed element (and before the disabled check), leaving `user-select: none` behind (browser check); swatch picker keys
ignored alpha (now `hexa`); disabled swatches registered only in an effect (wrong server HTML); a disabled
`Multiselect`'s chips stayed removable (`Chip::is_disabled`); removing a chip or clearing an `OptionalSelect` lost the
focus (now the next chip or the trigger; chips keyed) and the Multiselect trigger's focus was invisible; `max` dropped
a chosen option instead of refusing the new one; stale docs (swatch deviations, CLAUDE.md's tiptap line). Checked fine:
the propagation port, the `try_run` blur paths, the `use_move` split, the tiptap value sync, the slider marks.

Virtual focus (2026-10-06, found by the book): `use_combobox` ports upstream's "re-show focus ring" effect (a
virtual focus event on the focused input once no option is virtually focused; check in `combobox_tests`). Checked:
`use_selectable_item`'s `move_virtual_focus` runs after the input's `aria-activedescendant` changed, so in a combo box
it sends nothing, as upstream's (React commits the attribute before `useEffect`; Leptos' render effects run before
`Effect::new`); only an autocomplete, which sets the attribute in its focus listener, gets the events.

Review 2026-10-06 (label slots, submenus, context menus, animations), fixed: submenu popovers don't close on
scroll (upstream's `!isSubmenu`); popovers and modals stop containing focus while exiting; the combo box's button and
listbox reference the label only if one is rendered; the safe-triangle side is re-measured per opening; the macOS
Control+Enter fallback survives an unmount; `use_label` warns about a missing label after mount (atoms' presence);
an exit without a rendered element closes at once.

Entry/exit animations (R3d, 2026-10-06): `animation.ts` re-synced (the element is hidden with layout-neutral
styles until ready, `on_enter`/`on_exit` callbacks, only running document-timeline animations are awaited, earlier
transitions cancelled; `is_exiting` derived synchronously, so an element kept while exiting is never remounted);
`Popover` (entering once placed), `ModalBackdrop` + `ModalContent` (waiting for both) and `Tooltip` render
`data-entering`/`data-exiting` and stay mounted until their exit animations end; a popover's underlay only while
open. Browser checks with CSS animations in the popover and dialog tests.

Close on scroll (R3d, 2026-10-06): `use_close_on_scroll` re-synced to 99e6102368 (a54c33a8e: listens on the
window and every shadow root around the trigger, `utils::shadow_dom::propagation_targets`; shadow-aware
containment); popover test from `useOverlayPosition.test.tsx`'s scroll tests.

Context menus (R3d, 2026-10-06): `use_context_menu` (right click, Shift+F10, Control+Enter fallback on macOS,
long press on iOS through the element's own `use_press`), `MenuTriggerType::ContextMenu` (opens at the point,
no `aria-haspopup`/`-expanded`/`-controls`, a right click outside closes), `UseButtonInput::on_context_menu` and
`PressResponder`'s, `MenuTriggerStateApi::set_point` (the menu trigger state now forwards its overlay's point to the
popover); `MenuTrigger` gives its popover RAC's defaults (`BottomStart`, offset 0 for context menus), scoped to
it (`ClearTriggerContexts` resets them inside overlays); popovers carry RAC's `data-trigger`. Subdialogs
(`SubmenuKind::Dialog`): as RAC, a submenu's popover is a dialog (unless it holds one), contains focus and is focused
on opening unless opened by pointer; browser test from RAC's "should contain focus for subdialogs". Browser test
from RAC's context menu tests.

Submenus (R3d, 2026-10-06): `use_submenu_trigger_state` (level from the root's stack, an `OverlayTriggerState`
view for the popover), `use_submenu_trigger` (press/hover-with-delay/arrow opening, arrow/Escape/focusing another
item closing, RTL), `use_safely_mouse_to_submenu` (the parent menu ignores the pointer while it heads for the
submenu); `use_menu_item` trigger mode (no action/selection, `aria-haspopup`/`-expanded`/`-controls`, no DOM focus
on mouse down) and `use_menu` submenu props; popover groups (`use_overlay`/`use_interact_outside`/`use_popover`
`group`, a root popover's `display: contents` container its submenus' popovers mount into, as RAC's
`PopoverGroupContext`; the overlay stack counts entries per open overlay); atoms `SubmenuTrigger` (key of its trigger
`MenuItem`, a `Popover` with a `Menu` of its own collection), `Menu` keeps a root state without trigger, `Popover`
defaults (`EndTop`, non-modal) inside a submenu; trigger items get `data-has-submenu`/`data-open`. Browser test from
RAC's submenu tests (`test_submenu.rs`), book section on the menu atom page.

Label slots (2026-10-06): `has_label` of `use_label` and the hooks on it is a `Signal<bool>` and `aria-labelledby`
follows it; atoms detect their `Label` part (`LabelPresence`: a guess from the ARIA props until mounted, so server
HTML references a likely label, then whether the `Label` rendered, captured through `LabelContext`), as RAC's
`useSlot`. No more dangling `aria-labelledby` without a `Label`, and a `Label` next to an `aria_label` names the
element together with it (themed `ProgressBar`/`Meter` keep both). Browser checks in the progress bar test.

Review 2026-10-06 (state-props sweep, progress/meter), fixed: `LabelContext` was provided in component bodies
(reaching sibling `Label`s), now through `Provider`s (browser check); a bound slider resets to its initial values
and a step keeps the app's other thumb values; `set_values`/`set_open` work without the value prop; `ModalBackdrop`
and `Popover` got `default_open`/`on_open_change` with RAC's local-vs-trigger state rule (`overlay_open_state`);
select and combo box report picking the previous value after the app changed a bound value (browser check); typed
fill width; `NumberFormatter` builds its ICU formatter once (`icu_provider` `sync` feature: ICU formatters are now
`Send + Sync`); stale docs; theme stripes loop seamlessly and respect reduced motion.

Meter and progress bar (R3g, 2026-10-06): `use_progress_bar` re-ported generic over `NumberValue` (`value:
Signal<Option<T>>`, `None` = indeterminate; `format_options` with upstream's percent default through the new
`use_number_formatter`; `value_label`; labelling through `use_label` with a `<span>` label; returns `props`,
`label_props`, `percentage`, `value_text`), `use_meter` on it (`role="meter"`); atoms `ProgressBar<T>`/`Meter<T>` with
the parts `ProgressBarFill`/`MeterFill` and `ProgressBarValueText`/`MeterValueText` (RAC's render props) and
`data-indeterminate`; themed `ProgressBar` rebuilt on the atom. `LabelContext` split from `FieldContext` (RAC's
design: any atom with a visible label provides one). `NumberSignal<T>`/`OptionalNumberSignal<T>`: number props whose
conversions accept only `NumberValue`s, so a generic component's `T` is inferred (`#[prop(into)] Signal<T>` can't:
Leptos also converts any value into `Signal<Option<_>>`). Unit tests from `useProgressBar.test.js`, browser test from
RAC's `ProgressBar.test.js`/`Meter.test.js`; atom pages in the book. The number fields' and the themed slider's
`value` props are `OptionalNumberSignal`/`NumberSignal` too (`<NumberField value=Some(200)>` infers `i32`).
`use_spin_button` was already at upstream 99e6102368 (keyboard repeats, typed shortcuts); new browser test from
`useSpinButton.test.js` (roles and ARIA props, keys and page fallbacks, disabled/read-only, announcements with the
minus sign).

Slider (R3g, 2026-10-06): `use_slider_state` re-ported generic over `NumberValue` (`SliderState<T>`, C3, `Signal`
range/step, `ValueBinding`, `format_options`, `value_label`/`page_size` for the color slider; react-stately's
`restrictValues` snapping), `use_slider` (on `use_field`: label, description, error slots; track presses on
`pointerdown`), `use_slider_thumb` (visually hidden `<input type="range">`, keyboard through `use_move` and
shortcuts, form reset), `use_slider_marks` on `f64` inputs with reactive `in_range`; atoms `Slider<T>` (type-erased
context for its parts), `SliderTrack`, `SliderFill` (upstream's, with `offset`), `SliderThumb`, `SliderOutput`,
`SliderThumbTooltip`, `SliderMarks`/`SliderMark`; themed `Slider<T>`/`RangeSlider<T>` (`value` + `set_value`,
`RangeInclusive`); `NumberValue::from_f64`; color slider on the new state. Browser test from RAC `Slider.test.js`.

Links, breadcrumbs, toolbar (R3f, 2026-10-06): `use_link` re-ported (explicit props like `use_button`; `Default`;
`href` signal, not rendered while disabled; `tabindex` for non-anchor links; `aria_label`; hover/focus state; a
surrounding responder's trigger props, capture, shortcuts and disabled state), `LinkElementType::{Anchor, Other}`,
`LinkTarget::{Blank, Same, Parent, Top, Named}` (`noopener` added for `Blank`); `use_anchor_link` on `use_link`
(`UseAnchorLinkInput::new(href)` + nested `UseLinkInput`); one `Link` atom (`LinkExt` merged; `<A>` while enabled,
`<span role="link">` while disabled; `current_match: CurrentMatch`, `replace`, `target`, `rel`, data attributes;
`LinkContext` for containers); `use_breadcrumbs`/`use_breadcrumb_item` re-ported (item on `use_link`, current item
disabled with `aria-current`), atoms `Breadcrumbs`/`Breadcrumb` (`is_current`, `on_action`); `Toolbar` atom; Button
atom `form` (`ButtonFormAttributes` with typed `FormMethod`, `FormEncType`, `LinkTarget`). Browser tests from RAC
`Link`/`Breadcrumbs`/`Toolbar`; book pages for the atoms, link/breadcrumbs hook pages updated.

Disclosure (R3f, 2026-10-06): `use_disclosure` re-ported (configures `use_button`, `role="group"` panel,
`hidden="until-found"` with `beforematch`, `--disclosure-panel-width/height` for transitions),
`use_disclosure_state` (C3/C4) and `use_disclosure_group_state` (`DisclosureGroupExpansion`); atoms `Disclosure`,
`DisclosureTrigger`, `DisclosurePanel`, `DisclosureGroup`; themed `Collapsible`/`Collapsibles` rebuilt on them (`OnOpen`
gone). Browser test from RAC `Disclosure.test.js`; the hydration check now accepts ids the client adds. Book: hook
page, new atom page, collapsible pages, kit `Disclosure` on the atoms.

Text inputs and fields (R3c, 2026-10-05): `use_label`/`use_field` re-ported, C14 field parts (`atoms::field`:
`FieldContext`, `Label`, `Description`, `FieldError`), text field C1/C2, atoms `TextField`/`Input`/`TextArea`/`Form`
(validation behavior from the form, else `Native`), `use_form_validation` re-ported, `use_search_field` on
`use_text_field` + SearchField atom, number field generic over its value type (C15: `NumberValue`, `ValueBinding`,
`NumberFieldState<T>`, ICU4X formatter/parser) + NumberField atom, one generic `Input` for every field; legacy
`TextInput`/`PasswordInput`/`NumberInput`/`Label`/`Field`/`FormControl` deleted; themed field gaps fixed
(`input_type` signal, `MaybeProp` texts, pass-through attributes). Browser tests from RAC
`TextField`/`Form`/`SearchField`/`NumberField`.

Overlay positioning (R3d, 2026-10-06): `calculate_position` + `use_overlay_position` re-ported to 99e6102368:
typed `Placement` (upstream's 22 placements) replaces the two-axis `PlacementX`/`PlacementY`; absolute positioning in
the containing block (containing-block detection, margins, visual viewport origin, scale-independent sizes,
`bottom`/`right` anchoring); arrows (`arrow_props`, `OverlayArrow` atom with optional children), `target_rect` (the
state's `point` for context menus), `trigger_anchor_point`/`--trigger-anchor-point`, `--trigger-width`, user
`max_height`, `boundary`, `should_update_position`, `PositionUpdater`, scroll anchoring, pinch-zoom freeze,
close-on-scroll inside the hook and suppressed while the visual viewport resizes; `data-placement` from the resolved
side (was wrong for side placements); Select/ComboBox popovers default to `BottomStart` (RAC). Native tests: upstream's
`calculatePosition.test.ts` matrix; browser test `test_overlay_position.rs` (offset, centering, arrow, flip,
`--trigger-width`). Menu `Separator`s.

Separator (R3g, 2026-10-06): `use_separator` re-ported (C9 `props`, `aria-orientation` only when vertical, id and
labels), `Separator` atom (`<hr>`, else `<div role="separator">`; inside a `Menu` always a `div`), themed `Separator`
on it with `orientation` and theme styles (`--separator-*`); book atom page, component page with CSS variables.

Overlays (R3d, 2026-10-05): one `OverlayTriggerState` (C3, `ValueBinding`, `point`) under the menu, tooltip, select
and combo box states (`OverlayState`/`MenuTriggerStateApi` traits); re-ported to 99e6102368: `use_dialog` (title and
content slots, fallback name from a `DialogTrigger`), `use_modal_backdrop`, `use_popover` (generic over the state,
`PopoverModality`), `use_overlay` (Escape via shortcuts, no underlay props), `aria_hide_outside` (`HideMode`),
`use_tooltip_trigger_state` (`Duration` delays, `TooltipTiming`); atoms `DialogTrigger`, `Dialog`/`DialogTitle`/
`DialogDescription`, `ModalBackdrop`/`ModalContent`, `Popover` (modal popover is the dialog unless one is inside),
`TooltipTrigger`/`Tooltip` (through the extended `FocusableContext`), `MenuTrigger` (press and long press)/`Menu`/
`MenuItem`(+label/description/shortcut)/`MenuItems`/`MenuSection`; `SelectPopover`/`ComboBoxPopover` on the shared
popover rendering; `PressResponder` carries trigger ARIA props, shortcuts and long-press callbacks; FocusScope's
active scope tracked document-wide; themed `Modal`/`Popover` rebuilt. Browser tests from RAC
`Popover`/`Dialog`/`Tooltip`/`Menu` and React Spectrum `MenuTrigger` tests.

Collections migration (design: `documentation/design-collections.md`): core (`Key`, `Collection` + builder,
`SelectionManager`, list state and delegates, `use_selectable_collection`), listbox + select, menu, combobox
(virtual focus), grid list, tag group, tree, grid, table (+ column resizing), tabs, drag and drop (full re-port);
`hooks::selection` deleted. Text field rewritten (hook-owned `TextFieldState`). Legacy react-aria API removed (IE key
names, browser workarounds, `ValidationState`, deprecated props). SSR: all `StoredValue::new_local` uses sit
behind client-only cfgs.

Virtualization (2026-10-07, for agnite dev-ui): a port of `react-stately/virtualizer` + `react-stately/layout` +
`react-aria/virtualizer` in `hooks::virtualizer` (`Layout`, `LayoutInfo`, `ListLayout` incl. the AI layout's end
anchoring, `ScrollAnchorTracker`, overscan, the virtualizer state; `use_virtualizer_state`, `use_scroll_view`,
`use_virtualizer_item`; upstream's `ScrollAnchor.test.ts` ported). Atoms: `Virtualizer` (RAC) virtualizes a
`ListBox` rendered through `ListBoxItems` (focused key persisted, the layout as `LayoutDelegate`,
`aria-posinset`/`aria-setsize`); `VirtualList` (no upstream equivalent): plain app-rendered rows, text stays
selectable (rows holding the selection persist), `is_anchored_to_end` as C4 state. Items render into slots that
never move (Leptos' `For` moves kept rows, which collapses a text selection); a deferred `scroll_to` yields to the
user's scroll. Tests `virtualizer_tests`, `virtual_list_tests`.

Keyboard shortcuts (2026-10-07, the book's findings): Shift is ignored for uncased characters (`/`, `?`, digits)
unless required; `KeyboardKey::ControlSymbol` (⌃) on Apple; `use_global_shortcuts` shares one document listener
(the most recent binding wins, `continue_propagation` passes on); `Shortcut::to_aria_keyshortcuts` →
`utils::aria::AriaKeyshortcuts`; the `Keys` atom (literal keys); `node_ref` on the `Input`/`TextArea` atoms;
`utils::syntax_highlight` public, the `syntax-highlight` feature without `components`.

Default classes (2026-10-07, step 1 of removing the components layer): every atom rendering its own element has
`leptonic-<AtomName>` (`utils::default_class::with_default_class`; the caller's classes add to it).

TagGroup atoms (2026-10-07, replacing the `Chip` component): `TagGroup` (label/description slots), `TagList`
(`data-empty`, focus ring, `empty_state`), `Tag` (RAC's data attributes incl. `data-allows-removing`,
`data-selection-mode`), `TagItems`, `TagRemoveButton` (RAC: a `Button` with `slot="remove"`); test
`tag_group_atom_tests` from RAC's `TagGroup.test.js`.

Atom theme (2026-10-07): react-aria-components' starter styles ported to `leptonic-theme/scss/atoms/` (51
stylesheets, entry `leptonic-atoms.scss`, `documentation/atom-theme.md`, converter `scripts/port-starter-css.py`,
drift tracked by `scripts/upstream-drift.sh`).

Review of the day's work (2026-10-07): `scroll_to` converts to the view's scroll coordinates and expects the
clamped position; `VirtualList`'s end detection uses the list's own scroll metrics; the first render applies its
layout options; "item sizes changed" holds for one render only; the item size observer's callback no longer
leaks; global shortcuts skip bindings removed mid-dispatch and let `Ignored` fall through; one platform predicate
for matching and showing `Mod`; `Shortcut::parse` takes the `+` key; TagGroup deviations documented. Also:
the virtualized focused item scrolls into view once rendered (`ItemElements::get_tracked`), landmark checks run
after route transitions settle, `ListBox`/`GridList`/`Menu`/`Tabs`/`ComboBoxButton` data attributes as RAC.

Browser tests (2026-10-07): the component fixtures and tests are removed with their layer (the test-app's shell
uses `ThemeProvider`, no stylesheet theme); tests no longer depend on styling: `wait_for_count`,
`wait_for_selector_text` and waits for `data-entering` around overlay animations, focus helpers compare the
rendered text (`innerText`: WebDriver skips `display: contents` children), tags found by name, descriptions read
by text content, the combo box test closes the options its cleared input opens.

Dependencies (2026-10-07, the user's high-priority item): all upgraded to their latest versions (compatible
ones together; strum 0.28, itertools 0.15, cargo_toml 1.0 one at a time), unused ones removed (`leptos_reactive`
with the unused Leptos 0.6 `utils::signals`, `educe`, `indoc`, `leptos_meta`, `reactive_graph`; the test-app's
`tower`, `tower-http`), and every dependency of `leptonic`, `leptonic-theme` and the test-app declared with
`default-features = false` plus only the features needed. The `atoms` and `components` features now gate their
modules (they compiled always), `icondata` is optional with `components`, `leptonic-theme` reruns its build when its
SCSS changes.

Clipboard (2026-10-07, for the book's "Copy as Markdown"): `utils::clipboard::write_text_deferred` writes text that
is still loading: called in the press handler, it issues `navigator.clipboard.write` with a `ClipboardItem` whose
text is a promise (Safari keeps the user activation for it), completed when the future gives the text
(`ClipboardError::NoText` when it gives none). Test `clipboard_write_tests` (CDP grants clipboard read).

`syntect` (2026-10-07): `default-fancy` replaced by `default-syntaxes`, `html` and `regex-fancy` (no theme dumps,
plist or YAML loaders: the library only highlights with the bundled syntaxes into classed HTML).

Components layer removed (2026-10-07, the user's decision; steps: default classes `leptonic-<AtomName>`, the atom
theme, the book off `leptonic::components` (book session), the deletion): `leptonic/src/components/`, the features
`components`, `tiptap`, `sanitize`, `themes` and the deps `icondata`, `leptos-tiptap`, `ammonia`, `ordered-float`;
`full` = hooks, atoms, clipboard, syntax-highlight; `ToStaticStrRepr` and `OptionDeref` (std's `Option::as_deref`);
the component stylesheets and themes in `leptonic-theme` (`leptonic-themes.scss` is no longer generated, the theme
crate copies only the atom theme); `documentation/components-implementation.md`. The starter templates
(`examples/leptonic-template-*`) were ported to atoms + the atom theme (the user's decision) and got the
`.cargo/config.toml` leptonic needs. What replaced each component (decided 2026-10-07):
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

Date/time fixes (2026-10-07, from the fidelity review's leftovers):
- 12-hour times use the locale's own 12-hour clock: `HourCycle::H12`/`H24` map to ICU4X's `Clock12`/`Clock24`
  (`Intl`'s `hour12`), so ja-JP shows "午前0:30" (h11), not "12" (unit tests in `datepicker/format.rs`).
- Day periods are AM/PM as in `Intl`: ICU4X's data gives German hour-only 12-hour times CLDR's flexible day periods
  (`h 'Uhr' B`: "nachts"); a `B` field's text is replaced with the locale's AM/PM name (formatted from an `a`
  pattern). `names_the_day_periods_of_the_locale` passes again. Browser test `twelve_hour_clocks`
  (`date_picker_tests`: de-DE minute and hour, ja-JP). ICU4X 2.2 can't format the `B` field at all (a literal "B"),
  and the apps' lockfiles still had 2.2: the library now requires ICU4X `2.3` (all `icu_*` crates).
- Not a bug: "a date field's placeholder (`V::today()`) gives a different `aria-valuemax` on server and client".
  Segment limits don't depend on the placeholder (the day's maximum is a constant, as react-stately's
  `getMaximumDaysInMonth()`); a browser test with the client's clock set to another month confirmed it.

## Resolved findings

- Found by the book's re-check (2026-10-06; both done 2026-10-06: `utils::HideMode` exported, Cmd (+ Shift) +
  Home/End registered on macOS): `HideMode` isn't exported from `utils` (`utils/mod.rs:66`; the book's
  `aria_hide_outside` page says only the default mode is available); on macOS, Command + Home/End do nothing in
  collections: `with_selection_modifiers` (`hooks/collections/use_selectable_collection.rs:191-199`) registers
  Home/End plain, with Shift and with Alt, never with Meta, so `is_ctrl_key_pressed` can't be true (upstream: Cmd +
  Home/End move to the first/last item).
- Theme and component gaps stated on the book's Buttons/Fields pages (2026-10-06; done 2026-10-06: the button theme
  matches `data-disabled` (buttons and link buttons), `ButtonSize::Big` has styles, the styled `Slider` has disabled
  styles, a thumb focus ring (`data-focus-visible`) and a vertical layout, `SliderMark` places marks by orientation
  and writing direction (`inset-inline-start`/`bottom`); browser tests `button_components_tests`,
  `slider_components_tests`. `TiptapEditor`: `default_value` + `on_change` (the instance reads its content once), a
  group named by `aria_label`, the menu a `Toolbar` (arrow keys) of buttons with `aria-pressed` (the styled `Button`
  got `aria_pressed`), the theme's menu styles matched `[variant]` instead of `[data-variant]` (never applied), focus
  shows on the editor's border; `tiptap_editor_components_tests`. Then moved to leptos-tiptap 0.10 (its JS ships as
  wasm-bindgen snippets: no more `leptos-tiptap-build`, JS copying, `js-dir` metadata or `Root` script tags, and no
  race between the tiptap bundle and hydration, which made the test flaky); `TiptapEditor` on `use_tiptap_editor`
  with C4 HTML props `value`/`set_value`/`default_value`/`on_change: Callback<String>`): the styled `Button` looks enabled
  when disabled (the theme matches only `[aria-disabled="true"]`, the atom renders `disabled` + `data-disabled`;
  `button.scss:119, 252, 396`); `ButtonSize::Big` has no styles; the styled `Slider`/`RangeSlider` have no disabled
  styling, no thumb focus ring and no vertical layout; `SliderMark` positions with `left` regardless of orientation
  (vertical marks; check right-to-left too); `TiptapEditor` reads `value` once (`get_untracked`: a `default_value` in
  disguise, against the state-props rule), has no `label`/`aria_label`, and its menu buttons don't announce the active
  format (`aria-pressed`).
- Library gaps found by the book's content agents (2026-10-06; all done 2026-10-06: `use_press` ported upstream's
  propagation (triggers return "should stop", each handler stops once; browser checks from upstream's "event
  bubbling" tests); `Chip` takes `on_dismiss` and renders a `Button` named by `dismiss_label` ("Dismiss"), `color` is a
  plain `Signal`; the `Multiselect`'s chips and the `OptionalSelect`'s clear button moved out of the trigger button
  (buttons can't contain buttons) into a `.leptonic-select-control` box; blur paths `try_run` their callbacks (a
  focused button removed by its own press blurred after disposal and panicked; documented in hooks-implementation.md,
  "Blur After Disposal"); `use_move` without constraint, `use_constrained_move(input, options)` returns its parts
  without `Option`; `Pressable` as upstream, which sets neither data attributes nor `aria-disabled`): `use_press`'s click handler always stops the click
  unless `PressPropagation::Continue` (upstream stops it only when press-up/press-end don't continue), so a parent
  press never sees a nested one (blocks the Event Propagation demo); the `Chip` component's dismiss icon reacts to
  clicks only (not focusable, not announced); `use_move` returns the constraint parts as `Option` although
  `constraint` was given (callers `expect`); `Pressable` sets no data attributes and no `aria-disabled` when a
  surrounding `PressResponder` disables it.
- Library findings of the book audit (2026-10-06; details with file:line in the book session's audit reports).
  Done 2026-10-06: `use_focus_ring` drops `is_focused` when disabled (test); `use_spin_button` reads "Empty"
  without a value and disables its returned steppers with the spin button (test); `Default` for
  `UseInteractOutsideInput`/`UseScrollWheelInput`/`UseFocusInput`, `UseOverlayInput::new`,
  `UseExitAnimationInput::new`; `LinkButton`'s unused `active` removed; `use_search_field` uses `expect_target`;
  `ColorArea`/`ColorSwatch` in `atoms::prelude` (grid/table/tabs wait for the legacy names, item a); doc comments
  (`has_label` examples, `use_exit_animation` example, radio's C4 note, modal `is_dismissable`, select's
  `SelectLabel`, radio `orientation`, table resizer keyboard flow); `GridRow` `data-focus-visible`, `GridCell`
  `data-focused`/`data-focus-visible`, `Hoverable` `data-hovered` (tests); legacy tabs internals `pub(crate)`;
  `AppBar` is a `<header>`, `Alert` has `role="alert"`, `Skeleton` stops animating with reduced motion, the light
  theme's `--link-color` is `#b83f2b` (5.5:1 on white). The hooks' focus visibility (slider thumb, color area/slider)
  is as upstream: callers compose `use_focus_ring` (the atoms set the data attributes). `use_prevent_scroll` returns
  nothing (as upstream; its input has `Default`); `Announcement::LabelledBy(Vec<String>)`; public
  `utils::clipboard::write_text` (`clipboard` feature, async, `ClipboardError`), used by `Code`. The styled
  `Select`/`OptionalSelect`/`Multiselect` take `label`, `aria_label`, `is_disabled` and `name`, and bind their value
  through the atom's `value`/`set_value` (no syncing effect); opening them panicked (`classes="leptonic-input
  search"`: whitespace in a class name), found by the new `select_components_tests`. The styled `TableHeaderCell` is
  focusable when pressable and takes `aria_sort` (`table_components_tests`). C1: the styled `Button`, `LinkButton`,
  `DateTimeInput` and `TiptapEditor` take `is_disabled` (was `disabled`; book call sites sent to its session).
  Open below (minus those):
  - Accessibility of styled components: `Select` has no `label`/`aria_label`/`name`/`is_disabled` (its Quick Start
    renders an unnamed select); `TabSelector` is a div with `on:click`, no tabindex, no `aria-selected`; sortable
    `TableHeaderCell`s aren't focusable; `Drawer` has no dismissal, focus containment or scroll lock; `Alert` and
    `Toast` have no role/live region, and `ToastTimeout::None` toasts close by mouse only; `Skeleton` animates without
    a reduced-motion rule; `ColorPicker`'s palette uses `use_move` instead of `use_color_area` (no role, name or
    tabindex); `AppBar` renders a `div` (should be `<header>`); the theme's `--link-color` (brand red) fails 4.5:1 on
    the light background.
  - Focus visibility: `use_slider_thumb`, `use_color_area` and `use_color_slider` expose no focus-visible state (their
    focused element is a visually hidden input), so styled demos can't show keyboard focus; `GridRow` lacks
    `data-focus-visible`, `GridCell` `data-focused`/`data-focus-visible`; `Hoverable` sets no `data-hovered`.
  - Behavior: color area arrow keys move 1 px (`use_move`) instead of the channel step; `use_focus_ring`'s
    `is_focused` stays true when disabled while focused; `use_spin_button` sets `aria-valuetext="undefined"` for
    `None` and `is_disabled` doesn't disable its steppers; `use_color_field` sets neither `value` nor
    `disabled`/`readonly` (the book demo works around it).
  - API shape: no `Default` for `UseInteractOutsideInput`, `UseScrollWheelInput`, `UseFocusInput`,
    `UseExitAnimationInput`, `UseOverlayInput`; `aria_label: Option<&'static str>` on color area/slider/wheel/fields;
    `ColorArea` atom has `&'static str` props, no `value` + `set_value`, no data attributes; `use_color_picker_state`
    and color slider/channel states take their input by reference; color slider state and hook both need
    `is_disabled`/`orientation`; `use_move` returns `Option` constraint parts that callers `expect`;
    `date_picker` dialog props lack `IntoAttrs`; `NumberFormatOptions` currency/unit are strings;
    `Announcement::LabelledBy(String)` takes a space-separated list; `use_prevent_scroll` returns empty props;
    `LinkButton` has an unused `active` prop; `disabled` vs `is_disabled` naming; `components::tabs::{use_tabs,
    TabsContent, TabSelectors, TabsContext}` should be `pub(crate)`; `atoms::prelude` lacks the grid, table, tabs,
    color area and color swatch atoms; a public clipboard utility (the book's copy button duplicates `Code`'s private
    `copy_to_clipboard`); `use_search_field` uses `e.target().and_then(..)` instead of `expect_target()`.
  - Stale or wrong doc comments: `has_label: true` examples (`use_field.rs:93`, `use_text_field.rs:414`,
    `use_progress_bar.rs:142`, `use_meter.rs:80`), the `use_exit_animation` example lacks `on_exit`,
    `atoms/radio.rs:28-29` and `:60`, `atoms/modal.rs:55` and `components/modal.rs:50` (`is_dismissable` and
    Escape), `atoms/table.rs:67`, `atoms/select.rs:97` (names a nonexistent `SelectLabel`).
  - Outdated plan entries: "the `ProgressBar` component renders no ARIA" (it is built on the atom now); R3k "icon
    aria-hidden" is done.
- Leftovers of the C4 state-prop sweep in library doc comments (2026-10-06, found by the book's wording pass):
  `atoms/checkbox.rs` says `state` doesn't apply inside a group; the `ModalBackdrop` example in `atoms/modal.rs` uses
  `state=is_open`; `ModalBackdrop`'s `is_dismissable` doc says it covers Escape (the book says outside clicks only:
  check which is true). `DateTimeInput` still takes `get`/`set` instead of `value`/`set_value`.
- More gaps found while writing the book's concept pages (2026-10-06): `DateTimeInput` panics when opened without
  a value (`get.get().unwrap()` in `components/datetime_input.rs`); `ColorPicker`'s `ColorPalette` uses `use_move`
  instead of `use_color_area` and has no role, name or tabindex (keyboard and screen reader users can't reach it);
  the `Select` component has no `label`/`description` props unlike TextField/SearchField/NumberField; the comment in
  `atoms/search_field.rs` ("As TextField: no controlled `value`") is stale; `use_clipboard` lives in `hooks::dnd` and
  needs no `clipboard` feature (the feature only enables the `Code` copy button); `DismissButton`'s label is fixed
  English; nothing uses `utils::DateTimeFormatter` yet. The doc comment of `use_breadcrumbs` says its props and
  `aria_label` belong to the navigation landmark, while the attrs type and the atom put them on the `<ol>` (the book
  follows the atom).
- Gaps found while writing the book's group overviews (2026-10-06): the `Toast` component has no role or live
  region (`Alert` has `role="alert"` now) (screen readers aren't told; apps must use the live announcer); `Drawer` has no dismissal, focus containment or scroll lock; the `Chip`
  component doesn't build on the tag group hooks and `ColorPicker` not on the color hooks; `components::card`,
  `tile` and `sanitized_html` were undocumented.
- Book (2026-10-05): `PopoverTrigger` (atom) spreads `aria-haspopup`/`aria-expanded`/`aria-controls` onto its
  wrapper `<div>`, not onto the button inside, so the button doesn't announce the popover (rework planned with
  `DialogTrigger`).
- Book (2026-10-05): done: modal pages (no `title`/`description`, `DialogTitle` heading, `<section>`, bound open
  state), Button atom data attributes and ARIA props, bindings on the Tabs/ListBox/Table/ComboBox/Select atom demos
  (`selected_key`, `selection`, `sort_descriptor` incl. `None`, `value`, `input_value`) and hook rows, `aria_label`
  rows as `MaybeProp<String>`, book search on the themed `SearchField`, popover hook demos with `use_button` +
  `use_overlay_trigger`, vertical toolbar demo with `use_button`; `/doc/hooks/use-color-channel-field` passes.
- Book (2026-10-05): new pages Field Atoms (`/doc/atoms/field`), Form Atom (`/doc/atoms/form`), Text Field atom and
  NumberField atom tabs; per-family part sections removed; `aria_label`/`validation_behavior` tables updated (checked
  against the library by a script); data attributes documented as `"true"`. The agent's separate target dir had
  grown to 154 GB (stale `deps` artifacts) and filled the disk; see the builds entry.
- Toggles (2026-10-05, from the book): `label.click()` (a virtual click, as assistive technology sends) didn't
  toggle a checkbox/switch: react-aria's `useToggle` leaves virtual label presses to the input but prevents the
  label's click. Ours toggles on them (documented deviation); read-only now also holds for bound states, and a
  rejected radio change restores the whole group's `checked`.
- Dialog (2026-10-05, from the book): closing a focused dialog panicked: removing it fires `focusout` after its
  owner was disposed, and the handler read a disposed `StoredValue`. DOM handlers that can run during unmount must
  use `try_*` accessors. `Dialog`'s `aria_label` is now rendered. Both covered by `test_dialog.rs`.
- Listbox safety net found: options in `disabled_keys` lacked `aria-disabled`; the listbox dropped the collection's
  focus handlers and tab index; collections listened to `focus`/`blur`, which don't bubble (now `focusin`/
  `focusout`); arrow keys always wrapped; select's `aria-controls` pointed at a non-existent id.
- Element ids were random (`Uuid::new_v4()`), so server and hydrated client disagreed. Now `utils::id::use_id`;
  `test_hydration_ids` compares server HTML ids with the hydrated DOM.
- Live announcer: replaced the unused context/provider design with react-aria's singleton.
- use_number_field prevented Enter's default (upstream #10200); use_button ran press handling before keyboard
  handlers; menu trigger + button were two press state machines; `is_virtual_pointer_event` misclassified desktop
  zero-pressure presses as screen reader clicks.
- Book (2026-10-05, leptonic-5e): the book is built with leptonic. Shell: app bar buttons are leptonic `Button`s; the
  small-screen menus are `Sheet`s on `ModalBackdrop`/`ModalContent`/`Dialog` (Escape, focus containment, inert
  page) instead of the legacy `Drawer` + manual scroll lock; search on `use_search_field` + `Shortcut`
  (Ctrl/Cmd+K), results as `Link`s, Enter opens the first, result count announced; copy button announces via the
  live announcer. Kit: `Disclosure` (`use_disclosure`) replaces `<details>`; `DocTable`/`ApiRow`/`KeyRow` render
  leptonic's `Table*` components; `Keys` renders `KbdKey` (a unit test checks every key name). Demos use leptonic's
  `Button`/`Checkbox`/`ToggleButtonGroup` for auxiliary controls; all stylesheets use the design tokens of
  `STYLE_GUIDE.md`. Library fixes prompted: Dialog panicked on close and dropped `aria_label`; `KeyboardKey` parses
  `Command`/`Option`/`Fn`.
- Book (2026-10-05): all checks build against the live library (no frozen copy). `kit::api_check` compares every
  Input/Return/Fields/Props table (each names its struct/component with `of`) with the library source. User rules:
  never show react-aria deviations in the book; the shared tree must always compile. Inline code wraps (8 pages
  overflowed at 390px, which an audit against the growing mobile layout viewport had missed).
- Book (2026-10-05): demo stylesheets are one file per sidebar section plus `_shared.scss` and concept files
  (verified by identical computed styles on 110 pages). Section overview pages generate their member tables from
  `nav.rs` (`SectionMembers`; every nav entry needs a summary, enforced by a test). `ReactAriaSource` kit component
  for react-aria parts without docs pages. Grid/Table atoms stay out of `atoms::prelude` on purpose (name clash with
  the legacy components prelude). Inline code in table cells overlapped when wrapping (theme's `1em` inline line
  height; fixed in `_article.scss`). Cells not selecting their row in cell focus mode is react-aria behavior (a cell
  with an action owns its presses; Space is not an action key). Demos with row actions use replace selection.
- Book (2026-10-04): all ~130 pages migrated to the kit; narrow screens stack table rows into cards. "View styles"
  shows only the rules (and keyframes) a demo uses.

## Book (until 2026-10-07)

The Book section of `PLAN.md` before it was reduced to open work, verbatim:


Page and writing rules: `documentation/documentation-strategy.md`; look, design tokens and which leptonic piece to use:
`examples/book-ssr/STYLE_GUIDE.md`. Library gaps the book hits go into the roadmap above (R3/R4) and, while the book
waits for them, under "Waiting on the library" below.

#### Verification loop
- Clippy (ssr + hydrate/wasm32), zero findings; unit tests (`cargo test --features ssr --lib`), including
  `kit::api_check` (every API table against the library source).
- Browser tests (`just book-browser-test`, `tests/`): page errors, dark theme, internal links and anchors, 390px
  width, Markdown export, on every page.
- Screenshots of affected pages when changing visuals (`just book-serve-isolated` for a second instance).
- Checks build against the live library, as the user's `just serve` does (separate target directory only).

#### Navigation restructure (decided and done 2026-10-06)
Rules: `documentation/documentation-strategy.md` ("Terminology", "Navigation", title rule); the sidebar is
`src/nav.rs` (parts Guides / Concepts / Building blocks, placement rules unit-tested), URLs in `src/routes.rs` (old
URLs redirect from `moved_*` modules). Concepts that are hook- or component-only today (Tree, Tag Group, Date
Field, Time Field, Color Field/Slider/Wheel, Alert, Toast, ...) get an overview and tabs once a second layer exists.

#### Next
- [x] Book audit fixes (2026-10-06): conventions in the strategy ("Prose", multi-item layer pages) and STYLE_GUIDE
  (§2 contrast tokens, §3 files per sidebar group, §5 Demos); demo stylesheets per group; shell, kit, search and
  chrome; every area's pages and demos. The interrupted areas were re-checked and their open items fixed.
- [x] Short Quick Start demos of their own for the Combobox, Grid, Grid List, Menu and Table overviews (2026-10-06);
  full book browser run green (26/26, incl. the new WCAG contrast check, which now skips collapsed sidebar groups).
- [x] Event Propagation guide: a demo with nested pressables (a card with a button whose press callbacks may
  continue propagation), now that `use_press` stops events like upstream.
- [x] Virtual focus (`utils/virtual_focus.rs`: `move_virtual_focus`, `get_virtually_focused_element`) is
  undocumented: Focus area or the Combobox Hooks tab. Done 2026-10-06: utility page `/doc/focus/virtual-focus`
  (virtual focus vs. roving tab index, `should_use_virtual_focus` in collection hooks, the four functions, demo,
  accessibility), linked from the Focus overview, Combobox Hooks and Collection State.
- [x] Color pages after the library's R3i rework: document the final color API (field/channel field atoms, swatch
  picker, picker state and atom, the Color type, the ColorPicker component) once the library session sends its
  summary. Done 2026-10-06: Color Field is a concept with overview, Hooks and Atoms tabs (Keyboard moved to the
  overview); new Color Swatch Picker concept (atoms, `/doc/color-swatch-picker`) and Color Picker Atom tab; the Color
  overview documents the color types, `ColorValue`/`ColorChannel`, `Color` and color names (with a parsing demo),
  which the pages link instead of the picker state page; slider/wheel atom tables use the channel generic `Ch`; the
  component page, previews and demos on `Color`; changelog (Color, ColorPicker props, conversion fixes).
- [ ] `Focusable` atom (new 2026-10-06, `atoms/focusable.rs`): a section next to `Pressable` (props table,
  custom tooltip trigger example, as upstream's "Custom trigger"); the FocusRing atom's new `is_text_input` and
  `data-focused` rows are in.
- [x] Overlay features from R3d (2026-10-06): context menus (Menu atoms section and demo, `use_context_menu` page in
  Interactions, since `use_button` and `PressResponder` use it too), submenu hooks on the Menu hooks page, subdialogs,
  `data-entering`/`data-exiting`/`data-trigger` and exit animations on the Popover, Modal and Tooltip atoms.
- [x] Shell tests (2026-10-05, `tests/ui_tests/test_shell.rs`): search (button, results, Escape clearing then
  closing, Enter opening the first result), Ctrl+K, the phone-width documentation menu (opens as a dialog with focus
  inside, Escape closes it and returns focus), a demo's "View source" (`aria-expanded`, code hidden/shown). Clicks
  below the fold must scroll instantly first: the book scrolls smoothly (`scroll-behavior: smooth`).
- [x] Date & Time on the re-ported date pickers (R3h, 2026-10-07): Date Field and Time Field are concepts with
  overview, Hooks and Atoms tabs (`/doc/date-field/{hook,atom}`, `/doc/time-field/{hook,atom}`; Time Field's tabs are
  `use_time_field_state` and the `TimeField` atom), new Date Picker Atoms tab (`DatePicker`, `DateRangePicker`, group,
  button, `DateInput part`); hook pages and demos on `hooks::datepicker` (the `time`/`jiff` conversion, the segment
  stopgap and the 22 "Limitations" items are gone: fixed by the re-port), Quick Starts on the atoms, Date & Time
  overview, Calendar Atoms composition, changelog. No page uses `hooks::datepicker_old` any more.
- [x] Date & Time after the library's fixes (2026-10-07): `DateInput`/`DatePickerGroup` group attributes documented
  and used, `DateSelector`'s error message shown only while invalid, Time Field Hooks page with `use_time_field`.
- [x] Calendar on the re-ported calendar (R3h, 2026-10-07): Calendar Hooks page and demos on `hooks::calendar`
  (focus-follow stopgap `calendar_focus.rs` removed), new Calendar Atoms tab (`/doc/calendar/atom`, single and
  two-month range demos), DateSelector page and demos (`initial_view` replaces Guide Mode, now `#views`), the
  Calendar overview's Quick Start on the atoms, the date picker demo's calendar on the atoms, changelog. When the
  library renames `calendar` → `calendar`, update the paths and the `of="calendar::..."` tags.
- [x] Parallel browser tests (2026-10-05): library suite `BROWSER_TEST_PARALLELISM` (default 4; 38 tests: 68s
  sequential, 35s at 2, 22.6s at 4 with the slowest test first; the server-panic check runs after all in a sequential
  runner, `ui_tests::after_all`); book checks run in parallel (default 3; 4m19s → 2m13s). The parallel book run made
  the Markdown export race deterministic: fixed, the index and search wait for `warm_markdown_cache`.
- [ ] Browser test performance (harness agent, 2026-10-05): session reuse saved ~3% (230ms per session vs 67ms per
  reset) and was dropped from browser-test (user); the cost was `wait_for_no_selector` waiting out the 3s implicit
  wait (fixed with `count_matching`, library suite 3m15s → 1m27s; direct `find_all(..).len()` absence checks in
  `test_menu.rs`/`test_select.rs` switched too). Book (2026-10-05): the two page walks are split into 8 shards each,
  links/anchors are checked once after all shards (`ui_tests::after_all`, shards record into a shared store), default
  parallelism a quarter of the CPU cores (2..=8): 3m04s → 51s on 32 cores. Open: implicit wait 0 + explicit waits
  only?; `table_resizing_tests` (16.6s) pointer chains and `expect_widths` uninstrumented; book page loads still
  hydrate debug wasm (~900ms each): a release/wasm-opt build for browser tests would cut that further. Step timing:
  `BROWSER_TEST_LOG_STEPS=1`.
- [x] `chrome-for-testing-manager` 0.13 vs 0.12: browser-test moved to 0.13 (the path patches apply again).
- [ ] Test servers stopped by an external SIGTERM (2026-10-05, three times): cargo-leptos logs "Received 'SIGTERM'
  shutdown signal" mid-run (cargo-leptos itself got a SIGTERM; our harness sends SIGINT). Twice two runs served
  from the same target dir at that moment; no name-based kills, no OOM daemon. Concurrent book runs in
  `target/browser-test` also break each other's builds ("Could not rename book-ssr_bg.wasm"). Never run two browser
  suites of one app at the same time.

- [x] Menu atoms page (2026-10-05, `/doc/menu/atom`, demo with an action menu and checkable items); the Menu
  overview's Quick Start uses the atoms; `PressResponder` page has the `shortcuts` row.
- [x] browser-test's nestable groups (2026-10-05): both harnesses make one run, `BrowserTests::sequential()` with
  the parallel group and a sequential "after all" group (the library's runs always, `run_always()`).
- [x] Overlay docs (2026-10-05): Popover atom page "Popover as Dialog" (incl. untitled dialogs named by the trigger);
  Modal atom page "Opened by a Button" (`DialogTrigger`, `ModalBackdrop`'s `state` now optional in the table);
  Popover/Modal concept pages mention `DialogTrigger` (the popover overview no longer claims non-modal by default);
  themed Popover's modality row; underlay prose on the popover/modal/overlay hook pages. New Tooltip atom page
  (`/doc/tooltip/atom`, demo with delay/warm-up and a disabled toggle); the Tooltip overview's Quick Start uses the
  atoms; the hook page's mode table says `TooltipTriggerMode`.
- [x] Toast, landmarks, DatePicker (2026-10-07): Toast is a concept (`/doc/toast`, tabs `/hook`, `/atom`,
  `/component`) with an overview (Quick Start on the component, accessibility, keyboard), Toast Hooks
  (`use_toast_state`/`ToastQueue`, `use_toast_region`, `use_toast`, demo), Toast Atoms (region and parts, demo,
  data attributes) and the rewritten component page (`push`/`close`/`clear`/`queue`, `ToastRoot`'s
  `max_visible_toasts`, timeouts). New building block `use_landmark` (`/doc/focus/use-landmark`: roles, demo,
  `LandmarkController`, F6 keys), linked from the Focus overview, the toast pages, Status and the Accessibility
  guide. The Date Picker Component tab documents the new `DatePicker` (props, values, limits and validation, views;
  the overview's Quick Start uses it) instead of `DateTimeInput`. Changelog updated.
- [ ] Book shell landmarks: register the app bar, the sidebar navigation, the main content and the table of contents
  with `use_landmark`, so that F6 / Shift+F6 / Alt+F6 move between them on every page (today only toast regions and
  the `use_landmark` demo are registered).
- [ ] The search test (`search_lists_results_clears_closes_and_opens_the_first`) times out waiting 10 s for results
  while the server still warms the Markdown cache (~17 s for 301 pages under load, 2026-10-07): wait for the warm-up
  explicitly or give it a longer timeout.

#### Waiting on the library
- [x] `Sheet` → hook-based Drawer (R3k): done 2026-10-07, the small-screen menus are leptonic `Drawer`s with the book's
  close button (`app::MenuCloseButton`); `sheet.rs` and its styles removed. The Drawer and Transitions pages are
  rewritten for the new APIs.
- Search results → Autocomplete with arrow keys through results (R3j).
- Calendar (R3h/R4): the `DateSelector`'s English labels ("choose a year", "Previous years") and the calendar strings
  wait for localized strings.
- Document once implemented: tree tables; the `Label` atom (renders a `for` pointing at a
  generated id nothing has, R3c).

#### Moved from the library's findings log (2026-10-07)
- Book (2026-10-06): `atoms/grid.rs` and `atoms/grid_list.rs` don't follow the atom-page structure (CSS examples
  with `rgba()`, plain "Escape"/"Enter" instead of `Keys`): rewrite. The Marks demo in `atoms/slider.rs` is inline in
  the page, without `source`: move it into `demos/`.
- Book: the Dockerfile builds from the repository root (path dependency on `../../leptonic`); not yet test-built.
  The Markdown middleware wraps the page router (it ran after axum's routing, so every `.md` request hit the 404
  fallback). The unfinished `/theme-editor` prototype was removed.

#### After the reduction (2026-10-07)
- Keyboard APIs: `use_global_shortcuts` page (`/doc/interactions/use-global-shortcuts`), Kbd as a concept
  (`/doc/kbd` overview, `/doc/kbd/atom` with `ShortcutKeys` and `Shortcut::keys`, `/doc/kbd/component`),
  `is_text_input`/`is_typing_target` on the focusability page; the book's search uses `use_global_shortcuts`.
- Landing page: the six cards replaced by an install line and three feature columns, content centered in the window.
- Shell landmarks (`MainLandmark`, `NavLandmark`; the skip link uses `LandmarkController::focus_main`), the
  `utils::scroll` page, the Themes page on the new `ThemeProvider` props, Drawer `width` and AppBar `aria_label`.
- Virtualizer pages (2026-10-07): `/doc/collection-state/virtualizer` (atoms `Virtualizer` and `VirtualList`, a
  10,000-option ListBox demo and a log demo following its end) and `/doc/collection-state/use-virtualizer-state`
  (the three hooks, `Layout`, `LayoutInfo`, a 100,000-row hook demo), linked from the ListBox pages.
- Off the components layer (2026-10-07): A — shell, kit (`Code`, `Keys`, `Icon`, `Link`, `DocTable`) and pages on
  atoms + book styles, `Root` → `ThemeProvider` + the book's toast queue; B — Component tabs removed (redirects to the
  atom pages' `#styling`), every demo on atoms + book CSS, styling sections on every atom page, recipes for the layout
  pieces, Chip → Tag Group, Kbd one atom page, Transitions removed, component stylesheet and `ComponentDemoContexts`
  removed; C — guides, changelog ("Removed: the components layer"), landing page, sidebar markers H/A, strategy and
  style guide describe hooks + atoms + the optional atom theme. "Copy as Markdown" downloads only on press (5-minute
  cache) and writes with `write_text_deferred`.


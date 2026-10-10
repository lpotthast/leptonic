# Leptonic: history

Finished work, moved out of `PLAN.md` (which holds open work only), condensed to what was done and decided, and
when. The rules that came out of it live in the documents `README.md` lists; git history has the details. Most
recent first within each part.

## Library: review of commit 07f628b1 against react-aria (2026-10-10)

The commit (1,254 files, another model's work) was reviewed area by area against react-spectrum `740c6c5c4a`.
Most of it held up; these defects were fixed:
- Interactions: merged long press groups ran a disabled group's callbacks; a disabled element's click propagates
  again (pointer and virtual clicks, as upstream; the comment claiming otherwise was wrong).
- Focus: `use_focus` compares the event target (`getEventTarget`) with the current target and the active
  element, so focus inside a shadow host's own shadow root is no longer the host's; the synthetic blur observer
  runs for every focus (Firefox: disabling a focused element with only `on_focus` dispatched no blur).
- Collections: listbox links follow the configured selection behavior (not the touch-selection `Toggle`);
  `ListBoxItems` no longer rebuilds every item's content when another item comes or goes (`Node::same_content`);
  label/description slot ids survive a content re-render; an earlier scroll frame is cancelled; submenu dialogs
  are labelled by their trigger item; rows and tags show `data-focus-visible` only while they have focus
  themselves.
- Tables: lazily mounted tree rows and cells no longer vanish or rebuild when a signal read while building them
  changes (`untrack`); select-all checkbox says "Select" in single selection; Grid/GridList/Table
  `aria_labelledby` back to `Option<String>` (C2); missing RAC data attributes on `TableCell`, `TableRow`,
  `Table`, `TableBody`, `GridListItem`; `data-tree-column="true"`; `Tag`/`TagItems` warn instead of panicking.
- Forms and pickers: number field `inputmode` tracks `min_value` and the format; a date-time range edited by day
  keeps its times; the enter animation hides a popover only until it is first ready (closing popovers were
  clipped away during their exit animation); read-only combo boxes let keys bubble; arrows wait for IME
  composition; a handled Select trigger arrow stops; `ComboBoxButton` `data-pressed` and `ComboBoxValue`
  `data-placeholder` as RAC; bound combo box text no longer follows a renamed option (upstream). `Input` and
  `TextArea` work outside a field as plain styled inputs (RAC). `labels()` treats an empty `aria_label` as none
  (calendar callers build upstream's label).
- Dead code and docs: unused element captures in the color hooks, stale doc examples, deviation headings, the
  `prevent_focus` deviation block; upstream headers synced (no-op commits); `upstream-drift.sh --mark-synced`
  works with BSD sed.
- Tests: 18 slider browser cases the commit added were never registered (one needed a fix to stay inside the
  viewport); the "Timed fruit" fixture and its full timer sequence are back.

The 42 browser failures blamed on Chrome 155 had host causes: the suite ran on macOS for the first time
(Ctrl-based select all, Ctrl+arrow and Ctrl-click are Meta/Alt there, as react-aria's `isCtrlKeyPressed` and
`isNonContiguousSelectionModifier`; Shift+F10 fires no `contextmenu`), Chrome 155 ignores the user-agent
override's `navigator.platform` (iPhone emulation now sets `userAgentData`), and a touch drag case raced the
200 ms drag delay. Tests now take the page's modifiers (`primary_modifier`, `non_contiguous_selection_modifier`,
`click_with_primary_modifier`), emulate `Platform::Linux` where they describe non-Mac behavior, and
`dispatch_all` sends timed sequences in one script. Four cases still fail with a German keyboard layout
(`PLAN.md`, "Testing infrastructure").

Then the leftovers, decided and fixed:
- Overlays: resize-triggered repositioning (`use_overlay_position`) and the popover's `--trigger-width` run in the
  next animation frame (`utils::next_frame::NextFrame`, coalesced, cancelled on close and cleanup): writing them in
  the `ResizeObserver` callback, as upstream does, resized the shallower popover during the browser's delivery,
  which reports "ResizeObserver loop completed with undelivered notifications" as an uncaught error (and in apps'
  error monitoring). Rule in `leptos-and-dom.md`, "Resize Observers".
- `aria_labelledby`: atoms take id lists as `Option<String>` (C2); hook inputs `Signal<Option<String>>` only where
  composing callers supply changing ids; `UseMenuInput`'s `MaybeProp` aligned. Rule in `conventions.md` C2.
- `TimeField` bounds: `TimeBound::{TimeOfDay(Time), Absolute(T::Absolute)}` (upstream's `TimeValue` bounds, typed;
  a `TimeField<Time>` takes only times of day).
- `GridList` `orientation` (prop, hook input, `data-orientation`, atom theme rules); table rows and cells always
  carry `data-level` (`--table-row-level`), as RAC; `is_tabbable` excludes only `tabindex="-1"` and both
  selectors equal upstream's.
- Browser tests type printable characters through CDP with US key definitions (`tests/pages/keyboard.rs`,
  `ElementActions::type_keys`), independent of the host's keyboard layout; chords and named keys stay on WebDriver.
- Book: removed APIs (`Shortcut::key`, state constructors, `OverlayTriggerState::from`), standalone
  `Input`/`TextArea`, `TimeBound`, GridList orientation, table/grid list data attributes, `VirtualList` wide rows,
  the changelog; its own shell tests detected Apple platforms case-sensitively.

Verification: 687 native tests, clippy (full, full+ssr, default, browser harness, test-app ssr and hydrate, book ssr
and hydrate), all 1,575 library browser cases, all 240 book browser cases and 47 book native tests.

## Library: `VirtualList` horizontal overflow, assertr 0.8 (2026-10-10)

- `VirtualList` scrolls horizontally when rows are wider than the list (agnite dev-ui: its log lines, which don't
  wrap, were cut off since it moved from a plain scrolling element to `VirtualList`). `ListLayout` gained
  `ListLayoutOptions::allows_overflow_across` (an addition, off by default): measured items overflowing their layout
  info widen the content to the widest one, items keep the view's breadth and stay visible wherever it scrolls
  across. Upstream (react-stately's `ListLayout`, still on `main`) keeps the content as wide as the view, its
  `ScrollView` then sets `overflow-x: hidden` (against scrollbar flicker while resizing), and the items'
  `contain: size layout style` keeps their overflow out of the scrollable area; both stay. The widest overflow is
  cached (recomputed only when the widest one narrows or goes), removed items are dropped with the cached layout
  nodes they belong to, and a new view width forgets them (the items are measured again), so appends cost no extra
  pass. Three native cases and two browser cases (`wide_lines_scroll_horizontally`,
  `wrapping_lines_ends_horizontal_scrolling`: a wrap toggle restyles the rows, observed rows are measured again);
  both browser cases failed with the option off.
- Tests adapted to assertr's latest API: extractors without the `get_` prefix (`ok`, `some`, ...), `Patience`'s
  `with_timeout`/`with_interval`/`with_consistency_duration`.
- Verification: 668 native tests passed (one ignored benchmark), strict clippy passed for `--tests`, `--tests
  --features full`, the browser harness and the test app. The browser suite (1,545 cases, Chrome for Testing
  155.0.8059.39, browser-test 0.6) passed except 42 keyboard and iOS-emulation cases, which also fail in fresh
  sessions and don't involve the virtualizer (`PLAN.md`, "Testing infrastructure").

## Library: focused PLAN fixes (2026-10-09)

- Slider percentage conversion rounds to the step, then clamps once, matching upstream. A range of 0..230 with
  step 20 converts 100% to 230; changing a thumb still snaps to a valid step. Integer and fractional regressions
  failed before the fix and cover clamped endpoints, below-midpoint rounding and unchanged setter behavior.
- Root ThemeProvider cleanup restores the document's previous `data-theme`, or removes it if absent initially.
  It preserves later external writes and leaves nested providers out of document ownership. Four browser cases
  cover removal/remount, restoration after a theme change, nested disposal and external writes; both missing
  cleanup behaviors were reproduced before the fix. Cleanup stores only Send-safe strings for SSR.
- SelectTrigger, SelectValue, SelectPopover and HiddenSelect warn and render nothing outside Select. Guards
  precede hooks and children; concrete Option views avoid adding erasure. Four SSR cases and a browser case
  first reproduced the panics, then passed. The component signature stays `impl IntoView` for Leptos debug erasure.
- Updated the corresponding book pages and atom implementation guidance. Removed the stale field-button
  focus-visible finding after confirming the shared button props and existing browser regressions for number
  steppers and the search clear button.
- Verification through Just: 665 native, 664 SSR and 2 theme tests passed (one intentional ignored benchmark in
  each library configuration), all 1,543 library browser cases passed (94.7 s), all 240 book browser cases passed
  (18.64 s), and rustdoc passed with warnings denied. Strict clippy passed for full, SSR, atoms without defaults,
  hooks without defaults, the browser harness and the test app's SSR/hydrate builds. Formatting and whitespace
  checks passed, as did the book's 47 native/API checks and 2 server asset tests. Theme storage replacement,
  Select auto-focus and other families' context guards remain in PLAN.

## Library and book: build warnings and canonical imports (2026-10-09)

- Removed the flat internal hook and utility re-exports. Library consumers now import hooks from their family,
  shared public types from the crate root, private helpers from their defining module, and dependency types from
  their crate. Public root exports refer directly to definitions; there is no internal convenience alias layer.
- Browser focus-scope tracking and item-measurement state compile only where they are used. The scope registry is
  absent from SSR and hooks-only builds while its pure native tests remain available; overlays without atoms
  retain their top-layer focus exemption. Atom-only context and default-class helpers are gated without hiding
  their native tests.
- Removed the book's unsupported `watch` and `env` cargo-leptos metadata. The remaining third-party
  `proc-macro-error2` future-incompatibility notice is tracked in PLAN with its dependency chain and upstream issue.
- Book verification exposed a date-picker SSR cleanup panic: local storage wrapped an empty DOM reference in a
  thread-bound value. Ordinary storage now holds an optional wrapped DOM element, leaving a movable `None` on the
  server. A native regression first reproduced cleanup on another thread, then passed with the fix.
- Verification through Just: native 663 passed, SSR 658 passed (one intentionally ignored benchmark each),
  theme 2 passed; date-picker browser cases 86/86 and book browser cases 240/240. Strict clippy checks passed for
  full, SSR, default and no-default feature sets, including the ordinary hooks-only library build. The book's
  development build has no source or cargo-leptos metadata warnings; the dependency notice remains.
- `just doc` passes with warnings denied after replacing three public rustdoc links to private implementation
  details with descriptions of the public behavior.

## Library: refactor integration completed (2026-10-09)

- Reconciled the in-progress hook/atom APIs with the test app, browser helpers and book: explicit input structs,
  family imports, typed values and callbacks, state setters, date/time generics, slots and attribute spreads.
- Collection rendering follows per-key node changes. Hidden selects retain selected values outside the current
  options, synchronize native selection reactively and map submitted values once; Select triggers stay pressed
  while open and popovers clear field/label contexts.
- Fixed combo-box selection against the original collection, removed-tab keyboard navigation terminating when
  all remaining tabs are disabled, and virtualizer reuse of measured section rows. Re-enabled its regression test
  and verified repeated appends and text-size changes.
- EventHandler uses a reusable native `Fn` listener, preserving owner, listener options and cleanup while allowing
  synchronous nested dispatch; empty handlers attach nothing. Focusable context handlers follow disabled-state
  changes; FocusScope containment and restoration respect the active scope tree and shadow-root focus.
- Space-triggered native clicks no longer duplicate press callbacks or suppress checkbox toggling. Context-menu
  labels follow the active trigger; drag state covers selected items and clears when the dragged source disappears.
- Inert overlay isolation also sets `aria-hidden`, preserving accessible names from referenced background labels;
  cleanup restores an author's previous `aria-hidden` value. The SVG/inert regression covers both behaviors.
- Date-picker fields preserve description attributes when combining press props, pass their label to segments,
  clear field contexts inside popovers and connect open buttons to their dialogs. Calendar focus clamps to current
  bounds; pressing a disabled navigation button preserves an unfinished range. Hour padding uses a single-digit
  reference hour rather than midnight in a twelve-hour clock.
- Optional slot references are published after hydration, so descriptions and error messages work before or
  after their controls in the markup; the initial client attribute now agrees with the server's absent reference.
  Date-range separator presses use the native event target to restore the correct segment's focus.
- Completed the calendar/date state attributes and upstream implementation headers, and consolidated redundant
  calendar/picker fixtures. The controlled-valid override remains an explicit API difference: `is_invalid` adds
  invalidity rather than hiding native validation errors.
- ThemeProvider follows the shared state-binding rules for setter-only and read-only controlled values, and
  suppresses notifications for unchanged themes. Storage API replacement and document-theme cleanup remain open.
- Corrected fixtures and browser helpers where migrations had changed native focus, labels, element structure,
  serialization or event order; retained behavioral assertions. Tests locate the deepest focused shadow element
  while still sending keys through WebDriver's native element command.
- Final verification: all 1,538 library browser cases passed (84.2 s), including the newly added slot-ordering
  case; native full 663 passed and full+SSR 657 passed (one intentionally ignored benchmark in each), theme 2
  passed, doctests 5 passed (143 examples explicitly marked ignored). The book passed 47 native/API checks,
  2 server asset tests and all 240 browser cases against the integrated tree.
- Checked the library's default, no-default, atoms+clipboard, full, full+SSR and WASM hydrate feature builds with
  clippy, plus the browser harness and both apps' SSR/hydrate builds. Repository formatting and whitespace checks
  passed. Further API redesigns, coverage additions, theme work and performance measurements remain in PLAN.

## Library: review fixes 2026-10-09 (Grid, Table, GridList, Tree)
- Tree/table expand button on a fresh page kept focus: `prevent_focus` now blurs the target when nothing had focus
  (`<body>` can't be refocused; upstream has the same bug); `tree::expand_button` left the known issues.
- GridList/Tree rows listen on `focusin` (React's `onFocus`): clicking a row's child makes the row the focused key.
- `calculate_column_sizes` compares signs like `Math.sign` (a column clamped at its minimum left the others
  overflowing); removed focused column headers refocus the same column of the first row.
- Section labels as `useLabels` (header via a slot, plus `aria_label`, reactive); "sortable column" description on
  sortable headers; the selection column derives from `show_selection_checkboxes` + selection mode
  (`TableCollection::with_selection_column`, `TableOptions` removed); cell-count/duplicate-column `dev_warn`.
- Doubled accessible names fixed: resizers labelled by the column's name element ("Resizer Name"), table expand
  buttons by "Expand/Collapse <row text>" (upstream's structure gives "Collapse Collapse Games" in Chrome).
- Tree hooks take `keyboard_navigation_behavior`, `should_select_on_press_up`, `focus_mode`,
  `allows_arrow_navigation`, `on_context_menu`; atoms `should_select_on_press_up`, reactive `aria_labelledby` (undone 2026-10-10, C2); a
  grid-layout `GridList` uses Tab navigation (RAC); `TableHeader` hover; `expect_context` → `dev_warn`.
- Performance: tree row positions once per table change, constant-time `(row, column)` cell lookups
  (`cell_key`), a columns-only memo (`TableState.columns`) for headers and the column layout, tree tables build
  rows only once shown.
- Tests: native (sign bug, fp rounding, later column smaller, removed column header, selection column); browser
  cases in `test_grid_list_cases.rs` (new fixture `/atoms/grid-list-cases`), tree (type-ahead, Home/End, no expand
  when selectable/action, empty tree, Escape none, press up, text inputs), table (selection column, removed header,
  Mod+A single, sortable description, header hover); upstream headers completed; native tests on `with_owner`.

## Library: review fixes 2026-10-09 (Focus)

- Modality: one global state (`use_focus_visible.rs`): `Option<Modality>` (no `Unknown`), the notified modality and
  per-kind (text input or not) notifications in `ArcRwSignal`s written only on change; no subscriber registry.
  `use_focus_visible` is a `Memo` over them (`is_disabled` and `modality` removed); `use_focus_ring` reads them only
  while focused and takes the modality at focus (fixes the stale value after a click); new `is_focus_visible()`
  (tracked). Per-item subscribers migrated (`use_option`, `use_menu_item`, `use_grid_list_item`, `use_tag`, Table/
  Grid contexts). The default window's tracking survives `beforeunload`; `add_window_focus_tracking(&Element) ->
  WindowFocusTracking` guard (waits for `DOMContentLoaded`), `tear_down_window_focus_tracking` removed; listeners
  use `utils::event_listeners`.
- FocusScope: active-scope tracking per react-aria (`focus_scope_tree::track_focus`; exact `shouldContainFocus`,
  `is_containing`; focus can't escape a containing scope to a sibling or ancestor scope); `first_in_scope`
  (tabbable, else focusable) for auto focus, recapture and restore fallback; recapture to the active scope's first;
  restore Tab handler skips while a containing scope is in between; pending focusout frame cancelled on cleanup /
  `contain` off; shadow-aware active element, targets and containment; `restore_focus: Signal<bool>`; typed
  `RestoreFocusEvent` (const removed); provides `FocusManager` directly (`FocusScopeContext` removed,
  `use_focus_manager_context()`); `focus_scope_tree` crate-private; `is_element_in_child_of_active_scope` includes
  the active scope.
- Focus manager: `FocusManager` is `Copy` (state in a `StoredValue`), `create_focus_manager()` (was
  `use_focus_manager(UseFocusManagerInput)`), `from` only positions the walker (container elements work),
  `focusability: Focusability::{Focusable, Tabbable}`; toolbar/radio group create theirs once (no per-key arena
  leak). Tree walker: tri-state filter (`NodeFilterResult`), radios deduped by the walker's current node, `from`
  option removed (no upstream caller).
- `use_focus_ring`: `target: FocusRingTarget` (was `within: bool`), `auto_focus` removed (no effect upstream; also
  on the `FocusRing` atom). `use_focus`: target/current-target checks as upstream (they weren't; fixed 2026-10-10), blur observer only with a blur
  callback. `use_has_tabbable_child` writes only on change; `Signal::stored` defaults. `is_tabbable` reads the
  `tabindex` attribute. Headers (incl. upstream test files) completed; deviation blocks for `Focusable`/`FocusRing`;
  stale doc examples fixed; unused `focus_html_element`/`get_radio_group_name` removed.
- Tests added (verified in the completion run above): FocusScope multiple scopes, non-tabbable/contenteditable skipping, modifier
  Tab, restore after children change, skip-after-trigger, no Tab without restore, DOM order, auto focus kept /
  fallback, focusable-first-in-scope, portal child (with/without contain), child active regardless of DOM, correct
  scope on unmount, 4 stacked-dialog variants, 4 shadow-DOM cases; focus manager of a FocusScope and container
  elements; focus visible beforeunload and 4 other-window (iframe) cases; focusSafely and useFocus in shadow DOM;
  native tests for the modality signals and `shouldContainFocus`. Fixtures merged (focus manager wrap/accept/
  outside scopes, FocusScope containment = select-on-tab scope).

## Library: review fixes 2026-10-09 (Buttons, links, disclosure, tabs, toolbar, small widgets)
- One way for id lists: `IdRefs` (`IdRefs::derive([..])`, `collect::<IdRefs>()`; each id once) replaces
  `AriaDescribedby`, `AriaControls`, `join_slot_ids`, `dedup_ids` and every hand-written join (button, link, menu
  item, field, text field, slider thumb, calendar cell, select, date field/picker, table, column header, toggle,
  radio, checkbox group, `Focusable`); `utils::labels` (react-aria's `useLabels`) replaces `use_label::labels` and
  labels the tab list, tab panels (new `aria_label`), `DismissButton`, calendars and date segments.
- `hooks/merged/*`, `MergeWith`/`MergeWithExt` deleted: independent hooks are each spread onto the element;
  `UsePopoverProps { overlay, position }`; docs updated (hooks-implementation.md, conventions.md).
- Ids known while rendering: `PressResponderTrigger.id: RwSignal<String>` (the trigger's generated id, replaced by
  the pressable's own: `use_button` while rendering, `Pressable`/`use_link` once mounted); `Button` has an `id` prop
  (default: generated); disclosure panels, untitled dialogs, popovers and menus reference it in the server HTML;
  `ensure_element_id`, `DialogTriggerContext::ensure_trigger_id` deleted; `<Button attr:id>` migrated to `id`.
- Tabs: `TabListState` holds the tab/panel ids and the collection id (`TabListData`/`TabListItemData` deleted;
  hooks take `state`); `TabListState` is the atoms' context (RAC `TabListStateContext`); `use_tab` takes a
  `FocusableContext` (tooltips on tabs) and focus callbacks (`UseFocusableItemProps`, moved from the column header
  to `use_focusable`); `Tabs` root `data-focused`/`data-focus-visible`/`data-disabled`; `TabPanel` enter/exit
  animations (`data-entering`/`data-exiting`), `aria_label`; `Tab` hover/focus callbacks; disabled keys a `Memo`;
  missing context warns instead of panicking.
- Disclosure: a panel rendered again picks up the current state (`hidden_until_found` per render,
  `manage_panel` tracks the element); grouped `is_expanded` a `Memo`; `DisclosureState` in the context; panel role
  without writing a signal during render; group `expansion` a signal; group state tests on `with_owner`.
- Link: one `use_link` per `Link` (`element_type` a signal); `target` signals on `use_link`/`use_button`/`Link`;
  `link_rel` (no `noopener` with `Opener`); context `on_press` first; `UseLinkInput`/`Link` focus, key, hover-change,
  press-start/end/change callbacks and `auto_focus`.
- Button: `aria_disabled` passthrough, pending anchors drop `target`, per-element-type constants,
  `prevent_focus_on_press`/`allow_focus_when_disabled` signals, deviation block completed.
- ToggleButton: no panic without `value` in a group (warns, standalone), press/hover/focus/key callbacks,
  `auto_focus`, `prevent_focus_on_press`, ARIA relations; toolbar hook takes its `element`.
- `ProgressBarFill`/`MeterFill` set `--percent`; parts warn outside their parent; the deliberately unlabelled
  progress bar (and its fixture expectation) removed; typed-values conversion a `Memo`.
- Tests written (verified in the completion run above): tabs keyboard delegate (native), tab list state (with_owner + flush), IdRefs,
  labels, link_rel; browser: disclosure remount and merged Report/Group fixture, theme setter/controlled, progress
  zero range, meter ranges, default classes + `--percent`, toolbar classes/orientation/dividers/aria-label, tabs
  (disabled tabs, default key, Home/End, panel label, RTL, Space, panel tab stop, tooltip, roving tabindex, root
  state).

## Library: review fixes 2026-10-09 (Color and slider)
- Bugs (native failing tests first): HSV/HSL → RGB wrap hues (`rem_euclid`; 450° chartreuse, −30° rose); the
  area/wheel/color-slider states step from the current value instead of an Effect-updated `latest` (stale between
  an app change and the flush, never in SSR, and a rejected color was not re-sent); `latest` only for
  `on_change_end`, reset at drag start; color field: `#rgba` parses (alpha), committing an empty field reports no
  change; color slider value text names the display color (hue at full saturation, opaque color).
- Fidelity: `ColorArea` defaults to white, `ColorWheel` to `hsl(0, 100%, 50%)`, `ColorSwatchPicker` to black;
  no `aria-disabled` on the area group; color slider thumb `forced-color-adjust: none`; hover as RAC (disabled
  sliders' tracks/fills and color thumbs hover; a slider thumb only by its own `is_disabled`).
- Performance: channel formatters and color-name strings cached per locale (`utils/color`); `channels()` is an
  array.
- API: C13 (`Fraction` in `SliderState` percent methods, marks; `Point` in area/wheel states); wheel radii
  `Signal<f64>` + `aria_errormessage`; `SliderPopover::{Never, OnHover, OnDrag, OnHoverOrDrag, Always}`; hook
  marks types renamed `SliderMarkPlacement`/`CustomSliderMark`, `SliderMarkValue::Fraction`;
  `UseColorChannelFieldInput` flat (no throw-away number state); `ColorValue::gradient_stops` replaces
  `ColorChannelRange.gradient_stops`; `ColorArea` `x_channel_step`/`y_channel_step`/`aria_details`,
  `ColorWheel`/`ColorSlider` `aria_describedby`/`aria_details`; `expect_context` → `dev_warn` + nothing
  (ColorThumb, ColorSlider parts, ColorWheelTrack, ColorSwatchPickerItems).
- Tests: native ports of useSliderState, useColorFieldState and Slider.test page size/clamping; native tests on
  `testing::with_owner`; `/atoms/slider-interactions` in sections (the missing-value warning only in its
  section, `missing_value` takes it), redundant instances merged. Many browser cases added and verified in the completion run above.

## Library: review fixes 2026-10-09 (cross-cutting)
- Dead API deleted: `Language`, `Mount`, `ViewProducer`/`ViewCallback` (`utils/callback.rs`), crate-root CSS aliases,
  `track_in_local_storage` (`read_from_local_storage` private), `Out::Fn`/`new_fn`/`Default`, `AriaGrabbed`; the
  `hooks` feature (it gated nothing) removed; feature comment typos fixed.
- No prelude, one path per public item (the user's decision): `leptonic::prelude`/`atoms::prelude` deleted; hook
  families are public modules (`leptonic::hooks::<family>::X`; implementation submodules crate-private,
  flat hook re-exports removed); `utils` private, user-facing items re-exported flat at the crate root (`leptonic::Locale`,
  `leptonic::IntoAttrs` (moved to private `attrs.rs`), `leptonic::leptos_styles`/`leptos_classes`, ...); every import
  in leptonic, test-app, tests and book migrated by script.
- `utils::{style,styles,css}` merged into private `utils::styles`; `utils/locale.rs` merged into `i18n.rs`;
  RTL from ICU4X `LocaleDirectionality` (Mend, Rohg now RTL; native tests); `Locale.direction` private + accessor;
  `I18nContext` `Copy` with `set_locale()` method; i18n header fixed.
- Atom API: Calendar/RangeCalendar `on_focused_value_change`; Tabs `on_selected_key_change`; ComboBox
  `on_input_value_change`; Table `on_sort_descriptor_change: Callback<Option<SortDescriptor>>` via `from_state_props`;
  `default_selected_keys: Vec<Key>` -> `default_selection: Selection` (ListBox, GridList, Grid, Table, TagGroup, Menu,
  MenuSection); DisclosureGroup `default_expanded_keys: HashSet` (hook too); reactive `orientation` (RadioGroup,
  ListBox, ToggleButtonGroup; list hooks take `Signal<Orientation>`), `selection_behavior` (Grid, Table),
  `disallow_empty_selection` (6 atoms, toggle group state); `Breadcrumb`/`Disclosure` `key`; `*Ctx` -> `*Context`;
  `Menu.aria_labelledby: Option<String>`; `dev_warn!` for an overlay in a `DialogTrigger` ignoring its `set_open`.
- C3/C8/C12: `UseScrollViewReturn.scroller: ScrollViewScroller` (methods); `UseGridKeyboardDelegateInput`,
  `UseTableKeyboardDelegateInput`, `UseListStateViewInput`, `UseToastStateInput`, `UseListCollectionInput`; overlay
  arrow role typed.
- Hydration: `use_platform_check` (false until mounted) for the date segment's iOS role, the color area's mobile
  inputs and the number field's iOS role description/input mode (`UseTextFieldInput.input_mode` is a signal).

## Library: review fixes 2026-10-09 (Virtualizer)

- `ListLayout` (native benchmark `list_layout::tests::timing_a_long_list`, 20,000 rows estimated, release; before →
  after, layout only): append at the top 2.6 → 0.4 ms (collection build ~9.3 ms excluded), append at the end with
  everything laid out 11.9 → ~5.6 ms, scroll frame over a fully laid out list 1.42 ms → 3.5 µs, scrolling down a
  fresh list one view 1.87 ms → 13 µs. Visible root nodes by binary search plus a side list (headers, loaders,
  sticky, persisted keys + ancestors computed once per call); continuation of a partial build when only the end of
  the requested area moves; content-only node comparison (index/position/siblings ignored: a log dropping lines
  keeps measured sizes); sections rebuilt when the collection changes; top-level count/loaders counted once per
  collection; removal pass skipped when the build visited every cached node; one hash lookup per cached node.
- UPSTREAM BUGS FIXED: `buildSectionHeader` subtracted the position twice (headers narrower than rows; test
  `section_headers_are_as_wide_as_the_rows`); `buildSection`'s skip check uses the row (test still open, PLAN).
- API: `ItemSize { Fixed, Estimated }` for `row_size`/`heading_size`; `anchor_to_end: Option<EndAnchor { threshold }>`
  (warns in horizontal lists); `VirtualList` takes `VirtualListOptions { row_size, gap, padding, end_threshold }`.
- Measuring: `use_item_measurer` → `ItemMeasurer` shared by a collection's items: estimated items measured together
  in one microtask (reads only), one `ResizeObserver` per collection observing a `display: flow-root` content wrapper
  per item (text-only rows observed; new rows observed in the next frame, avoiding the RO loop error).
- `Virtualizer`: no `is_scrolling` reordering or scroll-end pass (renderer keeps rows in place); `is_scrolling`
  read untracked; clock `performance.now()`.
- Tests: `ScrollExtent` (`ElementActions::scroll_extent`), `tests/fixtures/virtual_list.rs` (`VirtualListActions`).

## Library: review fixes 2026-10-09 (Drag and drop, clipboard)

- App callbacks run untracked: the drag manager's session listeners, its first-frame setup and its mutation observer
  run in `untrack` (as Leptos runs `on:` handlers), `DroppableCollectionState::get_drop_operation` is untracked, the
  clipboard's document listeners and the post-drop focus update too; the "signal read outside a reactive context"
  warnings on `/hooks/dnd*` are gone (the `dnd_targets` fixture's `get_drop_operation` still reads a signal).
- Bugs: `utils::clipboard` returns `Unavailable` when `navigator.clipboard` is undefined (insecure contexts) instead of
  throwing; `use_drag` survives `drag`/`dragend` sent to a removed drag source (`try_*`, one end per drag through a
  shared `NativeDrag`), and a `dragend` before the first frame keeps the source from being marked dragging (drag
  generation); `begin_dragging` returns `Err(DragInProgress)` while a drag is set up, so a second Enter starts no
  second drag (no `on_drag_start`, no console error).
- Performance: one `Memo` of the dragged keys per draggable collection (`drag_count`), session drag types computed
  once (`DragSessionInfo.types`), droppable items' validity a `Memo`, borrowing `DropOperationEvent<'_>` with shared
  dragging keys, `use_drop`'s `dragover` reads the target's rect and the drag types once, `ListDropTargetDelegate`
  caches the item keys per collection.
- Fidelity: shadow-DOM-aware event targets (`event_target_element`) and `expect_current_target` in `use_drag`,
  `use_drop` and the drag manager; a disabled drop button isn't described; the post-drop selection includes nested
  (tree) items; auto scroll starts in an animation frame; `ListDropTargetDelegate::new` no longer calls a hook (the
  direction is passed to `drop_target_from_point`); `use_clipboard` moved to `hooks::clipboard`; fixtures write
  `data-dragging`/`data-drop-target` as `"true"`/absent.
- Tests: `test_dnd_native.rs` (46 cases of dnd.test.js: events, nested sources and targets, drag data, files and
  directories, drop operations, drag previews, keyboard data and nested targets, the bug cases) on the new
  `/hooks/dnd-native` fixture with a stand-in `DataTransfer` (`DndActions::begin_external_drag`, typed `transfer()`);
  `test_dnd_draggable_collection.rs` (all 13 cases of useDraggableCollection.test.js) on
  `/hooks/dnd-draggable-collection`; 6 screen reader cases (TalkBack on Android, targets added/removed/hidden),
  4 screen reader cases of useDroppableCollection; clipboard writes denied and without a clipboard
  (`fixtures/clipboard.rs`); native tests for backward horizontal drop target navigation, touch texts and
  `drag_count`. `Platform::Android`, `SyntheticEvent::detail`/`size` added to the test helpers.

## Library: review fixes 2026-10-09 (ComboBox, Menu, TagGroup)

- Context clearing (RAC `clearContexts`), one mechanism: `utils/scoped_context.rs` `clear_context::<T>()` /
  `use_clearable_context::<T>()` / `ClearContexts` (a cleared context remembers the provided value it hides;
  providers stay unchanged). `PopoverParts.clear_contexts` and `PopoverDefaults.clear_contexts` apply it to a
  popover's content; `ComboBoxPopover` clears `LabelContext`, `FieldContext`, `InputContext`; a `ListBox` clears
  `ListBoxParent` for its descendants. `Label`, `Description`, `FieldError`, `Input`, `TextArea` and `ListBox`
  read through it; `Input`/`TextArea` without a field `dev_warn` and render nothing (no panic).
- Combo box: a server-filtered one (`filter: None`) opens without options (`can_open`: react-aria's `|| items`);
  `ComboBoxButton` is `data-pressed` while open, renders `data-focused`/`data-focus-visible`, is captured
  (`UseComboBoxInput.button_element`; no `get_element_by_id` per event); the popover's `--trigger-width` spans input
  and button (`PopoverParts.width_with`), re-measured on resize; Escape no longer prevents the default; arrow keys
  act on repeats (Enter/Tab/Escape don't); button and listbox labelled through `labels()` (useLabels);
  `menu_trigger`/`allows_custom_value`/`allows_empty_collection` are signals; `should_focus_wrap` prop; new
  `ComboBoxValue` atom; closing snapshots the shown options before the text resets (upstream's frozen collection).
  `ComboBoxState` holds its `Copy` settings directly (no `StoredValue` lock per read) and the value as a `Memo`.
- Menu: hovering any expanded trigger keeps focus in its submenu; submenu levels come from the menu
  (`MenuData::submenu_level`, `UseSubmenuTriggerStateInput.level`); `open_submenu`/`close_submenu` notify only on
  change; section headings follow the collection; `Menu` props `should_focus_wrap` (default true),
  `disallow_empty_selection`, `empty_state`; `MenuItem` `data-hovered`; slot attributes cloned per render; the
  dead `ContextMenuTrigger` `target_id` state and its `UseMenuTriggerMenuProps` in the trigger context are gone;
  typed `KeyboardKey` in `use_menu_item`; EventAccessors in `use_submenu_trigger`/`use_menu_trigger`.
- Tags: typed `AriaAtomic`/`AriaRelevant`; no keyboard handlers without `on_remove`; the remove description
  treats a virtual modality on touch devices as pointer; `TagList` re-renders keep its attributes;
  `TagRemoveButton` renders `data-focused`/`data-focus-visible`/`data-disabled`.
- Tests: native combo box state cases of useComboBoxState.test.js (open/toggle reasons, async loading, frozen
  collection, ...), submenu levels, menu trigger change notifications, context clearing; browser cases for the
  combo box (pressed button, toggling, read-only, disabled, scrolling, clear contexts, server filtering, trigger
  width, form reset, section headers, ...), menu states (`/atoms/menu-states`), press-drag-release, submenu
  sections, nested subdialogs, menu trigger hook (disabled, long press) and tag groups (hover, press, disabled
  removal, actions, order, RTL, keyboard selection, held Backspace). Helpers: `ElementActions::count_synthetic_events`,
  `style_property`, `double_click`. Fixtures: TimedComboBox merged into ControlledComboBox; unused buttons removed.
- Upstream headers of every module of the family list their source and test files.

## Library: review fixes 2026-10-09 (Interactions)

- `use_press`: the click after a long press is prevented (a long-pressed link doesn't navigate, a submit button
  doesn't submit); long press start and its timer moved into the press start (re-entering restarts it, one end per
  start); coordinates relative to the pressed element for every event (`point: Point`, keyboard and synthetic: its
  center); `type_()` for input/button key checks (a text `<input>` without `type` no longer presses on Space/Enter);
  a script click during a dragged-out pointer press ends that press instead of pressing as virtual; the virtual
  pointer flag resets on every real pointer down; several press hooks on one link open it once; no user callback
  runs under the state borrow; no long press machinery (description, listeners) without a `LongPress` group.
- APIs: `EventHandler::into_on` gives `OnEvent<D>` (nothing attached for empty handlers; `to_on` gone);
  `PointerType` is `Copy` (`Unknown`, `PointerType::of`); `PressEventKind`/`LongPressEventKind`, `Element` targets
  (press, long press, hover); `LongPress` group (C11 signals, merged through `PressResponder`); `Shortcut` keyed by
  `KeyboardKey` (case-insensitive key names in `parse`); `InteractOutsideEvent`; `ScrollEvent` in pixels for every
  `deltaMode`; `EventWrapper` folded into `KeyboardEventWrapper`, one `Arc<AtomicBool>` per `PropagationControl`;
  `prevent_focus` returns a `FocusPrevention` disposed at press end/unmount; `use_move` uses the target's window,
  unknown pointers move as a mouse, no `CapturedElement`; `use_prevent_scroll` moved to `hooks::overlay`;
  `Pressable` takes every press prop, `Hoverable` `on_hover_change` and `TypedChildren`. `use_keyboard`'s propagation
  with shortcuts documented as a deviation (one model for wrapped handlers).
- Tests: usePress cases (touch, the click fallback, left button only, pointer/keyboard modifiers, coordinates for
  mouse, touch, keyboard and cancel, focus on click, VoiceOver/Android/TalkBack virtual pointers, links and the link
  role, checkboxes, re-entrancy, iOS page `user-select`, style changes during a press), the bug cases above, long
  press click prevention and re-entry, hover after touch, Pressable role warnings, Android context menu, scroll wheel
  (new fixture), useKeyboard's Mac/Windows shortcut matching and createKeyboardShortcutHandler parsing (native).

## Library: review fixes 2026-10-09 (Collections core, ListBox, Select)

- Per-key notification in `SelectionManager`: `is_selected(key)`/`is_focused_key(key)` subscribe their reader to
  that key only (ref-counted `ArcTrigger`s); `set_focused_key` notifies the old and new key, selection changes the
  changed keys (synchronously; a bound selection changed by the app through an Effect). `has_focused_key()` and
  `is_empty()` are memos. Item hooks read focus key-first (only the focused item tracks `is_focused()`), item flags
  are `Memo`s; tag tabindex, table header focus and draggable items migrated.
- Keyboard delegates are memos (`keyboard_delegate_memo`: list, grid, table, tabs); the arrow wrap target is computed
  only when wrapping.
- `FilterQuery` (normalized once) for `Filter::contains/starts_with/ends_with` (no per-call allocations);
  `ComboBoxFilter` is `Fn(input) -> OptionMatcher`, so `use_contains_filter` prepares the input once per keystroke.
- `Key::id_fragment` (unambiguous, no whitespace) for every key-derived element id (options, grid list rows, tabs,
  table headers and cells); integer keys hold `i128` (`From<u64>`, `SelectionValue for u64`/`isize`).
- Select: single mode keeps one key of a longer default or bound value; trigger ArrowLeft/Right repeat while held.
- Collections: auto focus sorts the selected keys (deviation recorded), the scroll frame is cancelled on focus change
  and unmount; `use_listbox`'s link behavior follows the selection behavior (`link_behavior: Signal<LinkBehavior>`
  in `CollectionOptions`, `UseSelectableItemInput`, `GridListData`); section heading text and `UseOptionReturn.link`
  follow the collection; `ListBox` atoms `dev_warn` instead of panicking outside their parents.
- `use_list_collection`'s `text_value` returns any `Into<Arc<str>>`; explicit `CloseOnSelect`/`SelectOnFocus`/
  `SelectOnPressUp` enums (no macro); upstream headers incl. test files; native tests on `with_owner`; new native
  tests (per-key notification, grid edges, horizontal grid, collection rebuilds, single-mode select).
- New fixtures/cases (verified in the completion run above): `/atoms/listbox-selection` (23 cases), `/atoms/select-behavior`
  (25 cases); `#sf-empty` merged into `#sf-unloaded`.

## Library: review fixes 2026-10-09 (Calendar and date pickers)

- Date fields format the Gregorian date they edit in every locale (`format.rs` forces `CalendarAlgorithm::Gregory`):
  th-TH, fa-IR and `-u-ca-` locales showed Buddhist/Persian years in descriptions and min/max errors.
- `DatePicker<DateTime>` with `granularity = Day` keeps the value's time when a date is selected (was midnight).
- `DateRangePicker`: a value emptied from outside clears the shown ends at once (derived, no `Effect::watch`).
- `DateTimeFormatter`'s 12-hour clock is the locale's (`Clock12`: h11 in ja-JP), as the date field's.
- First day of the week: Monday for `-u-ca-iso8601` (unless `-fw-`); upstream's fr/fr-CA/fr-FR cases tested.
- `TimeField`/`use_time_field_state` bounds are times of day (`civil::Time`) on the value's day (react-aria's
  `convertValue(minValue, day)`): a `TimeField<DateTime>` can say "not before 09:00" on any day.
- Typed picker state contexts: `use_date_picker_state_context::<V>()`, `use_date_range_picker_state_context::<V>()`
  (RAC `DatePickerStateContext`), e.g. for a `TimeField` in the popover.
- `DateFieldPicker` carries the picker's `labelledby`/`describedby`: the field labels its segments with them (callers
  no longer patch `DateFieldData`), and the picker's inner field no longer warns about a missing label.
- Memoized: field/picker formatters (`format::Formatters`, built on first use), segment number formatters,
  validation (`Memo`, limits formatted only when violated), the segment's month/hour text formatters, the calendar
  heading's and year picker's formatters. Month/year picker labels localized (`aria_label: Signal<String>`).
- `ListFormatOptions::kind` (was `r#type`), `ListFormatter::new` falls back to the root locale's list of the same
  type; `plurals.rs` and `list_formatter.rs` headers and deviation blocks; hidden date input `aria-hidden` typed.
- Native tests of `hooks/{calendar,datepicker}` on `testing::with_owner` (19 sites); new native tests mirror
  `@internationalized/date`'s manipulation, queries and conversion (DST) tests.

## Library: review fixes 2026-10-09 (Overlays)

- Hide-until-placed styles of `use_enter_animation` are returned (`UseEnterAnimationReturn::styles`) and merged into
  the popover's and tooltip's `Styles` (one writer of `style`, `lessons.md`): before, a reactive part (the popover's
  `--trigger-width`) wiped them, and a popover whose positioning paused showed unplaced at (0,0).
- Landmark navigation skips landmarks inside `[inert]` (the page behind our modals), as inside `[aria-hidden]`.
- `aria_hide_outside` in `HideMode::Inert` hides non-HTML elements (SVG) with `aria-hidden`, as upstream; its node
  sets are JavaScript `Set`s (one call per membership check instead of one per compared element).
- `DismissButton` renders `type="button"` (no longer submits an enclosing form; addition).
- API: `use_modal` deleted (legacy `ModalProvider` API; the book's demos use `use_modal_backdrop` alone); one
  `OverlayPositionOptions` (with `Default`) in `UseOverlayPositionInput` and `UsePopoverInput` (field `position`);
  `UsePopoverInput::scroll`, passed the list box by `Select`/`ComboBox` and the menu by `MenuTrigger`'s popover;
  `Popover` props `boundary`, `should_update_position`, `target_rect`, `on_enter`, `on_exit`; `ModalBackdrop`/
  `ModalContent` `on_enter`/`on_exit`; `use_modal_backdrop` generic over `S: OverlayState`; `ToastKey(u64)` toast
  keys; `Popover` without a trigger and `ModalContent` outside a `ModalBackdrop` `dev_warn` and render nothing;
  `Debug` on the overlay hooks' returns; stale comments fixed; `// Upstream:` headers name the upstream tests.
- Tests: page-clock timing for the tooltip and toast timers (`StopwatchStart` from plain events, `StopwatchEnd::
  Disappears`); new helpers `Page::click_at`, `Page::dispatch_to` (window/document), `Page::record_attr_of`; new
  fixtures `overlay-position-options` and `overlay-state`; cases for the positioning options (props change, window
  resize, max height, target margin, cross offset, RTL start/end, target rect, arrow boundary offset, boundary),
  hidden until placed, document/window scroll, modal popovers on scroll, own open state inside a `DialogTrigger`,
  standalone popovers, popovers in modals, the modal's outside filter, enter/exit callbacks, auto focus in a modal
  opened from a menu, useDialog's descriptions, title warnings and shadow-root focus, useTooltip's hover cases,
  landmark backward wrap and Shift+F6; `aria_hide_outside::mutations` split into its six upstream cases; popover
  cases use the "Toggled" instance (the "Info" one is gone).

## Library

### Review 2026-10-09: findings fixed the same day

Found done when `PLAN.md` was condensed (2026-10-09, checked against the code); the open rest is in `PLAN.md`.

- Cross-cutting: empty event handlers attach no DOM listener; `IdRefs` and `labels` (useLabels) join id lists;
  `ClearContexts`/`clear_context` keep contexts out of popovers (ComboBox, nested ListBox); EventAccessors and
  shadow-aware `get_event_target` in the combo box, menu and DnD hooks; RTL from `LocaleDirectionality`; dead API
  removed (`Language`, `Mount`, `ViewCallback`/`ViewProducer`, the CSS aliases, `track_in_local_storage`,
  `Out::Fn`, `MergeWith`/`hooks/merged`, `AriaGrabbed`, the `hooks` feature); atom API consistency (C4 names,
  `default_selection: Selection`, `HashSet` set values, C11 signals, `key` props, `*Context` names); the C recount's
  C3/C8/C10/C12 cases; `Signal::derive` → `Memo` and per-locale ICU4X caches in forms, number field, dates, tabs,
  color; `KNOWN_PAGE_PROBLEMS` replaced by exact declared expectations (`tests/fixtures/expectations.rs`).
- Interactions and focus: `use_press` (click prevented after a long press, long press on re-entry, `point`
  coordinates relative to the pressed element, `type_()` key checks, `LINK_CLICKED` guard, no callbacks under a state
  borrow, `Option<LongPress>`), typed `PressEvent`/`PointerType`/`InteractOutsideEvent`, `Pressable` with every press
  prop; FocusScope containment (`should_contain_focus`), one `first_in_scope`, modality tracking deduped and kept
  across `beforeunload`, `FocusManager` context API, RAII window tracking, typed focus options; their upstream tests.
- Overlays: hide-until-placed styles through `Styles`, `OverlayPositionOptions`, popover `scroll`/`boundary`/
  `target_rect`/`on_enter`/`on_exit`, `use_modal` removed, `ToastKey`, generic `use_modal_backdrop`, `js_sys::Set`
  in `aria_hide_outside`, page-clock timing checks.
- Forms and collections: number formatting/parsing fixes (fraction digits, overflow, "1.2."), a generic spin button,
  per-key subscriptions in `SelectionManager`, `FilterQuery`, `Key::id_fragment`, ListBox/Select fidelity fixes;
  combo box async opening, menu/tag fidelity, their RAC test cases.
- Grid, DnD, virtualizer: GridList rows on `focusin`, `js_sign` column layout, column refocus, sortable descriptions,
  the selection column following the selection mode, malformed-table warnings, constant-time cell lookups; DnD page
  health, drag-generation guard, untracked event reads, borrowing `DropOperationEvent`, 46 native DnD cases and all of
  useDraggableCollection; layout by binary search and content comparison, batched item measuring, `ItemSize`/
  `EndAnchor`/`VirtualListOptions`.
- Dates, color, widgets: Gregorian formatting forced, times kept on date selection, memoized formatters, `Time`
  bounds, picker state contexts, the date test suites; hue normalization, color defaults as upstream, C13 types,
  reactive wheel radii; one `use_link` per `Link`, `use_tab` with `FocusableContext`, Tabs data attributes, event
  props on links, toggle buttons and tabs.
- assertr's thirtyfour integration (assertr 0.8, unreleased; features `thirtyfour`, `thirtyfour-cdp`): assertions and
  shared reads on `WebElement` (attributes, text, state, focus, accessible name/role/description), adopted by the
  browser tests (verified in the completion run above).

### Browser test suite rebuilt (2026-10-08 – 10-09)

From 139 tests sharing state to ~875 independent cases; the result is described in `testing.md`.

- One test per case (2026-10-08): every case a `pub async fn` loading its own page. 68 cases had silently depended
  on earlier ones, several had stopped checking anything, one hid a library bug (tree expand button). Registration
  went from `cases!` macros to explicit `TestGroup`s in `tests/ui_tests/mod.rs` (2026-10-09; logical groups span
  modules), with `#[browser_test]` (crate `browser-test-macros`) generating each case's type, name and description;
  test selection (`TestFilter::from_env()`: `BROWSER_TEST_FILTER`, `BROWSER_TEST_GROUP`) moved into browser-test.
- One way per check: lookups by `Locator` (CSS, `role(AriaRole::X)` on browser-computed roles, `.text`, `.has`; the
  XPath locator, the local role table and the JavaScript query engine are gone), singular lookups reject ambiguity,
  every state has read / `wait_for_*` / `*_stays`. Waits went from hand-rolled poll loops through `wait_for!` macros
  and a polling builder to assertr's eventual assertions (`eventually_ok`, `consistently_ok`, `Patience`; assertr
  0.8 and borrow-for were extended in the user's checkouts for this). Tests use thirtyfour's `WebElement` with
  `ElementActions`; fixture navigation, adapters and the health policy live in `tests/fixtures/`.
- Strength (review 2026-10-09): vacuous cases fixed, single reads after actions became waits or stays checks, packed
  cases split into one case per upstream test, full accessible names and validation messages asserted, timers
  measured in the page (`start_stopwatch`) instead of slept (~75 fixed sleeps were gone already), macOS/iOS paths
  through `emulate_platform`, typed `SyntheticEvent` builders, `HeldPointer`. Every case's doc comment names its
  upstream test.
- Page health after every test (panics, uncaught errors, `console.error`/`console.warn`, duplicate ids, dangling id
  references, literal `attr:` attributes): when introduced it failed 117 cases on 8 fixtures (unlabeled fields,
  `form="test"` without its form, the slider's missing-value warning, untracked reads in DnD), now PLAN items with
  exact per-route warning exceptions. After a Rust panic, waits give up at once. The hydration test reports every
  failing fixture, checks that ids keep their element and covers the query variants (`QUERY_VARIANTS`);
  `server_did_not_panic` names each panic's request.
- Speed and isolation: Chrome Headless Shell, session reuse with a per-test reset (`SessionReset::manual` keeping the
  HTTP cache), the test-app built `--release` (13 MB of wasm instead of 33) with its server at `opt-level = 1`,
  content-hashed immutable assets, hydration awaited in the page, lookups filtered in the page, per-section fixtures
  (`?only=`), ICU4X data of the fixtures' locales only, shorter fixture timers: 821 tests took 2m 40s at parallelism
  4, 871 now take ~35 s at 8 (`testing.md`, "Speed and memory"). A suite serves its site from its own target
  dir; the suite is no default test target (`cargo test --test browser_test`).
- browser-test 0.6 (the user's checkout): failure reports with the test-code frames, the last steps and the last
  value seen; Chrome profiles under `<target>/tmp`; cancellation on Ctrl-C; `ring` instead of `aws-lc-rs`; pages
  brought to the front (chromedriver started them unfocused, so script focus fired no events).
- Found on the way: the virtualizer took its own scrolls for the user's, so `VirtualList` stopped following its end
  (`lessons.md`); the tree expand button bug; doubled accessible names (PLAN).
- Also 2026-10-09: a read-only review of every hook, atom and test by 13 agents (findings in `PLAN.md`, "Review
  2026-10-09"); native toast-queue tests (react-stately's `useToastState.test.js`); `testing::with_owner` suppresses
  untracked-read warnings only for the test body; broken `// Upstream:` headers fixed so the drift script reads them.

### Features and fixes (2026-10-07 – 10-08)

- Tree tables (agnite dev-ui's request; react-aria-components' `Treeble`): child rows via `ItemBuilder::children`,
  `TableTreeInput` (tree column + an expansion state shared with trees), `role="treegrid"` with levels and positions,
  ArrowRight/ArrowLeft expand, collapse and go to the parent (mirrored in RTL), `Table`'s `tree_column` and
  expanded-keys props, `TableExpandButton`; collapsed rows stay rendered but `hidden`. Type-ahead walks the shown
  rows; a row collapsed out of view hands focus to its nearest shown ancestor. Tests mirror `Treeble.test.js`.
- Typed selection values (C16, the user's decision): `RadioGroup`, `CheckboxGroup` and `ToggleButtonGroup` generic
  over `V: SelectionValue`, `Select<S>`/`ComboBox<S>` over the selection's shape (replacing `selection_mode`, where
  several values compiled with single selection); `selection_value!` for enums. Hooks stay key-based.
- Localized strings (`utils::intl_strings`, feature `intl-strings`, default on): react-aria's 21 message bundles in
  34 locales, converted by `scripts/port-intl-strings.py` into typed per-family structs and formatted at runtime
  (ICU plural/select); a test parses every message of every locale (it found upstream's sr-SP `{veza}`). Every family
  uses them. With them came the grid selection announcements and the highlight-selection description.
- The single `Checkbox`, `Radio` and `Switch` atoms removed (the user, 2026-10-07; react-aria-components deprecates
  them): `*Field` + `*Button` pairs.
- Dates: 12-hour times use the locale's own clock (ja-JP "午前0:30", h11) and day periods are AM/PM as in `Intl`
  (ICU4X's flexible day periods replaced); ICU4X 2.3 required.
- Fixes: `use_table` merges its sort description into `aria-describedby`; item labels and menu item roles follow the
  collection; tree rows follow an item gaining children; merged press + hover props fire `on_double_press`;
  `CollatorSensitivity` maps as ECMA-402; collection nodes carry their document position (a 20,000-row append 14.3 →
  11.6 ms); the keyboard delegates use the per-locale collator instead of building one per key press; the
  virtualizer's and the toast's clocks are WebAssembly-only (they panicked natively); `just clippy` also checks
  release builds; rustdoc is warning-free.
- Not reproduced or not library bugs: RTL menus, nested-modal focus restore, crudkit's Enter on a cell's `Button`
  (`table_navigation_tests` guards it), the safe triangle (the test moved too coarsely), RTL date picker arrows (the
  fixture had no `dir="rtl"`), two flakes caused by test timing (`lessons.md`).
- 42 native `*_state` tests (color channel field, picker and slider, form validation, text field, virtualizer).

### Fidelity review (2026-10-07)

A read-only review of the whole library against react-spectrum @ 99e6102368 (one pass per family plus architecture,
docs and tests), applied the same day by nine agents, failing test first. The user's decisions: modals focus the
dialog on open (`ModalContent` `auto_focus` defaults to false) and set no `aria-modal`, as upstream; the `LinkButton`
atom is removed (use `Link`); `Propagation` covers only press and keyboard events. Most hooks moved to one input
struct (C8); the book's changelog lists the renames ("Changed during development").

- Interactions and focus: `use_press` keyup in the capture phase, the macOS Meta keyup synthesis, drag out/in through
  `pointerenter`/`pointerleave`; `use_move` re-ported (no axis filter, which fixed the sliders' cross-axis arrows);
  `use_focus_visible` synced; FocusScope select-on-Tab and Tab outside the scope.
- Collections: `Filter` by characters, a memoized collator per locale, the combo box's custom value, validation and
  hidden form inputs, `Collection::build` in O(n), section atoms (`ListBoxSection` + heading, `GridListSection`,
  `GridListHeader`, `GridListItemDescription`), per-item `disabled_behavior`.
- Grid, table, virtualizer, DnD: nested column groups, reactive table atoms, follow mode keeps measured sizes, RTL
  `scroll_to`, keyboard DnD hides with `inert`, DnD tests with synthesized native drags.
- Overlays: containment and measurements per opening, positions reset on reopen, popover `dir`, toast focus by key.
- Forms: `NumberParser` re-ported from `@internationalized/number` (~40 upstream tests), `format_to_parts`, saturating
  integer values, `FormValidationState` (C3), `CheckboxField`/`RadioField`/`SwitchField` with their `*Button`s.
- Calendar and dates: the range calendar's touch tap, the server's today during SSR, `CalendarMonthPicker`/
  `CalendarYearPicker`, autofill (`use_hidden_date_input`), formatting on ICU4X field sets.
- Slider, color, buttons, tabs: per-thumb labels, `ColorWheel` without a channel, `Button`'s pending state,
  `TabPanels`, `Fraction` for meter and progress bar.
- The six tests still failing afterwards were fixed at their root causes: the item registry keeps every live
  registration per key; a disabled first tab is skipped while rendering (it was skipped in a client Effect only, so
  hydration differed); `SelectionManager` notifies only on changes; group items keep their own validation behavior;
  native validity is re-read when constraint attributes change.

### Roadmap R1–R5 (2026-10-05 – 10-07)

The 2026-10-05 audit found 18 bug/crash risks, 13 library-wide conventions to settle, ~78 untested hooks and 101
files on an old upstream commit; the roadmap worked through them:

- R1 bugs and crash risks: a typed, infallible `Href`; timers and deferred callbacks outliving their owner
  (`OwnerAlive`); transient global listeners as drop guards (`utils::event_listeners`); hydration-stable ids
  (`use_id` instead of random `Uuid`s); SSR global state (`use_description` client-only, `use_prevent_scroll`
  releasing only its own count); the Effect Read Order rule (from dev-ui's ComboBox report); `I18nProvider` scoped to
  its children; a modal's hide-outside revealing overlays opened from inside it (`keep_visible`).
- R2 conventions C1–C16 applied library-wide (`conventions.md`): no `new()` on inputs (66 removed), C4 state props
  on every atom (`ValueBinding::from_state_props`), typed keys (`KeyboardKey`), `use_locale()`/`use_direction()`,
  one `Orientation`, `SelectionMode` defaulting to `None`, `dev_warn!`, no direct `web_sys::window()`.
- R3 families re-ported to 99e6102368, each with its deviation block, tests derived from upstream and atoms:
  toggles, checkbox/radio groups and the toolbar; fields (`use_label`/`use_field`, C14 field parts, `Form`, the
  number field generic over its value, C15); overlays (`OverlayTriggerState`; dialog, modal, popover, tooltip,
  menu; positioning with upstream's 22 placements and arrows; submenus, subdialogs, context menus, close on scroll,
  entry/exit animations); interactions and focus (`PressPropagation`, `Pressable`/`Focusable` on their child,
  FocusScope re-sync; tests from all of upstream's interaction tests); label slots (`LabelPresence`, RAC's
  `useSlot`); links, breadcrumbs, disclosure; meter, progress bar, spin button, slider (generic over
  `NumberValue`), separator, tabs; color (area, slider, wheel, fields, swatches, picker; `Color` keeps its color
  space, `Alpha<C>`, color names); calendar, date/time fields and pickers on jiff (`IncompleteDate`, segments from
  ICU4X parts; the `time` crate is gone); toast and landmarks; virtualization (react-stately's virtualizer and
  layouts; `Virtualizer` and `VirtualList` atoms, for dev-ui); keyboard shortcuts (`use_global_shortcuts`, `Keys`,
  `ShortcutKeys`); TagGroup atoms; clipboard (`write_text_deferred`); collections (`design-collections.md`;
  `hooks::selection` and react-aria's legacy API removed).
- R4/R5: interaction modality tracked from hook creation (crudkit's grid focus jump); `utils::scroll` public;
  assertr in unit tests; parallel browser tests; the harness writes the atom theme before starting the app.

### Components layer removed (2026-10-07, the user's decision)

Steps: default classes `leptonic-<AtomName>`, the atom theme (react-aria-components' starter styles,
`atom-theme.md`), the book off `leptonic::components`, then the deletion with the features `components`, `tiptap`,
`sanitize` and `themes` and their dependencies. The starter templates moved to atoms + the atom theme. What replaced
each component:

- Atoms: `Button`, `Link` (also for `LinkButton`), `Checkbox`/`Radio`/`Switch`, `TextField`, `NumberField`,
  `Slider`, `Select`, `Tabs`, `Table`, `Meter`, `ProgressBar`, `Separator`, `Modal`, `Popover`, the calendar and
  date picker atoms (`DateSelector`, `DatePicker`), `ColorPicker`, `Disclosure` (`Collapsible`), `ToastRegion` +
  `Toast` on a `ToastQueue`, `ShortcutKeys`/`Keys` (`Kbd`), `TagGroup`/`Tag` (`Chip`).
- Plain markup + CSS (recipes in the book): `Card`, `Stack`, `Grid`, `Tile`, `Skeleton`, `Typography`, `AppBar`,
  `Drawer`, `Alert`, `Icon` (use `leptos_icons`), the transitions (atoms expose `data-entering`/`data-exiting`).
- `Root`: `ThemeProvider` + `ToastRegion`, `dvh` units; `Code`: `utils::syntax_highlight` + the clipboard util;
  `TiptapEditor` and `SanitizedHtml`: use `leptos-tiptap` and `ammonia` directly.

Dependencies (2026-10-07, the user's priority): all upgraded, unused ones removed, every one declared with
`default-features = false`; the feature flags gate their modules; `syntect` without theme dumps and loaders.

### Found by the book and consumers (2026-10-04 – 10-07)

Most gaps of these weeks were reported by the book, crudkit and agnite dev-ui (`consumers.md`). Notable: closing a
focused dialog panicked (handlers running during unmount use `try_*` accessors: "Blur After Disposal"); random
element ids broke hydration (`use_id`, `test_hydration_ids`); the listbox lacked `aria-disabled`, listened to
non-bubbling `focus`/`blur` and always wrapped; `use_press` stopped every click (now upstream's propagation); macOS
lacked Cmd + Home/End in collections; the live announcer became react-aria's singleton. The themed components' gaps
went with the layer.

## Book

### 2026-10-09

- Completed the refactor integration across API tables, live demos, snippets and prose: flat exports and feature
  flags, typed styles and callbacks, focus/interactions, collections, overlays, virtualizers, dates and colors.
  API checking now understands slice types; hidden-select docs and demos use reactive option selection.
- The browser harness honors `BOOK_TARGET_DIR` (or `CARGO_TARGET_DIR`) and puts its generated site inside that
  target, so agents use the shared book target without creating another dependency build tree.
- Verified 47 native/API checks, 2 server asset tests, all 240 book browser tests, and SSR plus WASM hydrate clippy.
  The final browser run includes the integrated library and startup precompression changes. The browser checks
  cover page health, links, phone widths, dark-theme readability and Markdown exports.
- WASM is served precompressed in every profile, including `just serve` (`wasm-dev` / `server-dev`): server startup
  generates missing or stale gzip-6 and Brotli-4 sidecars atomically before accepting requests, reuses fresh
  production sidecars, and lets `leptos_axum` negotiate the encoding. No per-request WASM compression or additional
  command-line tools. Native tests cover negotiation, decoding, cache headers and refreshing stale files.

### 2026-10-08

- Guide "Optimizing Compile Times & Binary Sizes" (`/doc/optimizing-builds`), from `build-performance.md`'s advice;
  every number names the app it was measured on.
- Browser tests on browser-test 0.6 like the library's: every check a test of its own (240 tests in ~32 s; before:
  23 tests in 32 s with 23 browsers), the page walk one test per page, contrast one per theme and page.
- `wasm-dev` at `opt-level = "z"`: 19.7 MB of wasm instead of 43.8 MB, rebuilds no slower.
- Tree tables (Table Atoms "Tree Tables", a file-browser demo, `TableExpandButton`, the `<tr hidden>` styling
  pitfall), typed selects (`Select`/`ComboBox` value shapes, `selection_value!`), a live `use_color_picker_state`
  demo, a static review of 30 pages' snippets against the library.

### 2026-10-07

- Off the components layer: shell, kit, pages and demos on atoms + book CSS; Component tabs removed (redirects to
  the atom pages' styling sections); recipes for layout pieces; changelog "Removed: the components layer".
- Pages on the re-ported library: Date & Time, Calendar, Toast, Color, virtualizer and keyboard APIs; shell
  landmarks (`MainLandmark`, `NavLandmark`, the skip link); small-screen menus on `Drawer`.
- Dependencies: each with `default-features = false` and only its features; `scraper` 0.27, `tower-http` 0.7, `syn`
  3; unused crates removed; TLS on `ring` (23 s less CPU per server build); ICU4X data for 2.3.
- Fidelity follow-up: `kit::api_check` also compares types; `TabPanels`, `ContextMenuTrigger`, Button `is_pending`,
  the month/year pickers, grid list sections and localized strings documented; Markdown export fixes (line breaks in
  code blocks, `.md` links, `###` items in the index).
- Visual pass (light, dark, 390px): prose line height, code typography, identifiers breaking between words.
- The `*Field` + `*Button` pairs replace the single Checkbox/Radio/Switch atoms (125 files); keyboard tables only on
  concept overviews; accuracy sweeps of every prop default, `leptonic-<Atom>` class and data attribute.
- The theme is kept in a cookie and rendered by the server (no theme flash; the Themes guide shows the pattern).
- The server no longer compresses the dev wasm on the fly (suite 2m31s → 31 s; `lessons.md`).

### 2026-10-04 – 10-06

- All ~130 pages on the page kit; the book built with leptonic (shell, search with Ctrl/Cmd+K, kit components);
  `kit::api_check` compares every API table with the library source; one demo stylesheet per sidebar section; narrow
  screens stack table rows into cards.
- Navigation restructure (2026-10-06): Guides, Concepts and Building blocks (`nav.rs`, placement rules tested, old
  URLs redirect).
- Audit fixes across shell, kit, search and every area; Quick Starts for the collection overviews; the Event
  Propagation demo; the virtual focus page; context menus, submenus, subdialogs and overlay animations documented;
  shell tests; parallel book browser tests (4m19s → 2m13s, then 51 s in shards).

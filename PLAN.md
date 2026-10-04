# Leptonic architecture migration: working plan

Living document. Keep it short: remove finished items (git history has them), add new findings as they come up.
Last refreshed: 2026-10-04.

## Guiding decisions

- **Faithful behavior, idiomatic shape.** react-aria is the behavioral spec (keyboard, ARIA, focus, edge cases).
  The API shape is ours: signals instead of controlled/uncontrolled props, `CapturedElement` instead of refs,
  typed ARIA values, typed keyboard shortcuts. Document every deviation in the hook header.
- **Compose inputs, not DOM props.** A hook that configures an element rendered by another hook returns that hook's
  input (`use_menu_trigger` → `UseButtonInput`), combined by callers with struct update syntax. `MergeWith` is only
  for independent hooks on one element. Inputs implement `Default`. See `documentation/hooks-implementation.md`.
- **Upstream is tracked by commit.** Every ported file starts with `// Upstream: <path> @ <sha>`;
  `scripts/upstream-drift.sh` lists unabsorbed upstream commits, `--mark-synced` records a finished re-sync.
- **Tests are the definition of done.** Pure logic and `*_state` hooks get native unit tests. DOM behavior gets
  browser tests against `testing/test-app` (fixture registry, hydration marker, server-panic check). A hook without
  a test is not finished.
- **Docs follow code.** Every hook, atom and component change updates its book-ssr page, in plain, natural prose.
- **Target modern browsers** Leptonic targets Leptos, which already requires WASM. On top of that: Leptonic must not
  support the same set of browsers that react-aria supports. We can target fairly modern ones. Where react-aria
  introduces massive amounts of code to handle older browsers as well, we can omit that.

## Waiting on the user

- **Upstream release:** chrome-for-testing-manager on chrome-for-testing 0.5 (parses the new `linux-arm64`
  platform), then browser-test on that. Prepared on branch `cft-0.5-linux-arm64` in `../chrome-for-testing-manager`
  (version bumped to 0.12.1). Afterwards remove the temporary `[patch.crates-io]` in the root `Cargo.toml`.
- **leptos-styles suggestion:** an `add_reactive_unchecked` method; today an always-present reactive unchecked value
  needs `add_optional_unchecked(prop, move || Some(..))`. leptos-css lacks `Margin::all`-style helpers.
- **Commits:** nothing has been committed; the working tree holds all changes.

## Next up

- [ ] Collections step 0: menu and grid list safety nets; then steps 1–3 (collections core, `SelectionManager`,
  delegates), then listbox (fixes type-ahead) and select (fixes uncontrolled updates).

## Testing infrastructure

- [ ] Replace fixed `sleep`s in focus_scope / has_tabbable_child tests with polling helpers.
- [ ] Remove page-object helpers duplicated by `BaseActions` (`get_active_element_id`, `send_key_to_active`, ...).
- [ ] Try `BrowserTestParallelism::Parallel(n)` once the suite is slow (currently ~30 s warm).
- [ ] Native test helper for hooks (`with_owner`) and a way to run Effects natively, for `*_state` hooks.
- [ ] Extend the hydration id test (`test_hydration_ids.rs`) to every fixture that generates ids.
- [ ] Remaining random ids: DnD registration ids (`use_draggable`, `use_droppable`, created on both cfg paths),
  overlay stack ids, legacy components (radio, collapsible, tab, tiptap).
- [ ] SSR audit: 36 `StoredValue::new_local` uses, 20 files combine them with `on_cleanup`. On the server the owner
  may be disposed on another thread, which panics for `LocalStorage` (seen in the old spin button). The
  server-panic check only catches this for pages with fixtures.

## Hooks

### Re-sync with upstream
Work queue: `scripts/upstream-drift.sh` (top: use_combobox, use_select, use_selectable_collection, shadow_dom,
use_grid_list_item, use_focus_visible, use_date_segment). Done: use_press, use_button, use_spin_button,
use_number_field, use_menu_trigger, use_keyboard, use_focusable, virtual_click, live_announcer.

### Collections (design: `documentation/design-collections.md`)
Steps, each guarded by DOM/ARIA-level browser tests written *before* the rewrite:
- [ ] 0. Safety nets: listbox + select done (`test_listbox.rs`, `test_select.rs`; known-broken behavior runs
  with `BROWSER_TEST_KNOWN_ISSUES=1`: listbox type-ahead, uncontrolled select not updating). Still missing:
  menu (it also drops the collection's focus handlers, like the listbox did) and grid list.
- [ ] 1. `hooks/collections` (Key, Node, Collection, builder, use_collection, filter, flatten_expanded).
- [ ] 2. Selection core (`SelectionManager`, `use_list_state`, `use_single_select_list_state`), unit-tested.
- [ ] 3. Delegates (list with RTL + page up/down, delegate-based type-ahead), DnD onto the non-generic trait.
- [ ] 4. Listbox family. 5. Select. 6. Menu. 7. Combobox. 8. Grid list + tag group. 9. Tree. 10. Grid + table.
  11. Tabs. 12. Cleanup (delete generic selection code, duplicate enums), docs, mark upstream synced.

### Known gaps (from the former hook audit)
- use_press: macOS Meta keyup synthesis, pointer capture release.
- use_focus_visible: pointer sub-type tracking (mouse/pen/touch). use_focus_manager: focus with scrolling.
- use_slider: label click focuses first thumb (+ keyboard modality); form reset for thumbs; locale number formatting.
- use_overlay_position: offset, cross offset, container padding / boundary nudging, max height output, resolved
  placement, window resize, arrow positioning, close on scroll, is_open gating, boundary element, visual viewport.
- use_combobox: live announcements, compose use_text_field, freeze collection while closing, button out of tab order.
- use_menu_item: submenus (aria-haspopup/expanded, hover), drag from trigger to item.
- use_disclosure: animation support, `hidden="until-found"`. use_meter: `role="meter progressbar"` fallback.
- Calendar: drag-to-select ranges, ICU4X month names, time preservation. use_date_picker: focus-within, RTL.
- use_breadcrumbs: localized label; use_breadcrumb_item should compose use_link.
(Selection, table, tree, grid, option and select gaps are covered by the collections migration.)

### Thin `*_state` hooks that claim "no deviations" but are incomplete
- [ ] use_radio_group_state (read-only, disabled, last focused value, validation)
- [ ] use_time_field_state (granularity, min/max, validation)
- [ ] use_checkbox_state / use_checkbox_group_state, use_search_field_state, use_disclosure_state

### Missing hooks (by expected user value)
- [ ] use_visually_hidden (+ VisuallyHidden atom); replaces ad-hoc copies
- [ ] use_toggle_button, use_toggle_button_group (+ state)
- [ ] use_toast, use_toast_region, toast queue state (then rebuild the Toast component on it)
- [ ] use_calendar (single date; only the range variant exists)
- [ ] use_date_range_picker (+ state)
- [ ] use_context_menu, then `trigger="contextMenu"` for use_menu_trigger; use_submenu_trigger (+ state)
- [ ] use_landmark, use_action_group, use_autocomplete, use_table_column_resize
- [ ] Later: step list, token field, preview trigger, virtualizer

### Cross-cutting
- [ ] Port hooks onto `use_keyboard` shortcuts (upstream uses them in ~20 hooks, mostly with `allow_repeats`).
- [ ] Input composition elsewhere: which `MergeWith` types become unnecessary (e.g. link + press, tooltip trigger)?
- [ ] RTL from the i18n context everywhere (listbox and menu hard-code LTR).
- [ ] Localized strings (`use_localized_string_formatter` equivalent): "Empty", "Long press to open menu", ...
- [ ] SSR safety: `web_sys::window()` in use_grid, use_grid_list, use_grid_cell.

## Atoms

- [ ] Atoms drop `attr:` attributes (e.g. `<ListBox attr:id=..>`): users can't add ids/data attributes. Check
  every atom; probably their views wrap the element (Provider, fragments), so attributes land nowhere.

Hooks that are complete but have no atom yet: Menu, ComboBox, Tree, Toolbar, Tooltip, Breadcrumbs, Meter,
ProgressBar, Checkbox, Radio, Switch, TextField, NumberField, SearchField, Tabs, TagGroup, Disclosure,
ColorSlider, ColorWheel, ColorField, Calendar, DateField, DatePicker, Separator.

## Components (rebuild legacy ones on atoms)

Legacy (no hooks): alert, checkbox, chip, collapsible, datetime_input, drawer, input, progress_bar, radio,
separator, table, tab/tabs, toast, toggle, transitions. `calendar.rs` and `time.rs` are empty.

## Documentation

- [ ] Missing book pages: animation hooks, calendar, datepicker, form validation/reset, live announcer, atoms
  without pages (color_area, color_swatch, dialog, focus_manager, grid_list, hoverable, label, listbox, modal,
  select).
- [ ] Calendar concept route (open item in the documentation strategy).

## Findings log (most recent first)

- Listbox safety net found: options in `disabled_keys` lacked `aria-disabled`; the listbox dropped the collection's
  focus handlers and tab index (so DOM focus never followed the keyboard); collections listened to `focus`/`blur`,
  which don't bubble (React's `onFocus` does; now `focusin`/`focusout`); arrow keys always wrapped; select's
  `aria-controls` pointed at a non-existent id (and was set while closed); select label had a dangling `for`.

- Element ids were random (`Uuid::new_v4()`), so server and hydrated client disagreed. Now `utils::id::use_id`
  (Leptos' shared-context counter); `test_hydration_ids` compares server HTML ids with the hydrated DOM.

- Live announcer: the context/provider design was unused, so every announcement created a throwaway live region,
  and context lookups failed in owner-less event handlers. Replaced with react-aria's singleton.
- use_number_field prevented Enter's default, blocking implicit form submission (upstream changed in #10200).
- use_button ran press handling before keyboard handlers, so `preventDefault` hid Enter/Space from shortcuts.
- Menu trigger + button were two press state machines on one element (merged DOM props).
- `is_virtual_pointer_event` applied the Android TalkBack heuristic on all platforms, misclassifying desktop mouse
  presses with zero pressure (Safari, WebDriver) as screen reader clicks.
- Toolchain drift: assertr 0.4 → 0.7, leptos-use 0.18 → 0.19 (web-sys 0.3.106), wasm-bindgen 0.2.129.
- leptonic-theme's `generate` raced with itself under cargo-leptos (parallel server/client builds).
- Legacy components mixed `class=classes` with `class:` directives (collapse, drawer, grid, table); fixed.
- The build was broken: leptonic targeted an unpublished leptos-styles API.

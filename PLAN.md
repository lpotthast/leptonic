# Leptonic: working plan

The only todo list of the repository: library (`leptonic/`, `leptonic-theme/`, `testing/`) and book
(`examples/book-ssr/`, section "Book"). Living document. Keep it short: remove finished items (git history has
them), add new findings as they come up. Last refreshed: 2026-10-06 (R3f done, C10 keys; disclosure done, review findings in R1; library-wide audit 2026-10-05; raw findings with file/line
details in `documentation/audit-2026-10-05.md`, referenced below as "audit §n").

## Guiding decisions

- **No APIs alien to Rust.** react-aria defines *behavior*; where its API shape is a JavaScript idiom
  (stringly-typed values, `string | number` unions, runtime-parsed specs, controlled/uncontrolled prop pairs, magic
  strings like `'all'`), we design a Rust-native API instead (enums, newtypes, traits, typed builders, `Default` +
  struct update syntax, signals) and record it as an `API DIFFERENCES` deviation, with the reason. Use generics (or
  trait objects) wherever they make an API more capable or better typed (user, 2026-10-05): JS `number`/`any`/string
  values are candidates for type parameters (e.g. C15).
- **No legacy.** Nothing from react-aria's backwards-compatibility surface (deprecated props/aliases, IE-era key
  names, workarounds for unsupported browsers, `UNSTABLE_`/compat paths). And no parallel "legacy" implementations of
  our own: when replacing code, migrate all users and delete the old code in the same piece of work.
- **Faithful behavior, idiomatic shape.** react-aria is the behavioral spec (keyboard, ARIA, focus, edge cases).
  The API shape is ours: signals instead of controlled/uncontrolled props, `CapturedElement` instead of refs,
  typed ARIA values, typed keyboard shortcuts. Document every deviation in the hook header.
- **Compose inputs, not DOM props.** A hook that configures an element rendered by another hook returns that hook's
  input (`use_menu_trigger` → `UseButtonInput`), combined by callers with struct update syntax. `MergeWith` is only
  for independent hooks on one element. See `documentation/hooks-implementation.md`.
- **Upstream is tracked by commit.** Every ported file starts with `// Upstream: <path> @ <sha>`;
  `scripts/upstream-drift.sh` lists unabsorbed upstream commits, `--mark-synced` records a finished re-sync.
- **Tests are the definition of done.** Pure logic and `*_state` hooks get native unit tests. DOM behavior gets
  browser tests against `testing/test-app`, derived from react-aria's own tests. A hook without a test is not
  finished.
- **Docs follow code.** Every hook, atom and component change updates its book-ssr page in the same piece of work
  (one session owns library and book since 2026-10-05).
- **Target modern browsers.** Where react-aria carries code for old browsers, we omit it.

## API conventions (decided 2026-10-05, audit §1)

The reference modules (collections, grid, table, tabs, dnd, text field) already follow these; the rest migrates
family by family (Roadmap R3), the mechanical ones library-wide first (R2). Each convention gets documented in
`documentation/hooks-implementation.md` and as a global entry in `hooks/mod.rs`; per-hook deviations only list what
goes beyond them.

- **C1 State flags:** `is_disabled`, `is_read_only`, `is_required`, `is_invalid`, as `Signal<bool>` (default `false`),
  in hook inputs and atom props alike. Reason: react-aria's names (`isDisabled`), already the majority; one name per
  concept instead of `disabled`/`is_disabled`/`enabled`. DOM-level `*Props` keep DOM attribute names.
- **C2 Strings:** ids, `name`, `form`: `Option<String>`. User-visible text (`aria_label`, placeholders, value labels):
  `MaybeProp<String>` (Leptos' optional reactive prop type; text must be able to change with the locale; constants
  stay ergonomic via `into`). Never `&'static str` for dynamic values (it forced `Box::leak` in the radio group).
- **C3 State shape:** `use_x_state(..) -> XState`, a `Copy` struct with read-only `Signal`s and methods
  (`set_value`, `toggle`, ...), not `*StateReturn` structs of `Callback` fields with tuple arguments. Reason: methods
  are discoverable, typed and cheap; callbacks-as-getters are a JS props-bag shape.
- **C4 Hook-owned state:** hooks: `default_*` + `on_*_change` + state methods, or a `ValueBinding` to app state; the
  mutation path stays the hook's. Atoms and components (the user's rule, 2026-10-06): controlled state as two props,
  a readable `<x>` (`#[prop(into)]`: value, any signal, closure) and `set_<x>: Out<T>` (RwSignal, WriteSignal,
  StoredValue, closure, Callback; for `is_<x>` the setter is `set_<x>`), never one combined binding; uncontrolled `default_<x>` + `on_<x>_change`.
  `is_invalid: Signal<bool>` is OR-ed into the validation state (no `Option<Signal<bool>>` where `Some(false)` forces
  valid).
- **C5 Locale and direction come from the i18n context:** hooks never take `is_rtl`/`writing_direction`/`locale`.
  One reactive accessor `use_locale() -> Signal<Locale>` (plus `use_direction()`); `Locale: FromStr` with an error
  type (no silent en-US fallback). Locale-derived defaults (first day of week, hour cycle) are `Option<_>` = "from
  the locale".
- **C6 One `Orientation`** in `utils` (no `Default`; each input's `new` picks the upstream default) instead of the
  collection one living in `use_checkbox_group.rs` plus `SliderOrientation`/`SeparatorOrientation`/
  `ToolbarOrientation` copies.
- **C7 Constructors:** `Input::new(required..)` + struct update when something is required; `Default` only when
  every field has a meaningful default; no placeholder defaults that build invalid configs.
- **C8 One input:** `use_x(UseXInput)`; state goes into the input (`UseXInput::new(state, ..)`); settings that live
  on the state are read from it, never repeated on the hook input.
- **C9 Props naming:** the element the hook is named after gets `props`; others `<part>_props`. Label/description/
  error message come from `use_field` (`SlotProps`) everywhere, not per-hook error types or `label: Option<String>`
  used only as a flag.
- **C10 Callbacks:** `Option<Callback<NamedEvent>>`; `Arc<dyn Fn(&T) -> R>` aliases only for predicates over
  borrowed data. No tuple arguments, no `Callback<()>` for configuration, no bools meaning modes. Delays are
  `Duration`. Keys are `utils::key::KeyboardKey`, pointer types `PointerType` (no string comparisons).
- **C11 Reactivity:** anything a user could reasonably change at runtime is `Signal<T>` with a default;
  `Option<Signal<T>>` only for "inherit vs. override" (documented).
- **C12 ARIA typing:** typed enums from `utils/aria.rs`, `tabindex` as `i32`. The `&'static str` advice in
  hooks-implementation.md goes.
- **C14 Field parts (decided 2026-10-05 by the user):** one generic `Label`, `Description` and `FieldError` atom
  reading a `FieldContext` that every field atom provides (TextField, SearchField, NumberField, CheckboxGroup,
  RadioGroup, Select, ComboBox, ...), as react-aria-components' `LabelContext`/`TextContext`/`FieldErrorContext`.
  No per-family label/description/error parts.
- **C15 Number values (decided 2026-10-05 by the user):** the number field is generic over its value type
  (`NumberValue`, implemented for all primitive integers and floats): exact integer stepping and clamping, min/max
  defaulting to the type's bounds, ICU4X decimals for parsing and formatting (react-aria: JS numbers).
- **C13 Units:** `Fraction` (0..=1) newtype for percentages, `Point { x, y }` for coordinates, `Duration` for time.

## Waiting on the user

- **leptos-styles suggestion:** an `add_reactive_unchecked` method; today an always-present reactive unchecked value
  needs `add_optional_unchecked(prop, move || Some(..))`. leptos-css lacks `Margin::all`-style helpers.
- **leptos-tiptap suggestion** (2026-10-06): attributes for the editable element (tiptap's `editorProps.attributes`;
  0.10's `UseTiptapEditorInput` has none), so it can be labelled (`aria-label`/`aria-labelledby`; leptonic's
  `TiptapEditor` labels only its group).
- **leptos-element-capture suggestions:** `PartialEq`/`Eq`/`Hash` (identity) for `CapturedElement` (the collection
  item registry tracks registration ids instead); `try_get_untracked` for deferred callbacks (audit §2 B5).
- **Name of the third layer:** "component" is the styled layer (`leptonic::components`, feature `components`), but
  readers also know it as a Leptos `#[component]` and as a UI element. The book now says "concept" for UI elements
  and "styled component" where needed (2026-10-06). Renaming the layer itself (e.g. `styled`) would remove the last
  ambiguity but changes the public module and feature names: decide.
- **Commits:** nothing has been committed; the working tree holds all changes.

## Consumers

- **crudkit** (`../crudkit`, session `crudkit-8f`, 2026-10-05): `crudkit-leptos-ui` builds on hooks and atoms only.
  It depends on the `use_table*`, grid and selection-checkbox hooks (its own table with multi-column sort and
  selection: announce planned changes to these), TextField + `TextFieldState`, `ToggleState`, `atoms::field` +
  `FieldContext` (its own date/duration widgets), Modal/Dialog atoms, Tabs atoms, disclosure and separator hooks,
  and `use_number_field` until the NumberField atom lands. Bindings for Select value, selection and table sort are in
  (R3j, 2026-10-05). Its findings: sibling `ModalBackdrop`s / fields shared one context (fixed: atoms provide their
  contexts through `utils::scoped_context::scoped_view`/`Provider`, scoped to their children); focus not restored
  after closing a modal (not reproducible in the test app, which now checks it with a `Button` atom opener and sibling
  modals; possibly the same context leak, as `FocusScope` leaked its parent-scope context).

## Roadmap (audit 2026-10-05)

Order: R1 (bugs) → R2 (library-wide mechanical conventions, so families aren't migrated twice) → R3 families, each a
complete package: API onto the conventions, missing react-aria features, re-sync to the current upstream commit,
standard deviation block, fixture + browser test from upstream's tests, atoms, rebuilt components, book heads-up.
R4 items are pulled in where a family needs them.

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
- [ ] In their families: forms (R3b/R3c: radio `value`/roving tabindex/`Box::leak`ed names, number field hidden
  input and native `required`, switch on a real input, search field value property, checkbox `checked` property and
  `required`), slider screen-reader input and thumb focus (R3g), controlled tooltip state (R3d), macOS Meta keyup
  synthesis and `preventFocus` re-ports (R3e), legacy components' random ids (their rebuilds).
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
- [ ] Test gaps from that review: `aria_hide_outside` MutationObserver cases (elements added outside, into hidden
  containers, inside a target, reparented, top-layer) and "unhide after reorder"; popover reopened without its
  dialog (containment reset); link hover/focus/press data attributes and Enter; `AnchorLink` (scroll, hash without
  history entry); disabled `LinkButton`; disclosure group `on_expanded_change`, nested groups, focus ring.
- [x] SSR global state (audit 2026-10-06; rule in `documentation/hooks-implementation.md`, "Global State and SSR"):
  `use_description` is reactive and client-only (one API; the tag's description no longer touches the registry on
  the server); themed `Tab` and `TiptapEditor` ids from `use_id` (were random `Uuid`s: hydration mismatch, the
  editor could not attach); `use_prevent_scroll` releases only a count it holds (a disabled instance unmounting
  re-enabled scrolling under an open overlay).

### R2. Library-wide conventions (mechanical, before the families)
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
- [ ] C2 text types: moved into the R3 family packages (the `&'static str`/`Oco`/`Cow` text fields sit in the
  form, slider and color hooks; `MaybeProp<String>` changes each hook's `*Attrs`).
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
- [ ] C12 remaining string ARIA props and string tabindex: within their families (date field/picker, spin button,
  number field, calendar cell, date segment, breadcrumb item).
- [x] `SelectionMode` defaults to `None`; collection atoms take `Signal<SelectionMode>`.
- [x] `PropsWithStyles::into_parts` vs `into_inner`: both stay (spread vs. compose further), documented.
- [x] Rule sweep: no direct `web_sys::window()/document()` left in hooks/atoms/utils (`EventTargetExt::
  get_owner_document` returns `Option`; unused `get_owner_document`/`get_owner_window` free functions removed);
  `target().and_then(dyn_into)` → `expect_target()` (the `if let` forms and stored events stay); untracked reads
  in the press/slider handlers; developer warnings via `utils::dev_warn!` (debug builds only, as react-aria).

### R3. Families (each: API to C3/C4/C7/C8/C9, missing features, re-sync, deviation block, tests, atoms, components)
Ordered by user impact and readiness. Details: audit §1.2 (API), §3 (gaps), §4 (atoms/components), §2.6 (tests).
- [ ] **a. Quick wins:** done: dead `components/calendar.rs`/`time.rs`, the old `utils/event_listeners.rs`
  content, unused helpers (`AriaDescribedby::elements_with_ids`, `Key::as_cell`, `IncompleteDate::get_field`,
  `get_owner_document`/`get_owner_window`), commented-out code, `Mount::WhenShown`'s stray doc, the atom
  `ButtonWrapper` (only a theme class; the component stays), `ModalTitle` on the `DialogTitle` atom. Open: rebuild `components/tabs.rs`+`tab.rs` on `atoms::tabs` (not
  quick: the legacy API registers `<Tab>` children at runtime, the atoms need a collection: decide between a
  collecting builder that keeps `<Tabs><Tab ..>` and a collection-based component API).
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
  Open: no styled ToggleButton component yet.
- [ ] **c. Text inputs and fields:** open: re-port `NumberParser` fully from `@internationalized/number` (other
  numbering systems when pasting, literal stripping from formatted parts, unit plurals, accounting sign, fr-FR/Swiss
  group characters, percent rounding) together with a formatter that knows currencies/units/percent patterns
  from CLDR (`format_percent` appends `%`; German expects `50 %`), and re-sync `number_formatter.rs` (header still
  `@ 6f664fe911`);
  `DateTimeInput` and the `Select` search still style through
  the legacy `leptonic-input`/`leptonic-input-field` classes (`input.scss`).
- [x] **d. Overlays:** done 2026-10-06 (submenus, subdialogs, context menus, close on scroll, entry/exit
  animations; see Done).
- [x] **e. Interactions and focus:** done 2026-10-06 (see Done): `use_press`/`use_move`/focus APIs, `Pressable` and
  `Focusable` on their child, FocusScope re-sync (re-parenting, restore fallback, `focus_safely`, blur and Tab
  handling), browser tests from all of upstream's interaction tests (macOS/iOS-only paths of `useContextMenu` need
  those platforms).
- [ ] **f. Link, button, breadcrumbs, toolbar:** (the Pressable atom applies a responder's trigger props, done
  2026-10-06) rest: breadcrumbs and toolbar as concept pages in the book (overview + layers); localized default
  label of breadcrumbs (R4 localized strings).
- [x] **g. Meter, progress, spin button:** done 2026-10-06 (see Done), with a themed `Meter` component.
- [x] **Label slots:** done 2026-10-06 (see Done): the select's trigger and listbox, the combo box's button and
  listbox, the number field's steppers and the slider's thumbs follow whether the label is rendered.
- [ ] **Legacy components and the state-props rule (C4):** `Drawer` (`shown`, no setter), `DateTimeInput`
  (`get`/`set`), `DateSelector` (`value` + `on_change: Out`) get `<x>` + `set_<x>` when rebuilt (R3h for the date
  ones).
- [ ] **h. Calendar and date picker:** `time::Date`/`PrimitiveDateTime`, `Granularity`, `HourCycle`, `DateRange`,
  `time::Time` for time fields; segments in locale order, `beforeinput`/composition typing, invalid values kept,
  placeholder/leading zeros; single-date `use_calendar`, `use_date_range_picker`, calendar base outputs (prev/next
  button props, title, error, announcements), `trait CalendarGridState`; ICU4X month/weekday names; drag-to-select
  ranges, time preservation; date picker focus-within, RTL segment navigation, `use_field`; the picker's open state
  as `OverlayTriggerState` (moved from R3d). Book session's browser
  findings (2026-10-05): calendar cell never moves DOM focus; grid binds non-bubbling focus/blur (state never
  focused, range blur finalization dead); unavailable dates selectable via Enter (state substitutes the previous
  date), no `aria-disabled`, focus passes booked days; PageUp/Down jump to day 1 (react-aria keeps the day);
  `is_in_range` compares times (midnight max disables its own day); neighboring-month days selectable, mousedown
  switches month and loses the click; first day of week set twice; range calendar value goes stale; disabled
  calendar still moves the cursor; `clear()` skips `on_change`; segment input is a snapshot (`Signal<DateSegment>`
  needed: digit buffer, stale valuenow, Backspace); segments not tab stops until focused programmatically, no DOM
  focus movement, no contenteditable/inputmode/data-placeholder, labels lack the field label; Alt+ArrowDown also
  steps; typing past max clamps (react-aria restarts), Backspace clears the whole segment; first arrow steps from
  the placeholder; days wrap at the month length (react-aria: 31); min/max clamp instead of validating; no hidden
  input/form/reset; date-only values carry `now_utc()` time; time field ignores min/max and `use_time_field_state`;
  picker and field both set `id`/`role` on one element (dangling labelledby); picker ignores `hour_cycle_24`,
  `is_required`, stages from placeholders; reopening focuses today; `is_date_unavailable` takes `Date` in the picker
  but `OffsetDateTime` in the calendar; nothing localized; unused `UseDatePickerInput.is_required`/`hour_cycle_24`,
  `UseDateFieldStateInput.is_required`, `Day.highlighted` (always false); `Day`'s flags to C1 names. Atoms; rebuild
  `components/{date_selector,datetime_input}.rs` (`DateSelector` is mouse-only: clickable divs, no roles/keyboard/
  focus, only `use_calendar_state`; `DateTimeInput` panics without a value (`get().unwrap()`), its time selector is
  a TODO).
- [ ] **i. Color:** (mostly done 2026-10-06, see Done) Left: alpha (react-aria's `Color` has an alpha channel:
  `RGBA8` is an unused stub; `Color` parsing rejects `#rgba`/`rgba()`/`hsla()`/`hsba()`; swatches of transparent
  colors, the alpha channel in sliders and fields)).
- [ ] **j. Collections leftovers:** `use_listbox` doc example calls a nonexistent `UseOptionInput::new`;
  `UseHiddenSelectInput::trigger_id` is documented but ignored; legacy `Table`/`TableHeaderCell` treat `None` as
  defaults (`min_width` defaults to true), `table.scss` comments out `bordered`'s cell border; `ListBox`/`GridList` atoms' `collection`/`state` alternatives → one typed source;
  tri-state `Option<bool>` flags → enums; external state binding (C4): done 2026-10-05 for selections
  (`SelectionOptions::selection: Option<ValueBinding<Selection>>`, atoms' `selection` prop on ListBox/GridList/Grid/
  Table), Select (`value: ValueBinding<Vec<Key>>`) and the table's sort (`sort_descriptor`, `None` clears it), with
  unit tests; also ComboBox (`value`, `input_value`), Tabs (`selected_key: ValueBinding<Key>`) and
  `UseSingleSelectListStateInput::selected_key` (unit tests written, not run yet: browser-test checkout mid-edit
  blocks dev-dependency builds); still open: tree expansion; focus-mode enum names; tree input (keyboard delegate, Tab navigation,
  select on press up); tag group options; menu item links; per-item `on_action`; combobox item actions/links,
  `aria-labelledby` fallback, missing props (`should_focus_wrap`, `on_open_change`, ...); tab links; grid list
  selection checkbox (labelled by its row); `SelectionManager` cell selection/layout ranges; load more; DnD on the
  collection atoms + drag preview + tree drops; TagGroup/Tree atoms; Autocomplete (with upstream's `getPointerType`
  from `useFocusVisible`, which only `useAutocomplete` reads; we track no pointer type yet), then searchable
  Select/Multiselect (search keeps focus, collection unfiltered for the trigger text); legacy `components::select`
  issues; combo box label click shows no focus ring (check upstream first); `atoms::prelude` with `Grid*`/`Table*` once the legacy names are gone; table resizing leftovers (cursor
  overlay while mouse-resizing, resizer `data-focused`/`data-focus-visible`/`data-hovered` via focus ring + hover
  (programmatic focus on press makes `:focus-visible` match after mouse drags), scrollable ancestor, empty tables).
- [ ] **k. Toast and landmark:** `use_toast`/`use_toast_region`/queue state, `use_landmark`; Toast atoms; rebuild
  `components/toast.rs`; `drawer.rs` onto the Modal atom + animation hooks; `transitions/*` onto `hooks/animation`
  (theme styles, reduced motion, `inert` while hidden); `alert.rs` roles; `icon.rs` decorative `aria-hidden`,
  its `width`/`height` props have no effect (theme `svg { width: 100% }`).

### R4. Cross-cutting
- [x] One value for boolean state attributes (2026-10-05): `"true"` everywhere (as react-aria-components), through
  `utils::data_attributes::flag`; `SelectTrigger` no longer repeats `use_button`'s `data-focus-visible`.
- [ ] Found by the book's dogfooding (2026-10-05, leptonic-5e), done: `KbdKey` reads `KeyboardKey::spoken_name`
  (visually hidden) for glyphs and abbreviations; `Icon` is decorative (`aria-hidden`) unless labelled (`role="img"`
  + `aria-label`, which was rendered as a literal `aria_label` attribute); the themed `Button` has `button_type` and
  an optional `on_press`. Open: legacy `TableHeaderCell` always attaches
  `use_press`, so static table headers (the book's documentation tables) are press targets. No hook-based
  Drawer/sheet (the book
  composes `ModalBackdrop` + `ModalContent` + `Dialog`; neither has exit animations). Optional `Callback` props with generic arguments
  (`on_selection_change: Option<Callback<HashSet<Key>>>`) can't infer an untyped closure in `view!` (E0282).
- [ ] From crudkit (2026-10-05), grid: in a `use_table_cell` cell with `CellFocusMode::Child`, Enter on a `Button`
  atom (not the cell's first child) first moves focus to the cell's first focusable child, then the button's press
  fires. `use_press` stops the keydown on press start (as react-aria), so look at the cell's keyup/focus handling.
  Repro: a fixture row with two buttons in one cell. Also: `Dialog`'s `aria_label` is still `Option<String>` (C2).
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
- [ ] From the book (2026-10-05): done (C2 sweep): `aria_label: MaybeProp<String>` in the menu, calendar grid, combo
  box, grid, table, list box, toolbar, tab list and color swatch hooks, the ListBox/Grid/ToggleButtonGroup/Popover/
  ComboBox/ColorSwatch/Tabs/Table atoms and the themed Popover. Still `Option`: breadcrumbs' `label` (R3f re-port,
  also an English default), `use_anchor_link` and the `Icon` component (`Option<Oco>`); item/section labels come
  from the collection data. `classes="a b"` compiles but
  panics during SSR (`ClassName::from(&'static str)` in leptos-classes): accept/split it or reject it at compile time
  (upstream crate, user's). `FieldError`/`Description` outside a field: `dev_warn!`s added.
- [ ] Generics sweep (user, 2026-10-05: "wherever generics or trait objects give more capable, better typed APIs"):
  candidates: typed values instead of the dynamic collection `Key` for value-like selections (RadioGroup, Select,
  ComboBox, ToggleButtonGroup, CheckboxGroup values), slider/spin button/meter/progress values (generic numeric
  like C15), color channel values. Decide per family in its R3 package.
- [ ] Theme flash on every page load (book, 2026-10-05): `Root` keeps the theme in local storage (`signal_ls`), so
  the server always renders light and hydration switches to dark (`data-theme` differs between server and client
  markup, and every color transition animates). Keep the choice in a cookie the server reads, or apply it before
  first paint.
- [ ] Localized strings: a `use_localized_string_formatter` equivalent with upstream's message bundles (~36 hooks
  hard-code English), `useDefaultLocale` (browser language + `languagechange`), reactive `I18nProvider`. crudkit
  (German) overrides e.g. the table selection checkboxes' "Select All"/"Select" through
  `UseCheckboxInput.options.aria_label` meanwhile; keep that working.
- [ ] Atom hygiene: `data-hovered` everywhere (`use_option` must
  return `is_hovered`), `data-focus-visible` on Tab/GridRow/TableRow/ComboBoxButton, context for render state
  (selected/pressed) in Tab/GridRow/TableRow/GridListItem, no `leptos-*` classes in atoms, complete
  `atoms::prelude`, one rule for `Children` vs `ChildrenFn`, atoms dropping `attr:` attributes.
- [x] `use_visually_hidden` (`is_focusable`) + `VisuallyHidden` atom; one CSS source (`VISUALLY_HIDDEN_STYLE`) for
  the live announcer and hidden select; `DismissButton` re-ported on it (`aria_labelledby`, `MaybeProp` label).
- [ ] Port hooks onto `use_keyboard` shortcuts (upstream uses them in ~20 hooks).
- [ ] Input composition: which `MergeWith` types become unnecessary (e.g. link + press, tooltip trigger)?
- [ ] Theme: the global `[data-focus-visible]` outline also hits virtually focused listbox options; theme flash
  (`Root` keeps the theme in local storage, which SSR can't read: use a cookie).
- [ ] Later: step list, token field, preview trigger, virtualizer, `S/data` list helpers.
- [ ] Hygiene: 101 files on upstream commit 6f664fe911 and 3 others to re-sync (`scripts/upstream-drift.sh`);
  standard deviation blocks (88 files with ad-hoc formats, 41 without any, wrong "no deviations" claims, reasons for
  every API DIFFERENCES entry, "no upstream" headers for leptonic-only hooks), missing `// Upstream:` headers on
  ported utils, the legacy "mostly based on work in" lines; split the largest functions (`use_press` 899 lines,
  `use_move`, `use_selectable_collection`, `FocusScope`); `cfg_attr(ssr, allow(dead_code))` → precise `cfg`s; docs
  for public items: undocumented pub fields (`UseFocusInput` callbacks, DnD states,
  `SelectionOptions.disabled_behavior`, `UseListBoxInput`/`Return`, `CollectionOptions`, `SelectState`, `Column`,
  `TableColumnResizeState.table_state`, `TabListState.list`, `TreeState.expansion`, `UseSpinButtonInput`,
  `UseGridCellReturn`, `UseNumberFieldLabelProps`) and most atom/component props. The book's `api_tables` test
  (`cd examples/book-ssr && cargo test --features ssr --lib api_tables`) checks every documented API table against
  the structs and props: run it after API changes.

### R5. Testing infrastructure
- [ ] Every hook/atom family gets a fixture + browser test from upstream's tests as part of its R3 package (~78
  hooks and 14 atom families have none; audit §2.6). Browser tests without `// Upstream:` headers get one.
- [x] Std `assert!`/`assert_eq!` in unit tests → assertr (doc examples keep std asserts).
- [x] Fixed `sleep`s in focus_scope / has_tabbable_child tests → polling (`BaseActions::wait_for_active_id`).
  The remaining fixed sleeps check that something does *not* happen (disabled controls, non-submit buttons) or
  outlast a timeout (type-ahead, announcer); they stay.
- [x] Page-object helpers duplicated by `BaseActions` removed.
- [ ] Native test helper for hooks (`with_owner`) and a way to run Effects natively, for `*_state` hooks.
- [ ] Extend the hydration id test to every fixture that generates ids.
- [x] Parallel browser tests: done (see the Book section's "Parallel browser tests").
- [ ] Clippy in release too: some lints depend on type sizes that differ in release (`MaybeProp`, `StoredValue`: 8
  bytes, so `trivially_copy_pass_by_ref` fires only there; found 2026-10-06). `just clippy` checks debug builds only;
  add a release run (CI or the recipe) once the user decides on the cost.
- [x] Stale theme in browser runs (2026-10-06): with an outer `CARGO_TARGET_DIR`, leptonic's build script found no
  app and the test-app never got theme changes; and cargo-leptos may compile the styles before the build script
  writes the theme. The harness now writes the theme itself before starting the app (`leptonic_theme::generate`) and
  passes `LEPTONIC_APP_DIR`.
- [ ] Chrome profiles leak (browser-test crate, user's): chromedriver's `/tmp/org.chromium.Chromium.scoped_dir.*`
  profiles stay behind when a session isn't quit cleanly (killed runs, `QuitSessionTimeout`). 2026-10-06: 785 of
  them (14 GB) filled the /tmp quota and blocked every agent's tool output (removed). Fix in browser-test: pass an
  own `--user-data-dir` per session and remove it in its drop/cleanup, or sweep stale ones at startup.

## Done (summary)

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
(`--button-disabled-*`, dark values) instead of light-only literals.
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

## Book

Page and writing rules: `documentation/documentation-strategy.md`; look, design tokens and which leptonic piece to use:
`examples/book-ssr/STYLE_GUIDE.md`. Library gaps the book hits go into the roadmap above (R3/R4) and, while the book
waits for them, under "Waiting on the library" below.

### Verification loop
- Clippy (ssr + hydrate/wasm32), zero findings; unit tests (`cargo test --features ssr --lib`), including
  `kit::api_check` (every API table against the library source).
- Browser tests (`just book-browser-test`, `tests/`): page errors, dark theme, internal links and anchors, 390px
  width, Markdown export, on every page.
- Screenshots of affected pages when changing visuals (`just book-serve-isolated` for a second instance).
- Checks build against the live library, as the user's `just serve` does (separate target directory only).

### Navigation restructure (decided and done 2026-10-06)
Rules: `documentation/documentation-strategy.md` ("Terminology", "Navigation", title rule); the sidebar is
`src/nav.rs` (parts Guides / Concepts / Building blocks, placement rules unit-tested), URLs in `src/routes.rs` (old
URLs redirect from `moved_*` modules). Concepts that are hook- or component-only today (Tree, Tag Group, Date
Field, Time Field, Color Field/Slider/Wheel, Alert, Toast, ...) get an overview and tabs once a second layer exists.

### Next
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
- [ ] Date & Time: the hook pages document library limitations in "Limitations" sections (22 items); revise them as
  R3h lands. The calendar demos carry a focus-follow stopgap to remove once cells move focus themselves.
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

### Waiting on the library
- `Sheet` → hook-based Drawer (R3k), search results → Autocomplete with arrow keys through results (R3j).
- Document once implemented: table sort announcement, tree tables; the `Label` atom (renders a `for` pointing at a
  generated id nothing has, R3c).

## Findings log (most recent first)

Library and book. Book entries are marked "Book".

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
- FocusScope restore vs. a deferred auto focus (2026-10-06, also upstream): a native `<button>` activated by Enter
  fires a click with `detail == 0`, which counts as a virtual click, so a scope it mounts auto-focuses with
  `focus_safely` one frame later; an unmounting `restore_focus` scope's frame can run first and win (focus ends on
  its node to restore). Not hit by our atoms (menu items press via `use_press`, keyboard modality); the test-app's
  "Dialog from a menu" uses keydown handlers like upstream's test. Revisit if a user report shows it.
- Leftovers of the C4 state-prop sweep in library doc comments (2026-10-06, found by the book's wording pass):
  `atoms/checkbox.rs` says `state` doesn't apply inside a group; the `ModalBackdrop` example in `atoms/modal.rs` uses
  `state=is_open`; `ModalBackdrop`'s `is_dismissable` doc says it covers Escape (the book says outside clicks only:
  check which is true). `DateTimeInput` still takes `get`/`set` instead of `value`/`set_value`.
- Book (2026-10-06): `atoms/grid.rs` and `atoms/grid_list.rs` don't follow the atom-page structure (CSS examples
  with `rgba()`, plain "Escape"/"Enter" instead of `Keys`): rewrite. The Marks demo in `atoms/slider.rs` is inline in
  the page, without `source`: move it into `demos/`.
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
- Leptos `erase_components` and spread attributes (2026-10-06): in erased builds (our `.cargo/config.toml`, and
  cargo-leptos' dev builds), attributes spread onto a component (`attr:id`, ...) are applied once, to the elements
  the component first renders (`AnyViewWithAttrs`); when the component's root element is replaced (a reactive
  branch switching `<a>`/`<span>`), they are lost. Non-erased builds re-apply them per render. Affects the `Link`
  atom when `is_disabled` changes (documented there); worth reporting upstream with a minimal reproduction (a
  component returning `move || if flag.get() { Either::Left(view!{<span/>}) } else { Either::Right(view!{<b/>}) }`
  with `attr:id`). Also: hooks called inside such a reactive branch are disposed while the old element's attribute
  effects still run once (panic "already been disposed"): create them outside the branch and clone their attrs
  (hooks-implementation.md, "Props are Single-Use").
- Breadcrumbs' current item (2026-10-06): "the last item to register" is not hydration-safe (`For` hydrates each
  row before the next registers, the server registers all first), so `Breadcrumb` takes `is_current`. Test
  harness: `goto_path` now reports the page's caught errors when hydration fails; `by_role_and_text` knows the
  implicit roles of `<a href>` and `<button>`.

- Long-press descriptions (2026-10-05): `use_press` rendered a long-press `aria-describedby` only on the client
  (static attribute; the SSR branch had none, and `use_description`'s ids come from a client counter), so hydrated
  pages never had it. Now `UsePressProps::aria_describedby` is a signal set in an effect after mount, as react-aria's
  `useDescription` (layout effect): the server renders none, the client adds it.

- Focus restore and automated browsers (2026-10-05): `FocusScope` restores focus in a `requestAnimationFrame`
  callback (as react-aria's `useRestoreFocus`). Chrome runs no rAF callbacks in windows it considers occluded or in
  the background, even with `visibilityState == "visible"`, so focus restore "fails" there (crudkit's select report;
  the Claude-in-Chrome window). Check with `await new Promise(r => { requestAnimationFrame(() => r(true));
  setTimeout(() => r(false), 1000) })` before debugging focus restore.
- Book (2026-10-05): dark-theme check findings explained: `Root` keeps the theme in local storage (`signal_ls`), so
  the server always renders the light theme and hydration switches to dark; every element with a color transition
  (`.leptonic-btn`: `transition: all 0.1s`, the demo buttons) then animates from light to dark, and the check ran as
  soon as `<body>` was dark. The check now waits for running `CSSTransition`s and parks the pointer at 0,0; all three
  book browser tests pass, dark-theme findings 0. Library: users of the dark theme see the light theme flash on every
  page load (server renders light, client reads dark at hydration, so `data-theme` also mismatches between server
  and client markup); persist the theme in a cookie (or apply it before first paint) so SSR renders the chosen theme.
- Book (2026-10-05): `PopoverTrigger` (atom) spreads `aria-haspopup`/`aria-expanded`/`aria-controls` onto its
  wrapper `<div>`, not onto the button inside, so the button doesn't announce the popover (rework planned with
  `DialogTrigger`).
- Book (2026-10-05): done: modal pages (no `title`/`description`, `DialogTitle` heading, `<section>`, bound open
  state), Button atom data attributes and ARIA props, bindings on the Tabs/ListBox/Table/ComboBox/Select atom demos
  (`selected_key`, `selection`, `sort_descriptor` incl. `None`, `value`, `input_value`) and hook rows, `aria_label`
  rows as `MaybeProp<String>`, book search on the themed `SearchField`, popover hook demos with `use_button` +
  `use_overlay_trigger`, vertical toolbar demo with `use_button`; `/doc/hooks/use-color-channel-field` passes.
- Builds (2026-10-05): the target dirs filled the disk. Cause: the staged `split-debuginfo = "unpacked"` in book-ssr's
  `[profile.dev]` plus ~16 GB of DWARF per plain-cargo book build (no `--cfg erase_components`): the `.dwo` files get
  new random names per rustc run and pile up (rust#161824), and the 17 GB rlib was rewritten on every build. Fixed
  (user decisions): book `debug = "line-tables-only"`, no split debuginfo; `--cfg=erase_components` for all builds in
  the root `.cargo/config.toml` (apps set `disable-erase-components`), no duplicated rustflags in the app configs, so
  every build shares fingerprints; one shared agent target dir per project (CLAUDE.md). Book: 3.8 GB, no `.dwo`.
  Verified: library clippy/unit/browser tests, book clippy/unit tests and dark-theme check pass; the book's phone-width
  and Markdown checks were cut off by an external SIGTERM (rerun). Release builds are type-erased now too.
- Book (2026-10-05): new pages Field Atoms (`/doc/atoms/field`), Form Atom (`/doc/atoms/form`), Text Field atom and
  NumberField atom tabs; per-family part sections removed; `aria_label`/`validation_behavior` tables updated (checked
  against the library by a script); data attributes documented as `"true"`. The agent's separate target dir had
  grown to 154 GB (stale `deps` artifacts) and filled the disk; see the builds entry.
- R3c (2026-10-05): behavior changes visible to users: field atoms (and the themed Checkbox/Radio/Switch) default to
  `ValidationBehavior::Native` like react-aria-components (errors on submission, focus to the first invalid field)
  instead of `Aria`; `NumberFormatOptions::default()` groups digits like `Intl.NumberFormat` ("1,024"). Leptos'
  `view!` needs generics on closing tags too (`<NumberField<u8>>..</NumberField<u8>>`); `#[component] fn Input`
  generates a struct `InputProps` (name clash with an enum of the same name). Contexts provided in a component's
  body (not with `<Provider>`) keep its element the root, so `attr:`/`on:` on the component reach it.
- Toggles (2026-10-05, from the book): `label.click()` (a virtual click, as assistive technology sends) didn't
  toggle a checkbox/switch: react-aria's `useToggle` leaves virtual label presses to the input but prevents the
  label's click. Ours toggles on them (documented deviation); read-only now also holds for bound states, and a
  rejected radio change restores the whole group's `checked`.
- Dialog (2026-10-05, from the book): closing a focused dialog panicked: removing it fires `focusout` after its
  owner was disposed, and the handler read a disposed `StoredValue`. DOM handlers that can run during unmount must
  use `try_*` accessors. `Dialog`'s `aria_label` is now rendered. Both covered by `test_dialog.rs`.
- R3b (2026-10-05): `use_form_validation` only re-read native validity when the realtime validation changed;
  react-aria re-reads it after every render, before the queued commit. Checking a required checkbox or radio
  therefore never cleared the native error. Fixed with `NativeValidityReaders` run by the commit. Leptos' `view!`
  can't parse closure parameters with generic type annotations (`move |v: Option<Key>| ..`) as attribute values;
  brace them (`on_change={move |v: Option<Key>| ..}`). `#[prop(optional)]` `Option<T>` props can't receive an
  `Option` through `view!`; components forwarding them build the atom from its props struct.
- Audit 2026-10-05 (four read-only passes, `documentation/audit-2026-10-05.md`): 18 bug/crash risks, 13 library-wide
  API conventions to settle, ~78 untested hooks, 101 files on an old upstream commit; PLAN reorganized into the
  roadmap above. Leptos event delegation is off (tachys `delegation` feature unused): every `on:` handler is its own
  closure, which is what the nested-dispatch rule protects.
- An intermittent menu panic was `FocusScope`'s `FocusManager` reading its disposed `NodeRef` from a pending focus
  callback; the getters now treat a disposed scope as having no element. Found once page errors started to include
  Rust panic messages. `LocalStorage` stored values in hooks that render during SSR panic on the server (dropped on
  another thread); use thread-safe storage there.
- Focusing inside a `focusin` handler threw "closure invoked recursively" (Leptos listener closures can't be
  re-entered) in every collection and in `FocusScope` containment; focus moves now run in a microtask (see
  hooks-implementation.md, "No Nested Dispatch of the Same Event Type"). Browser tests fail on uncaught page errors.
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
- Book: the Dockerfile builds from the repository root (path dependency on `../../leptonic`); not yet test-built.
  The Markdown middleware wraps the page router (it ran after axum's routing, so every `.md` request hit the 404
  fallback). The unfinished `/theme-editor` prototype was removed.

# Leptonic: working plan

The only todo list of the repository: library (`leptonic/`, `leptonic-theme/`, `testing/`) and book
(`examples/book-ssr/`, section "Book"). Living document. Keep it short: remove finished items (git history has
them), add new findings as they come up. Last refreshed: 2026-10-05 (library-wide audit; raw findings with file/line
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
- **Docs follow code.** Every hook, atom and component change updates its book-ssr page (owned by the book session:
  send it a heads-up; only minimal compile fixes from here).
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
- **C4 Hook-owned state:** no controlled inputs (`value` + `on_change`, `SliderValues::Controlled`,
  `is_open: Option<Signal>`): `default_*` + `on_*_change` + state methods. `is_invalid: Signal<bool>` is OR-ed into
  the validation state (no `Option<Signal<bool>>` where `Some(false)` forces valid).
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

- **Upstream release:** chrome-for-testing-manager on chrome-for-testing 0.5 (parses the new `linux-arm64`
  platform), then browser-test on that. Prepared on branch `cft-0.5-linux-arm64` in `../chrome-for-testing-manager`
  (version bumped to 0.12.1). Afterwards remove the temporary `[patch.crates-io]` in the root `Cargo.toml`.
- **leptos-styles suggestion:** an `add_reactive_unchecked` method; today an always-present reactive unchecked value
  needs `add_optional_unchecked(prop, move || Some(..))`. leptos-css lacks `Margin::all`-style helpers.
- **leptos-element-capture suggestions:** `PartialEq`/`Eq`/`Hash` (identity) for `CapturedElement` (the collection
  item registry tracks registration ids instead); `try_get_untracked` for deferred callbacks (audit §2 B5).
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

### R2. Library-wide conventions (mechanical, before the families)
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
- [ ] **c. Text inputs and fields:** done 2026-10-05: `use_label` re-ported, `use_field` built on it
  (`aria_label: MaybeProp<String>` on all field hooks, `UseFieldLabelProps` → `UseLabelProps`); C14 field parts
  (`atoms::field`: `FieldContext`, `Label`, `Description`, `FieldError`; per-family parts of CheckboxGroup,
  RadioGroup, Select, ComboBox removed, ComboBox label is a `<label for>` as upstream); text field C1/C2
  (`is_required: Signal<bool>`, `placeholder: MaybeProp<String>`, `is_focused` returned, no `value` attribute on
  textareas), `TextFieldState::from(RwSignal)`/`with_on_change`; atoms `TextField`, `Input`/`TextArea`
  (`InputContext`), `Form` (`FormContext`, server errors); field atoms default to the `Form`'s validation behavior,
  else `Native` (as RAC; was `Aria`); `use_form_validation` re-ported (textarea/select, focuses the first invalid
  field with keyboard modality, `focus` option, the hidden select focuses the trigger); browser test from RAC
  `TextField`/`Form` tests; `use_search_field` re-ported on `use_text_field` (input wraps a `UseTextFieldInput`,
  returns the clear button's `UseButtonInput`; `use_search_field_state` gone), SearchField atom +
  `SearchFieldClearButton`, browser test from RAC `SearchField` and `useSearchField` tests; number field generic over
  its value type (C15: `utils::NumberValue` for all primitive numbers, exact integers; `utils::ValueBinding<T>` binds
  app state; `NumberFieldState<T>` per C3 with settings read from it (C8), `CommitBehavior::{Snap, Validate}` with
  native range validation, snapping to explicit steps only; `use_number_field` re-ported on `use_text_field` (shared
  validation state, `beforeinput` filtering, composition, paste, wheel, stepper labels, hidden input left to the
  caller); NumberField atom + `NumberFieldGroup`/`NumberFieldIncrementButton`/`NumberFieldDecrementButton`; one generic
  `Input` for every field (`InputContext` holds a type-erased attribute factory, so ComboBox and custom fields use it
  too; `ComboBoxInput` removed); formatter generic and exact (ICU4X decimals, half-expand rounding, grouping by
  default as Intl, significant/integer digits and sign display implemented, the unimplemented `notation` and
  `compact_display` removed), parser generic (no group separators without grouping, leading separators allowed as
  upstream); browser test from RAC `NumberField` tests; `FieldError` `message` function over the validation result
  (RAC's render function; `FieldContext.validation_details`). Open: re-port `NumberParser` fully from `@internationalized/number` (other numbering systems when
  pasting, literal stripping from formatted parts, unit plurals, accounting sign, fr-FR/Swiss group characters,
  percent rounding) together with a formatter that knows currencies/units from CLDR; `TextFieldState`/`ToggleState`
  onto `ValueBinding`?. Done 2026-10-05: legacy `TextInput`/`PasswordInput`/`NumberInput`/`Label`/`Field`/
  `FormControl` deleted (with `label.scss`, `form.scss`, `field.scss` and dead `input.scss` rules) after the book moved
  to the themed `TextField`/`SearchField`/`NumberField` (new component page). Themed field gaps found by the book,
  done: `input_type` is a `Signal` (hook, atoms, components; show-password fixture), `label`/`description` are
  `MaybeProp` (C2), `id`/`form`/`pattern`/`input_mode`/`aria_describedby` passed through, `SearchField` validation
  props; browser test extended (not run yet: harness agent has the suites). `DateTimeInput` and the
  `Select` search still style through the legacy `leptonic-input`/`leptonic-input-field` classes (`input.scss`).
- [ ] **d. Overlays:** done 2026-10-05: `use_overlay_trigger_state` → `OverlayTriggerState` (C3, `ValueBinding`,
  `point: Option<Point>`, new `utils::Point`); `MenuTriggerState` on it (+ submenu stack); `use_menu_trigger` generic
  over `MenuTriggerStateApi` (implemented by `MenuTriggerState`, `SelectState`, `ComboBoxState`); the non-upstream
  `use_modal_state`/`use_dialog_state` (and `confirm`) removed. Done 2026-10-05: `use_dialog` re-ported to
  99e6102368 (title and content as slots referenced while rendered, alert dialogs described by their content,
  `aria_labelledby`/`aria_describedby`/`is_entering`, missing-name `dev_warn!`; no `title`/`description` text
  inputs); `Dialog` atom renders `<section>`, `DialogTitle` a heading (`HeadingLevel`, default `<h2>`),
  `DialogDescription` the content slot; themed `Modal` lost `title`/`description` (its `ModalTitle` names it);
  dialog browser test checks naming/description. Done 2026-10-05: `use_modal_backdrop` re-ported to 99e6102368 on
  `OverlayTriggerState` (`UseModalBackdropInput::new(state)`, `is_entering` with `keep_visible`); `ModalBackdrop`
  atom and themed `Modal` take `state: OverlayTriggerState` (from `RwSignal<bool>`, a signal pair or a
  `ValueBinding`) instead of `is_open`/`show_when` + `on_close`; `ValueBinding` in the preludes. Done 2026-10-05:
  `use_popover` re-ported on `OverlayTriggerState` (`UsePopoverInput::new(state)`, optional `trigger`, non-modal
  popovers close on scroll); new `DialogTrigger` atom (owns the state, passes press handler + `aria-expanded`/
  `aria-controls` + trigger capture to its `Button` through `PressResponder`'s new `trigger` props, merged in
  `use_button`); new `Popover` atom on `use_popover` (state/trigger from the `DialogTrigger` or props, modal by
  default with dismiss buttons, `ClearPressResponder` for its content, `data-placement`; old
  `PopoverTrigger`/`PopoverContent`/`PopoverContext` removed); a `Dialog` in a `DialogTrigger` is its `aria-controls`;
  `ModalBackdrop`'s `state` falls back to the `DialogTrigger`; themed `Popover` rebuilt on them (`state` replaces
  `show_when`/`on_close`); fixture + browser test from RAC `Popover.test.js`. FocusScope: the active scope is tracked by
  one document-level `focusin` listener (react-aria `useActiveScopeTracker`) plus a check at registration; a scope
  whose dialog focused itself before the scope's own listener existed was never active, so focus wasn't restored
  after an outside click. Done 2026-10-05: `use_tooltip_trigger_state` re-ported to 99e6102368 →
  `TooltipTriggerState` (C3) on `OverlayTriggerState` (`value` binding), `Duration` delays, `open`/`close` take
  `TooltipTiming` (C10), `should_skip_animation`, a pending close isn't restarted, replaced tooltips close at once and
  leave the registry. Done 2026-10-05: `FocusableContext` extended (react-aria-components' `FocusableProvider`):
  `aria_describedby` (merged into the element's own description by use_button/text field/toggle/radio, rendered by the
  links) and `attrs` (further attributes, spread through the `Send + Sync` `FocusableContextAttr`), disabled elements
  get none; new `TooltipTrigger`/`Tooltip` atoms on it (any focusable atom is a trigger; the hook enum is now
  `TooltipTriggerMode`); fixture + browser test from RAC `Tooltip.test.js` (hover delay, warm-up, leave, focus,
  Escape). Done 2026-10-05: `OverlayState` trait (`OverlayTriggerState`, menu/select/combo box states; the menu
  trait extends it), `use_popover` generic over it (returns `popover_element`), one `render_popover` for the `Popover`
  atom, `SelectPopover` (modal, as RAC) and `ComboBoxPopover` (non-modal), both with offset/padding/flip props; tests
  check the select's modality. Done 2026-10-05: an untitled `Dialog` in a `DialogTrigger` is named by the trigger
  (`UseDialogInput::fallback_aria_labelledby`; `DialogTriggerContext::ensure_trigger_id` gives the trigger an id when
  the dialog mounts, keeping an own `attr:id`); popover test from RAC `Dialog.test.js`.
  Done 2026-10-05: `use_overlay` re-ported to 99e6102368 (Escape through `use_keyboard` shortcuts, bubbling when
  keyboard dismissal is disabled); `underlay_props`/`backdrop_props` removed from `use_overlay`, `use_popover`,
  `use_modal_backdrop` (upstream's are empty since the Firefox workaround was dropped; underlay elements stay).
  Done 2026-10-05: `PopoverModality { Modal, NonModal }` replaces `is_non_modal: bool` (C10 "no bools meaning modes")
  in `UsePopoverInput` and the `Popover` atom/component.
  Done 2026-10-05: a modal popover without a dialog inside is the dialog (RAC): `role="dialog"`, `tabindex="-1"`,
  focused once rendered unless focus is inside, named by `aria_label`/`aria_labelledby` (new `Popover` props) or the
  `DialogTrigger`'s trigger; the select's popover is named like its listbox; tests from RAC `Popover.test.js`.
  Done 2026-10-05: `aria_hide_outside` re-ported to 99e6102368 with `HideMode { AriaHidden (default), Inert }`
  (react-aria's `shouldUseInert`): modal backdrop and modal popovers use `inert`, the combo box and drags
  `aria-hidden` as upstream, so the page stays usable behind an open combo box (before, everything was inert: a
  click on another button only closed the combo box); combo box test checks it.
  Done 2026-10-05: Menu atoms (`MenuTrigger`, `Menu`, `MenuItem` with `MenuItemLabel`/`MenuItemDescription`/
  `MenuItemShortcut`, `MenuItems`, `MenuSection`; RAC `Menu.test.tsx` subset in `test_menu_atoms.rs`): the trigger
  hands its button press handlers, ARIA props and shortcuts through `PressResponder` (new `shortcuts`), the popover
  its state through `DialogTriggerContext::new`; the menu is labelled by the trigger's rendered id
  (`UseMenuInput::aria_labelledby` is a `MaybeProp` now); closing the overlay state any way also closes submenus.
  Done 2026-10-05: long-press menu triggers (`MenuTrigger trigger=MenuTriggerType::LongPress`; `PressResponder` carries
  long-press callbacks and the description; upstream's text "Long press or press Alt + ArrowDown to open menu").
  Open: overlay position:
  arrow, point targets, scroll anchoring, visual viewport;
  `use_submenu_trigger`, `use_context_menu`, separators in menus; exit animations (`use_enter_animation` hiding, `watch_animations`
  filter); fixtures + tests.
- [ ] **e. Interactions and focus:** `PressResponderContext::registered` is never read (no warning for responders
  without a pressable child); `UseFocusVisibleInput::auto_focus` is overwritten by an effect once tracking is
  enabled; `use_press` `force_*` → typed options, `on_press: Option`; `use_move`
  constraint split (`use_constrained_move` or `MoveConstraintConfig`), `axis: Signal<MoveAxis>`, `Point`; focus event
  payloads unified (`UseInteractOutsideInput` mixes `PointerEvent` and `MouseEvent`); `FocusManager` atom renamed (clash); Pressable atom on its child (no wrapper div), PressResponder
  OR-ing; FocusScope re-parenting, restore fallback, `focus_safely`; FocusRing `is_text_input`; `Focusable`
  component; re-sync leftovers (press pointer capture release, focus-visible pointer sub-types, focus manager
  focus with scrolling); tests from upstream's interaction tests.
- [ ] **f. Link, button, breadcrumbs, toolbar, disclosure:** non-anchor breadcrumb items lack `role="link"`; check
  whether a disabled `use_link` `<a>` keeps its `href` (focusable/navigable); `UseLinkInput`/`UseAnchorLinkInput`
  lack `Default`/`new`; legacy `CollapsibleBody` takes a static `class: String`; typed `Href`, `LinkTarget::{Blank, Same, Parent, Top,
  Named}`, consistent href/rel/aria_current types, typed form attributes, Button atom pass-through (aria/id/form),
  `Link`+`LinkExt` merged, a `replace` option (don't add a history entry); breadcrumbs on `use_link`; toolbar focus management (RTL, wrap, restore, nested);
  disclosure on `use_button`, `region`→`group`, `hidden="until-found"`, `use_disclosure_group_state`; atoms
  Breadcrumbs/Toolbar/Disclosure(Group)/Link data attributes; rebuild `components/collapsible.rs`.
- [ ] **g. Slider, meter, progress, separator, spin button:** `use_separator` sets `aria-orientation="horizontal"`
  explicitly (react-aria omits it); the theme doesn't style the Separator (`solid` class does nothing); slider's
  stale "type=hidden" deviation note; the Slider component has no `aria_label` (unnamed thumbs); `UseSliderStateReturn` tuple callbacks (C3/C10),
  `RangeSlider` classes/styles without `into`; drop `SliderValues`, `Step` enum, required thumb
  `index`, `Fraction`, label wiring and output ids, `on_change_end` for keys, form reset, `NumberFormatOptions` for
  value labels, `None` = indeterminate progress, aria-label inputs, meter `role="meter progressbar"` fallback; atoms Meter/ProgressBar/Separator; rebuild their
  components; slider test header from upstream.
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
- [ ] **i. Color:** `use_color_area`'s deviation note claims the Y input becomes focusable after keyboard use (it
  always has `tabindex=-1` + `aria-hidden`); `use_color_swatch` claims no deviations (no localized color name,
  `aria_label` replaces instead of joining the name); `use_color_channel_field` has no `name`/`form`, hardcoded
  validation, Page Up/Down ignore the channel's page size; state shape per C3, one `set_value`/`on_change` rule, `use_color_area` keyboard step/page step,
  thumb sync for Page/Home/End, `on_change_end`, color field disabled/commit; `Color` CSS parsing, `FromStr`, alpha;
  typed `track_style`/`BlendMode`; atoms ColorSlider/ColorField/ColorWheel/ColorPicker/ColorSwatchPicker; rebuild
  `components/color_picker.rs`.
- [ ] **j. Collections leftovers:** `use_listbox` doc example calls a nonexistent `UseOptionInput::new`;
  `UseHiddenSelectInput::trigger_id` is documented but ignored; legacy `Table`/`TableHeaderCell` treat `None` as
  defaults (`min_width` defaults to true), `table.scss` comments out `bordered`'s cell border; `ListBox`/`GridList` atoms' `collection`/`state` alternatives → one typed source;
  tri-state `Option<bool>` flags → enums; external state binding (C4): done 2026-10-05 for selections
  (`SelectionOptions::selection: Option<ValueBinding<Selection>>`, atoms' `selection` prop on ListBox/GridList/Grid/
  Table), Select (`value: ValueBinding<Vec<Key>>`) and the table's sort (`sort_descriptor`, `None` clears it), with
  unit tests; also ComboBox (`value`, `input_value`), Tabs (`selected_key: ValueBinding<Key>`) and
  `UseSingleSelectListStateInput::selected_key` (unit tests written, not run yet: browser-test checkout mid-edit
  blocks dev-dependency builds); still open: ToggleButtonGroup, tree expansion; focus-mode enum names; tree input (keyboard delegate, Tab navigation,
  select on press up); tag group options; menu item links; per-item `on_action`; combobox item actions/links,
  `aria-labelledby` fallback, missing props (`should_focus_wrap`, `on_open_change`, ...); tab links; grid list
  selection checkbox (labelled by its row); `SelectionManager` cell selection/layout ranges; load more; DnD on the
  collection atoms + drag preview + tree drops; TagGroup/Tree atoms; Autocomplete, then searchable
  Select/Multiselect (search keeps focus, collection unfiltered for the trigger text); legacy `components::select`
  issues; combo box label click shows no focus ring (check upstream first); `atoms::prelude` with `Grid*`/`Table*` once the legacy names are gone; table resizing leftovers (cursor
  overlay while mouse-resizing, resizer `data-focused`/`data-focus-visible`/`data-hovered` via focus ring + hover
  (programmatic focus on press makes `:focus-visible` match after mouse drags), scrollable ancestor, empty tables).
- [ ] **k. Toast and landmark:** `use_toast`/`use_toast_region`/queue state, `use_landmark`; Toast atoms; rebuild
  `components/toast.rs`; `drawer.rs` onto the Modal atom + animation hooks; `transitions/*` onto `hooks/animation`
  (theme styles, reduced motion, `inert` while hidden); `alert.rs` roles; `icon.rs` decorative `aria-hidden`,
  its `width`/`height` props have no effect (theme `svg { width: 100% }`); `Chip.color` treats `None` as a default.

### R4. Cross-cutting
- [x] One value for boolean state attributes (2026-10-05): `"true"` everywhere (as react-aria-components), through
  `utils::data_attributes::flag`; `SelectTrigger` no longer repeats `use_button`'s `data-focus-visible`.
- [ ] Found by the book's dogfooding (2026-10-05, leptonic-5e), done: `KbdKey` reads `KeyboardKey::spoken_name`
  (visually hidden) for glyphs and abbreviations; `Icon` is decorative (`aria-hidden`) unless labelled (`role="img"`
  + `aria-label`, which was rendered as a literal `aria_label` attribute); the themed `Button` has `button_type` and
  an optional `on_press`. Open: legacy `TableHeaderCell` always attaches
  `use_press`, so static table headers (the book's documentation tables) are press targets. No `Disclosure` atom
  yet (the book composes `use_disclosure` + `use_disclosure_state` itself) and no hook-based Drawer/sheet (the book
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

## Done (summary)

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

### Next
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
- Kit `Disclosure` → Disclosure atom (R3f), `Sheet` → hook-based Drawer (R3k), search results → Autocomplete with
  arrow keys through results (R3j), themed `Button` with `button_type` (form demos still use native submit/reset
  buttons, R4), `KbdKey` accessible names (R4).
- Document once implemented: table sort announcement, tree tables; the `Label` atom (renders a `for` pointing at a
  generated id nothing has, R3c).

## Findings log (most recent first)

Library and book. Book entries are marked "Book".

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

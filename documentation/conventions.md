# Leptonic: guiding decisions and API conventions

Long-term rules for the library. Working rules for agents: `AGENTS.md`; implementation patterns:
`hooks-implementation.md`, `atoms-implementation.md`, `leptos-and-dom.md`.

## Guiding decisions

- **No APIs alien to Rust.** react-aria defines *behavior*; where its API shape is a JavaScript idiom
  (stringly-typed values, `string | number` unions, runtime-parsed specs, controlled/uncontrolled prop pairs, magic
  strings like `'all'`), we design a Rust-native API (enums, newtypes, traits, typed builders, `Default` + struct
  update syntax, signals) and record it as an `API DIFFERENCES` deviation with its reason. Use generics (or trait
  objects) wherever they make an API more capable or better typed (the user): JS `number`/`any`/string values are
  candidates for type parameters (C15, C16).
- **Faithful behavior, idiomatic shape.** react-aria is the behavioral spec (keyboard, ARIA, focus, edge cases); the
  shape is ours: signals instead of controlled/uncontrolled props, `CapturedElement` instead of refs, typed ARIA
  values and keyboard shortcuts. Every deviation is documented in the file's header (`porting.md`).
- **No legacy.** Nothing from react-aria's backwards-compatibility surface (deprecated props and aliases, IE-era key
  names, workarounds for unsupported browsers, `UNSTABLE_`/compat paths); modern browsers only (PointerEvent is
  assumed). No parallel old implementations of our own: a replacement migrates every user and deletes the old code
  in the same piece of work.
- **One way to do a thing** (the user): every task has exactly one API, helper or pattern. No second constructor,
  alias, conversion path, merge mechanism or test helper for the same job; when two exist, keep the better one,
  migrate every user (library, test-app, tests, book) and delete the other. Set-valued props and values are always
  `HashSet`, never `Vec`.
- **Delete what is no longer relevant** (the user): unused or obsolete public API is removed, not kept "just in
  case". Tell the book about removals it documents.
- **No prelude; one path per public item** (the user): no prelude module, no glob re-exports of atoms or hooks.
  Atoms and hooks are imported from their module (`leptonic::atoms::button::Button`,
  `leptonic::hooks::interactions::use_press`); everything else users need (`Out`, `ValueBinding`, `Locale`,
  `I18nProvider`, ARIA types, ...) is re-exported flat from `lib.rs` (`leptonic::Locale`) out of a private module, so
  there is no second path. Internal code imports the same way: hooks from their family, shared types from the crate
  root, dependency types from their crate; no crate-private convenience aliases.
- **Leptonic's code is user code** (the user, 2026-10-17: "I want users to be able to look at how we implement atoms
  and be able to copy most code for their own spin"): atoms and hooks building on hooks call the public hooks and use
  their `Part`s exactly as users do (`let UseButtonReturn { button, .. } = use_button(input); let (attrs, styles) =
  button.attrs();`); no crate-private second body of a hook (the former `use_x_raw`). What an atom or hook builds its
  public output with is public (`Part`, `Remount`, `signal_attr`, `Spread`, `data_flags`, `with_default_class`,
  `id_ref_prop`, `scoped_view`, ...); crate-private stays what is an implementation detail of one module or family
  (`hooks-implementation.md`, "One way: leptonic composes its public hooks").
- **Compose inputs, not DOM props.** A hook configuring an element another hook renders returns that hook's input
  (`use_menu_trigger` → `UseButtonInput`); independent hooks on one element are each spread onto it
  (`hooks-implementation.md`, "Combining Hooks").
- **No styled components** (the user, 2026-10-07): leptonic is hooks + atoms. Pre-built, opinionated components
  aren't reusable enough to justify their public surface; apps style atoms themselves. Kept: the optional atom theme
  (`atom-theme.md`), maintained; the book styles the atoms itself.
- **Event propagation follows upstream** (the user): only event types whose react-aria counterpart has
  `continuePropagation()` implement `Propagation` (`hooks-implementation.md`, "Event Propagation Control").
- **Tests are the definition of done.** Pure logic and `*_state` hooks get native unit tests, DOM behavior browser
  tests derived from react-aria's own tests; a hook without tests is not finished. Bugs and flaky tests are fixed when
  found, starting with a failing test (the user: the browser test suite is of utmost importance). `testing.md`.
- **Performance priorities** (the user, 2026-10-16): wasm binary size first, then allocation count, then allocated
  bytes; runtime speed after them, and never traded for size without measuring both. Performance is measured at the
  test-app's `opt-level = "z"`. Cost model and optimization rules: `performance.md`.
- **Leptonic's own `MaybeSignal<T>`** (the user, 2026-10-16): inputs and props callers mostly set to constants
  take `MaybeSignal<T>`, a fixed value or a `Signal` (`utils/maybe_signal.rs`). Fixed costs nothing: no allocation,
  no arena slot, no tracking, no render effect as attribute. Leptos deprecated its `MaybeSignal` (0.7.0-rc3,
  leptos-rs/leptos#3098) for ergonomics and maintenance, not performance; `Signal::from(value)` is a stored signal
  (2 allocations) that renders through a render effect, negligible in apps but not in leptonic's dozens of inputs
  per atom (`performance.md`, R3). Closures convert explicitly (`Signal::derive(f)`; coherence forbids `From<F>`).
- **Internationalization on ICU4X** (`icu_*` v2, compiled data), the Rust counterpart of react-aria's
  `@internationalized` packages: `js_sys::Intl` doesn't exist on the server, ICU4X runs everywhere. Apps may bake
  data for fewer locales (`build-performance.md`, "ICU4X data").
- **Dates and times on jiff** (or ICU4X's types), never `time` or `chrono`.
- **Dependencies** (the user): every dependency is declared with `default-features = false` and only the features the
  code needs (say why where it isn't obvious), at its latest version; feature flags gate the modules that need them.
- **Upstream is tracked by commit**: `// Upstream: <path> @ <sha>` headers and `scripts/upstream-drift.sh`
  (`porting.md`).

## API conventions

Every hook and atom follows these (decided 2026-10-05; open migrations are in `PLAN.md`). They are project-wide
deviations from react-aria, recorded once in `leptonic/src/hooks/mod.rs`; per-file deviation blocks list only what
goes beyond them.

- **C1 State flags:** `is_disabled`, `is_read_only`, `is_required`, `is_invalid`, as `Signal<bool>` (default
  `false`), in hook inputs and atom props alike: react-aria's names, one name per concept. DOM-level `*Props` keep DOM
  attribute names.
- **C2 Ids and strings:** ids are `Id` (`utils/id.rs`: a static `&str` or a shared `Arc<str>`, never an owned
  `Oco`; clones and attribute rendering never copy): `use_id(prefix) -> Id`, an element's own id `Option<Id>` in
  hook inputs and atom props (`id="x"` converts for free) and `Id` in props, derived ids via `Id::derived`. `Id` has
  no `Default` (an empty id is invalid, the user 2026-10-17): an id that may be missing is an `Option<Id>`. Id
  references (`aria_labelledby`, `aria_describedby`, `aria_details`, `aria_controls`, `aria_errormessage`, `for`) are
  reactive, as referenced elements come and go: `Signal<Option<Id>>` in hook inputs and props, joined with
  `IdRefs::derive` (constants folded once, a single reactive contribution passed on); a slot's `referenced_id` is a
  `Signal<Option<Id>>`. Atom reference props stay `MaybeProp<String>` (closures don't convert into leptonic's
  `MaybeSignal` yet) and convert with `utils::id::id_ref_prop` (fixed: once). `name`, `form`: `Option<String>`.
  User-visible text (`aria_label`, placeholders, value labels): `MaybeProp<String>` (it must be able to follow the
  locale). Never `&'static str` for dynamic values (it forced `Box::leak`).
- **C3 State shape:** `use_x_state(..) -> XState`, a `Copy` struct of read-only `Signal`s and methods (`set_value`,
  `toggle`, ...), not structs of `Callback` fields: methods are discoverable, typed and cheap.
- **C4 Hook-owned state:** a state hook takes `default_*` + `on_*_change`, or a `ValueBinding` to app state; every
  change goes through the hook's methods, so invariants and change callbacks always apply (replaces
  `useControlledState`; details: `hooks-implementation.md`, "Hook-Owned State"). Atoms (the user): a readable `<x>`
  and a writable `set_<x>: Out<T>`, never one combined binding; uncontrolled `default_<x>` + `on_<x>_change`.
  `is_invalid` is OR-ed into the validation state (no `Option<Signal<bool>>` whose `Some(false)` forces valid).
- **C5 Locale and direction from the i18n context:** hooks never take `is_rtl`/`locale`; `use_locale() ->
  Signal<Locale>` and `use_direction()`. `Locale: FromStr` with an error (no silent en-US fallback). Locale-derived
  defaults (first day of week, hour cycle) are `Option<_>` meaning "from the locale".
- **C6 One `Orientation`** in `utils`, without `Default` (callers name it; docs give the upstream default).
- **C7 No constructors on `*Input`s** (the user): struct literals naming every field; `Default` only when every field
  has a meaningful default (then struct update syntax); no placeholder defaults that build invalid configurations.
- **C8 One input:** `use_x(UseXInput)`; state goes into the input (`UseXInput { state, .. }`); settings the state
  holds are read from it, never repeated on the hook input.
- **C9 Parts naming** (the user, 2026-10-17): every props value a public hook returns is a `Part<P>`, named after its
  element, the hook's own included: `button`, `link`, `listbox`, `option`, `menu`, `item`, `tab`, `row`, `cell`,
  `grid`, `group`, `trigger`, `popover`, `overlay`, `arrow`, `input`, `label`, `field`, `description`,
  `error_message`, `panel`. Hooks adding a behavior to any element (`use_press`, `use_hover`, `use_focus`,
  `use_focus_within`, `use_focus_ring`, `use_focusable`, `use_keyboard`, `use_move`, `use_scroll_wheel`,
  `use_context_menu`, `use_interact_outside`, `use_has_tabbable_child`, `use_visually_hidden`, `use_clipboard`) name
  it `target`. A hook input for another hook is named after that element too (`trigger_button: UseButtonInput` next to
  the trigger's own `trigger` part, `text_field: UseTextFieldInput` next to the combo box's `input`). Description
  and error message come from `use_field` (`Part<SlotProps>`) everywhere.
- **C10 Callbacks:** `Option<Callback<NamedEvent>>`; `Arc<dyn Fn(&T) -> R>` only for predicates over borrowed data.
  No tuple arguments, no `Callback<()>` for configuration, no bools meaning modes. Delays are `Duration`, keys
  `KeyboardKey`, pointer types `PointerType`.
- **C11 Reactivity:** whatever a user could reasonably change at runtime is a `Signal<T>` with a default;
  `Option<Signal<T>>` only for "inherit vs. override" (documented).
- **C12 ARIA typing:** typed enums from `utils/aria.rs`, `tabindex` as `i32`.
- **C13 Units:** `Fraction` (0..=1) for percentages, `Point { x, y }` for coordinates, `Duration` for time.
- **C14 Field parts** (the user): one generic `Label`, `Description` and `FieldError` atom reading the `FieldContext`
  every field atom provides (react-aria-components' `LabelContext`/`TextContext`/`FieldErrorContext`); no
  per-family parts.
- **C15 Number values** (the user): the number field is generic over `NumberValue` (every primitive integer and
  float): exact integer stepping and clamping, min/max defaulting to the type's bounds, ICU4X decimals for parsing and
  formatting.
- **C16 Selection values** (the user): atoms whose value is a selection are generic over `V: SelectionValue`
  (`to_key`/`from_key`; `Key`, `String`, integers, enums through `selection_value!`): `RadioGroup<V>` (`Option<V>`),
  `CheckboxGroup<V>`/`ToggleButtonGroup<V>` (`HashSet<V>`), `Select<S>`/`ComboBox<S>` whose type is the selection
  mode (`S: SelectedValues`: `Option<V>` or `HashSet<V>`; no `selection_mode` prop). Default and controlled props of
  one state have the same type. Hooks keep `Key`s (`atoms/typed_values.rs` converts at the boundary); items take a
  `Key` (`value=Size::Small` through `From<Size> for Key`), and a key of no value of the group's type warns in debug
  builds.

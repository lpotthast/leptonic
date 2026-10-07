# Leptonic: guiding decisions and API conventions

Long-term rules for the library (moved out of `PLAN.md`, which holds open work only). `CLAUDE.md` has the
working rules for agents; `documentation/hooks-implementation.md` and `atoms-implementation.md` the implementation
patterns.

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
- **Tests are the definition of done.** Pure logic and `*_state` hooks get native unit tests (`testing::with_owner`
  runs their Effects). DOM behavior gets browser tests against `testing/test-app`, derived from react-aria's own
  tests. A hook without a test is not finished.
- **Docs follow code.** Every hook and atom change gets its book-ssr page updated: the library session
  sends the new API to the book session, which updates pages, API tables and demos (`CLAUDE.md`, "Working in
  Parallel").
- **No styled components** (the user's decision, 2026-10-07): leptonic is hooks + atoms. Pre-built, opinionated
  components aren't reusable enough to justify their public surface (apps style atoms themselves, quickly); the
  `components` layer was removed (2026-10-07). Kept: an optional CSS theme for the atoms, targeting their default class names and data
  attributes (as react-aria-components' starter styles target `.react-aria-*` classes). Kept and maintained (the user
  confirmed 2026-10-07, after a doubt about the cost); the book styles the atoms itself.
- **Default classes** (2026-10-07): every atom rendering its own element starts with
  `let classes = with_default_class("leptonic-<AtomName>", classes);` (`utils::default_class`) and documents it
  ("Default class: `leptonic-<AtomName>`."). The caller's classes add to the default (react-aria-components: a
  `className` replaces it). Atoms without an element of their own (providers, triggers, iterators) get none.
- **Dependencies** (the user's rule, 2026-10-07): every dependency is declared with `default-features = false` and
  only the features the code needs (say why where it isn't obvious); keep them at their latest versions.
- **Event propagation follows upstream** (the user's decision, 2026-10-07): only event types whose react-aria
  counterpart has `continuePropagation()` (press and keyboard events) implement `Propagation` (stopped by default,
  opt-in bubbling); all others keep upstream's fixed behavior (hover/focus never stop, move/scroll wheel/DnD always
  stop). Details: `hooks-implementation.md`, "Event Propagation Control".
- **Target modern browsers.** Where react-aria carries code for old browsers, we omit it.

## API conventions (decided 2026-10-05, audit §1)

Every hook and atom follows these (the remaining migrations are items in `PLAN.md`). The short table in
`documentation/hooks-implementation.md` ("API Conventions") and the global entries in `hooks/mod.rs` summarize them;
per-hook deviation blocks only list what goes beyond them.

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
  mutation path stays the hook's. Atoms (the user's rule, 2026-10-06): controlled state as two props,
  a readable `<x>` (`#[prop(into)]`: value, any signal, closure) and `set_<x>: Out<T>` (RwSignal, WriteSignal,
  StoredValue, closure, Callback; for `is_<x>` the setter is `set_<x>`), never one combined binding; uncontrolled `default_<x>` + `on_<x>_change`.
  `is_invalid: Signal<bool>` is OR-ed into the validation state (no `Option<Signal<bool>>` where `Some(false)` forces
  valid).
- **C5 Locale and direction come from the i18n context:** hooks never take `is_rtl`/`writing_direction`/`locale`.
  One reactive accessor `use_locale() -> Signal<Locale>` (plus `use_direction()`); `Locale: FromStr` with an error
  type (no silent en-US fallback). Locale-derived defaults (first day of week, hour cycle) are `Option<_>` = "from
  the locale".
- **C6 One `Orientation`** in `utils` (no `Default`; callers name it, the docs give the upstream default) instead of the
  collection one living in `use_checkbox_group.rs` plus `SliderOrientation`/`SeparatorOrientation`/
  `ToolbarOrientation` copies.
- **C7 Constructors:** none on `*Input` types (the user's rule, 2026-10-07; was `Input::new(required..)`): callers
  write struct literals naming every field; `Default` only when every field has a meaningful default (then struct
  update syntax); no placeholder defaults that build invalid configs.
- **C8 One input:** `use_x(UseXInput)`; state goes into the input (`UseXInput { state, .. }`); settings that live
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
- **C13 Units:** `Fraction` (0..=1) newtype for percentages, `Point { x, y }` for coordinates, `Duration` for time.
- **C14 Field parts (decided 2026-10-05 by the user):** one generic `Label`, `Description` and `FieldError` atom
  reading a `FieldContext` that every field atom provides (TextField, SearchField, NumberField, CheckboxGroup,
  RadioGroup, Select, ComboBox, ...), as react-aria-components' `LabelContext`/`TextContext`/`FieldErrorContext`.
  No per-family label/description/error parts.
- **C15 Number values (decided 2026-10-05 by the user):** the number field is generic over its value type
  (`NumberValue`, implemented for all primitive integers and floats): exact integer stepping and clamping, min/max
  defaulting to the type's bounds, ICU4X decimals for parsing and formatting (react-aria: JS numbers).

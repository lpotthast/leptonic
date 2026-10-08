# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Leptonic is a library for the Leptos web framework (Rust-based reactive web framework): accessible hooks ported from
react-aria and unstyled atoms built on them (ported from react-aria-components), plus an optional CSS theme for the
atoms.

## Code Quality Principles

- **Rust-native APIs**: Port react-aria's behavior faithfully, but never copy API shapes that are alien to Rust
  (stringly-typed values, `string | number` unions, runtime-parsed specs, props objects merged at runtime). Adapt them to idiomatic Rust/Leptos APIs (enums, newtypes, traits, typed builders,
  `Default` + struct update syntax, signals) and document the adaptation as an `API DIFFERENCES` deviation. Use
  generics (or trait objects) wherever they make an API more capable or better typed (e.g. a number field generic over
  its value type instead of JS's `number`).
- **State props of atoms** (the user's rule, 2026-10-06): controlled state is two props, a readable
  `<x>` (`#[prop(into)]` `Signal<T>`/`MaybeProp<T>`: a plain value, any signal or a closure) and a writable
  `set_<x>: Out<T>` (an `RwSignal`, `WriteSignal`, `StoredValue`, closure or `Callback`), never one combined binding
  that forces e.g. an `RwSignal` (for `is_<x>`, the setter is `set_<x>`: `is_open` + `set_open`). Uncontrolled: `default_<x>`, plus `on_<x>_change` to observe. Hooks keep
  `ValueBinding` internally (the atom builds it from the two props).
- **No constructors on input structs** (the user's rule, 2026-10-07): never add `new(..)` (or similar) functions to
  `*Input` types; callers write struct literals with every field named (or `Default` + struct update syntax), so
  creating an input is explicit and its field names stay visible.
- **No workarounds**: Do not use temporary workarounds instead of fixing real underlying issues. Always address the root
  cause.
- **Long-term solutions**: Prefer maintainable, long-term solutions over quick fixes that accumulate technical debt.
- **Clean code**: Strive for clean, readable, and maintainable code.
- **SSR safety**: Always use the SSR-safe `use_window()` and `use_document()` functions from `leptos-use` instead of
  `web_sys::window()` and `web_sys::document()`. The `leptos-use` variants return `None` during server-side rendering
  rather than panicking. Access the inner value via `.as_ref()` which returns `Option<&web_sys::Window>` /
  `Option<&web_sys::Document>`.
- **Event accessor invariants**: In DOM event handler closures, `e.target()` and `e.current_target()` are always
  `Some`. Use the `EventAccessors` extension trait (`expect_target()`, `expect_current_target()`) from `utils/mod.rs`
  instead of `.unwrap()`, `.expect()`, or `.and_then()`. **Exception:** `current_target` becomes `null` after the
  handler returns (per DOM spec), so stored/deferred events must use `if let Some(...)` for `.current_target()`.
- **Event propagation** (the user's decision, 2026-10-07): exactly the event types whose react-aria counterpart has
  `continuePropagation()` implement the sealed `Propagation` trait (`utils/propagation_control.rs`): press events
  (`PressEvent`) and keyboard events (`KeyboardEventWrapper`; react-aria's `BaseEvent<KeyboardEvent>`). They stop
  propagation by default; user handlers call `continue_propagation()` to let the event bubble. Every other event type
  (hover, move, long press, focus, focus within, scroll wheel, DnD) has no `Propagation` and keeps upstream's fixed
  behavior: hover and focus events never stop propagation (stopping `focusin`/`focusout` would break nested
  focus-within containers and collection listeners), move/scroll-wheel/DnD stop the native events they handle
  unconditionally. Details: `documentation/hooks-implementation.md`, "Event Propagation Control".

## Working in Parallel

Several agent sessions work in this repository at the same time, each owning one area:

- **Library** (`leptonic/`, `leptonic-theme/`, `testing/`): open work in `PLAN.md`; guiding decisions and API
  conventions in `documentation/conventions.md` (summary table in `documentation/hooks-implementation.md`); pitfalls
  in `documentation/lessons.md`; consumers in `documentation/consumers.md`; finished work in
  `documentation/history.md`; the atom theme (CSS ported from react-aria-components' starter styles) in
  `documentation/atom-theme.md`; compile times and binary sizes (measurements, findings, advice for users) in
  `documentation/build-performance.md`.
- **Book** (`examples/book-ssr/`): todos in `PLAN.md` (section "Book"); page structure, kit and writing rules in
  `documentation/documentation-strategy.md`; look, design tokens and which leptonic piece to use in
  `examples/book-ssr/STYLE_GUIDE.md`.

Rules for every agent:

- **Central todos.** Open work lives only in the root `PLAN.md`, never in scratch files, private notes or other plan
  files. `PLAN.md` holds open items only: move finished work to `documentation/history.md` and long-term rules or
  knowledge to the documentation files above. A finding about another area goes into that area's part of `PLAN.md` (or to its owner, who records it
  there).
- **Stay in your area.** When a library change breaks the book build, make only minimal compile fixes in book files
  and leave page texts to the book session (and tell it what changed).
- **The tree always compiles.** The user runs `just serve` on the shared working tree. Write files completely, wire
  them in only once they compile, and fix errors immediately.
- **Hands off the git index.** Don't run `git add`, `git mv`, `git rm`, `git reset` or `git restore --staged` unless
  the user asks for a commit; rename and delete files with plain `mv`/`rm`. The index is shared state.
- **Build against the live sources, in the shared agent target dirs.** Never build against a frozen copy of the
  library. Don't use the user's target dirs (contention) and don't create your own: all agents share one per project,
  `CARGO_TARGET_DIR=<repo>/target/agents` (leptonic, leptonic-theme), `<repo>/examples/book-ssr/target/agents` and
  `<repo>/testing/test-app/target/agents` (absolute paths; inside the app, so leptonic's build script finds it). Builds
  of a whole dependency tree cost gigabytes per directory. Prefer `cargo check` where you don't need a build. All
  builds use the same rustflags (repository `.cargo/config.toml`), so artifacts are shared; don't set `RUSTFLAGS`.
  Their incremental caches grow by tens of GB a day: when the disk runs low, `just clean-agent-incremental`.

## Build Commands

This project uses a `Justfile` for task automation. Run `just` to see all available commands.

**Initial setup:**

```bash
just once              # One-time dev environment setup (enables WASM target, installs tools)
```

**Development:**

```bash
just fmt               # Format all crates (stable fmt, then nightly fmt for import grouping; needs nightly toolchain)
just clippy            # Run `cargo clippy --tests` on every crate (lint levels configured in Cargo.toml [lints.clippy])
just test              # Run tests for all crates
just sort              # Sort dependencies in Cargo.toml files
just leptosfmt         # Format Leptos view macros
just serve             # Run the book-ssr documentation app for manual testing
```

**Single crate commands (faster feedback loop):**

```bash
cargo check -p leptonic                    # Quick compilation check
cargo test -p leptonic                     # Run tests for leptonic crate only
cargo test -p leptonic test_name           # Run a specific test
cargo clippy -p leptonic                   # Clippy on leptonic only
cargo check -p leptonic --features full    # Check atoms too (default feature is only `hooks`)
```

Styling props use the published `leptos-classes` and `leptos-styles` crates. `leptos-styles` only accepts typed
declarations (`WidthProperty.declare(..)`, typed custom properties via `css_custom_property!`). Properties `leptos-css`
does not cover yet (`position`, `transform`, `display`, ...) go through the explicit `*_unchecked` methods. For values
computed at runtime use the non-panicking helpers in `utils/css.rs` (`computed_pct`, `computed_px`, `computed_size`).
Never mix `class=classes` with `class:foo=` directives on one element; use `Classes::add_reactive` instead.

**Running the documentation app (primary manual testing target):**

```bash
just serve                                          # Recommended: runs book-ssr at https://127.0.0.1:4100
cd examples/book-ssr && cargo leptos serve          # Equivalent manual command
```

## Architecture

The library has two layers, hooks and atoms, plus an optional CSS theme for the atoms. The styled components layer
was removed (the user's decision, 2026-10-07; `documentation/conventions.md`, "No styled components").

1. **Hooks** (`leptonic/src/hooks/`) - Low-level interaction logic (usePress, useFocus, useCalendar). Handle ARIA
   attributes and accessibility. No rendering.
   All hooks are based on `react-aria` hooks from Adobe's react-spectrum library, checked out at `~/dev/react-spectrum`
   (sources in `packages/react-aria/src/` and `packages/react-stately/src/`). Each ported file starts with
   `// Upstream: <path> @ <commit>` lines; `scripts/upstream-drift.sh` lists upstream commits not yet absorbed (see
   `documentation/hooks-implementation.md`, "Tracking Upstream Changes").
   react-aria supports environments not supporting modern PointerEvent's. We DO NOT support these. Any hook we
   implement may assume that PointerEvent is available.

2. **Atoms** (`leptonic/src/atoms/`) - Unstyled single-element components built on hooks (Button, Link,
   Popover), ported from react-aria-components. Styled through their default class (`leptonic-<AtomName>`) and data
   attributes; `leptonic-theme` has an optional atom theme.

For more details, see: [documentation/architecture.md](documentation/architecture.md).
For implementation patterns, see: [documentation/hooks-implementation.md](documentation/hooks-implementation.md),
[documentation/atoms-implementation.md](documentation/atoms-implementation.md).
For book-ssr page structure and content guidelines,
see: [documentation/documentation-strategy.md](documentation/documentation-strategy.md).

**Other key directories:**

- `leptonic/src/utils/` - Typed ARIA types (`AriaRole`, etc.), event propagation control, focus/scroll utilities,
  i18n/locale, platform detection, color types.
- `leptonic-theme/` - The optional atom theme (SCSS, light and dark), copied into apps by leptonic's build script

### ICU4X for Internationalization

The i18n layer uses [ICU4X](https://github.com/unicode-org/icu4x) (`icu_*` crates, v2) — the Rust-native equivalent
of react-aria's `@internationalized` packages. ICU4X is maintained by the Unicode Consortium, is pure Rust,
WASM-compatible, and SSR-safe (no JS runtime needed on the server).

**Why ICU4X over `js_sys::Intl`:** The browser `Intl` APIs panic during SSR because there is no JS runtime. ICU4X
provides the same locale-aware functionality as compiled Rust with baked-in CLDR data.

**Modules using ICU4X:**

| Module                         | ICU4X crate                      | Purpose                                                         |
|--------------------------------|----------------------------------|-----------------------------------------------------------------|
| `utils/i18n.rs`                | `icu_locale`                     | Locale parsing, script-based RTL detection via `LocaleExpander` |
| `utils/filter.rs`              | `icu_collator`, `icu_normalizer` | Locale-aware string collation and filtering (SSR-safe)          |
| `utils/number_formatter.rs`    | `icu_decimal`                    | Locale-aware number formatting                                  |
| `utils/date_time_formatter.rs` | `icu_datetime`                   | Locale-aware date/time formatting                               |
| `utils/list_formatter.rs`      | `icu_list`                       | Locale-aware list formatting ("A, B, and C")                    |
| `utils/plurals.rs`             | `icu_plurals`                    | Plural category lookup for ARIA labels                          |

All ICU4X crates use the default compiled-data mode (CLDR baked into each sub-crate). No datagen step is needed.

### Hook Implementation

See [documentation/hooks-implementation.md](documentation/hooks-implementation.md) for detailed patterns:

- Attrs tuple pattern and spreading
- Element reference pattern (`IntoElementMaybeSignal`)
- Element capture pattern (`ElementCaptureAttr`)
- Event handler patterns (Copy requirements, cleanup, dynamic listeners)
- React-aria deviation documentation format
- Hook-owned state (C4): a state hook takes `default_*` + `on_*_change`, or a `ValueBinding` to app state (the
  atoms build it from their `x` + `set_x` props). Callers read `Signal`s and change the state only through the
  state's methods, so the hook's invariants and change callbacks always apply. This replaces React Aria's
  `useControlledState` value/defaultValue pair. See `documentation/hooks-implementation.md`.
- Book-SSR documentation page structure

## Book-SSR (Documentation App)

The `examples/book-ssr/` directory contains the primary documentation site for leptonic and serves as the main app for
manual testing during development. It is named "book" following Rust ecosystem convention (like "The Rust Book").

- **Running**: `just serve` (or `cd examples/book-ssr && cargo leptos serve`). Available at `https://127.0.0.1:4100`.
  `just book-serve-isolated [port]` serves a second instance (default port 4300) with its own target directory.
- **Tests**: unit tests (`cargo test --features ssr --lib`, including `kit::api_check`, which compares every API table
  with the library source) and browser tests (`just book-browser-test`, `examples/book-ssr/tests/`): every page loads
  without errors, demos are readable in the dark theme, internal links and anchors resolve, pages fit a 390px screen,
  and the Markdown export lists every page. `BOOK_TEST_PAGES=<text>` limits them to matching pages.
- **Built with leptonic**: the book uses leptonic for everything leptonic provides (buttons, links, dialogs,
  disclosures, toggles, tables, keys, ...); its own widgets are compositions of leptonic hooks and atoms. Library
  gaps are fixed in the library, never worked around in the book. The book has its own look: it styles every atom
  itself (default classes `leptonic-<AtomName>` + data attributes, book tokens) and never loads the atom theme. See
  `examples/book-ssr/STYLE_GUIDE.md`.
- **Quality bar**: Must always compile and have zero clippy lints (checked with `clippy::all` and `clippy::pedantic` via
  `[lints.clippy]` in its `Cargo.toml`).
- **Dependency**: Uses `leptonic` via path dependency with features `atoms` and `clipboard`; `syntax-highlight` only
  in its `ssr` feature (code blocks are highlighted on the server, `documentation/build-performance.md`).
- **ICU4X data**: the book bakes its own ICU4X data (`examples/book-ssr/icu4x-data`, set up in its
  `.cargo/config.toml`). After updating the `icu_*` crates, regenerate it with `just book-icu-data`: data of another
  ICU4X minor version breaks every book build.
- **Not a workspace member**: Excluded from the root workspace; managed via the root Justfile.
- **Page structure**: The sidebar (`src/nav.rs`) has three parts: guides; **concepts** (every UI element, e.g.
  Button or Slider, as one entry whose layers are tabs, grouped by purpose); and **building blocks** (hooks, atoms and
  utilities many concepts share, e.g. `use_press`, grouped into areas like Interactions or Focus). Page files live in
  `src/pages/documentation/` by layer (`hooks/`, `atoms/`, `utils/`), plus `concepts/` (concept overviews),
  `groups/` (group and area overviews, with recipes for layout pieces leptonic has no atom for) and
  `getting_started/` (guides). Pages are written with the page kit in
  `src/kit/`. See `documentation/documentation-strategy.md` for terminology,
  page types, navigation rules, the kit and writing guidelines.
- **When adding or modifying hooks or atoms**: The corresponding book-ssr documentation page should be
  updated or created to demonstrate the change.

## Feature Flags

Default feature is `hooks`. Feature hierarchy: `hooks` → `atoms` (each gates its module). Every dependency is
declared with `default-features = false` and only the features it needs.

- `hooks` - Low-level interaction hooks
- `atoms` - Unstyled single-element components (requires hooks)
- `clipboard` - Clipboard support (`utils::clipboard`)
- `syntax-highlight` (syntect, `utils::syntax_highlight`) - Syntax highlighting into classed HTML
- `ssr` / `hydrate` - Server-side rendering support
- `nightly` - Enables `leptos/nightly`
- `full` - hooks, atoms, clipboard, syntax-highlight (not ssr/hydrate/nightly)

## Key Types

- `Out<O, S>` - Flexible output type for atom props, accepts WriteSignal, RwSignal, Callback, or function pointers
- `Mount` - Controls when child views are mounted (Once vs WhenShown)
- `Width`, `Height` (aliases of `CssDimension`), `Margin`, `Padding`, `FontWeight` - CSS types from `utils/css.rs`,
  re-exported at the crate root

## Configuration

**Required in `.cargo/config.toml`:**

```toml
[build]
rustflags = ["--cfg=web_sys_unstable_apis"]
```

Leptonic is written against web-sys' unstable signatures (e.g. `Element::scroll_top()` returning `f64`), so this flag
is required to build it, not only for leptos-use functions.

**Build script metadata** (in consuming app's Cargo.toml):

```toml
[package.metadata.leptonic]
style-dir = "style"   # The build script copies the optional atom theme's SCSS to `<style-dir>/leptonic`
```

The build script finds that `Cargo.toml` by walking up from `OUT_DIR`. A `CARGO_TARGET_DIR` inside the app (e.g.
`examples/book-ssr/target/<name>`) works as is; one elsewhere needs `LEPTONIC_APP_DIR=<app dir>`, otherwise the
theme is silently not copied.

## Workspace Structure

- `leptonic/` - The library (hooks, atoms, utils)
- `leptonic-theme/` - The atom theme (SCSS)
- `examples/book-ssr/` - Documentation app and primary manual testing target
- `examples/leptonic-template-*` - Starter templates (git submodules; atoms + the atom theme)
- `testing/test-app/` - Leptos app that browser tests drive (`just serve-test-app` serves it at
  `http://127.0.0.1:4200` for manual inspection)

Only `leptonic` and `leptonic-theme` are workspace members; `examples/` and `testing/` are excluded and have their own
`Cargo.lock`, so use `--manifest-path` (as the Justfile does) rather than `-p` for them.

## Testing

When generating tests, use the `assertr` library for assertions instead of standard `assert!` macros.

Native unit tests of hooks run inside `crate::testing::with_owner(|| ..)`; call `flush_effects()` to run their Effects
(`documentation/hooks-implementation.md`, "Native Tests").

### Browser Tests

Browser tests live in `leptonic/tests/` and drive the test-app in `testing/test-app/`. They use `leptos-browser-test`
(starts `cargo leptos serve` on a random port) and `browser-test` (Chrome for Testing + chromedriver, one fresh
WebDriver session per test, 4 tests in parallel by default, `thirtyfour` re-exported as `browser_test::thirtyfour`).
Tests must not depend on each other or on shared server state; checks of the whole run go into `ui_tests::after_all()`.

- **Fixtures**: every test page lives in its own module under `testing/test-app/src/pages/{atoms,hooks}/`
  and is registered in `FIXTURES` (`testing/test-app/src/pages/mod.rs`). It is served at `/{group}/{name}`.
- **Hydration**: the test-app sets `data-hydrated` on `<body>` once hydration finished. `BaseActions::goto_path`
  waits for it, so tests never interact with a page whose event handlers aren't attached yet.
- **Tests**: page objects in `tests/pages/` (implement `BaseActions` to get shared helpers: clicking, reading text,
  focus/active-element queries, keyboard input, waiting), test implementations in `tests/ui_tests/test_*.rs`
  (implement `BrowserTest<str>`; the context is the app's base URL). Register new tests in `ui_tests::all()`.
- **Failures fail `cargo test`**: the runner uses `FailurePolicy::RunAll` and reports every failing test.
  Assertions use `assertr` (panics are reported as test failures); helpers return `Result<_, rootcause::Report>`.
- **Prefer waiting over sleeping**: use the polling helpers (`wait_for_selector`, `wait_for_text`,
  `wait_for_active_text`, `wait_for_attr`, `wait_for_prop`, and for anything else the `wait_for!`/`wait_until!`
  macros of `tests/polling/mod.rs`) instead of fixed sleeps or hand-rolled loops; focus and state often change in
  effects after the event. Negative checks ("nothing changed") use `stays!` (re-checked over 300 ms; `stays_for!`
  over a longer window), not a single read.
- **Find elements as users do**: by role and text (`by_role_and_text`, `css("[role=listbox]")`). Atoms generate
  their own ids.
- **Derive tests from react-aria**: react-aria's own tests (`../react-spectrum/packages/react-aria/test/`,
  `react-aria-components/test/`) specify expected behavior. Base our tests on them and name the mirrored upstream test
  file in an `// Upstream:` header of the test file, so `scripts/upstream-drift.sh` reports upstream test changes.
- **Known issues**: behavior known to be broken lives in `*KnownIssues` tests that only run with
  `BROWSER_TEST_KNOWN_ISSUES=1` (see `ui_tests::all()`). Move a check into the regular test once it's fixed.
- **Hydration**: `test_hydration_ids.rs` compares server-rendered ids with the hydrated DOM and checks id
  references on every fixture the test app's index page lists (in 4 parallel shards; no list to maintain).
  `test_server_panics.rs` (runs last) fails the run if the server panicked.
- **Running**: `just browser-test`. `BROWSER_TEST_VISIBLE=1` shows the browser, `BROWSER_TEST_PAUSE=1` pauses before
  each test, `BROWSER_TEST_DRIVER_OUTPUT=1` forwards chromedriver output (or `just browser-test-visible`).
  `BROWSER_TEST_FILTER=<text>` runs only the tests whose name contains `<text>` (e.g. `grid_tests`).
  `BROWSER_TEST_PARALLELISM=<n>` sets how many tests run at once (`1`: sequential). `TEST_APP_TARGET_DIR=<dir>`
  builds the test-app there instead of in the inherited `CARGO_TARGET_DIR` (agents:
  `CARGO_TARGET_DIR=<repo>/target/agents TEST_APP_TARGET_DIR=<repo>/testing/test-app/target/agents`). The run summary lists the
  slowest tests and steps; `BROWSER_TEST_LOG_STEPS=1` logs every step. Never run two suites of one app at the same
  time: they share the app's build directory.
- **Toolchain**: the installed `wasm-bindgen` CLI version must match the `wasm-bindgen` version in the test-app's
  `Cargo.lock`; otherwise `cargo leptos serve` fails. Update the lockfile (`cargo update -p wasm-bindgen -p js-sys
  -p web-sys -p wasm-bindgen-futures`) or the CLI.
- **Always execute browser tests** when adding or modifying them. Compilation alone is not sufficient — browser tests
  must be run and pass before considering the work complete.

## Clippy Lint Overrides

The root `Cargo.toml` `[workspace.lints.clippy]` sets `all` and `pedantic` to **deny**, with a list of allowed
overrides. `leptonic/src/lib.rs` repeats some of these as crate-level `#![allow(...)]` (CLI `-D clippy::pedantic`
would otherwise override Cargo.toml) and additionally allows `ignored_unit_patterns` and `type_complexity`, which fire
on `view!`/`#[component]` expansions. Treat these files as the source of truth.

Book-ssr is not a workspace member and has its own `[lints.clippy]` section in `Cargo.toml` that sets `all` and
`pedantic` to deny, with its own allows (notably `let_unit_value`, because Leptos view macros generate unit-value
let-bindings).

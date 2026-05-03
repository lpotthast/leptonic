# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Leptonic is a component library for the Leptos web framework (Rust-based reactive web framework). It provides UI
components with theming capabilities, built on a layered architecture of hooks, atoms, and components.

## Code Quality Principles

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
- **Event propagation**: Leptonic events stop propagation by default. User handlers call `continue_propagation()` to
  opt in to bubbling. Implemented via the sealed `Propagation` trait and `PropagationControl` from
  `utils/propagation_control.rs`. All user-facing event types must implement `Propagation`.

## Build Commands

This project uses a `Justfile` for task automation. Run `just` to see all available commands.

**Initial setup:**

```bash
just once              # One-time dev environment setup (enables WASM target, installs tools)
```

**Development:**

```bash
just fmt               # Format all crates with cargo fmt
just clippy            # Run clippy (lint levels configured in Cargo.toml [lints.clippy])
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
```

**Running the documentation app (primary manual testing target):**

```bash
just serve                                          # Recommended: runs book-ssr at https://127.0.0.1:4100
cd examples/book-ssr && cargo leptos serve          # Equivalent manual command
```

## Architecture

The library follows a three-layer hierarchy:

1. **Hooks** (`leptonic/src/hooks/`) - Low-level interaction logic (usePress, useFocus, useCalendar). Handle ARIA
   attributes and accessibility. No rendering.
   All hooks are based on `react-aria` hooks from Adobe's react-spectrum library, checked out at `~/dev/react-spectrum`.
   react-aria supports environments not supporting modern PointerEvent's. We DO NOT support these. Any hook we
   implement may assume that PointerEvent is available.

2. **Atoms** (`leptonic/src/atoms/`) - Headless/unstyled single-element components built on hooks (Button, Link,
   Popover). Easy to style.

3. **Components** (`leptonic/src/components/`) - Pre-built, feature-rich components built on atoms. Include styling and
   complex behavior (Modal, Select, DateSelector, Table, Toast, Tabs, etc.).

For more details, see: [documentation/architecture.md](documentation/architecture.md).
For implementation patterns, see: [documentation/hooks-implementation.md](documentation/hooks-implementation.md),
[documentation/atoms-implementation.md](documentation/atoms-implementation.md),
[documentation/components-implementation.md](documentation/components-implementation.md).
For book-ssr page structure and content guidelines,
see: [documentation/documentation-strategy.md](documentation/documentation-strategy.md).

**Other key directories:**

- `leptonic/src/contexts/` - Global event contexts (click, keyboard, pointer, scroll, resize)
- `leptonic/src/utils/` - Typed ARIA types (`AriaRole`, etc.), event propagation control, focus/scroll utilities,
  i18n/locale, platform detection, color types.
- `leptonic-theme/` - Theme system with SCSS stylesheets and light/dark themes

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
- Hook-owned state: Hooks always create and own their `WriteSignal` internally. Callers get
  read-only `Signal<T>` output and must use the hook's mutation callbacks. This is an intentional
  deviation from React Aria's `useControlledState` pattern. See `documentation/hooks-implementation.md`.
- Book-SSR documentation page structure

## Book-SSR (Documentation App)

The `examples/book-ssr/` directory contains the primary documentation site for leptonic and serves as the main app for
manual testing during development. It is named "book" following Rust ecosystem convention (like "The Rust Book").

- **Running**: `just serve` (or `cd examples/book-ssr && cargo leptos serve`). Available at `https://127.0.0.1:4100`.
- **Quality bar**: Must always compile and have zero clippy lints (checked with `clippy::all` and `clippy::pedantic` via
  `[lints.clippy]` in its `Cargo.toml`).
- **Dependency**: Uses `leptonic` via path dependency with `features = ["full"]`.
- **Not a workspace member**: Excluded from the root workspace; managed via the root Justfile.
- **Page structure**: Pages live in `src/pages/documentation/`, organized by layer (`hooks/`, `atoms/`, `components/`)
  and by concept (`concepts/`, `domains/`). The concept-based organization groups related hooks/atoms/components under
  a single concept page (e.g., Button, Slider) while domain pages group behavioral hook families (e.g., Interactions,
  Focus). See `documentation/documentation-strategy.md` for the full page type taxonomy.
- **When adding or modifying hooks, atoms, or components**: The corresponding book-ssr documentation page should be
  updated or created to demonstrate the change.

## Feature Flags

Default feature is `hooks`. Feature hierarchy: `hooks` → `atoms` → `components`

- `hooks` - Low-level interaction hooks
- `atoms` - Headless base components (requires hooks)
- `components` - Full pre-built components (requires atoms)
- `clipboard` - Clipboard support (requires `web_sys_unstable_apis` rustflag)
- `tiptap` - Rich text editor integration
- `ssr` / `hydrate` - Server-side rendering support
- `full` - All features combined

## Key Types

- `Out<O, S>` - Flexible output type for component props, accepts WriteSignal, RwSignal, Callback, or function pointers
- `Mount` - Controls when child views are mounted (Once vs WhenShown)
- `Size`, `Width`, `Height`, `Margin` - CSS size/dimension types

## Configuration

**Required in `.cargo/config.toml`:**

```toml
[build]
rustflags = ["--cfg=web_sys_unstable_apis"]
```

This is required for leptos-use functions.

**Build script metadata** (in consuming app's Cargo.toml):

```toml
[package.metadata.leptonic]
style-dir = "style/leptonic"   # Where to output generated SCSS
js-dir = "public/js"           # Where to output JS dependencies (for tiptap)
```

## Workspace Structure

- `leptonic/` - Core component library
- `leptonic-theme/` - Theme generation and SCSS
- `examples/book-ssr/` - Documentation app and primary manual testing target
- `examples/leptonic-template-*` - Starter templates (git submodules)

## Testing

When generating tests, use the `assertr` library for assertions instead of standard `assert!` macros.

### Browser Tests

Browser tests live in `leptonic/tests/` and use **thirtyfour** (Selenium WebDriver) against a test-app served at
`http://127.0.0.1:4200`. The test-app source is in `testing/test-app/`.

- **Running**: `just browser-test` (or `BROWSER_TEST_VISIBLE=1 just browser-test` for visual debugging).
- **Always execute browser tests** when adding or modifying them. Compilation alone is not sufficient — browser tests
  must be run and pass before considering the work complete.

## Clippy Lint Overrides

These lints are allowed in workspace: `option_if_let_else`, `module_name_repetitions`, `must_use_candidate`,
`wildcard_imports`

Book-ssr is not a workspace member and has its own `[lints.clippy]` section in `Cargo.toml` that sets `all` and
`pedantic` to deny, with additional allows: `must_use_candidate`, `wildcard_imports`, `module_name_repetitions`, and
`let_unit_value` (Leptos view macros generate unit-value let-bindings).

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

## Build Commands

This project uses a `Justfile` for task automation. Run `just` to see all available commands.

**Initial setup:**

```bash
just once              # One-time dev environment setup (enables WASM target, installs tools)
```

**Development:**

```bash
just fmt               # Format all crates with cargo fmt
just clippy            # Run clippy with strict flags (-Dclippy::all -Dclippy::pedantic)
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
cargo clippy -p leptonic -- -Dclippy::all -Dclippy::pedantic  # Clippy on leptonic only
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

**Other key directories:**

- `leptonic/src/contexts/` - Global event contexts (click, keyboard, pointer, scroll, resize)
- `leptonic/src/utils/` - Utility functions (ARIA, color, time, signals)
- `leptonic-theme/` - Theme system with SCSS stylesheets and light/dark themes

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
- **Quality bar**: Must always compile and have zero clippy lints.
- **Dependency**: Uses `leptonic` via path dependency with `features = ["full"]`.
- **Not a workspace member**: Excluded from the root workspace; managed via the root Justfile.
- **Page structure**: Pages live in `src/pages/documentation/`, organized by layer — `hooks/`, `atoms/`, `components/`
  (components further split into `input/`, `layout/`, `feedback/`, `general/`, `animation/`).
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

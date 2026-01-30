# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Leptonic is a component library for the Leptos web framework (Rust-based reactive web framework). It provides UI components with theming capabilities, built on a layered architecture of hooks, atoms, and components.

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
```

**Running examples:**
```bash
cd examples/book-ssr && cargo leptos watch     # Main documentation site (SSR)
cd examples/leptonic-template-csr && trunk serve    # CSR template
cd examples/leptonic-template-ssr && cargo leptos watch   # SSR template
```

## Architecture

The library follows a three-layer hierarchy:

1. **Hooks** (`leptonic/src/hooks/`) - Low-level interaction logic (usePress, useFocus, useCalendar). Handle ARIA attributes and accessibility. No rendering.

2. **Atoms** (`leptonic/src/atoms/`) - Headless/unstyled single-element components built on hooks (Button, Link, Popover). Easy to style.

3. **Components** (`leptonic/src/components/`) - Pre-built, feature-rich components built on atoms. Include styling and complex behavior (Modal, Select, DateSelector, Table, Toast, Tabs, etc.).

**Other key directories:**
- `leptonic/src/contexts/` - Global event contexts (click, keyboard, pointer, scroll, resize)
- `leptonic/src/utils/` - Utility functions (ARIA, color, time, signals)
- `leptonic-theme/` - Theme system with SCSS stylesheets and light/dark themes

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
- `examples/` - Git submodules with template projects (book-ssr, template-csr, template-ssr, template-tauri)

Examples are git submodules. Clone with `--recurse-submodules`.

## Clippy Lint Overrides

These lints are allowed in workspace: `option_if_let_else`, `module_name_repetitions`, `must_use_candidate`, `wildcard_imports`

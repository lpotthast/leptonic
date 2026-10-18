# AGENTS.md

Leptonic is a library for the Leptos web framework: accessible **hooks** ported from react-aria/react-stately
(`leptonic/src/hooks/`: behavior and ARIA, no rendering), unstyled single-element **atoms** on them, ported from
react-aria-components (`leptonic/src/atoms/`, feature `atoms`), and an optional CSS theme for the atoms
(`leptonic-theme/`). Upstream is checked out at `~/dev/react-spectrum` (`packages/react-aria/src/`,
`packages/react-stately/src/`, `packages/react-aria-components/src/`). The book (`examples/book-ssr/`) documents the
library and is the app for manual testing; the browser tests drive `testing/test-app/`. Everything beyond this file
is in `documentation/` (index: `documentation/README.md`); open work in `PLAN.md`.

## Test runs: never in parallel, one agent runs them for all

Every test run (`cargo test`, `just browser-test`, `just browser-perf`, the book's suites) builds the shared tree
(the test-app alone takes minutes); parallel runs compete for target dirs, locks and CPU, and timing-sensitive tests
fail.

- One agent controls test execution: the session that spawned the others, or the one the user names. The others
  never start test runs: they request one (filters; check or budget update) and keep working meanwhile. The
  controlling agent batches the waiting requests into one build (budget updates first, then checks) and hands each
  requester its own results.
- The tree compiles before you request: one agent's unfinished file breaks the run for everyone.
- While working on a hook or atom, run only its tests (`BROWSER_TEST_FILTER=<subject>`: its behavior and performance
  tests); the full suite only as the final verification step.
- Browser tests are run, not only compiled, before work counts as done.

## Working in parallel

Several agent sessions share the working tree, each owning an area: the **library** (`leptonic/`, `leptonic-theme/`,
`testing/`) or the **book** (`examples/book-ssr/`; its rules: `documentation/documentation-strategy.md`,
`examples/book-ssr/STYLE_GUIDE.md`).

- **Stay in your area.** A finding about another area goes into its part of `PLAN.md` (or to its owner). When a
  library change breaks the book build, make only minimal compile fixes there and tell the book session what changed.
- **The tree always compiles** (the user runs `just serve` on it): write files completely, wire them in once they
  compile, fix errors at once.
- **Fix and format tools only on your own files** (`rustfmt <file>`, `leptosfmt <file>`), never on the tree
  (`cargo fmt`, `just fmt`, `cargo fix`, `cargo clippy --fix`): they rewrite other agents' files mid-change.
- **Hands off the git index**: no `git add`, `git mv`, `git rm`, `git reset` or `git restore --staged` unless the user
  asks for a commit; rename and delete with plain `mv`/`rm`.
- **Shared agent target dirs, live sources**: `CARGO_TARGET_DIR=<repo>/target/agents` (leptonic, leptonic-theme),
  `<repo>/examples/book-ssr/target/agents`, `<repo>/testing/test-app/target/agents` (absolute paths; test runs set
  `TEST_APP_TARGET_DIR` to the last). Never the user's target dirs or one of your own, never a frozen copy of the
  library, never `RUSTFLAGS` (all builds share `.cargo/config.toml`'s). Prefer `cargo check`. Low disk:
  `just clean-agent-incremental`.
- **`PLAN.md` is the only todo list**: open items only, each short; finished work goes to `documentation/history.md`
  as a short entry, long-term knowledge into the document `documentation/README.md` names for it. Documentation
  holds valuable long-term knowledge only (decisions with their reasons, rules, pitfalls with their causes), no
  narratives, measurement logs or what the code already says. **Knowledge to keep goes into this file or
  `documentation/`, never into an agent's private memory.**

## How the user wants us to work

- **Only one layer of subagents**: the main session may spawn subagents; they never spawn their own.
- **No scope creep**: do exactly the assigned items; ideas go into `PLAN.md`. The user's own crates
  (`~/dev/<name>`: leptos-element-capture, leptos-classes, leptos-styles, leptos-css, browser-test, assertr, ...)
  change only when asked, on a branch; publishing is the user's call. leptonic depends on their crates.io versions.
- **Question decisions** (your own, the user's, earlier ones) and put structural choices to the user as decisions
  (a second code path, a new public concept, a trade-off): never decide them silently or bury them in a summary.
- **One way to do a thing**: one API, helper or test helper per job; delete what's no longer relevant (no legacy,
  no deprecated or compat paths, from react-aria or our own past); explicit code over macros and indirection, even
  when repetitive; no new checker/linter/generator tooling without asking (`kit::api_check` is the ceiling).
- **Atoms are copyable examples**: atoms and leptonic's own hook composition use the public hooks exactly as users do;
  no crate-private shortcuts or second code paths for performance (win it inside the shared types, e.g. `Part`).
- **Before creating a file**, check its path doesn't exist; **before adding a field to a public input**, grep the
  book, `testing/` and `leptonic/tests` for its struct literals and update them in the same change.
- **Logging**: `tracing_subscriber::fmt` layers always keep `.with_file(true).with_line_number(true)`. **Client
  code** stays Rust (e.g. `reqwest` for downloads, never hand-written `fetch` interop).

## Rules for the code

- **Rust-native APIs.** Port react-aria's behavior faithfully, never its JavaScript API shapes (stringly-typed
  values, `string | number` unions, runtime-parsed specs, props merged at runtime): enums, newtypes, traits,
  generics, `Default` + struct update syntax, signals, each recorded as an `API DIFFERENCES` deviation. The API
  conventions C1–C16: `documentation/conventions.md`.
- **State props of atoms**: a readable `<x>` (`#[prop(into, optional)] Option<Signal<T>>`: value, signal or closure)
  and a writable `set_<x>: Option<Out<T>>` (`is_<x>` pairs with `set_<x>`), never one combined binding; uncontrolled:
  `default_<x>` + `on_<x>_change`. Hooks take a `ValueBinding` (`ValueBinding::from_state_props`).
- **No constructors on `*Input` structs**: callers write struct literals naming every field (or `Default` + struct
  update syntax).
- **Props go to one element at a time**: every props value a hook returns is a `Part` (fields named after their
  element: `UseButtonReturn { button, .. }`, `button.attrs()`); props types are never `Clone`; parts mount again by
  building fresh props from cheap handles (`Copy` signals, refcounted `Id`s and handlers), never by cloning stored
  attributes. Ids are `Id`. Avoid clones and types that allocate per copy (`String` ids, `Oco::Owned`). Don't extend
  `MaybeSignal` beyond its current use without the user (`documentation/conventions.md`).
- **No workarounds**: fix root causes. **Bugs and flaky tests are fixed when found** (root cause, failing test
  first); `PLAN.md` only for what can't be done now.
- **SSR safety**: `leptos_use::use_window()`/`use_document()` (`None` on the server; `.as_ref()`), never
  `web_sys::window()`/`document()`.
- **Event accessors**: in DOM handlers `expect_target()`/`expect_current_target()` (`utils/dom_ext.rs`), never
  `unwrap`/`expect`/`and_then`; a stored or deferred event uses `if let Some(..)` for `current_target()` (null once
  the handler returned).
- **Event propagation**: only `PressEvent` and `KeyboardEventWrapper` implement `Propagation` (stopped by default,
  `continue_propagation()` lets the event bubble), as only their react-aria counterparts have
  `continuePropagation()`; every other event keeps upstream's fixed behavior (`documentation/hooks-implementation.md`,
  "Event Propagation Control").
- **Performance priorities**: wasm binary size first, then allocation count, then allocated bytes; never trade size
  for speed without measuring both. Measure at the test-app's `opt-level = "z"`. Knowledge:
  `documentation/performance.md`. Optimizations that keep behavior identical to react-aria's are done right away
  (the user, 2026-10-18), API changes included when they are minor and usability stays the same; one that changes
  behavior is presented to the user first, with the exact behavior difference. Patterns to avoid, and how to find
  them in the budgets and the code: `performance.md` (R1–R17, section 5) and `leptos-and-dom.md` ("State
  Corrections", "Ownership and Cleanup"); check new and changed code against them, and add each newly found pattern
  there in abstract form.
- **Tests**: improving the integration tests is of utmost importance; `assertr`, never std `assert!`; one way per
  check (`documentation/testing.md`); cases derived from react-aria's own tests (`// Upstream:` header); every hook
  and atom has a performance test. A hook or atom change updates its book page (via the book session).

## Commands

```bash
just serve                # the book at https://127.0.0.1:4100
just serve-test-app       # the test-app at http://127.0.0.1:4200
cargo check -p leptonic --features full            # quick check, atoms included
cd testing/test-app && cargo check --features ssr  # the apps from their directory (their .cargo/config.toml)
just clippy               # every crate (lint levels: Cargo.toml, lib.rs); `just verify`: everything that must pass
just browser-test         # the browser suite (through the agent running tests, above)
just browser-test-timed   # the suite sequentially, timing each test per machine (`documentation/testing.md`)
just book-icu-data; just test-app-icu-data   # after updating the icu_* crates: other ICU4X data breaks the build
```

`examples/` and `testing/` are not workspace members: use `--manifest-path` or their directory, not `-p`. Features:
`leptonic/Cargo.toml`. The test-app builds only while `~/dev/leptos` is on branch `leptonic-metrics` (it patches
`reactive_graph`; `documentation/testing.md`).

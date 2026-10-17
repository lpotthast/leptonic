# Porting from react-aria

react-aria defines the behavior; the API shape is ours (`conventions.md`, "Guiding decisions"). This file covers the
porting itself, for hooks, atoms, the atom theme (`atom-theme.md`) and tests alike: where upstream lives, how a file
records what it ports and from which commit, how it documents deviations, and how to absorb upstream changes.

Upstream is react-spectrum, checked out at `~/dev/react-spectrum`. Since its package consolidation (2026-03) the
sources live in `packages/react-aria/src/<package>/`, `packages/react-stately/src/<package>/` and
`packages/react-aria-components/src/`; the old `packages/@react-aria/*` packages only re-export them.

## Tracking Upstream Changes

Every file that ports upstream code starts with `// Upstream:` lines listing ALL relevant upstream files (the user's
rule): the sources it ports (react-aria hook, react-stately state, react-aria-components component, shared utils it
mirrors) and the upstream test files of that behavior (react-aria's hook tests, react-stately's state tests,
react-aria-components' and `@adobe/react-spectrum`'s component tests), since the tests specify the behavior:

```rust
// Upstream: react-aria/src/interactions/usePress.ts @ 6f664fe911
// Upstream: react-stately/src/toggle/useToggleState.ts @ 6f664fe911
```

The path is relative to react-spectrum's `packages/`; the hash is the commit the file was last synced against:
everything upstream up to it is ported or consciously skipped (and listed in the deviation block). Leptonic-only
files start with `// No upstream: <what it is for>.` instead.

`scripts/upstream-drift.sh` turns these lines into a work list:

```bash
scripts/upstream-drift.sh                 # Files with unabsorbed upstream commits, most drifted first.
scripts/upstream-drift.sh -v use_press    # The commits themselves, for files matching the filter.
scripts/upstream-drift.sh --mark-synced leptonic/src/hooks/interactions/use_press.rs
```

Tests use the same header for the upstream tests they mirror: browser test files
(`// Upstream: react-aria-components/test/ListBox.test.js @ <sha>` in `tests/ui_tests/test_listbox.rs`) and native
test modules (in their source file's header, next to the ported sources). The script reads only unindented lines at
the top; an indented comment on a case names only the upstream case (`// Upstream: "with indeterminate prop".`).
Derive test cases from upstream's tests and implementation instead of inventing them.

To re-sync a file: read the listed commits (`git -C ~/dev/react-spectrum show <hash>`), port what applies, add
deviations for what doesn't, cover the behavior with tests, then `--mark-synced` the file. A new file's headers use
react-spectrum's current HEAD (`git -C ~/dev/react-spectrum rev-parse --short=10 HEAD`).

## React-Aria Deviations

Every intentional difference from upstream is documented at the top of the file:

```rust
// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - [What differs]: [why]. React-aria: [what it does].
//
// ## OMITTED FEATURES
// - `featureName`: [why]. React-aria: [what it does].
//
// =============================================================================
```

Use exactly these categories, in this order, and only the ones that apply. **Every entry states its reason.**

| Category                      | Use When                                                              |
|-------------------------------|-----------------------------------------------------------------------|
| `API DIFFERENCES`             | Naming or structural changes (Rust-native API shapes)                 |
| `DIFFERENT BEHAVIOR`          | Same feature, different approach                                      |
| `LEPTOS-SPECIFIC ADAPTATIONS` | Changes required by Rust/Leptos (event model, SSR, ownership)         |
| `ADDITIONS`                   | Functionality react-aria doesn't have                                 |
| `OMITTED FEATURES`            | Not implemented: intentionally (say why) or not yet (say what blocks) |

A file without deviations says so in one line: `// No deviations from react-aria beyond the project-wide API
conventions.` Project-wide deviations (the API conventions, hook-owned state, `CapturedElement` instead of refs,
locale from context) are recorded once in `leptonic/src/hooks/mod.rs`; don't repeat them per file.

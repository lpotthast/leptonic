# Porting from react-aria

react-aria defines the behavior; the API shape is ours (`conventions.md`, "Guiding decisions"). This file is about
the porting itself, for hooks, atoms, the atom theme and tests alike: where upstream lives, how a file records what
it ports and from which commit, how it documents where it deviates, and how to absorb upstream changes. The atom
theme's stylesheets follow the same headers (`atom-theme.md`).

Most hooks are based on hooks from Adobe's `react-aria` library (part of `react-spectrum`), checked out at
`{leptonic_root_dir}/../react-spectrum`. Since react-spectrum's package consolidation (2026-03), the sources live in
`packages/react-aria/src/<package>/` and `packages/react-stately/src/<package>/`. The old `packages/@react-aria/*`
packages only re-export them.

## Tracking Upstream Changes

react-aria keeps evolving, so every file that ports upstream code declares where it came from, at the very top.
Every hook or atom with a react-aria counterpart lists ALL relevant upstream files there (the user's rule,
2026-10-09): the sources it ports (react-aria hook, react-stately state, react-aria-components component, shared
utils it mirrors) and the upstream test files of that behavior (react-aria's hook tests, react-stately's state tests,
react-aria-components' and `@adobe/react-spectrum`'s component tests), since the tests specify the expected behavior:

```rust
// Upstream: react-aria/src/interactions/usePress.ts @ 6f664fe911
// Upstream: react-stately/src/toggle/useToggleState.ts @ 6f664fe911
```

The path is relative to react-spectrum's `packages/` directory. The hash is the react-spectrum commit this file was
last synced against: everything upstream up to that commit is either ported or consciously skipped (and then listed
in the deviations block below).

`scripts/upstream-drift.sh` turns these lines into a work list:

```bash
scripts/upstream-drift.sh                 # Files with unabsorbed upstream commits, most drifted first.
scripts/upstream-drift.sh -v use_press    # The commits themselves, for files matching the filter.
scripts/upstream-drift.sh --mark-synced leptonic/src/hooks/interactions/use_press.rs
```

Browser tests use the same header for the react-aria tests they mirror (e.g.
`// Upstream: react-aria-components/test/ListBox.test.js @ <sha>` in `leptonic/tests/ui_tests/test_listbox.rs`), and
so do native test modules: a `mod tests` whose cases mirror upstream tests names those test files in its source
file's header, next to the ported sources (`use_progress_bar.rs`:
`// Upstream: react-aria/test/progress/useProgressBar.test.js @ <sha>`), never in indented comments the script
doesn't read (an indented comment on a case names only the upstream case: `// Upstream: "with indeterminate
prop".`). So new or changed upstream tests show up in the drift report too. react-aria's tests are the best
specification of expected behavior: derive our test cases from them (and from the implementation), instead of
inventing them.

To re-sync a hook: read the listed upstream commits (`git -C ../react-spectrum show <hash>`), port what applies, add
deviations for what doesn't, cover the behavior with tests, then run `--mark-synced` on the file. When you add a new
hook, add its `// Upstream:` lines with react-spectrum's current HEAD (`git -C ../react-spectrum rev-parse
--short=10 HEAD`).

## React-Aria Deviations

Wherever we decided to deviate from the react-aria implementation of a hook, document these intentional differences
at the top of hook files:

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

A hook without deviations says so in one line: `// No deviations from react-aria beyond the project-wide API
conventions.` Leptonic-only hooks (no upstream counterpart) start with `// No upstream: <what it is for>.` instead of
an `// Upstream:` header. Project-wide deviations (the API conventions of `conventions.md`, hook-owned state, `CapturedElement`
instead of refs) are recorded once in `leptonic/src/hooks/mod.rs`; don't repeat them per hook.
